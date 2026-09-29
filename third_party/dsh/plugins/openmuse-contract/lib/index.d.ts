export const protocolName: 'openmuse.contract';
export const protocolMajor: 1;
export const protocolMinor: 0;

export const contractErrorCodes: readonly ContractErrorCode[];

export type PrincipalKind = 'user' | 'agent' | 'plugin' | 'service' | 'device';
export type ContractErrorCode =
  | 'DENIED'
  | 'NOT_FOUND'
  | 'CONFLICT'
  | 'EXPIRED'
  | 'STALE_GENERATION'
  | 'UNAVAILABLE'
  | 'TRANSIENT'
  | 'INTEGRITY_FAILED';
export type ReceiptState = 'committed' | 'rejected' | 'cancelled' | 'expired' | 'failed';
export type HandleState = 'active' | 'revoked' | 'expired';
export type LeaseState = 'active' | 'released' | 'revoked' | 'expired';

export interface ProtocolVersion {
  name: 'openmuse.contract';
  major: 1;
  minor: number;
}

export interface PrincipalRef {
  principalRef: string;
  kind: PrincipalKind;
}

export interface ContractScope {
  authorityRef: string;
  workspaceRef?: string;
  resourceRef?: string;
}

export interface EnvelopeContext {
  protocol: ProtocolVersion;
  requestId: string;
  actor: PrincipalRef;
  caller: PrincipalRef;
  scope: ContractScope;
  generation: number;
  deadlineAtMs: number;
  cancellationRef: string;
}

export interface Receipt {
  receiptRef: string;
  requestId: string;
  generation: number;
  state: ReceiptState;
  issuedAtMs: number;
  effects: string[];
}

export interface ContractFailure {
  code: ContractErrorCode;
  message: string;
  retryable: boolean;
  details: Record<string, unknown>;
}

export interface RequestEnvelope extends EnvelopeContext {
  kind: 'request';
  operation: string;
  payload: unknown;
}

export interface OkOutcome {
  status: 'ok';
  receipt: Receipt;
  value: unknown;
}

export interface ErrorOutcome {
  status: 'error';
  receipt: Receipt;
  error: ContractFailure;
}

export interface ResponseEnvelope extends EnvelopeContext {
  kind: 'response';
  outcome: OkOutcome | ErrorOutcome;
}

export interface Descriptor {
  descriptorRef: string;
  generation: number;
  revision: string;
  issuedAtMs: number;
  value: unknown;
}

export interface CapabilityHandle {
  handleRef: string;
  audience: PrincipalRef;
  scope: ContractScope;
  access: string[];
  generation: number;
  issuedAtMs: number;
  expiresAtMs: number;
  state: HandleState;
}

export interface Lease {
  leaseRef: string;
  holder: PrincipalRef;
  scope: ContractScope;
  generation: number;
  issuedAtMs: number;
  expiresAtMs: number;
  state: LeaseState;
}

export interface LifecycleSnapshot {
  protocol: ProtocolVersion;
  descriptor: Descriptor;
  handle: CapabilityHandle;
  lease: Lease;
  receipt: Receipt;
}

export interface ParseOptions {
  expectedGeneration?: number;
}

export class OpenMuseContractError extends TypeError {}

export function parseContractEnvelope(
  input: unknown,
  options?: ParseOptions,
): RequestEnvelope | ResponseEnvelope;

export function parseLifecycleSnapshot(input: unknown): LifecycleSnapshot;
export function ensureLiveAt(request: RequestEnvelope, nowMs: number): void;
