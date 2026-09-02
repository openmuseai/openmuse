import { randomBytes, randomUUID, timingSafeEqual } from "node:crypto";
import type { WireEnvelope } from "../contract/types.js";
import {
  byteLength,
  isTransportError,
  transportError,
  type MuseHostTransport,
  type RuntimeProof,
  type TransportConnection,
  type TransportDependencies,
  type TransportError,
  type TransportHandler,
  type TransportLaunch,
  type TransportLimits,
  type TransportRequest,
  type TransportRequestContext
} from "./types.js";

interface ResolvedLimits {
  readonly maxPayloadBytes: number;
  readonly maxResponseBytes: number;
  readonly maxConcurrentRequests: number;
  readonly maxConcurrentStreams: number;
  readonly maxStreamFrameBytes: number;
  readonly maxConnections: number;
  readonly maxConnectionAgeMs: number;
  readonly maxDeadlineHorizonMs: number;
}

interface ConnectionRecord {
  readonly credential: TransportConnection;
  readonly slots: Set<RequestSlot>;
}

interface RequestSlot {
  readonly controller: AbortController;
  readonly connection: ConnectionRecord;
  readonly stream: boolean;
  readonly deadlineTimer: ReturnType<typeof setTimeout>;
  readonly expiryTimer: ReturnType<typeof setTimeout>;
  readonly detachCaller: () => void;
  released: boolean;
}

const OPAQUE_ID = /^[A-Za-z0-9._~-]{1,128}$/u;
const MAX_TIMER_MS = 2_147_483_647;

const equalSecret = (left: string, right: string): boolean => {
  const a = Buffer.from(left);
  const b = Buffer.from(right);
  return a.length === b.length && timingSafeEqual(a, b);
};

const positive = (value: number, name: string): number => {
  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new TypeError(`${name} must be a positive safe integer`);
  }
  return value;
};

const resolveLimits = (limits: TransportLimits): ResolvedLimits => ({
  maxPayloadBytes: positive(limits.maxPayloadBytes, "maxPayloadBytes"),
  maxResponseBytes: positive(
    limits.maxResponseBytes ?? limits.maxPayloadBytes,
    "maxResponseBytes"
  ),
  maxConcurrentRequests: positive(
    limits.maxConcurrentRequests,
    "maxConcurrentRequests"
  ),
  maxConcurrentStreams: positive(
    limits.maxConcurrentStreams ?? limits.maxConcurrentRequests,
    "maxConcurrentStreams"
  ),
  maxStreamFrameBytes: positive(
    limits.maxStreamFrameBytes ?? limits.maxResponseBytes ?? limits.maxPayloadBytes,
    "maxStreamFrameBytes"
  ),
  maxConnections: positive(limits.maxConnections ?? 8, "maxConnections"),
  maxConnectionAgeMs: positive(
    limits.maxConnectionAgeMs ?? 5 * 60_000,
    "maxConnectionAgeMs"
  ),
  maxDeadlineHorizonMs: positive(
    limits.maxDeadlineHorizonMs ?? 5 * 60_000,
    "maxDeadlineHorizonMs"
  )
});

const normalizedAbort = (reason: unknown, fallback: TransportError): TransportError =>
  isTransportError(reason) ? reason : fallback;

/**
 * Test/development carrier that implements the same session and request lifecycle expected from
 * the desktop UDS/named-pipe carrier. The connection token authenticates only this carrier.
 */
export class InProcessMuseHostTransport implements MuseHostTransport {
  readonly launch: TransportLaunch;

  readonly #limits: ResolvedLimits;
  readonly #now: () => number;
  readonly #authenticatePeer: NonNullable<TransportDependencies["authenticatePeer"]>;
  readonly #randomId: () => string;
  readonly #randomSecret: () => string;
  readonly #connections = new Map<string, ConnectionRecord>();
  readonly #slots = new Set<RequestSlot>();
  #closed = false;
  #activeRequests = 0;
  #activeStreams = 0;

  constructor(
    private readonly handler: TransportHandler,
    limits: TransportLimits,
    hostGeneration = "host.1",
    dependencies: TransportDependencies = {}
  ) {
    if (!OPAQUE_ID.test(hostGeneration)) {
      throw new TypeError("hostGeneration must be an opaque ID");
    }
    this.#limits = resolveLimits(limits);
    this.#now = dependencies.now ?? Date.now;
    this.#authenticatePeer = dependencies.authenticatePeer ?? (() => true);
    this.#randomId = dependencies.randomId ?? randomUUID;
    this.#randomSecret =
      dependencies.randomSecret ?? (() => randomBytes(32).toString("base64url"));
    this.launch = Object.freeze({
      endpoint: "in-process://muse-host",
      nonce: this.#randomSecret(),
      hostGeneration
    });
  }

  async connect(proof: RuntimeProof): Promise<TransportConnection> {
    if (
      this.#closed ||
      !OPAQUE_ID.test(proof.runtimeInstanceId) ||
      !equalSecret(proof.nonce, this.launch.nonce)
    ) {
      throw transportError("UNAUTHENTICATED", "transport handshake rejected");
    }
    if (!(await this.#authenticatePeer(Object.freeze({ ...proof })))) {
      throw transportError("FORBIDDEN", "transport peer is not authorized");
    }
    this.#sweepExpiredConnections();
    if (this.#connections.size >= this.#limits.maxConnections) {
      throw transportError("RATE_LIMITED", "transport connection limit exceeded");
    }
    const credential: TransportConnection = Object.freeze({
      connectionId: this.#randomId(),
      token: this.#randomSecret(),
      runtimeInstanceId: proof.runtimeInstanceId,
      hostGeneration: this.launch.hostGeneration,
      expiresAt: this.#now() + this.#limits.maxConnectionAgeMs
    });
    this.#connections.set(credential.connectionId, {
      credential,
      slots: new Set<RequestSlot>()
    });
    return credential;
  }

  async disconnect(connection: TransportConnection, reason?: unknown): Promise<void> {
    const record = this.#connections.get(connection.connectionId);
    if (record === undefined || !this.#matches(record.credential, connection)) return;
    this.#connections.delete(connection.connectionId);
    this.#abortRecord(
      record,
      normalizedAbort(
        reason,
        transportError("UNAVAILABLE", "transport connection closed")
      )
    );
  }

  async unary(request: TransportRequest): Promise<WireEnvelope> {
    const slot = this.#admit(request, false);
    const context = this.#context(slot.connection.credential, request.deadlineAt);
    try {
      const response = await this.#raceAbort(
        this.handler.unary(request.message, slot.controller.signal, context),
        slot.controller.signal
      );
      if (byteLength(response) > this.#limits.maxResponseBytes) {
        throw transportError("INTERNAL", "transport response exceeds limit");
      }
      return response;
    } finally {
      this.#release(slot);
    }
  }

  async *stream(request: TransportRequest): AsyncIterableIterator<WireEnvelope> {
    if (this.handler.stream === undefined) {
      throw transportError("UNAVAILABLE", "stream transport is unavailable");
    }
    const slot = this.#admit(request, true);
    const context = this.#context(slot.connection.credential, request.deadlineAt);
    let iterator: AsyncIterator<WireEnvelope> | undefined;
    try {
      iterator = this.handler
        .stream(request.message, slot.controller.signal, context)
        [Symbol.asyncIterator]();
      for (;;) {
        const item = await this.#raceAbort(iterator.next(), slot.controller.signal);
        if (item.done === true) break;
        if (byteLength(item.value) > this.#limits.maxStreamFrameBytes) {
          throw transportError("INTERNAL", "transport stream frame exceeds limit");
        }
        yield item.value;
      }
    } finally {
      if (!slot.controller.signal.aborted) {
        slot.controller.abort(transportError("CANCELLED", "stream consumer closed"));
      }
      if (iterator?.return !== undefined) {
        try {
          await iterator.return();
        } catch {
          // The transport error/cancellation already determines the externally visible result.
        }
      }
      this.#release(slot);
    }
  }

  async close(reason?: unknown): Promise<void> {
    if (this.#closed) return;
    this.#closed = true;
    const error = normalizedAbort(
      reason,
      transportError("UNAVAILABLE", "transport closed")
    );
    for (const record of this.#connections.values()) this.#abortRecord(record, error);
    this.#connections.clear();
  }

  #admit(request: TransportRequest, stream: boolean): RequestSlot {
    const record = this.#authorize(request.connection);
    if (byteLength(request.message) > this.#limits.maxPayloadBytes) {
      throw transportError("INVALID_ENVELOPE", "transport payload exceeds limit");
    }
    if (this.#activeRequests >= this.#limits.maxConcurrentRequests) {
      throw transportError("RATE_LIMITED", "transport concurrency limit exceeded");
    }
    if (stream && this.#activeStreams >= this.#limits.maxConcurrentStreams) {
      throw transportError("RATE_LIMITED", "transport stream limit exceeded");
    }
    const now = this.#now();
    if (!Number.isSafeInteger(request.deadlineAt) || request.deadlineAt <= now) {
      throw transportError("DEADLINE_EXCEEDED", "deadline already elapsed");
    }
    if (request.deadlineAt - now > this.#limits.maxDeadlineHorizonMs) {
      throw transportError("INVALID_ENVELOPE", "deadline exceeds transport horizon");
    }
    if (request.signal?.aborted === true) {
      throw transportError("CANCELLED", "caller cancelled request");
    }

    const controller = new AbortController();
    const deadlineTimer = setTimeout(
      () => controller.abort(transportError("DEADLINE_EXCEEDED", "deadline elapsed")),
      Math.min(request.deadlineAt - now, MAX_TIMER_MS)
    );
    const expiryTimer = setTimeout(
      () => controller.abort(transportError("UNAUTHENTICATED", "connection expired")),
      Math.min(record.credential.expiresAt - now, MAX_TIMER_MS)
    );
    const onCallerAbort = (): void => {
      controller.abort(transportError("CANCELLED", "caller cancelled request"));
    };
    request.signal?.addEventListener("abort", onCallerAbort, { once: true });
    const slot: RequestSlot = {
      controller,
      connection: record,
      stream,
      deadlineTimer,
      expiryTimer,
      detachCaller: () => request.signal?.removeEventListener("abort", onCallerAbort),
      released: false
    };
    record.slots.add(slot);
    this.#slots.add(slot);
    this.#activeRequests += 1;
    if (stream) this.#activeStreams += 1;
    return slot;
  }

  #authorize(connection: TransportConnection): ConnectionRecord {
    const record = this.#connections.get(connection.connectionId);
    if (this.#closed || record === undefined) {
      throw transportError("UNAUTHENTICATED", "connection is not active");
    }
    if (connection.hostGeneration !== this.launch.hostGeneration) {
      throw transportError("HOST_GENERATION_STALE", "connection host generation is stale");
    }
    if (!this.#matches(record.credential, connection)) {
      throw transportError("UNAUTHENTICATED", "connection is not active");
    }
    if (record.credential.expiresAt <= this.#now()) {
      this.#connections.delete(connection.connectionId);
      this.#abortRecord(
        record,
        transportError("UNAUTHENTICATED", "connection expired")
      );
      throw transportError("UNAUTHENTICATED", "connection expired");
    }
    return record;
  }

  #sweepExpiredConnections(): void {
    const now = this.#now();
    for (const [connectionId, record] of this.#connections) {
      if (record.credential.expiresAt > now) continue;
      this.#connections.delete(connectionId);
      this.#abortRecord(
        record,
        transportError("UNAUTHENTICATED", "connection expired")
      );
    }
  }

  #matches(expected: TransportConnection, actual: TransportConnection): boolean {
    return (
      expected.connectionId === actual.connectionId &&
      expected.runtimeInstanceId === actual.runtimeInstanceId &&
      expected.hostGeneration === actual.hostGeneration &&
      expected.expiresAt === actual.expiresAt &&
      equalSecret(expected.token, actual.token)
    );
  }

  #context(connection: TransportConnection, deadlineAt: number): TransportRequestContext {
    return Object.freeze({
      connectionId: connection.connectionId,
      runtimeInstanceId: connection.runtimeInstanceId,
      hostGeneration: connection.hostGeneration,
      deadlineAt
    });
  }

  async #raceAbort<T>(operation: Promise<T>, signal: AbortSignal): Promise<T> {
    if (signal.aborted) throw signal.reason;
    return new Promise<T>((resolve, reject) => {
      const cleanup = (): void => signal.removeEventListener("abort", aborted);
      const aborted = (): void => {
        cleanup();
        reject(signal.reason);
      };
      signal.addEventListener("abort", aborted, { once: true });
      operation.then(
        value => {
          cleanup();
          resolve(value);
        },
        error => {
          cleanup();
          reject(error);
        }
      );
    });
  }

  #abortRecord(record: ConnectionRecord, error: TransportError): void {
    for (const slot of [...record.slots]) {
      slot.controller.abort(error);
      this.#release(slot);
    }
  }

  #release(slot: RequestSlot): void {
    if (slot.released) return;
    slot.released = true;
    clearTimeout(slot.deadlineTimer);
    clearTimeout(slot.expiryTimer);
    slot.detachCaller();
    slot.connection.slots.delete(slot);
    this.#slots.delete(slot);
    this.#activeRequests -= 1;
    if (slot.stream) this.#activeStreams -= 1;
  }
}
