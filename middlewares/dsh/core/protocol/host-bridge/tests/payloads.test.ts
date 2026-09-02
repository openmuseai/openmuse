import validCases from "../fixtures/v1/valid/messages.json" with { type: "json" };
import { describe, expect, it } from "vitest";
import {
  decodeMessage,
  parseOpaqueId,
  ProtocolViolation,
  type JsonObject,
  type NegotiatedProtocol
} from "../src/index.js";
import { parseConformanceCases } from "../src/testing/index.js";

const negotiated: NegotiatedProtocol = {
  major: 1,
  minor: 0,
  features: new Set(["muse.events"]),
  hostSessionId: parseOpaqueId("host-session.1", "HostSessionId")
};

const fixtureMessage = (name: string): Record<string, unknown> => {
  const fixture = parseConformanceCases(validCases).find((entry) => entry.name === name);
  if (fixture === undefined) throw new Error(`missing fixture: ${name}`);
  return structuredClone(fixture.message) as Record<string, unknown>;
};

describe("stable protocol states", () => {
  it("accepts all cancel dispositions without implying rollback", () => {
    for (const disposition of ["accepted", "already_terminal", "not_found"]) {
      const message = fixtureMessage("cancel response");
      message.payload = { ok: true, value: { disposition } };
      expect((decodeMessage(message, negotiated).payload as JsonObject).ok).toBe(true);
    }
  });

  it("accepts the complete closed command status set including unknown", () => {
    const statuses = [
      "accepted",
      "running",
      "applied_local",
      "sync_pending",
      "synced",
      "rejected",
      "failed",
      "cancelled",
      "unknown"
    ];
    for (const status of statuses) {
      const message = fixtureMessage("status response");
      const payload = message.payload as { value: { status: string; receipt: { status: string } } };
      payload.value.status = status;
      payload.value.receipt.status = status;
      expect((decodeMessage(message, negotiated).payload as JsonObject).ok).toBe(true);
    }
  });

  it("preserves an unknown Provider error code and details", () => {
    const message = fixtureMessage("invoke response");
    message.payload = {
      ok: false,
      error: {
        kind: "provider",
        namespace: "sample.provider",
        code: "NEW_STABLE_FAILURE",
        message: "safe provider message",
        retryable: true,
        retryAfterMs: 250,
        details: { hint: "retry" }
      }
    };
    const payload = decodeMessage(message, negotiated).payload as JsonObject;
    expect((payload.error as JsonObject).code).toBe("NEW_STABLE_FAILURE");
    expect((payload.error as JsonObject).details).toEqual({ hint: "retry" });
  });

  it("enforces conditional policy fields", () => {
    for (const decision of ["allow", "deny"]) {
      const message = fixtureMessage("policy evaluate response");
      message.payload = {
        ok: true,
        value: { policyDecisionId: "policy-decision.2", decision, expiresAt: 1700000060051 }
      };
      expect(() => decodeMessage(message, negotiated)).not.toThrow();
    }
    const missingApproval = fixtureMessage("policy evaluate response");
    missingApproval.payload = {
      ok: true,
      value: { policyDecisionId: "policy-decision.2", decision: "approval_required", expiresAt: 1700000060051 }
    };
    expect(() => decodeMessage(missingApproval, negotiated)).toThrowError(ProtocolViolation);
  });

  it("validates Provider event metadata without interpreting its payload", () => {
    const message = fixtureMessage("bridge event");
    message.payload = {
      subscriptionId: "subscription.1",
      cursor: "cursor.3",
      occurredAt: 1700000000043,
      hostGeneration: "host-generation.1",
      data: {
        eventKind: "provider.event",
        descriptorId: "descriptor.1",
        descriptorRevision: "revision.2",
        eventType: "sample.changed",
        schemaDigest: "sha256:4444444444444444444444444444444444444444444444444444444444444444",
        payload: { opaque: true }
      }
    };
    const decoded = decodeMessage(message, negotiated);
    expect(((decoded.payload as JsonObject).data as JsonObject).payload).toEqual({ opaque: true });
  });
});
