import { digestInput } from "../codec/digest.js";
import { bridgeFailure } from "../codec/errors.js";
import type {
  InvocationAdmission,
  InvocationAdmissionContext,
  InvokeRequestPayload
} from "../contract/types.js";

export const admitInvocation = (
  request: InvokeRequestPayload,
  context: InvocationAdmissionContext
): InvocationAdmission => {
  if (context.envelopeHostSessionId !== context.currentHostSessionId) {
    return { ok: false, error: bridgeFailure("HOST_SESSION_EXPIRED", "host session is no longer current") };
  }
  if (request.deadlineAt <= context.now) {
    return { ok: false, error: bridgeFailure("DEADLINE_EXCEEDED", "invoke deadline elapsed before dispatch") };
  }
  const binding = context.binding;
  if (binding === undefined || binding.bindingId !== request.bindingId) {
    return { ok: false, error: bridgeFailure("BINDING_NOT_FOUND", "binding is not available") };
  }
  if (binding.revoked) return { ok: false, error: bridgeFailure("BINDING_REVOKED", "binding has been revoked") };
  if (binding.expiresAt <= context.now) {
    return { ok: false, error: bridgeFailure("BINDING_EXPIRED", "binding has expired") };
  }
  if (binding.hostGeneration !== context.currentHostGeneration) {
    return { ok: false, error: bridgeFailure("HOST_GENERATION_STALE", "binding belongs to a stale host generation") };
  }
  if (binding.scopeRef !== context.currentScopeRef) {
    return { ok: false, error: bridgeFailure("SCOPE_MISMATCH", "binding scope is no longer current") };
  }
  const operation = binding.operations.get(request.operationId);
  if (operation === undefined) {
    return { ok: false, error: bridgeFailure("OPERATION_NOT_FOUND", "operation is not part of the current binding") };
  }
  if (operation.idempotency === "required" && request.idempotencyKey === undefined) {
    return { ok: false, error: bridgeFailure("INPUT_INVALID", "operation requires an idempotency key") };
  }
  if (operation.idempotency === "none" && request.idempotencyKey !== undefined) {
    return { ok: false, error: bridgeFailure("INPUT_INVALID", "operation does not accept an idempotency key") };
  }
  if (context.grantRequired) {
    const grant = context.grant;
    if (request.grantId === undefined || grant === undefined || grant.grantId !== request.grantId) {
      return { ok: false, error: bridgeFailure("GRANT_INVALID", "a matching host grant is required") };
    }
    if (grant.consumed) return { ok: false, error: bridgeFailure("GRANT_CONSUMED", "grant was already consumed") };
    if (grant.expiresAt <= context.now) return { ok: false, error: bridgeFailure("GRANT_EXPIRED", "grant has expired") };
    if (
      grant.bindingId !== request.bindingId ||
      grant.operationId !== request.operationId ||
      grant.inputDigest !== digestInput(request.input)
    ) {
      return { ok: false, error: bridgeFailure("GRANT_INVALID", "grant is not bound to this invocation") };
    }
  }
  return { ok: true, operation, effectiveDeadlineAt: request.deadlineAt };
};
