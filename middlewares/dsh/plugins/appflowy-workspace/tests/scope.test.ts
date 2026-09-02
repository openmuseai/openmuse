import { describe, expect, it } from "vitest";
import { assertWorkspaceScope } from "../src/scope.js";

describe("assertWorkspaceScope", () => {
  it("allows matching or missing ids", () => {
    expect(assertWorkspaceScope("ws-1", { workspaceId: "ws-1" })).toEqual({ ok: true });
    expect(assertWorkspaceScope("ws-1", {})).toEqual({ ok: true });
    expect(assertWorkspaceScope(undefined, { workspaceId: "ws-2" })).toEqual({ ok: true });
  });

  it("rejects a bound/invoke mismatch", () => {
    const result = assertWorkspaceScope("ws-1", { workspaceId: "ws-2" });
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected mismatch");
    expect(result.code).toBe("SCOPE_MISMATCH");
    expect(result.message).toContain("SCOPE_MISMATCH");
  });
});
