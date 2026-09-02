import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import type { JsonValue } from "@muse/host-bridge";
import {
  PARENT_BRIDGE_SCRIPT,
  getLastDeviceAuth,
  getLastDocumentFocus,
  handleParentInbound,
  injectParentBridgeScript,
  jsonLooksForbidden,
  listBoundWorkspaceViews,
  parseWorkspaceViewsQuery,
  parentBridgeEnabled,
  resetParentBridgeState,
  type ParentBridgeDeps
} from "../src/parent-bridge.js";
import {
  applyWorkspaceHint,
  getLastWorkspaceHint,
  type DshWorkspace,
  type DshWorkspaceRegistry
} from "@muse/plugin-appflowy-workspace";

const fakeRegistry = (): DshWorkspaceRegistry => {
  const items: Array<DshWorkspace & { title: string }> = [];
  return {
    async create(path, title) {
      const existing = items.find(item => item.path === path);
      if (existing !== undefined) return existing;
      const workspace: DshWorkspace & { title: string } = {
        id: `ws-${items.length + 1}`,
        path,
        title: title ?? "untitled",
        async setTitle(next) {
          workspace.title = next;
        }
      };
      items.unshift(workspace);
      return workspace;
    },
    async resolveByPath(path) {
      return items.find(item => item.path === path);
    },
    get(id) {
      return items.find(item => item.id === id);
    },
    async delete() {
      return false;
    },
    list() {
      return [...items];
    }
  };
};

const depsOf = (
  registry: DshWorkspaceRegistry,
  ingested: JsonValue[] = [],
  removed: string[] = []
): ParentBridgeDeps => ({
  registry,
  ingestContribution: payload => {
    ingested.push(payload);
  },
  removeSurface: ref => {
    removed.push(ref);
  }
});

describe("parent-bridge inbound", () => {
  const previousRoot = process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
  afterEach(() => {
    resetParentBridgeState();
    if (previousRoot === undefined) delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
    else process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = previousRoot;
  });

  it("pins a workspace from parent-hello and workspace.bind", async () => {
    const root = await mkdtemp(join(tmpdir(), "muse-parent-bridge-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    const registry = fakeRegistry();
    const deps = depsOf(registry);
    const hello = await handleParentInbound({
      source: "muse.appflowy-web",
      type: "parent-hello",
      workspaceRef: "alpha-ws",
      workspaceTitle: "Alpha"
    }, deps);
    expect(hello).toEqual({ ok: true, bound: "alpha-ws" });
    expect(getLastWorkspaceHint()?.appflowyWorkspaceId).toBe("alpha-ws");
    const rebound = await handleParentInbound({
      source: "muse.appflowy-web",
      type: "workspace.bind",
      workspaceRef: "beta-ws",
      workspaceTitle: "Beta"
    }, deps);
    expect(rebound).toEqual({ ok: true, bound: "beta-ws" });
    expect(getLastWorkspaceHint()?.title).toBe("Beta");
  });

  it("ignores hello without workspaceRef and does not overwrite an existing pin", async () => {
    const root = await mkdtemp(join(tmpdir(), "muse-parent-bridge-keep-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    const registry = fakeRegistry();
    await applyWorkspaceHint(registry, { appflowyWorkspaceId: "kept", title: "Kept" });
    const result = await handleParentInbound({
      source: "muse.appflowy-web",
      type: "parent-hello",
      theme: "dark"
    }, depsOf(registry));
    expect(result).toEqual({ ok: true });
    expect(getLastWorkspaceHint()?.appflowyWorkspaceId).toBe("kept");
  });

  it("rejects oversized payloads, forbidden fields, and unknown sources", async () => {
    const registry = fakeRegistry();
    const deps = depsOf(registry);
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "workspace.bind",
      workspaceRef: "x".repeat(40_000)
    }, deps)).toMatchObject({ ok: false, error: "PAYLOAD_TOO_LARGE" });
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "workspace.bind",
      workspaceRef: "ok",
      access_token: "steal"
    }, deps)).toMatchObject({ ok: false, error: "FORBIDDEN_FIELD" });
    expect(await handleParentInbound({
      source: "evil",
      type: "workspace.bind",
      workspaceRef: "ok"
    }, deps)).toMatchObject({ ok: false, error: "INVALID_SOURCE" });
    expect(jsonLooksForbidden(JSON.stringify({ refresh_token: "nope" }))).toBe(true);
  });

  it("forwards context.contribute envelopes to the broker", async () => {
    const ingested: JsonValue[] = [];
    const envelope = {
      protocol: "muse.context-contribution/v1",
      pluginId: "muse.appflowy.workspace",
      pluginVersion: "1.0.0",
      facetInstanceRef: "facet.1",
      surfaceInstanceRef: "surface.1",
      surfaceKind: "appflowy.workspace",
      scopeRef: "workspace.1",
      contextType: "workspace.focus",
      contextSchemaDigest: "sha256:4c3a6bf1cd8249cc96f6aac63ad78b9be04172634f0313ce3ba52163f66043b7",
      contextRevision: "1",
      epochRef: "epoch.1",
      lane: "control",
      capturedAt: 1,
      expiresAt: Date.now() + 60_000,
      payload: { workspaceId: "w1", viewId: "9ffadd30-5a73-4a3c-9caf-7c8e191f8b65" }
    };
    const result = await handleParentInbound({
      source: "muse.appflowy-web",
      type: "context.contribute",
      envelope
    }, depsOf(fakeRegistry(), ingested));
    expect(result).toEqual({ ok: true });
    expect(ingested).toEqual([envelope]);
    expect(getLastDocumentFocus()).toEqual({
      workspaceId: "w1",
      viewId: "9ffadd30-5a73-4a3c-9caf-7c8e191f8b65"
    });
  });

  it("stores a two-segment device token from parent-hello without echoing it", async () => {
    const result = await handleParentInbound({
      source: "muse.appflowy-web",
      type: "parent-hello",
      deviceToken: "aaa.bbb",
      deviceId: "web.1"
    }, depsOf(fakeRegistry()));
    expect(result).toEqual({ ok: true });
    expect(JSON.stringify(result)).not.toContain("aaa.bbb");
    expect(getLastDeviceAuth()).toEqual({ token: "aaa.bbb", deviceId: "web.1" });
  });

  it("injects a script without raw < in the body", () => {
    const body = PARENT_BRIDGE_SCRIPT.slice("<script>".length, -"</script>".length);
    expect(body).not.toContain("<");
    expect(body.toLowerCase()).not.toContain("</script");
    const html = injectParentBridgeScript("<head></head>");
    expect(html.startsWith("<head>" + PARENT_BRIDGE_SCRIPT)).toBe(true);
    expect(body).toContain("frame-ready");
    expect(body).toContain("muse.dsh-web");
  });
});

describe("parent-bridge workspace views list", () => {
  const previousRoot = process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
  const previousCloud = process.env.MUSE_DOCUMENT_CLOUD_URL;
  afterEach(() => {
    resetParentBridgeState();
    if (previousRoot === undefined) delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
    else process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = previousRoot;
    if (previousCloud === undefined) delete process.env.MUSE_DOCUMENT_CLOUD_URL;
    else process.env.MUSE_DOCUMENT_CLOUD_URL = previousCloud;
  });

  it("parses cursor/limit/depth and ignores empty ids", () => {
    expect(parseWorkspaceViewsQuery("/muse/v1/workspace/views?limit=99&depth=9&cursor=v1")).toEqual({
      cursor: "v1",
      limit: 64,
      depth: 4
    });
  });

  it("returns NO_WORKSPACE then NO_DEVICE_TOKEN before calling Cloud", async () => {
    expect(await listBoundWorkspaceViews()).toEqual({ ok: false, status: 409, error: "NO_WORKSPACE" });
    const root = await mkdtemp(join(tmpdir(), "muse-views-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    await handleParentInbound({
      source: "muse.appflowy-web",
      type: "parent-hello",
      workspaceRef: "alpha-ws",
      workspaceTitle: "Alpha"
    }, depsOf(fakeRegistry()));
    expect(await listBoundWorkspaceViews()).toEqual({ ok: false, status: 401, error: "NO_DEVICE_TOKEN" });
  });

  it("lists Cloud folder views for the bound workspace and never returns the device token", async () => {
    const root = await mkdtemp(join(tmpdir(), "muse-views-ok-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    process.env.MUSE_DOCUMENT_CLOUD_URL = "http://cloud.test";
    await handleParentInbound({
      source: "muse.appflowy-web",
      type: "parent-hello",
      workspaceRef: "alpha-ws",
      workspaceTitle: "Alpha",
      deviceToken: "aaa.bbb",
      deviceId: "web.1"
    }, depsOf(fakeRegistry()));
    const result = await listBoundWorkspaceViews({ limit: 32, depth: 3 }, {
      fetchImpl: (async (input, init) => {
        expect(String(input)).toBe("http://cloud.test/api/muse/workspace/tree");
        const headers = init?.headers as Record<string, string>;
        expect(headers.Authorization).toBe("Bearer aaa.bbb");
        expect(headers["X-Muse-Device-Id"]).toBe("web.1");
        const payload = JSON.parse(String(init?.body)) as Record<string, unknown>;
        expect(payload).toMatchObject({
          operation: "workspace.tree.query",
          workspaceId: "alpha-ws",
          limit: 32,
          depth: 3
        });
        return new Response(JSON.stringify({
          code: 0,
          data: {
            protocol: "muse.workspace/tree/v1",
            workspaceId: "alpha-ws",
            rootViewId: "root",
            truncated: false,
            items: [{
              viewId: "v1",
              parentViewId: null,
              title: "Getting started",
              layout: "document",
              isSpace: false,
              depth: 0
            }]
          }
        }), { status: 200, headers: { "Content-Type": "application/json" } });
      }) as typeof fetch
    });
    expect(result.ok).toBe(true);
    if (!result.ok) throw new Error("expected ok");
    expect(JSON.stringify(result.body)).not.toContain("aaa.bbb");
    expect(result.body).toMatchObject({
      ok: true,
      source: "cloud.folder",
      tree: {
        protocol: "muse.workspace/tree/v1",
        workspaceId: "alpha-ws",
        items: [{ viewId: "v1", title: "Getting started" }]
      }
    });
    const mismatch = await listBoundWorkspaceViews({ workspaceId: "other-ws" });
    expect(mismatch).toEqual({ ok: false, status: 403, error: "SCOPE_MISMATCH" });
  });
});

describe("parent-bridge is Web Tx only", () => {
  const previousFlag = process.env.MUSE_PARENT_BRIDGE;
  const previousCloud = process.env.MUSE_DOCUMENT_CLOUD_URL;
  afterEach(() => {
    if (previousFlag === undefined) delete process.env.MUSE_PARENT_BRIDGE;
    else process.env.MUSE_PARENT_BRIDGE = previousFlag;
    if (previousCloud === undefined) delete process.env.MUSE_DOCUMENT_CLOUD_URL;
    else process.env.MUSE_DOCUMENT_CLOUD_URL = previousCloud;
  });

  it("stays off on Desktop (no cloud URL) and on when Remote Cloud URL is set", () => {
    delete process.env.MUSE_PARENT_BRIDGE;
    delete process.env.MUSE_DOCUMENT_CLOUD_URL;
    expect(parentBridgeEnabled()).toBe(false);
    process.env.MUSE_DOCUMENT_CLOUD_URL = "http://cloud.test";
    expect(parentBridgeEnabled()).toBe(true);
    process.env.MUSE_PARENT_BRIDGE = "0";
    expect(parentBridgeEnabled()).toBe(false);
    process.env.MUSE_PARENT_BRIDGE = "1";
    delete process.env.MUSE_DOCUMENT_CLOUD_URL;
    expect(parentBridgeEnabled()).toBe(true);
  });
});
