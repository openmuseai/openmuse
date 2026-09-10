import type { JsonValue } from "@muse/host-bridge";

/** Host → DSH Facet: bounded AppFlowy folder projection (ids/titles only). */
export const WORKSPACE_CATALOG_CONTEXT_TYPE = "workspace.catalog";
export const WORKSPACE_CATALOG_DIGEST =
  "sha256:2538c0adb882241b625b48c0013e4a0dd248d8b391cac22ebdc14e154c50548e";

export const WORKSPACE_TREE_PROTOCOL = "muse.workspace/tree/v1";
export const WORKSPACE_CATALOG_MAX_ITEMS = 64;
export const WORKSPACE_CATALOG_MAX_DEPTH = 4;

const LAYOUTS = ["document", "grid", "board", "calendar", "chat"] as const;
export type WorkspaceCatalogLayout = (typeof LAYOUTS)[number];

export interface WorkspaceCatalogItem {
  readonly viewId: string;
  readonly title: string;
  readonly layout: WorkspaceCatalogLayout;
  readonly isSpace: boolean;
  readonly depth: number;
  readonly parentViewId?: string | null;
}

export interface WorkspaceCatalog {
  readonly workspaceId: string;
  readonly items: readonly WorkspaceCatalogItem[];
  readonly truncated: boolean;
}

const FORBIDDEN = /access_token|refresh_token|api[_-]?key/i;

let lastCatalog: WorkspaceCatalog | undefined;

export const getLastWorkspaceCatalog = (): WorkspaceCatalog | undefined => lastCatalog;

export const resetLastWorkspaceCatalog = (): void => {
  lastCatalog = undefined;
};

const asLayout = (value: unknown): WorkspaceCatalogLayout | undefined => {
  if (typeof value === "string" && (LAYOUTS as readonly string[]).includes(value)) {
    return value as WorkspaceCatalogLayout;
  }
  return undefined;
};

const asItem = (value: unknown): WorkspaceCatalogItem | undefined => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return undefined;
  const rec = value as Record<string, unknown>;
  const viewId = typeof rec.viewId === "string" ? rec.viewId.trim() : "";
  if (viewId.length === 0 || viewId.length > 128) return undefined;
  const title = typeof rec.title === "string" && rec.title.trim().length > 0
    ? rec.title.trim().slice(0, 256)
    : "Untitled";
  const layout = asLayout(rec.layout) ?? "document";
  const depth = typeof rec.depth === "number" && Number.isFinite(rec.depth)
    ? Math.min(WORKSPACE_CATALOG_MAX_DEPTH, Math.max(0, Math.floor(rec.depth)))
    : 0;
  const parentViewId = typeof rec.parentViewId === "string" && rec.parentViewId.trim().length > 0
    ? rec.parentViewId.trim().slice(0, 128)
    : rec.parentViewId === null
      ? null
      : undefined;
  return {
    viewId,
    title,
    layout,
    isSpace: rec.isSpace === true,
    depth,
    ...(parentViewId === undefined ? {} : { parentViewId })
  };
};

export const parseWorkspaceCatalogPayload = (payload: unknown): WorkspaceCatalog | undefined => {
  if (payload === null || typeof payload !== "object" || Array.isArray(payload)) return undefined;
  const rec = payload as Record<string, unknown>;
  const workspaceId = typeof rec.workspaceId === "string" ? rec.workspaceId.trim() : "";
  if (workspaceId.length === 0 || workspaceId.length > 128) return undefined;
  if (!Array.isArray(rec.items)) return undefined;
  const items: WorkspaceCatalogItem[] = [];
  for (const raw of rec.items) {
    const item = asItem(raw);
    if (item === undefined) continue;
    items.push(item);
    if (items.length >= WORKSPACE_CATALOG_MAX_ITEMS) break;
  }
  const truncated = rec.truncated === true || rec.items.length > items.length;
  const catalog: WorkspaceCatalog = { workspaceId, items, truncated };
  const serialized = JSON.stringify(catalog);
  if (FORBIDDEN.test(serialized)) return undefined;
  return catalog;
};

export const rememberWorkspaceCatalog = (catalog: WorkspaceCatalog): void => {
  lastCatalog = catalog;
};

export const rememberWorkspaceCatalogFromEnvelope = (envelope: unknown): boolean => {
  if (envelope === null || typeof envelope !== "object" || Array.isArray(envelope)) return false;
  const rec = envelope as Record<string, unknown>;
  if (rec.contextType !== WORKSPACE_CATALOG_CONTEXT_TYPE) return false;
  const parsed = parseWorkspaceCatalogPayload(rec.payload);
  if (parsed === undefined) return false;
  lastCatalog = parsed;
  return true;
};

export const projectCatalogTree = (
  catalog: WorkspaceCatalog,
  query: {
    readonly parentViewId?: string;
    readonly cursor?: string;
    readonly limit?: number;
    readonly depth?: number;
  } = {}
): JsonValue => {
  const depthCap = query.depth === undefined
    ? WORKSPACE_CATALOG_MAX_DEPTH
    : Math.min(WORKSPACE_CATALOG_MAX_DEPTH, Math.max(1, Math.floor(query.depth)));
  const parent = query.parentViewId?.trim() ?? "";
  let items = catalog.items.filter(item => item.depth <= depthCap);
  if (parent.length > 0) {
    items = items.filter(item => item.parentViewId === parent || item.viewId === parent);
  }
  const offset = query.cursor !== undefined && query.cursor.length > 0
    ? Number.parseInt(query.cursor, 10)
    : 0;
  const start = Number.isFinite(offset) && offset > 0 ? offset : 0;
  const limit = query.limit === undefined
    ? WORKSPACE_CATALOG_MAX_ITEMS
    : Math.min(WORKSPACE_CATALOG_MAX_ITEMS, Math.max(1, Math.floor(query.limit)));
  const page = items.slice(start, start + limit);
  const truncated = catalog.truncated || start + page.length < items.length;
  const nextCursor = truncated ? String(start + page.length) : undefined;
  return {
    protocol: WORKSPACE_TREE_PROTOCOL,
    workspaceId: catalog.workspaceId,
    rootViewId: parent.length > 0 ? parent : catalog.workspaceId,
    truncated,
    ...(nextCursor === undefined ? {} : { nextCursor }),
    items: page.map(item => ({
      viewId: item.viewId,
      title: item.title,
      layout: item.layout,
      isSpace: item.isSpace,
      depth: item.depth,
      parentViewId: item.parentViewId === undefined ? null : item.parentViewId
    }))
  };
};

export const projectBoundCatalogTree = (
  boundWorkspaceId: string | undefined,
  query: JsonValue
): JsonValue | undefined => {
  const catalog = lastCatalog;
  if (catalog === undefined) return undefined;
  if (boundWorkspaceId !== undefined && catalog.workspaceId !== boundWorkspaceId) return undefined;
  const rec = query !== null && typeof query === "object" && !Array.isArray(query)
    ? query as Record<string, unknown>
    : {};
  const requested = typeof rec.workspaceId === "string" ? rec.workspaceId.trim() : "";
  if (requested.length > 0 && requested !== catalog.workspaceId) return undefined;
  return projectCatalogTree(catalog, {
    ...(typeof rec.parentViewId === "string" ? { parentViewId: rec.parentViewId } : {}),
    ...(typeof rec.cursor === "string" ? { cursor: rec.cursor } : {}),
    ...(typeof rec.limit === "number" ? { limit: rec.limit } : {}),
    ...(typeof rec.depth === "number" ? { depth: rec.depth } : {})
  });
};

export const cloudTreeUnwired = (result: { readonly code: string; readonly message: string }): boolean =>
  result.code === "UNAVAILABLE"
  && (result.message.includes("CLOUD_COLLAB_ADAPTER_NOT_WIRED")
    || result.message.includes("NOT_FOUND")
    || result.message.includes("unreachable")
    || result.message.includes("invalid workspace adapter"));
