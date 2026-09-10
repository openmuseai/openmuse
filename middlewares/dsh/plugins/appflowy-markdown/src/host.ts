import type { JsonValue } from "@muse/host-bridge";
import type { InProcessDomainProvider, InProcessInvokeContext } from "@muse/plugin-kit";
import {
  APPFLOWY_MARKDOWN_APPLY_OPERATION,
  APPFLOWY_MARKDOWN_FAMILY,
  APPFLOWY_MARKDOWN_PROPOSE_OPERATION,
  APPFLOWY_MARKDOWN_READ_OPERATION,
  APPFLOWY_MARKDOWN_STATUS_OPERATION,
  applyProviderInputSchema,
  applyProviderOutputSchema,
  proposeProviderInputSchema,
  proposeProviderOutputSchema,
  providerInputSchema,
  providerOutputSchema,
  statusProviderInputSchema,
  statusProviderOutputSchema
} from "./index.js";
import { invokeCloudDocument } from "./cloud.js";
import { cloudDocumentUnwired, projectSnapshotDocument } from "./snapshot.js";
import { canonicalizeJson, digestSchema } from "@muse/host-bridge";

const BINDING_ID = "binding.appflowy-markdown";
const PROVIDER_ID = "provider.appflowy-markdown";

const resource = (schema: JsonValue) => ({
  sha256: digestSchema(schema),
  byteLength: canonicalizeJson(schema).byteLength,
  draft: "2020-12" as const,
  inline: schema
});

export const markdownDescriptor = {
  descriptorId: "appflowy.document.local",
  revision: "3",
  familyId: APPFLOWY_MARKDOWN_FAMILY,
  contractVersion: { major: 2, minor: 0 },
  providerInstanceId: PROVIDER_ID,
  operations: [{
    operationId: APPFLOWY_MARKDOWN_READ_OPERATION,
    effect: "read",
    inputSchema: resource(providerInputSchema),
    outputSchema: resource(providerOutputSchema),
    cancellable: true,
    idempotency: "none"
  }, {
    operationId: APPFLOWY_MARKDOWN_PROPOSE_OPERATION, effect: "read",
    inputSchema: resource(proposeProviderInputSchema), outputSchema: resource(proposeProviderOutputSchema),
    cancellable: true, idempotency: "none"
  }, {
    operationId: APPFLOWY_MARKDOWN_APPLY_OPERATION, effect: "local_write",
    inputSchema: resource(applyProviderInputSchema), outputSchema: resource(applyProviderOutputSchema),
    cancellable: true, idempotency: "required"
  }, {
    operationId: APPFLOWY_MARKDOWN_STATUS_OPERATION, effect: "read",
    inputSchema: resource(statusProviderInputSchema), outputSchema: resource(statusProviderOutputSchema),
    cancellable: true, idempotency: "none"
  }],
  events: []
} as const;

const fillSelection = (input: JsonValue, ctx: InProcessInvokeContext): JsonValue => {
  const rec = input !== null && typeof input === "object" && !Array.isArray(input)
    ? { ...(input as Record<string, unknown>) }
    : {};
  if (typeof rec.workspaceId !== "string" || rec.workspaceId.trim().length === 0) {
    const workspaceId = ctx.documentFocus?.workspaceId ?? ctx.boundWorkspaceId;
    if (workspaceId !== undefined) rec.workspaceId = workspaceId;
  }
  if ((typeof rec.viewId !== "string" || rec.viewId.trim().length === 0) && ctx.documentFocus?.viewId !== undefined) {
    rec.viewId = ctx.documentFocus.viewId;
  }
  return rec as JsonValue;
};

const e2eValues = (): Record<string, JsonValue> => {
  const markdown = process.env.MUSE_APPFLOWY_E2E_MARKDOWN
    ?? "# Muse AppFlowy\n\nDSH successfully called the AppFlowy Markdown plugin.";
  const revision = `sha256:${"1".repeat(64)}`;
  return {
    [APPFLOWY_MARKDOWN_READ_OPERATION]: {
      protocol: "muse.document/snapshot/v2", resourceRef: "document.e2e", revision,
      content: { mediaType: "text/markdown", text: markdown, truncated: false, byteLength: Buffer.byteLength(markdown) }
    },
    [APPFLOWY_MARKDOWN_PROPOSE_OPERATION]: {
      protocol: "muse.document/proposal/v2", proposalRef: "proposal.e2e", resourceRef: "document.e2e", expectedRevision: revision,
      preview: { before: markdown, after: `${markdown}\nMuse`, changed: true }, approvalRequired: true, expiresAt: Date.now() + 300_000
    },
    [APPFLOWY_MARKDOWN_APPLY_OPERATION]: {
      protocol: "muse.document/receipt/v2", commandRef: "command.e2e", status: "applied", resourceRef: "document.e2e",
      previousRevision: revision, revision: `sha256:${"2".repeat(64)}`, eventCursor: "1", eventPublicationStatus: "published",
      idempotencyKey: "e2e"
    },
    [APPFLOWY_MARKDOWN_STATUS_OPERATION]: { status: "pending", expiresAt: Date.now() + 300_000 }
  };
};

export const e2eMarkdownProvider: InProcessDomainProvider = {
  descriptor: markdownDescriptor,
  bindingId: BINDING_ID,
  invoke: ({ operationId }) => {
    const value = e2eValues()[operationId];
    if (value === undefined) {
      return Promise.resolve({ ok: false, code: "OPERATION_NOT_FOUND", message: "operation unavailable" });
    }
    return Promise.resolve({ ok: true, value });
  }
};

export const createCloudMarkdownProvider = (): InProcessDomainProvider => ({
  descriptor: markdownDescriptor,
  bindingId: BINDING_ID,
  prepareInput: fillSelection,
  invoke: async ({ operationId, input, ctx }) => {
    if (operationId === APPFLOWY_MARKDOWN_READ_OPERATION) {
      const rec = input as { viewId?: unknown };
      const viewId = typeof rec.viewId === "string" ? rec.viewId.trim() : "";
      if (viewId.length === 0) {
        return { ok: false, code: "NO_CURRENT_SELECTION", message: "NO_CURRENT_SELECTION: no focused AppFlowy document" };
      }
      const baseUrl = ctx.cloudBaseUrl;
      if (baseUrl !== undefined) {
        const cloud = await invokeCloudDocument({
          baseUrl,
          operationId,
          payload: input,
          ...(ctx.accessToken === undefined ? {} : { accessToken: ctx.accessToken }),
          ...(ctx.deviceId === undefined ? {} : { deviceId: ctx.deviceId })
        });
        if (cloud.ok) return cloud;
        if (!cloudDocumentUnwired(cloud)) return cloud;
      }
      const snapshot = projectSnapshotDocument(viewId);
      if (snapshot !== undefined) return { ok: true, value: snapshot };
      return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" };
    }
    const baseUrl = ctx.cloudBaseUrl;
    if (baseUrl === undefined) {
        return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: document adapter unreachable" };
    }
    return invokeCloudDocument({
      baseUrl,
      operationId,
      payload: input,
      ...(ctx.accessToken === undefined ? {} : { accessToken: ctx.accessToken }),
      ...(ctx.deviceId === undefined ? {} : { deviceId: ctx.deviceId })
    });
  }
});
