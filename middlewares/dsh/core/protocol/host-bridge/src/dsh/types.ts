import type {
  BridgeEventPayload,
  ClientCorrelation,
  HelloResponseValue,
  InvokeRequestPayload,
  InvokeResponsePayload,
  JsonValue,
  MessageKind,
  MuseHostTransport,
  RuntimeProof,
  TransportConnection,
  WireEnvelope
} from "../index.js";

export type MuseHostRequestKind = Exclude<
  MessageKind,
  "hello.request" | "hello.response" | "bridge.event" | `${string}.response`
>;

export type MuseHostConnectionState =
  | "disconnected"
  | "connecting"
  | "ready"
  | "closing"
  | "closed";

export interface MuseHostStateSnapshot {
  readonly state: MuseHostConnectionState;
  readonly hostGeneration?: string;
  readonly runtimeInstanceId?: string;
  /** Stable safe category only; never contains endpoint, nonce, token, payload or stack. */
  readonly diagnosticCode?: string;
}

export interface MuseHostConnectorResult {
  readonly transport: MuseHostTransport;
  readonly proof: RuntimeProof;
}

/** Launcher-owned seam. Secrets stay inside this object and never enter Cordis config. */
export interface MuseHostConnector {
  open(signal: AbortSignal): Promise<MuseHostConnectorResult>;
}

export interface MuseHostServiceConfig {
  readonly requestTimeoutMs?: number;
  readonly subscriptionLifetimeMs?: number;
  readonly reconnectDelayMs?: number;
  readonly autoStart?: boolean;
}

export interface MuseHostRequestOptions {
  readonly signal?: AbortSignal;
  readonly deadlineAt?: number;
  readonly correlation?: ClientCorrelation;
}

export interface DshAgentIdentity {
  readonly id: string;
}

/** Narrow structural projection of DSH's public Agent/Tool execution seams. */
export interface DshInvocationSource {
  readonly sessionId?: string;
  readonly agent?: DshAgentIdentity;
  readonly turn?: number;
  readonly step?: number;
  readonly toolCallId?: string;
  readonly signal: AbortSignal;
}

export interface MuseHostReadySession {
  readonly transport: MuseHostTransport;
  readonly connection: TransportConnection;
  readonly hello: HelloResponseValue;
}

export interface MuseHostServiceApi {
  snapshot(): MuseHostStateSnapshot;
  connect(signal?: AbortSignal): Promise<MuseHostReadySession>;
  request(
    kind: MuseHostRequestKind,
    payload: JsonValue,
    options?: MuseHostRequestOptions
  ): Promise<WireEnvelope>;
  invoke(
    payload: InvokeRequestPayload,
    source: DshInvocationSource
  ): Promise<InvokeResponsePayload>;
  close(): Promise<void>;
}

export interface MuseHostCordisEvents {
  "museHost/state": (snapshot: MuseHostStateSnapshot) => void;
  "museHost/event": (event: BridgeEventPayload) => void;
  "museHost/binding-invalidated": (event: BridgeEventPayload) => void;
  "museHost/command-status": (event: BridgeEventPayload) => void;
  "museHost/approval-resolved": (payload: JsonValue) => void;
}
