import {
  HARD_LIMITS,
  canonicalizeJson,
  digestSchema,
  type JsonValue,
  type TransportHandler,
  type WireEnvelope
} from "@muse/host-bridge";
import type { InProcessDomainProvider, InProcessInvokeContext } from "@muse/plugin-kit";

export const COMPOSITION_HOST_GENERATION = "appflowy.e2e.1";
export const COMPOSITION_HOST_SESSION_ID = "host-session.appflowy-e2e";

export const schemaResource = (schema: JsonValue) => ({
  sha256: digestSchema(schema),
  byteLength: canonicalizeJson(schema).byteLength,
  draft: "2020-12" as const,
  inline: schema
});

export interface CompositionInvokeSlots {
  boundWorkspaceId?: () => string | undefined;
  documentFocus?: () => { workspaceId: string; viewId: string } | undefined;
  deviceAuth?: () => { token: string; deviceId: string } | undefined;
}

const boundOperations = (provider: InProcessDomainProvider) =>
  provider.descriptor.operations.map(operation => ({
    operationId: operation.operationId,
    effect: operation.effect,
    inputSchemaDigest: operation.inputSchema.sha256,
    outputSchemaDigest: operation.outputSchema.sha256,
    cancellable: operation.cancellable,
    idempotency: operation.idempotency
  }));

const response = (request: WireEnvelope, kind: WireEnvelope["kind"], payload: JsonValue): WireEnvelope => ({
  protocol: "muse-bridge", major: 1, minor: 0, kind,
  ...(request.requestId === undefined ? {} : { requestId: request.requestId }),
  ...(kind === "hello.response" ? {} : { hostSessionId: COMPOSITION_HOST_SESSION_ID as never }),
  sentAt: Date.now(), payload
});

export class InProcessCompositionHandler implements TransportHandler {
  constructor(
    private readonly providers: readonly InProcessDomainProvider[],
    private readonly options: {
      readonly cloudBaseUrl?: string;
      readonly accessToken?: string;
      readonly slots?: CompositionInvokeSlots;
    } = {}
  ) {}

  async unary(
    message: WireEnvelope,
    _signal?: AbortSignal,
    _context?: import("@muse/host-bridge").TransportRequestContext
  ): Promise<WireEnvelope> {
    if (message.kind === "hello.request") {
      return response(message, "hello.response", {
        ok: true,
        value: {
          selectedVersion: { major: 1, minor: 0 }, features: [], limits: HARD_LIMITS,
          hostSessionId: COMPOSITION_HOST_SESSION_ID, hostGeneration: COMPOSITION_HOST_GENERATION,
          serverTime: Date.now(), clockSkewToleranceMs: 1000, authorityRevision: "authority.e2e.1"
        }
      } as unknown as JsonValue);
    }
    if (message.kind === "discover.request") {
      return response(message, "discover.response", {
        ok: true,
        value: {
          registryRevision: "registry.e2e.1",
          descriptors: this.providers.map(provider => provider.descriptor)
        }
      } as unknown as JsonValue);
    }
    if (message.kind === "bind.request") {
      const requested = message.payload as { descriptorId?: string; operationIds?: string[] } | undefined;
      const selected = this.providers.find(provider => provider.descriptor.descriptorId === requested?.descriptorId)
        ?? this.providers[0];
      if (selected === undefined) {
        return response(message, "bind.response", {
          ok: false,
          error: { kind: "bridge", code: "OPERATION_NOT_FOUND", message: "no in-process providers", retryable: false }
        } as unknown as JsonValue);
      }
      const available = boundOperations(selected);
      const wanted = requested?.operationIds;
      const operations = Array.isArray(wanted) && wanted.length > 0
        ? available.filter(operation => wanted.includes(operation.operationId))
        : available;
      return response(message, "bind.response", {
        ok: true,
        value: {
          bindingId: selected.bindingId,
          descriptorId: selected.descriptor.descriptorId,
          descriptorRevision: selected.descriptor.revision,
          providerInstanceId: selected.descriptor.providerInstanceId,
          hostGeneration: COMPOSITION_HOST_GENERATION,
          scopeRef: `scope.${selected.descriptor.familyId}`,
          expiresAt: Date.now() + 300_000,
          operations
        }
      } as unknown as JsonValue);
    }
    if (message.kind === "policy.evaluate.request") {
      return response(message, "policy.evaluate.response", {
        ok: true,
        value: {
          policyDecisionId: "policy.e2e", decision: "approval_required",
          approvalId: "approval.e2e", expiresAt: Date.now() + 60_000
        }
      } as unknown as JsonValue);
    }
    if (message.kind === "policy.finalize.request") {
      return response(message, "policy.finalize.response", {
        ok: true,
        value: {
          policyDecisionId: "policy.e2e", outcome: "approved", grantId: "grant.e2e",
          expiresAt: Date.now() + 60_000
        }
      } as unknown as JsonValue);
    }
    if (message.kind === "invoke.request") {
      const payload = message.payload as unknown as {
        traceId: string; bindingId: string; operationId: string; input?: JsonValue;
      };
      const provider = this.providers.find(item => item.bindingId === payload.bindingId);
      if (provider === undefined) {
        return response(message, "invoke.response", {
          ok: false,
          error: { kind: "bridge", code: "OPERATION_NOT_FOUND", message: "operation unavailable", retryable: false }
        } as unknown as JsonValue);
      }
      const auth = this.options.slots?.deviceAuth?.();
      const boundWorkspaceId = this.options.slots?.boundWorkspaceId?.();
      const documentFocus = this.options.slots?.documentFocus?.();
      const accessToken = this.options.accessToken ?? auth?.token;
      const deviceId = auth?.deviceId;
      const ctx: InProcessInvokeContext = {
        ...(boundWorkspaceId === undefined ? {} : { boundWorkspaceId }),
        ...(documentFocus === undefined ? {} : { documentFocus }),
        ...(accessToken === undefined ? {} : { accessToken }),
        ...(deviceId === undefined ? {} : { deviceId }),
        ...(this.options.cloudBaseUrl === undefined ? {} : { cloudBaseUrl: this.options.cloudBaseUrl })
      };
      const prepared = provider.prepareInput?.(payload.input ?? {}, ctx) ?? (payload.input ?? {});
      const invoked = await provider.invoke({
        operationId: payload.operationId,
        input: prepared,
        ctx
      });
      if (!invoked.ok) {
        return response(message, "invoke.response", {
          ok: false,
          error: { kind: "bridge", code: invoked.code, message: invoked.message, retryable: false }
        } as unknown as JsonValue);
      }
      return response(message, "invoke.response", {
        ok: true,
        value: invoked.value,
        receipt: {
          receiptId: `receipt.${provider.descriptor.familyId}`,
          requestId: message.requestId,
          traceId: payload.traceId,
          hostSessionId: COMPOSITION_HOST_SESSION_ID,
          hostGeneration: COMPOSITION_HOST_GENERATION,
          bindingId: payload.bindingId,
          operationId: payload.operationId,
          policyDecision: "allow",
          status: "applied_local",
          issuedAt: Date.now()
        }
      } as unknown as JsonValue);
    }
    throw new Error(`unsupported Muse request: ${message.kind}`);
  }
}
