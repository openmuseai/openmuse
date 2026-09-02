import { describe, expect, it } from "vitest";
import {
  admitInvocation,
  digestInput,
  digestSchema,
  parseOpaqueId,
  type BoundOperation,
  type InvocationAdmissionContext,
  type InvokeRequestPayload
} from "../src/index.js";
import {
  createReferenceBinding,
  referenceHostGeneration,
  referenceHostSessionId
} from "../src/testing/index.js";

const now = 1_700_000_000_000;
const operation: BoundOperation = {
  operationId: "sample.echo",
  effect: "sync_write",
  inputSchemaDigest: digestSchema({ type: "object" }),
  outputSchemaDigest: digestSchema({ type: "object" }),
  cancellable: true,
  idempotency: "required"
};
const binding = createReferenceBinding({ now, operations: [operation] });
const input = { value: "hello" } as const;
const request: InvokeRequestPayload = {
  traceId: parseOpaqueId("trace.reference", "TraceId"),
  bindingId: binding.bindingId,
  operationId: operation.operationId,
  input,
  deadlineAt: now + 30_000,
  cancellationId: "cancel.reference",
  idempotencyKey: "idempotency.reference",
  grantId: "grant.reference"
};
const baseContext: InvocationAdmissionContext = {
  now,
  envelopeHostSessionId: referenceHostSessionId,
  currentHostSessionId: referenceHostSessionId,
  currentHostGeneration: referenceHostGeneration,
  currentScopeRef: binding.scopeRef,
  binding,
  grantRequired: true,
  grant: {
    grantId: "grant.reference",
    bindingId: binding.bindingId,
    operationId: operation.operationId,
    inputDigest: digestInput(input),
    expiresAt: now + 10_000,
    consumed: false
  }
};

describe("invocation admission", () => {
  it("admits a current binding and matching one-use grant", () => {
    expect(admitInvocation(request, baseContext)).toMatchObject({ ok: true, effectiveDeadlineAt: request.deadlineAt });
  });

  it("rejects expired bindings", () => {
    const expired = createReferenceBinding({ now, expiresInMs: -1, operations: [operation] });
    expect(admitInvocation({ ...request, bindingId: expired.bindingId }, { ...baseContext, binding: expired })).toMatchObject({
      ok: false,
      error: { code: "BINDING_EXPIRED" }
    });
  });

  it("rejects unknown operations without fallback", () => {
    expect(admitInvocation({ ...request, operationId: "sample.unknown" }, baseContext)).toMatchObject({
      ok: false,
      error: { code: "OPERATION_NOT_FOUND" }
    });
  });

  it("rejects consumed and input-mismatched grants", () => {
    expect(admitInvocation(request, { ...baseContext, grant: { ...baseContext.grant!, consumed: true } })).toMatchObject({
      ok: false,
      error: { code: "GRANT_CONSUMED" }
    });
    expect(admitInvocation({ ...request, input: { value: "other" } }, baseContext)).toMatchObject({
      ok: false,
      error: { code: "GRANT_INVALID" }
    });
  });

  it("rejects expired grants and elapsed deadlines", () => {
    expect(admitInvocation(request, {
      ...baseContext,
      grant: { ...baseContext.grant!, expiresAt: now }
    })).toMatchObject({ ok: false, error: { code: "GRANT_EXPIRED" } });
    expect(admitInvocation({ ...request, deadlineAt: now }, baseContext)).toMatchObject({
      ok: false,
      error: { code: "DEADLINE_EXCEEDED" }
    });
  });

  it("revalidates generation and scope on every invocation", () => {
    expect(admitInvocation(request, {
      ...baseContext,
      currentHostGeneration: parseOpaqueId("host-generation.other", "HostGeneration")
    })).toMatchObject({ ok: false, error: { code: "HOST_GENERATION_STALE" } });
    expect(admitInvocation(request, { ...baseContext, currentScopeRef: "scope.other" })).toMatchObject({
      ok: false,
      error: { code: "SCOPE_MISMATCH" }
    });
  });
});
