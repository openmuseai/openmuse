import { getConfigValue } from '@/utils/runtime-config';

export const DEFAULT_DSH_ORIGIN = 'http://127.0.0.1:3080';

export const MUSE_DSH_PARENT_SOURCE = 'muse.appflowy-web';
export const MUSE_DSH_FRAME_SOURCE = 'muse.dsh-web';

export function normalizeDshOrigin(raw: string | undefined | null): string {
  const trimmed = (raw ?? '').trim().replace(/\/$/, '');
  if (!trimmed) return DEFAULT_DSH_ORIGIN;
  try {
    const url = new URL(trimmed);
    if (url.protocol !== 'http:' && url.protocol !== 'https:') {
      return DEFAULT_DSH_ORIGIN;
    }
    return `${url.protocol}//${url.host}`;
  } catch {
    return DEFAULT_DSH_ORIGIN;
  }
}

/** iframe src. Keeps a path so same-origin `/dsh/` is not collapsed to the marketing site. */
export function normalizeDshFrameSrc(raw: string | undefined | null): string {
  const trimmed = (raw ?? '').trim();
  if (!trimmed) return DEFAULT_DSH_ORIGIN;
  try {
    const url = new URL(trimmed);
    if (url.protocol !== 'http:' && url.protocol !== 'https:') {
      return DEFAULT_DSH_ORIGIN;
    }
    const path = url.pathname.replace(/\/+$/, '');
    if (!path || path === '/') {
      return `${url.protocol}//${url.host}`;
    }
    return `${url.protocol}//${url.host}${path}/`;
  } catch {
    return DEFAULT_DSH_ORIGIN;
  }
}

export function dshMessageOrigin(frameSrc: string): string {
  return normalizeDshOrigin(frameSrc);
}

export function getDshAgentUrl(): string {
  return normalizeDshFrameSrc(getConfigValue('APPFLOWY_DSH_AGENT_URL', DEFAULT_DSH_ORIGIN));
}

/** P0 shared-instance path. After the systemd pool cutover this URL 502s; do not iframe it. */
export function isSharedDshFrameSrc(raw: string | undefined | null): boolean {
  const trimmed = (raw ?? '').trim();
  if (!trimmed) return false;
  try {
    return new URL(trimmed).pathname.replace(/\/+$/, '') === '/dsh';
  } catch {
    return false;
  }
}

export function canMountDshIframe(src: string | undefined | null): boolean {
  const trimmed = (src ?? '').trim();
  if (!trimmed) return false;
  try {
    const parsed = new URL(trimmed);
    if (parsed.protocol !== 'http:' && parsed.protocol !== 'https:') return false;
    return !isSharedDshFrameSrc(trimmed);
  } catch {
    return false;
  }
}

/** Sidecar / custom host may use the configured URL. Same-origin `/dsh/` must wait for session/open. */
export function shouldFallbackToConfiguredDsh(configuredUrl: string): boolean {
  return isLoopbackDshOrigin(configuredUrl) || !isSharedDshFrameSrc(configuredUrl);
}

export function isLoopbackDshOrigin(origin: string): boolean {
  try {
    const host = new URL(normalizeDshOrigin(origin)).hostname;
    return host === '127.0.0.1' || host === 'localhost' || host === '[::1]' || host === '::1';
  } catch {
    return false;
  }
}

export function isAllowedDshMessageOrigin(eventOrigin: string, configuredOrigin: string): boolean {
  return normalizeDshOrigin(eventOrigin) === normalizeDshOrigin(configuredOrigin);
}

export function frameSrcCspValue(dshOrigin: string): string {
  return `frame-src 'self' ${normalizeDshOrigin(dshOrigin)}`;
}

export type DshProbeResult = 'ok' | 'down';

export async function probeDshOrigin(
  origin: string,
  fetchImpl: typeof fetch = fetch,
  timeoutMs = 4000
): Promise<DshProbeResult> {
  const target = `${normalizeDshOrigin(origin)}/`;
  const ctrl = new AbortController();
  const timer = setTimeout(() => ctrl.abort(), timeoutMs);
  try {
    await fetchImpl(target, { method: 'GET', mode: 'no-cors', cache: 'no-store', signal: ctrl.signal });
    return 'ok';
  } catch {
    return 'down';
  } finally {
    clearTimeout(timer);
  }
}

export type MuseDshFrameReady = {
  source: typeof MUSE_DSH_FRAME_SOURCE;
  type: 'frame-ready';
};

export type MuseDshParentHello = {
  source: typeof MUSE_DSH_PARENT_SOURCE;
  type: 'parent-hello';
  requestId?: string;
  theme?: string;
  locale?: string;
  deviceToken?: string;
  deviceId?: string;
  kid?: string;
  expiresAt?: number;
  workspaceRef?: string;
  workspaceTitle?: string;
};

export function parseDshFrameReady(data: unknown): MuseDshFrameReady | null {
  if (!data || typeof data !== 'object') return null;
  const rec = data as Record<string, unknown>;
  if (rec.source !== MUSE_DSH_FRAME_SOURCE || rec.type !== 'frame-ready') return null;
  return { source: MUSE_DSH_FRAME_SOURCE, type: 'frame-ready' };
}

export function parentHelloMessage(
  theme?: string,
  locale?: string,
  device?: { token: string; kid?: string; expiresAt?: number; deviceId?: string },
  workspace?: { workspaceRef: string; workspaceTitle?: string },
  requestId?: string
): MuseDshParentHello {
  return {
    source: MUSE_DSH_PARENT_SOURCE,
    type: 'parent-hello',
    ...(requestId ? { requestId } : {}),
    theme,
    locale,
    ...(device
      ? {
          deviceToken: device.token,
          kid: device.kid,
          expiresAt: device.expiresAt,
          ...(device.deviceId ? { deviceId: device.deviceId } : {}),
        }
      : {}),
    ...(workspace
      ? {
          workspaceRef: workspace.workspaceRef,
          ...(workspace.workspaceTitle ? { workspaceTitle: workspace.workspaceTitle } : {}),
        }
      : {}),
  };
}

export const MAX_DSH_POSTMESSAGE_BYTES = 32 * 1024;

export type MuseDshWorkspaceBind = {
  source: typeof MUSE_DSH_PARENT_SOURCE;
  type: 'workspace.bind';
  requestId?: string;
  workspaceRef: string;
  workspaceTitle?: string;
  deviceToken?: string;
  deviceId?: string;
};

export type MuseDshBridgeReply = {
  source: typeof MUSE_DSH_FRAME_SOURCE;
  type: 'bridge.reply';
  requestId?: string;
  status: number;
  body: { ok?: boolean; bound?: string; error?: string } | null;
};

export type MuseDshContextContribute = {
  source: typeof MUSE_DSH_PARENT_SOURCE;
  type: 'context.contribute';
  envelope: Record<string, unknown>;
};

export type MuseDshSurfaceClosed = {
  source: typeof MUSE_DSH_PARENT_SOURCE;
  type: 'surface.closed';
  surfaceInstanceRef: string;
};

export type MuseDshIntentDispatch = {
  source: typeof MUSE_DSH_FRAME_SOURCE;
  type: 'intent.dispatch';
  intent: MusePresentationIntent;
};

export type MusePresentationIntent = {
  protocol: 'muse.presentation-intent/v1';
  pluginId: string;
  scopeRef: string;
  intentType: string;
  intentSchemaDigest: string;
  intentRef: string;
  requestedAt: number;
  expiresAt: number;
  payload: Record<string, unknown>;
  targetSurfaceInstanceRef?: string;
};

export type MusePresentationIntentResult = {
  protocol: 'muse.presentation-intent-result/v1';
  intentRef: string;
  status: 'applied' | 'rejected' | 'stale' | 'not-found' | 'not-supported' | 'surface-closed' | 'timed-out';
  completedAt: number;
  reasonCode?: string;
  appliedSurfaceInstanceRef?: string;
};

export type MuseDshIntentReceipt = {
  source: typeof MUSE_DSH_PARENT_SOURCE;
  type: 'intent.receipt';
  result: MusePresentationIntentResult;
};

export function jsonByteLength(value: unknown): number {
  try {
    const json = JSON.stringify(value);
    if (typeof TextEncoder === 'function') {
      return new TextEncoder().encode(json).length;
    }
    return json.length;
  } catch {
    return Number.POSITIVE_INFINITY;
  }
}

export function isOversizedDshMessage(value: unknown): boolean {
  return jsonByteLength(value) > MAX_DSH_POSTMESSAGE_BYTES;
}

const FORBIDDEN_POSTMESSAGE = /access_token|refresh_token|api[_-]?key/i;

export function containsForbiddenDshFields(value: unknown): boolean {
  try {
    return FORBIDDEN_POSTMESSAGE.test(JSON.stringify(value));
  } catch {
    return true;
  }
}

function asRecord(data: unknown): Record<string, unknown> | null {
  if (!data || typeof data !== 'object') return null;
  return data as Record<string, unknown>;
}

export function workspaceBindMessage(
  workspaceRef: string,
  workspaceTitle?: string,
  device?: { token: string; deviceId?: string },
  requestId?: string
): MuseDshWorkspaceBind {
  return {
    source: MUSE_DSH_PARENT_SOURCE,
    type: 'workspace.bind',
    workspaceRef,
    ...(workspaceTitle ? { workspaceTitle } : {}),
    ...(requestId ? { requestId } : {}),
    ...(device ? { deviceToken: device.token, ...(device.deviceId ? { deviceId: device.deviceId } : {}) } : {}),
  };
}

export function parseWorkspaceBind(data: unknown): MuseDshWorkspaceBind | null {
  const rec = asRecord(data);
  if (!rec || rec.source !== MUSE_DSH_PARENT_SOURCE || rec.type !== 'workspace.bind') return null;
  if (typeof rec.workspaceRef !== 'string' || rec.workspaceRef.trim().length === 0) return null;
  if (rec.workspaceRef.trim().length > 128) return null;
  if (isOversizedDshMessage(data) || containsForbiddenDshFields(data)) return null;
  const title = typeof rec.workspaceTitle === 'string' ? rec.workspaceTitle : undefined;
  const deviceToken = typeof rec.deviceToken === 'string' ? rec.deviceToken : undefined;
  const deviceId = typeof rec.deviceId === 'string' ? rec.deviceId : undefined;
  const requestId = typeof rec.requestId === 'string' ? rec.requestId : undefined;
  return {
    source: MUSE_DSH_PARENT_SOURCE,
    type: 'workspace.bind',
    workspaceRef: rec.workspaceRef.trim(),
    ...(title ? { workspaceTitle: title.slice(0, 256) } : {}),
    ...(requestId ? { requestId } : {}),
    ...(deviceToken ? { deviceToken } : {}),
    ...(deviceId ? { deviceId } : {}),
  };
}

export function parseBridgeReply(data: unknown): MuseDshBridgeReply | null {
  const rec = asRecord(data);
  if (!rec || rec.source !== MUSE_DSH_FRAME_SOURCE || rec.type !== 'bridge.reply') return null;
  const status = typeof rec.status === 'number' ? rec.status : 0;
  const body = rec.body && typeof rec.body === 'object' ? (rec.body as MuseDshBridgeReply['body']) : null;
  return {
    source: MUSE_DSH_FRAME_SOURCE,
    type: 'bridge.reply',
    status,
    body,
    ...(typeof rec.requestId === 'string' ? { requestId: rec.requestId } : {}),
  };
}

export function contextContributeMessage(envelope: Record<string, unknown>): MuseDshContextContribute {
  return {
    source: MUSE_DSH_PARENT_SOURCE,
    type: 'context.contribute',
    envelope,
  };
}

export function surfaceClosedMessage(surfaceInstanceRef: string): MuseDshSurfaceClosed {
  return {
    source: MUSE_DSH_PARENT_SOURCE,
    type: 'surface.closed',
    surfaceInstanceRef,
  };
}

export function parseIntentDispatch(data: unknown): MuseDshIntentDispatch | null {
  const rec = asRecord(data);
  if (!rec || rec.source !== MUSE_DSH_FRAME_SOURCE || rec.type !== 'intent.dispatch') return null;
  if (isOversizedDshMessage(data) || containsForbiddenDshFields(data)) return null;
  const intent = asRecord(rec.intent);
  if (!intent) return null;
  if (intent.protocol !== 'muse.presentation-intent/v1') return null;
  if (typeof intent.intentType !== 'string' || typeof intent.intentRef !== 'string') return null;
  if (typeof intent.expiresAt !== 'number' || typeof intent.requestedAt !== 'number') return null;
  const payload = asRecord(intent.payload) ?? {};
  return {
    source: MUSE_DSH_FRAME_SOURCE,
    type: 'intent.dispatch',
    intent: {
      protocol: 'muse.presentation-intent/v1',
      pluginId: typeof intent.pluginId === 'string' ? intent.pluginId : 'muse.appflowy.workspace',
      scopeRef: typeof intent.scopeRef === 'string' ? intent.scopeRef : 'workspace.unknown',
      intentType: intent.intentType,
      intentSchemaDigest: typeof intent.intentSchemaDigest === 'string' ? intent.intentSchemaDigest : '',
      intentRef: intent.intentRef,
      requestedAt: intent.requestedAt,
      expiresAt: intent.expiresAt,
      payload,
      ...(typeof intent.targetSurfaceInstanceRef === 'string'
        ? { targetSurfaceInstanceRef: intent.targetSurfaceInstanceRef }
        : {}),
    },
  };
}

export function intentReceiptMessage(result: MusePresentationIntentResult): MuseDshIntentReceipt {
  return {
    source: MUSE_DSH_PARENT_SOURCE,
    type: 'intent.receipt',
    result,
  };
}

export function webHybridFlagOn(
  key: 'MUSE_WEB_WORKSPACE_BIND' | 'MUSE_WEB_CONTEXT_UPLINK' | 'MUSE_WEB_INTENT_DOWNLINK',
  defaultOn = true
): boolean {
  const raw = getConfigValue(key, defaultOn ? '1' : '0').trim().toLowerCase();
  return raw !== '0' && raw !== 'false' && raw !== 'off';
}
