import { Context, Service } from "@deepseek-ai/cordis";
import Loader from "@deepseek-ai/cordis-plugin-loader";
import SystemPrompt from "@deepseek-ai/dsh-system-prompt";
import ToolRuntime from "@deepseek-ai/dsh-tools";
import { describe, expect, it, vi } from "vitest";
import {
  HARD_LIMITS,
  canonicalizeJson,
  digestSchema,
  type BridgeEventPayload,
  type CapabilityDescriptor,
  type InvokeRequestPayload,
  type InvokeResponsePayload,
  type JsonValue,
  type SchemaResourceV1,
  type WireEnvelope
} from "@muse/host-bridge";
import type {
  DshInvocationSource,
  MuseHostReadySession,
  MuseHostRequestKind,
  MuseHostRequestOptions,
  MuseHostStateSnapshot
} from "@muse/host-bridge/dsh";
import { createMusePlugin } from "../src/dsh/index.js";
import type { MusePluginDefinition, MusePluginDiagnostic } from "../src/index.js";

const fullSchema = (body: Record<string, JsonValue>): SchemaResourceV1 => {
  const schema = { $schema: "https://json-schema.org/draft/2020-12/schema", ...body } as JsonValue;
  return {
    sha256: digestSchema(schema),
    byteLength: canonicalizeJson(schema).byteLength,
    draft: "2020-12",
    inline: schema
  };
};

const inputResource = fullSchema({
  type: "object",
  properties: { text: { type: "string" } },
  required: ["text"],
  additionalProperties: false
});
const outputResource = fullSchema({
  type: "object",
  properties: { echoed: { type: "string" } },
  required: ["echoed"],
  additionalProperties: false
});

const descriptor = (major = 1, minor = 2): CapabilityDescriptor => ({
  descriptorId: "descriptor.echo" as never,
  revision: "revision.1" as never,
  familyId: "example.echo",
  contractVersion: { major, minor },
  providerInstanceId: "provider.1" as never,
  operations: [{
    operationId: "echo.read",
    effect: "read",
    inputSchema: inputResource,
    outputSchema: outputResource,
    cancellable: true,
    idempotency: "none"
  }],
  events: []
});

const definition = (version = "1.0.0"): MusePluginDefinition => ({
  pluginId: "example-echo",
  version,
  bridgeMajor: 1,
  targets: [{
    familyId: "example.echo",
    contract: { major: 1, minMinor: 0, maxMinor: 3 },
    requiredOperations: ["echo.read"],
    tools: [{
      name: "example_echo",
      description: `Echo through a compatible provider (${version}).`,
      operationId: "echo.read",
      parameters: {
        type: "object",
        properties: { message: { type: "string" } },
        required: ["message"],
        additionalProperties: false
      },
      output: {
        type: "object",
        properties: { message: { type: "string" } },
        required: ["message"],
        additionalProperties: false
      },
      adapter: {
        accepts: contract => contract.inputSchema !== null && contract.outputSchema !== null,
        toProviderInput: args => ({ text: (args as { message: string }).message }),
        fromProviderOutput: value => ({ message: (value as { echoed: string }).echoed })
      }
    }]
  }]
});

const envelope = (kind: WireEnvelope["kind"], payload: JsonValue): WireEnvelope => ({
  protocol: "muse-bridge",
  major: 1,
  minor: 0,
  kind,
  requestId: "request.1" as never,
  hostSessionId: "host-session.1" as never,
  sentAt: Date.now(),
  payload
});

class FakeMuseHost extends Service {
  descriptors: CapabilityDescriptor[] = [descriptor()];
  discoverCount = 0;
  bindCount = 0;
  invokeCount = 0;
  invokeImpl: (payload: InvokeRequestPayload, source: DshInvocationSource) => Promise<InvokeResponsePayload> =
    async payload => ({
      ok: true,
      value: { echoed: (payload.input as { text: string }).text },
      receipt: {
        receiptId: "receipt.1" as never,
        requestId: "request.1" as never,
        traceId: payload.traceId,
        hostSessionId: "host-session.1" as never,
        hostGeneration: "host.1" as never,
        bindingId: payload.bindingId,
        operationId: payload.operationId,
        policyDecision: "allow",
        status: "applied_local",
        issuedAt: Date.now()
      }
    });

  constructor(ctx: Context) {
    super(ctx, "museHost");
  }

  snapshot(): MuseHostStateSnapshot { return { state: "ready", hostGeneration: "host.1" }; }

  connect(): Promise<MuseHostReadySession> {
    return Promise.resolve({
      transport: {} as never,
      connection: {} as never,
      hello: {
        selectedVersion: { major: 1, minor: 0 },
        features: [],
        limits: HARD_LIMITS,
        hostSessionId: "host-session.1" as never,
        hostGeneration: "host.1" as never,
        serverTime: Date.now(),
        clockSkewToleranceMs: 1000,
        authorityRevision: "authority.1"
      }
    });
  }

  request(kind: MuseHostRequestKind, payload: JsonValue, _options?: MuseHostRequestOptions): Promise<WireEnvelope> {
    if (kind === "discover.request") {
      this.discoverCount += 1;
      return Promise.resolve(envelope("discover.response", {
        ok: true,
        value: { registryRevision: `registry.${this.discoverCount}`, descriptors: this.descriptors }
      } as unknown as JsonValue));
    }
    if (kind === "bind.request") {
      this.bindCount += 1;
      const request = payload as unknown as { descriptorId: string; descriptorRevision: string; operationIds: string[] };
      const selected = this.descriptors.find(item => item.descriptorId === request.descriptorId);
      if (selected === undefined) throw new Error("unknown descriptor");
      return Promise.resolve(envelope("bind.response", {
        ok: true,
        value: {
          bindingId: `binding.${this.bindCount}`,
          descriptorId: request.descriptorId,
          descriptorRevision: request.descriptorRevision,
          providerInstanceId: selected.providerInstanceId,
          hostGeneration: "host.1",
          scopeRef: "scope.1",
          expiresAt: Date.now() + 60_000,
          operations: selected.operations.map(operation => ({
            operationId: operation.operationId,
            effect: operation.effect,
            inputSchemaDigest: operation.inputSchema.sha256,
            outputSchemaDigest: operation.outputSchema.sha256,
            cancellable: operation.cancellable,
            idempotency: operation.idempotency
          }))
        }
      } as unknown as JsonValue));
    }
    throw new Error(`unexpected request ${kind}`);
  }

  invoke(payload: InvokeRequestPayload, source: DshInvocationSource): Promise<InvokeResponsePayload> {
    this.invokeCount += 1;
    return this.invokeImpl(payload, source);
  }

  close(): Promise<void> { return Promise.resolve(); }
}

const setup = async () => {
  const ctx = new Context();
  await ctx.plugin(SystemPrompt);
  await ctx.plugin(ToolRuntime);
  await ctx.plugin(FakeMuseHost);
  return { ctx, host: ctx.museHost as FakeMuseHost };
};

const invalidation = (): BridgeEventPayload => ({
  subscriptionId: "subscription.1" as never,
  cursor: "cursor.1" as never,
  occurredAt: Date.now(),
  hostGeneration: "host.1" as never,
  data: { eventKind: "binding.invalidated", bindingId: "binding.1" as never, reason: "revoked" }
});

describe("Muse Plugin discovery and DSH Tool lifecycle", () => {
  it("rejects a malformed manifest before Cordis mount or Host discovery", async () => {
    const { ctx, host } = await setup();
    const malformed = {
      ...definition(),
      pluginId: "Invalid Plugin",
      targets: [{
        ...definition().targets[0]!,
        tools: [definition().targets[0]!.tools[0]!, { ...definition().targets[0]!.tools[0]! }]
      }]
    };
    expect(() => createMusePlugin(malformed)).toThrowError(/pluginId/u);
    expect(host.discoverCount).toBe(0);
    expect(ctx.tools.schemas()).toEqual([]);
  });

  it("discovers, adapts both DTO boundaries, and exposes only the model contract", async () => {
    const { ctx, host } = await setup();
    const fiber = await ctx.plugin(createMusePlugin(definition()));
    expect(ctx.tools.schemas()).toEqual([{
      name: "example_echo",
      description: "Echo through a compatible provider (1.0.0).",
      parameters: definition().targets[0]?.tools[0]?.parameters
    }]);
    expect(JSON.stringify(ctx.tools.schemas())).not.toMatch(/binding\.|descriptor\.|provider\.|receipt/iu);

    const result = await ctx.tools.execute({
      callId: "call.1" as never,
      name: "example_echo",
      arguments: { message: "hello" },
      signal: new AbortController().signal
    });
    expect(result).toMatchObject({ isError: false, value: { message: "hello" } });
    expect(host.invokeCount).toBe(1);
    await fiber.dispose();
  });

  it("fails closed for an unknown contract major and missing required operation", async () => {
    const { ctx, host } = await setup();
    const diagnostics: MusePluginDiagnostic[] = [];
    host.descriptors = [descriptor(2, 0)];
    const fiber = await ctx.plugin(createMusePlugin(definition(), { onDiagnostic: value => diagnostics.push(value) }));
    expect(ctx.tools.schemas()).toEqual([]);
    expect(host.bindCount).toBe(0);
    expect(diagnostics.at(-1)).toMatchObject({ phase: "incompatible", code: "REQUIRED_TARGET_UNAVAILABLE" });
    await fiber.dispose();
  });

  it("validates model input, Provider output, and mapped model output at separate boundaries", async () => {
    const { ctx, host } = await setup();
    const fiber = await ctx.plugin(createMusePlugin(definition()));
    const badArgs = await ctx.tools.execute({
      callId: "call.bad-args" as never,
      name: "example_echo",
      arguments: { wrong: true },
      signal: new AbortController().signal
    });
    expect(badArgs).toMatchObject({ isError: true });
    expect(host.invokeCount).toBe(0);

    host.invokeImpl = async payload => ({
      ok: true,
      value: { unexpected: true },
      receipt: {
        receiptId: "receipt.bad" as never,
        requestId: "request.1" as never,
        traceId: payload.traceId,
        hostSessionId: "host-session.1" as never,
        hostGeneration: "host.1" as never,
        bindingId: payload.bindingId,
        operationId: payload.operationId,
        policyDecision: "allow",
        status: "applied_local",
        issuedAt: Date.now()
      }
    });
    const badProviderOutput = await ctx.tools.execute({
      callId: "call.bad-output" as never,
      name: "example_echo",
      arguments: { message: "hello" },
      signal: new AbortController().signal
    });
    expect(badProviderOutput).toMatchObject({ isError: true });
    expect(host.invokeCount).toBe(1);
    await fiber.dispose();
  });

  it("revokes immediately, cancels in-flight work, drains, and rebinds", async () => {
    const { ctx, host } = await setup();
    const fiber = await ctx.plugin(createMusePlugin(definition()));
    let observedAbort = false;
    host.invokeImpl = (_payload, source) => new Promise((_resolve, reject) => {
      source.signal.addEventListener("abort", () => {
        observedAbort = true;
        reject(source.signal.reason);
      }, { once: true });
    });
    const running = ctx.tools.execute({
      callId: "call.pending" as never,
      name: "example_echo",
      arguments: { message: "pending" },
      signal: new AbortController().signal
    });
    await vi.waitFor(() => expect(host.invokeCount).toBe(1));
    ctx.emit("museHost/event", invalidation());
    expect(ctx.tools.schemas()).toEqual([]);
    await expect(running).resolves.toMatchObject({ isError: true });
    expect(observedAbort).toBe(true);
    await vi.waitFor(() => expect(ctx.tools.schemas().map(tool => tool.name)).toEqual(["example_echo"]));
    expect(host.bindCount).toBe(2);
    await fiber.dispose();
  });

  it("disposes every contribution and ignores later Host events", async () => {
    const { ctx, host } = await setup();
    const fiber = await ctx.plugin(createMusePlugin(definition()));
    const before = host.discoverCount;
    await fiber.dispose();
    expect(ctx.tools.schemas()).toEqual([]);
    ctx.emit("museHost/event", invalidation());
    await new Promise(resolve => setTimeout(resolve, 0));
    expect(host.discoverCount).toBe(before);
  });

  it("loads, removes, and replaces a Plugin through the real Cordis Loader", async () => {
    const { ctx } = await setup();
    await ctx.plugin(Loader);
    const modules = new Map<string, unknown>([
      ["@muse/example-v1", createMusePlugin(definition("1.0.0"))],
      ["@muse/example-v2", createMusePlugin(definition("2.0.0"))]
    ]);
    ctx.loader.internal = {
      version: "v2",
      async import(specifier: string) {
        const plugin = modules.get(specifier);
        if (plugin === undefined) throw new Error(`unexpected Loader import: ${specifier}`);
        return plugin;
      }
    } as unknown as NonNullable<typeof ctx.loader.internal>;
    const v1 = await ctx.loader.create({ name: "@muse/example-v1" });
    await ctx.loader.await();
    expect(ctx.tools.schemas()[0]?.description).toContain("1.0.0");
    await ctx.loader.remove(v1);
    await ctx.loader.await();
    expect(ctx.tools.schemas()).toEqual([]);
    await ctx.loader.create({ name: "@muse/example-v2" });
    await ctx.loader.await();
    expect(ctx.tools.schemas()).toHaveLength(1);
    expect(ctx.tools.schemas()[0]?.description).toContain("2.0.0");
    await ctx.fiber.dispose();
  });
});
