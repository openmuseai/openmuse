import { afterEach, beforeEach, describe, expect, it, jest } from '@jest/globals';
import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import React from 'react';

jest.mock('@/components/ai-chat/Resizer', () => ({
  __esModule: true,
  default: () => <div data-testid='dsh-resizer' />,
}));

const mockProbe = jest.fn(async () => 'ok' as const);
let mockDshAgentUrl = 'http://127.0.0.1:3080';

jest.mock('@/utils/runtime-config', () => ({
  getConfigValue: jest.fn((key: string, defaultValue: string) => {
    if (key === 'APPFLOWY_DSH_AGENT_URL') return mockDshAgentUrl;
    return defaultValue;
  }),
}));

jest.mock('@/components/dsh-agent/dsh-origin', () => {
  const actual = jest.requireActual('@/components/dsh-agent/dsh-origin') as typeof import('@/components/dsh-agent/dsh-origin');
  return {
    ...actual,
    probeDshOrigin: (...args: unknown[]) => mockProbe(...args),
  };
});

jest.mock('@/components/dsh-agent/dsh-device-token', () => ({
  fetchDshDeviceToken: jest.fn(async () => null),
  getOrCreateDshDeviceId: () => 'web.test-device',
  syncDshIngressCookie: jest.fn(),
}));

const mockGetTokenParsed = jest.fn(() => ({
  access_token: 'test-jwt',
  expires_at: 9_999_999_999,
  refresh_token: 'refresh',
  user: { id: 'user-1', email: 'a@b.c' },
}));

jest.mock('@/application/session/token', () => ({
  getTokenParsed: () => mockGetTokenParsed(),
}));

jest.mock('@/components/dsh-agent/dsh-session', () => {
  const actual = jest.requireActual('@/components/dsh-agent/dsh-session') as typeof import('@/components/dsh-agent/dsh-session');
  return {
    ...actual,
    openDshSession: jest.fn(async () => ({
      kind: 'denied' as const,
      status: 404,
      code: 'SESSION_DENIED' as const,
    })),
    closeDshSession: jest.fn(async () => undefined),
    heartbeatDshSession: jest.fn(async () => undefined),
  };
});

jest.mock('@/components/dsh-agent/dsh-app-state', () => ({
  useDshWorkspaceId: () => mockWorkspaceId,
  useDshWorkspaceTitle: () => undefined,
  useDshViewId: () => undefined,
  useDshOutline: () => undefined,
  useDshToView: () => undefined,
}));

import { DshAgentProvider } from '@/components/dsh-agent/DshAgentProvider';
import { closeDshSession, openDshSession } from '@/components/dsh-agent/dsh-session';
import { fetchDshDeviceToken } from '@/components/dsh-agent/dsh-device-token';
import { BIND_ACK_TIMEOUT_MS } from '@/components/dsh-agent/dsh-attachment';

const mockOpenDshSession = openDshSession as jest.MockedFunction<typeof openDshSession>;
const mockCloseDshSession = closeDshSession as jest.MockedFunction<typeof closeDshSession>;
const mockFetchDshDeviceToken = fetchDshDeviceToken as jest.MockedFunction<typeof fetchDshDeviceToken>;
let mockWorkspaceId: string | undefined;

const TENANT_URL = 'https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/';

function renderPanel() {
  return render(
    <DshAgentProvider>
      <button type='button' data-testid='external-toggle'>
        noop
      </button>
    </DshAgentProvider>
  );
}

describe('DshAgentPanel lifecycle (E0)', () => {
  beforeEach(() => {
    mockProbe.mockReset();
    mockProbe.mockResolvedValue('ok');
    mockWorkspaceId = undefined;
    mockDshAgentUrl = 'http://127.0.0.1:3080';
    mockGetTokenParsed.mockReset();
    mockGetTokenParsed.mockReturnValue({
      access_token: 'test-jwt',
      expires_at: 9_999_999_999,
      refresh_token: 'refresh',
      user: { id: 'user-1', email: 'a@b.c' },
    });
    mockOpenDshSession.mockReset();
    mockOpenDshSession.mockResolvedValue({ kind: 'denied', status: 404, code: 'SESSION_DENIED' });
    mockCloseDshSession.mockReset();
    mockCloseDshSession.mockResolvedValue(undefined);
    mockFetchDshDeviceToken.mockReset();
    mockFetchDshDeviceToken.mockResolvedValue(null);
    window.localStorage.clear();
  });

  afterEach(() => {
    jest.useRealTimers();
    jest.clearAllMocks();
  });

  it('does not create an iframe before the panel is opened', () => {
    renderPanel();
    expect(screen.queryByTestId('dsh-agent-iframe')).toBeNull();
    expect(screen.queryByTestId('dsh-agent-panel')).toBeNull();
  });

  it('E0-T1/T6 loads session webUrl and posts session/open', async () => {
    mockWorkspaceId = 'ws-1';
    mockOpenDshSession.mockResolvedValue({
      kind: 'ready',
      session: { sessionRef: 'sess-1', webUrl: TENANT_URL },
    });
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();

    const iframe = await waitFor(() => screen.getByTestId('dsh-agent-iframe'));
    expect(iframe.getAttribute('src')).toBe(TENANT_URL);
    expect(mockOpenDshSession).toHaveBeenCalled();
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-attachment-stage')).toBe('Presenting');
    expect(screen.getByTestId('dsh-agent-pending')).toBeTruthy();
  });

  it('E0-T2 does not iframe the configured /dsh/ path', async () => {
    mockDshAgentUrl = 'https://openmuseai.com/dsh/';
    mockWorkspaceId = 'ws-1';
    mockOpenDshSession.mockResolvedValue({
      kind: 'ready',
      session: { sessionRef: 'sess-1', webUrl: TENANT_URL },
    });
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();

    const iframe = await waitFor(() => screen.getByTestId('dsh-agent-iframe'));
    expect(iframe.getAttribute('src')).toBe(TENANT_URL);
    expect(iframe.getAttribute('src')).not.toContain('/dsh');
  });

  it('E0-T4 does not POST without a JWT', async () => {
    mockGetTokenParsed.mockReturnValue(null);
    mockWorkspaceId = 'ws-1';
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();

    const error = await waitFor(() => screen.getByTestId('dsh-agent-error'));
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-error-code')).toBe('NEED_AUTH');
    expect(error.textContent).toMatch(/Sign in/);
    expect(mockOpenDshSession).not.toHaveBeenCalled();
    expect(screen.getByTestId('dsh-agent-retry')).toBeTruthy();
  });

  it('E0-T5 does not POST without a workspace', async () => {
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();

    const error = await waitFor(() => screen.getByTestId('dsh-agent-error'));
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-error-code')).toBe('NEED_WORKSPACE');
    expect(error.textContent).toMatch(/workspace/i);
    expect(mockOpenDshSession).not.toHaveBeenCalled();
    expect(screen.queryByTestId('dsh-agent-iframe')).toBeNull();
  });

  it('E0-T7 shows the waiting room then retries', async () => {
    mockWorkspaceId = 'ws-1';
    mockOpenDshSession
      .mockResolvedValueOnce({
        kind: 'queued',
        session: { sessionRef: 'sess-q', queuePosition: 1, retryAfterMs: 30 },
      })
      .mockResolvedValueOnce({
        kind: 'ready',
        session: { sessionRef: 'sess-1', webUrl: TENANT_URL },
      });
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();

    await waitFor(() =>
      expect(screen.getByTestId('dsh-agent-pending').textContent).toMatch(/position 1/)
    );
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-attachment-stage')).toBe('Queued');
    expect(screen.queryByTestId('dsh-agent-iframe')).toBeNull();

    const iframe = await waitFor(() => screen.getByTestId('dsh-agent-iframe'), { timeout: 2000 });
    expect(iframe.getAttribute('src')).toBe(TENANT_URL);
    expect(mockOpenDshSession.mock.calls.length).toBeGreaterThanOrEqual(2);
  });

  it('E0-T8 refuses a /dsh/ webUrl', async () => {
    mockDshAgentUrl = 'https://openmuseai.com/dsh/';
    mockWorkspaceId = 'ws-1';
    mockOpenDshSession.mockResolvedValue({
      kind: 'ready',
      session: { sessionRef: 'sess-1', webUrl: 'https://openmuseai.com/dsh/' },
    });
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();

    await waitFor(() => screen.getByTestId('dsh-agent-error'));
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-error-code')).toBe('SESSION_DENIED');
    expect(screen.queryByTestId('dsh-agent-iframe')).toBeNull();
  });

  it('E0-T9 keeps only the latest workspace webUrl', async () => {
    mockWorkspaceId = 'ws-1';
    let resolveFirst: ((value: Awaited<ReturnType<typeof openDshSession>>) => void) | undefined;
    mockOpenDshSession.mockImplementationOnce(
      () =>
        new Promise((resolve) => {
          resolveFirst = resolve;
        })
    );
    mockOpenDshSession.mockResolvedValue({
      kind: 'ready',
      session: { sessionRef: 'sess-2', webUrl: TENANT_URL },
    });
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    const view = renderPanel();

    mockWorkspaceId = 'ws-2';
    view.rerender(
      <DshAgentProvider>
        <button type='button' data-testid='external-toggle'>
          noop
        </button>
      </DshAgentProvider>
    );

    resolveFirst?.({
      kind: 'ready',
      session: {
        sessionRef: 'sess-1',
        webUrl: 'https://openmuseai.com/u/11111111111111111111111111111111/',
      },
    });

    await waitFor(() =>
      expect(screen.getByTestId('dsh-agent-iframe').getAttribute('src')).toBe(TENANT_URL)
    );
  });

  it('E0-T10 closes the session when the panel is closed', async () => {
    mockWorkspaceId = 'ws-1';
    mockOpenDshSession.mockResolvedValue({
      kind: 'ready',
      session: { sessionRef: 'sess-1', webUrl: TENANT_URL },
    });
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();

    const iframe = await waitFor(() => screen.getByTestId('dsh-agent-iframe'));
    await act(async () => {
      fireEvent.click(screen.getByLabelText('Close DeepSeek Agent panel'));
    });
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-open')).toBe('false');
    expect(screen.getByTestId('dsh-agent-iframe')).toBe(iframe);
    expect(mockCloseDshSession).toHaveBeenCalled();
  });

  it('E0-T13 never renders an empty content well', async () => {
    mockGetTokenParsed.mockReturnValue(null);
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();
    await waitFor(() => screen.getByTestId('dsh-agent-panel'));
    expect(
      screen.queryByTestId('dsh-agent-iframe') ||
        screen.queryByTestId('dsh-agent-error') ||
        screen.queryByTestId('dsh-agent-pending')
    ).toBeTruthy();
  });

  it('shows a sidecar error when loopback probe fails', async () => {
    mockProbe.mockResolvedValue('down');
    mockWorkspaceId = 'ws-1';
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();

    const error = await waitFor(() => screen.getByTestId('dsh-agent-error'));
    expect(error.textContent).toMatch(/Start the local DSH sidecar/);
    expect(screen.queryByTestId('dsh-agent-iframe')).toBeNull();
  });
});

function dispatchFrameMessage(iframe: HTMLIFrameElement, data: unknown) {
  const source = iframe.contentWindow;
  if (!source) {
    throw new Error('iframe has no contentWindow');
  }
  const origin = new URL(iframe.src).origin;
  window.dispatchEvent(
    new MessageEvent('message', {
      origin,
      source,
      data,
    })
  );
}

describe('DshAgentPanel bind RPC (E1)', () => {
  beforeEach(() => {
    mockProbe.mockReset();
    mockProbe.mockResolvedValue('ok');
    mockWorkspaceId = 'ws-1';
    mockDshAgentUrl = 'http://127.0.0.1:3080';
    mockGetTokenParsed.mockReset();
    mockGetTokenParsed.mockReturnValue({
      access_token: 'test-jwt',
      expires_at: 9_999_999_999,
      refresh_token: 'refresh',
      user: { id: 'user-1', email: 'a@b.c' },
    });
    mockOpenDshSession.mockReset();
    mockOpenDshSession.mockResolvedValue({
      kind: 'ready',
      session: { sessionRef: 'sess-1', webUrl: TENANT_URL },
    });
    mockCloseDshSession.mockReset();
    mockCloseDshSession.mockResolvedValue(undefined);
    mockFetchDshDeviceToken.mockReset();
    mockFetchDshDeviceToken.mockResolvedValue({
      token: 'device.token',
      kid: 'v1',
      expiresAt: 9_999_999_999_999,
      deviceId: 'web.test-device',
    });
    window.localStorage.clear();
  });

  afterEach(() => {
    jest.useRealTimers();
    jest.clearAllMocks();
  });

  it('E1-T6 does not enter HybridLive or contribute before bind ACK', async () => {
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();
    const iframe = (await waitFor(() => screen.getByTestId('dsh-agent-iframe'))) as HTMLIFrameElement;
    const post = jest.spyOn(iframe.contentWindow as Window, 'postMessage');
    await act(async () => {
      dispatchFrameMessage(iframe, { source: 'muse.dsh-web', type: 'frame-ready' });
    });
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-attachment-stage')).not.toBe(
      'HybridLive'
    );
    expect(post.mock.calls.filter((call) => (call[0] as { type?: string })?.type === 'context.contribute')).toHaveLength(
      0
    );
  });

  it('E1-T7 enters HybridLive after bound ACK', async () => {
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();
    const iframe = (await waitFor(() => screen.getByTestId('dsh-agent-iframe'))) as HTMLIFrameElement;
    await act(async () => {
      dispatchFrameMessage(iframe, { source: 'muse.dsh-web', type: 'frame-ready' });
    });
    await act(async () => {
      dispatchFrameMessage(iframe, {
        source: 'muse.dsh-web',
        type: 'bridge.reply',
        status: 200,
        body: { ok: true, bound: 'ws-1' },
      });
    });
    await waitFor(() =>
      expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-attachment-stage')).toBe(
        'HybridLive'
      )
    );
  });

  it('E1-T9 times out to BRIDGE_SILENT and keeps the iframe', async () => {
    jest.useFakeTimers();
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();
    const iframe = (await waitFor(() => screen.getByTestId('dsh-agent-iframe'))) as HTMLIFrameElement;
    await act(async () => {
      dispatchFrameMessage(iframe, { source: 'muse.dsh-web', type: 'frame-ready' });
    });
    await act(async () => {
      jest.advanceTimersByTime(BIND_ACK_TIMEOUT_MS);
    });
    await waitFor(() =>
      expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-error-code')).toBe(
        'BRIDGE_SILENT'
      )
    );
    expect(screen.getByTestId('dsh-agent-iframe')).toBe(iframe);
  });

  it('E1-T11 maps rejected bind to BIND_REJECTED and keeps the iframe', async () => {
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();
    const iframe = (await waitFor(() => screen.getByTestId('dsh-agent-iframe'))) as HTMLIFrameElement;
    await act(async () => {
      dispatchFrameMessage(iframe, { source: 'muse.dsh-web', type: 'frame-ready' });
    });
    await act(async () => {
      dispatchFrameMessage(iframe, {
        source: 'muse.dsh-web',
        type: 'bridge.reply',
        status: 401,
        body: { ok: false, error: 'DEVICE_AUTH_REJECTED' },
      });
    });
    await waitFor(() =>
      expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-error-code')).toBe(
        'BIND_REJECTED'
      )
    );
    expect(screen.getByTestId('dsh-agent-iframe')).toBe(iframe);
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-attachment-stage')).not.toBe(
      'HybridLive'
    );
  });

  it('E3-T1/T9 keeps attachmentId on retry and exposes a copy control', async () => {
    mockGetTokenParsed.mockReturnValue(null);
    window.localStorage.setItem('appflowy.dshAgent.open', '1');
    renderPanel();
    const panel = await waitFor(() => screen.getByTestId('dsh-agent-panel'));
    const id = panel.getAttribute('data-attachment-id');
    expect(id).toMatch(/^att_/);
    expect(screen.getByTestId('dsh-agent-copy-id').textContent).toBe(id);
    await act(async () => {
      fireEvent.click(screen.getByTestId('dsh-agent-retry'));
    });
    expect(screen.getByTestId('dsh-agent-panel').getAttribute('data-attachment-id')).toBe(id);
  });
});
