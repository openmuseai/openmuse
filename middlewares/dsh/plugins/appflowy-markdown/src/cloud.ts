import type { JsonValue } from "@muse/host-bridge";
import {
  APPFLOWY_MARKDOWN_APPLY_OPERATION,
  APPFLOWY_MARKDOWN_PROPOSE_OPERATION,
  APPFLOWY_MARKDOWN_READ_OPERATION,
  APPFLOWY_MARKDOWN_STATUS_OPERATION
} from "./index.js";

export const CLOUD_DOCUMENT_QUERY_PATH = "/api/muse/document/query";
export const CLOUD_DOCUMENT_PROPOSE_PATH = "/api/muse/document/propose";
export const CLOUD_DOCUMENT_APPLY_PATH = "/api/muse/document/apply";
export const CLOUD_DOCUMENT_STATUS_PATH = "/api/muse/document/status";

/** ErrorCode::FeatureNotAvailable in AppFlowy-Cloud app-error. */
export const CLOUD_FEATURE_NOT_AVAILABLE = 1067;

export function cloudDocumentBaseUrl(env: NodeJS.ProcessEnv = process.env): string | undefined {
  const raw = env.MUSE_DOCUMENT_CLOUD_URL?.trim();
  return raw ? raw.replace(/\/$/, "") : undefined;
}

export function cloudDocumentApplyEnabled(env: NodeJS.ProcessEnv = process.env): boolean {
  return env.MUSE_DOCUMENT_CLOUD_APPLY_ENABLED === "1";
}

export function pathForDocumentOperation(operationId: string): string | undefined {
  switch (operationId) {
    case APPFLOWY_MARKDOWN_READ_OPERATION:
      return CLOUD_DOCUMENT_QUERY_PATH;
    case APPFLOWY_MARKDOWN_PROPOSE_OPERATION:
      return CLOUD_DOCUMENT_PROPOSE_PATH;
    case APPFLOWY_MARKDOWN_APPLY_OPERATION:
      return CLOUD_DOCUMENT_APPLY_PATH;
    case APPFLOWY_MARKDOWN_STATUS_OPERATION:
      return CLOUD_DOCUMENT_STATUS_PATH;
    default:
      return undefined;
  }
}

export type CloudDocumentInvokeResult =
  | { readonly ok: true; readonly value: JsonValue }
  | { readonly ok: false; readonly code: "UNAVAILABLE" | "OPERATION_NOT_FOUND"; readonly message: string };

function receiptStatus(value: unknown): string | undefined {
  if (!value || typeof value !== "object") return undefined;
  const status = (value as { status?: unknown }).status;
  return typeof status === "string" ? status : undefined;
}

/**
 * Remote DSH must not report a successful apply until collab is wired
 * (`MUSE_DOCUMENT_CLOUD_APPLY_ENABLED=1`).
 */
export function interpretCloudDocumentResponse(
  operationId: string,
  body: unknown,
  applyEnabled = cloudDocumentApplyEnabled()
): CloudDocumentInvokeResult {
  if (!body || typeof body !== "object") {
    return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: invalid document adapter response" };
  }
  const rec = body as { code?: number; message?: string; data?: unknown };
  if (rec.code !== undefined && rec.code !== 0) {
    const message = typeof rec.message === "string" && rec.message.length > 0
      ? rec.message
      : "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED";
    return { ok: false, code: "UNAVAILABLE", message };
  }
  const value = rec.data;
  if (operationId === APPFLOWY_MARKDOWN_APPLY_OPERATION) {
    const status = receiptStatus(value);
    if (status === "applied" && !applyEnabled) {
      return {
        ok: false,
        code: "UNAVAILABLE",
        message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED"
      };
    }
  }
  if (value === undefined) {
    return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" };
  }
  return { ok: true, value: value as JsonValue };
}

export async function invokeCloudDocument(options: {
  baseUrl: string;
  operationId: string;
  payload: JsonValue;
  accessToken?: string;
  deviceId?: string;
  fetchImpl?: typeof fetch;
  applyEnabled?: boolean;
}): Promise<CloudDocumentInvokeResult> {
  const path = pathForDocumentOperation(options.operationId);
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
      if (res.status === 404) {
        return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" };
      }
      return {
        ok: false,
        code: "UNAVAILABLE",
        message: `UNAVAILABLE: invalid document adapter response (HTTP ${String(res.status)})`
      };
    }
    return interpretCloudDocumentResponse(
      options.operationId,
      parsed,
      options.applyEnabled ?? cloudDocumentApplyEnabled()
    );
  } catch {
    return { ok: false, code: "UNAVAILABLE", message: "UNAVAILABLE: document adapter unreachable" };
  }
}
