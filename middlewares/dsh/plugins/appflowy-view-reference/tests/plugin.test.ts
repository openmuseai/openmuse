import { Context, Service } from "@deepseek-ai/cordis";
import SystemPrompt from "@deepseek-ai/dsh-system-prompt";
import ToolRuntime from "@deepseek-ai/dsh-tools";
import { CallId, createMessage, createToolResultMessage } from "@deepseek-ai/dsh-llm";
import { Session, SessionId, foldRequestHeader } from "@deepseek-ai/dsh-session";
import { describe, expect, it } from "vitest";
import {
  HARD_LIMITS,
  canonicalizeJson,
  digestSchema,
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
import { MuseProviderSchemas, assertMuseToolSchemas } from "@muse/plugin-kit";
import {
  APPFLOWY_VIEW_REFERENCE_TOOL,
  appFlowyViewReferenceDefinition,
  modelInputSchema,
  providerInputSchema,
  providerOutputSchema
} from "../src/index.js";
import { createAppFlowyViewReferencePlugin } from "../src/dsh.js";

const resource = (schema: JsonValue): SchemaResourceV1 => ({
  sha256: digestSchema(schema),
  byteLength: canonicalizeJson(schema).byteLength,
  draft: "2020-12",
  inline: schema
});

const descriptor: CapabilityDescriptor = {
  descriptorId: "appflowy.view-reference.local" as never,
  revision: "1" as never,
  familyId: "appflowy.view-reference",
  contractVersion: { major: 1, minor: 0 },
  providerInstanceId: "provider.appflowy" as never,
  operations: [{
    operationId: "view.reference.read",
    effect: "read",
    inputSchema: resource(providerInputSchema),
    outputSchema: resource(providerOutputSchema),
    cancellable: true,
    idempotency: "none"
  }],
  events: []
};

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

const safeResult: JsonValue = {
  view: {
    title: "Roadmap",
    titleTruncated: false,
    layout: "document",
    locked: false,
    childCount: 1
  },
  children: [{
    title: "Q3",
    titleTruncated: false,
    layout: "document",
    locked: null
  }],
  page: { returned: 1, hasMore: false }
};

class FakeMuseHost extends Service {
  discoverPayload: JsonValue | undefined;
  invokePayload: InvokeRequestPayload | undefined;

  constructor(ctx: Context) {
    super(ctx, "museHost");
  }

  snapshot(): MuseHostStateSnapshot {
    return { state: "ready", hostGeneration: "host.1" };
  }

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
      this.discoverPayload = payload;
      return Promise.resolve(envelope("discover.response", {
        ok: true,
        value: { registryRevision: "registry.1", descriptors: [descriptor] }
      } as unknown as JsonValue));
    }
    if (kind === "bind.request") {
      return Promise.resolve(envelope("bind.response", {
        ok: true,
        value: {
          bindingId: "binding.1",
          descriptorId: descriptor.descriptorId,
          descriptorRevision: descriptor.revision,
          providerInstanceId: descriptor.providerInstanceId,
          hostGeneration: "host.1",
          scopeRef: "scope.current-view",
          expiresAt: Date.now() + 60_000,
          operations: [{
            operationId: "view.reference.read",
            effect: "read",
            inputSchemaDigest: descriptor.operations[0]?.inputSchema.sha256,
            outputSchemaDigest: descriptor.operations[0]?.outputSchema.sha256,
            cancellable: true,
            idempotency: "none"
          }]
        }
      } as unknown as JsonValue));
    }
    throw new Error(`unexpected request ${kind}`);
  }

  invoke(payload: InvokeRequestPayload, _source: DshInvocationSource): Promise<InvokeResponsePayload> {
    this.invokePayload = payload;
    return Promise.resolve({
      ok: true,
      value: safeResult,
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
  }

  close(): Promise<void> {
    return Promise.resolve();
  }
}

describe("AppFlowy current View reference Plugin", () => {
  it("pins exact Provider schemas and exposes no target identifier to the model", async () => {
    const tool = appFlowyViewReferenceDefinition.targets[0]?.tools[0];
    expect(tool?.parameters).toEqual(modelInputSchema);
    expect(JSON.stringify(tool?.parameters)).not.toMatch(/workspace|viewId|actor|binding/iu);
    expect(tool?.adapter.accepts({
      descriptor,
      operation: descriptor.operations[0]!,
      inputSchema: providerInputSchema,
      outputSchema: providerOutputSchema
    })).toBe(true);
    assertMuseToolSchemas(tool!.parameters, tool!.output);
    const schemas = new MuseProviderSchemas(undefined, HARD_LIMITS);
    await expect(schemas.materialize(resource(providerInputSchema), new AbortController().signal)).resolves.toBeDefined();
    await expect(schemas.materialize(resource(providerOutputSchema), new AbortController().signal)).resolves.toBeDefined();
  });

  it("binds current selection, executes through the real ToolRuntime, and disposes cleanly", async () => {
    const ctx = new Context();
    await ctx.plugin(SystemPrompt);
    await ctx.plugin(ToolRuntime);
    await ctx.plugin(FakeMuseHost);
    const diagnostics: unknown[] = [];
    const fiber = await ctx.plugin(createAppFlowyViewReferencePlugin({
      onDiagnostic: diagnostic => diagnostics.push(diagnostic)
    }));
    expect(ctx.tools.schemas().map(tool => tool.name), JSON.stringify(diagnostics)).toEqual([
      APPFLOWY_VIEW_REFERENCE_TOOL
    ]);
    expect((ctx.museHost as FakeMuseHost).discoverPayload).toMatchObject({
      scopeHint: { refs: { "appflowy.selection": "current" } }
    });

    const result = await ctx.tools.execute({
      callId: "call.1" as never,
      name: APPFLOWY_VIEW_REFERENCE_TOOL,
      arguments: { limit: 20 },
      signal: new AbortController().signal
    });
    expect(result).toMatchObject({ isError: false, value: safeResult });
    expect((ctx.museHost as FakeMuseHost).invokePayload?.input).toEqual({ limit: 20 });

    const session = Session.create(SessionId("appflowy-view-reference"));
    session.append("turn/start", { turn: 1 });
    session.append("request/header", {
      header: {
        config: { provider: "mock", model: "mock" },
        tools: ctx.tools.schemas()
      },
      reason: "initial"
    });
    session.append("assistant/message", {
      turn: 1,
      step: 1,
      message: createMessage({
        role: "assistant",
        content: [{
          type: "tool-call",
          id: CallId("call.1"),
          name: APPFLOWY_VIEW_REFERENCE_TOOL,
          arguments: JSON.stringify({ limit: 20 })
        }],
        source: { kind: "model", provider: "mock", model: "mock" }
      })
    }, { surfaceOp: "append" });
    session.append("tool/result", {
      turn: 1,
      step: 1,
      message: createToolResultMessage({
        callId: CallId("call.1"),
        content: [{ type: "text", text: JSON.stringify(result.value) }],
        isError: false
      })
    }, { surfaceOp: "append" });
    session.append("turn/end", { turn: 1, reason: { kind: "completed" } });
    const replayed = Session.create(SessionId("appflowy-view-reference-replay"), structuredClone(session.events));
    expect(foldRequestHeader(replayed.events)?.tools?.map(tool => tool.name)).toEqual([
      APPFLOWY_VIEW_REFERENCE_TOOL
    ]);
    expect(replayed.deriveMessages()).toEqual(session.deriveMessages());
    expect(JSON.stringify(replayed.deriveMessages())).toContain("Roadmap");

    await fiber.dispose();
    expect(ctx.tools.schemas()).toEqual([]);
    await ctx.fiber.dispose();
  });
});
