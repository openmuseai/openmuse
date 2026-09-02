import { randomUUID } from "node:crypto";
import type { Context } from "@deepseek-ai/cordis";
import type {} from "@muse/host-bridge/dsh";
import type { MuseHostServiceApi } from "@muse/host-bridge/dsh";
import {
  HARD_LIMITS,
  canonicalizeJson,
  digestInput,
  type BindRequestPayload,
  type BridgeEventPayload,
  type BridgeLimits,
  type BridgeResult,
  type CapabilityDescriptor,
  type DiscoverResponseValue,
  type InvokeRequestPayload,
  type JsonValue,
  type OperationDescriptor,
  type WireBinding,
  type WireEnvelope
} from "@muse/host-bridge";
import { clientCorrelationFromDsh, getMuseApprovalProofRequester, resolveMusePolicy, type MuseApprovalProofRequester } from "@muse/host-bridge/dsh";
import {
  validateJsonSchemaValue,
  type JsonSchemaNode,
  type ObjectJsonSchema,
  type ToolDefinition,
  type ToolRuntime,
  type ToolRunContext
} from "@deepseek-ai/dsh-tools";
import { MusePluginCompatibilityError, MuseToolInvocationError } from "../errors.js";
import { validateMusePluginDefinition } from "../manifest.js";
import { assertMuseToolSchemas, MuseProviderSchemas } from "../schema.js";
import type {
  MuseCapabilityTarget,
  MusePluginDefinition,
  MusePluginDiagnostic,
  MusePluginRuntimeConfig,
  MusePreparedTool,
  MuseProviderOperationContract,
  MuseToolContribution
} from "../types.js";

declare module "@deepseek-ai/cordis" {
  interface Context {
    tools: ToolRuntime;
    museHost: MuseHostServiceApi;
  }
  interface Events {
    "museHost/event"(event: BridgeEventPayload): void;
  }
}

interface ResolvedConfig {
  readonly discoveryTimeoutMs: number;
  readonly invocationTimeoutMs: number;
  readonly pageSize: number;
  readonly scopeHint?: MusePluginRuntimeConfig["scopeHint"];
  readonly resolveSchema?: MusePluginRuntimeConfig["resolveSchema"];
  readonly onDiagnostic?: MusePluginRuntimeConfig["onDiagnostic"];
  readonly approvalProofRequester?: MuseApprovalProofRequester;
}

interface PreparedGeneration {
  readonly tools: readonly MusePreparedTool[];
  readonly limits: BridgeLimits;
}

const positive = (value: number | undefined, fallback: number, name: string): number => {
  const result = value ?? fallback;
  if (!Number.isSafeInteger(result) || result <= 0) throw new TypeError(`${name} must be a positive integer`);
  return result;
};

const configOf = (config: MusePluginRuntimeConfig): ResolvedConfig => ({
  discoveryTimeoutMs: positive(config.discoveryTimeoutMs, 15_000, "discoveryTimeoutMs"),
  invocationTimeoutMs: positive(config.invocationTimeoutMs, 30_000, "invocationTimeoutMs"),
  pageSize: positive(config.pageSize, 100, "pageSize"),
  ...(config.scopeHint === undefined ? {} : { scopeHint: config.scopeHint }),
  ...(config.resolveSchema === undefined ? {} : { resolveSchema: config.resolveSchema }),
  ...(config.onDiagnostic === undefined ? {} : { onDiagnostic: config.onDiagnostic }),
  ...(config.approvalProofRequester === undefined ? {} : { approvalProofRequester: config.approvalProofRequester })
});

const unwrap = <T>(envelope: WireEnvelope, expectedKind: string): T => {
  if (envelope.kind !== expectedKind) throw new Error(`expected ${expectedKind}, received ${envelope.kind}`);
  const payload = envelope.payload as unknown as BridgeResult<T>;
  if (payload === null || typeof payload !== "object" || typeof payload.ok !== "boolean") {
    throw new Error(`${expectedKind} payload is not a BridgeResult`);
  }
  if (!payload.ok) throw new MuseToolInvocationError(payload.error);
  return payload.value;
};

const unique = <T>(values: readonly T[]): T[] => [...new Set(values)];

const operationMap = (descriptor: CapabilityDescriptor): ReadonlyMap<string, OperationDescriptor> =>
  new Map(descriptor.operations.map(operation => [operation.operationId, operation]));

const acceptsVersion = (descriptor: CapabilityDescriptor, target: MuseCapabilityTarget): boolean =>
  descriptor.familyId === target.familyId
  && descriptor.contractVersion.major === target.contract.major
  && descriptor.contractVersion.minor >= target.contract.minMinor
  && descriptor.contractVersion.minor <= target.contract.maxMinor;

const jsonArgs = (args: unknown): JsonValue => args as JsonValue;

const content = (tool: MuseToolContribution, args: JsonValue, value: JsonValue): { type: "text"; text: string }[] =>
  tool.render === undefined
    ? [{ type: "text", text: JSON.stringify(value, null, 2) }]
    : [...tool.render(args, value)];

export class MusePluginRuntime {
  private readonly config: ResolvedConfig;
  private disposed = false;
  private epoch = 0;
  private refreshQueue: Promise<void> = Promise.resolve();
  private unregisterTools: (() => void)[] = [];
  private bindingIds = new Set<string>();
  private descriptorIds = new Set<string>();
  private active = new Set<Promise<unknown>>();
  private invocationAbort = new AbortController();
  private expiryTimer: ReturnType<typeof setTimeout> | undefined;

  constructor(
    private readonly ctx: Context,
    private readonly definition: MusePluginDefinition,
    config: MusePluginRuntimeConfig = {}
  ) {
    validateMusePluginDefinition(definition);
    const approval = config.approvalProofRequester ?? getMuseApprovalProofRequester(ctx);
    this.config = configOf({
      ...config,
      ...(approval === undefined ? {} : { approvalProofRequester: approval })
    });
  }

  async start(): Promise<void> {
    this.ctx.on("museHost/event", event => this.onHostEvent(event));
    await this.enqueueRefresh("START");
  }

  async dispose(): Promise<void> {
    if (this.disposed) return;
    this.disposed = true;
    this.epoch += 1;
    this.revokeTools();
    await this.cancelAndDrain();
    await this.refreshQueue.catch(() => undefined);
    this.diagnostic("disposed", "DISPOSED");
  }

  private onHostEvent(event: BridgeEventPayload): void {
    const kind = event.data.eventKind;
    if (kind !== "binding.invalidated" && kind !== "descriptor.changed" && kind !== "host.generation.changed") return;
    if (kind === "binding.invalidated" && !this.bindingIds.has(event.data.bindingId)) return;
    if (kind === "descriptor.changed" && this.descriptorIds.size > 0 && !this.descriptorIds.has(event.data.descriptorId)) return;
    this.epoch += 1;
    this.revokeTools();
    void this.cancelAndDrain();
    void this.enqueueRefresh(kind.toUpperCase().replaceAll(".", "_"));
  }

  private enqueueRefresh(reason: string): Promise<void> {
    this.refreshQueue = this.refreshQueue
      .catch(() => undefined)
      .then(() => this.refresh(reason));
    return this.refreshQueue;
  }

  private async refresh(reason: string): Promise<void> {
    if (this.disposed) return;
    const epoch = ++this.epoch;
    this.revokeTools();
    await this.cancelAndDrain();
    if (this.disposed || epoch !== this.epoch) return;
    this.invocationAbort = new AbortController();
    this.diagnostic(reason === "START" ? "discovering" : "refreshing", reason);
    try {
      const prepared = await this.prepare(epoch);
      if (this.disposed || epoch !== this.epoch) return;
      this.publish(prepared);
      this.diagnostic("active", "BOUND");
    } catch (error) {
      if (this.disposed || epoch !== this.epoch) return;
      this.revokeTools();
      const code = error instanceof MusePluginCompatibilityError ? error.code : "DISCOVERY_FAILED";
      this.diagnostic("incompatible", code, error instanceof Error ? error.message : "Unknown compatibility failure");
    }
  }

  private async prepare(epoch: number): Promise<PreparedGeneration> {
    const session = await this.ctx.museHost.connect(this.invocationAbort.signal);
    const limits = session.hello.limits;
    const schemas = new MuseProviderSchemas(this.config.resolveSchema, limits);
    const descriptors = await this.discover(limits);
    const tools: MusePreparedTool[] = [];
    let earliestExpiry = Number.POSITIVE_INFINITY;

    for (const target of this.definition.targets) {
      const targetTools = await this.prepareTarget(target, descriptors, schemas);
      if (targetTools === undefined) {
        if (target.optional === true) continue;
        throw new MusePluginCompatibilityError("REQUIRED_TARGET_UNAVAILABLE", `No compatible provider for ${target.familyId}`);
      }
      tools.push(...targetTools.tools);
      earliestExpiry = Math.min(earliestExpiry, targetTools.binding.expiresAt);
    }

    const names = tools.map(item => item.contribution.name);
    if (new Set(names).size !== names.length) {
      throw new MusePluginCompatibilityError("DUPLICATE_TOOL_NAME", "Plugin contributes duplicate Tool names");
    }
    if (epoch !== this.epoch) throw new MusePluginCompatibilityError("STALE_REFRESH", "Refresh generation changed");
    if (Number.isFinite(earliestExpiry)) this.scheduleExpiry(earliestExpiry);
    return { tools, limits };
  }

  private async discover(limits: BridgeLimits): Promise<CapabilityDescriptor[]> {
    const output: CapabilityDescriptor[] = [];
    const cursors = new Set<string>();
    let cursor: string | undefined;
    do {
      const deadlineAt = Date.now() + this.config.discoveryTimeoutMs;
      const envelope = await this.ctx.museHost.request("discover.request", {
        deadlineAt,
        pageSize: Math.min(this.config.pageSize, limits.maxDiscoverDescriptors),
        families: unique(this.definition.targets.map(target => target.familyId)),
        ...(cursor === undefined ? {} : { cursor }),
        ...(this.config.scopeHint === undefined ? {} : { scopeHint: this.config.scopeHint })
      } as unknown as JsonValue, { signal: this.invocationAbort.signal, deadlineAt });
      const page = unwrap<DiscoverResponseValue>(envelope, "discover.response");
      output.push(...page.descriptors);
      if (output.length > limits.maxDiscoverDescriptors) {
        throw new MusePluginCompatibilityError("DISCOVERY_LIMIT", "Discovery descriptor limit exceeded");
      }
      cursor = page.nextCursor;
      if (cursor !== undefined && cursors.has(cursor)) {
        throw new MusePluginCompatibilityError("DISCOVERY_CURSOR_LOOP", "Discovery cursor repeated");
      }
      if (cursor !== undefined) cursors.add(cursor);
    } while (cursor !== undefined);
    return output;
  }

  private async prepareTarget(
    target: MuseCapabilityTarget,
    descriptors: readonly CapabilityDescriptor[],
    schemas: MuseProviderSchemas
  ): Promise<{ binding: WireBinding; tools: MusePreparedTool[] } | undefined> {
    const candidates = descriptors
      .filter(descriptor => acceptsVersion(descriptor, target))
      .sort((left, right) => {
        const version = right.contractVersion.minor - left.contractVersion.minor;
        if (version !== 0) return version;
        const leftId = String(left.descriptorId);
        const rightId = String(right.descriptorId);
        return leftId < rightId ? -1 : leftId > rightId ? 1 : 0;
      });
    for (const descriptor of candidates) {
      const operations = operationMap(descriptor);
      if (target.requiredOperations.some(id => !operations.has(id))) continue;
      const contracts = new Map<string, {
        contract: MuseProviderOperationContract;
        inputValidator: Awaited<ReturnType<MuseProviderSchemas["materialize"]>>["validator"];
        outputValidator: Awaited<ReturnType<MuseProviderSchemas["materialize"]>>["validator"];
      }>();
      let compatible = true;
      for (const tool of target.tools) {
        assertMuseToolSchemas(tool.parameters, tool.output);
        const operation = operations.get(tool.operationId);
        if (operation === undefined) {
          if (tool.optional === true) continue;
          compatible = false;
          break;
        }
        const [input, output] = await Promise.all([
          schemas.materialize(operation.inputSchema, this.invocationAbort.signal),
          schemas.materialize(operation.outputSchema, this.invocationAbort.signal)
        ]);
        const contract = { descriptor, operation, inputSchema: input.raw, outputSchema: output.raw };
        if (!await tool.adapter.accepts(contract)) {
          compatible = false;
          break;
        }
        contracts.set(tool.operationId, {
          contract,
          inputValidator: input.validator,
          outputValidator: output.validator
        });
      }
      if (!compatible) continue;

      const operationIds = unique([
        ...target.requiredOperations,
        ...(target.optionalOperations ?? []).filter(id => operations.has(id)),
        ...target.tools.filter(tool => operations.has(tool.operationId)).map(tool => tool.operationId)
      ]);
      const deadlineAt = Date.now() + this.config.discoveryTimeoutMs;
      const request: BindRequestPayload = {
        descriptorId: descriptor.descriptorId,
        descriptorRevision: descriptor.revision,
        operationIds,
        deadlineAt,
        ...(this.config.scopeHint === undefined ? {} : { scopeHint: this.config.scopeHint })
      };
      const envelope = await this.ctx.museHost.request(
        "bind.request",
        request as unknown as JsonValue,
        { signal: this.invocationAbort.signal, deadlineAt }
      );
      const binding = unwrap<WireBinding>(envelope, "bind.response");
      const boundIds = new Set(binding.operations.map(operation => operation.operationId));
      if (
        binding.descriptorId !== descriptor.descriptorId
        || binding.descriptorRevision !== descriptor.revision
        || binding.expiresAt <= Date.now()
        || operationIds.some(id => !boundIds.has(id))
        || binding.operations.some(operation => !operationIds.includes(operation.operationId))
      ) {
        throw new MusePluginCompatibilityError("INVALID_BINDING", "Host returned a mismatched or expired binding");
      }
      const prepared = target.tools.flatMap(contribution => {
        const resolved = contracts.get(contribution.operationId);
        return resolved === undefined ? [] : [{ contribution, binding, ...resolved }];
      });
      return { binding, tools: prepared };
    }
    return undefined;
  }

  private publish(prepared: PreparedGeneration): void {
    const registered: (() => void)[] = [];
    try {
      for (const tool of prepared.tools) registered.push(this.ctx.tools.register(this.definitionOf(tool, prepared.limits)));
    } catch (error) {
      for (const dispose of registered.reverse()) dispose();
      throw error;
    }
    this.unregisterTools = registered;
    this.bindingIds = new Set(prepared.tools.map(tool => String(tool.binding.bindingId)));
    this.descriptorIds = new Set(prepared.tools.map(tool => String(tool.binding.descriptorId)));
  }

  private definitionOf(prepared: MusePreparedTool, limits: BridgeLimits): ToolDefinition {
    const { contribution, contract, binding, inputValidator, outputValidator } = prepared;
    const timeoutMs = contribution.timeoutMs ?? this.config.invocationTimeoutMs;
    return {
      name: contribution.name,
      description: contribution.description,
      parameters: contribution.parameters as unknown as Record<string, unknown>,
      output: {
        schema: contribution.output,
        render: (args, value) => content(contribution, jsonArgs(args), value)
      },
      timeoutMs,
      execute: async (args, exec) => {
        const violations = validateJsonSchemaValue(contribution.parameters, args, "");
        if (violations.length > 0) throw new Error(`invalid Tool arguments: ${violations.join("; ")}`);
        return this.track(async () => {
          const signal = AbortSignal.any([exec.signal, this.invocationAbort.signal]);
          const input = await contribution.adapter.toProviderInput(jsonArgs(args), contract);
          const wireInput = inputValidator.validate(input, limits.maxInputBytes, "INPUT_INVALID");
          const deadlineAt = Date.now() + timeoutMs;
          const policy = contribution.policy === undefined
            ? undefined
            : await this.resolvePolicy(contribution, binding, wireInput, deadlineAt, signal, exec);
          const payload: InvokeRequestPayload = {
            traceId: randomUUID() as never,
            bindingId: binding.bindingId,
            operationId: contribution.operationId,
            input: wireInput,
            deadlineAt,
            cancellationId: randomUUID(),
            ...(policy?.grantId === undefined ? {} : { grantId: policy.grantId }),
            ...(contract.operation.idempotency === "required" ? { idempotencyKey: randomUUID() } : {})
          };
          const response = await this.ctx.museHost.invoke(payload, this.sourceOf(exec, signal));
          if (!response.ok) throw new MuseToolInvocationError(response.error, response.receipt);
          const wireOutput = outputValidator.validate(response.value, limits.maxOutputBytes, "OUTPUT_INVALID");
          const modelOutput = await contribution.adapter.fromProviderOutput(wireOutput, response.receipt, contract);
          const outputViolations = validateJsonSchemaValue(contribution.output, modelOutput, "");
          if (outputViolations.length > 0) throw new Error(`invalid Tool result: ${outputViolations.join("; ")}`);
          if (canonicalizeJson(modelOutput).byteLength > limits.maxOutputBytes) {
            throw new Error("Tool result exceeds the negotiated model-output byte limit");
          }
          return modelOutput;
        });
      }
    };
  }

  private async resolvePolicy(
    contribution: MuseToolContribution,
    binding: WireBinding,
    input: JsonValue,
    deadlineAt: number,
    signal: AbortSignal,
    exec: ToolRunContext
  ): Promise<{ readonly grantId?: string }> {
    const policy = contribution.policy;
    if (policy === undefined) return {};
    const approval = this.config.approvalProofRequester
      ?? getMuseApprovalProofRequester(this.ctx);
    if (approval === undefined) throw new Error(`write Tool ${contribution.name} requires a trusted approval proof requester`);
    const result = await resolveMusePolicy(this.ctx.museHost, {
      traceId: randomUUID() as never,
      bindingId: binding.bindingId,
      operationId: contribution.operationId,
      inputDigest: digestInput(input),
      effect: policy.effect,
      deadlineAt
    }, approval, signal, clientCorrelationFromDsh(this.sourceOf(exec, signal)));
    if (result.kind === "allow") return {};
    if (result.kind === "approved") return { grantId: result.grantId };
    if (result.kind === "deny") throw new Error(result.reason ?? `Host policy denied ${contribution.name}`);
    throw new Error(`approval for ${contribution.name} was ${result.outcome}`);
  }

  private sourceOf(exec: ToolRunContext, signal: AbortSignal) {
    return {
      ...(exec.agent === undefined ? {} : { agent: { id: String(exec.agent.id) } }),
      toolCallId: String(exec.callId),
      signal
    };
  }

  private track<T>(operation: () => Promise<T>): Promise<T> {
    const promise = operation();
    this.active.add(promise);
    void promise.finally(() => this.active.delete(promise)).catch(() => undefined);
    return promise;
  }

  private revokeTools(): void {
    if (this.expiryTimer !== undefined) clearTimeout(this.expiryTimer);
    this.expiryTimer = undefined;
    for (const dispose of this.unregisterTools.splice(0).reverse()) dispose();
    this.bindingIds.clear();
    this.descriptorIds.clear();
  }

  private async cancelAndDrain(): Promise<void> {
    this.invocationAbort.abort(new Error("Muse Plugin binding invalidated"));
    await Promise.allSettled([...this.active]);
  }

  private scheduleExpiry(expiresAt: number): void {
    if (this.expiryTimer !== undefined) clearTimeout(this.expiryTimer);
    const delay = Math.max(0, Math.min(expiresAt - Date.now(), 2_147_483_647));
    this.expiryTimer = setTimeout(() => {
      this.epoch += 1;
      this.revokeTools();
      void this.cancelAndDrain();
      void this.enqueueRefresh("BINDING_EXPIRED");
    }, delay);
  }

  private diagnostic(phase: MusePluginDiagnostic["phase"], code: string, detail?: string): void {
    this.config.onDiagnostic?.({
      pluginId: this.definition.pluginId,
      phase,
      code,
      ...(detail === undefined ? {} : { detail }),
      toolNames: this.ctx.tools.schemas().map(tool => tool.name)
    });
  }
}

export interface MuseCordisPlugin {
  readonly name: string;
  readonly inject: readonly ["museHost", "tools"];
  apply(ctx: Context): Promise<void>;
}

export const createMusePlugin = (
  definition: MusePluginDefinition,
  config: MusePluginRuntimeConfig = {}
): MuseCordisPlugin => {
  // This runs before Loader/Cordis receives a Plugin object, so malformed packages cannot own
  // effects or publish a subset of their Tools.
  validateMusePluginDefinition(definition);
  return {
    name: definition.pluginId,
    inject: ["museHost", "tools"],
    async apply(ctx): Promise<void> {
      const runtime = new MusePluginRuntime(ctx, definition, config);
      ctx.effect(() => () => runtime.dispose(), `muse-plugin:${definition.pluginId}`);
      await runtime.start();
    }
  };
};
