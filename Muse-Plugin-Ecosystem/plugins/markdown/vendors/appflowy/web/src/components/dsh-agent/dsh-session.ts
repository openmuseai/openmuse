import { getTokenParsed } from '@/application/session/token';
import { type AttachmentErrorCode } from '@/components/dsh-agent/dsh-attachment';
import { getOrCreateDshDeviceId } from '@/components/dsh-agent/dsh-device-token';

export const DSH_SESSION_OPEN_PATH = '/api/muse/dsh/session/open';
export const DSH_SESSION_CLOSE_PATH = '/api/muse/dsh/session/close';
export const DSH_SESSION_HEARTBEAT_PATH = '/api/muse/dsh/session/heartbeat';

export type DshSessionOpen = {
  sessionRef: string;
  instanceRef?: string;
  webUrl?: string;
  expiresAt?: number;
  queuePosition?: number;
  retryAfterMs?: number;
  nodeId?: string;
};

export type SessionOpenResult =
  | { kind: 'ready'; session: DshSessionOpen }
  | { kind: 'queued'; session: DshSessionOpen }
  | { kind: 'denied'; status: number; code: AttachmentErrorCode };

export function resolveDshAccessToken(parsed: unknown): string | null {
  if (!parsed || typeof parsed !== 'object') return null;
  const rec = parsed as Record<string, unknown>;
  for (const key of ['access_token', 'accessToken'] as const) {
    const value = rec[key];
    if (typeof value === 'string' && value.trim()) return value.trim();
  }
  return null;
}

function asRecord(data: unknown): Record<string, unknown> | null {
  if (!data || typeof data !== 'object') return null;
  return data as Record<string, unknown>;
}

export function pickDshSessionPayload(data: unknown): DshSessionOpen | null {
  const rec = asRecord(data);
  if (!rec) return null;
  const sessionRef = typeof rec.sessionRef === 'string' ? rec.sessionRef.trim() : '';
  if (!sessionRef) return null;
  const webUrl = typeof rec.webUrl === 'string' && rec.webUrl.startsWith('http') ? rec.webUrl : undefined;
  return {
    sessionRef,
    ...(typeof rec.instanceRef === 'string' ? { instanceRef: rec.instanceRef } : {}),
    ...(webUrl ? { webUrl } : {}),
    ...(typeof rec.expiresAt === 'number' ? { expiresAt: rec.expiresAt } : {}),
    ...(typeof rec.queuePosition === 'number' ? { queuePosition: rec.queuePosition } : {}),
    ...(typeof rec.retryAfterMs === 'number' ? { retryAfterMs: rec.retryAfterMs } : {}),
    ...(typeof rec.nodeId === 'string' ? { nodeId: rec.nodeId } : {}),
  };
}

export function isQueuedSession(status: number, session: DshSessionOpen | null): boolean {
  if (session && session.queuePosition != null && !session.webUrl) return true;
  if (status === 503 && session && (session.retryAfterMs != null || session.queuePosition != null)) return true;
  return false;
}

export function denyCodeFromHttp(status: number, data: unknown): AttachmentErrorCode {
  const rec = asRecord(data);
  const message = typeof rec?.message === 'string' ? rec.message : '';
  if (status === 401) return 'SESSION_DENIED';
  if (status === 403 || message.includes('SCOPE_MISMATCH')) return 'SCOPE_MISMATCH';
  if (status === 0 || status === 503 || status === 502) return 'POOL_UNAVAILABLE';
  return 'SESSION_DENIED';
}

async function postSession(
  path: string,
  body: Record<string, unknown>,
  options?: { accessToken?: string | null; fetchImpl?: typeof fetch; attachmentId?: string }
): Promise<{ status: number; data: unknown } | null> {
  const accessToken = options?.accessToken ?? resolveDshAccessToken(getTokenParsed());
  if (!accessToken) return null;
  const fetchImpl = options?.fetchImpl ?? fetch;
  try {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${accessToken}`,
    };
    const attachmentId = options?.attachmentId?.trim();
    if (attachmentId) headers['X-Muse-Attachment-Id'] = attachmentId;
    const res = await fetchImpl(path, {
      method: 'POST',
      headers,
      credentials: 'same-origin',
      body: JSON.stringify(body),
    });
    const raw = await res.text();
    let parsed: unknown = null;
    try {
      parsed = raw ? JSON.parse(raw) : null;
    } catch {
      parsed = null;
    }
    const rec = asRecord(parsed);
    return { status: res.status, data: rec?.data ?? parsed };
  } catch {
    return { status: 0, data: null };
  }
}

export async function openDshSession(options: {
  workspaceId: string;
  deviceId?: string;
  accessToken?: string | null;
  fetchImpl?: typeof fetch;
  attachmentId?: string;
}): Promise<SessionOpenResult> {
  const workspaceId = options.workspaceId.trim();
  if (!workspaceId) return { kind: 'denied', status: 400, code: 'NEED_WORKSPACE' };
  const accessToken = options.accessToken ?? resolveDshAccessToken(getTokenParsed());
  if (!accessToken) return { kind: 'denied', status: 401, code: 'NEED_AUTH' };
  const deviceId = options.deviceId?.trim() || getOrCreateDshDeviceId();
  const result = await postSession(
    DSH_SESSION_OPEN_PATH,
    { workspaceId, deviceId },
    { ...options, accessToken }
  );
  if (!result) return { kind: 'denied', status: 401, code: 'NEED_AUTH' };
  const session = pickDshSessionPayload(result.data);
  if (isQueuedSession(result.status, session)) {
    return {
      kind: 'queued',
      session: session ?? { sessionRef: 'queued', queuePosition: 1, retryAfterMs: 10_000 },
    };
  }
  if (result.status === 200 && session) {
    return { kind: 'ready', session };
  }
  return { kind: 'denied', status: result.status, code: denyCodeFromHttp(result.status, result.data) };
}

export async function closeDshSession(options: {
  sessionRef: string;
  deviceId?: string;
  accessToken?: string | null;
  fetchImpl?: typeof fetch;
}): Promise<void> {
  const sessionRef = options.sessionRef.trim();
  if (!sessionRef) return;
  await postSession(
    DSH_SESSION_CLOSE_PATH,
    { sessionRef, deviceId: options.deviceId?.trim() || getOrCreateDshDeviceId() },
    options
  );
}

export async function heartbeatDshSession(options: {
  sessionRef: string;
  accessToken?: string | null;
  fetchImpl?: typeof fetch;
}): Promise<void> {
  const sessionRef = options.sessionRef.trim();
  if (!sessionRef) return;
  await postSession(DSH_SESSION_HEARTBEAT_PATH, { sessionRef }, options);
}
