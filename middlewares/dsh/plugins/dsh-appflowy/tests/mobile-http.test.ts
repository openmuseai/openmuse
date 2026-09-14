import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { Context } from "@deepseek-ai/cordis";
import { expect, it, vi } from "vitest";
import { apply, buildPresentationIntent, enqueuePresentationIntent } from "../src/parent-bridge.js";

it("real HTTP/SSE carrier: capability, authenticated bind, shared hosts, scope and teardown", async () => {
  const saved = { ...process.env };
  const realFetch = globalThis.fetch;
  const routes = new Map<string, (req: IncomingMessage, res: ServerResponse) => void | Promise<void>>();
  const disposers: Array<() => void> = [];
  const removed: string[] = [];
  const workspaces: Array<{ id: string; path: string; title: string }> = [];
  const server = createServer((req, res) => {
    const handler = routes.get(new URL(req.url ?? "/", "http://localhost").pathname);
    if (!handler) { res.writeHead(404); res.end(); return; }
    void Promise.resolve(handler(req, res)).catch(() => { res.writeHead(500); res.end(); });
  });
  let reader: ReadableStreamDefaultReader<Uint8Array> | undefined;
  try {
    process.env.MUSE_DOCUMENT_CLOUD_URL = "https://cloud.fixture.invalid";
    process.env.MUSE_PARENT_BRIDGE = "1";
    process.env.MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST = "0";
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = await mkdtemp(join(tmpdir(), "muse-mobile-http-"));
    vi.spyOn(globalThis, "fetch").mockImplementation((input, init) => {
      if (String(input).startsWith("https://cloud.fixture.invalid/")) {
        expect(new Headers(init?.headers).get("authorization")).toBe("Bearer fixture.signature");
        expect(new Headers(init?.headers).get("x-muse-device-id")).toBe("mobile.fixture");
        return Promise.resolve(Response.json({ code: 0, data: { workspaceId: "ws-1", title: "Docs" } }));
      }
      return realFetch(input, init);
    });
    apply({
      effect(factory: () => unknown) { const disposer = factory(); if (typeof disposer === "function") disposers.push(disposer as () => void); },
      webServer: {
        register(route: { path: string; handler: (req: IncomingMessage, res: ServerResponse) => void | Promise<void> }) {
          routes.set(route.path, route.handler); return () => { routes.delete(route.path); };
        },
        tapIndex() { return () => {}; }
      },
      workspaceRegistry: {
        async create(path: string, title: string) { const item = { id: "dsh-workspace", path, title }; workspaces.push(item); return item; },
        async resolveByPath(path: string) { return workspaces.find(w => w.path === path); },
        get(id: string) { return workspaces.find(w => w.id === id); },
        list() { return [...workspaces]; }, async delete() { return false; }
      },
      museContextBroker: {
        ingestContribution() {}, removeSurface(ref: string) { removed.push(ref); }, pinSurface() { return () => {}; }, registerProjection() { return () => {}; }
      },
      tools: { register() { return () => {}; } }
    } as unknown as Context);
    await new Promise<void>(resolve => server.listen(0, "127.0.0.1", resolve));
    const address = server.address();
    if (address === null || typeof address === "string") throw new Error("SERVER_NOT_STARTED");
    const base = `http://127.0.0.1:${address.port}/muse/v1/parent-bridge`;
    expect(await (await fetch(`${base}/capabilities`)).json()).toMatchObject({ nativeHttpSse: false });
    process.env.MUSE_MOBILE_BRIDGE = "1";
    expect(await (await fetch(`${base}/capabilities`)).json()).toMatchObject({
      nativeHttpSse: true, sharedHosts: true, mode: "exclusive-test"
    });
    delete process.env.MUSE_MOBILE_BRIDGE;
    process.env.MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST = "1";
    const headers = { "Content-Type": "application/json", Authorization: "Bearer fixture.signature",
      "X-Muse-Device-Id": "mobile.fixture", "X-Muse-Client": "mobile", "X-Muse-Connection-Id": "connection.one" };
    const post = (body: unknown, extra: Record<string, string> = {}) => fetch(base, { method: "POST", headers: { ...headers, ...extra }, body: JSON.stringify(body) });
    const hello = { source: "muse.appflowy-mobile", type: "parent-hello", workspaceRef: "ws-1" };
    expect(await (await post(hello)).json()).toEqual({ ok: true, bound: "ws-1" });
    const stream = await fetch(`${base}/events`, { headers });
    expect(stream.headers.get("content-type")).toContain("text/event-stream");
    reader = stream.body!.getReader();
    const ready = new TextDecoder().decode((await reader.read()).value);
    expect(ready).toContain('"type":"bridge.ready"');
    const intent = buildPresentationIntent({ intentType: "surface.open", viewId: "view-1", workspaceId: "ws-1" });
    expect(enqueuePresentationIntent(intent)).toBe(true);
    expect(new TextDecoder().decode((await reader.read()).value)).toContain(intent.intentRef);
    expect((await post(hello, { "X-Muse-Connection-Id": "connection.two" })).status).toBe(200);
    expect((await post({ source: "muse.appflowy-mobile", type: "context.contribute", envelope: {
      pluginId: "muse.appflowy.workspace", scopeRef: "workspace.foreign", payload: { workspaceId: "foreign" }
    } })).status).toBe(403);
    expect((await fetch(base, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ source: "muse.appflowy-web", type: "workspace.bind", workspaceRef: "other" }) })).status).toBe(403);
    const webSame = await fetch(base, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ source: "muse.appflowy-web", type: "workspace.bind", workspaceRef: "ws-1" }) });
    expect(webSame.status).not.toBe(409);
    const webEvents = await fetch(`${base}/events`);
    expect(webEvents.status).toBe(200);
    expect(webEvents.headers.get("content-type")).toContain("text/event-stream");
    await webEvents.body?.cancel();
    expect(await (await post({ source: "muse.appflowy-mobile", type: "peer.close" })).json()).toEqual({ ok: true });
    expect(removed).not.toContain("surface.appflowy.workspace.ws-1");
  } finally {
    await reader?.cancel();
    for (const dispose of disposers.reverse()) dispose();
    server.closeAllConnections();
    await new Promise<void>(resolve => server.close(() => resolve()));
    vi.restoreAllMocks();
    for (const key of ["MUSE_DOCUMENT_CLOUD_URL", "MUSE_PARENT_BRIDGE", "MUSE_MOBILE_BRIDGE", "MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST", "MUSE_APPFLOWY_DSH_WORKSPACE_ROOT"]) {
      if (saved[key] === undefined) delete process.env[key]; else process.env[key] = saved[key];
    }
  }
});
