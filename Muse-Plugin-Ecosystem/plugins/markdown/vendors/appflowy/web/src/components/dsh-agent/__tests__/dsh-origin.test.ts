import { afterEach, describe, expect, it, jest } from '@jest/globals';

jest.mock('@/utils/runtime-config', () => ({
  getConfigValue: jest.fn((_key: string, defaultValue: string) => defaultValue),
}));

import { getConfigValue } from '@/utils/runtime-config';
import {
  DEFAULT_DSH_ORIGIN,
  MUSE_DSH_FRAME_SOURCE,
  frameSrcCspValue,
  getDshAgentUrl,
  canMountDshIframe,
  isAllowedDshMessageOrigin,
  isLoopbackDshOrigin,
  isSharedDshFrameSrc,
  normalizeDshFrameSrc,
  normalizeDshOrigin,
  parseDshFrameReady,
  parseBridgeReply,
  probeDshOrigin,
  shouldFallbackToConfiguredDsh,
} from '@/components/dsh-agent/dsh-origin';

const mockGetConfigValue = getConfigValue as jest.MockedFunction<typeof getConfigValue>;

describe('dsh-origin', () => {
  afterEach(() => {
    jest.clearAllMocks();
  });

  it('normalizes origin and rejects non-http schemes', () => {
    expect(normalizeDshOrigin('https://dsh.example.com/chat/')).toBe('https://dsh.example.com');
    expect(normalizeDshOrigin('javascript:alert(1)')).toBe(DEFAULT_DSH_ORIGIN);
    expect(normalizeDshOrigin('')).toBe(DEFAULT_DSH_ORIGIN);
    expect(normalizeDshFrameSrc('https://openmuseai.com/dsh')).toBe('https://openmuseai.com/dsh/');
    expect(normalizeDshFrameSrc('https://dsh.example.com/chat/')).toBe('https://dsh.example.com/chat/');
    expect(normalizeDshFrameSrc('https://dsh.example.com')).toBe('https://dsh.example.com');
    expect(isSharedDshFrameSrc('https://openmuseai.com/dsh/')).toBe(true);
    expect(isSharedDshFrameSrc('https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/')).toBe(false);
    expect(canMountDshIframe('https://openmuseai.com/dsh/')).toBe(false);
    expect(canMountDshIframe('https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/')).toBe(true);
    expect(shouldFallbackToConfiguredDsh('https://openmuseai.com/dsh/')).toBe(false);
    expect(shouldFallbackToConfiguredDsh('http://127.0.0.1:3080')).toBe(true);
  });

  it('detects loopback hosts', () => {
    expect(isLoopbackDshOrigin('http://127.0.0.1:3080')).toBe(true);
    expect(isLoopbackDshOrigin('http://localhost:3080')).toBe(true);
    expect(isLoopbackDshOrigin('https://dsh.example.com')).toBe(false);
  });

  it('rejects postMessage from a different origin', () => {
    expect(isAllowedDshMessageOrigin('https://evil.test', 'https://dsh.example.com')).toBe(false);
    expect(isAllowedDshMessageOrigin('https://dsh.example.com', 'https://dsh.example.com/')).toBe(true);
    expect(isAllowedDshMessageOrigin('https://openmuseai.com', 'https://openmuseai.com/dsh/')).toBe(true);
  });

  it('builds frame-src CSP for the configured origin', () => {
    expect(frameSrcCspValue('https://dsh.example.com/app')).toBe("frame-src 'self' https://dsh.example.com");
  });

  it('reads DSH URL from runtime config and keeps a same-origin path', () => {
    mockGetConfigValue.mockReturnValue('https://openmuseai.com/dsh/');
    expect(getDshAgentUrl()).toBe('https://openmuseai.com/dsh/');
  });

  it('parses frame-ready and ignores other payloads', () => {
    expect(parseDshFrameReady({ source: MUSE_DSH_FRAME_SOURCE, type: 'frame-ready' })).toEqual({
      source: MUSE_DSH_FRAME_SOURCE,
      type: 'frame-ready',
    });
    expect(parseDshFrameReady({ source: 'other', type: 'frame-ready' })).toBeNull();
    expect(parseDshFrameReady('ready')).toBeNull();
  });

  it('parses bridge.reply for bind ACK', () => {
    expect(
      parseBridgeReply({
        source: MUSE_DSH_FRAME_SOURCE,
        type: 'bridge.reply',
        requestId: 'req_bind_1',
        status: 200,
        body: { ok: true, bound: 'ws-1' },
      })
    ).toEqual({
      source: MUSE_DSH_FRAME_SOURCE,
      type: 'bridge.reply',
      requestId: 'req_bind_1',
      status: 200,
      body: { ok: true, bound: 'ws-1' },
    });
    expect(parseBridgeReply({ source: MUSE_DSH_FRAME_SOURCE, type: 'frame-ready' })).toBeNull();
  });

  it('probe treats a successful no-cors fetch as ok', async () => {
    const fetchImpl = jest.fn(async () => ({ ok: true } as Response));
    await expect(probeDshOrigin('http://127.0.0.1:3080', fetchImpl as unknown as typeof fetch)).resolves.toBe('ok');
    expect(fetchImpl).toHaveBeenCalled();
  });

  it('probe treats a rejected fetch as down', async () => {
    const fetchImpl = jest.fn(async () => {
      throw new Error('Failed to fetch');
    }) as unknown as typeof fetch;
    await expect(probeDshOrigin('http://127.0.0.1:3080', fetchImpl)).resolves.toBe('down');
  });
});
