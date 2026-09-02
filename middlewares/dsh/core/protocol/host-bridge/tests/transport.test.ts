import { describe, expect, it, vi } from "vitest";
import {
  IdempotentTransportHandler,
  InProcessMuseHostTransport,
  type TransportConnection,
  type TransportHandler,
  type WireEnvelope
} from "../src/index.js";

const message: WireEnvelope = {
  protocol: "muse-bridge",
  major: 1,
  minor: 0,
  kind: "hello.request",
  sentAt: 1,
  payload: { offer: {} }
};
const handler: TransportHandler = { unary: async value => value };
const limits = { maxPayloadBytes: 4096, maxConcurrentRequests: 2 } as const;

const connect = (
  transport: InProcessMuseHostTransport,
  runtimeInstanceId = "runtime.1"
): Promise<TransportConnection> =>
  transport.connect({ runtimeInstanceId, nonce: transport.launch.nonce });

describe("in-process transport identity", () => {
  it("rejects an invalid nonce and binds credentials to a runtime identity", async () => {
    const transport = new InProcessMuseHostTransport(handler, limits);
    await expect(
      transport.connect({ runtimeInstanceId: "runtime.1", nonce: "wrong" })
    ).rejects.toMatchObject({ code: "UNAUTHENTICATED" });
    const connection = await connect(transport);
    const forged = { ...connection, runtimeInstanceId: "runtime.2" };
    await expect(
      transport.unary({ connection: forged, message, deadlineAt: Date.now() + 1000 })
    ).rejects.toMatchObject({ code: "UNAUTHENTICATED" });
    const stale = { ...connection, hostGeneration: "host.2" };
    await expect(
      transport.unary({ connection: stale, message, deadlineAt: Date.now() + 1000 })
    ).rejects.toMatchObject({ code: "HOST_GENERATION_STALE" });
  });

  it("uses a carrier peer authenticator before issuing a credential", async () => {
    const transport = new InProcessMuseHostTransport(handler, limits, "host.1", {
      authenticatePeer: proof => proof.peerIdentity === "uid.501"
    });
    await expect(
      transport.connect({
        runtimeInstanceId: "runtime.1",
        nonce: transport.launch.nonce,
        peerIdentity: "uid.502"
      })
    ).rejects.toMatchObject({ code: "FORBIDDEN" });
    await expect(
      transport.connect({
        runtimeInstanceId: "runtime.1",
        nonce: transport.launch.nonce,
        peerIdentity: "uid.501"
      })
    ).resolves.toMatchObject({ runtimeInstanceId: "runtime.1" });
  });

  it("expires connection tokens and enforces the connection ceiling", async () => {
    let now = 10;
    let id = 0;
    const transport = new InProcessMuseHostTransport(
      handler,
      { ...limits, maxConnections: 1, maxConnectionAgeMs: 100 },
      "host.1",
      { now: () => now, randomId: () => `connection.${++id}` }
    );
    const connection = await connect(transport);
    await expect(connect(transport, "runtime.2")).rejects.toMatchObject({
      code: "RATE_LIMITED"
    });
    now = 111;
    await expect(connect(transport, "runtime.2")).resolves.toMatchObject({
      runtimeInstanceId: "runtime.2"
    });
    await expect(
      transport.unary({ connection, message, deadlineAt: 200 })
    ).rejects.toMatchObject({ code: "UNAUTHENTICATED" });
  });
});

describe("in-process request lifecycle", () => {
  it("enforces request, response and concurrent request limits", async () => {
    let release!: () => void;
    const blocked: TransportHandler = {
      unary: async value => new Promise(resolve => (release = () => resolve(value)))
    };
    const transport = new InProcessMuseHostTransport(blocked, {
      maxPayloadBytes: 4096,
      maxResponseBytes: 256,
      maxConcurrentRequests: 1
    });
    const connection = await connect(transport);
    await expect(
      transport.unary({
        connection,
        message: { ...message, payload: { value: "x".repeat(5000) } },
        deadlineAt: Date.now() + 1000
      })
    ).rejects.toMatchObject({ code: "INVALID_ENVELOPE" });
    const first = transport.unary({ connection, message, deadlineAt: Date.now() + 1000 });
    await expect(
      transport.unary({ connection, message, deadlineAt: Date.now() + 1000 })
    ).rejects.toMatchObject({ code: "RATE_LIMITED" });
    release();
    await expect(first).resolves.toEqual(message);

    const oversized: TransportHandler = {
      unary: async value => ({ ...value, payload: { value: "x".repeat(500) } })
    };
    const responseTransport = new InProcessMuseHostTransport(oversized, {
      ...limits,
      maxResponseBytes: 128
    });
    const responseConnection = await connect(responseTransport);
    await expect(
      responseTransport.unary({
        connection: responseConnection,
        message,
        deadlineAt: Date.now() + 1000
      })
    ).rejects.toMatchObject({ code: "INTERNAL" });
  });

  it("turns deadline and caller cancellation into Host cancellation", async () => {
    vi.useFakeTimers();
    vi.setSystemTime(1000);
    try {
      const waiting: TransportHandler = {
        unary: async (_, signal) =>
          new Promise((_, reject) =>
            signal.addEventListener("abort", () => reject(signal.reason), { once: true })
          )
      };
      const transport = new InProcessMuseHostTransport(waiting, limits);
      const connection = await connect(transport);
      const deadline = transport.unary({ connection, message, deadlineAt: 1010 });
      const deadlineAssertion = expect(deadline).rejects.toMatchObject({
        code: "DEADLINE_EXCEEDED"
      });
      await vi.advanceTimersByTimeAsync(10);
      await deadlineAssertion;

      const controller = new AbortController();
      const cancelled = transport.unary({
        connection,
        message,
        deadlineAt: 2000,
        signal: controller.signal
      });
      controller.abort("untrusted reason");
      await expect(cancelled).rejects.toMatchObject({ code: "CANCELLED" });
    } finally {
      vi.useRealTimers();
    }
  });

  it("disconnect cancels only that runtime while close cancels all pending work", async () => {
    const waiting: TransportHandler = { unary: async () => new Promise(() => undefined) };
    const transport = new InProcessMuseHostTransport(waiting, {
      ...limits,
      maxConnections: 2
    });
    const firstConnection = await connect(transport, "runtime.1");
    const secondConnection = await connect(transport, "runtime.2");
    const first = transport.unary({
      connection: firstConnection,
      message,
      deadlineAt: Date.now() + 1000
    });
    const second = transport.unary({
      connection: secondConnection,
      message,
      deadlineAt: Date.now() + 1000
    });
    await transport.disconnect(firstConnection);
    await expect(first).rejects.toMatchObject({ code: "UNAVAILABLE" });
    await transport.close();
    await expect(second).rejects.toMatchObject({ code: "UNAVAILABLE" });
  });
});

describe("in-process stream lifecycle", () => {
  it("holds concurrency admission for the complete iterator lifetime", async () => {
    let release!: () => void;
    const streaming: TransportHandler = {
      unary: async value => value,
      stream: async function* (value) {
        await new Promise<void>(resolve => (release = resolve));
        yield value;
      }
    };
    const transport = new InProcessMuseHostTransport(streaming, {
      ...limits,
      maxConcurrentRequests: 2,
      maxConcurrentStreams: 1
    });
    const connection = await connect(transport);
    const first = transport.stream({ connection, message, deadlineAt: Date.now() + 1000 });
    const firstFrame = first.next();
    const second = transport.stream({ connection, message, deadlineAt: Date.now() + 1000 });
    await expect(second.next()).rejects.toMatchObject({ code: "RATE_LIMITED" });
    release();
    await expect(firstFrame).resolves.toMatchObject({ value: message, done: false });
    await first.return?.(undefined);
  });

  it("bounds each frame and aborts the producer when the consumer stops", async () => {
    let aborted = false;
    const streaming: TransportHandler = {
      unary: async value => value,
      stream: async function* (value, signal) {
        signal.addEventListener("abort", () => (aborted = true), { once: true });
        yield value;
        yield { ...value, payload: { data: "x".repeat(1000) } };
      }
    };
    const transport = new InProcessMuseHostTransport(streaming, {
      ...limits,
      maxStreamFrameBytes: 256
    });
    const connection = await connect(transport);
    const stream = transport.stream({ connection, message, deadlineAt: Date.now() + 1000 });
    await expect(stream.next()).resolves.toMatchObject({ done: false });
    await expect(stream.next()).rejects.toMatchObject({ code: "INTERNAL" });
    expect(aborted).toBe(true);
  });
});

describe("invoke idempotency", () => {
  const invoke = (input: number): WireEnvelope => ({
    ...message,
    kind: "invoke.request",
    payload: {
      bindingId: "binding.1",
      operationId: "sample.write",
      input,
      idempotencyKey: "idem.1"
    }
  });

  it("reuses a completed outcome across a reconnect of the same runtime", async () => {
    let calls = 0;
    const delegate: TransportHandler = {
      unary: async value => {
        calls += 1;
        return value;
      }
    };
    const transport = new InProcessMuseHostTransport(
      new IdempotentTransportHandler(delegate, { maxEntries: 8, ttlMs: 60_000 }),
      limits
    );
    const first = await connect(transport);
    await transport.unary({ connection: first, message: invoke(1), deadlineAt: Date.now() + 1000 });
    await transport.disconnect(first);
    const second = await connect(transport);
    await transport.unary({ connection: second, message: invoke(1), deadlineAt: Date.now() + 1000 });
    expect(calls).toBe(1);
  });

  it("coalesces concurrent retries and rejects key reuse with different input", async () => {
    let calls = 0;
    let release!: () => void;
    const delegate: TransportHandler = {
      unary: async value => {
        calls += 1;
        await new Promise<void>(resolve => (release = resolve));
        return value;
      }
    };
    const transport = new InProcessMuseHostTransport(
      new IdempotentTransportHandler(delegate, { maxEntries: 8, ttlMs: 60_000 }),
      limits
    );
    const connection = await connect(transport);
    const first = transport.unary({
      connection,
      message: invoke(1),
      deadlineAt: Date.now() + 1000
    });
    const retry = transport.unary({
      connection,
      message: invoke(1),
      deadlineAt: Date.now() + 1000
    });
    await vi.waitFor(() => expect(calls).toBe(1));
    release();
    await Promise.all([first, retry]);
    await expect(
      transport.unary({
        connection,
        message: invoke(2),
        deadlineAt: Date.now() + 1000
      })
    ).rejects.toMatchObject({ code: "INVALID_ENVELOPE" });
    expect(calls).toBe(1);
  });
});
