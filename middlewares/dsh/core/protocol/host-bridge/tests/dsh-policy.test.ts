import { describe, expect, it } from "vitest";
import type { MuseHostRequestKind } from "../src/dsh/index.js";
import { resolveMusePolicy, type MuseApprovalProofRequester, type MuseHostServiceApi } from "../src/dsh/index.js";

const host = (decision: "allow" | "deny" | "approval_required"): MuseHostServiceApi => ({
  snapshot: () => ({ state: "ready" }), connect: async () => ({} as never), invoke: async () => ({} as never), close: async () => undefined,
  request: async (kind: MuseHostRequestKind) => kind === "policy.evaluate.request"
    ? ({ kind: "policy.evaluate.response", payload: { ok: true, value: { policyDecisionId: "policy.1", decision, expiresAt: 99, ...(decision === "approval_required" ? { approvalId: "approval.1" } : {}) } } } as never)
    : ({ kind: "policy.finalize.response", payload: { ok: true, value: { policyDecisionId: "policy.1", outcome: "approved", grantId: "grant.1", expiresAt: 99 } } } as never)
});
const request = { traceId: "trace.1", bindingId: "binding.1", operationId: "view.rename.apply", inputDigest: "sha256:test", effect: "local_write", deadlineAt: 99 } as never;

describe("resolveMusePolicy", () => {
  it("does not ask or finalize a direct allow", async () => {
    const approval: MuseApprovalProofRequester = { request: async () => { throw new Error("must not ask"); } };
    await expect(resolveMusePolicy(host("allow"), request, approval, new AbortController().signal)).resolves.toMatchObject({ kind: "allow" });
  });
  it("finalizes only a trusted approved proof", async () => {
    const approval: MuseApprovalProofRequester = { request: async () => ({ outcome: "approved", proofId: "proof.1" }) };
    await expect(resolveMusePolicy(host("approval_required"), request, approval, new AbortController().signal)).resolves.toMatchObject({ kind: "approved", grantId: "grant.1" });
  });
  it("never finalizes an unavailable approval", async () => {
    const approval: MuseApprovalProofRequester = { request: async () => ({ outcome: "unavailable" }) };
    await expect(resolveMusePolicy(host("approval_required"), request, approval, new AbortController().signal)).resolves.toMatchObject({ kind: "not-approved", outcome: "unavailable" });
  });
});
