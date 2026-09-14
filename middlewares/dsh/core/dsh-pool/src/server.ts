import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import type { InstancePool } from "./pool.js";

const readJson = (req: IncomingMessage): Promise<Record<string, unknown>> =>
  new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    req.on("data", chunk => chunks.push(chunk as Buffer));
    req.on("end", () => {
      try {
        resolve(JSON.parse(Buffer.concat(chunks).toString("utf8") || "{}") as Record<string, unknown>);
      } catch (error) {
        reject(error);
      }
    });
    req.on("error", reject);
  });

const write = (res: ServerResponse, status: number, body: unknown): void => {
  res.writeHead(status, { "content-type": "application/json" });
  res.end(JSON.stringify(body));
};

export const createControlServer = (pool: InstancePool): ReturnType<typeof createServer> => {
  return createServer((req, res) => {
    void (async () => {
      const path = req.url ?? "/";
      try {
        if (req.method === "GET" && path === "/healthz") {
          write(res, 200, pool.status());
          return;
        }
        if (req.method === "POST" && path === "/internal/session/open") {
          const body = await readJson(req);
          const accountRef = String(body.accountRef ?? "");
          const workspaceRef = String(body.workspaceRef ?? "");
          const deviceId = String(body.deviceId ?? "");
          if (!accountRef || !workspaceRef || !deviceId) {
            write(res, 400, { ok: false, error: "INVALID_SESSION_OPEN" });
            return;
          }
          write(res, 200, await pool.open({ accountRef, workspaceRef, deviceId }));
          return;
        }
        if (req.method === "POST" && path === "/internal/session/close") {
          const body = await readJson(req);
          write(res, 200, await pool.close(String(body.sessionRef ?? ""), String(body.deviceId ?? "")));
          return;
        }
        if (req.method === "POST" && path === "/internal/session/heartbeat") {
          const body = await readJson(req);
          write(res, 200, await pool.heartbeat(String(body.sessionRef ?? "")));
          return;
        }
        write(res, 404, { ok: false, error: "NOT_FOUND" });
      } catch (error) {
        write(res, 500, { ok: false, error: error instanceof Error ? error.message : "POOL_ERROR" });
      }
    })();
  });
};
