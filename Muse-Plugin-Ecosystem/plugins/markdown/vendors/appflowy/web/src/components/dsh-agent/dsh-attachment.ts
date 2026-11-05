export type AttachmentStage =
  | 'Closed'
  | 'ResolvingIdentity'
  | 'Placing'
  | 'Queued'
  | 'Presenting'
  | 'Binding'
  | 'HybridLive'
  | 'Failed';

export type AttachmentErrorCode =
  | 'NEED_AUTH'
  | 'NEED_WORKSPACE'
  | 'NEED_API_KEY'
  | 'SESSION_DENIED'
  | 'SCOPE_MISMATCH'
  | 'POOL_UNAVAILABLE'
  | 'QUEUED'
  | 'COLD_START'
  | 'SIDECAR_EXIT'
  | 'INGRESS_DENIED'
  | 'FRAME_TIMEOUT'
  | 'BRIDGE_SILENT'
  | 'NO_DEVICE_TOKEN'
  | 'BIND_REJECTED'
  | 'FLAG_OFF';

export type AttachmentError = {
  stage: AttachmentStage;
  code: AttachmentErrorCode;
  message: string;
  retryable: boolean;
};

const MESSAGES: Record<AttachmentErrorCode, string> = {
  NEED_AUTH: 'Sign in is required to start the agent.',
  NEED_WORKSPACE: 'Open a workspace, then start the agent.',
  NEED_API_KEY: 'Enter the model API key in the DeepSeek panel.',
  SESSION_DENIED: 'The agent instance did not start. Retry, or wait a minute if this is the first open today.',
  SCOPE_MISMATCH: 'You are not a member of this workspace.',
  POOL_UNAVAILABLE: 'The agent control plane is unavailable. Retry in a moment.',
  QUEUED: 'Waiting in line. The first start can take a minute.',
  COLD_START: 'Starting agent… first open can take a minute.',
  SIDECAR_EXIT: 'The local agent process exited. Retry to start it again.',
  INGRESS_DENIED: 'The agent page was blocked (sign-in cookie missing or expired).',
  FRAME_TIMEOUT: 'The agent page did not become ready in time.',
  BRIDGE_SILENT: 'The agent UI loaded but the host bridge did not respond.',
  NO_DEVICE_TOKEN: 'Device credentials are missing. Retry to issue them.',
  BIND_REJECTED: 'The agent could not bind this workspace.',
  FLAG_OFF: 'Workspace linking is turned off on the agent.',
};

const RETRYABLE: Record<AttachmentErrorCode, boolean> = {
  NEED_AUTH: false,
  NEED_WORKSPACE: false,
  NEED_API_KEY: false,
  SESSION_DENIED: true,
  SCOPE_MISMATCH: false,
  POOL_UNAVAILABLE: true,
  QUEUED: true,
  COLD_START: true,
  SIDECAR_EXIT: true,
  INGRESS_DENIED: true,
  FRAME_TIMEOUT: true,
  BRIDGE_SILENT: true,
  NO_DEVICE_TOKEN: true,
  BIND_REJECTED: false,
  FLAG_OFF: false,
};

export function attachmentError(
  stage: AttachmentStage,
  code: AttachmentErrorCode,
  message?: string
): AttachmentError {
  return {
    stage,
    code,
    message: message?.trim() || MESSAGES[code],
    retryable: RETRYABLE[code],
  };
}

export function newAttachmentId(): string {
  const uuid =
    typeof globalThis.crypto !== 'undefined' && typeof globalThis.crypto.randomUUID === 'function'
      ? globalThis.crypto.randomUUID()
      : `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
  return `att_${uuid}`;
}

export type AgentAttachment = {
  attachmentId: string;
  generation: number;
  stage: AttachmentStage;
  frameUrl: string;
  sessionRef: string | null;
  queuePosition?: number;
  retryAfterMs?: number;
  error?: AttachmentError;
  frameLoaded: boolean;
};

export function createAttachment(attachmentId = newAttachmentId()): AgentAttachment {
  return {
    attachmentId,
    generation: 0,
    stage: 'Closed',
    frameUrl: '',
    sessionRef: null,
    frameLoaded: false,
  };
}

export function bumpGeneration(state: AgentAttachment): AgentAttachment {
  return { ...state, generation: state.generation + 1 };
}

export function isLiveGeneration(state: AgentAttachment, generation: number): boolean {
  return state.generation === generation;
}

export function queuedCopy(queuePosition?: number): string {
  if (typeof queuePosition === 'number' && queuePosition > 0) {
    return `Waiting in line (position ${queuePosition}). The first start can take a minute.`;
  }
  return MESSAGES.QUEUED;
}

export function placingCopy(): string {
  return MESSAGES.COLD_START;
}

/** Presenting / Binding / HybridLive require a mountable frame URL (invariant A1). */
export function stageRequiresFrame(stage: AttachmentStage): boolean {
  return stage === 'Presenting' || stage === 'Binding' || stage === 'HybridLive';
}

/** Binding/bridge failures keep the DSH shell mounted (E1-T9 / E1-P3). */
export const KEEP_FRAME_ERROR_CODES: ReadonlySet<AttachmentErrorCode> = new Set([
  'NO_DEVICE_TOKEN',
  'BIND_REJECTED',
  'BRIDGE_SILENT',
  'FLAG_OFF',
  'FRAME_TIMEOUT',
  'INGRESS_DENIED',
]);

export function keepFrameOnFailed(code: AttachmentErrorCode | undefined): boolean {
  return !!code && KEEP_FRAME_ERROR_CODES.has(code);
}

/** Host waits this long after hello/bind for `bridge.reply` (E1-T9). */
export const BIND_ACK_TIMEOUT_MS = 8000;

export const ATTACHMENT_ERROR_CODES: AttachmentErrorCode[] = Object.keys(MESSAGES) as AttachmentErrorCode[];
