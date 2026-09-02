import { digestInput } from "../codec/digest.js";
import type { JsonObject, JsonValue, WireEnvelope } from "../contract/types.js";
import {
  transportError,
  type TransportHandler,
  type TransportRequestContext
} from "./types.js";

interface IdempotencyEntry {
  readonly fingerprint: string;
  readonly expiresAt: number;
  readonly result: Promise<WireEnvelope>;
  settled: boolean;
}

export interface IdempotencyOptions {
  readonly maxEntries: number;
  readonly ttlMs: number;
  readonly now?: () => number;
}

const objectPayload = (value: JsonValue): JsonObject | undefined => {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return undefined;
  return value as JsonObject;
};

const stringField = (value: JsonValue | undefined): string | undefined =>
  typeof value === "string" ? value : undefined;

/**
 * Host-side unary decorator for invoke idempotency. Entries are scoped to runtime + exact
 * binding/operation/key and survive a transport reconnect within this Host generation.
 */
export class IdempotentTransportHandler implements TransportHandler {
  readonly #entries = new Map<string, IdempotencyEntry>();
  readonly #now: () => number;

  constructor(
    private readonly delegate: TransportHandler,
    private readonly options: IdempotencyOptions
  ) {
    if (!Number.isSafeInteger(options.maxEntries) || options.maxEntries <= 0) {
      throw new TypeError("maxEntries must be a positive safe integer");
    }
    if (!Number.isSafeInteger(options.ttlMs) || options.ttlMs <= 0) {
      throw new TypeError("ttlMs must be a positive safe integer");
    }
    this.#now = options.now ?? Date.now;
  }

  async unary(
    message: WireEnvelope,
    signal: AbortSignal,
    context: TransportRequestContext
  ): Promise<WireEnvelope> {
    const identity = this.#identity(message, context);
    if (identity === undefined) return this.delegate.unary(message, signal, context);

    this.#sweep();
    const existing = this.#entries.get(identity.key);
    if (existing !== undefined) {
      if (existing.fingerprint !== identity.fingerprint) {
        throw transportError(
          "INVALID_ENVELOPE",
          "idempotency key was reused with different invocation input"
        );
      }
      return structuredClone(await existing.result);
    }
    this.#makeRoom();
    const entry: IdempotencyEntry = {
      fingerprint: identity.fingerprint,
      expiresAt: this.#now() + this.options.ttlMs,
      result: Promise.resolve().then(() => this.delegate.unary(message, signal, context)),
      settled: false
    };
    this.#entries.set(identity.key, entry);
    try {
      const result = await entry.result;
      entry.settled = true;
      return structuredClone(result);
    } catch (error) {
      if (this.#entries.get(identity.key) === entry) this.#entries.delete(identity.key);
      throw error;
    }
  }

  stream(
    message: WireEnvelope,
    signal: AbortSignal,
    context: TransportRequestContext
  ): AsyncIterable<WireEnvelope> {
    if (this.delegate.stream === undefined) {
      throw transportError("UNAVAILABLE", "stream transport is unavailable");
    }
    return this.delegate.stream(message, signal, context);
  }

  #identity(
    message: WireEnvelope,
    context: TransportRequestContext
  ): { readonly key: string; readonly fingerprint: string } | undefined {
    if (message.kind !== "invoke.request") return undefined;
    const payload = objectPayload(message.payload);
    if (payload === undefined) return undefined;
    const idempotencyKey = stringField(payload.idempotencyKey);
    if (idempotencyKey === undefined) return undefined;
    const bindingId = stringField(payload.bindingId);
    const operationId = stringField(payload.operationId);
    const input = payload.input;
    if (bindingId === undefined || operationId === undefined || input === undefined) {
      throw transportError("INVALID_ENVELOPE", "idempotent invoke identity is incomplete");
    }
    return {
      key: `${context.runtimeInstanceId}\0${bindingId}\0${operationId}\0${idempotencyKey}`,
      fingerprint: digestInput({ bindingId, operationId, input })
    };
  }

  #sweep(): void {
    const now = this.#now();
    for (const [key, entry] of this.#entries) {
      if (entry.settled && entry.expiresAt <= now) this.#entries.delete(key);
    }
  }

  #makeRoom(): void {
    if (this.#entries.size < this.options.maxEntries) return;
    for (const [key, entry] of this.#entries) {
      if (entry.settled) {
        this.#entries.delete(key);
        return;
      }
    }
    throw transportError("RATE_LIMITED", "idempotency registry is full");
  }
}
