import type { JsonValue } from "@muse/host-bridge";

export const CLOUD_WORKSPACE_CURRENT_PATH = "/api/muse/workspace/current";
export const CLOUD_WORKSPACE_TREE_PATH = "/api/muse/workspace/tree";
export const WORKSPACE_CURRENT_OPERATION = "workspace.current.query";
export const WORKSPACE_TREE_OPERATION = "workspace.tree.query";
export const WORKSPACE_FAMILY = "muse.workspace";

export function pathForWorkspaceOperation(operationId: string): string | undefined {
  switch (operationId) {
    case WORKSPACE_CURRENT_OPERATION:
      return CLOUD_WORKSPACE_CURRENT_PATH;
    case WORKSPACE_TREE_OPERATION:
      return CLOUD_WORKSPACE_TREE_PATH;
    default:
      return undefined;
  }
}

export type CloudWorkspaceInvokeResult =
  | { readonly ok: true; readonly value: JsonValue }
  | { readonly ok: false; readonly code: "UNAVAILABLE" | "OPERATION_NOT_FOUND" | "SCOPE_MISMATCH"; readonly message: string };

export function interpretCloudWorkspaceResponse(body: unknown): CloudWorkspaceInvokeResult {
  if (!body || typeof body !== "object") {
    return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: invalid workspace adapter response" };
  }
  const rec = body as { code?: number; message?: string; data?: unknown };
  if (rec.code !== undefined && rec.code !== 0) {
    const message = typeof rec.message === "string" && rec.message.length > 0
      ? rec.message
      : "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED";
    if (message.includes("SCOPE_MISMATCH")) {
      return { ok: false, code: "SCOPE_MISMATCH", message };
    }
    if (message === "NOT_FOUND" || message.includes("CLOUD_COLLAB_ADAPTER_NOT_WIRED")) {
      return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" };
    }
    return { ok: false, code: "UNAVAILABLE", message };
  }
  if (rec.data === undefined) {
    return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" };
  }
  return { ok: true, value: rec.data as JsonValue };
}

export const currentInputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  additionalProperties: false,
  properties: {
    workspaceId: { type: "string", minLength: 1, maxLength: 128 }
  }
};

export const currentOutputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  additionalProperties: false,
  required: ["workspaceId", "title"],
  properties: {
    workspaceId: { type: "string" },
    title: { type: "string" },
    role: { type: ["string", "null"] }
  }
};

export const treeInputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  additionalProperties: false,
  properties: {
    workspaceId: { type: "string", minLength: 1, maxLength: 128 },
    parentViewId: { type: "string", minLength: 1, maxLength: 128 },
    cursor: { type: "string", minLength: 1, maxLength: 128 },
    limit: { type: "integer", minimum: 1, maximum: 64 },
    depth: { type: "integer", minimum: 1, maximum: 4 }
  }
};

export const treeOutputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  additionalProperties: false,
  required: ["protocol", "workspaceId", "rootViewId", "truncated", "items"],
  properties: {
    protocol: { type: "string", const: "muse.workspace/tree/v1" },
    workspaceId: { type: "string" },
    rootViewId: { type: "string" },
    truncated: { type: "boolean" },
    nextCursor: { type: "string" },
    items: {
      type: "array",
      maxItems: 64,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["viewId", "title", "layout", "isSpace", "depth"],
        properties: {
          viewId: { type: "string" },
          parentViewId: { type: ["string", "null"] },
          title: { type: "string", maxLength: 256 },
          layout: { type: "string", enum: ["document", "grid", "board", "calendar", "chat"] },
          isSpace: { type: "boolean" },
          depth: { type: "integer", minimum: 0, maximum: 4 }
        }
      }
    }
  }
};

export async function invokeCloudWorkspace(options: {
  baseUrl: string;
  operationId: string;
  payload: JsonValue;
  accessToken?: string;
  deviceId?: string;
  fetchImpl?: typeof fetch;
}): Promise<CloudWorkspaceInvokeResult> {
  const path = pathForWorkspaceOperation(options.operationId);
  if (path === undefined) {
    return { ok: false, code: "OPERATION_NOT_FOUND", message: "operation unavailable" };
  }
  const fetchImpl = options.fetchImpl ?? fetch;
  try {
    const res = await fetchImpl(`${options.baseUrl}${path}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        ...(options.accessToken ? { Authorization: `Bearer ${options.accessToken}` } : {}),
        ...(options.deviceId ? { "X-Muse-Device-Id": options.deviceId } : {})
      },
      body: JSON.stringify({ operation: options.operationId, ...(options.payload as object) })
    });
    const raw = await res.text();
    let parsed: unknown = undefined;
    try {
      parsed = raw.length === 0 ? {} : JSON.parse(raw);
    } catch {
      if (res.status === 401 || res.status === 403) {
        return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: document adapter unauthorized" };
      }
      return {
        ok: false,
        code: "UNAVAILABLE",
        message: `UNAVAILABLE: invalid workspace adapter response (HTTP ${String(res.status)})`
      };
    }
    return interpretCloudWorkspaceResponse(parsed);
  } catch {
    return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: workspace adapter unreachable" };
  }
}
