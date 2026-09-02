import { describe, expect, it } from "vitest";
import {
  DesktopMuseHostTransport,
  encodeLengthDelimitedFrame,
  type DesktopCarrier,
  type DesktopCarrierConnection,
  type DesktopCarrierEndpoint,
  type TransportConnection,
  type TransportLaunch,
  type WireEnvelope
} from "../src/index.js";

const encoder = new TextEncoder();

const launch: TransportLaunch = {
  endpoint: "/tmp/muse-desktop-stream-test.sock",
  nonce: "nonce.1",
  hostGeneration: "host.1"
};

const credential: TransportConnection = {
  connectionId: "connection.1",
  token: "token.1",
  runtimeInstanceId: "runtime.1",
  hostGeneration: "host.1",
  expiresAt: Date.now() + 60_000
};

const envelope = (kind: WireEnvelope["kind"], payload: WireEnvelope["payload"]): WireEnvelope => ({
  protocol: "muse-bridge",
  major: 1,
  minor: 0,
  kind,
  hostSessionId: "host-session.1" as never,
  sentAt: Date.now(),
  payload
});

const controlFrame = (message: WireEnvelope): Uint8Array =>
  encodeLengthDelimitedFrame(encoder.encode(JSON.stringify({ ok: true, message })), 1024 * 1024);

class StreamCarrier implements DesktopCarrier {
  written?: Uint8Array;
  closed = false;

  constructor(private readonly frames: readonly Uint8Array[]) {}

  async connect(_endpoint: DesktopCarrierEndpoint, signal: AbortSignal): Promise<DesktopCarrierConnection> {
    const frames = this.frames;
    const self = this;
    return {
      endpoint: _endpoint,
      async write(frame) { self.written = frame; },
      async close() { self.closed = true; },
      readable: {
        async *[Symbol.asyncIterator]() {
          for (const frame of frames) yield frame;
          await new Promise<void>(resolve => {
            if (signal.aborted) resolve();
            else signal.addEventListener("abort", () => resolve(), { once: true });
          });
        }
      }
    };
  }
}

describe("desktop stream transport", () => {
  it("keeps a framed carrier open and yields every Host message until cancellation", async () => {
    const acknowledgement = envelope("subscribe.response", {
      ok: true,
      value: { subscriptionId: "subscription.1", startCursor: "cursor.0" }
    });
    const event = envelope("bridge.event", {
      subscriptionId: "subscription.1",
      cursor: "cursor.1",
      occurredAt: Date.now(),
      hostGeneration: "host.1",
      data: { eventKind: "stream.gap", reason: "retention_gap" }
    });
    const carrier = new StreamCarrier([controlFrame(acknowledgement), controlFrame(event)]);
    const transport = new DesktopMuseHostTransport(launch, carrier);
    const controller = new AbortController();
    const stream = transport.stream({
      connection: credential,
      message: envelope("subscribe.request", {
        filters: { eventKinds: ["stream.gap"] },
        deadlineAt: Date.now() + 60_000
      }),
      deadlineAt: Date.now() + 60_000,
      signal: controller.signal
    });
    await expect(stream.next()).resolves.toMatchObject({ value: acknowledgement, done: false });
    await expect(stream.next()).resolves.toMatchObject({ value: event, done: false });
    controller.abort();
    await expect(stream.next()).resolves.toEqual({ value: undefined, done: true });
    expect(carrier.written).toBeDefined();
    expect(carrier.closed).toBe(true);
  });

  it("transport close aborts all active streams", async () => {
    const carrier = new StreamCarrier([]);
    const transport = new DesktopMuseHostTransport(launch, carrier);
    const stream = transport.stream({
      connection: credential,
      message: envelope("subscribe.request", {
        filters: { eventKinds: ["stream.gap"] }, deadlineAt: Date.now() + 60_000
      }),
      deadlineAt: Date.now() + 60_000
    });
    const pending = stream.next();
    await transport.close();
    await expect(pending).resolves.toEqual({ value: undefined, done: true });
  });
});
