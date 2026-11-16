import { spawn, execFile } from 'node:child_process';
import { once } from 'node:events';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';
import { createMessageConnection, StreamMessageReader, StreamMessageWriter, CancellationTokenSource } from 'vscode-jsonrpc/node.js';
import { abortable, checkAbort, fail } from './service.js';

const methods = {
  definition: ['textDocument/definition', 'definitionProvider'],
  references: ['textDocument/references', 'referencesProvider'],
  implementation: ['textDocument/implementation', 'implementationProvider'],
  hover: ['textDocument/hover', 'hoverProvider'],
};
const execute = promisify(execFile);
async function groupAlive(pid) {
  // Query only process-group/state columns, never another app's arguments.
  const { stdout } = await execute('ps', ['-axo', 'pgid=,stat=']);
  return stdout.split('\n').some(line => {
    const [group, state] = line.trim().split(/\s+/);
    return Number(group) === pid && state && !state.startsWith('Z');
  });
}
async function within(promise, ms) {
  let timer;
  try { return await Promise.race([promise, new Promise(resolve => { timer = setTimeout(resolve, ms); })]); }
  finally { clearTimeout(timer); }
}
function validRange(range) {
  if (!range) fail('MALFORMED_RESPONSE');
  for (const p of [range.start, range.end]) {
    if (!Number.isSafeInteger(p?.line) || p.line < 0 ||
        !Number.isSafeInteger(p?.character) || p.character < 0) fail('MALFORMED_RESPONSE');
  }
  if (range.end.line < range.start.line ||
      range.end.line === range.start.line && range.end.character < range.start.character) fail('MALFORMED_RESPONSE');
  return range;
}
export function normalizeLocations(value) {
  if (value == null) return [];
  return (Array.isArray(value) ? value : [value]).map(item => {
    if (!item || typeof item !== 'object') fail('MALFORMED_RESPONSE');
    const uri = item.targetUri ?? item.uri;
    if (typeof uri !== 'string' || uri.length > 8192) fail('MALFORMED_RESPONSE');
    const range = validRange(item.targetRange ?? item.range);
    const selectionRange = validRange(item.targetSelectionRange ?? range);
    const compare = (a, b) => a.line - b.line || a.character - b.character;
    if (compare(selectionRange.start, range.start) < 0 || compare(selectionRange.end, range.end) > 0) fail('MALFORMED_RESPONSE');
    return { uri, range, selectionRange };
  });
}

/** Host-registered commands only; each backend owns exactly one process group.
 * No remote request may override profile, command, args, env, or cwd. */
export function stdioBackendFactory(profiles, { shutdownMs = 2000, killMs = 1000 } = {}) {
  const registered = structuredClone(profiles);
  return document => {
    const profile = registered[document.profile];
    if (!profile || typeof profile.command !== 'string' || !profile.command) fail('LANGUAGE_UNCONFIGURED');
    return new StdioBackend(profile, { shutdownMs, killMs });
  };
}

class StdioBackend {
  #child;
  #connection;
  #capabilities;
  #stopping;
  #exited;
  #dead = false;
  constructor(profile, policy) { this.profile = profile; this.policy = policy; }
  async start(document, signal) {
    checkAbort(signal);
    // An adapter for another execution world must install another backend.
    if (!document.rootUri.startsWith('file:')) fail('UNSUPPORTED_EXECUTION_WORLD');
    const env = {};
    for (const name of ['PATH', 'HOME', 'USERPROFILE', 'SystemRoot', 'TMPDIR', 'TEMP', 'LANG', 'LC_ALL']) {
      if (process.env[name]) env[name] = process.env[name];
    }
    Object.assign(env, this.profile.env ?? {});
    const child = this.#child = spawn(this.profile.command, this.profile.args ?? [], {
      cwd: fileURLToPath(document.rootUri), env, shell: false,
      detached: process.platform !== 'win32', stdio: ['pipe', 'pipe', 'pipe'], windowsHide: true,
    });
    // Never forward language stderr into product trace (may contain source/paths).
    child.stderr.resume();
    this.#exited = new Promise(resolve => {
      child.once('exit', () => { this.#dead = true; resolve(); });
      child.once('error', () => { this.#dead = true; resolve(); });
    });
    await abortable(once(child, 'spawn'), signal);
    checkAbort(signal);
    const connection = this.#connection = createMessageConnection(
      new StreamMessageReader(child.stdout), new StreamMessageWriter(child.stdin));
    connection.onRequest('workspace/configuration', params =>
      (params.items ?? []).map(item => {
        let value = this.profile.configuration ?? {};
        for (const part of (item.section ?? '').split('.').filter(Boolean)) {
          if (!Object.hasOwn(value ?? {}, part)) return null;
          value = value[part];
        }
        return value;
      }));
    connection.onRequest('workspace/applyEdit', () => ({ applied: false, failureReason: 'Read-only navigation' }));
    connection.onRequest('workspace/workspaceFolders', () => [{ uri: document.rootUri, name: 'Workspace' }]);
    connection.onRequest('client/registerCapability', () => null);
    connection.onRequest('client/unregisterCapability', () => null);
    connection.onRequest('window/workDoneProgress/create', () => null);
    connection.listen();
    const initialized = await this.#request('initialize', {
      processId: null, rootUri: document.rootUri,
      workspaceFolders: [{ uri: document.rootUri, name: 'Workspace' }],
      capabilities: {
        general: { positionEncodings: ['utf-16'] },
        textDocument: { definition: { linkSupport: true }, implementation: { linkSupport: true } },
        workspace: { configuration: true, workspaceFolders: true },
      },
      initializationOptions: this.profile.initializationOptions ?? null,
    }, signal);
    this.#capabilities = initialized?.capabilities ?? {};
    if ((this.#capabilities.positionEncoding ?? 'utf-16') !== 'utf-16') fail('UNSUPPORTED_POSITION_ENCODING');
    const sync = this.#capabilities.textDocumentSync;
    if (!(typeof sync === 'number' ? sync > 0 : sync?.openClose === true)) fail('UNSUPPORTED_DOCUMENT_SYNC');
    await connection.sendNotification('initialized', {});
  }
  async #request(method, params, signal) {
    checkAbort(signal);
    const token = new CancellationTokenSource();
    const abort = () => token.cancel();
    signal?.addEventListener('abort', abort, { once: true });
    try { return await abortable(this.#connection.sendRequest(method, params, token.token), signal); }
    finally { signal?.removeEventListener('abort', abort); token.dispose(); }
  }
  async query(document, input, signal) {
    checkAbort(signal);
    const [method, capability] = methods[input.operation] ?? [];
    if (!capability || !this.#capabilities[capability]) fail('UNSUPPORTED_OPERATION');
    await this.#connection.sendNotification('textDocument/didOpen', { textDocument: {
      uri: document.uri, languageId: document.languageId, version: 1, text: document.text,
    } });
    try {
      const value = await this.#request(method, {
        textDocument: { uri: document.uri }, position: input.position,
        ...(input.operation === 'references' ? { context: { includeDeclaration: true } } : {}),
      }, signal);
      if (input.operation === 'hover') return { hover: value == null ? null : {
        contents: value.contents, ...(value.range ? { range: validRange(value.range) } : {}),
      } };
      return { locations: normalizeLocations(value) };
    } finally {
      if (!this.#dead && !signal.aborted) await this.#connection.sendNotification('textDocument/didClose', { textDocument: { uri: document.uri } });
    }
  }
  async stop() {
    if (this.#stopping) return this.#stopping;
    this.#stopping = (async () => {
      const child = this.#child;
      if (!child) return;
      if (!this.#dead && this.#connection) {
        await within(
          (async () => {
            await this.#connection.sendRequest('shutdown');
            await this.#connection.sendNotification('exit');
            await this.#exited;
          })().catch(() => {}), this.policy.shutdownMs);
      }
      // Kill the owned tree even when its leader exited gracefully: a language
      // server can leave worker children in its detached process group.
      if (child.pid) {
        if (process.platform === 'win32') {
          if (!this.#dead) await new Promise((resolve, reject) => {
            const killer = spawn('taskkill', ['/pid', String(child.pid), '/T', '/F'], { windowsHide: true });
            killer.once('error', reject);
            killer.once('exit', code => code === 0 ? resolve() : reject(new Error('STOP_FAILED')));
          });
        } else {
          // On macOS, killing an already emptied process group may report
          // EPERM instead of ESRCH. Establish liveness before sending a signal;
          // a real permission failure on a live group must still fail stop.
          if (await groupAlive(child.pid)) {
            try { process.kill(-child.pid, 'SIGKILL'); }
            catch (error) { if (error.code !== 'ESRCH') throw error; }
          }
        }
      }
      await within(this.#exited, this.policy.killMs);
      if (process.platform !== 'win32' && child.pid) {
        const deadline = Date.now() + this.policy.killMs;
        while (await groupAlive(child.pid)) {
          if (Date.now() >= deadline) fail('STOP_FAILED');
          await new Promise(resolve => setTimeout(resolve, 10));
        }
      }
      this.#connection?.dispose();
      if (!this.#dead) fail('STOP_FAILED');
    })();
    try { await this.#stopping; }
    catch (error) { this.#stopping = undefined; throw error; }
  }
}
