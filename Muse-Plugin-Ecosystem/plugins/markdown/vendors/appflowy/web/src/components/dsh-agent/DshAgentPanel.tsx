import { IconButton, Tooltip } from '@mui/material';
import React, { useEffect, useMemo, useRef, useState } from 'react';

import { ReactComponent as Close } from '@/assets/icons/close.svg';
import { ReactComponent as AiChat } from '@/assets/icons/ai_chat.svg';
import {
  useDshOutline,
  useDshToView,
  useDshViewId,
  useDshWorkspaceId,
  useDshWorkspaceTitle,
} from '@/components/dsh-agent/dsh-app-state';
import { getOutlineExpands } from '@/components/_shared/outline/utils';
import { useDshAgentContext } from '@/components/dsh-agent/DshAgentProvider';
import Resizer from '@/components/ai-chat/Resizer';
import {
  attachmentError,
  BIND_ACK_TIMEOUT_MS,
  createAttachment,
  keepFrameOnFailed,
  placingCopy,
  queuedCopy,
  stageRequiresFrame,
  type AgentAttachment,
} from '@/components/dsh-agent/dsh-attachment';
import { fetchDshDeviceToken, getOrCreateDshDeviceId, syncDshIngressCookie } from '@/components/dsh-agent/dsh-device-token';
import { closeDshSession, heartbeatDshSession, openDshSession, resolveDshAccessToken } from '@/components/dsh-agent/dsh-session';
import { getTokenParsed } from '@/application/session/token';
import {
  buildMarkdownSelectionEnvelope,
  buildMarkdownSnapshotEnvelope,
  buildMarkdownSurfaceEnvelope,
  buildTreeUiEnvelope,
  buildWorkspaceCatalogEnvelope,
  buildWorkspaceFocusEnvelope,
  contributionMessageOrNull,
  documentSurfaceRef,
  expandedViewIdsFromStorage,
  readVisibleEditorText,
  receiptOrNull,
  workspaceSurfaceRef,
} from '@/components/dsh-agent/dsh-hybrid';
import { WorkspaceUiFacet } from './dsh-workspace-facet';
import {
  dshMessageOrigin,
  isAllowedDshMessageOrigin,
  canMountDshIframe,
  isLoopbackDshOrigin,
  parentHelloMessage,
  shouldFallbackToConfiguredDsh,
  parseDshFrameReady,
  parseBridgeReply,
  parseIntentDispatch,
  probeDshOrigin,
  surfaceClosedMessage,
  webHybridFlagOn,
  workspaceBindMessage,
} from '@/components/dsh-agent/dsh-origin';

/**
 * Hosts the official DSH Web Client in an iframe (compatibility surface).
 * Closing the panel collapses width to 0; the iframe stays mounted after first open.
 */
function DshAgentPanel() {
  const { open, width, url, setOpen, setWidth } = useDshAgentContext();
  const [everOpened, setEverOpened] = useState(open);
  const [iframeKey, setIframeKey] = useState(0);
  const [retryNonce, setRetryNonce] = useState(0);
  const [attachment, setAttachment] = useState<AgentAttachment>(() => createAttachment());
  const generationRef = useRef(0);
  const queuedTimerRef = useRef<number | undefined>(undefined);
  const bindTimerRef = useRef<number | undefined>(undefined);
  const bindSentRef = useRef(false);
  const deviceIssueSettledRef = useRef(false);
  const sessionRef = useRef<string | null>(null);
  const [deviceToken, setDeviceToken] = useState<{
    token: string;
    kid?: string;
    expiresAt?: number;
    deviceId?: string;
  } | null>(null);
  const iframeRef = useRef<HTMLIFrameElement | null>(null);
  const workspaceId = useDshWorkspaceId();
  const viewId = useDshViewId();
  const outline = useDshOutline();
  const toView = useDshToView();
  const workspaceTitle = useDshWorkspaceTitle();
  const viewTitle = useMemo(() => {
    if (!viewId || !outline) return undefined;
    const stack = [...outline];
    while (stack.length > 0) {
      const current = stack.pop();
      if (!current) continue;
      if (current.view_id === viewId) return current.name;
      if (current.children) stack.push(...current.children);
    }
    return undefined;
  }, [outline, viewId]);
  const [selection, setSelection] = useState({ text: '', collapsed: true });
  const [expandedIds, setExpandedIds] = useState<string[]>([]);
  const contextRevision = useRef(0);
  const previousDocSurface = useRef<string | undefined>(undefined);
  const previousWorkspaceSurface = useRef<string | undefined>(undefined);
  const frameUrl = attachment.frameUrl;

  const postToFrame = (data: unknown) => {
    const frame = iframeRef.current?.contentWindow;
    if (!frame) return;
    frame.postMessage(data, dshMessageOrigin(frameUrl));
  };

  useEffect(() => {
    if (open) setEverOpened(true);
  }, [open]);

  useEffect(() => {
    if (!open) return;
    let cancelled = false;
    deviceIssueSettledRef.current = false;
    void fetchDshDeviceToken().then((issued) => {
      if (cancelled) return;
      deviceIssueSettledRef.current = true;
      if (!issued) {
        setDeviceToken(null);
        return;
      }
      setDeviceToken({
        token: issued.token,
        kid: issued.kid,
        expiresAt: issued.expiresAt,
        deviceId: issued.deviceId,
      });
    });
    return () => {
      cancelled = true;
    };
  }, [open, url, retryNonce]);

  useEffect(() => {
    if (!open) {
      generationRef.current += 1;
      const current = sessionRef.current;
      sessionRef.current = null;
      if (current) {
        void closeDshSession({ sessionRef: current, deviceId: getOrCreateDshDeviceId() });
      }
      return;
    }
    const gen = ++generationRef.current;
    bindSentRef.current = false;
    clearBindTimer();
    const parsed = getTokenParsed();
    const token = resolveDshAccessToken(parsed);
    if (!token) {
      setAttachment((prev) => ({
        ...prev,
        generation: gen,
        stage: 'Failed',
        error: attachmentError('ResolvingIdentity', 'NEED_AUTH'),
        frameLoaded: false,
      }));
      return;
    }
    if (!workspaceId) {
      setAttachment((prev) => ({
        ...prev,
        generation: gen,
        stage: 'Failed',
        error: attachmentError('ResolvingIdentity', 'NEED_WORKSPACE'),
        frameLoaded: false,
      }));
      return;
    }
    syncDshIngressCookie(token, parsed?.expires_at);
    setAttachment((prev) => ({
      ...prev,
      generation: gen,
      stage: 'Placing',
      error: undefined,
      queuePosition: undefined,
      retryAfterMs: undefined,
      frameLoaded: false,
    }));
    void openDshSession({
      workspaceId,
      deviceId: getOrCreateDshDeviceId(),
      accessToken: token,
      attachmentId: attachment.attachmentId,
    }).then((result) => {
      if (gen !== generationRef.current) return;
      if (result.kind === 'queued') {
        sessionRef.current = result.session.sessionRef;
        setAttachment((prev) => ({
          ...prev,
          generation: gen,
          stage: 'Queued',
          sessionRef: result.session.sessionRef,
          queuePosition: result.session.queuePosition,
          retryAfterMs: result.session.retryAfterMs,
          error: undefined,
        }));
        const wait = result.session.retryAfterMs ?? 10_000;
        queuedTimerRef.current = window.setTimeout(() => {
          if (gen !== generationRef.current) return;
          setRetryNonce((n) => n + 1);
        }, wait);
        return;
      }
      if (result.kind === 'ready' && result.session.webUrl && canMountDshIframe(result.session.webUrl)) {
        sessionRef.current = result.session.sessionRef;
        setAttachment((prev) => ({
          ...prev,
          generation: gen,
          stage: 'Presenting',
          sessionRef: result.session.sessionRef,
          frameUrl: result.session.webUrl ?? '',
          error: undefined,
          frameLoaded: false,
        }));
        bindSentRef.current = false;
        return;
      }
      if (shouldFallbackToConfiguredDsh(url) && canMountDshIframe(url)) {
        setAttachment((prev) => ({
          ...prev,
          generation: gen,
          stage: 'Presenting',
          frameUrl: url,
          error: undefined,
          frameLoaded: false,
        }));
        return;
      }
      if (result.kind === 'ready') {
        setAttachment((prev) => ({
          ...prev,
          generation: gen,
          stage: 'Failed',
          error: attachmentError('Placing', 'SESSION_DENIED'),
          frameLoaded: false,
        }));
        return;
      }
      setAttachment((prev) => ({
        ...prev,
        generation: gen,
        stage: 'Failed',
        error: attachmentError('Placing', result.code),
        frameLoaded: false,
      }));
    });
    return () => {
      if (queuedTimerRef.current !== undefined) {
        window.clearTimeout(queuedTimerRef.current);
        queuedTimerRef.current = undefined;
      }
    };
  }, [open, url, workspaceId, retryNonce]);

  useEffect(() => {
    if (!open) return;
    const timer = window.setInterval(() => {
      const current = sessionRef.current;
      if (current) void heartbeatDshSession({ sessionRef: current });
    }, 60_000);
    return () => window.clearInterval(timer);
  }, [open]);

  useEffect(() => {
    if (attachment.stage !== 'Presenting') return;
    if (!canMountDshIframe(frameUrl) || !isLoopbackDshOrigin(frameUrl)) return;
    let cancelled = false;
    void probeDshOrigin(frameUrl).then((result) => {
      if (cancelled || generationRef.current !== attachment.generation) return;
      if (result === 'down') {
        setAttachment((prev) => ({
          ...prev,
          stage: 'Failed',
          error: attachmentError(
            'Presenting',
            'POOL_UNAVAILABLE',
            'Start the local DSH sidecar, then retry:\n./deploy/dev.sh local --no-mac --dsh'
          ),
        }));
      }
    });
    return () => {
      cancelled = true;
    };
  }, [attachment.stage, attachment.generation, frameUrl, iframeKey]);

  const helloPayload = (token: typeof deviceToken) =>
    parentHelloMessage(
      undefined,
      undefined,
      token ?? undefined,
      workspaceId && webHybridFlagOn('MUSE_WEB_WORKSPACE_BIND')
        ? { workspaceRef: workspaceId, workspaceTitle }
        : undefined,
      `req_hello_${attachment.generation}`
    );

  const bindPayload = (token: typeof deviceToken) =>
    workspaceId
      ? workspaceBindMessage(
          workspaceId,
          workspaceTitle,
          token ? { token: token.token, deviceId: token.deviceId } : undefined,
          `req_bind_${attachment.generation}`
        )
      : null;

  const clearBindTimer = () => {
    if (bindTimerRef.current !== undefined) {
      window.clearTimeout(bindTimerRef.current);
      bindTimerRef.current = undefined;
    }
  };

  const failBinding = (code: 'NO_DEVICE_TOKEN' | 'BIND_REJECTED' | 'BRIDGE_SILENT' | 'FLAG_OFF') => {
    clearBindTimer();
    setAttachment((prev) => {
      if (prev.stage === 'HybridLive') return prev;
      if (prev.stage !== 'Binding' && prev.stage !== 'Presenting' && prev.stage !== 'Failed') return prev;
      return {
        ...prev,
        stage: 'Failed',
        error: attachmentError('Binding', code),
      };
    });
  };

  const sendHelloAndBind = (token: NonNullable<typeof deviceToken>) => {
    const frame = iframeRef.current?.contentWindow;
    if (!frame || bindSentRef.current) return;
    bindSentRef.current = true;
    setAttachment((prev) => (prev.stage === 'Presenting' ? { ...prev, stage: 'Binding' } : prev));
    frame.postMessage(helloPayload(token), dshMessageOrigin(frameUrl));
    const bind = bindPayload(token);
    if (webHybridFlagOn('MUSE_WEB_WORKSPACE_BIND') && bind) {
      frame.postMessage(bind, dshMessageOrigin(frameUrl));
    }
    clearBindTimer();
    bindTimerRef.current = window.setTimeout(() => {
      failBinding('BRIDGE_SILENT');
    }, BIND_ACK_TIMEOUT_MS);
  };

  useEffect(() => {
    const onMessage = (event: MessageEvent) => {
      if (!isAllowedDshMessageOrigin(event.origin, frameUrl)) return;
      if (event.source !== iframeRef.current?.contentWindow) return;
      if (parseDshFrameReady(event.data)) {
        setAttachment((prev) =>
          prev.stage === 'Presenting' ? { ...prev, stage: 'Binding', frameLoaded: true } : prev
        );
        return;
      }
      const reply = parseBridgeReply(event.data);
      if (reply) {
        if (reply.body?.ok && typeof reply.body.bound === 'string' && reply.body.bound === workspaceId) {
          clearBindTimer();
          setAttachment((prev) => ({ ...prev, stage: 'HybridLive', error: undefined }));
          return;
        }
        if (reply.body && reply.body.ok === false) {
          const code =
            reply.body.error === 'NO_DEVICE_TOKEN'
              ? 'NO_DEVICE_TOKEN'
              : reply.body.error === 'FLAG_OFF'
                ? 'FLAG_OFF'
                : 'BIND_REJECTED';
          failBinding(code);
        }
      }
    };
    window.addEventListener('message', onMessage);
    return () => window.removeEventListener('message', onMessage);
  }, [frameUrl, workspaceId]);

  useEffect(() => {
    if (!everOpened) return;
    if (attachment.stage !== 'Binding') return;
    if (!canMountDshIframe(frameUrl)) return;
    if (deviceToken) {
      sendHelloAndBind(deviceToken);
      return;
    }
    if (deviceIssueSettledRef.current) {
      failBinding('NO_DEVICE_TOKEN');
    }
  }, [attachment.stage, everOpened, iframeKey, frameUrl, deviceToken, workspaceId, workspaceTitle]);

  useEffect(() => {
    return () => clearBindTimer();
  }, []);

  useEffect(() => {
    if (!webHybridFlagOn('MUSE_WEB_INTENT_DOWNLINK')) return;
    const facet = new WorkspaceUiFacet({ workspaceId, toView });
    let disposed = false;
    const onMessage = async (event: MessageEvent) => {
      if (!isAllowedDshMessageOrigin(event.origin, frameUrl)) return;
      if (event.source !== iframeRef.current?.contentWindow) return;
      const parsed = parseIntentDispatch(event.data);
      if (!parsed) return;
      const result = await facet.dispatch(parsed.intent);
      if (disposed) return;
      const receipt = receiptOrNull(result);
      if (!receipt) return;
      const frame = event.source;
      if (frame && 'postMessage' in frame) {
        (frame as Window).postMessage(receipt, dshMessageOrigin(frameUrl));
      }
    };
    window.addEventListener('message', onMessage);
    return () => { disposed = true; facet.close(); window.removeEventListener('message', onMessage); };
  }, [frameUrl, workspaceId, toView]);

  useEffect(() => {
    if (!webHybridFlagOn('MUSE_WEB_CONTEXT_UPLINK') || !everOpened) return;
    let timer: number | undefined;
    const publishExpands = () => {
      const next = expandedViewIdsFromStorage(getOutlineExpands());
      setExpandedIds((prev) =>
        prev.length === next.length && prev.every((id, index) => id === next[index]) ? prev : next
      );
    };
    publishExpands();
    const onStorage = (event: StorageEvent) => {
      if (event.key === 'outline_expanded') publishExpands();
    };
    window.addEventListener('storage', onStorage);
    timer = window.setInterval(publishExpands, 1500);
    return () => {
      window.removeEventListener('storage', onStorage);
      if (timer !== undefined) window.clearInterval(timer);
    };
  }, [everOpened]);

  useEffect(() => {
    if (!webHybridFlagOn('MUSE_WEB_CONTEXT_UPLINK') || !everOpened) return;
    let debounce: number | undefined;
    const onSel = () => {
      if (debounce !== undefined) window.clearTimeout(debounce);
      debounce = window.setTimeout(() => {
        const sel = document.getSelection();
        const text = (sel?.toString() ?? '').slice(0, 2048);
        setSelection({ text, collapsed: text.length === 0 });
      }, 200);
    };
    document.addEventListener('selectionchange', onSel);
    return () => {
      document.removeEventListener('selectionchange', onSel);
      if (debounce !== undefined) window.clearTimeout(debounce);
    };
  }, [everOpened]);

  useEffect(() => {
    if (!webHybridFlagOn('MUSE_WEB_CONTEXT_UPLINK') || !workspaceId || !everOpened) return;
    if (attachment.stage !== 'HybridLive') return;
    const handle = window.setTimeout(() => {
      contextRevision.current += 1;
      const revision = contextRevision.current;
      const nextWorkspaceSurface = workspaceSurfaceRef(workspaceId);
      if (previousWorkspaceSurface.current && previousWorkspaceSurface.current !== nextWorkspaceSurface) {
        postToFrame(surfaceClosedMessage(previousWorkspaceSurface.current));
      }
      previousWorkspaceSurface.current = nextWorkspaceSurface;
      const nextDocSurface = viewId ? documentSurfaceRef(viewId) : undefined;
      if (previousDocSurface.current && previousDocSurface.current !== nextDocSurface) {
        postToFrame(surfaceClosedMessage(previousDocSurface.current));
      }
      previousDocSurface.current = nextDocSurface;

      const messages = [
        contributionMessageOrNull(
          buildWorkspaceFocusEnvelope({
            workspaceId,
            title: workspaceTitle,
            viewId,
            revision,
          })
        ),
        contributionMessageOrNull(
          buildTreeUiEnvelope({
            workspaceId,
            expandedViewIds: expandedIds,
            revision,
          })
        ),
      ];
      if (outline !== undefined) {
        messages.push(
          contributionMessageOrNull(
            buildWorkspaceCatalogEnvelope({
              workspaceId,
              outline,
              revision,
            })
          )
        );
      }
      if (viewId) {
        messages.push(
          contributionMessageOrNull(
            buildMarkdownSurfaceEnvelope({
              workspaceId,
              viewId,
              title: viewTitle,
              revision,
            })
          ),
          contributionMessageOrNull(
            buildMarkdownSelectionEnvelope({
              workspaceId,
              viewId,
              selectedText: selection.text,
              collapsed: selection.collapsed,
              revision,
            })
          )
        );
        const editor = readVisibleEditorText();
        const snapshotText = editor.text.length > 0 ? editor.text : selection.text;
        if (snapshotText.trim().length > 0) {
          messages.push(
            contributionMessageOrNull(
              buildMarkdownSnapshotEnvelope({
                workspaceId,
                viewId,
                text: snapshotText,
                revision,
              })
            )
          );
        }
      }
      for (const message of messages) {
        if (message) postToFrame(message);
      }
    }, 200);
    return () => window.clearTimeout(handle);
  }, [
    workspaceId,
    workspaceTitle,
    viewId,
    viewTitle,
    expandedIds,
    outline,
    selection,
    everOpened,
    frameUrl,
    attachment.stage,
  ]);

  const title = useMemo(() => {
    if (!frameUrl) return 'Starting…';
    try {
      return new URL(frameUrl).host;
    } catch {
      return frameUrl;
    }
  }, [frameUrl]);

  const keepFailedFrame =
    attachment.stage === 'Failed' &&
    keepFrameOnFailed(attachment.error?.code) &&
    canMountDshIframe(frameUrl);
  const showFrame =
    everOpened &&
    canMountDshIframe(frameUrl) &&
    (stageRequiresFrame(attachment.stage) || keepFailedFrame);
  const pendingCopy =
    attachment.stage === 'Queued'
      ? queuedCopy(attachment.queuePosition)
      : placingCopy();

  const retry = () => {
    bindSentRef.current = false;
    clearBindTimer();
    setAttachment((prev) => ({
      ...prev,
      frameLoaded: false,
      error: undefined,
      stage: 'Placing',
    }));
    setIframeKey((k) => k + 1);
    setRetryNonce((n) => n + 1);
  };

  if (!everOpened && !open) {
    return null;
  }

  const content =
    showFrame ? (
      <>
        {!attachment.frameLoaded && attachment.stage !== 'Failed' ? (
          <div
            className='absolute inset-0 z-[1] flex items-center justify-center bg-background-primary p-6 text-xs text-text-secondary'
            data-testid='dsh-agent-pending'
          >
            Loading agent UI…
          </div>
        ) : null}
        {attachment.stage === 'Failed' && attachment.error ? (
          <div
            className='absolute inset-0 z-[2] flex flex-col items-center justify-center gap-3 bg-background-primary/90 p-6 text-center'
            data-testid='dsh-agent-error'
          >
            <p className='text-sm text-text-primary'>Agent is unavailable</p>
            <p className='max-w-xs whitespace-pre-line text-xs text-text-secondary'>
              {attachment.error.message}
            </p>
            <button
              type='button'
              data-testid='dsh-agent-retry'
              className='rounded border border-line-border px-3 py-1.5 text-xs text-text-primary hover:bg-fill-secondary'
              onClick={retry}
            >
              Retry
            </button>
          </div>
        ) : null}
        <iframe
          key={iframeKey}
          ref={iframeRef}
          data-testid='dsh-agent-iframe'
          title='DeepSeek Harness'
          src={frameUrl}
          onLoad={() =>
            setAttachment((prev) => ({ ...prev, frameLoaded: true }))
          }
          className='h-full w-full border-0 bg-background-primary'
          allow='clipboard-read; clipboard-write'
        />
      </>
    ) : attachment.stage === 'Failed' ? (
      <div className='flex h-full flex-col items-center justify-center gap-3 p-6 text-center' data-testid='dsh-agent-error'>
        <p className='text-sm text-text-primary'>Agent is unavailable</p>
        <p className='max-w-xs whitespace-pre-line text-xs text-text-secondary'>
          {attachment.error?.message || pendingCopy}
        </p>
        <button
          type='button'
          data-testid='dsh-agent-retry'
          className='rounded border border-line-border px-3 py-1.5 text-xs text-text-primary hover:bg-fill-secondary'
          onClick={retry}
        >
          Retry
        </button>
      </div>
    ) : (
      <div className='flex h-full items-center justify-center p-6 text-xs text-text-secondary' data-testid='dsh-agent-pending'>
        {pendingCopy}
      </div>
    );

  return (
    <div
      data-testid='dsh-agent-panel'
      data-open={open ? 'true' : 'false'}
      data-attachment-stage={attachment.stage}
      data-error-code={attachment.error?.code ?? ''}
      data-attachment-id={attachment.attachmentId}
      className='fixed right-0 top-0 z-[90] h-screen bg-background-primary'
      style={{ width: open ? width : 0, overflow: 'hidden' }}
      aria-hidden={!open}
    >
      <div
        style={{ width }}
        className='relative flex h-full flex-col overflow-hidden border-l border-line-border shadow-sm'
      >
        <div className='flex h-12 shrink-0 items-center justify-between gap-2 border-b border-line-border px-3'>
          <div className='flex min-w-0 items-center gap-2'>
            <AiChat className='h-4 w-4 shrink-0 text-text-secondary' />
            <div className='min-w-0'>
              <div className='truncate text-sm font-medium text-text-primary'>DeepSeek Agent</div>
              <div className='truncate text-xs text-text-secondary'>{title}</div>
            </div>
          </div>
          <div className='flex shrink-0 items-center gap-1'>
            <button
              type='button'
              data-testid='dsh-agent-copy-id'
              title={attachment.attachmentId}
              className='max-w-[7rem] truncate px-1 text-[10px] text-text-secondary hover:text-text-primary'
              onClick={() => {
                void navigator.clipboard?.writeText(attachment.attachmentId);
              }}
            >
              {attachment.attachmentId}
            </button>
            <Tooltip title='Reload iframe'>
              <IconButton size='small' onClick={retry} data-testid='dsh-agent-reload'>
                <span className='px-1 text-xs text-text-secondary'>Reload</span>
              </IconButton>
            </Tooltip>
            <Tooltip title='Open in new tab'>
              <IconButton
                size='small'
                onClick={() => {
                  if (canMountDshIframe(frameUrl)) window.open(frameUrl, '_blank', 'noopener,noreferrer');
                }}
                aria-label='Open DeepSeek Agent in new tab'
              >
                <span className='px-1 text-xs text-text-secondary'>↗</span>
              </IconButton>
            </Tooltip>
            <Tooltip title='Close'>
              <IconButton size='small' onClick={() => setOpen(false)} aria-label='Close DeepSeek Agent panel'>
                <Close className='h-4 w-4 text-text-secondary' />
              </IconButton>
            </Tooltip>
          </div>
        </div>

        <div className='relative min-h-0 flex-1 bg-background-primary'>{content}</div>

        {open ? (
          <Resizer
            drawerWidth={width}
            onResize={setWidth}
            minWidth={320}
            maxWidth={typeof window !== 'undefined' ? Math.max(480, window.innerWidth * 0.6) : 800}
          />
        ) : null}
      </div>
    </div>
  );
}

export default React.memo(DshAgentPanel);
