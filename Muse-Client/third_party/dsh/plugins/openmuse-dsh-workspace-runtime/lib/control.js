const CONTROL_SCHEMA = 'openmuse.workspace-runtime.control@1';

export class RemoteControlError extends Error {
  constructor(message, code = 'REMOTE_CONTROL_ERROR', options) {
    super(message, options);
    this.name = 'RemoteControlError';
    this.code = code;
  }
}

function requiredString(value, name) {
  if (typeof value !== 'string' || value.length === 0) {
    throw new RemoteControlError(`${name} must be a non-empty string`, 'INVALID_ATTACHMENT');
  }
  return value;
}

export function validateAttachment(value, expectedAudience, nowMs = Date.now()) {
  if (!value || typeof value !== 'object') {
    throw new RemoteControlError('control attachment is required', 'INVALID_ATTACHMENT');
  }
  const attachment = Object.freeze({
    tokenRef: requiredString(value.tokenRef, 'tokenRef'),
    runtimeRef: requiredString(value.runtimeRef, 'runtimeRef'),
    audience: requiredString(value.audience, 'audience'),
    generation: value.generation,
    expiresAtMs: value.expiresAtMs,
  });
  if (!Number.isSafeInteger(attachment.generation) || attachment.generation <= 0) {
    throw new RemoteControlError('generation must be a positive safe integer', 'INVALID_ATTACHMENT');
  }
  if (!Number.isSafeInteger(attachment.expiresAtMs) || attachment.expiresAtMs <= nowMs) {
    throw new RemoteControlError('control attachment has expired', 'ATTACHMENT_EXPIRED');
  }
  if (attachment.audience !== expectedAudience) {
    throw new RemoteControlError('control attachment audience mismatch', 'ATTACHMENT_AUDIENCE_MISMATCH');
  }
  return attachment;
}

export class WorkspaceRuntimeControl {
  constructor({ transport, attachment, audience, now = Date.now }) {
    if (!transport || typeof transport.call !== 'function') {
      throw new TypeError('workspace runtime transport.call is required');
    }
    if (typeof audience !== 'string' || audience.length === 0) {
      throw new TypeError('workspace runtime audience is required');
    }
    this.transport = transport;
    this.audience = audience;
    this.now = now;
    this.attachment = validateAttachment(attachment, audience, now());
  }

  envelope(method, params = {}) {
    validateAttachment(this.attachment, this.audience, this.now());
    return {
      schema: CONTROL_SCHEMA,
      method,
      attachment: this.attachment,
      params,
    };
  }

  call(method, params, signal) {
    signal?.throwIfAborted();
    return this.transport.call(this.envelope(method, params), { signal });
  }

  stream(method, params, signal) {
    if (typeof this.transport.stream !== 'function') {
      throw new RemoteControlError('transport does not implement response streaming', 'CAPABILITY_UNAVAILABLE');
    }
    signal?.throwIfAborted();
    return this.transport.stream(this.envelope(method, params), { signal });
  }

  openProcess(method, params) {
    if (typeof this.transport.openProcess !== 'function') {
      throw new RemoteControlError('transport does not implement process streaming', 'CAPABILITY_UNAVAILABLE');
    }
    return this.transport.openProcess(this.envelope(method, params));
  }

  watch(params, changed, signal) {
    if (typeof this.transport.watch !== 'function') {
      throw new RemoteControlError('transport does not implement filesystem watching', 'CAPABILITY_UNAVAILABLE');
    }
    signal.throwIfAborted();
    return this.transport.watch(this.envelope('fs.watch', params), changed, signal);
  }
}

export function resolveControl(config) {
  if (config?.control instanceof WorkspaceRuntimeControl) return config.control;
  return new WorkspaceRuntimeControl(config ?? {});
}

export { CONTROL_SCHEMA };
