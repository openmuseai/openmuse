import { randomUUID } from "node:crypto";
import type {
  ApprovalId,
  BridgeResult,
  ClientCorrelation,
  JsonValue,
  PolicyEvaluateRequestPayload,
  PolicyEvaluateResponseValue,
  PolicyFinalizeResponseValue,
  WireEnvelope
} from "../index.js";
import type { MuseHostServiceApi } from "./types.js";

/** Trusted desktop/DSH channel: a Plugin can request a proof but never mint it. */
export interface MuseApprovalProofRequester {
  request(input: {
    readonly approvalId: ApprovalId;
    readonly reason?: string;
    readonly signal: AbortSignal;
  }): Promise<
    | { readonly outcome: "approved"; readonly proofId: string }
    | { readonly outcome: "rejected" | "cancelled" | "unavailable" }
  >;
}

const approvalRequesters = new WeakMap<object, MuseApprovalProofRequester>();
const APPROVAL_REQUESTER = Symbol.for("muse.host.approvalProofRequester");
const PROCESS_APPROVAL_REQUESTER = Symbol.for("muse.host.approvalProofRequester.process");

type ApprovalHost = object & {
  readonly root?: object;
  [APPROVAL_REQUESTER]?: MuseApprovalProofRequester;
};

type ProcessApprovalHost = typeof globalThis & {
  [PROCESS_APPROVAL_REQUESTER]?: MuseApprovalProofRequester;
};

const hostOf = (ctx: object): object => {
  const root = (ctx as ApprovalHost).root;
  return root ?? ctx;
};

/** Deployment-owned; never a Plugin-provided value. */
export const setMuseApprovalProofRequester = (
  ctx: object,
  requester: MuseApprovalProofRequester
): void => {
  const host = hostOf(ctx);
  approvalRequesters.set(ctx, requester);
  approvalRequesters.set(host, requester);
  (host as ApprovalHost)[APPROVAL_REQUESTER] = requester;
  (ctx as ApprovalHost)[APPROVAL_REQUESTER] = requester;
  (globalThis as ProcessApprovalHost)[PROCESS_APPROVAL_REQUESTER] = requester;
};

export const getMuseApprovalProofRequester = (
  ctx: object
): MuseApprovalProofRequester | undefined => {
  const host = hostOf(ctx);
  return (
    approvalRequesters.get(ctx)
    ?? approvalRequesters.get(host)
    ?? (ctx as ApprovalHost)[APPROVAL_REQUESTER]
    ?? (host as ApprovalHost)[APPROVAL_REQUESTER]
    ?? (globalThis as ProcessApprovalHost)[PROCESS_APPROVAL_REQUESTER]
  );
};

export type MusePolicyResolution =
  | { readonly kind: "allow"; readonly policyDecisionId: string }
  | { readonly kind: "deny"; readonly policyDecisionId: string; readonly reason?: string }
  | { readonly kind: "approved"; readonly policyDecisionId: string; readonly grantId: string; readonly expiresAt: number }
  | { readonly kind: "not-approved"; readonly policyDecisionId: string; readonly outcome: "rejected" | "cancelled" | "unavailable" };

const valueOf = <T>(envelope: WireEnvelope, kind: string): T => {
  if (envelope.kind !== kind) throw new Error(`expected ${kind}, received ${envelope.kind}`);
  const result = envelope.payload as unknown as BridgeResult<T>;
  if (!result.ok) throw new Error(`Muse Host policy request failed: ${result.error.code}`);
  return result.value;
};

/** Resolves policy only; the caller must pass its grant to final Host admission. */
export const resolveMusePolicy = async (
  host: MuseHostServiceApi,
  request: PolicyEvaluateRequestPayload,
  approval: MuseApprovalProofRequester,
  signal: AbortSignal,
  correlation?: ClientCorrelation
): Promise<MusePolicyResolution> => {
  const evaluated = valueOf<PolicyEvaluateResponseValue>(
    await host.request("policy.evaluate.request", request as unknown as JsonValue, {
      signal,
      deadlineAt: request.deadlineAt,
      ...(correlation === undefined ? {} : { correlation })
    }),
    "policy.evaluate.response"
  );
  if (evaluated.decision === "allow") return { kind: "allow", policyDecisionId: evaluated.policyDecisionId };
  if (evaluated.decision === "deny") return { kind: "deny", policyDecisionId: evaluated.policyDecisionId, ...(evaluated.reason === undefined ? {} : { reason: evaluated.reason }) };
  if (evaluated.approvalId === undefined) throw new Error("approval-required policy result omitted approvalId");
  const answer = await approval.request({ approvalId: evaluated.approvalId, ...(evaluated.reason === undefined ? {} : { reason: evaluated.reason }), signal });
  if (answer.outcome !== "approved") return { kind: "not-approved", policyDecisionId: evaluated.policyDecisionId, outcome: answer.outcome };
  const finalized = valueOf<PolicyFinalizeResponseValue>(
    await host.request("policy.finalize.request", {
      traceId: randomUUID(), approvalId: evaluated.approvalId, outcome: "approved", approvalProofId: answer.proofId, deadlineAt: request.deadlineAt
    } as unknown as JsonValue, {
      signal,
      deadlineAt: request.deadlineAt,
      ...(correlation === undefined ? {} : { correlation })
    }),
    "policy.finalize.response"
  );
  if (finalized.outcome !== "approved" || finalized.grantId === undefined || finalized.expiresAt === undefined) throw new Error("approved policy finalization omitted a grant");
  return { kind: "approved", policyDecisionId: finalized.policyDecisionId, grantId: finalized.grantId, expiresAt: finalized.expiresAt };
};
