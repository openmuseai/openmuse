import { afterEach, describe, expect, it } from "vitest";
import {
  parseWorkspaceCatalogPayload,
  projectBoundCatalogTree,
  rememberWorkspaceCatalog,
  rememberWorkspaceCatalogFromEnvelope,
  resetLastWorkspaceCatalog,
  WORKSPACE_CATALOG_CONTEXT_TYPE,
  WORKSPACE_CATALOG_DIGEST,
  WORKSPACE_TREE_PROTOCOL
} from "../src/catalog.js";
import { createCloudWorkspaceProvider, resolveWorkspaceTree } from "../src/host.js";
import { listBoundWorkspaceViews } from "../src/views.js";
import { interpretCloudWorkspaceResponse } from "../src/cloud.js";
import { applyWorkspaceHint, resetLastWorkspaceHint, type DshWorkspace, type DshWorkspaceRegistry } from "../src/identity.js";
import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

const catalog = {
  workspaceId: "ws-1",
  truncated: false,
  items: [{
    viewId: "v1",
    title: "Getting started",
    layout: "document" as const,
    isSpace: false,
    depth: 0,
    parentViewId: null
  }, {
    viewId: "v2",
    title: "Notes",
    layout: "document" as const,
    isSpace: false,
    depth: 1,
    parentViewId: "v1"
  }]
};

const envelopeOf = (payload: unknown) => ({
  protocol: "muse.context-contribution/v1",
  pluginId: "muse.appflowy.workspace",
  contextType: WORKSPACE_CATALOG_CONTEXT_TYPE,
  contextSchemaDigest: WORKSPACE_CATALOG_DIGEST,
  payload
});

const fakeRegistry = (): DshWorkspaceRegistry => {
  const items: Array<DshWorkspace & { title: string }> = [];
  return {
    async create(path, title) {
      const workspace: DshWorkspace & { title: string } = {
        id: `ws-${items.length + 1}`,
        path,
        title: title ?? "untitled"
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

describe("workspace catalog (Host → DSH Facet)", () => {
  afterEach(() => {
    resetLastWorkspaceHint();
    resetLastWorkspaceCatalog();
  });

  it("parses a bounded catalog and projects tree items", () => {
    const parsed = parseWorkspaceCatalogPayload(catalog);
    expect(parsed?.items).toHaveLength(2);
    rememberWorkspaceCatalog(parsed!);
    const tree = projectBoundCatalogTree("ws-1", { workspaceId: "ws-1", depth: 4 });
    expect(tree).toMatchObject({
      protocol: WORKSPACE_TREE_PROTOCOL,
      workspaceId: "ws-1",
      truncated: false,
      items: [{ viewId: "v1", title: "Getting started" }, { viewId: "v2", title: "Notes" }]
    });
  });

  it("rejects catalog payloads that contain secret substrings", () => {
    expect(parseWorkspaceCatalogPayload({
      ...catalog,
      items: [{ ...catalog.items[0], title: "leaked access_token value" }]
    })).toBeUndefined();
  });

  it("remembers catalog from a contribute envelope", () => {
    expect(rememberWorkspaceCatalogFromEnvelope(envelopeOf(catalog))).toBe(true);
    expect(projectBoundCatalogTree("ws-1", {})).toMatchObject({ workspaceId: "ws-1" });
  });

  it("maps Cloud NOT_FOUND to CLOUD_COLLAB_ADAPTER_NOT_WIRED", () => {
    const result = interpretCloudWorkspaceResponse({ code: 1, message: "NOT_FOUND" });
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected failure");
    expect(result.code).toBe("UNAVAILABLE");
    expect(result.message).toContain("CLOUD_COLLAB_ADAPTER_NOT_WIRED");
    expect(result.message).not.toBe("NOT_FOUND");
  });

  it("falls back to Host catalog when Cloud tree is unwired", async () => {
    rememberWorkspaceCatalog(catalog);
    const fetchImpl = (async () =>
      new Response(JSON.stringify({ code: 1, message: "NOT_FOUND" }), { status: 404 })
    ) as unknown as typeof fetch;
    const tree = await resolveWorkspaceTree({
      boundWorkspaceId: "ws-1",
      input: { workspaceId: "ws-1" },
      cloudBaseUrl: "http://cloud.test",
      fetchImpl
    });
    expect(tree.ok).toBe(true);
    if (!tree.ok) throw new Error("expected ok");
    expect(tree.source).toBe("host.catalog");
    expect(JSON.stringify(tree.value)).toContain("Getting started");
  });

  it("does not fall back on SCOPE_MISMATCH", async () => {
    rememberWorkspaceCatalog(catalog);
    const tree = await resolveWorkspaceTree({
      boundWorkspaceId: "ws-1",
      input: { workspaceId: "other" },
      cloudBaseUrl: "http://cloud.test"
    });
    expect(tree.ok).toBe(false);
    if (tree.ok) throw new Error("expected failure");
    expect(tree.code).toBe("SCOPE_MISMATCH");
  });

  it("E4-T5 lists host.catalog when Cloud returns 501", async () => {
    const previousRoot = process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
    const root = await mkdtemp(join(tmpdir(), "muse-catalog-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    try {
      await applyWorkspaceHint(fakeRegistry(), { appflowyWorkspaceId: "ws-1", title: "Docs" });
      rememberWorkspaceCatalog(catalog);
      const listed = await listBoundWorkspaceViews({}, {
        auth: { token: "aaa.bbb", deviceId: "web.1" },
        cloudBaseUrl: "http://cloud.test",
        fetchImpl: (async () =>
          new Response(
            JSON.stringify({ code: 1067, message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" }),
            { status: 501 }
          )
        ) as typeof fetch
      });
      expect(listed.ok).toBe(true);
      if (!listed.ok) throw new Error("expected ok");
      expect(listed.body).toMatchObject({ source: "host.catalog" });
      expect(JSON.stringify(listed.body)).toContain("Getting started");
    } finally {
      if (previousRoot === undefined) delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
      else process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = previousRoot;
    }
  });

  it("clears catalog when the bound AppFlowy workspace changes", async () => {
    const previousRoot = process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
    const root = await mkdtemp(join(tmpdir(), "muse-catalog-switch-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    try {
      const registry = fakeRegistry();
      await applyWorkspaceHint(registry, { appflowyWorkspaceId: "ws-1", title: "A" });
      rememberWorkspaceCatalog(catalog);
      await applyWorkspaceHint(registry, { appflowyWorkspaceId: "ws-2", title: "B" });
      expect(projectBoundCatalogTree("ws-1", {})).toBeUndefined();
      expect(projectBoundCatalogTree("ws-2", {})).toBeUndefined();
    } finally {
      if (previousRoot === undefined) delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
      else process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = previousRoot;
    }
  });

  it("keeps Cloud tree when the adapter returns items", async () => {
    rememberWorkspaceCatalog(catalog);
    const cloudTree = {
      protocol: WORKSPACE_TREE_PROTOCOL,
      workspaceId: "ws-1",
      rootViewId: "ws-1",
      truncated: false,
      items: [{ viewId: "cloud-1", title: "From collab", layout: "document", isSpace: false, depth: 0 }]
    };
    const tree = await resolveWorkspaceTree({
      boundWorkspaceId: "ws-1",
      input: { workspaceId: "ws-1" },
      cloudBaseUrl: "http://cloud.test",
      fetchImpl: (async () =>
        new Response(JSON.stringify({ code: 0, data: cloudTree }), { status: 200 })
      ) as typeof fetch
    });
    expect(tree.ok).toBe(true);
    if (!tree.ok) throw new Error("expected ok");
    expect(tree.source).toBe("cloud.folder");
    expect(JSON.stringify(tree.value)).toContain("From collab");
  });

  it("provider tree query uses catalog when Cloud is missing", async () => {
    rememberWorkspaceCatalog(catalog);
    const provider = createCloudWorkspaceProvider();
    const result = await provider.invoke({
      operationId: "workspace.tree.query",
      input: { workspaceId: "ws-1" },
      ctx: { boundWorkspaceId: "ws-1" }
    });
    expect(result.ok).toBe(true);
    if (!result.ok) throw new Error("expected ok");
    expect(JSON.stringify(result.value)).toContain("Getting started");
  });
});
