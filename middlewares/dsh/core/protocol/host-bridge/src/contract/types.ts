import type {
  ApprovalId,
  AuditRef,
  BindingId,
  CancellationId,
  CommandId,
  DescriptorId,
  DescriptorRevision,
  EventCursor,
  GrantId,
  HostGeneration,
  HostSessionId,
  IdempotencyKey,
  InputDigest,
  PolicyDecisionId,
  ProviderInstanceId,
  ReceiptId,
  RequestId,
  RuntimeInstanceId,
  SchemaDigest,
  SubscriptionId,
  TraceId
} from "./brands.js";
import type { MessageKind } from "./methods.js";

export type JsonPrimitive = null | boolean | number | string;
export type JsonArray = readonly JsonValue[];
export type JsonObject = { readonly [key: string]: JsonValue };
export type JsonValue = JsonPrimitive | JsonArray | JsonObject;

export interface BridgeLimits {
  readonly maxMessageBytes: number;
  readonly maxInlineSchemaBytes: number;
  readonly maxResolvedSchemaBytes: number;
  readonly maxDiscoverPageBytes: number;
  readonly maxDiscoverDescriptors: number;
  readonly maxInputBytes: number;
  readonly maxOutputBytes: number;
  readonly maxErrorDetailsBytes: number;
  readonly maxEventPayloadBytes: number;
  readonly maxJsonDepth: number;
  readonly maxContainerChildren: number;
}

export interface NegotiatedProtocol {
  readonly major: 1;
  readonly minor: number;
  readonly features: ReadonlySet<string>;
  readonly hostSessionId: HostSessionId;
}

export interface WireEnvelope {
  readonly protocol: "muse-bridge";
  readonly major: 1;
  readonly minor: number;
  readonly kind: MessageKind;
  readonly requestId?: RequestId;
  readonly hostSessionId?: HostSessionId;
  readonly sentAt: number;
  readonly payload: JsonValue;
  readonly extensions?: Readonly<Record<string, JsonValue>>;
}

export interface VersionRange {
  readonly major: number;
  readonly minMinor: number;
  readonly maxMinor: number;
}

export interface ProtocolOffer {
  readonly versions: readonly VersionRange[];
  readonly features: readonly string[];
  readonly limits: BridgeLimits;
}

export interface NegotiationResult {
  readonly major: 1;
  readonly minor: number;
  readonly features: readonly string[];
  readonly limits: BridgeLimits;
}

export interface SchemaResourceV1 {
  readonly sha256: SchemaDigest;
  readonly byteLength: number;
  readonly draft: "2020-12";
  readonly inline?: JsonValue;
  readonly uri?: string;
}

export const BRIDGE_ERROR_CODES = [
  "INVALID_ENVELOPE",
  "UNSUPPORTED_PROTOCOL",
  "HANDSHAKE_REQUIRED",
  "REQUEST_ID_MISMATCH",
  "UNAUTHENTICATED",
  "FORBIDDEN",
  "HOST_SESSION_EXPIRED",
  "HOST_GENERATION_STALE",
  "CAPABILITY_NOT_FOUND",
  "DESCRIPTOR_STALE",
  "BINDING_NOT_FOUND",
  "BINDING_EXPIRED",
  "BINDING_REVOKED",
  "SCOPE_MISMATCH",
  "OPERATION_NOT_FOUND",
  "SCHEMA_UNAVAILABLE",
  "SCHEMA_DIGEST_MISMATCH",
  "INPUT_INVALID",
  "OUTPUT_INVALID",
  "POLICY_DENIED",
  "APPROVAL_REQUIRED",
  "APPROVAL_REJECTED",
  "GRANT_INVALID",
  "GRANT_EXPIRED",
  "GRANT_CONSUMED",
  "CANCELLED",
  "DEADLINE_EXCEEDED",
  "CURSOR_EXPIRED",
  "RATE_LIMITED",
  "UNAVAILABLE",
  "INTERNAL"
] as const;

export type BridgeErrorCode = (typeof BRIDGE_ERROR_CODES)[number];

export interface BridgeError {
  readonly kind: "bridge";
  readonly code: BridgeErrorCode;
  readonly message: string;
  readonly retryable: boolean;
  readonly retryAfterMs?: number;
  readonly details?: Readonly<Record<string, JsonValue>>;
}

export interface ProviderError {
  readonly kind: "provider";
  readonly namespace: string;
  readonly code: string;
  readonly message: string;
  readonly retryable: boolean;
  readonly retryAfterMs?: number;
  readonly details?: JsonValue;
}

export type ProtocolError = BridgeError | ProviderError;

export type Effect = "read" | "local_write" | "sync_write" | "external_side_effect";
export type IdempotencyMode = "none" | "optional" | "required";

export interface BoundOperation {
  readonly operationId: string;
  readonly effect: Effect;
  readonly inputSchemaDigest: SchemaDigest;
  readonly outputSchemaDigest: SchemaDigest;
  readonly cancellable: boolean;
  readonly idempotency: IdempotencyMode;
}

export interface BindingSnapshot {
  readonly bindingId: BindingId;
  readonly descriptorRevision: string;
  readonly hostGeneration: HostGeneration;
  readonly scopeRef: string;
  readonly expiresAt: number;
  readonly revoked: boolean;
  readonly operations: ReadonlyMap<string, BoundOperation>;
}

export interface InvokeRequestPayload {
  readonly traceId: TraceId;
  readonly bindingId: BindingId;
  readonly operationId: string;
  readonly input: JsonValue;
  readonly deadlineAt: number;
  readonly cancellationId: string;
  readonly idempotencyKey?: string;
  readonly grantId?: string;
  readonly clientCorrelation?: ClientCorrelation;
}

export interface InvocationAdmissionContext {
  readonly now: number;
  readonly envelopeHostSessionId: HostSessionId;
  readonly currentHostSessionId: HostSessionId;
  readonly currentHostGeneration: HostGeneration;
  readonly currentScopeRef: string;
  readonly binding?: BindingSnapshot;
  readonly grant?: {
    readonly grantId: string;
    readonly bindingId: BindingId;
    readonly operationId: string;
    readonly inputDigest: InputDigest;
    readonly expiresAt: number;
    readonly consumed: boolean;
  };
  readonly grantRequired: boolean;
}

export type InvocationAdmission =
  | { readonly ok: true; readonly operation: BoundOperation; readonly effectiveDeadlineAt: number }
  | { readonly ok: false; readonly error: BridgeError };

export interface ResponseCorrelation {
  readonly requestId: RequestId;
  readonly hostSessionId?: HostSessionId;
}

export type BridgeResult<Value> =
  | { readonly ok: true; readonly value: Value }
  | { readonly ok: false; readonly error: ProtocolError };

export interface ClientCorrelation {
  readonly sessionRef?: string;
  readonly turnRef?: string;
  readonly stepRef?: string;
  readonly toolCallRef?: string;
}

export interface ScopeHint {
  readonly refs: Readonly<Record<string, string>>;
}

export interface HelloRequestPayload {
  readonly runtime: { readonly kind: "desktop" | "mobile"; readonly instanceId: RuntimeInstanceId };
  readonly versions: readonly VersionRange[];
  readonly features: readonly string[];
  readonly limits?: BridgeLimits;
  readonly clientCorrelation?: ClientCorrelation;
}

export interface HelloResponseValue {
  readonly selectedVersion: { readonly major: 1; readonly minor: number };
  readonly features: readonly string[];
  readonly limits: BridgeLimits;
  readonly hostSessionId: HostSessionId;
  readonly hostGeneration: HostGeneration;
  readonly serverTime: number;
  readonly clockSkewToleranceMs: number;
  readonly authorityRevision: string;
}

export interface OperationDescriptor {
  readonly operationId: string;
  readonly effect: Effect;
  readonly inputSchema: SchemaResourceV1;
  readonly outputSchema: SchemaResourceV1;
  readonly cancellable: boolean;
  readonly idempotency: IdempotencyMode;
  readonly title?: string;
  readonly summary?: string;
}

export interface EventDescriptor {
  readonly eventType: string;
  readonly payloadSchema: SchemaResourceV1;
  readonly ignorable?: boolean;
  readonly title?: string;
}

export interface CapabilityDescriptor {
  readonly descriptorId: DescriptorId;
  readonly revision: DescriptorRevision;
  readonly familyId: string;
  readonly contractVersion: { readonly major: number; readonly minor: number };
  readonly providerInstanceId: ProviderInstanceId;
  readonly operations: readonly OperationDescriptor[];
  readonly events: readonly EventDescriptor[];
  readonly title?: string;
  readonly summary?: string;
}

export interface DiscoverRequestPayload {
  readonly deadlineAt: number;
  readonly pageSize: number;
  readonly cursor?: EventCursor;
  readonly families?: readonly string[];
  readonly scopeHint?: ScopeHint;
}

export interface DiscoverResponseValue {
  readonly registryRevision: string;
  readonly descriptors: readonly CapabilityDescriptor[];
  readonly nextCursor?: EventCursor;
}

export interface BindRequestPayload {
  readonly descriptorId: DescriptorId;
  readonly descriptorRevision: DescriptorRevision;
  readonly operationIds: readonly string[];
  readonly scopeHint?: ScopeHint;
  readonly deadlineAt: number;
}

export interface WireBinding {
  readonly bindingId: BindingId;
  readonly descriptorId: DescriptorId;
  readonly descriptorRevision: DescriptorRevision;
  readonly providerInstanceId: ProviderInstanceId;
  readonly hostGeneration: HostGeneration;
  readonly scopeRef: string;
  readonly expiresAt: number;
  readonly operations: readonly BoundOperation[];
}

export type CommandStatus =
  | "accepted"
  | "running"
  | "applied_local"
  | "sync_pending"
  | "synced"
  | "rejected"
  | "failed"
  | "cancelled"
  | "unknown";

export interface Receipt {
  readonly receiptId: ReceiptId;
  readonly requestId: RequestId;
  readonly traceId: TraceId;
  readonly hostSessionId: HostSessionId;
  readonly hostGeneration: HostGeneration;
  readonly bindingId: BindingId;
  readonly operationId: string;
  readonly policyDecision: "allow" | "deny" | "approval_required";
  readonly status: CommandStatus;
  readonly issuedAt: number;
  readonly clientCorrelation?: ClientCorrelation;
  readonly commandId?: CommandId;
  readonly idempotencyKey?: IdempotencyKey;
  readonly approvalId?: ApprovalId;
  readonly auditRef?: AuditRef;
}

export type InvokeResponsePayload =
  | { readonly ok: true; readonly value: JsonValue; readonly receipt: Receipt }
  | { readonly ok: false; readonly error: ProtocolError; readonly receipt?: Receipt };

export interface SubscribeRequestPayload {
  readonly filters: {
    readonly eventKinds?: readonly BridgeEventKind[];
    readonly providerEventTypes?: readonly string[];
    readonly bindingIds?: readonly BindingId[];
  };
  readonly afterCursor?: EventCursor;
  readonly deadlineAt: number;
}

export interface SubscribeResponseValue {
  readonly subscriptionId: SubscriptionId;
  readonly startCursor: EventCursor;
  readonly retentionFromCursor?: EventCursor;
}

export type BridgeEventKind =
  | "descriptor.changed"
  | "binding.invalidated"
  | "host.generation.changed"
  | "command.status.changed"
  | "provider.event"
  | "stream.gap"
  | "stream.error";

export type BridgeEventData =
  | {
      readonly eventKind: "descriptor.changed";
      readonly descriptorId: DescriptorId;
      readonly revision?: DescriptorRevision;
      readonly change: "added" | "updated" | "removed";
    }
  | {
      readonly eventKind: "binding.invalidated";
      readonly bindingId: BindingId;
      readonly reason: "expired" | "revoked" | "descriptor_stale" | "generation_changed" | "scope_changed";
    }
  | {
      readonly eventKind: "host.generation.changed";
      readonly previousGeneration: HostGeneration;
      readonly currentGeneration: HostGeneration;
    }
  | {
      readonly eventKind: "command.status.changed";
      readonly commandId: CommandId;
      readonly status: CommandStatus;
      readonly statusRevision: string;
      readonly receipt?: Receipt;
    }
  | {
      readonly eventKind: "provider.event";
      readonly descriptorId: DescriptorId;
      readonly descriptorRevision: DescriptorRevision;
      readonly eventType: string;
      readonly schemaDigest: SchemaDigest;
      readonly payload: JsonValue;
    }
  | {
      readonly eventKind: "stream.gap";
      readonly reason: "cursor_expired" | "retention_gap" | "malformed_frame" | "generation_changed";
      readonly afterCursor?: EventCursor;
    }
  | { readonly eventKind: "stream.error"; readonly error: BridgeError };

export interface BridgeEventPayload {
  readonly subscriptionId: SubscriptionId;
  readonly cursor: EventCursor;
  readonly occurredAt: number;
  readonly hostGeneration: HostGeneration;
  readonly data: BridgeEventData;
}

export interface PolicyEvaluateRequestPayload {
  readonly traceId: TraceId;
  readonly bindingId: BindingId;
  readonly operationId: string;
  readonly inputDigest: InputDigest;
  readonly effect: Effect;
  readonly deadlineAt: number;
}

export interface PolicyEvaluateResponseValue {
  readonly policyDecisionId: PolicyDecisionId;
  readonly decision: "allow" | "deny" | "approval_required";
  readonly expiresAt: number;
  readonly approvalId?: ApprovalId;
  readonly reason?: string;
}

export interface PolicyFinalizeRequestPayload {
  readonly traceId: TraceId;
  readonly approvalId: ApprovalId;
  readonly outcome: "approved" | "rejected";
  readonly approvalProofId: string;
  readonly deadlineAt: number;
}

export interface PolicyFinalizeResponseValue {
  readonly policyDecisionId: PolicyDecisionId;
  readonly outcome: "approved" | "rejected";
  readonly grantId?: GrantId;
  readonly expiresAt?: number;
}

export interface CancelRequestPayload {
  readonly cancellationId: CancellationId;
  readonly deadlineAt: number;
}

export interface CancelResponseValue {
  readonly disposition: "accepted" | "already_terminal" | "not_found";
}

export type StatusRequestPayload =
  | { readonly commandId: CommandId; readonly deadlineAt: number }
  | { readonly bindingId: BindingId; readonly idempotencyKey: IdempotencyKey; readonly deadlineAt: number };

export interface StatusResponseValue {
  readonly status: CommandStatus;
  readonly statusRevision: string;
  readonly updatedAt: number;
  readonly receipt: Receipt;
}

interface BaseEnvelope<Kind extends MessageKind, Payload> {
  readonly protocol: "muse-bridge";
  readonly major: 1;
  readonly minor: number;
  readonly kind: Kind;
  readonly sentAt: number;
  readonly payload: Payload;
  readonly extensions?: Readonly<Record<string, JsonValue>>;
}

type HelloEnvelope<Kind extends "hello.request" | "hello.response", Payload> = BaseEnvelope<Kind, Payload> & {
  readonly requestId: RequestId;
  readonly hostSessionId?: never;
};

type SessionEnvelope<Kind extends Exclude<MessageKind, "hello.request" | "hello.response" | "bridge.event">, Payload> =
  BaseEnvelope<Kind, Payload> & { readonly requestId: RequestId; readonly hostSessionId: HostSessionId };

type EventEnvelope = BaseEnvelope<"bridge.event", BridgeEventPayload> & {
  readonly requestId?: never;
  readonly hostSessionId: HostSessionId;
};

export type BridgeMessageV1 =
  | HelloEnvelope<"hello.request", HelloRequestPayload>
  | HelloEnvelope<"hello.response", BridgeResult<HelloResponseValue>>
  | SessionEnvelope<"discover.request", DiscoverRequestPayload>
  | SessionEnvelope<"discover.response", BridgeResult<DiscoverResponseValue>>
  | SessionEnvelope<"bind.request", BindRequestPayload>
  | SessionEnvelope<"bind.response", BridgeResult<WireBinding>>
  | SessionEnvelope<"invoke.request", InvokeRequestPayload>
  | SessionEnvelope<"invoke.response", InvokeResponsePayload>
  | SessionEnvelope<"subscribe.request", SubscribeRequestPayload>
  | SessionEnvelope<"subscribe.response", BridgeResult<SubscribeResponseValue>>
  | EventEnvelope
  | SessionEnvelope<"policy.evaluate.request", PolicyEvaluateRequestPayload>
  | SessionEnvelope<"policy.evaluate.response", BridgeResult<PolicyEvaluateResponseValue>>
  | SessionEnvelope<"policy.finalize.request", PolicyFinalizeRequestPayload>
  | SessionEnvelope<"policy.finalize.response", BridgeResult<PolicyFinalizeResponseValue>>
  | SessionEnvelope<"cancel.request", CancelRequestPayload>
  | SessionEnvelope<"cancel.response", BridgeResult<CancelResponseValue>>
  | SessionEnvelope<"status.request", StatusRequestPayload>
  | SessionEnvelope<"status.response", BridgeResult<StatusResponseValue>>;
