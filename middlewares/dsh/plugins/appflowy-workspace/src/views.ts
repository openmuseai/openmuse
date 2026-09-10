import type { JsonValue } from "@muse/host-bridge";
import { getLastWorkspaceHint } from "./identity.js";
import { resolveWorkspaceTree } from "./host.js";

export const WORKSPACE_VIEWS_PATH = "/muse/v1/workspace/views";

export interface WorkspaceViewsQuery {
  readonly workspaceId?: string;
  readonly parentViewId?: string;
  readonly cursor?: string;
  readonly limit?: number;
  readonly depth?: number;
}

export type ListBoundWorkspaceViewsResult =
  | { readonly ok: true; readonly status: 200; readonly body: JsonValue }
  | { readonly ok: false; readonly status: number; readonly error: string };

const clampInt = (raw: string | null, min: number, max: number): number | undefined => {
  if (raw === null || raw.length === 0) return undefined;
  const parsed = Number.parseInt(raw, 10);
  if (!Number.isFinite(parsed)) return undefined;
  return Math.min(max, Math.max(min, parsed));
};

export const parseWorkspaceViewsQuery = (url: string | undefined): WorkspaceViewsQuery => {
  let parsed: URL;
  try {
    parsed = new URL(url ?? "", "http://127.0.0.1");
  } catch {
    return {};
  }
  const workspaceId = parsed.searchParams.get("workspaceId")?.trim() ?? "";
  const parentViewId = parsed.searchParams.get("parentViewId")?.trim() ?? "";
  const cursor = parsed.searchParams.get("cursor")?.trim() ?? "";
  const limit = clampInt(parsed.searchParams.get("limit"), 1, 64);
  const depth = clampInt(parsed.searchParams.get("depth"), 1, 4);
  return {
    ...(workspaceId.length > 0 ? { workspaceId } : {}),
    ...(parentViewId.length > 0 ? { parentViewId } : {}),
    ...(cursor.length > 0 ? { cursor } : {}),
    ...(limit === undefined ? {} : { limit }),
    ...(depth === undefined ? {} : { depth })
  };
};

const FORBIDDEN = /access_token|refresh_token|api[_-]?key/i;

export const listBoundWorkspaceViews = async (
  query: WorkspaceViewsQuery = {},
  deps: {
    readonly auth?: { readonly token: string; readonly deviceId: string };
    readonly cloudBaseUrl?: string;
    readonly fetchImpl?: typeof fetch;
  } = {}
): Promise<ListBoundWorkspaceViewsResult> => {
  const hint = getLastWorkspaceHint();
  if (hint === undefined) {
    return { ok: false, status: 409, error: "NO_WORKSPACE" };
  }
  if (query.workspaceId !== undefined && query.workspaceId !== hint.appflowyWorkspaceId) {
    return { ok: false, status: 403, error: "SCOPE_MISMATCH" };
  }
  const auth = deps.auth;
  if (auth === undefined) {
    return { ok: false, status: 401, error: "NO_DEVICE_TOKEN" };
  }
  const baseUrl = deps.cloudBaseUrl ?? process.env.MUSE_DOCUMENT_CLOUD_URL?.trim()?.replace(/\/$/, "");
  const invoked = await resolveWorkspaceTree({
    boundWorkspaceId: hint.appflowyWorkspaceId,
    input: {
      workspaceId: hint.appflowyWorkspaceId,
      ...(query.parentViewId === undefined ? {} : { parentViewId: query.parentViewId }),
      ...(query.cursor === undefined ? {} : { cursor: query.cursor }),
      ...(query.limit === undefined ? {} : { limit: query.limit }),
      ...(query.depth === undefined ? {} : { depth: query.depth })
    },
    ...(baseUrl === undefined || baseUrl.length === 0 ? {} : { cloudBaseUrl: baseUrl }),
    accessToken: auth.token,
    deviceId: auth.deviceId,
    ...(deps.fetchImpl === undefined ? {} : { fetchImpl: deps.fetchImpl })
  });
  if (!invoked.ok) {
    const unauthorized = invoked.message.includes("unauthorized") || invoked.message.includes("TOKEN_REVOKED");
    const status = invoked.code === "SCOPE_MISMATCH" ? 403 : unauthorized ? 401 : 502;
    return { ok: false, status, error: invoked.code };
  }
  const serialized = JSON.stringify(invoked.value);
  if (FORBIDDEN.test(serialized) || serialized.includes(auth.token)) {
    return { ok: false, status: 502, error: "UNAVAILABLE" };
  }
  return {
    ok: true,
    status: 200,
    body: {
      ok: true,
      source: invoked.source,
      cwdNote: "DSH cwd is README-only; this list is AppFlowy folder views",
      tree: invoked.value
    }
  };
};
