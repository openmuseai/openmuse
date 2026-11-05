import { WorkspaceUiFacet, type ApplyIntentContext } from './dsh-workspace-facet';
import {
  MAX_DSH_POSTMESSAGE_BYTES,
  containsForbiddenDshFields,
  contextContributeMessage,
  intentReceiptMessage,
  isOversizedDshMessage,
  jsonByteLength,
  type MusePresentationIntent,
  type MusePresentationIntentResult,
} from '@/components/dsh-agent/dsh-origin';

export const MARKDOWN_SURFACE_DIGEST =
  'sha256:780a1eed2737ec2f9e7dd1fe56cd74a3a7803aea0fc23e37837fd33d458c7b92';
export const MARKDOWN_SELECTION_DIGEST =
  'sha256:39caf711d2a5cc6a452fb9e4b5aa2f0bf2f5c695a1a751c0d06de2e7255946c0';
export const MARKDOWN_VIEWPORT_DIGEST =
  'sha256:b7d780858ade44ad51997b60293220630b8a15ae411bc38bb665420c8e82c4a8';
export const MARKDOWN_SNAPSHOT_DIGEST =
  'sha256:e4ea242dbdbc52565fb474cd042af4da590ae88a9d4715e9694ce031eaee03b3';
export const WORKSPACE_FOCUS_DIGEST =
  'sha256:4c3a6bf1cd8249cc96f6aac63ad78b9be04172634f0313ce3ba52163f66043b7';
export const WORKSPACE_TREE_UI_DIGEST =
  'sha256:c0381f40eaaa753e7bb28e79e74c8e44573864aa89928a3ba246a524389edbda';
export const WORKSPACE_CATALOG_DIGEST =
  'sha256:2538c0adb882241b625b48c0013e4a0dd248d8b391cac22ebdc14e154c50548e';

const SELECTION_TEXT_MAX = 2048;
const EXPANDED_IDS_MAX = 64;
const CATALOG_MAX_ITEMS = 64;
const CATALOG_MAX_DEPTH = 4;
const SNAPSHOT_MAX_BYTES = 32 * 1024;

export const workspaceSurfaceRef = (workspaceId: string): string =>
  `surface.appflowy.workspace.${workspaceId}`;

export const documentSurfaceRef = (viewId: string): string => `surface.appflowy.doc.${viewId}`;

const sanitizeText = (value: string, max: number): string =>
  Array.from(value.replace(/[<>\u0000-\u001f]/gu, ' ')).slice(0, max).join('');

export function expandedViewIdsFromStorage(raw: unknown): string[] {
  if (!raw || typeof raw !== 'object' || Array.isArray(raw)) return [];
  return Object.entries(raw as Record<string, unknown>)
    .filter(([, expanded]) => expanded === true)
    .map(([id]) => id)
    .filter((id) => id.length > 0 && id.length <= 128)
    .slice(0, EXPANDED_IDS_MAX);
}

export function buildContextEnvelope(input: {
  pluginId: string;
  pluginVersion: string;
  contextType: string;
  contextSchemaDigest: string;
  surfaceInstanceRef: string;
  surfaceKind: string;
  scopeRef: string;
  epochRef: string;
  revision: number;
  lane: 'control' | 'state';
  ttlMs: number;
  payload: Record<string, unknown>;
  now?: number;
}): Record<string, unknown> {
  const now = input.now ?? Date.now();
  return {
    protocol: 'muse.context-contribution/v1',
    pluginId: input.pluginId,
    pluginVersion: input.pluginVersion,
    facetInstanceRef: `facet.${input.pluginId}`,
    surfaceInstanceRef: input.surfaceInstanceRef,
    surfaceKind: input.surfaceKind,
    scopeRef: input.scopeRef,
    contextType: input.contextType,
    contextSchemaDigest: input.contextSchemaDigest,
    contextRevision: String(Math.max(0, Math.floor(input.revision))),
    epochRef: input.epochRef,
    lane: input.lane,
    capturedAt: now,
    expiresAt: now + input.ttlMs,
    payload: input.payload,
  };
}

export function buildWorkspaceFocusEnvelope(input: {
  workspaceId: string;
  title?: string;
  viewId?: string;
  revision: number;
  now?: number;
}): Record<string, unknown> {
  return buildContextEnvelope({
    pluginId: 'muse.appflowy.workspace',
    pluginVersion: '1.0.0',
    contextType: 'workspace.focus',
    contextSchemaDigest: WORKSPACE_FOCUS_DIGEST,
    surfaceInstanceRef: workspaceSurfaceRef(input.workspaceId),
    surfaceKind: 'appflowy.workspace',
    scopeRef: `workspace.${input.workspaceId}`,
    epochRef: `epoch.${input.workspaceId}`,
    revision: input.revision,
    lane: 'control',
    ttlMs: 120_000,
    payload: {
      workspaceId: input.workspaceId,
      ...(input.title ? { title: sanitizeText(input.title, 256) } : {}),
      ...(input.viewId ? { viewId: input.viewId } : {}),
    },
    now: input.now,
  });
}

export function buildTreeUiEnvelope(input: {
  workspaceId: string;
  expandedViewIds: readonly string[];
  revision: number;
  now?: number;
}): Record<string, unknown> {
  return buildContextEnvelope({
    pluginId: 'muse.appflowy.workspace',
    pluginVersion: '1.0.0',
    contextType: 'workspace.tree.ui',
    contextSchemaDigest: WORKSPACE_TREE_UI_DIGEST,
    surfaceInstanceRef: workspaceSurfaceRef(input.workspaceId),
    surfaceKind: 'appflowy.workspace',
    scopeRef: `workspace.${input.workspaceId}`,
    epochRef: `epoch.${input.workspaceId}`,
    revision: input.revision,
    lane: 'control',
    ttlMs: 120_000,
    payload: {
      workspaceId: input.workspaceId,
      expandedViewIds: input.expandedViewIds.slice(0, EXPANDED_IDS_MAX),
    },
    now: input.now,
  });
}

export type WorkspaceCatalogLayout = 'document' | 'grid' | 'board' | 'calendar' | 'chat';

export interface WorkspaceCatalogItem {
  readonly viewId: string;
  readonly title: string;
  readonly layout: WorkspaceCatalogLayout;
  readonly isSpace: boolean;
  readonly depth: number;
  readonly parentViewId?: string | null;
}

/** Loose Host outline node (AppFlowy View or a test fixture). */
export interface WorkspaceOutlineNode {
  readonly view_id?: unknown;
  readonly viewId?: unknown;
  readonly name?: unknown;
  readonly title?: unknown;
  readonly layout?: unknown;
  readonly extra?: { readonly is_space?: unknown } | null;
  readonly parent_view_id?: unknown;
  readonly parentViewId?: unknown;
  readonly children?: unknown;
}

export function catalogLayoutOf(layout: unknown): WorkspaceCatalogLayout {
  if (layout === 1 || layout === 'grid') return 'grid';
  if (layout === 2 || layout === 'board') return 'board';
  if (layout === 3 || layout === 'calendar') return 'calendar';
  if (layout === 4 || layout === 'chat' || layout === 'AIChat') return 'chat';
  return 'document';
}

export function flattenOutlineToCatalog(
  outline: readonly WorkspaceOutlineNode[] | undefined,
  parentViewId: string | null = null,
  depth = 0
): { items: WorkspaceCatalogItem[]; truncated: boolean } {
  const items: WorkspaceCatalogItem[] = [];
  let truncated = false;
  if (outline === undefined || depth > CATALOG_MAX_DEPTH) {
    return { items, truncated: depth > CATALOG_MAX_DEPTH && (outline?.length ?? 0) > 0 };
  }
  for (const node of outline) {
    if (items.length >= CATALOG_MAX_ITEMS) {
      truncated = true;
      break;
    }
    const viewIdRaw = typeof node.view_id === 'string' ? node.view_id : typeof node.viewId === 'string' ? node.viewId : '';
    const viewId = viewIdRaw.trim().slice(0, 128);
    if (viewId.length === 0) continue;
    const titleRaw = typeof node.name === 'string' ? node.name : typeof node.title === 'string' ? node.title : 'Untitled';
    const parentRaw = typeof node.parent_view_id === 'string'
      ? node.parent_view_id
      : typeof node.parentViewId === 'string'
        ? node.parentViewId
        : parentViewId;
    items.push({
      viewId,
      title: Array.from(titleRaw.trim() || 'Untitled').slice(0, 256).join(''),
      layout: catalogLayoutOf(node.layout),
      isSpace: node.extra?.is_space === true,
      depth,
      parentViewId: parentRaw === null || parentRaw === undefined ? null : String(parentRaw).slice(0, 128),
    });
    const children = Array.isArray(node.children) ? (node.children as WorkspaceOutlineNode[]) : [];
    if (children.length > 0 && depth < CATALOG_MAX_DEPTH) {
      const nested = flattenOutlineToCatalog(children, viewId, depth + 1);
      const room = CATALOG_MAX_ITEMS - items.length;
      items.push(...nested.items.slice(0, room));
      if (nested.truncated || nested.items.length > room) truncated = true;
    } else if (children.length > 0) {
      truncated = true;
    }
  }
  return { items, truncated };
}

export function buildWorkspaceCatalogEnvelope(input: {
  workspaceId: string;
  outline?: readonly WorkspaceOutlineNode[];
  revision: number;
  now?: number;
}): Record<string, unknown> {
  const { items, truncated } = flattenOutlineToCatalog(input.outline);
  return buildContextEnvelope({
    pluginId: 'muse.appflowy.workspace',
    pluginVersion: '1.0.0',
    contextType: 'workspace.catalog',
    contextSchemaDigest: WORKSPACE_CATALOG_DIGEST,
    surfaceInstanceRef: workspaceSurfaceRef(input.workspaceId),
    surfaceKind: 'appflowy.workspace',
    scopeRef: `workspace.${input.workspaceId}`,
    epochRef: `epoch.${input.workspaceId}`,
    revision: input.revision,
    lane: 'control',
    ttlMs: 120_000,
    payload: {
      workspaceId: input.workspaceId,
      truncated,
      items,
    },
    now: input.now,
  });
}

export function buildMarkdownSurfaceEnvelope(input: {
  workspaceId: string;
  viewId: string;
  title?: string;
  readOnly?: boolean;
  revision: number;
  now?: number;
}): Record<string, unknown> {
  return buildContextEnvelope({
    pluginId: 'muse.appflowy.markdown',
    pluginVersion: '2.0.0',
    contextType: 'markdown.surface',
    contextSchemaDigest: MARKDOWN_SURFACE_DIGEST,
    surfaceInstanceRef: documentSurfaceRef(input.viewId),
    surfaceKind: 'appflowy.markdown',
    scopeRef: input.viewId,
    epochRef: `epoch.${input.workspaceId}.${input.viewId}`,
    revision: input.revision,
    lane: 'control',
    ttlMs: 90_000,
    payload: {
      resourceRef: input.viewId,
      viewRef: input.viewId,
      title: sanitizeText(input.title ?? 'Untitled', 256),
      mode: 'edit',
      readOnly: input.readOnly === true,
    },
    now: input.now,
  });
}

export function buildMarkdownSelectionEnvelope(input: {
  workspaceId: string;
  viewId: string;
  selectedText: string;
  collapsed: boolean;
  revision: number;
  now?: number;
}): Record<string, unknown> {
  const selected = sanitizeText(input.selectedText, SELECTION_TEXT_MAX);
  return buildContextEnvelope({
    pluginId: 'muse.appflowy.markdown',
    pluginVersion: '2.0.0',
    contextType: 'markdown.selection',
    contextSchemaDigest: MARKDOWN_SELECTION_DIGEST,
    surfaceInstanceRef: documentSurfaceRef(input.viewId),
    surfaceKind: 'appflowy.markdown',
    scopeRef: input.viewId,
    epochRef: `epoch.${input.workspaceId}.${input.viewId}`,
    revision: input.revision,
    lane: 'control',
    ttlMs: 30_000,
    payload: {
      collapsed: input.collapsed || selected.length === 0,
      selectedText: selected,
      selectedBlockRefs: [],
    },
    now: input.now,
  });
}

export function boundUtf8Text(text: string, maxBytes: number): { text: string; truncated: boolean; byteLength: number } {
  let byteLength = 0;
  let end = 0;
  while (end < text.length) {
    const code = text.charCodeAt(end);
    const size = code <= 0x7f ? 1 : code <= 0x7ff ? 2 : code >= 0xd800 && code <= 0xdbff ? 4 : 3;
    if (byteLength + size > maxBytes) break;
    byteLength += size;
    end += size === 4 ? 2 : 1;
  }
  const sliced = text.slice(0, end);
  return { text: sliced, truncated: end < text.length, byteLength };
}

export function readVisibleEditorText(maxBytes = SNAPSHOT_MAX_BYTES): { text: string; truncated: boolean; byteLength: number } {
  if (typeof document === 'undefined') return { text: '', truncated: false, byteLength: 0 };
  const root =
    document.querySelector('.appflowy-editor') ??
    document.querySelector('[data-testid="document-editor"]');
  const raw = (root?.textContent ?? '').replace(/[\u0000-\u0008\u000b\u000c\u000e-\u001f]/g, ' ').trim();
  if (raw.length === 0) return { text: '', truncated: false, byteLength: 0 };
  return boundUtf8Text(raw, maxBytes);
}

export function buildMarkdownSnapshotEnvelope(input: {
  workspaceId: string;
  viewId: string;
  text: string;
  revision: number;
  now?: number;
}): Record<string, unknown> {
  const bounded = boundUtf8Text(input.text, SNAPSHOT_MAX_BYTES);
  return buildContextEnvelope({
    pluginId: 'muse.appflowy.markdown',
    pluginVersion: '2.0.0',
    contextType: 'markdown.snapshot',
    contextSchemaDigest: MARKDOWN_SNAPSHOT_DIGEST,
    surfaceInstanceRef: documentSurfaceRef(input.viewId),
    surfaceKind: 'appflowy.markdown',
    scopeRef: input.viewId,
    epochRef: `epoch.${input.workspaceId}.${input.viewId}`,
    revision: input.revision,
    lane: 'state',
    ttlMs: 90_000,
    payload: {
      workspaceId: input.workspaceId,
      viewId: input.viewId,
      text: bounded.text,
      truncated: bounded.truncated,
      byteLength: bounded.byteLength,
    },
    now: input.now,
  });
}

export function contributionMessageOrNull(envelope: Record<string, unknown>) {
  const message = contextContributeMessage(envelope);
  if (isOversizedDshMessage(message) || containsForbiddenDshFields(message)) return null;
  return message;
}

export type { ApplyIntentContext } from './dsh-workspace-facet';

export function applyPresentationIntent(intent: MusePresentationIntent, ctx: ApplyIntentContext): Promise<MusePresentationIntentResult> {
  return new WorkspaceUiFacet(ctx).dispatch(intent);
}

export function receiptOrNull(result: MusePresentationIntentResult) {
  const message = intentReceiptMessage(result);
  if (isOversizedDshMessage(message) || containsForbiddenDshFields(message)) return null;
  return message;
}

export { jsonByteLength, MAX_DSH_POSTMESSAGE_BYTES };
