/**
 * dsh-model-capabilities — host half.
 *
 * Same-origin HTTP bridge for the browser half (the pattern the
 * dsh-font-settings bundle uses): a third-party bundle cannot rely on the
 * generated `ctx.remote.settings` wire, so this Host face owns
 * read-modify-write against the `llm-pi-ai` settings namespace and exposes two
 * small JSON routes:
 *
 *   GET  /model-capabilities?provider=<route>  → view of one provider's models,
 *        capability fields, request headers and baseURL, with the namespace
 *        revision for fencing.
 *   POST /model-capabilities                   → merge capability edits into
 *        the provider's models / reasoning / defaultInput / compat / headers
 *        and persist through settings.mutate (revision-fenced, pi-ai
 *        schema-validated).
 *
 * opencode Go session affinity (https://opencode.ai/docs/go/): the gateway
 * asks tools to include `x-opencode-session` so it can route per session and
 * optimize prompt caching. This Host face resolves that header per DSH
 * session at the wire layer, through two complementary paths:
 *
 * 1. Placeholder (the user's own custom header is the control point): a
 *    header value containing `{{session}}` — e.g. the user creates
 *    `x-opencode-session: {{session}}` in the card's request-header editor —
 *    is substituted per request with the current session's stable opaque
 *    token (SHA-256 of the session id, `dsh-` + 12 base36 chars). Works on
 *    any host; a static value is sent as-is and always wins over the
 *    automatic stamp.
 * 2. Automatic stamp: when the user set no `x-opencode-session` and the
 *    provider's baseURL host matches the affinity host list (default
 *    `opencode.ai`), the same token is stamped automatically.
 *
 * Both paths draw the token from one `llm/stream` waterfall listener
 * (registered `global` — scoped cross-plugin listeners are dropped by the
 * LlmRuntime context filter) that reads the request's `sessionId` and hands a
 * per-host FIFO ledger to a single `globalThis.fetch` wrapper (the waterfall
 * callback and the stream's single wire fetch pair up one-to-one). The token
 * is stable across the turns of one session and differs across sessions.
 * Leftover literal placeholders (non-session contexts) are dropped, never
 * sent. The harness attribution User-Agent (`deepseek-harness/…`) already
 * satisfies the "properly identifies itself" requirement and stays reserved —
 * user-set headers may not override it.
 *
 * Data owner remains the pi-ai adapter's `llm-pi-ai` namespace; nothing is
 * owned here. Both handlers run in the Host realm, so ordinary plain objects
 * pass the settings service's prototype checks.
 */
import { createHash } from 'node:crypto';

/** No hard service dependencies: webServer and settings are both optional reads. */
export const inject = [];

const NS = 'llm-pi-ai';
const LEVELS = ['off', 'minimal', 'low', 'medium', 'high', 'xhigh', 'max'];
const MODALITIES = ['text', 'image'];
const THINKING_FORMATS = ['openai', 'deepseek', 'openrouter', 'together', 'baseten', 'zai', 'qwen', 'chat-template', 'qwen-chat-template', 'string-thinking', 'ant-ling'];
const MAX_TOKEN_FIELDS = ['max_completion_tokens', 'max_tokens'];
const COMPAT_BOOL_KEYS = ['supportsStore', 'supportsDeveloperRole', 'supportsReasoningEffort', 'supportsUsageInStreaming', 'supportsFinishReason', 'requiresToolResultName', 'requiresAssistantAfterToolResult', 'requiresThinkingAsText', 'requiresReasoningContentOnAssistantMessages', 'supportsThinkingTokenBudget', 'supportsStrictMode', 'supportsLongCacheRetention', 'supportsEagerToolInputStreaming', 'supportsCacheControlOnTools', 'supportsTemperature', 'forceAdaptiveThinking', 'allowEmptySignature', 'supportsStrictTools'];
/** Headers the harness attribution owns; the pi-ai adapter drops them from profile headers. */
const RESERVED_HEADERS = new Set(['user-agent']);
/** Wire header carrying opencode's session-affinity token. */
const AFFINITY_HEADER = 'x-opencode-session';
/** Value placeholder the user writes in a custom header; the wire layer substitutes
 * the current DSH session's stable token (SHA-256 of the session id) per request. */
const SESSION_PLACEHOLDER = '{{session}}';
/** baseURL host suffixes that receive the per-session affinity header by default. */
const DEFAULT_AFFINITY_HOSTS = ['opencode.ai'];
/** RFC 9110 field-name token characters (lowercased before the test). */
const HEADER_NAME_RE = /^[a-z0-9!#$%&'*+\-.^_`|~]+$/;
const MAX_HEADER_VALUE_CHARS = 512;
const MAX_BODY_BYTES = 1024 * 1024;

function readBody(req) {
  return new Promise((resolve, reject) => {
    let data = '';
    let settled = false;
    const fail = (error) => {
      if (settled) return;
      settled = true;
      req.destroy();
      reject(error);
    };
    req.on('data', (chunk) => {
      data += chunk;
      if (data.length > MAX_BODY_BYTES) fail(new Error('request body too large'));
    });
    req.on('end', () => {
      if (settled) return;
      settled = true;
      resolve(data);
    });
    req.on('error', (error) => fail(error));
  });
}

function sendJson(res, status, payload) {
  const body = JSON.stringify(payload);
  res.writeHead(status, {
    'content-type': 'application/json; charset=utf-8',
    'cache-control': 'no-store',
  });
  res.end(body);
}

function cleanModalities(value) {
  return Array.isArray(value) ? value.filter((m) => MODALITIES.includes(m)) : null;
}

function cleanEfforts(value) {
  if (value === false) return false;
  if (typeof value !== 'object' || value === null) return null;
  const next = {};
  for (const level of LEVELS) if (value[level] !== undefined) next[level] = value[level];
  return Object.keys(next).length === 0 ? null : next;
}

function cleanCompat(value) {
  if (typeof value !== 'object' || value === null) return null;
  const next = {};
  for (const key of COMPAT_BOOL_KEYS) if (typeof value[key] === 'boolean') next[key] = value[key];
  if (THINKING_FORMATS.includes(value.thinkingFormat)) next.thinkingFormat = value.thinkingFormat;
  if (MAX_TOKEN_FIELDS.includes(value.maxTokensField)) next.maxTokensField = value.maxTokensField;
  return Object.keys(next).length === 0 ? null : next;
}

/** The profile's request-header dict, names lowercased for case-insensitive views. */
function cleanHeaders(value) {
  if (typeof value !== 'object' || value === null) return null;
  const next = {};
  for (const [name, headerValue] of Object.entries(value)) {
    if (typeof headerValue !== 'string') continue;
    next[name.toLowerCase()] = headerValue;
  }
  return Object.keys(next).length === 0 ? null : next;
}

/* ── per-session affinity (wire layer) ─────────────────────────────────────── */

/** Default opencode hosts plus extra host suffixes from the bundle row config. */
function affinityHostsOf(config) {
  const hosts = [...DEFAULT_AFFINITY_HOSTS];
  if (Array.isArray(config?.hosts)) {
    for (const host of config.hosts) {
      if (typeof host === 'string' && host.trim() !== '') hosts.push(host.trim().toLowerCase());
    }
  }
  return hosts;
}

function baseURLHost(baseURL) {
  if (typeof baseURL !== 'string' || baseURL === '') return null;
  try {
    return new URL(baseURL).hostname.toLowerCase();
  } catch {
    return null;
  }
}

/** Exact host or subdomain match against the affinity host suffixes. */
function hostMatches(host, hosts) {
  return hosts.some((candidate) => host === candidate || host.endsWith(`.${candidate}`));
}

/** Stable opaque per-session token: `dsh-` + 12 base36 chars of a domain-separated digest. */
function affinityToken(sessionId) {
  const digest = createHash('sha256').update(`dsh-session-affinity:v1:${sessionId}`).digest();
  const alphabet = 'abcdefghijklmnopqrstuvwxyz0123456789';
  let token = 'dsh-';
  for (let at = 0; at < 12; at += 1) token += alphabet[digest[at] % alphabet.length];
  return token;
}

/** One stored provider profile (plain object) by route, or null. */
function profileFor(settings, route) {
  if (settings === undefined || typeof route !== 'string') return null;
  const section = settings.get(NS);
  if (typeof section !== 'object' || section === null) return null;
  const profile = section.providers?.[route];
  return typeof profile === 'object' && profile !== null ? profile : null;
}

/** Whether this profile's baseURL host receives the per-session header. */
function affinityForProfile(profile, hosts, disabled) {
  if (disabled) return null;
  const host = baseURLHost(profile?.baseURL);
  return host !== null && hostMatches(host, hosts) ? 'per-session' : null;
}

/** Whether the profile's own request headers carry the {{session}} placeholder. */
function headersWantSession(profile) {
  const headers = profile?.headers;
  if (typeof headers !== 'object' || headers === null) return false;
  return Object.values(headers).some((value) => typeof value === 'string' && value.includes(SESSION_PLACEHOLDER));
}

/**
 * Per-stream affinity handoff. The `llm/stream` waterfall listener and the
 * wire fetch pair up one-to-one per model call (the pi-ai adapter pins
 * `maxRetries: 0`, so each stream performs exactly one fetch), and the
 * waterfall callback runs synchronously before the stream's fetch starts.
 * AsyncLocalStorage was tried first and does NOT survive the pi-ai adapter's
 * internal await chain, so the handoff is a per-host FIFO queue of tokens the
 * listener enqueues synchronously and the fetch wrapper dequeues; a
 * last-used token per host keeps subsequent same-session calls (title
 * generation, auxiliary calls) on the same bucket even when they bypass the
 * waterfall. Under genuinely concurrent same-host streams from different
 * sessions the FIFO can momentarily swap tokens; the very next call
 * self-corrects, and the gateway contract (header present, value stable per
 * session) still holds.
 */
function createAffinityLedger() {
  /** host → FIFO of tokens awaiting their stream's fetch */
  const pending = new Map();
  /** host → most recently used token (fallback when the queue is empty) */
  const lastUsed = new Map();
  return {
    enqueue(host, token) {
      const queue = pending.get(host) ?? [];
      queue.push(token);
      pending.set(host, queue);
      lastUsed.set(host, token);
    },
    take(host) {
      const queue = pending.get(host);
      const token = queue !== undefined && queue.length > 0 ? queue.shift() : lastUsed.get(host);
      if (token !== undefined) lastUsed.set(host, token);
      return token;
    },
    clear() {
      pending.clear();
      lastUsed.clear();
    },
  };
}

/** Hostname of a fetch input (string URL, URL, or Request), or null. */
function fetchInputHost(input) {
  try {
    if (typeof input === 'string') return new URL(input).hostname.toLowerCase();
    if (input instanceof URL) return input.hostname.toLowerCase();
    if (typeof input === 'object' && input !== null && typeof input.url === 'string') return new URL(input.url).hostname.toLowerCase();
  } catch {
    // Malformed input: fall through to null.
  }
  return null;
}

/**
 * Wrap globalThis.fetch once. Requests to affinity hosts get the next
 * enqueued session token stamped on them; every other outbound request —
 * web searches, discovery, other hosts — passes through untouched.
 */
function installSessionFetch(ledger, hosts) {
  const target = globalThis;
  const original = target.fetch;
  if (typeof original !== 'function') return () => {};
  const wrapped = async function fetch(input, init) {
    const host = fetchInputHost(input);
    if (host === null) return original(input, init);
    const token = ledger.take(host);
    const affinity = hostMatches(host, hosts);
    if (token === undefined && !affinity) return original(input, init);
    let headers;
    try {
      headers = new Headers(init === undefined || init === null ? undefined : init.headers);
    } catch {
      return original(input, init);
    }
    // The user's own x-opencode-session (stored profile headers reach the wire
    // through pi-ai's optionsHeaders merge) is the source of truth: a
    // {{session}} placeholder is substituted per request, a static value is
    // sent as-is. Only when the user set no such header does the automatic
    // per-session stamp apply (and only on affinity hosts).
    const userSession = headers.get(AFFINITY_HEADER);
    if (token !== undefined) {
      for (const [name, value] of [...headers.entries()]) {
        if (value.includes(SESSION_PLACEHOLDER)) headers.set(name, value.replaceAll(SESSION_PLACEHOLDER, token));
      }
      if (userSession === null && affinity) headers.set(AFFINITY_HEADER, token);
    }
    // Never leak the literal placeholder to a gateway: when no session token
    // is available (non-session contexts like model discovery), the marked
    // header is dropped instead of sent with the raw marker.
    const leftovers = [];
    for (const [name, value] of headers.entries()) {
      if (value.includes(SESSION_PLACEHOLDER)) leftovers.push(name);
    }
    for (const name of leftovers) headers.delete(name);
    return original(input, { ...(init ?? {}), headers });
  };
  target.fetch = wrapped;
  return () => {
    if (target.fetch === wrapped) target.fetch = original;
  };
}

/** One provider's capability view + the namespace revision the view was read at. */
function viewOf(profile, revision, sessionAffinity) {
  const out = {
    ok: true,
    revision,
    hasModelsList: Array.isArray(profile.models),
    reasoning: typeof profile.reasoning === 'string' && LEVELS.includes(profile.reasoning) ? profile.reasoning : null,
    defaultInput: cleanModalities(profile.defaultInput),
    compat: cleanCompat(profile.compat),
    baseURL: typeof profile.baseURL === 'string' ? profile.baseURL : null,
    headers: cleanHeaders(profile.headers),
    sessionAffinity: sessionAffinity ?? null,
    models: [],
  };
  if (Array.isArray(profile.models)) {
    out.models = profile.models.map((model) => ({
      id: String(model?.id ?? ''),
      name: typeof model?.name === 'string' ? model.name : null,
      contextWindow: typeof model?.contextWindow === 'number' ? model.contextWindow : null,
      maxTokens: typeof model?.maxTokens === 'number' ? model.maxTokens : null,
      input: cleanModalities(model?.input),
      reasoningEfforts: cleanEfforts(model?.reasoningEfforts),
      compat: model?.compat && typeof model.compat === 'object' ? model.compat : null,
    }));
  }
  return out;
}

export function apply(ctx, config) {
  const hosts = affinityHostsOf(config);
  const affinityDisabled = config?.disableSessionAffinity === true;
  /** Per-stream affinity tokens handed from the waterfall listener to the fetch wrapper. */
  const ledger = createAffinityLedger();
  ctx.effect(() => {
    const uninstall = installSessionFetch(ledger, hosts);
    return () => {
      ledger.clear();
      uninstall();
    };
  }, 'dsh-model-capabilities: session fetch wrapper');
  // Cross-plugin listeners on a service waterfall must register globally:
  // the LlmRuntime's context filter drops scoped listeners from other plugins.
  ctx.on('llm/stream', (options, next) => {
    try {
      if (!affinityDisabled && options !== null && typeof options === 'object' && typeof options.provider === 'string') {
        const profile = profileFor(ctx.get('settings'), options.provider);
        const host = baseURLHost(profile?.baseURL);
        // A stream needs its session token when its host is an affinity host
        // (automatic stamp) OR the user's own headers carry a {{session}}
        // placeholder (explicit opt-in, works on any host).
        if (host !== null && (affinityForProfile(profile, hosts, false) === 'per-session' || headersWantSession(profile)) && typeof options.sessionId === 'string' && options.sessionId !== '') {
          ledger.enqueue(host, affinityToken(options.sessionId));
        }
      }
    } catch {
      // Any lookup failure must never block the stream; fall through unwrapped.
    }
    return next();
  }, { global: true });
  // The HTTP bridge only exists in web profiles, and webServer may activate
  // after this plugin, so register on first sight instead of a hard inject —
  // a declared inject would keep the whole tree pending in headless/tui
  // profiles that never provide the service (the injection chain needs none
  // of it there).
  let bridgeRegistered = false;
  const registerBridge = (webServer) => {
    if (bridgeRegistered || webServer === undefined || webServer === null || typeof webServer.register !== 'function') return;
    bridgeRegistered = true;
    ctx.effect(() => webServer.register({
      kind: 'prefix',
      path: '/model-capabilities',
    handler: async (req, res) => {
      const url = new URL(req.url ?? '/', 'http://localhost');
      const settings = ctx.get('settings');
      if (settings === undefined) {
        return sendJson(res, 503, { ok: false, error: 'settings service unavailable' });
      }
      try {
        if (req.method === 'GET' && url.pathname === '/model-capabilities') {
          const provider = url.searchParams.get('provider') ?? '';
          if (provider === '') return sendJson(res, 400, { ok: false, error: 'missing provider' });
          let revision = 0;
          for (const descriptor of settings.describe()) if (descriptor.ns === NS) revision = Number(descriptor.revision) || 0;
          const section = settings.get(NS);
          const profile = typeof section === 'object' && section !== null ? section.providers?.[provider] : undefined;
          if (typeof profile !== 'object' || profile === null) return sendJson(res, 404, { ok: false, error: 'provider-not-found' });
          return sendJson(res, 200, viewOf(profile, revision, affinityForProfile(profile, hosts, affinityDisabled)));
        }
        if (req.method === 'POST' && url.pathname === '/model-capabilities') {
          let payload;
          try {
            payload = JSON.parse(await readBody(req));
          } catch {
            return sendJson(res, 400, { ok: false, error: 'invalid JSON body' });
          }
          const provider = typeof payload?.provider === 'string' ? payload.provider : '';
          if (provider === '') return sendJson(res, 400, { ok: false, error: 'missing provider' });
          const expectedRevision = typeof payload?.revision === 'number' ? payload.revision : undefined;
          const section = settings.get(NS);
          const profile = typeof section === 'object' && section !== null ? section.providers?.[provider] : undefined;
          if (typeof profile !== 'object' || profile === null) return sendJson(res, 404, { ok: false, error: 'provider-not-found' });
          if (!Array.isArray(profile.models)) return sendJson(res, 409, { ok: false, error: 'profile-has-no-models-list' });

          const incoming = Array.isArray(payload.models) ? payload.models : [];
          const byId = new Map(incoming.map((entry) => [String(entry?.id ?? ''), entry]));
          const nextModels = profile.models.map((model) => {
            const edit = byId.get(String(model?.id ?? ''));
            if (edit === undefined) return model;
            const next = { ...model };
            if (edit.input === null || edit.input === undefined) delete next.input;
            else if (Array.isArray(edit.input)) next.input = edit.input.filter((m) => MODALITIES.includes(m));
            if (edit.reasoningEfforts === null || edit.reasoningEfforts === undefined) delete next.reasoningEfforts;
            else if (edit.reasoningEfforts === false) next.reasoningEfforts = false;
            else if (typeof edit.reasoningEfforts === 'object' && edit.reasoningEfforts !== null) {
              const efforts = cleanEfforts(edit.reasoningEfforts);
              if (efforts === null) delete next.reasoningEfforts;
              else next.reasoningEfforts = efforts;
            }
            return next;
          });

          const ops = [{ op: 'set', path: ['providers', provider, 'models'], value: nextModels }];
          const route = ['providers', provider];
          if (payload.reasoning === null || payload.reasoning === undefined) ops.push({ op: 'unset', path: [...route, 'reasoning'] });
          else if (LEVELS.includes(payload.reasoning)) ops.push({ op: 'set', path: [...route, 'reasoning'], value: payload.reasoning });
          if (payload.defaultInput === null || payload.defaultInput === undefined) ops.push({ op: 'unset', path: [...route, 'defaultInput'] });
          else if (Array.isArray(payload.defaultInput)) ops.push({ op: 'set', path: [...route, 'defaultInput'], value: payload.defaultInput.filter((m) => MODALITIES.includes(m)) });

          // Headers use full-replace semantics: the client sends the desired
          // final dict (`headers: {}` clears every header); with the key absent
          // the stored dict is left untouched. Names merge case-insensitively
          // (lowercased). pi-ai's namespace validator (assertValidHeaders)
          // stays the final authority — its rejection surfaces verbatim
          // through the mutate-failure path.
          if (Object.prototype.hasOwnProperty.call(payload, 'headers')) {
            const incomingHeaders = payload.headers;
            if (incomingHeaders !== null && typeof incomingHeaders !== 'object') {
              return sendJson(res, 400, { ok: false, error: 'headers must be an object of name → value ({} or null clears all)' });
            }
            const mergedHeaders = {};
            for (const [rawName, rawValue] of Object.entries(incomingHeaders ?? {})) {
              const name = String(rawName).trim().toLowerCase();
              const value = typeof rawValue === 'string' ? rawValue.trim() : '';
              if (name === '') return sendJson(res, 400, { ok: false, error: 'header name is empty' });
              if (!HEADER_NAME_RE.test(name)) return sendJson(res, 400, { ok: false, error: `invalid header name: ${name}` });
              if (RESERVED_HEADERS.has(name)) return sendJson(res, 400, { ok: false, error: `header "${name}" is owned by the harness attribution User-Agent and cannot be set` });
              if (value === '' || value.length > MAX_HEADER_VALUE_CHARS || /[\r\n\u0000]/.test(value)) {
                return sendJson(res, 400, { ok: false, error: `invalid value for header "${name}" (empty, over ${MAX_HEADER_VALUE_CHARS} chars, or contains control characters)` });
              }
              mergedHeaders[name] = value;
            }
            if (Object.keys(mergedHeaders).length === 0) ops.push({ op: 'unset', path: [...route, 'headers'] });
            else ops.push({ op: 'set', path: [...route, 'headers'], value: mergedHeaders });
          }

          const patch = typeof payload.compatPatch === 'object' && payload.compatPatch !== null ? payload.compatPatch : {};
          const currentCompat = typeof profile.compat === 'object' && profile.compat !== null ? { ...profile.compat } : {};
          for (const key of ['supportsDeveloperRole', 'supportsReasoningEffort']) {
            if (patch[key] === true || patch[key] === false) currentCompat[key] = patch[key];
            else if (patch[key] === 'unset') delete currentCompat[key];
          }
          for (const key of ['maxTokensField', 'thinkingFormat']) {
            if (MAX_TOKEN_FIELDS.includes(patch[key]) || THINKING_FORMATS.includes(patch[key])) currentCompat[key] = patch[key];
            else if (patch[key] === 'unset') delete currentCompat[key];
          }
          if (Object.keys(currentCompat).length === 0) ops.push({ op: 'unset', path: [...route, 'compat'] });
          else ops.push({ op: 'set', path: [...route, 'compat'], value: currentCompat });

          try {
            await settings.mutate(NS, ops, expectedRevision);
            return sendJson(res, 200, { ok: true });
          } catch (error) {
            if (error?.code === 'SETTINGS_CONFLICT') return sendJson(res, 409, { ok: false, error: 'conflict' });
            return sendJson(res, 500, { ok: false, error: String(error?.message ?? error) });
          }
        }
        return sendJson(res, 404, { ok: false, error: 'not found' });
      } catch (error) {
        return sendJson(res, 500, { ok: false, error: String(error?.message ?? error) });
      }
    },
    }), 'dsh-model-capabilities: routes');
  };
  registerBridge(ctx.get('webServer'));
  ctx.on('internal/service', (name, impl) => {
    try {
      if (name === 'webServer') registerBridge(impl);
    } catch {
      // Service-notification failures must never break the provider.
    }
  }, { global: true });
}
