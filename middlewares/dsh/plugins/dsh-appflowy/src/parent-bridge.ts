import type { IncomingMessage, ServerResponse } from "node:http";
import { randomUUID } from "node:crypto";
import type { Context } from "@deepseek-ai/cordis";
import type { JsonValue } from "@muse/host-bridge";
import { defineTool } from "@deepseek-ai/dsh-tools";
import type { ContextContributionEnvelopeV1, PresentationIntentEnvelopeV1 } from "@muse/plugin-facets";
import {
  APPFLOWY_WORKSPACE_TITLE,
  applyWorkspaceHint,
  getLastWorkspaceHint,
  listBoundWorkspaceViews as listWorkspaceViews,
  parseWorkspaceViewsQuery,
  resetLastWorkspaceHint,
  type DshWorkspaceRegistry,
  WORKSPACE_VIEWS_PATH
} from "@muse/plugin-appflowy-workspace";
import { getLastDeviceAuth, resetSession, setLastDeviceAuth, setLastDocumentFocus } from "./session.js";
import { createPresentationFacetInbox } from "./presentation-facets.js";
import { ExclusiveMobileLease, verifyMobileWorkspace, type MobileIdentity } from "./mobile-lease.js";

export const PARENT_BRIDGE_PATH = "/muse/v1/parent-bridge";
export const PARENT_BRIDGE_EVENTS_PATH = "/muse/v1/parent-bridge/events";
export const PARENT_BRIDGE_INTENTS_PATH = "/muse/v1/parent-bridge/intents";
export const PARENT_BRIDGE_VIEWS_PATH = WORKSPACE_VIEWS_PATH;
export const MAX_PARENT_MESSAGE_BYTES = 32 * 1024;
export const PARENT_SOURCE = "muse.appflowy-web";
export const MOBILE_SOURCE = "muse.appflowy-mobile";
export const FRAME_SOURCE = "muse.dsh-web";

export const WORKSPACE_FOCUS_DIGEST =
  "sha256:4c3a6bf1cd8249cc96f6aac63ad78b9be04172634f0313ce3ba52163f66043b7";
export const WORKSPACE_TREE_UI_DIGEST =
  "sha256:c0381f40eaaa753e7bb28e79e74c8e44573864aa89928a3ba246a524389edbda";
export const SURFACE_OPEN_DIGEST =
  "sha256:c6337b4511a9ef40dbae92c23784cec068a2ff355ac6bb8e95141328c34905f0";
export const SURFACE_REVEAL_DIGEST =
  "sha256:f50404c3696300a2f00c7ac9ba5f413d48987d458669bf397fad0c6b85eabd8b";

const FORBIDDEN = /access_token|refresh_token|api[_-]?key/i;

type WebServer = {
  register(route: {
    kind: "exact" | "prefix";
    path: string;
    handler: (req: IncomingMessage, res: ServerResponse) => void | Promise<void>;
  }): () => void;
  tapIndex(transform: (html: string) => string): () => void;
};

type BrokerApi = {
  ingestContribution(payload: JsonValue): void;
  removeSurface(surfaceInstanceRef: string): void;
  pinSurface(surfaceInstanceRef: string): () => void;
  registerProjection(projection: {
    pluginId: string;
    contextType: string;
    schemaDigest: string;
    priority: number;
    maxTokens: number;
    render(envelope: ContextContributionEnvelopeV1): string | undefined;
  }): () => void;
};

type ToolsApi = {
  register(tool: unknown): () => void;
};

type HostContext = Context & {
  webServer: WebServer;
  workspaceRegistry: DshWorkspaceRegistry;
  museContextBroker: BrokerApi;
  tools?: ToolsApi;
};

export const envFlagEnabled = (name: string, defaultOn = true): boolean => {
  const raw = process.env[name]?.trim().toLowerCase();
  if (raw === undefined || raw.length === 0) return defaultOn;
  return raw !== "0" && raw !== "false" && raw !== "off";
};

const byteLength = (value: unknown): number => {
  try {
    return Buffer.byteLength(JSON.stringify(value), "utf8");
  } catch {
    return Number.POSITIVE_INFINITY;
  }
};

const asRecord = (value: unknown): Record<string, unknown> | undefined =>
  value !== null && typeof value === "object" && !Array.isArray(value)
    ? value as Record<string, unknown>
    : undefined;

const clean = (value: unknown, max: number): string =>
  typeof value === "string" ? value.replace(/[<>\u0000-\u001f]/gu, " ").slice(0, max) : "";

const payloadObject = (envelope: ContextContributionEnvelopeV1): Record<string, unknown> =>
  envelope.payload !== null && typeof envelope.payload === "object" && !Array.isArray(envelope.payload)
    ? envelope.payload as Record<string, unknown>
    : {};

export const jsonLooksForbidden = (raw: string): boolean => FORBIDDEN.test(raw);

export interface ParentInboundOk {
  readonly ok: true;
  readonly bound?: string;
}

export interface ParentInboundErr {
  readonly ok: false;
  readonly error: string;
}

export type ParentInboundResult = ParentInboundOk | ParentInboundErr;

export interface ParentBridgeDeps {
  readonly registry: DshWorkspaceRegistry;
  readonly ingestContribution: (payload: JsonValue) => void;
  readonly removeSurface: (ref: string) => void;
  readonly pinSurface?: (ref: string) => void;
}

let lastWorkspaceSurface: string | undefined;
let focusedDocumentSurface: string | undefined;
const surfaceCleanup = new Map<string, () => void>();
const trackSurface = (ref: string, remove: (ref: string) => void): void => {
  if (surfaceCleanup.size >= 128 && !surfaceCleanup.has(ref)) {
    const first = surfaceCleanup.entries().next().value;
    if (first !== undefined) { first[1](); surfaceCleanup.delete(first[0]); }
  }
  surfaceCleanup.set(ref, () => remove(ref));
};
const sseClients = new Set<ServerResponse>();
const mobileLease = new ExclusiveMobileLease({ verify: verifyMobileWorkspace, clearContext: () => resetParentBridgeState() });
const mobileEnabled = (): boolean => process.env.MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST === "1";
const nativeRequest = (req: IncomingMessage): boolean => req.headers["x-muse-client"] === "mobile";
const mobileIdentity = (req: IncomingMessage): MobileIdentity => {
  const authorization = req.headers.authorization ?? "";
  const token = authorization.startsWith("Bearer ") ? authorization.slice(7) : "";
  const deviceId = req.headers["x-muse-device-id"];
  const connectionId = req.headers["x-muse-connection-id"];
  if (token.length > 8192 || token.split(".").length !== 2 || typeof deviceId !== "string" || !deviceId || deviceId.length > 128) {
    throw new Error("DEVICE_AUTH_REQUIRED");
  }
  if (typeof connectionId !== "string" || !connectionId || connectionId.length > 128) throw new Error("CONNECTION_ID_REQUIRED");
  return { token, deviceId, connectionId };
};

const workspaceSurfaceRef = (workspaceId: string): string =>
  `surface.appflowy.workspace.${workspaceId}`;

export const resetParentBridgeState = (): void => {
  lastWorkspaceSurface = undefined;
  focusedDocumentSurface = undefined;
  for (const cleanup of surfaceCleanup.values()) cleanup();
  surfaceCleanup.clear();
  resetSession();
  resetLastWorkspaceHint();
  for (const client of sseClients) {
    try {
      client.end();
    } catch {
      /* ignore */
    }
  }
  sseClients.clear();
};

export { getLastDeviceAuth, getLastDocumentFocus } from "./session.js";
export { parseWorkspaceViewsQuery };

export const listBoundWorkspaceViews = (
  query: Parameters<typeof listWorkspaceViews>[0] = {},
  deps: Parameters<typeof listWorkspaceViews>[1] = {}
) => {
  const auth = deps.auth ?? getLastDeviceAuth();
  return listWorkspaceViews(query, {
    ...deps,
    ...(auth === undefined ? {} : { auth })
  });
};

const rememberDeviceAuth = (rec: Record<string, unknown>): void => {
  const token = typeof rec.deviceToken === "string" ? rec.deviceToken.trim() : "";
  if (token.length === 0 || token.startsWith("sk-") || token.includes("..")) return;
  if (token.split(".").length !== 2) return;
  const deviceId = typeof rec.deviceId === "string" ? rec.deviceId.trim().slice(0, 128) : "";
  setLastDeviceAuth({ token, deviceId });
};

const rememberDocumentFocus = (envelope: unknown): void => {
  const rec = asRecord(envelope);
  if (rec === undefined) return;
  const payload = asRecord(rec.payload) ?? {};
  const contextType = typeof rec.contextType === "string" ? rec.contextType : "";
  const scopeRef = typeof rec.scopeRef === "string" ? rec.scopeRef : "";
  const surface = typeof rec.surfaceInstanceRef === "string" ? rec.surfaceInstanceRef : "";
  const workspaceFromScope = scopeRef.startsWith("workspace.") ? scopeRef.slice("workspace.".length) : "";
  const viewFromSurface = surface.startsWith("surface.appflowy.doc.")
    ? surface.slice("surface.appflowy.doc.".length)
    : "";
  const workspaceId = clean(
    typeof payload.workspaceId === "string" && payload.workspaceId.trim().length > 0
      ? payload.workspaceId
      : workspaceFromScope || getLastWorkspaceHint()?.appflowyWorkspaceId,
    128
  );
  const viewId = clean(
    typeof payload.viewId === "string" && payload.viewId.trim().length > 0
      ? payload.viewId
      : typeof payload.viewRef === "string" ? payload.viewRef : viewFromSurface,
    128
  );
  if (
    (contextType === "workspace.focus" || contextType === "markdown.surface" || contextType === "markdown.selection")
    && workspaceId.length > 0
    && viewId.length > 0
  ) {
    setLastDocumentFocus({ workspaceId, viewId });
    focusedDocumentSurface = surface;
  }
};

const bindWorkspace = async (
  rec: Record<string, unknown>,
  deps: ParentBridgeDeps
): Promise<ParentInboundResult> => {
  if (!envFlagEnabled("MUSE_WEB_WORKSPACE_BIND")) return { ok: true };
  const workspaceRef = typeof rec.workspaceRef === "string" ? rec.workspaceRef.trim() : "";
  if (workspaceRef.length === 0 || workspaceRef.length > 128) {
    return { ok: true };
  }
  const title = typeof rec.workspaceTitle === "string" && rec.workspaceTitle.trim().length > 0
    ? rec.workspaceTitle.trim().slice(0, 256)
    : typeof rec.title === "string" && rec.title.trim().length > 0
      ? rec.title.trim().slice(0, 256)
      : APPFLOWY_WORKSPACE_TITLE;
  const previous = getLastWorkspaceHint()?.appflowyWorkspaceId;
  await applyWorkspaceHint(deps.registry, {
    appflowyWorkspaceId: workspaceRef,
    title,
    updatedAt: Date.now()
  });
  const nextSurface = workspaceSurfaceRef(workspaceRef);
  if (previous !== undefined && previous !== workspaceRef) {
    setLastDocumentFocus(undefined);
    focusedDocumentSurface = undefined;
    deps.removeSurface(workspaceSurfaceRef(previous));
    if (lastWorkspaceSurface !== undefined && lastWorkspaceSurface !== nextSurface) {
      deps.removeSurface(lastWorkspaceSurface);
    }
  }
  lastWorkspaceSurface = nextSurface;
  trackSurface(nextSurface, deps.removeSurface);
  deps.pinSurface?.(nextSurface);
  return { ok: true, bound: workspaceRef };
};

export const handleParentInbound = async (
  raw: unknown,
  deps: ParentBridgeDeps
): Promise<ParentInboundResult> => {
  if (byteLength(raw) > MAX_PARENT_MESSAGE_BYTES) {
    return { ok: false, error: "PAYLOAD_TOO_LARGE" };
  }
  const serialized = (() => {
    try {
      return JSON.stringify(raw);
    } catch {
      return "";
    }
  })();
  if (serialized.length === 0 || jsonLooksForbidden(serialized)) {
    return { ok: false, error: "FORBIDDEN_FIELD" };
  }
  const rec = asRecord(raw);
  if (rec === undefined) return { ok: false, error: "INVALID_MESSAGE" };
  if (rec.source !== PARENT_SOURCE) return { ok: false, error: "INVALID_SOURCE" };
  const type = rec.type;
  if (type === "parent-hello" || type === "workspace.bind") {
    if (type === "parent-hello") rememberDeviceAuth(rec);
    return bindWorkspace(rec, deps);
  }
  if (type === "context.contribute") {
    if (!envFlagEnabled("MUSE_WEB_CONTEXT_UPLINK")) return { ok: true };
    const envelope = rec.envelope ?? rec;
    try {
      createPresentationFacetInbox({
        contribute: value => {
          deps.ingestContribution(value as unknown as JsonValue);
          trackSurface(value.surfaceInstanceRef, deps.removeSurface);
        },
        rememberFocus: rememberDocumentFocus,
        ...(deps.pinSurface === undefined ? {} : { pinSurface: deps.pinSurface })
      }).dispatch(envelope);
    } catch {
      return { ok: false, error: "INVALID_FACET_CONTRACT" };
    }
    return { ok: true };
  }
  if (type === "surface.closed") {
    const ref = typeof rec.surfaceInstanceRef === "string" ? rec.surfaceInstanceRef : "";
    if (ref.length > 0) deps.removeSurface(ref);
    surfaceCleanup.delete(ref);
    if (ref === focusedDocumentSurface) { setLastDocumentFocus(undefined); focusedDocumentSurface = undefined; }
    return { ok: true };
  }
  if (type === "intent.receipt") {
    return { ok: true };
  }
  return { ok: true };
};

export const buildPresentationIntent = (input: {
  intentType: "surface.open" | "surface.revealRange";
  viewId: string;
  workspaceId?: string;
  blockId?: string;
}): PresentationIntentEnvelopeV1 => {
  const bound = getLastWorkspaceHint()?.appflowyWorkspaceId;
  const workspaceId = input.workspaceId ?? bound ?? "unbound";
  const payload: Record<string, string> = { viewId: input.viewId, workspaceId };
  if (input.blockId !== undefined && input.blockId.length > 0) {
    payload.blockId = input.blockId;
  }
  return {
    protocol: "muse.presentation-intent/v1",
    pluginId: "muse.appflowy.workspace",
    scopeRef: `workspace.${workspaceId}`,
    intentType: input.intentType,
    intentSchemaDigest: input.intentType === "surface.open" ? SURFACE_OPEN_DIGEST : SURFACE_REVEAL_DIGEST,
    intentRef: `intent.${randomUUID()}`,
    requestedAt: Date.now(),
    expiresAt: Date.now() + 30_000,
    payload
  };
};

export const enqueuePresentationIntent = (envelope: PresentationIntentEnvelopeV1): boolean => {
  if (!envFlagEnabled("MUSE_WEB_INTENT_DOWNLINK")) return false;
  if (envelope.expiresAt <= Date.now()) return false;
  if (byteLength(envelope) > MAX_PARENT_MESSAGE_BYTES) return false;
  const message = {
    source: FRAME_SOURCE,
    type: "intent.dispatch",
    intent: envelope
  };
  const data = JSON.stringify(message).replace(/</gu, "\\u003c");
  for (const client of sseClients) {
    try {
      client.write(`data: ${data}\n\n`);
    } catch {
      sseClients.delete(client);
    }
  }
  return true;
};

const readBody = (req: IncomingMessage, maxBytes: number): Promise<unknown> =>
  new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    let size = 0;
    req.on("data", (chunk: Buffer) => {
      size += chunk.length;
      if (size > maxBytes) {
        req.destroy();
        reject(new Error("PAYLOAD_TOO_LARGE"));
        return;
      }
      chunks.push(chunk);
    });
    req.on("end", () => {
      const raw = Buffer.concat(chunks).toString("utf8");
      if (raw.length === 0) {
        resolve(undefined);
        return;
      }
      try {
        resolve(JSON.parse(raw) as unknown);
      } catch {
        reject(new Error("INVALID_JSON"));
      }
    });
    req.on("error", reject);
  });

const writeJson = (res: ServerResponse, status: number, body: JsonValue): void => {
  if (res.headersSent) return;
  res.writeHead(status, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(body));
};

/**
 * Injected into DSH index.html. Forwards parent postMessage to Host HTTP and
 * pushes PresentationIntent back to the AppFlowy-Web parent.
 * Must not contain a raw `<` or `</script>`.
 */
export const PARENT_BRIDGE_SCRIPT =
  "<script>(function(){" +
  "if(!window.parent||window.parent===window)return;" +
  "var parentOrigin=\"\";" +
  "var parentWin=null;" +
  "function postHost(path,body){" +
  "var xhr=new XMLHttpRequest();" +
  "xhr.open(\"POST\",path,true);" +
  "xhr.setRequestHeader(\"Content-Type\",\"application/json\");" +
  "xhr.send(JSON.stringify(body));" +
  "}" +
  "window.addEventListener(\"message\",function(ev){" +
  "var data=ev.data;" +
  "if(!data||data.source!==\"muse.appflowy-web\")return;" +
  "parentOrigin=ev.origin;" +
  "parentWin=ev.source;" +
  "postHost(\"/muse/v1/parent-bridge\",data);" +
  "});" +
  "try{" +
  "if(window.parent&&window.parent!==window){" +
  "window.parent.postMessage({source:\"muse.dsh-web\",type:\"frame-ready\"},\"*\");" +
  "}" +
  "}catch(err){}" +
  "try{" +
  "var es=new EventSource(\"/muse/v1/parent-bridge/events\");" +
  "es.onmessage=function(e){" +
  "var parsed;" +
  "try{parsed=JSON.parse(e.data);}catch(err){return;}" +
  "if(parentWin&&parentOrigin)parentWin.postMessage(parsed,parentOrigin);" +
  "};" +
  "}catch(err){}" +
  "})();</script>";

export const injectParentBridgeScript = (html: string): string => {
  const head = html.indexOf("<head>");
  if (head === -1) return `${PARENT_BRIDGE_SCRIPT}${html}`;
  return `${html.slice(0, head + 6)}${PARENT_BRIDGE_SCRIPT}${html.slice(head + 6)}`;
};

const presentTool = defineTool({
  name: "muse_appflowy_present",
  description:
    "Ask the AppFlowy Web UI to open a view or reveal a block in the current workspace. Does not write document text.",
  parameters: {
    intentType: { type: "string", required: true },
    viewId: { type: "string", required: true },
    workspaceId: { type: "string" },
    blockId: { type: "string" }
  },
  output: {
    schema: {
      type: "object",
      additionalProperties: false,
      properties: {
        intentRef: { type: "string" },
        status: { type: "string" }
      }
    },
    render: (_args, value) => [{ type: "text", text: JSON.stringify(value) }]
  },
  async execute(args) {
    const intentType = args.intentType === "surface.revealRange" ? "surface.revealRange" : "surface.open";
    const bound = getLastWorkspaceHint()?.appflowyWorkspaceId;
    const workspaceId = typeof args.workspaceId === "string" ? args.workspaceId : bound;
    if (bound !== undefined && workspaceId !== undefined && workspaceId !== bound) {
      return { intentRef: "", status: "SCOPE_MISMATCH" };
    }
    const envelope = buildPresentationIntent({
      intentType,
      viewId: String(args.viewId ?? ""),
      ...(workspaceId === undefined ? {} : { workspaceId }),
      ...(typeof args.blockId === "string" ? { blockId: args.blockId } : {})
    });
    const queued = enqueuePresentationIntent(envelope);
    return { intentRef: envelope.intentRef, status: queued ? "queued" : "rejected" };
  }
});

export const name = "@muse/dsh-appflowy/parent-bridge";
export const inject = ["webServer", "workspaceRegistry", "museContextBroker", "tools"];

/** Desktop sidecar uses hint files + UDS; iframe HTTP is Web/Remote only. */
export const parentBridgeEnabled = (env: NodeJS.ProcessEnv = process.env): boolean => {
  const flag = env.MUSE_PARENT_BRIDGE?.trim().toLowerCase();
  if (flag === "0" || flag === "false" || flag === "off") return false;
  if (flag === "1" || flag === "true" || flag === "on") return true;
  return Boolean(env.MUSE_DOCUMENT_CLOUD_URL?.trim());
};

export const apply = (ctx: Context): void => {
  if (!parentBridgeEnabled()) return;
  const host = ctx as HostContext;
  const server = host.webServer;
  const registry = host.workspaceRegistry;
  const broker = host.museContextBroker;
  if (server?.register === undefined || server.tapIndex === undefined) {
    throw new Error("webServer is required for the AppFlowy parent-bridge");
  }
  if (registry === undefined) {
    throw new Error("workspaceRegistry is required for the AppFlowy parent-bridge");
  }
  if (broker === undefined) {
    throw new Error("museContextBroker is required for the AppFlowy parent-bridge");
  }

  const deps: ParentBridgeDeps = {
    registry,
    ingestContribution: payload => broker.ingestContribution(payload),
    removeSurface: ref => broker.removeSurface(ref),
    pinSurface: ref => {
      broker.pinSurface(ref);
    }
  };
  ctx.effect(() => () => { mobileLease.release(); resetParentBridgeState(); }, "muse.appflowy.parentBridgeDispose");

  ctx.effect(
    () => server.tapIndex(injectParentBridgeScript),
    "muse.appflowy.parentBridgeScript"
  );

  ctx.effect(
    () => server.register({
      kind: "exact", path: `${PARENT_BRIDGE_PATH}/capabilities`,
      handler: (req, res) => {
        if (req.method !== "GET") { writeJson(res, 405, { ok: false }); return; }
        writeJson(res, 200, { protocol: "muse.parent-bridge/v1", nativeHttpSse: mobileEnabled(),
          mode: "exclusive-test", scopedMultiTenant: false, replay: false });
      }
    }), "muse.appflowy.parentBridgeCapabilities"
  );

  ctx.effect(
    () => server.register({
      kind: "exact",
      path: PARENT_BRIDGE_PATH,
      handler: async (req, res) => {
        if (req.method !== "POST") {
          writeJson(res, 405, { ok: false, error: "METHOD_NOT_ALLOWED" });
          return;
        }
        try {
          let body = await readBody(req, MAX_PARENT_MESSAGE_BYTES);
          if (nativeRequest(req)) {
            if (!mobileEnabled()) { writeJson(res, 403, { ok: false, error: "MOBILE_DISABLED" }); return; }
            const rec = asRecord(body);
            if (rec?.source !== MOBILE_SOURCE) { writeJson(res, 400, { ok: false, error: "INVALID_SOURCE" }); return; }
            const identity = mobileIdentity(req);
            await mobileLease.authorize(identity, typeof rec.workspaceRef === "string" ? rec.workspaceRef : undefined);
            const envelope = asRecord(rec.envelope);
            const payload = asRecord(envelope?.payload);
            if ((payload?.workspaceId !== undefined && payload.workspaceId !== mobileLease.workspaceId) ||
                (envelope?.pluginId === "muse.appflowy.workspace" && envelope.scopeRef !== `workspace.${mobileLease.workspaceId}`)) {
              writeJson(res, 403, { ok: false, error: "SCOPE_MISMATCH" }); return;
            }
            if (rec.type === "peer.close") {
              mobileLease.release(); writeJson(res, 200, { ok: true }); return;
            }
            if (rec.type === "peer.ping") { writeJson(res, 200, { ok: true }); return; }
            // Carrier translation only. Both platforms enter the same Facet inbox.
            body = { ...rec, source: PARENT_SOURCE,
              ...(rec.type === "parent-hello" ? { deviceToken: identity.token, deviceId: identity.deviceId } : {}) };
          } else if (mobileLease.occupied) {
            writeJson(res, 409, { ok: false, error: "HOST_IN_USE" }); return;
          }
          const result = await handleParentInbound(body, deps);
          writeJson(res, result.ok ? 200 : 400, result as unknown as JsonValue);
        } catch (error) {
          const message = error instanceof Error ? error.message : "INVALID_JSON";
          writeJson(res, message === "PAYLOAD_TOO_LARGE" ? 413 : message === "HOST_IN_USE" ? 409 : 400, { ok: false, error: message });
        }
      }
    }),
    "muse.appflowy.parentBridgeHttp"
  );

  ctx.effect(
    () => server.register({
      kind: "exact",
      path: PARENT_BRIDGE_EVENTS_PATH,
      handler: (req, res) => {
        if (req.method !== "GET") {
          writeJson(res, 405, { ok: false, error: "METHOD_NOT_ALLOWED" });
          return;
        }
        if (nativeRequest(req)) {
          try {
            if (!mobileEnabled() || !mobileLease.matches(mobileIdentity(req))) throw new Error("DEVICE_AUTH_REQUIRED");
          } catch { writeJson(res, 403, { ok: false, error: "DEVICE_AUTH_REQUIRED" }); return; }
        } else if (mobileLease.occupied) {
          writeJson(res, 409, { ok: false, error: "HOST_IN_USE" }); return;
        }
        res.writeHead(200, {
          "Content-Type": "text/event-stream; charset=utf-8",
          "Cache-Control": "no-cache",
          Connection: "keep-alive"
        });
        res.write("retry: 2000\n\n");
        if (nativeRequest(req)) res.write(`data: ${JSON.stringify({ source: FRAME_SOURCE, type: "bridge.ready", workspaceRef: mobileLease.workspaceId })}\n\n`);
        sseClients.add(res);
        const heartbeat = setInterval(() => res.write(": heartbeat\n\n"), 15_000);
        heartbeat.unref();
        req.on("close", () => {
          clearInterval(heartbeat);
          sseClients.delete(res);
        });
      }
    }),
    "muse.appflowy.parentBridgeEvents"
  );

  ctx.effect(
    () => server.register({
      kind: "exact",
      path: PARENT_BRIDGE_INTENTS_PATH,
      handler: async (req, res) => {
        if (mobileLease.occupied) { writeJson(res, 409, { ok: false, error: "HOST_IN_USE" }); return; }
        if (req.method !== "POST") {
          writeJson(res, 405, { ok: false, error: "METHOD_NOT_ALLOWED" });
          return;
        }
        try {
          const body = asRecord(await readBody(req, MAX_PARENT_MESSAGE_BYTES));
          const intentType = body?.intentType === "surface.revealRange" ? "surface.revealRange" : "surface.open";
          const viewId = typeof body?.viewId === "string" ? body.viewId : "";
          if (viewId.length === 0) {
            writeJson(res, 400, { ok: false, error: "VIEW_ID_REQUIRED" });
            return;
          }
          const envelope = buildPresentationIntent({
            intentType,
            viewId,
            ...(typeof body?.workspaceId === "string" ? { workspaceId: body.workspaceId } : {}),
            ...(typeof body?.blockId === "string" ? { blockId: body.blockId } : {})
          });
          const queued = enqueuePresentationIntent(envelope);
          writeJson(res, queued ? 200 : 409, {
            ok: queued,
            intentRef: envelope.intentRef,
            status: queued ? "queued" : "rejected"
          });
        } catch {
          writeJson(res, 400, { ok: false, error: "INVALID_JSON" });
        }
      }
    }),
    "muse.appflowy.parentBridgeIntents"
  );

  ctx.effect(
    () => server.register({
      kind: "exact",
      path: PARENT_BRIDGE_VIEWS_PATH,
      handler: async (req, res) => {
        if (mobileLease.occupied) { writeJson(res, 409, { ok: false, error: "HOST_IN_USE" }); return; }
        if (req.method !== "GET") {
          writeJson(res, 405, { ok: false, error: "METHOD_NOT_ALLOWED" });
          return;
        }
        const result = await listBoundWorkspaceViews(parseWorkspaceViewsQuery(req.url));
        if (!result.ok) {
          writeJson(res, result.status, { ok: false, error: result.error });
          return;
        }
        writeJson(res, result.status, result.body);
      }
    }),
    "muse.appflowy.parentBridgeViews"
  );

  ctx.effect(() => {
    const disposers = [
      broker.registerProjection({
        pluginId: "muse.appflowy.workspace",
        contextType: "workspace.focus",
        schemaDigest: WORKSPACE_FOCUS_DIGEST,
        priority: 110,
        maxTokens: 80,
        render: envelope => {
          const value = payloadObject(envelope);
          return `AppFlowy workspace focus: id=${clean(value.workspaceId, 128)} title="${clean(value.title, 256)}" view=${clean(value.viewId, 128) || "none"}.`;
        }
      }),
      broker.registerProjection({
        pluginId: "muse.appflowy.workspace",
        contextType: "workspace.tree.ui",
        schemaDigest: WORKSPACE_TREE_UI_DIGEST,
        priority: 40,
        maxTokens: 120,
        render: envelope => {
          const value = payloadObject(envelope);
          const ids = Array.isArray(value.expandedViewIds)
            ? value.expandedViewIds.slice(0, 64).map(item => clean(item, 128)).filter(item => item.length > 0)
            : [];
          return `Sidebar expanded view ids (${ids.length}): ${ids.join(", ") || "none"}.`;
        }
      })
    ];
    return () => disposers.reverse().forEach(dispose => dispose());
  }, "muse.appflowy.workspace-context-projections");

  if (host.tools?.register !== undefined) {
    ctx.effect(
      () => host.tools!.register(presentTool),
      "muse.appflowy.presentTool"
    );
  }
};
