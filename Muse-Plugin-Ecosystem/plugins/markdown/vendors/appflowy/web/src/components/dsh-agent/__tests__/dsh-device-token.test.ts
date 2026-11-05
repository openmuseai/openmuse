import { afterEach, describe, expect, it, jest } from '@jest/globals';

import {
  DSH_DEVICE_TOKEN_PATH,
  fetchDshDeviceToken,
  getOrCreateDshDeviceId,
  pickDshDeviceTokenPayload,
  syncDshIngressCookie,
} from '@/components/dsh-agent/dsh-device-token';

jest.mock('@/application/session/token', () => ({
  getTokenParsed: jest.fn(() => null),
}));

import { getTokenParsed } from '@/application/session/token';

const mockGetToken = getTokenParsed as jest.MockedFunction<typeof getTokenParsed>;

function jsonResponse(status: number, body: unknown): Response {
  return {
    status,
    ok: status >= 200 && status < 300,
    text: async () => (typeof body === 'string' ? body : JSON.stringify(body)),
  } as unknown as Response;
}

describe('fetchDshDeviceToken', () => {
  afterEach(() => {
    jest.clearAllMocks();
    window.localStorage.clear();
  });

  it('returns null without an access token and does not fetch', async () => {
    mockGetToken.mockReturnValue(null);
    const fetchImpl = jest.fn();
    await expect(fetchDshDeviceToken({ fetchImpl: fetchImpl as unknown as typeof fetch })).resolves.toBeNull();
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it('returns null on 401', async () => {
    const fetchImpl = jest.fn(async () => jsonResponse(401, {})) as unknown as typeof fetch;
    await expect(
      fetchDshDeviceToken({ accessToken: 'gotrue-jwt', fetchImpl })
    ).resolves.toBeNull();
  });

  it('strips secret-like fields from a Cloud payload', () => {
    const picked = pickDshDeviceTokenPayload({
      token: 'aaa.bbb',
      expiresAt: 9_999,
      deviceId: 'web.1',
      kid: 'v1',
      api_key: 'sk-should-not-leak',
    });
    expect(picked).toEqual({
      token: 'aaa.bbb',
      expiresAt: 9_999,
      deviceId: 'web.1',
      kid: 'v1',
    });
    expect(JSON.stringify(picked)).not.toContain('sk-');
    expect(JSON.stringify(picked)).not.toContain('api_key');
  });

  it('returns the token payload without copying secret-like fields', async () => {
    const payload = {
      code: 0,
      data: {
        token: 'aaa.bbb',
        expiresAt: 9_999,
        deviceId: 'web.1',
        kid: 'v1',
        api_key: 'sk-should-not-leak',
      },
    };
    const fetchImpl = jest.fn(async () => jsonResponse(200, payload)) as unknown as typeof fetch;
    const issued = await fetchDshDeviceToken({ accessToken: 'gotrue-jwt', fetchImpl });
    expect(issued).toEqual({
      token: 'aaa.bbb',
      expiresAt: 9_999,
      deviceId: 'web.1',
      kid: 'v1',
    });
    expect(JSON.stringify(issued)).not.toContain('sk-');
    expect(JSON.stringify(issued)).not.toContain('api_key');
    expect(fetchImpl).toHaveBeenCalledWith(
      DSH_DEVICE_TOKEN_PATH,
      expect.objectContaining({ method: 'POST' })
    );
  });

  it('rejects a token that looks like a model key', async () => {
    const payload = { code: 0, data: { token: 'sk-live-123', expiresAt: 1, deviceId: 'x', kid: 'v1' } };
    const fetchImpl = jest.fn(async () => jsonResponse(200, payload)) as unknown as typeof fetch;
    await expect(fetchDshDeviceToken({ accessToken: 't', fetchImpl })).resolves.toBeNull();
  });

  it('persists a device id', () => {
    const a = getOrCreateDshDeviceId();
    const b = getOrCreateDshDeviceId();
    expect(a).toBe(b);
    expect(a.startsWith('web.')).toBe(true);
  });
});

describe('syncDshIngressCookie', () => {
  it('writes and clears the access_token cookie without echoing secrets in the helper API', () => {
    const jar = { cookie: '' };
    syncDshIngressCookie('jwt.token.here', Date.now() / 1000 + 120, jar);
    expect(jar.cookie).toContain('access_token=jwt.token.here');
    expect(jar.cookie).toContain('Path=/');
    syncDshIngressCookie(null, undefined, jar);
    expect(jar.cookie).toContain('Max-Age=0');
  });
});
