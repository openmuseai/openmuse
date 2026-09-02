import type { BridgeErrorCode, JsonValue, WireEnvelope } from "../contract/types.js";

export interface TransportLimits {
  readonly maxPayloadBytes: number;
  readonly maxConcurrentRequests: number;
  readonly maxResponseBytes?: number;
  readonly maxConcurrentStreams?: number;
  readonly maxStreamFrameBytes?: number;
  readonly maxConnections?: number;
  readonly maxConnectionAgeMs?: number;
  readonly maxDeadlineHorizonMs?: number;
}

export interface RuntimeProof {
  readonly runtimeInstanceId: string;
  readonly nonce: string;
  /** Carrier-authenticated peer description; never supplied by a Bridge envelope. */
  readonly peerIdentity?: string;
}

export interface TransportLaunch {
  readonly endpoint: string;
  readonly nonce: string;
  readonly hostGeneration: string;
}

/** A transport credential, not a domain capability or policy grant. */
export interface TransportConnection {
  readonly connectionId: string;
  readonly token: string;
  readonly runtimeInstanceId: string;
  readonly hostGeneration: string;
  readonly expiresAt: number;
}

export interface TransportRequest {
  readonly connection: TransportConnection;
  readonly message: WireEnvelope;
  readonly deadlineAt: number;
  readonly signal?: AbortSignal;
}

export interface TransportRequestContext {
  readonly connectionId: string;
  readonly runtimeInstanceId: string;
  readonly hostGeneration: string;
  readonly deadlineAt: number;
}

export interface TransportHandler {
  unary(
    message: WireEnvelope,
    signal: AbortSignal,
    context: TransportRequestContext
  ): Promise<WireEnvelope>;
  stream?(
    message: WireEnvelope,
    signal: AbortSignal,
    context: TransportRequestContext
  ): AsyncIterable<WireEnvelope>;
}

export interface MuseHostTransport {
  connect(proof: RuntimeProof): Promise<TransportConnection>;
  disconnect(connection: TransportConnection, reason?: unknown): Promise<void>;
  unary(request: TransportRequest): Promise<WireEnvelope>;
  stream(request: TransportRequest): AsyncIterableIterator<WireEnvelope>;
  close(reason?: unknown): Promise<void>;
}

export interface TransportError {
  readonly code: Extract<
    BridgeErrorCode,
    | "UNAUTHENTICATED"
    | "FORBIDDEN"
    | "HOST_GENERATION_STALE"
    | "DEADLINE_EXCEEDED"
    | "CANCELLED"
    | "RATE_LIMITED"
    | "UNAVAILABLE"
    | "INVALID_ENVELOPE"
    | "INTERNAL"
  >;
  readonly message: string;
}

export interface TransportDependencies {
  readonly now?: () => number;
  readonly authenticatePeer?: (proof: Readonly<RuntimeProof>) => boolean | Promise<boolean>;
  readonly randomId?: () => string;
  readonly randomSecret?: () => string;
}

export const transportError = (
  code: TransportError["code"],
  message: string
): TransportError => ({ code, message });

export const isTransportError = (value: unknown): value is TransportError =>
  typeof value === "object" &&
  value !== null &&
  "code" in value &&
  "message" in value &&
  typeof value.code === "string" &&
  typeof value.message === "string";

export const byteLength = (value: JsonValue | object): number =>
  new TextEncoder().encode(JSON.stringify(value)).byteLength;
