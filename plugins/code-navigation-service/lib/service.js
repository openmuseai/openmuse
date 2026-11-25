import { performance } from 'node:perf_hooks';
import { randomUUID } from 'node:crypto';

export class NavigationError extends Error {
  constructor(code) { super(code); this.code = code; }
}
export const fail = (code) => { throw new NavigationError(code); };
const operations = new Set(['definition', 'references', 'implementation', 'hover']);
const defaults = Object.freeze({
  maxInstances: 2, maxPending: 8, idleMs: 300_000,
  queryTimeoutMs: 5_000, startupTimeoutMs: 15_000,
  maxDocumentBytes: 1_048_576, maxResults: 100,
});

export function checkAbort(signal) {
  if (signal?.aborted) throw signal.reason instanceof NavigationError
    ? signal.reason : new NavigationError('CANCELLED');
}
export function abortable(promise, signal) {
  if (!signal) return promise;
  if (signal.aborted) return Promise.reject(signal.reason instanceof NavigationError
    ? signal.reason : new NavigationError('CANCELLED'));
  return new Promise((resolve, reject) => {
    const abort = () => reject(signal.reason instanceof NavigationError
      ? signal.reason : new NavigationError('CANCELLED'));
    signal.addEventListener('abort', abort, { once: true });
    Promise.resolve(promise).then(resolve, reject).finally(() => signal.removeEventListener('abort', abort));
  });
}

/** No process, timer, filesystem access or network work is performed by construction
 * or enable(). The resource owner and backend factory are installed by a trusted
 * Host adapter, never supplied in a remote navigation request. */
export class CodeNavigationService {
  #enabled = false;
  #epoch = 0;
  #entries = new Map();
  #requests = new Set();
  #stopping;
  constructor({ owner, backendFactory, limits = {}, trace = () => {} }) {
    this.owner = owner;
    this.backendFactory = backendFactory;
    this.trace = trace;
    this.limits = Object.freeze({ ...defaults, ...limits });
    for (const [key, value] of Object.entries(this.limits)) {
      if (!(key in defaults) || !Number.isSafeInteger(value) || value < 1) fail('INVALID_POLICY');
    }
  }
  get status() {
    return { enabled: this.#enabled, stopping: !!this.#stopping,
      instances: this.#entries.size, pending: this.#requests.size };
  }
  enable() {
    if (this.#stopping) fail('STOPPING');
    this.#enabled = true;
  }
  async disable() { this.#enabled = false; await this.stopAll(); }
  async stopAll() {
    if (this.#stopping) return this.#stopping;
    this.#epoch++;
    for (const request of this.#requests) request.abort(new NavigationError('DISABLED'));
    this.#stopping = (async () => {
      const results = await Promise.allSettled([...this.#entries.values()].map(e => this.#stop(e)));
      if (results.some(r => r.status === 'rejected')) fail('STOP_FAILED');
    })();
    try { await this.#stopping; } finally { this.#stopping = undefined; }
  }
  async #stop(entry) {
    if (entry.stopping) return entry.stopping;
    clearTimeout(entry.timer);
    entry.stopping = (async () => {
      await entry.backend?.stop();
      if (this.#entries.get(entry.key) === entry) this.#entries.delete(entry.key);
    })();
    try { await entry.stopping; } catch (error) {
      entry.stopping = undefined; // Keep a failed instance visible and retryable.
      throw error;
    }
  }
  #idle(entry) {
    if (entry.users || entry.stopping || this.#entries.get(entry.key) !== entry) return;
    entry.timer = setTimeout(() => {
      this.#stop(entry).catch(() => this.#emit({ stage: 'stop', status: 'STOP_FAILED' }));
    }, this.limits.idleMs);
    entry.timer.unref?.();
  }
  #emit(event) {
    try { this.trace({ v: 1, ...event }); } catch { /* Diagnostics cannot break navigation. */ }
  }
  async query(input, { access, signal } = {}) {
    if (!this.#enabled) fail('DISABLED');
    if (this.#stopping) fail('STOPPING');
    if (!input || !operations.has(input.operation) ||
        typeof input.version !== 'string' || !input.version ||
        !Number.isSafeInteger(input.position?.line) || input.position.line < 0 ||
        !Number.isSafeInteger(input.position?.character) || input.position.character < 0) fail('INVALID_QUERY');
    checkAbort(signal);
    if (this.#requests.size >= this.limits.maxPending) fail('BUSY');
    const controller = new AbortController();
    this.#requests.add(controller);
    const abort = () => controller.abort(new NavigationError('CANCELLED'));
    signal?.addEventListener('abort', abort, { once: true });
    const epoch = this.#epoch;
    const traceId = randomUUID();
    const started = performance.now();
    let entry, timer, task, status = 'ok', cold = false;
    const arm = ms => {
      clearTimeout(timer);
      timer = setTimeout(() => controller.abort(new NavigationError('TIMEOUT')), Math.max(1, ms - (performance.now() - started)));
    };
    arm(this.limits.startupTimeoutMs);
    try {
      const document = await abortable(this.owner.authorizeAndRead(input, access, controller.signal), controller.signal);
      checkAbort(controller.signal);
      if (epoch !== this.#epoch || !this.#enabled) fail('DISABLED');
      if (document.version !== input.version) fail('VERSION_MISMATCH');
      if (typeof document.text !== 'string' || Buffer.byteLength(document.text) > this.limits.maxDocumentBytes) fail('DOCUMENT_TOO_LARGE');
      const line = document.text.split('\n')[input.position.line];
      if (line === undefined || input.position.character > line.length) fail('INVALID_POSITION');
      for (const key of ['partition', 'workspaceKey', 'profile', 'rootUri', 'uri', 'languageId']) {
        if (typeof document[key] !== 'string' || !document[key]) fail('INVALID_DOCUMENT');
      }
      const key = JSON.stringify([document.partition, document.workspaceKey, document.profile]);
      entry = this.#entries.get(key);
      if (entry?.stopping) fail('STOPPING');
      if (!entry) {
        if (this.#entries.size >= this.limits.maxInstances) {
          const idle = [...this.#entries.values()].find(e => !e.users && !e.stopping);
          if (!idle) fail('BUSY');
          await this.#stop(idle);
          checkAbort(controller.signal);
          // Another asynchronous admission may have acquired the released slot.
          if (this.#entries.size >= this.limits.maxInstances) fail('BUSY');
        }
        if (epoch !== this.#epoch) fail('DISABLED');
        if (this.#entries.has(key)) fail('BUSY');
        // Factory only creates a handle. start() is the sole process-spawn path.
        entry = { key, backend: this.backendFactory(document), tail: Promise.resolve(), users: 0, ready: false };
        this.#entries.set(key, entry);
      }
      cold = !entry.ready;
      arm(cold ? this.limits.startupTimeoutMs : this.limits.queryTimeoutMs);
      clearTimeout(entry.timer);
      entry.users++;
      const queued = performance.now();
      const work = entry.tail.then(async () => {
        checkAbort(controller.signal);
        if (epoch !== this.#epoch || entry.stopping) fail('DISABLED');
        this.#emit({ traceId, stage: 'queue', elapsedMs: performance.now() - queued });
        try {
          if (!entry.ready) {
            const begin = performance.now();
            await abortable(entry.backend.start(document, controller.signal), controller.signal);
            entry.ready = true;
            this.#emit({ traceId, stage: 'initialize', elapsedMs: performance.now() - begin });
          }
          await abortable(this.owner.assertCurrent(document, access, controller.signal), controller.signal);
          const begin = performance.now();
          const result = await abortable(entry.backend.query(document, input, controller.signal), controller.signal);
          this.#emit({ traceId, stage: 'query', elapsedMs: performance.now() - begin });
          await abortable(this.owner.assertCurrent(document, access, controller.signal), controller.signal);
          checkAbort(controller.signal);
          if (input.operation === 'hover') return { kind: 'hover', hover: result.hover, basis: 'semantic' };
          if (!Array.isArray(result.locations)) fail('MALFORMED_RESPONSE');
          const matches = [];
          for (const location of result.locations.slice(0, this.limits.maxResults)) {
            matches.push(await abortable(this.owner.resolveLocation(location, document, access, controller.signal), controller.signal));
          }
          await abortable(this.owner.assertCurrent(document, access, controller.signal), controller.signal);
          checkAbort(controller.signal);
          return { kind: 'locations', matches, truncated: result.locations.length > this.limits.maxResults, basis: 'semantic' };
        } catch (error) {
          // Reset transports on faults/cancellation; do not leave a blocked queue.
          await this.#stop(entry);
          throw error;
        }
      });
      task = work.finally(() => { entry.users--; this.#idle(entry); });
      entry.tail = task.catch(() => {});
      // Queued cancellation returns immediately, but keeps its admission slot
      // until the bounded queue drains; repeated cancellations cannot grow it.
      return await abortable(task, controller.signal);
    } catch (error) {
      status = controller.signal.aborted ? controller.signal.reason.code : (error.code ?? 'FAILED');
      if (controller.signal.aborted) throw controller.signal.reason;
      throw error;
    } finally {
      clearTimeout(timer);
      if (task) task.finally(() => this.#requests.delete(controller)).catch(() => {});
      else this.#requests.delete(controller);
      signal?.removeEventListener('abort', abort);
      this.#emit({ traceId, stage: 'total', status, cold, elapsedMs: performance.now() - started,
        instances: this.#entries.size });
    }
  }
}

/** Independent service-plugin lifecycle; settings persistence and remote access
 * remain with the resource Host. Import/activate never starts a language process. */
export function createCodeNavigationPlugin(options) {
  const service = new CodeNavigationService(options);
  return { id: 'com.openmuse.code-navigation', service,
    activate: async () => {}, deactivate: () => service.disable() };
}
