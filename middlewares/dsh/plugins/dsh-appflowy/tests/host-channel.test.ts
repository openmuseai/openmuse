import { afterEach, describe, expect, it, vi } from "vitest";
import { mkdtemp } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  handleParentInbound,
  hostAuthRequired,
  resetParentBridgeState
} from "../src/parent-bridge.js";
import { HOST_VERIFY_TTL_MS, verifyHostWorkspace } from "../src/mobile-lease.js";
import type { DshWorkspace, DshWorkspaceRegistry } from "@muse/plugin-appflowy-workspace";

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

describe("P0 host channel", () => {
  const saved = { ...process.env };
  afterEach(() => {
    resetParentBridgeState();
    vi.restoreAllMocks();
    for (const key of ["MUSE_DOCUMENT_CLOUD_URL", "MUSE_REQUIRE_HOST_AUTH", "MUSE_WEB_WORKSPACE_BIND", "MUSE_APPFLOWY_DSH_WORKSPACE_ROOT"]) {
      if (saved[key] === undefined) delete process.env[key];
      else process.env[key] = saved[key];
    }
  });

  it("defaults host auth on when Cloud URL is set and off on desktop", () => {
    delete process.env.MUSE_REQUIRE_HOST_AUTH;
    delete process.env.MUSE_DOCUMENT_CLOUD_URL;
    expect(hostAuthRequired()).toBe(false);
    process.env.MUSE_DOCUMENT_CLOUD_URL = "https://cloud.example";
    expect(hostAuthRequired()).toBe(true);
    process.env.MUSE_REQUIRE_HOST_AUTH = "0";
    expect(hostAuthRequired()).toBe(false);
  });

  it("E1-T2 rejects bind without a device token when host auth is required", async () => {
    const root = await mkdtemp(join(tmpdir(), "muse-host-auth-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    process.env.MUSE_DOCUMENT_CLOUD_URL = "http://cloud.test";
    const result = await handleParentInbound({
      source: "muse.appflowy-web",
      type: "workspace.bind",
      workspaceRef: "ws-1"
    }, {
      registry: fakeRegistry(),
      ingestContribution: () => undefined,
      removeSurface: () => undefined
    });
    expect(result).toEqual({ ok: false, error: "NO_DEVICE_TOKEN" });
  });

  it("rejects forged token, wrong workspace, and pins only after Cloud membership", async () => {
    const root = await mkdtemp(join(tmpdir(), "muse-host-ok-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    process.env.MUSE_DOCUMENT_CLOUD_URL = "http://cloud.test";
    const fetchMock = vi.spyOn(globalThis, "fetch").mockImplementation(async (input, init) => {
      expect(String(input)).toBe("http://cloud.test/api/muse/workspace/current");
      const body = JSON.parse(String(init?.body)) as { workspaceId: string };
      const token = new Headers(init?.headers).get("authorization");
      if (token !== "Bearer good.token") {
        return new Response("", { status: 401 });
      }
      if (body.workspaceId !== "ws-owned") {
        return Response.json({ code: 1, message: "SCOPE_MISMATCH" }, { status: 200 });
      }
      return Response.json({ code: 0, data: { workspaceId: "ws-owned" } });
    });
    const deps = {
      registry: fakeRegistry(),
      ingestContribution: () => undefined,
      removeSurface: () => undefined
    };
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "parent-hello",
      workspaceRef: "ws-owned",
      deviceToken: "evil.token",
      deviceId: "web.1"
    }, deps)).toEqual({ ok: false, error: "DEVICE_AUTH_REJECTED" });
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "workspace.bind",
      workspaceRef: "ws-other",
      deviceToken: "good.token",
      deviceId: "web.1"
    }, deps)).toEqual({ ok: false, error: "SCOPE_MISMATCH" });
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "parent-hello",
      workspaceRef: "ws-owned",
      workspaceTitle: "Owned",
      deviceToken: "good.token",
      deviceId: "web.1"
    }, deps)).toEqual({ ok: true, bound: "ws-owned" });
    const before = fetchMock.mock.calls.length;
    await verifyHostWorkspace({ token: "good.token", deviceId: "web.1" }, "ws-owned");
    expect(fetchMock.mock.calls.length).toBe(before);
    await verifyHostWorkspace(
      { token: "good.token", deviceId: "web.1" },
      "ws-owned",
      Date.now() + HOST_VERIFY_TTL_MS + 1
    );
    expect(fetchMock.mock.calls.length).toBe(before + 1);
  });

  it("E1-T1 pins hello with a valid device token", async () => {
    const root = await mkdtemp(join(tmpdir(), "muse-host-t1-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    process.env.MUSE_DOCUMENT_CLOUD_URL = "http://cloud.test";
    vi.spyOn(globalThis, "fetch").mockResolvedValue(
      Response.json({ code: 0, data: { workspaceId: "ws-1" } })
    );
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "parent-hello",
      workspaceRef: "ws-1",
      deviceToken: "good.token",
      deviceId: "web.1"
    }, {
      registry: fakeRegistry(),
      ingestContribution: () => undefined,
      removeSurface: () => undefined
    })).toEqual({ ok: true, bound: "ws-1" });
  });

  it("E1-T3 bind carries its own token and does not need a prior hello", async () => {
    resetParentBridgeState();
    const root = await mkdtemp(join(tmpdir(), "muse-host-t3-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    process.env.MUSE_DOCUMENT_CLOUD_URL = "http://cloud.test";
    vi.spyOn(globalThis, "fetch").mockResolvedValue(
      Response.json({ code: 0, data: { workspaceId: "ws-1" } })
    );
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "workspace.bind",
      workspaceRef: "ws-1",
      deviceToken: "good.token",
      deviceId: "web.1"
    }, {
      registry: fakeRegistry(),
      ingestContribution: () => undefined,
      removeSurface: () => undefined
    })).toEqual({ ok: true, bound: "ws-1" });
  });

  it("E1-T11/T12 maps forged token and cross-workspace token", async () => {
    const root = await mkdtemp(join(tmpdir(), "muse-host-t11-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    process.env.MUSE_DOCUMENT_CLOUD_URL = "http://cloud.test";
    vi.spyOn(globalThis, "fetch").mockImplementation(async (input, init) => {
      const body = JSON.parse(String(init?.body)) as { workspaceId: string };
      const token = new Headers(init?.headers).get("authorization");
      if (token !== "Bearer good.token") return new Response("", { status: 401 });
      if (body.workspaceId !== "ws-owned") {
        return Response.json({ code: 1, message: "SCOPE_MISMATCH" }, { status: 200 });
      }
      return Response.json({ code: 0, data: { workspaceId: "ws-owned" } });
    });
    const deps = {
      registry: fakeRegistry(),
      ingestContribution: () => undefined,
      removeSurface: () => undefined
    };
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "workspace.bind",
      workspaceRef: "ws-owned",
      deviceToken: "evil.token",
      deviceId: "web.1"
    }, deps)).toEqual({ ok: false, error: "DEVICE_AUTH_REJECTED" });
    expect(await handleParentInbound({
      source: "muse.appflowy-web",
      type: "workspace.bind",
      workspaceRef: "ws-other",
      deviceToken: "good.token",
      deviceId: "web.1"
    }, deps)).toEqual({ ok: false, error: "SCOPE_MISMATCH" });
  });
});
