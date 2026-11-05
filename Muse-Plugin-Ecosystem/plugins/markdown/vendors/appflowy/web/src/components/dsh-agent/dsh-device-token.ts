import { getTokenParsed } from '@/application/session/token';

const DEVICE_ID_KEY = 'appflowy.dshAgent.deviceId';
export const DSH_DEVICE_TOKEN_PATH = '/api/muse/dsh/device-token';

export type DshDeviceToken = {
  token: string;
  expiresAt: number;
  deviceId: string;
  kid: string;
};

function newWebDeviceId(): string {
  const uuid =
    typeof globalThis.crypto !== 'undefined' && typeof globalThis.crypto.randomUUID === 'function'
      ? globalThis.crypto.randomUUID()
      : `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 12)}`;
  return `web.${uuid}`;
}

export function getOrCreateDshDeviceId(storage: Storage = localStorage): string {
  try {
    const existing = storage.getItem(DEVICE_ID_KEY);
    if (existing && existing.trim()) return existing.trim();
    const created = newWebDeviceId();
    storage.setItem(DEVICE_ID_KEY, created);
    return created;
  } catch {
    return newWebDeviceId();
  }
}

export function pickDshDeviceTokenPayload(data: unknown): DshDeviceToken | null {
  if (!data || typeof data !== 'object') return null;
  const rec = data as Record<string, unknown>;
  if (typeof rec.token !== 'string' || rec.token.length === 0) return null;
  if (typeof rec.expiresAt !== 'number') return null;
  if (rec.token.startsWith('sk-')) return null;
  return {
    token: rec.token,
    expiresAt: rec.expiresAt,
    deviceId: typeof rec.deviceId === 'string' ? rec.deviceId : '',
    kid: typeof rec.kid === 'string' ? rec.kid : 'v1',
  };
}

/** Same-origin `/dsh/` nginx auth_request reads this cookie. Not HttpOnly — same XSS surface as localStorage. */
export function syncDshIngressCookie(
  accessToken: string | null | undefined,
  expiresAt?: number,
  cookieTarget: { cookie: string } | undefined = typeof document === 'undefined' ? undefined : document
): void {
  if (!cookieTarget) return;
  const secure = typeof location === 'undefined' || location.protocol === 'https:' ? '; Secure' : '';
  if (!accessToken) {
    cookieTarget.cookie = `access_token=; Path=/; Max-Age=0; SameSite=Lax${secure}`;
    return;
  }
  const nowSec = Date.now() / 1000;
  const maxAge =
    typeof expiresAt === 'number' && expiresAt > nowSec ? Math.max(60, Math.floor(expiresAt - nowSec)) : 900;
  cookieTarget.cookie = `access_token=${encodeURIComponent(accessToken)}; Path=/; Max-Age=${maxAge}; SameSite=Lax${secure}`;
}

export async function fetchDshDeviceToken(options?: {
  sessionRef?: string;
  accessToken?: string | null;
  fetchImpl?: typeof fetch;
}): Promise<DshDeviceToken | null> {
  const accessToken = options?.accessToken ?? getTokenParsed()?.access_token ?? null;
  if (!accessToken) return null;

  const fetchImpl = options?.fetchImpl ?? fetch;
  const deviceId = getOrCreateDshDeviceId();
  try {
    const res = await fetchImpl(DSH_DEVICE_TOKEN_PATH, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${accessToken}`,
      },
      body: JSON.stringify({
        deviceId,
        sessionRef: options?.sessionRef ?? '',
      }),
    });
    if (res.status === 401 || res.status === 404) return null;
    if (!res.ok) return null;
    const raw = await res.text();
    const body = JSON.parse(raw) as { data?: unknown; code?: number };
    if (body.code !== undefined && body.code !== 0) return null;
    return pickDshDeviceTokenPayload(body.data);
  } catch {
    return null;
  }
}
