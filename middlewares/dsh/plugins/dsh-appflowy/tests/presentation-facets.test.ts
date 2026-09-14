import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";
import { createPresentationFacetInbox } from "../src/presentation-facets.js";
import { ExclusiveMobileLease, SharedHostSession } from "../src/mobile-lease.js";

const fixture = JSON.parse(readFileSync(new URL("../fixtures/presentation-v1.json", import.meta.url), "utf8"));

describe("transport independent Facet dispatch", () => {
  it("accepts the same native/Web context unchanged", () => {
    const contribute = vi.fn();
    const inbox = createPresentationFacetInbox({ contribute, rememberFocus: vi.fn() });
    inbox.dispatch(fixture.context, 2000);
    expect(contribute).toHaveBeenCalledWith(fixture.context);
  });
  it("rejects an unknown facet, schema drift and expired context before business execution", () => {
    const contribute = vi.fn();
    const inbox = createPresentationFacetInbox({ contribute, rememberFocus: vi.fn() });
    expect(() => inbox.dispatch({ ...fixture.context, pluginId: "foreign.plugin" }, 2000)).toThrow("FACET_NOT_FOUND");
    expect(() => inbox.dispatch({ ...fixture.context, contextSchemaDigest: `sha256:${"0".repeat(64)}` }, 2000)).toThrow("SCHEMA_DIGEST_MISMATCH");
    expect(() => inbox.dispatch(fixture.context, 10000)).toThrow("CONTEXT_EXPIRED");
    expect(contribute).not.toHaveBeenCalled();
  });
});

describe("exclusive test carrier lease (NOT multi-tenant authentication)", () => {
  const a = { token: "payload.signature-a", deviceId: "mobile.a", connectionId: "a" };
  const b = { token: "payload.signature-b", deviceId: "mobile.b", connectionId: "b" };
  it("rejects a second controller and workspace switch without overwriting credentials", async () => {
    const clearContext = vi.fn();
    const lease = new ExclusiveMobileLease({ verify: async () => {}, clearContext });
    try {
      await lease.authorize(a, "ws-1");
      await expect(lease.authorize(b, "ws-1")).rejects.toThrow("HOST_IN_USE");
      await expect(lease.authorize(a, "ws-2")).rejects.toThrow("SCOPE_MISMATCH");
      expect(lease.matches(a)).toBe(true);
      expect(clearContext).toHaveBeenCalledTimes(1);
    } finally { lease.release(); }
  });
  it("reserves the slot during async Cloud validation", async () => {
    let finish!: () => void;
    const lease = new ExclusiveMobileLease({ verify: () => new Promise<void>(resolve => { finish = resolve; }), clearContext: vi.fn() });
    const first = lease.authorize(a, "ws-1");
    expect(lease.occupied).toBe(true);
    await expect(lease.authorize(b, "ws-1")).rejects.toThrow("HOST_IN_USE");
    finish(); await first; lease.release();
  });
  it("expires and clears stale context; never accepts failed Cloud verification", async () => {
    let now = 1000;
    const clearContext = vi.fn();
    const verify = vi.fn(async () => {});
    const lease = new ExclusiveMobileLease({ verify, clearContext, clock: () => now });
    await lease.authorize(a, "ws-1");
    now += 61000;
    expect(lease.matches(a)).toBe(false);
    expect(clearContext).toHaveBeenCalledTimes(2);
    verify.mockRejectedValueOnce(new Error("revoked"));
    await expect(lease.authorize(a, "ws-1")).rejects.toThrow("DEVICE_AUTH_REJECTED");
    expect(lease.occupied).toBe(false);
  });

  it("E2-T12 two instance-local leases do not 409 each other", async () => {
    const tenantA = new ExclusiveMobileLease({ verify: async () => {}, clearContext: vi.fn() });
    const tenantB = new ExclusiveMobileLease({ verify: async () => {}, clearContext: vi.fn() });
    try {
      await tenantA.authorize(a, "ws-a");
      await tenantB.authorize(b, "ws-b");
      expect(tenantA.workspaceId).toBe("ws-a");
      expect(tenantB.workspaceId).toBe("ws-b");
    } finally {
      tenantA.release();
      tenantB.release();
    }
  });
});

describe("shared host session (Web + Mobile on one tenant instance)", () => {
  const a = { token: "payload.signature-a", deviceId: "mobile.a", connectionId: "a" };
  const b = { token: "payload.signature-b", deviceId: "mobile.b", connectionId: "b" };

  it("attaches a second Host on the same workspace without HOST_IN_USE", async () => {
    const session = new SharedHostSession({ verify: async () => {} });
    await session.authorize(a, "ws-1");
    await session.authorize(b, "ws-1");
    expect(session.size).toBe(2);
    expect(session.matches(a)).toBe(true);
    expect(session.matches(b)).toBe(true);
    session.detach(a);
    expect(session.matches(b)).toBe(true);
    expect(session.workspaceId).toBe("ws-1");
    session.detach(b);
    expect(session.occupied).toBe(false);
    expect(session.workspaceId).toBe("ws-1");
  });

  it("rejects a second Host that pins a different workspace", async () => {
    const session = new SharedHostSession({ verify: async () => {} });
    await session.authorize(a, "ws-1");
    await expect(session.authorize(b, "ws-2")).rejects.toThrow("SCOPE_MISMATCH");
    expect(session.matches(a)).toBe(true);
  });

  it("rejects replacing the workspace pin while another Host is attached", async () => {
    const session = new SharedHostSession({ verify: async () => {} });
    await session.authorize(a, "ws-1");
    expect(() => session.pinWorkspace("ws-2", { replace: true })).toThrow("SCOPE_MISMATCH");
    session.detach(a);
    expect(() => session.pinWorkspace("ws-2", { replace: true })).not.toThrow();
    expect(session.workspaceId).toBe("ws-2");
  });
});
