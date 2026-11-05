import { afterEach, describe, expect, it, jest } from '@jest/globals';

import {
  DSH_SESSION_OPEN_PATH,
  closeDshSession,
  denyCodeFromHttp,
  isQueuedSession,
  openDshSession,
  pickDshSessionPayload,
  resolveDshAccessToken,
} from '@/components/dsh-agent/dsh-session';

jest.mock('@/application/session/token', () => ({
  getTokenParsed: jest.fn(() => null),
}));

function jsonResponse(status: number, body: unknown): Response {
  return {
    status,
    ok: status >= 200 && status < 300,
    text: async () => (typeof body === 'string' ? body : JSON.stringify(body)),
  } as unknown as Response;
}

describe('openDshSession', () => {
  afterEach(() => {
    jest.clearAllMocks();
    window.localStorage.clear();
  });

  it('E0-T11 reads access_token or accessToken', () => {
    expect(resolveDshAccessToken({ access_token: 'abc' })).toBe('abc');
    expect(resolveDshAccessToken({ accessToken: 'def' })).toBe('def');
    expect(resolveDshAccessToken({})).toBeNull();
  });

  it('returns NEED_AUTH without an access token and does not fetch', async () => {
    const fetchImpl = jest.fn();
    await expect(
      openDshSession({ workspaceId: 'ws-1', fetchImpl: fetchImpl as unknown as typeof fetch })
    ).resolves.toEqual({ kind: 'denied', status: 401, code: 'NEED_AUTH' });
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it('returns SESSION_DENIED on 404 when the pool BFF is not deployed', async () => {
    const fetchImpl = jest.fn(async () => jsonResponse(404, {})) as unknown as typeof fetch;
    await expect(
      openDshSession({ workspaceId: 'ws-1', accessToken: 'jwt', fetchImpl })
    ).resolves.toEqual({ kind: 'denied', status: 404, code: 'SESSION_DENIED' });
  });

  it('returns ready webUrl from Cloud data', async () => {
    const fetchImpl = jest.fn(async () =>
      jsonResponse(200, {
        code: 0,
        data: {
          sessionRef: 'sess-1',
          instanceRef: 'inst-1',
          webUrl: 'https://openmuseai.com/u/abcd/',
        },
      })
    ) as unknown as typeof fetch;
    const opened = await openDshSession({
      workspaceId: 'ws-1',
      deviceId: 'web.1',
      accessToken: 'jwt',
      attachmentId: 'att_1',
      fetchImpl,
    });
    expect(opened).toEqual({
      kind: 'ready',
      session: {
        sessionRef: 'sess-1',
        instanceRef: 'inst-1',
        webUrl: 'https://openmuseai.com/u/abcd/',
      },
    });
    expect(fetchImpl).toHaveBeenCalledWith(
      DSH_SESSION_OPEN_PATH,
      expect.objectContaining({
        method: 'POST',
        headers: expect.objectContaining({ 'X-Muse-Attachment-Id': 'att_1' }),
      })
    );
  });

  it('returns queued on 503 with queuePosition and does not sleep-retry', async () => {
    const fetchImpl = jest.fn(async () =>
      jsonResponse(503, {
        data: { sessionRef: 'sess-q', queuePosition: 1, retryAfterMs: 10_000 },
      })
    ) as unknown as typeof fetch;
    await expect(
      openDshSession({ workspaceId: 'ws-1', accessToken: 'jwt', fetchImpl })
    ).resolves.toEqual({
      kind: 'queued',
      session: { sessionRef: 'sess-q', queuePosition: 1, retryAfterMs: 10_000 },
    });
    expect(fetchImpl).toHaveBeenCalledTimes(1);
  });

  it('rejects javascript webUrl', () => {
    expect(
      pickDshSessionPayload({ sessionRef: 's', webUrl: 'javascript:alert(1)' })
    ).toEqual({ sessionRef: 's' });
  });

  it('maps deny codes', () => {
    expect(denyCodeFromHttp(401, {})).toBe('SESSION_DENIED');
    expect(denyCodeFromHttp(400, { message: 'SCOPE_MISMATCH' })).toBe('SCOPE_MISMATCH');
    expect(denyCodeFromHttp(0, null)).toBe('POOL_UNAVAILABLE');
    expect(isQueuedSession(503, { sessionRef: 's', queuePosition: 1, retryAfterMs: 1000 })).toBe(true);
    expect(isQueuedSession(200, { sessionRef: 's', webUrl: 'https://x/' })).toBe(false);
  });
});

describe('closeDshSession', () => {
  it('posts close when a session exists', async () => {
    const fetchImpl = jest.fn(async () => jsonResponse(200, { code: 0 })) as unknown as typeof fetch;
    await closeDshSession({ sessionRef: 'sess-1', deviceId: 'web.1', accessToken: 'jwt', fetchImpl });
    expect(fetchImpl).toHaveBeenCalled();
  });
});
