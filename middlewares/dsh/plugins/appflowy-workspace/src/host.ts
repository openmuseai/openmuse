import type { JsonValue } from "@muse/host-bridge";
import type { InProcessDomainProvider, InProcessInvokeContext } from "@muse/plugin-kit";
import { canonicalizeJson, digestSchema } from "@muse/host-bridge";
import {
  WORKSPACE_CURRENT_OPERATION,
  WORKSPACE_FAMILY,
  WORKSPACE_TREE_OPERATION,
  currentInputSchema,
  currentOutputSchema,
  invokeCloudWorkspace,
  treeInputSchema,
  treeOutputSchema
} from "./cloud.js";
import { assertWorkspaceScope } from "./scope.js";

const BINDING_ID = "binding.appflowy-workspace";
const PROVIDER_ID = "provider.appflowy-workspace";

const resource = (schema: JsonValue) => ({
  sha256: digestSchema(schema),
  byteLength: canonicalizeJson(schema).byteLength,
  draft: "2020-12" as const,
  inline: schema
});

export const workspaceDescriptor = {
  descriptorId: "appflowy.workspace.cloud",
  revision: "1",
  familyId: WORKSPACE_FAMILY,
  contractVersion: { major: 1, minor: 0 },
  providerInstanceId: PROVIDER_ID,
  operations: [{
    operationId: WORKSPACE_CURRENT_OPERATION,
    effect: "read",
    inputSchema: resource(currentInputSchema),
    outputSchema: resource(currentOutputSchema),
    cancellable: true,
    idempotency: "none"
  }, {
    operationId: WORKSPACE_TREE_OPERATION,
    effect: "read",
    inputSchema: resource(treeInputSchema),
    outputSchema: resource(treeOutputSchema),
    cancellable: true,
    idempotency: "none"
  }],
  events: []
} as const;

const fillWorkspace = (input: JsonValue, ctx: InProcessInvokeContext): JsonValue => {
  const rec = input !== null && typeof input === "object" && !Array.isArray(input)
    ? { ...(input as Record<string, unknown>) }
    : {};
  if ((typeof rec.workspaceId !== "string" || rec.workspaceId.trim().length === 0) && ctx.boundWorkspaceId !== undefined) {
    rec.workspaceId = ctx.boundWorkspaceId;
  }
  return rec as JsonValue;
};

const E2E_WORKSPACE_ID = "00000000-0000-0000-0000-000000000001";

const e2eValues: Record<string, JsonValue> = {
  [WORKSPACE_CURRENT_OPERATION]: {
    workspaceId: E2E_WORKSPACE_ID, title: "E2E", role: "owner"
  },
  [WORKSPACE_TREE_OPERATION]: {
    protocol: "muse.workspace/tree/v1",
    workspaceId: E2E_WORKSPACE_ID,
    rootViewId: E2E_WORKSPACE_ID,
    truncated: false,
    items: [{
      viewId: "00000000-0000-0000-0000-000000000002",
      parentViewId: E2E_WORKSPACE_ID,
      title: "Getting started",
      layout: "document",
      isSpace: false,
      depth: 1
    }]
  }
};

export const e2eWorkspaceProvider: InProcessDomainProvider = {
  descriptor: workspaceDescriptor,
  bindingId: BINDING_ID,
  invoke: ({ operationId }) => {
    const value = e2eValues[operationId];
    if (value === undefined) {
      return Promise.resolve({ ok: false, code: "OPERATION_NOT_FOUND", message: "operation unavailable" });
    }
    return Promise.resolve({ ok: true, value });
  }
};

export const createCloudWorkspaceProvider = (): InProcessDomainProvider => ({
  descriptor: workspaceDescriptor,
  bindingId: BINDING_ID,
  prepareInput: fillWorkspace,
  invoke: async ({ operationId, input, ctx }) => {
    const scoped = assertWorkspaceScope(ctx.boundWorkspaceId, input);
    if (!scoped.ok) {
      return { ok: false, code: scoped.code, message: scoped.message };
    }
    const baseUrl = ctx.cloudBaseUrl;
    if (baseUrl === undefined) {
      return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: workspace adapter unreachable" };
    }
    return invokeCloudWorkspace({
      baseUrl,
      operationId,
      payload: input,
      ...(ctx.accessToken === undefined ? {} : { accessToken: ctx.accessToken }),
      ...(ctx.deviceId === undefined ? {} : { deviceId: ctx.deviceId })
    });
  }
});
