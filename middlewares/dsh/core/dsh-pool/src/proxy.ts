import { createServer, request as httpRequest, type IncomingMessage, type ServerResponse } from "node:http";
import { connect } from "node:net";
import type { Duplex } from "node:stream";
import { isJwtAuthorization } from "./executor.js";
import type { InstancePool } from "./pool.js";

const TENANT = /^\/u\/([a-f0-9]{32})(\/.*)?$/u;

export const stripUpstreamAuth = (headers: IncomingMessage["headers"]): Record<string, string | string[] | undefined> => {
  const next: Record<string, string | string[] | undefined> = { ...headers };
  delete next.cookie;
  delete next.Cookie;
  const auth = headers.authorization;
  if (typeof auth === "string" && isJwtAuthorization(auth)) delete next.authorization;
  return next;
};

type TenantRoute = { rest: string; port: number };
type TenantError = { error: "UNKNOWN_TENANT_PATH" | "INSTANCE_NOT_READY" };

const resolveTenant = (url: string, pool: InstancePool): TenantRoute | TenantError => {
  const match = TENANT.exec(url.split("?")[0] ?? "");
  if (match === null) return { error: "UNKNOWN_TENANT_PATH" };
  const hash = match[1] ?? "";
  const rest = match[2] ?? "/";
  const port = pool.lookupPort(hash);
  if (port === undefined) return { error: "INSTANCE_NOT_READY" };
  return { rest, port };
};

const upstreamPath = (url: string, rest: string): string =>
  `${rest}${url.includes("?") ? `?${url.split("?")[1]}` : ""}`;

const writeHeaderBlock = (
  headers: Record<string, string | string[] | undefined>
): string => {
  const lines: string[] = [];
  for (const [key, value] of Object.entries(headers)) {
    if (value === undefined) continue;
    if (Array.isArray(value)) {
      for (const item of value) lines.push(`${key}: ${item}`);
    } else {
      lines.push(`${key}: ${value}`);
    }
  }
  return lines.join("\r\n");
};

/**
 * Keep the public Host. DSH `isTrustedApiRequest` requires Origin host == Host;
 * rewriting to loopback (`127.0.0.1:port`) makes browser Origin fail with 403.
 */
export const createPoolProxy = (pool: InstancePool): ReturnType<typeof createServer> => {
  const server = createServer((req: IncomingMessage, res: ServerResponse) => {
    const url = req.url ?? "/";
    const route = resolveTenant(url, pool);
    if ("error" in route) {
      res.writeHead(route.error === "UNKNOWN_TENANT_PATH" ? 404 : 503, {
        "content-type": "application/json"
      });
      res.end(JSON.stringify({ ok: false, error: route.error }));
      return;
    }
    const headers = stripUpstreamAuth(req.headers);
    const upstream = httpRequest({
      host: "127.0.0.1",
      port: route.port,
      path: upstreamPath(url, route.rest),
      method: req.method,
      headers
    }, up => {
      res.writeHead(up.statusCode ?? 502, up.headers);
      up.pipe(res);
    });
    upstream.on("error", () => {
      if (!res.headersSent) res.writeHead(502);
      res.end();
    });
    req.pipe(upstream);
  });

  server.on("upgrade", (req: IncomingMessage, clientSocket: Duplex, head: Buffer) => {
    const url = req.url ?? "/";
    const route = resolveTenant(url, pool);
    if ("error" in route) {
      const status = route.error === "UNKNOWN_TENANT_PATH" ? 404 : 503;
      clientSocket.write(`HTTP/1.1 ${status} Error\r\nConnection: close\r\n\r\n`);
      clientSocket.destroy();
      return;
    }
    const headers = stripUpstreamAuth(req.headers);
    const path = upstreamPath(url, route.rest);
    const proxySocket = connect(route.port, "127.0.0.1");
    proxySocket.on("error", () => clientSocket.destroy());
    clientSocket.on("error", () => proxySocket.destroy());
    proxySocket.on("connect", () => {
      const block = writeHeaderBlock(headers);
      proxySocket.write(`${req.method ?? "GET"} ${path} HTTP/1.1\r\n${block}\r\n\r\n`);
      if (head.length > 0) proxySocket.write(head);
      proxySocket.pipe(clientSocket);
      clientSocket.pipe(proxySocket);
    });
  });

  return server;
};
