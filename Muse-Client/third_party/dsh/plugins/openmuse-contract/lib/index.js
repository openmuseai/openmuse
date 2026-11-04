/** Provider-neutral OpenMuse contract v1 parser used at TypeScript/DSH boundaries. */

export const protocolName = 'openmuse.contract';
export const protocolMajor = 1;
export const protocolMinor = 0;

export const contractErrorCodes = Object.freeze([
  'DENIED',
  'NOT_FOUND',
  'CONFLICT',
  'EXPIRED',
  'STALE_GENERATION',
  'UNAVAILABLE',
  'TRANSIENT',
  'INTEGRITY_FAILED',
]);

const principalKinds = new Set(['user', 'agent', 'plugin', 'service', 'device']);
const receiptStates = new Set(['committed', 'rejected', 'cancelled', 'expired', 'failed']);
const handleStates = new Set(['active', 'revoked', 'expired']);
const leaseStates = new Set(['active', 'released', 'revoked', 'expired']);
const maxSafeInteger = Number.MAX_SAFE_INTEGER;

export class OpenMuseContractError extends TypeError {}

export function parseContractEnvelope(input, options = {}) {
  const value = object(input, 'envelope');
  if (value.kind === 'request') return parseRequest(value, options);
  if (value.kind === 'response') return parseResponse(value, options);
  throw new OpenMuseContractError('kind must be request or response');
}

export function parseLifecycleSnapshot(input) {
  const value = object(input, 'lifecycle');
  keys(value, ['protocol', 'descriptor', 'handle', 'lease', 'receipt']);
  return {
    protocol: protocol(value.protocol),
    descriptor: descriptor(value.descriptor),
    handle: handle(value.handle),
    lease: lease(value.lease),
    receipt: receipt(value.receipt),
  };
}

function parseRequest(value, { expectedGeneration } = {}) {
  keys(value, [
    'protocol', 'kind', 'requestId', 'actor', 'caller', 'scope', 'generation',
    'deadlineAtMs', 'cancellationRef', 'operation', 'payload',
  ]);
  const generation = safeInteger(value.generation, 'generation', 1);
  checkGeneration(generation, expectedGeneration);
  return {
    protocol: protocol(value.protocol),
    kind: 'request',
    requestId: opaqueRef(value.requestId, 'requestId'),
    actor: principal(value.actor, 'actor'),
    caller: principal(value.caller, 'caller'),
    scope: scope(value.scope),
    generation,
    deadlineAtMs: safeInteger(value.deadlineAtMs, 'deadlineAtMs', 1),
    cancellationRef: opaqueRef(value.cancellationRef, 'cancellationRef'),
    operation: opaqueRef(value.operation, 'operation'),
    payload: clone(value.payload),
  };
}

function parseResponse(value, { expectedGeneration } = {}) {
  keys(value, [
    'protocol', 'kind', 'requestId', 'actor', 'caller', 'scope', 'generation',
    'deadlineAtMs', 'cancellationRef', 'outcome',
  ]);
  const requestId = opaqueRef(value.requestId, 'requestId');
  const generation = safeInteger(value.generation, 'generation', 1);
  checkGeneration(generation, expectedGeneration);
  const parsedOutcome = outcome(value.outcome);
  if (parsedOutcome.receipt.requestId !== requestId || parsedOutcome.receipt.generation !== generation) {
    throw new OpenMuseContractError('receipt does not match its envelope');
  }
  if (parsedOutcome.status === 'ok' && parsedOutcome.receipt.state !== 'committed') {
    throw new OpenMuseContractError('ok receipt must be committed');
  }
  if (parsedOutcome.status === 'error' && parsedOutcome.receipt.state === 'committed') {
    throw new OpenMuseContractError('error receipt cannot be committed');
  }
  return {
    protocol: protocol(value.protocol),
    kind: 'response',
    requestId,
    actor: principal(value.actor, 'actor'),
    caller: principal(value.caller, 'caller'),
    scope: scope(value.scope),
    generation,
    deadlineAtMs: safeInteger(value.deadlineAtMs, 'deadlineAtMs', 1),
    cancellationRef: opaqueRef(value.cancellationRef, 'cancellationRef'),
    outcome: parsedOutcome,
  };
}

export function ensureLiveAt(request, nowMs) {
  const now = safeInteger(nowMs, 'nowMs', 0);
  if (now >= request.deadlineAtMs) {
    throw new OpenMuseContractError(`deadline ${request.deadlineAtMs} has expired at ${now}`);
  }
}

function protocol(input) {
  const value = object(input, 'protocol');
  keys(value, ['name', 'major', 'minor']);
  if (value.name !== protocolName) throw new OpenMuseContractError('unsupported protocol name');
  const major = safeInteger(value.major, 'protocol.major', 1);
  const minor = safeInteger(value.minor, 'protocol.minor', 0);
  if (major !== protocolMajor || minor > protocolMinor) {
    throw new OpenMuseContractError(`incompatible protocol version ${major}.${minor}`);
  }
  return { name: protocolName, major, minor };
}

function principal(input, field) {
  const value = object(input, field);
  keys(value, ['principalRef', 'kind']);
  const kind = string(value.kind, `${field}.kind`);
  if (!principalKinds.has(kind)) throw new OpenMuseContractError(`${field}.kind has unsupported value ${kind}`);
  return { principalRef: opaqueRef(value.principalRef, `${field}.principalRef`), kind };
}

function scope(input) {
  const value = object(input, 'scope');
  keys(value, ['authorityRef'], ['workspaceRef', 'resourceRef']);
  return {
    authorityRef: opaqueRef(value.authorityRef, 'scope.authorityRef'),
    ...(value.workspaceRef === undefined ? {} : { workspaceRef: opaqueRef(value.workspaceRef, 'scope.workspaceRef') }),
    ...(value.resourceRef === undefined ? {} : { resourceRef: opaqueRef(value.resourceRef, 'scope.resourceRef') }),
  };
}

function outcome(input) {
  const value = object(input, 'outcome');
  if (value.status === 'ok') {
    keys(value, ['status', 'receipt', 'value']);
    return { status: 'ok', receipt: receipt(value.receipt), value: clone(value.value) };
  }
  if (value.status === 'error') {
    keys(value, ['status', 'receipt', 'error']);
    return { status: 'error', receipt: receipt(value.receipt), error: contractError(value.error) };
  }
  throw new OpenMuseContractError('unsupported outcome status');
}

function contractError(input) {
  const value = object(input, 'error');
  keys(value, ['code', 'message', 'retryable', 'details']);
  const code = string(value.code, 'error.code');
  if (!contractErrorCodes.includes(code)) throw new OpenMuseContractError(`unsupported error code ${code}`);
  if (typeof value.retryable !== 'boolean') throw new OpenMuseContractError('error.retryable must be a boolean');
  return {
    code,
    message: string(value.message, 'error.message'),
    retryable: value.retryable,
    details: clone(object(value.details, 'error.details')),
  };
}

function receipt(input) {
  const value = object(input, 'receipt');
  keys(value, ['receiptRef', 'requestId', 'generation', 'state', 'issuedAtMs', 'effects']);
  const state = enumString(value.state, 'receipt.state', receiptStates);
  return {
    receiptRef: opaqueRef(value.receiptRef, 'receipt.receiptRef'),
    requestId: opaqueRef(value.requestId, 'receipt.requestId'),
    generation: safeInteger(value.generation, 'receipt.generation', 1),
    state,
    issuedAtMs: safeInteger(value.issuedAtMs, 'receipt.issuedAtMs', 1),
    effects: stringArray(value.effects, 'receipt.effects', true),
  };
}

function descriptor(input) {
  const value = object(input, 'descriptor');
  keys(value, ['descriptorRef', 'generation', 'revision', 'issuedAtMs', 'value']);
  return {
    descriptorRef: opaqueRef(value.descriptorRef, 'descriptor.descriptorRef'),
    generation: safeInteger(value.generation, 'descriptor.generation', 1),
    revision: opaqueRef(value.revision, 'descriptor.revision'),
    issuedAtMs: safeInteger(value.issuedAtMs, 'descriptor.issuedAtMs', 1),
    value: clone(value.value),
  };
}

function handle(input) {
  const value = object(input, 'handle');
  keys(value, ['handleRef', 'audience', 'scope', 'access', 'generation', 'issuedAtMs', 'expiresAtMs', 'state']);
  const issuedAtMs = safeInteger(value.issuedAtMs, 'handle.issuedAtMs', 1);
  const expiresAtMs = safeInteger(value.expiresAtMs, 'handle.expiresAtMs', 1);
  interval(issuedAtMs, expiresAtMs, 'handle');
  const access = stringArray(value.access, 'handle.access', false);
  if (access.length === 0) throw new OpenMuseContractError('handle.access must not be empty');
  return {
    handleRef: opaqueRef(value.handleRef, 'handle.handleRef'),
    audience: principal(value.audience, 'handle.audience'),
    scope: scope(value.scope),
    access,
    generation: safeInteger(value.generation, 'handle.generation', 1),
    issuedAtMs,
    expiresAtMs,
    state: enumString(value.state, 'handle.state', handleStates),
  };
}

function lease(input) {
  const value = object(input, 'lease');
  keys(value, ['leaseRef', 'holder', 'scope', 'generation', 'issuedAtMs', 'expiresAtMs', 'state']);
  const issuedAtMs = safeInteger(value.issuedAtMs, 'lease.issuedAtMs', 1);
  const expiresAtMs = safeInteger(value.expiresAtMs, 'lease.expiresAtMs', 1);
  interval(issuedAtMs, expiresAtMs, 'lease');
  return {
    leaseRef: opaqueRef(value.leaseRef, 'lease.leaseRef'),
    holder: principal(value.holder, 'lease.holder'),
    scope: scope(value.scope),
    generation: safeInteger(value.generation, 'lease.generation', 1),
    issuedAtMs,
    expiresAtMs,
    state: enumString(value.state, 'lease.state', leaseStates),
  };
}

function checkGeneration(actual, expected) {
  if (expected === undefined) return;
  const checked = safeInteger(expected, 'expectedGeneration', 1);
  if (actual !== checked) throw new OpenMuseContractError(`stale generation: expected ${checked}, got ${actual}`);
}

function interval(issuedAtMs, expiresAtMs, field) {
  if (expiresAtMs <= issuedAtMs) throw new OpenMuseContractError(`${field}.expiresAtMs must be greater than issuedAtMs`);
}

function object(value, field) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    throw new OpenMuseContractError(`${field} must be an object`);
  }
  return value;
}

function keys(value, required, optional = []) {
  const allowed = new Set([...required, ...optional]);
  for (const field of required) {
    if (!Object.hasOwn(value, field)) throw new OpenMuseContractError(`missing field: ${field}`);
  }
  for (const field of Object.keys(value)) {
    if (!allowed.has(field)) throw new OpenMuseContractError(`unknown field: ${field}`);
  }
}

function string(value, field) {
  if (typeof value !== 'string' || value.length === 0) throw new OpenMuseContractError(`${field} must be a non-empty string`);
  return value;
}

function opaqueRef(value, field) {
  const checked = string(value, field);
  if (/\s/u.test(checked)) throw new OpenMuseContractError(`${field} must be an opaque reference`);
  return checked;
}

function enumString(value, field, allowed) {
  const checked = string(value, field);
  if (!allowed.has(checked)) throw new OpenMuseContractError(`${field} has unsupported value ${checked}`);
  return checked;
}

function safeInteger(value, field, minimum) {
  if (!Number.isSafeInteger(value) || value < minimum || value > maxSafeInteger) {
    throw new OpenMuseContractError(`${field} must be a safe integer >= ${minimum}`);
  }
  return value;
}

function stringArray(value, field, emptyAllowed) {
  if (!Array.isArray(value)) throw new OpenMuseContractError(`${field} must be an array`);
  const checked = value.map((item) => opaqueRef(item, field));
  if (!emptyAllowed && checked.length === 0) throw new OpenMuseContractError(`${field} must not be empty`);
  if (new Set(checked).size !== checked.length) throw new OpenMuseContractError(`${field} must contain unique values`);
  return checked;
}

function clone(value) {
  if (Array.isArray(value)) return value.map(clone);
  if (value !== null && typeof value === 'object') {
    return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, clone(item)]));
  }
  return value;
}
