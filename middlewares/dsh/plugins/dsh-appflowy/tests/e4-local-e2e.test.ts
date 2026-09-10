import { createServer } from "node:http";
import { afterEach, describe, expect, it } from "vitest";
import type { JsonValue } from "@muse/host-bridge";
import { InProcessCompositionHandler } from "../src/composition-host.js";
import { handleParentInbound, resetParentBridgeState, type ParentBridgeDeps } from "../src/parent-bridge.js";
import { createCloudWorkspaceProvider } from "@muse/plugin-appflowy-workspace/host";
import { createCloudMarkdownProvider } from "@muse/plugin-appflowy-markdown/host";
import {
  rememberWorkspaceCatalog,
  resetLastWorkspaceCatalog,
  type DshWorkspace,
  type DshWorkspaceRegistry
} from "@muse/plugin-appflowy-workspace";
import { rememberMarkdownSnapshot, resetLastMarkdownSnapshot } from "@muse/plugin-appflowy-markdown";
import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

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

const depsOf = (registry: DshWorkspaceRegistry): ParentBridgeDeps => ({
  registry,
  ingestContribution: () => undefined,
  removeSurface: () => undefined
});

const listenUnwired = async (): Promise<{ base: string; close: () => void }> => {
  const server = createServer((_req, res) => {
    res.writeHead(501, { "content-type": "application/json" });
    res.end(JSON.stringify({ code: 1067, message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" }));
  });
  await new Promise<void>(resolve => server.listen(0, "127.0.0.1", resolve));
  const addr = server.address();
  if (addr === null || typeof addr === "string") throw new Error("listen");
  return {
    base: `http://127.0.0.1:${addr.port}`,
    close: () => server.close()
  };
};

const invoke = async (
  handler: InProcessCompositionHandler,
  bindingId: string,
  operationId: string,
  input: JsonValue
) => {
  const reply = await handler.unary({
    protocol: "muse-bridge",
    major: 1,
    minor: 0,
    kind: "invoke.request",
    requestId: "request.e4" as never,
    sentAt: Date.now(),
    payload: { traceId: "trace.e4", bindingId, operationId, input } as unknown as JsonValue
  });
  return reply.payload as {
    ok?: boolean;
    value?: JsonValue;
    error?: { code?: string; message?: string };
  };
};

describe("E4 local end-to-end (Host catalog/snapshot → Cloud 501 → tools)", () => {
  const previousRoot = process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
  afterEach(() => {
    resetParentBridgeState();
    resetLastWorkspaceCatalog();
    resetLastMarkdownSnapshot();
    if (previousRoot === undefined) delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
    else process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = previousRoot;
  });

  it("lists Host sidebar titles when BFF tree is unwired", async () => {
    const root = await mkdtemp(join(tmpdir(), "muse-e4-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    const { base, close } = await listenUnwired();
    try {
      const bind = await handleParentInbound({
        source: "muse.appflowy-web",
        type: "parent-hello",
        workspaceRef: "ws-1",
        workspaceTitle: "Docs",
        deviceToken: "aaa.bbb",
        deviceId: "web.1"
      }, depsOf(fakeRegistry()));
      expect(bind).toMatchObject({ ok: true });

      rememberWorkspaceCatalog({
        workspaceId: "ws-1",
        truncated: false,
        items: [{
          viewId: "view-1",
          title: "Getting started",
          layout: "document",
          isSpace: false,
          depth: 0,
          parentViewId: null
        }]
      });

      const handler = new InProcessCompositionHandler(
        [createCloudMarkdownProvider(), createCloudWorkspaceProvider()],
        {
          cloudBaseUrl: base,
          slots: {
            boundWorkspaceId: () => "ws-1",
            deviceAuth: () => ({ token: "aaa.bbb", deviceId: "web.1" })
          }
        }
      );
      const listed = await invoke(handler, "binding.appflowy-workspace", "workspace.tree.query", {
        workspaceId: "ws-1"
      });
      expect(listed.ok).toBe(true);
      expect(JSON.stringify(listed.value)).toContain("Getting started");
      expect(JSON.stringify(listed.value)).not.toContain("NOT_FOUND");
      expect(JSON.stringify(listed.value)).not.toContain("text/markdown");
    } finally {
      close();
    }
  });

  it("read without focus is NO_CURRENT_SELECTION; matching snapshot serves current page", async () => {
    const { base, close } = await listenUnwired();
    try {
      const handler = new InProcessCompositionHandler(
        [createCloudMarkdownProvider(), createCloudWorkspaceProvider()],
        { cloudBaseUrl: base }
      );
      const missing = await invoke(handler, "binding.appflowy-markdown", "document.current.query", {});
      expect(missing.ok).toBe(false);
      expect(missing.error?.code).toBe("NO_CURRENT_SELECTION");
      expect(missing.error?.message).not.toContain("NOT_FOUND");

      rememberMarkdownSnapshot({
        viewId: "view-1",
        workspaceId: "ws-1",
        text: "# Getting started\n\nHello from Host.",
        truncated: false,
        byteLength: Buffer.byteLength("# Getting started\n\nHello from Host.")
      });
      const withFocus = new InProcessCompositionHandler(
        [createCloudMarkdownProvider(), createCloudWorkspaceProvider()],
        {
          cloudBaseUrl: base,
          slots: {
            documentFocus: () => ({ workspaceId: "ws-1", viewId: "view-1" })
          }
        }
      );
      const read = await invoke(withFocus, "binding.appflowy-markdown", "document.current.query", {});
      expect(read.ok).toBe(true);
      expect(JSON.stringify(read.value)).toContain("Hello from Host.");
      expect(JSON.stringify(read.value)).not.toContain("NOT_FOUND");
    } finally {
      close();
    }
  });
});
