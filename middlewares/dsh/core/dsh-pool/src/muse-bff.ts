import { createHash, createHmac, randomUUID, timingSafeEqual } from "node:crypto";
import { execFile } from "node:child_process";
import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

export type MuseActor = { uuid: string; deviceId?: string };

export type WorkspaceCurrent = { workspaceId: string; title: string; role?: string };

export type DeviceTokenClaims = {
  sessionRef: string;
  deviceId: string;
  actorRef: string;
  expiresAt: number;
  nonce: string;
  kid: string;
};

export type MuseBffDeps = {
  gotrueUrl: string;
  tokenSecret: string;
  tokenKid: string;
  tokenTtlSecs: number;
  nowMs?: () => number;
  verifyJwt?: (token: string) => Promise<string | undefined>;
  queryWorkspace?: (actorUuid: string, workspaceId: string) => Promise<WorkspaceCurrent | undefined>;
  poolUrl?: string;
  fetchImpl?: typeof fetch;
  onAccessLog?: (entry: Record<string, unknown>) => void;
};

const b64url = (buf: Buffer): string => buf.toString("base64url");

const json = (res: ServerResponse, status: number, body: unknown): void => {
  res.writeHead(status, { "content-type": "application/json" });
  res.end(JSON.stringify(body));
};

const empty = (res: ServerResponse, status: number): void => {
  res.writeHead(status);
  res.end();
};

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

export const extractIngressToken = (
  authorization: string | undefined,
  deviceTokenHeader: string | undefined,
  cookieHeader: string | undefined
): string | undefined => {
  if (authorization?.startsWith("Bearer ")) {
    const token = authorization.slice(7).trim();
    if (token) return token;
  }
  const device = deviceTokenHeader?.trim();
  if (device) return device;
  if (!cookieHeader) return undefined;
  for (const part of cookieHeader.split(";")) {
    const trimmed = part.trim();
    const eq = trimmed.indexOf("=");
    if (eq <= 0) continue;
    const name = trimmed.slice(0, eq).trim();
    const value = trimmed.slice(eq + 1).trim();
    if ((name === "access_token" || name === "token") && value) return value;
  }
  return undefined;
};

const hmacSign = (secret: string, payload: string): string =>
  b64url(createHmac("sha256", secret).update(payload).digest());

export const issueDeviceToken = (
  secret: string,
  kid: string,
  sessionRef: string,
  deviceId: string,
  actorRef: string,
  expiresAt: number,
  nonce = randomUUID()
): string => {
  const claims: DeviceTokenClaims = { sessionRef, deviceId, actorRef, expiresAt, nonce, kid };
  const payload = b64url(Buffer.from(JSON.stringify(claims)));
  return `${payload}.${hmacSign(secret, payload)}`;
};

export const verifyDeviceTokenSigned = (
  secret: string,
  expectedKid: string,
  token: string,
  nowMs: number
): DeviceTokenClaims => {
  const parts = token.split(".");
  if (parts.length !== 2 || !parts[0] || !parts[1]) throw new Error("TOKEN_INVALID");
  const [payload, signature] = parts;
  const expected = hmacSign(secret, payload);
  const a = Buffer.from(expected);
  const b = Buffer.from(signature);
  if (a.length !== b.length || !timingSafeEqual(a, b)) throw new Error("TOKEN_INVALID");
  const claims = JSON.parse(Buffer.from(payload, "base64url").toString("utf8")) as DeviceTokenClaims;
  if (claims.kid !== expectedKid) throw new Error("TOKEN_KID_MISMATCH");
  if (claims.expiresAt <= nowMs) throw new Error("TOKEN_EXPIRED_OR_DEVICE_MISMATCH");
  return claims;
};

const header = (req: IncomingMessage, name: string): string | undefined => {
  const raw = req.headers[name.toLowerCase()];
  if (typeof raw === "string") return raw;
  if (Array.isArray(raw) && raw[0]) return raw[0];
  return undefined;
};

export const hashWorkspaceRef = (workspaceId: string): string =>
  createHash("sha256").update(workspaceId).digest("hex").slice(0, 12);

const ATTACHMENT_ID = /^att_[A-Za-z0-9._-]{6,80}$/;

export const readAttachmentId = (req: IncomingMessage): string | undefined => {
  const raw = header(req, "x-muse-attachment-id")?.trim();
  if (raw && ATTACHMENT_ID.test(raw)) return raw;
  return undefined;
};

export const redactLogText = (value: string): string =>
  value
    .replace(/([?&]token=)[^&\s"']+/gi, "$1redacted")
    .replace(/access_token=[^;\s"']+/gi, "access_token=redacted")
    .replace(/Bearer\s+[A-Za-z0-9._-]+/gi, "Bearer redacted");

const gotrueVerify = async (deps: MuseBffDeps, token: string): Promise<string | undefined> => {
  if (deps.verifyJwt) return deps.verifyJwt(token);
  const fetchImpl = deps.fetchImpl ?? fetch;
  const res = await fetchImpl(`${deps.gotrueUrl.replace(/\/$/, "")}/user`, {
    headers: { Authorization: `Bearer ${token}` }
  });
  if (!res.ok) return undefined;
  const body = (await res.json()) as { id?: string };
  return typeof body.id === "string" && body.id ? body.id : undefined;
};

const roleLabel = (roleId: string): string | undefined => {
  if (roleId === "1") return "owner";
  if (roleId === "2") return "member";
  if (roleId === "3") return "guest";
  return undefined;
};

export const dockerPsqlWorkspace = async (
  actorUuid: string,
  workspaceId: string
): Promise<WorkspaceCurrent | undefined> => {
  if (!/^[0-9a-f-]{36}$/i.test(actorUuid) || !/^[0-9a-f-]{36}$/i.test(workspaceId)) return undefined;
  const sql =
    "SELECT w.workspace_name, wm.role_id FROM af_workspace_member wm " +
    "JOIN af_user u ON wm.uid = u.uid JOIN af_workspace w ON w.workspace_id = wm.workspace_id " +
    `WHERE u.uuid = '${actorUuid}' AND w.workspace_id = '${workspaceId}' ` +
    "AND w.deleted_at IS NULL LIMIT 1;";
  try {
    const { stdout } = await execFileAsync("docker", [
      "exec",
      "appflowy-cloud-postgres-1",
      "psql",
      "-U",
      "postgres",
      "-d",
      "postgres",
      "-tAc",
      sql
    ]);
    const line = stdout.trim();
    if (!line) return undefined;
    const [title, roleId] = line.split("|");
    const role = roleLabel(roleId ?? "");
    return {
      workspaceId,
      title: title || "Workspace",
      ...(role ? { role } : {})
    };
  } catch {
    return undefined;
  }
};

const resolveActor = async (deps: MuseBffDeps, req: IncomingMessage): Promise<MuseActor | undefined> => {
  const token = extractIngressToken(
    header(req, "authorization"),
    header(req, "x-muse-device-token"),
    header(req, "cookie")
  );
  if (!token) return undefined;
  const now = deps.nowMs?.() ?? Date.now();
  if (token.split(".").length === 2) {
    try {
      const claims = verifyDeviceTokenSigned(deps.tokenSecret, deps.tokenKid, token, now);
      const sent = header(req, "x-muse-device-id");
      if (sent && sent !== claims.deviceId) return undefined;
      return { uuid: claims.actorRef, deviceId: claims.deviceId };
    } catch {
      return undefined;
    }
  }
  const uuid = await gotrueVerify(deps, token);
  if (!uuid) return undefined;
  return { uuid };
};

const pathnameOf = (req: IncomingMessage): string => {
  try {
    return new URL(req.url ?? "/", "http://127.0.0.1").pathname;
  } catch {
    return req.url ?? "/";
  }
};

export const createMuseBff = (deps: MuseBffDeps): ReturnType<typeof createServer> => {
  const queryWorkspace = deps.queryWorkspace ?? dockerPsqlWorkspace;
  const accessLog = (entry: Record<string, unknown>): void => {
    const line = {
      ...entry,
      ts: deps.nowMs?.() ?? Date.now()
    };
    if (deps.onAccessLog) deps.onAccessLog(line);
  };
  return createServer((req, res) => {
    void (async () => {
      const path = pathnameOf(req);
      const method = req.method ?? "GET";
      try {
        if ((method === "GET" || method === "HEAD") && path === "/api/muse/dsh/ingress-auth") {
          const actor = await resolveActor(deps, req);
          if (!actor) {
            empty(res, 401);
            return;
          }
          empty(res, 200);
          return;
        }
        if (method === "POST" && path === "/api/muse/dsh/device-token") {
          const raw = extractIngressToken(
            header(req, "authorization"),
            undefined,
            header(req, "cookie")
          );
          if (!raw || raw.split(".").length === 2) {
            json(res, 401, { code: 1, message: "UNAVAILABLE: document adapter unauthorized" });
            return;
          }
          const actor = await resolveActor(deps, req);
          if (!actor) {
            json(res, 401, { code: 1, message: "UNAVAILABLE: document adapter unauthorized" });
            return;
          }
          const body = (await readJson(req).catch(() => ({}))) as Record<string, unknown>;
          const deviceId =
            typeof body.deviceId === "string" && body.deviceId.trim()
              ? body.deviceId.trim()
              : `web.${randomUUID()}`;
          const sessionRef =
            typeof body.sessionRef === "string" && body.sessionRef.trim()
              ? body.sessionRef.trim()
              : `account:${actor.uuid}`;
          const expiresAt = (deps.nowMs?.() ?? Date.now()) + Math.max(deps.tokenTtlSecs, 60) * 1000;
          const token = issueDeviceToken(
            deps.tokenSecret,
            deps.tokenKid,
            sessionRef,
            deviceId,
            actor.uuid,
            expiresAt
          );
          json(res, 200, { code: 0, data: { token, expiresAt, deviceId, kid: deps.tokenKid } });
          return;
        }
        if (method === "POST" && path === "/api/muse/workspace/current") {
          const actor = await resolveActor(deps, req);
          if (!actor) {
            json(res, 401, { code: 1, message: "UNAVAILABLE: document adapter unauthorized" });
            return;
          }
          const body = (await readJson(req).catch(() => ({}))) as Record<string, unknown>;
          const workspaceId =
            typeof body.workspaceId === "string"
              ? body.workspaceId.trim()
              : typeof body.workspace_id === "string"
                ? body.workspace_id.trim()
                : "";
          if (!workspaceId) {
            json(res, 400, { code: 1, message: "workspaceId required" });
            return;
          }
          const found = await queryWorkspace(actor.uuid, workspaceId);
          if (!found) {
            json(res, 400, { code: 1, message: "SCOPE_MISMATCH" });
            return;
          }
          json(res, 200, { code: 0, data: found });
          return;
        }
        if (
          method === "POST" &&
          (path === "/api/muse/dsh/session/open" ||
            path === "/api/muse/dsh/session/close" ||
            path === "/api/muse/dsh/session/heartbeat")
        ) {
          const attachmentId = readAttachmentId(req);
          const actor = await resolveActor(deps, req);
          if (!actor) {
            accessLog({ action: `session.${path.split("/").pop() ?? "open"}`, status: 401, ...(attachmentId ? { attachmentId } : {}) });
            json(res, 401, { code: 1, message: "UNAVAILABLE: document adapter unauthorized" });
            return;
          }
          const poolUrl = deps.poolUrl?.trim();
          if (!poolUrl) {
            accessLog({ action: `session.${path.split("/").pop() ?? "open"}`, status: 503, ...(attachmentId ? { attachmentId } : {}) });
            json(res, 503, { code: 1067, message: "UNAVAILABLE: dsh.session: POOL_UNAVAILABLE" });
            return;
          }
          const body = (await readJson(req).catch(() => ({}))) as Record<string, unknown>;
          const action = path.split("/").pop() ?? "open";
          let payload: Record<string, string>;
          let workspaceHash: string | undefined;
          if (action === "open") {
            const workspaceId =
              typeof body.workspaceId === "string"
                ? body.workspaceId.trim()
                : typeof body.workspace_id === "string"
                  ? body.workspace_id.trim()
                  : "";
            if (!workspaceId) {
              json(res, 400, { code: 1, message: "workspaceId required" });
              return;
            }
            const found = await queryWorkspace(actor.uuid, workspaceId);
            if (!found) {
              json(res, 400, { code: 1, message: "SCOPE_MISMATCH" });
              return;
            }
            const deviceId =
              typeof body.deviceId === "string" && body.deviceId.trim()
                ? body.deviceId.trim()
                : actor.deviceId ?? `web.${randomUUID()}`;
            payload = { accountRef: actor.uuid, workspaceRef: workspaceId, deviceId };
            workspaceHash = hashWorkspaceRef(workspaceId);
          } else if (action === "close") {
            payload = {
              sessionRef: typeof body.sessionRef === "string" ? body.sessionRef : "",
              deviceId:
                typeof body.deviceId === "string" && body.deviceId.trim()
                  ? body.deviceId.trim()
                  : actor.deviceId ?? ""
            };
          } else {
            payload = { sessionRef: typeof body.sessionRef === "string" ? body.sessionRef : "" };
          }
          const fetchImpl = deps.fetchImpl ?? fetch;
          let upstream: Response;
          try {
            upstream = await fetchImpl(`${poolUrl.replace(/\/$/u, "")}/internal/session/${action}`, {
              method: "POST",
              headers: { "content-type": "application/json" },
              body: JSON.stringify(payload),
              signal: AbortSignal.timeout(200_000)
            });
          } catch {
            accessLog({ action: `session.${action}`, status: 503, ...(attachmentId ? { attachmentId } : {}), ...(workspaceHash ? { workspaceHash } : {}) });
            json(res, 503, { code: 1067, message: "UNAVAILABLE: dsh.session: POOL_UNAVAILABLE" });
            return;
          }
          const raw = await upstream.text();
          let parsed: unknown = raw;
          try {
            parsed = raw ? JSON.parse(raw) : {};
          } catch {
            parsed = { message: raw };
          }
          if (!upstream.ok) {
            const status = upstream.status >= 500 ? 503 : upstream.status;
            accessLog({ action: `session.${action}`, status, ...(attachmentId ? { attachmentId } : {}), ...(workspaceHash ? { workspaceHash } : {}) });
            const detail =
              parsed && typeof parsed === "object" && "error" in parsed && typeof (parsed as { error: unknown }).error === "string"
                ? String((parsed as { error: string }).error).split("\n")[0]?.slice(0, 180)
                : "";
            json(res, status, {
              code: 1067,
              message: detail
                ? `UNAVAILABLE: dsh.session: ${redactLogText(detail)}`
                : `UNAVAILABLE: dsh.session: POOL_${upstream.status}`
            });
            return;
          }
          accessLog({ action: `session.${action}`, status: 200, ...(attachmentId ? { attachmentId } : {}), ...(workspaceHash ? { workspaceHash } : {}) });
          json(res, 200, { code: 0, data: parsed });
          return;
        }
        const unwired = new Set([
          "/api/muse/workspace/tree",
          "/api/muse/document/query",
          "/api/muse/document/propose",
          "/api/muse/document/apply",
          "/api/muse/document/status"
        ]);
        if (method === "POST" && unwired.has(path)) {
          const actor = await resolveActor(deps, req);
          if (!actor) {
            json(res, 401, { code: 1, message: "UNAVAILABLE: document adapter unauthorized" });
            return;
          }
          json(res, 501, { code: 1067, message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" });
          return;
        }
        json(res, 404, { code: 1, message: "NOT_FOUND" });
      } catch {
        json(res, 500, { code: 1, message: "BFF_ERROR" });
      }
    })();
  });
};
