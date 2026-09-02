declare const brandSymbol: unique symbol;

export type Brand<Value, Name extends string> = Value & { readonly [brandSymbol]: Name };

export type RequestId = Brand<string, "RequestId">;
export type TraceId = Brand<string, "TraceId">;
export type HostSessionId = Brand<string, "HostSessionId">;
export type RuntimeInstanceId = Brand<string, "RuntimeInstanceId">;
export type DescriptorId = Brand<string, "DescriptorId">;
export type DescriptorRevision = Brand<string, "DescriptorRevision">;
export type ProviderInstanceId = Brand<string, "ProviderInstanceId">;
export type HostGeneration = Brand<string, "HostGeneration">;
export type BindingId = Brand<string, "BindingId">;
export type CancellationId = Brand<string, "CancellationId">;
export type SubscriptionId = Brand<string, "SubscriptionId">;
export type EventCursor = Brand<string, "EventCursor">;
export type PolicyDecisionId = Brand<string, "PolicyDecisionId">;
export type ApprovalId = Brand<string, "ApprovalId">;
export type GrantId = Brand<string, "GrantId">;
export type ReceiptId = Brand<string, "ReceiptId">;
export type CommandId = Brand<string, "CommandId">;
export type AuditRef = Brand<string, "AuditRef">;
export type IdempotencyKey = Brand<string, "IdempotencyKey">;
export type SchemaDigest = Brand<string, "SchemaDigest">;
export type InputDigest = Brand<string, "InputDigest">;
export type GrantDigest = Brand<string, "GrantDigest">;

const opaqueIdPattern = /^[A-Za-z0-9._~-]{1,128}$/;
const digestPattern = /^sha256:[0-9a-f]{64}$/;

export const parseOpaqueId = <Name extends string>(value: unknown, label: Name): Brand<string, Name> => {
  if (typeof value !== "string" || !opaqueIdPattern.test(value)) {
    throw new TypeError(`${label} must be a 1-128 byte opaque ASCII identifier`);
  }
  return value as Brand<string, Name>;
};

export const parseDigest = <Name extends string>(value: unknown, label: Name): Brand<string, Name> => {
  if (typeof value !== "string" || !digestPattern.test(value)) {
    throw new TypeError(`${label} must use sha256:<64 lowercase hex>`);
  }
  return value as Brand<string, Name>;
};

export const isOpaqueId = (value: unknown): value is string => typeof value === "string" && opaqueIdPattern.test(value);
export const isDigest = (value: unknown): value is string => typeof value === "string" && digestPattern.test(value);
