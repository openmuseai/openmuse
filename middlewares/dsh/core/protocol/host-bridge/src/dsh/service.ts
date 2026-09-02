import { randomUUID } from "node:crypto";
import { Context, Service } from "@deepseek-ai/cordis";
import {
  HARD_LIMITS,
  assertResponseMatches,
  decodeMessage,
  isTransportError,
  transportError,
  type BridgeEventPayload,
  type BridgeLimits,
  type BridgeMessageV1,
  type ClientCorrelation,
  type HelloResponseValue,
  type InvokeRequestPayload,
  type InvokeResponsePayload,
  type JsonValue,
  type NegotiatedProtocol,
  type RequestId,
  type RuntimeInstanceId,
  type TransportConnection,
  type WireEnvelope
} from "../index.js";
import { clientCorrelationFromDsh } from "./identity.js";
import type {
  DshInvocationSource,
  MuseHostConnectionState,
  MuseHostConnectorResult,
  MuseHostReadySession,
  MuseHostRequestKind,
  MuseHostRequestOptions,
  MuseHostServiceApi,
  MuseHostServiceConfig,
  MuseHostStateSnapshot
} from "./types.js";

declare module "@deepseek-ai/cordis" {
  interface Context {
    museHost: MuseHostService;
  }

  interface Events {
    "museHost/state"(snapshot: MuseHostStateSnapshot): void;
    "museHost/event"(event: BridgeEventPayload): void;
    "museHost/binding-invalidated"(event: BridgeEventPayload): void;
    "museHost/command-status"(event: BridgeEventPayload): void;
    "museHost/approval-resolved"(payload: JsonValue): void;
  }
}

interface ResolvedConfig {
  readonly requestTimeoutMs: number;
  readonly subscriptionLifetimeMs: number;
  readonly reconnectDelayMs: number;
  readonly autoStart: boolean;
}

interface ActiveSession extends MuseHostReadySession {
  readonly negotiated: NegotiatedProtocol & { readonly limits: BridgeLimits };
  readonly proof: { readonly runtimeInstanceId: string };
}

const RESPONSE_KIND: Readonly<Record<MuseHostRequestKind, string>> = Object.freeze({
  "discover.request": "discover.response",
  "bind.request": "bind.response",
  "invoke.request": "invoke.response",
  "subscribe.request": "subscribe.response",
  "policy.evaluate.request": "policy.evaluate.response",
  "policy.finalize.request": "policy.finalize.response",
  "cancel.request": "cancel.response",
  "status.request": "status.response"
});

const positive = (value: number | undefined, fallback: number, name: string): number => {
  const resolved = value ?? fallback;
  if (!Number.isSafeInteger(resolved) || resolved <= 0 || resolved > 2_147_483_647) {
    throw new TypeError(`${name} must be a positive timer-safe integer`);
  }
  return resolved;
};

const resolveConfig = (config: MuseHostServiceConfig = {}): ResolvedConfig => Object.freeze({
  requestTimeoutMs: positive(config.requestTimeoutMs, 30_000, "requestTimeoutMs"),
  subscriptionLifetimeMs: positive(
    config.subscriptionLifetimeMs,
    240_000,
    "subscriptionLifetimeMs"
  ),
  reconnectDelayMs: positive(config.reconnectDelayMs, 1_000, "reconnectDelayMs"),
  autoStart: (() => {
    const value = config.autoStart ?? true;
    if (typeof value !== "boolean") throw new TypeError("autoStart must be a boolean");
    return value;
  })()
});

const requestId = (): RequestId => randomUUID() as RequestId;

const abortRace = async <T>(operation: Promise<T>, signal: AbortSignal): Promise<T> => {
  if (signal.aborted) throw signal.reason;
  return new Promise<T>((resolve, reject) => {
    const aborted = (): void => {
      signal.removeEventListener("abort", aborted);
      reject(signal.reason);
    };
    signal.addEventListener("abort", aborted, { once: true });
    operation.then(
      value => {
        signal.removeEventListener("abort", aborted);
        resolve(value);
      },
      error => {
        signal.removeEventListener("abort", aborted);
        reject(error);
      }
    );
  });
};

const delay = (durationMs: number, signal: AbortSignal): Promise<void> => {
  if (signal.aborted) return Promise.reject(signal.reason);
  return new Promise<void>((resolve, reject) => {
    const timer = setTimeout(() => {
      signal.removeEventListener("abort", aborted);
      resolve();
    }, durationMs);
    const aborted = (): void => {
      clearTimeout(timer);
      signal.removeEventListener("abort", aborted);
      reject(signal.reason);
    };
    signal.addEventListener("abort", aborted, { once: true });
  });
};

const safeDiagnosticCode = (error: unknown): string => {
  if (isTransportError(error)) return error.code;
  if (error instanceof Error && error.name === "ProtocolViolation") return "PROTOCOL_VIOLATION";
  return "CONNECT_FAILED";
};

const isProtocolViolation = (error: unknown): boolean =>
  error instanceof Error && error.name === "ProtocolViolation";

const shouldInvalidate = (error: unknown): boolean =>
  isProtocolViolation(error) || (
    isTransportError(error) && [
      "UNAUTHENTICATED",
      "HOST_GENERATION_STALE",
      "UNAVAILABLE"
    ].includes(error.code)
  );

export class MuseHostService extends Service implements MuseHostServiceApi {
  static inject = ["museHostConnector"];

  private readonly config: ResolvedConfig;
  private readonly lifecycle = new AbortController();
  private readonly active = new Set<Promise<unknown>>();
  private state: MuseHostStateSnapshot = Object.freeze({ state: "disconnected" });
  private session: ActiveSession | undefined;
  private connecting: Promise<ActiveSession> | undefined;
  private pump: Promise<void> | undefined;
  private closing: Promise<void> | undefined;
  private stopping = false;

  constructor(ctx: Context, config: MuseHostServiceConfig = {}) {
    super(ctx, "museHost");
    this.config = resolveConfig(config);
  }

  async *[Service.init](): AsyncGenerator<() => Promise<void>, void, void> {
    yield () => this.close();
    if (this.config.autoStart) this.pump = this.runEventPump();
  }

  snapshot(): MuseHostStateSnapshot {
    return this.state;
  }

  async connect(signal?: AbortSignal): Promise<MuseHostReadySession> {
    this.assertOpen();
    if (this.session !== undefined) return this.session;
    this.connecting ??= this.open().finally(() => {
      this.connecting = undefined;
    });
    const session = signal === undefined
      ? await this.connecting
      : await abortRace(this.connecting, signal);
    return session;
  }

  async request(
    kind: MuseHostRequestKind,
    payload: JsonValue,
    options: MuseHostRequestOptions = {}
  ): Promise<WireEnvelope> {
    this.assertOpen();
    const operation = this.runRequest(kind, payload, options);
    this.active.add(operation);
    try {
      return await operation;
    } finally {
      this.active.delete(operation);
    }
  }

  async invoke(
    payload: InvokeRequestPayload,
    source: DshInvocationSource
  ): Promise<InvokeResponsePayload> {
    const correlation = clientCorrelationFromDsh(source);
    const response = await this.request(
      "invoke.request",
      { ...payload, clientCorrelation: correlation } as unknown as JsonValue,
      { signal: source.signal, deadlineAt: payload.deadlineAt, correlation }
    );
    if (response.kind !== "invoke.response") {
      throw transportError("INTERNAL", "Muse Host returned an unexpected response kind");
    }
    return response.payload as unknown as InvokeResponsePayload;
  }

  close(): Promise<void> {
    this.closing ??= this.closeOwnedResources();
    return this.closing;
  }

  private async runRequest(
    kind: MuseHostRequestKind,
    payload: JsonValue,
    options: MuseHostRequestOptions
  ): Promise<WireEnvelope> {
    if (!Object.hasOwn(RESPONSE_KIND, kind)) {
      throw transportError("INVALID_ENVELOPE", "Muse Host request kind is not callable");
    }
    const signal = options.signal === undefined
      ? this.lifecycle.signal
      : AbortSignal.any([options.signal, this.lifecycle.signal]);
    const session = await this.connect(signal);
    const active = session as ActiveSession;
    const now = Date.now();
    const deadlineAt = options.deadlineAt ?? now + this.config.requestTimeoutMs;
    const id = requestId();
    const envelope = this.envelope(kind, payload, active.negotiated, id, options.correlation);
    decodeMessage(envelope, active.negotiated, active.negotiated.limits);
    try {
      const response = await active.transport.unary({
        connection: active.connection,
        message: envelope,
        deadlineAt,
        ...(signal === undefined ? {} : { signal })
      });
      const decoded = decodeMessage(response, active.negotiated, active.negotiated.limits);
      assertResponseMatches(decoded, {
        requestId: id,
        hostSessionId: active.negotiated.hostSessionId
      });
      if (decoded.kind !== RESPONSE_KIND[kind]) {
        throw transportError("INTERNAL", "Muse Host returned an unexpected response kind");
      }
      if (kind === "policy.finalize.request") {
        this.emitContained(
          "approval-resolved",
          () => this.ctx.emit("museHost/approval-resolved", decoded.payload as unknown as JsonValue)
        );
      }
      return decoded as unknown as WireEnvelope;
    } catch (error) {
      if (shouldInvalidate(error)) await this.invalidate(active, safeDiagnosticCode(error));
      throw error;
    }
  }

  private async open(): Promise<ActiveSession> {
    this.setState("connecting");
    let opened: MuseHostConnectorResult | undefined;
    let connection: TransportConnection | undefined;
    try {
      opened = await this.ctx.museHostConnector.open(this.lifecycle.signal);
      connection = await opened.transport.connect(opened.proof);
      const id = requestId();
      const now = Date.now();
      const hello: WireEnvelope = {
        protocol: "muse-bridge",
        major: 1,
        minor: 0,
        kind: "hello.request",
        requestId: id,
        sentAt: now,
        payload: {
          runtime: {
            kind: "desktop",
            instanceId: opened.proof.runtimeInstanceId as RuntimeInstanceId
          },
          versions: [{ major: 1, minMinor: 0, maxMinor: 0 }],
          features: [],
          limits: HARD_LIMITS
        } as unknown as JsonValue
      };
      decodeMessage(hello, undefined, HARD_LIMITS);
      const response = await opened.transport.unary({
        connection,
        message: hello,
        deadlineAt: now + this.config.requestTimeoutMs,
        signal: this.lifecycle.signal
      });
      const decoded = decodeMessage(response, undefined, HARD_LIMITS);
      assertResponseMatches(decoded, { requestId: id });
      if (decoded.kind !== "hello.response") {
        throw transportError("INTERNAL", "Muse Host hello returned an unexpected response kind");
      }
      const result = decoded.payload;
      if (!result.ok) throw result.error;
      const helloValue: HelloResponseValue = result.value;
      if (helloValue.hostGeneration !== connection.hostGeneration) {
        throw transportError("HOST_GENERATION_STALE", "transport and protocol generations differ");
      }
      const negotiated: NegotiatedProtocol = Object.freeze({
        major: 1,
        minor: helloValue.selectedVersion.minor,
        features: new Set(helloValue.features),
        hostSessionId: helloValue.hostSessionId
      });
      const session: ActiveSession = Object.freeze({
        transport: opened.transport,
        connection,
        hello: helloValue,
        negotiated: Object.freeze({ ...negotiated, limits: helloValue.limits }),
        proof: Object.freeze({ runtimeInstanceId: opened.proof.runtimeInstanceId })
      });
      if (this.stopping) {
        await opened.transport.close();
        throw transportError("UNAVAILABLE", "Muse Host Service is closing");
      }
      this.session = session;
      this.setState("ready", {
        hostGeneration: helloValue.hostGeneration,
        runtimeInstanceId: opened.proof.runtimeInstanceId
      });
      return session;
    } catch (error) {
      if (opened !== undefined) {
        if (connection !== undefined) await opened.transport.disconnect(connection).catch(() => undefined);
        await opened.transport.close().catch(() => undefined);
      }
      if (!this.stopping) this.setState("disconnected", { diagnosticCode: safeDiagnosticCode(error) });
      throw error;
    }
  }

  private envelope(
    kind: MuseHostRequestKind,
    payload: JsonValue,
    negotiated: NegotiatedProtocol,
    id: RequestId,
    correlation?: ClientCorrelation
  ): WireEnvelope {
    return Object.freeze({
      protocol: "muse-bridge" as const,
      major: 1 as const,
      minor: negotiated.minor,
      kind,
      requestId: id,
      hostSessionId: negotiated.hostSessionId,
      sentAt: Date.now(),
      payload,
      ...(correlation === undefined
        ? {}
        : { extensions: { "muse.client-correlation": correlation as unknown as JsonValue } })
    });
  }

  private async runEventPump(): Promise<void> {
    while (!this.stopping) {
      try {
        const session = await this.connect(this.lifecycle.signal) as ActiveSession;
        await this.consumeEvents(session);
        if (!this.stopping) await this.invalidate(session, "STREAM_ENDED");
      } catch (error) {
        if (this.stopping) return;
        if (this.session !== undefined && shouldInvalidate(error)) {
          await this.invalidate(this.session, safeDiagnosticCode(error));
        } else {
          this.setState("disconnected", { diagnosticCode: safeDiagnosticCode(error) });
        }
      }
      try {
        await delay(this.config.reconnectDelayMs, this.lifecycle.signal);
      } catch {
        return;
      }
    }
  }

  private async consumeEvents(session: ActiveSession): Promise<void> {
    const now = Date.now();
    const deadlineAt = now + this.config.subscriptionLifetimeMs;
    const id = requestId();
    const envelope = this.envelope(
      "subscribe.request",
      {
        filters: {
          eventKinds: [
            "binding.invalidated",
            "host.generation.changed",
            "command.status.changed",
            "provider.event",
            "stream.gap",
            "stream.error"
          ]
        },
        deadlineAt
      },
      session.negotiated,
      id
    );
    const stream = session.transport.stream({
      connection: session.connection,
      message: envelope,
      deadlineAt,
      signal: this.lifecycle.signal
    });
    let acknowledged = false;
    for await (const raw of stream) {
      const message: BridgeMessageV1 = decodeMessage(
        raw,
        session.negotiated,
        session.negotiated.limits
      );
      if (!acknowledged) {
        assertResponseMatches(message, {
          requestId: id,
          hostSessionId: session.negotiated.hostSessionId
        });
        if (message.kind !== "subscribe.response" || !message.payload.ok) {
          throw transportError("INTERNAL", "Muse Host subscription acknowledgement failed");
        }
        acknowledged = true;
        continue;
      }
      if (message.kind !== "bridge.event") {
        throw transportError("INTERNAL", "Muse Host subscription returned a non-event frame");
      }
      this.publishEvent(message.payload);
    }
    if (!acknowledged) {
      throw transportError("UNAVAILABLE", "Muse Host subscription ended before acknowledgement");
    }
  }

  private publishEvent(event: BridgeEventPayload): void {
    this.emitContained("event", () => this.ctx.emit("museHost/event", event));
    if (event.data.eventKind === "binding.invalidated") {
      this.emitContained(
        "binding-invalidated",
        () => this.ctx.emit("museHost/binding-invalidated", event)
      );
    }
    if (event.data.eventKind === "command.status.changed") {
      this.emitContained("command-status", () => this.ctx.emit("museHost/command-status", event));
    }
  }

  private async invalidate(session: ActiveSession, diagnosticCode: string): Promise<void> {
    if (this.session !== session) return;
    this.session = undefined;
    await session.transport.disconnect(session.connection).catch(() => undefined);
    await session.transport.close().catch(() => undefined);
    if (!this.stopping) this.setState("disconnected", { diagnosticCode });
  }

  private async closeOwnedResources(): Promise<void> {
    if (this.stopping) return;
    this.stopping = true;
    this.setState("closing");
    this.lifecycle.abort(transportError("UNAVAILABLE", "Muse Host Service closed"));
    await this.pump?.catch(() => undefined);
    await this.connecting?.catch(() => undefined);
    await Promise.allSettled([...this.active]);
    const session = this.session;
    this.session = undefined;
    if (session !== undefined) {
      await session.transport.disconnect(session.connection).catch(() => undefined);
      await session.transport.close().catch(() => undefined);
    }
    this.setState("closed");
  }

  private assertOpen(): void {
    if (this.stopping) throw transportError("UNAVAILABLE", "Muse Host Service is closed");
  }

  private setState(
    state: MuseHostConnectionState,
    details: Omit<MuseHostStateSnapshot, "state"> = {}
  ): void {
    const next = Object.freeze({ state, ...details });
    if (
      this.state.state === next.state &&
      this.state.hostGeneration === next.hostGeneration &&
      this.state.runtimeInstanceId === next.runtimeInstanceId &&
      this.state.diagnosticCode === next.diagnosticCode
    ) return;
    this.state = next;
    this.emitContained("state", () => this.ctx.emit("museHost/state", next));
    if (next.diagnosticCode !== undefined) {
      this.ctx.logger.warn("muse-host: connection state %s (%s)", state, next.diagnosticCode);
    }
  }

  private emitContained(label: string, emit: () => void): void {
    try {
      emit();
    } catch {
      this.ctx.logger.warn("muse-host: %s observer failed", label);
    }
  }
}

export default MuseHostService;
