import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Context } from "@deepseek-ai/cordis";
import Loader from "@deepseek-ai/cordis-plugin-loader";
import { describe, expect, it, vi } from "vitest";
import {
  HARD_LIMITS,
  InProcessMuseHostTransport,
  type BridgeEventPayload,
  type InvokeRequestPayload,
  type TransportHandler,
  type WireEnvelope
} from "../src/index.js";
import {
  MuseHostConnectorService,
  MuseHostService,
  clientCorrelationFromDsh,
  type MuseHostConnectorResult
} from "../src/dsh/index.js";

const hostSessionId = "host-session.1";
const hostGeneration = "host.1";

const response = (
  request: WireEnvelope,
  kind: WireEnvelope["kind"],
  payload: unknown
): WireEnvelope => ({
  protocol: "muse-bridge",
  major: 1,
  minor: 0,
  kind,
  ...(request.requestId === undefined ? {} : { requestId: request.requestId }),
  ...(kind === "hello.response" ? {} : { hostSessionId }),
  sentAt: Date.now(),
  payload
} as unknown as WireEnvelope);

const hello = (request: WireEnvelope): WireEnvelope => response(request, "hello.response", {
  ok: true,
  value: {
    selectedVersion: { major: 1, minor: 0 },
    features: [],
    limits: HARD_LIMITS,
    hostSessionId,
    hostGeneration,
    serverTime: Date.now(),
    clockSkewToleranceMs: 1000,
    authorityRevision: "authority.1"
  }
});

class TestConnector extends MuseHostConnectorService {
  constructor(ctx: Context, private readonly result: MuseHostConnectorResult) {
    super(ctx);
  }

  open(signal: AbortSignal): Promise<MuseHostConnectorResult> {
    if (signal.aborted) return Promise.reject(signal.reason);
    return Promise.resolve(this.result);
  }
}

describe("DSH public identity projection", () => {
  it("maps agent/tool identity without exposing non-opaque tool ids", () => {
    const signal = new AbortController().signal;
    const projected = clientCorrelationFromDsh({
      agent: { id: "session.1" },
      turn: 2,
      step: 3,
      toolCallId: "call:code:1",
      signal
    });
    expect(projected).toMatchObject({
      sessionRef: "session.1",
      turnRef: "turn.2",
      stepRef: "step.3"
    });
    expect(projected.toolCallRef).toMatch(/^toolcall\.[0-9a-f]{32}$/u);
    expect(projected.toolCallRef).not.toContain("call:code:1");
  });
});

describe("MuseHostService Cordis lifecycle", () => {
  it("remains pending without the launcher connector instead of crashing Root", async () => {
    const ctx = new Context();
    const fiber = ctx.plugin(MuseHostService, { autoStart: false });
    expect(ctx.get("museHost")).toBeUndefined();
    expect(ctx.registry.size).toBe(1);
    await fiber.dispose();
    expect(ctx.registry.size).toBe(0);
  });

  it("negotiates, forwards DSH correlation/events, and drains on Fiber dispose", async () => {
    let activeStreams = 0;
    let seenInvoke: WireEnvelope | undefined;
    const event: BridgeEventPayload = {
      subscriptionId: "subscription.1" as never,
      cursor: "cursor.1" as never,
      occurredAt: Date.now(),
      hostGeneration: hostGeneration as never,
      data: {
        eventKind: "binding.invalidated",
        bindingId: "binding.1" as never,
        reason: "revoked"
      }
    };
    const handler: TransportHandler = {
      unary: async request => {
        if (request.kind === "hello.request") return hello(request);
        if (request.kind === "invoke.request") {
          seenInvoke = request;
          const payload = request.payload as Record<string, unknown>;
          return response(request, "invoke.response", {
            ok: true,
            value: { accepted: true },
            receipt: {
              receiptId: "receipt.1",
              requestId: request.requestId,
              traceId: payload.traceId,
              hostSessionId,
              hostGeneration,
              bindingId: payload.bindingId,
              operationId: payload.operationId,
              policyDecision: "allow",
              status: "applied_local",
              issuedAt: Date.now(),
              clientCorrelation: payload.clientCorrelation
            }
          });
        }
        throw new Error(`unexpected unary ${request.kind}`);
      },
      stream: async function* (request, signal) {
        activeStreams += 1;
        try {
          yield response(request, "subscribe.response", {
            ok: true,
            value: { subscriptionId: "subscription.1", startCursor: "cursor.0" }
          });
          yield {
            protocol: "muse-bridge",
            major: 1,
            minor: 0,
            kind: "bridge.event",
            hostSessionId,
            sentAt: Date.now(),
            payload: event
          } as unknown as WireEnvelope;
          await new Promise<void>(resolveAbort => {
            if (signal.aborted) resolveAbort();
            else signal.addEventListener("abort", () => resolveAbort(), { once: true });
          });
        } finally {
          activeStreams -= 1;
        }
      }
    };
    const transport = new InProcessMuseHostTransport(
      handler,
      {
        maxPayloadBytes: HARD_LIMITS.maxMessageBytes,
        maxResponseBytes: HARD_LIMITS.maxMessageBytes,
        maxConcurrentRequests: 8,
        maxConcurrentStreams: 1,
        maxDeadlineHorizonMs: 10_000
      },
      hostGeneration
    );
    const ctx = new Context();
    await ctx.plugin(TestConnector, {
      transport,
      proof: { runtimeInstanceId: "runtime.1", nonce: transport.launch.nonce }
    });
    const events: BridgeEventPayload[] = [];
    ctx.on("museHost/binding-invalidated", value => events.push(value));
    const fiber = await ctx.plugin(MuseHostService, {
      requestTimeoutMs: 1000,
      subscriptionLifetimeMs: 5000,
      reconnectDelayMs: 10
    });
    await vi.waitFor(() => expect(ctx.museHost.snapshot().state).toBe("ready"));
    await vi.waitFor(() => expect(events).toHaveLength(1));

    const invoke: InvokeRequestPayload = {
      traceId: "trace.1" as never,
      bindingId: "binding.1" as never,
      operationId: "sample.write",
      input: { value: 1 },
      deadlineAt: Date.now() + 1000,
      cancellationId: "cancel.1"
    };
    const outcome = await ctx.museHost.invoke(invoke, {
      agent: { id: "session.1" },
      turn: 1,
      step: 2,
      toolCallId: "tool:1",
      signal: new AbortController().signal
    });
    expect(outcome).toMatchObject({ ok: true, value: { accepted: true } });
    expect(seenInvoke?.payload).toMatchObject({
      clientCorrelation: {
        sessionRef: "session.1",
        turnRef: "turn.1",
        stepRef: "step.2"
      }
    });

    const service = ctx.museHost;
    await fiber.dispose();
    expect(ctx.get("museHost")).toBeUndefined();
    expect(service.snapshot().state).toBe("closed");
    expect(activeStreams).toBe(0);
  });

  it("stays mounted with a safe diagnostic when Host connection fails", async () => {
    class FailingConnector extends MuseHostConnectorService {
      open(): Promise<MuseHostConnectorResult> {
        return Promise.reject(new Error("secret endpoint must not be logged"));
      }
    }
    const ctx = new Context();
    await ctx.plugin(FailingConnector);
    const fiber = await ctx.plugin(MuseHostService, { reconnectDelayMs: 10_000 });
    await vi.waitFor(() => expect(ctx.museHost.snapshot()).toMatchObject({
      state: "disconnected",
      diagnosticCode: "CONNECT_FAILED"
    }));
    expect(ctx.get("museHost")).toBeDefined();
    await fiber.dispose();
    expect(ctx.get("museHost")).toBeUndefined();
  });

  it("loads and unloads through the real Cordis Loader seam", async () => {
    const handler: TransportHandler = {
      unary: async request => {
        if (request.kind === "hello.request") return hello(request);
        throw new Error(`unexpected unary ${request.kind}`);
      }
    };
    const transport = new InProcessMuseHostTransport(
      handler,
      { maxPayloadBytes: HARD_LIMITS.maxMessageBytes, maxConcurrentRequests: 2 },
      hostGeneration
    );
    class LoaderConnector extends MuseHostConnectorService {
      open(signal: AbortSignal): Promise<MuseHostConnectorResult> {
        if (signal.aborted) return Promise.reject(signal.reason);
        return Promise.resolve({
          transport,
          proof: { runtimeInstanceId: "runtime.loader", nonce: transport.launch.nonce }
        });
      }
    }
    const ctx = new Context();
    await ctx.plugin(Loader);
    const modules = new Map<string, unknown>([
      ["@muse/host-connector-test", LoaderConnector],
      ["@muse/host-bridge/dsh", MuseHostService]
    ]);
    ctx.loader.internal = {
      version: "v2",
      async import(specifier: string) {
        const plugin = modules.get(specifier);
        if (plugin === undefined) throw new Error(`unexpected Loader import: ${specifier}`);
        return plugin;
      }
    } as unknown as NonNullable<typeof ctx.loader.internal>;
    await ctx.loader.create({ name: "@muse/host-connector-test" });
    const serviceEntry = await ctx.loader.create({
      name: "@muse/host-bridge/dsh",
      config: { autoStart: false, requestTimeoutMs: 1000 }
    });
    await ctx.loader.await();
    expect(ctx.get("museHost")).toBeDefined();
    const service = ctx.museHost;
    await expect(service.connect()).resolves.toMatchObject({
      hello: { hostGeneration }
    });

    await ctx.loader.remove(serviceEntry);
    await ctx.loader.await();
    expect(ctx.get("museHost")).toBeUndefined();
    expect(service.snapshot().state).toBe("closed");
    await ctx.fiber.dispose();
  });
});

describe("Muse DSH bundle", () => {
  it("declares a secret-free Bridge Service patch", () => {
    const root = resolve(import.meta.dirname, "..");
    const manifest = JSON.parse(readFileSync(resolve(root, "package.json"), "utf8")) as {
      dsh?: { bundle?: { patch?: string } };
    };
    expect(manifest.dsh?.bundle?.patch).toBe("./cordis.patch.yml");
    const patch = readFileSync(resolve(root, "cordis.patch.yml"), "utf8");
    expect(patch).toContain("@muse/host-bridge/dsh");
    const data = patch.split("\n").filter(line => !line.trimStart().startsWith("#")).join("\n");
    expect(data).not.toMatch(/nonce|token|endpoint|secret/iu);
  });
});
