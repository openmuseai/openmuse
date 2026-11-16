import { readFile, writeFile, mkdir, rename, rm, cp, realpath, lstat, mkdtemp, readdir } from 'node:fs/promises';
import { join, basename, isAbsolute, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { createHash, randomUUID } from 'node:crypto';
import { performance } from 'node:perf_hooks';

export const SERVICE = 'com.openmuse.code-navigation';
export const DART = 'com.openmuse.code-navigation.dart';
export const TYPESCRIPT = 'com.openmuse.code-navigation.typescript';
export const catalog = Object.freeze([
  { id: SERVICE, name: '代码导航服务', version: '0.1.1', dependencies: [], description: '按需启动语言服务；默认关闭，空闲自动释放。' },
  { id: DART, name: 'Dart 语言扩展', version: '0.1.0', dependencies: [SERVICE], description: '连接这台宿主已有的 Dart SDK，支持定义、引用和实现。' },
  { id: TYPESCRIPT, name: 'TypeScript / JavaScript 扩展', version: '0.1.0', dependencies: [SERVICE], description: '内含固定 TypeScript 5.9.3 与语言服务 6.1.0，离线安装。' },
].map(x => Object.freeze({ ...x, ecosystem: 'code-navigation', package: x.id, categories: ['编程'], target: 'resource-host', restartRequired: false })));
const fail = code => { throw Object.assign(new Error(code), { code }); };
const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const object = x => x != null && typeof x === 'object' && !Array.isArray(x);
const clone = x => JSON.parse(JSON.stringify(x));
const defaults = id => id === SERVICE ? { idleSeconds: 300, maxInstances: 2 } : { sdkPath: '', initializationOptions: {}, configuration: {} };

function configFor(id, input) {
  if (!object(input) || Buffer.byteLength(JSON.stringify(input)) > 49152) fail('INVALID_CONFIGURATION');
  const allowed = id === SERVICE ? ['idleSeconds', 'maxInstances'] : ['sdkPath', 'initializationOptions', 'configuration'];
  if (Object.keys(input).some(k => !allowed.includes(k))) fail('INVALID_CONFIGURATION');
  const value = { ...defaults(id), ...input };
  if (id === SERVICE) {
    if (!Number.isInteger(value.idleSeconds) || value.idleSeconds < 30 || value.idleSeconds > 3600 || !Number.isInteger(value.maxInstances) || value.maxInstances < 1 || value.maxInstances > 4) fail('INVALID_CONFIGURATION');
  } else {
    if (typeof value.sdkPath !== 'string' || value.sdkPath.length > 4096 || /[\r\n\0]/.test(value.sdkPath) || value.sdkPath && !isAbsolute(value.sdkPath)) fail('INVALID_CONFIGURATION');
    if (!object(value.initializationOptions) || !object(value.configuration)) fail('INVALID_CONFIGURATION');
    if (id === TYPESCRIPT && value.sdkPath) fail('INVALID_CONFIGURATION');
  }
  return clone(value);
}

/** Trusted host constructor supplies distribution and account roots. None are RPC fields.
 * The signed application owns index.json; each install verifies its full file closure. */
export class NavigationPluginManager {
  #tail = Promise.resolve();
  #runtime;
  #creating;
  #profileCache;
  #state;
  #loaded;
  #probes = new Set();
  #probeServices = new Set();
  constructor({ directory, artifactsRoot, partition }) {
    if (!partition || partition.length > 256) fail('DENIED');
    this.directory = join(directory, 'accounts', sha(partition));
    this.artifactsRoot = artifactsRoot;
    this.#loaded = this.#load();
  }
  async #load() {
    try { this.#state = JSON.parse(await readFile(join(this.directory, 'state.json'), 'utf8')); }
    catch (e) { if (e.code !== 'ENOENT') throw e; this.#state = { revision: 0, plugins: {}, workspaces: {} }; }
    if (!object(this.#state.plugins) || !object(this.#state.workspaces) || !Number.isSafeInteger(this.#state.revision)) fail('INVALID_POLICY');
  }
  async #save(state) {
    await mkdir(this.directory, { recursive: true, mode: 0o700 });
    const tmp = join(this.directory, `state-${randomUUID()}.tmp`);
    await writeFile(tmp, JSON.stringify(state), { mode: 0o600 });
    await rename(tmp, join(this.directory, 'state.json'));
    this.#state = state;
  }
  #exclusive(action) {
    const pending = this.#tail.catch(() => {}).then(async () => { await this.#loaded; return action(); });
    this.#tail = pending; return pending;
  }
  #entry(id) { const entry = catalog.find(x => x.id === id); if (!entry) fail('PLUGIN_NOT_FOUND'); return entry; }
  async plan(id) {
    await this.#loaded; await this.#tail.catch(() => {});
    const entry = this.#entry(id), installed = this.#state.plugins[id];
    return { ...entry, installedVersion: installed?.version ?? null, enabled: installed?.enabled === true,
      configuration: clone(installed?.configuration ?? defaults(id)), revision: this.#state.revision,
      detected: installed?.detected === true, instances: this.instances,
      action: !installed ? 'install' : installed.enabled ? 'already-installed' : 'enable' };
  }
  async #verify(id, root) {
    const index = JSON.parse(await readFile(join(this.artifactsRoot, 'index.json'), 'utf8'));
    const expected = index[id];
    if (!expected || !object(expected.files) || Object.keys(expected.files).length > 1000) fail('ARTIFACT_UNAVAILABLE');
    const actual = [];
    async function visit(directory, prefix = '') {
      for (const entry of await readdir(directory, { withFileTypes: true })) {
        const name = prefix + entry.name;
        if (entry.isSymbolicLink()) fail('ARTIFACT_INVALID');
        if (entry.isDirectory()) await visit(join(directory, entry.name), name + '/');
        else if (entry.isFile()) actual.push(name);
        else fail('ARTIFACT_INVALID');
      }
    }
    await visit(root);
    if (actual.length !== Object.keys(expected.files).length || actual.some(name => !(name in expected.files))) fail('ARTIFACT_INVALID');
    for (const [name, digest] of Object.entries(expected.files)) {
      if (!name || isAbsolute(name) || name.split(/[\\/]/).some(x => x === '..' || x === '.') || !/^[a-f0-9]{64}$/.test(digest)) fail('ARTIFACT_INVALID');
      const path = join(root, name);
      if (!(await lstat(path)).isFile() || sha(await readFile(path)) !== digest) fail('ARTIFACT_DIGEST_MISMATCH');
    }
    const manifest = JSON.parse(await readFile(join(root, 'plugin.json'), 'utf8'));
    if (manifest.id !== id || manifest.version !== this.#entry(id).version) fail('ARTIFACT_INVALID');
  }
  async #stop() {
    for (const controller of this.#probes) controller.abort();
    if (this.#creating) await this.#creating.catch(() => {});
    const services = [...this.#probeServices, ...(this.#runtime ? [this.#runtime] : [])];
    const results = await Promise.allSettled(services.map(service => service.disable()));
    for (let i = 0; i < results.length; i++) if (results[i].status === 'fulfilled') this.#probeServices.delete(services[i]);
    if (results.some(result => result.status === 'rejected')) fail('STOP_FAILED');
    this.#runtime = undefined;
  }
  async stop() {
    for (const controller of this.#probes) controller.abort();
    return this.#exclusive(() => this.#stop());
  }
  async manage(operation, id, { values, expectedRevision } = {}) {
    if (operation === 'list') return { plugins: catalog };
    if (operation === 'plan') return this.plan(id);
    if (operation === 'detect') return this.#exclusive(() => this.#detect(id));
    if (['disable', 'uninstall', 'configure'].includes(operation)) for (const controller of this.#probes) controller.abort();
    await this.#exclusive(async () => {
      this.#entry(id);
      if (!['install', 'configure', 'enable', 'disable', 'uninstall'].includes(operation)) fail('INVALID_OPERATION');
      const state = clone(this.#state), installed = state.plugins[id];
      if (operation === 'install') {
        if (installed) return;
        if (this.#entry(id).dependencies.some(dep => !state.plugins[dep])) fail('DEPENDENCY_REQUIRED');
        const source = join(this.artifactsRoot, id);
        await this.#verify(id, source);
        await mkdir(join(this.directory, 'plugins'), { recursive: true, mode: 0o700 });
        const temp = join(this.directory, 'plugins', `${id}-${randomUUID()}.tmp`);
        const destination = join(this.directory, 'plugins', id);
        try {
          await cp(source, temp, { recursive: true, dereference: false });
          await this.#verify(id, temp);
          await rm(destination, { recursive: true, force: true });
          await rename(temp, destination);
        } finally { await rm(temp, { recursive: true, force: true }); }
        state.plugins[id] = { version: this.#entry(id).version, enabled: false, configuration: defaults(id), detected: id === SERVICE };
      } else {
        if (!installed) fail('PLUGIN_NOT_INSTALLED');
        if (operation === 'configure') {
          if (expectedRevision !== state.revision) fail('CONFIGURATION_CONFLICT');
          installed.configuration = configFor(id, values); installed.detected = id === SERVICE; installed.enabled = false;
        }
        if (operation === 'enable') {
          if (!installed.detected) fail('DETECTION_REQUIRED');
          if (id !== SERVICE && !state.plugins[SERVICE]?.enabled) fail('SERVICE_DISABLED');
          installed.enabled = true;
        }
        if (operation === 'disable') installed.enabled = false;
        if (operation === 'uninstall') {
          if (catalog.some(x => x.dependencies.includes(id) && state.plugins[x.id])) fail('DEPENDENTS_INSTALLED');
          delete state.plugins[id];
          if (id === SERVICE) state.workspaces = {};
        }
        await this.#stop(); // Failure leaves the receipt unchanged, never claims released.
      }
      state.revision++;
      await this.#save(state);
      if (operation === 'uninstall') await rm(join(this.directory, 'plugins', id), { recursive: true, force: true });
    });
    return this.plan(id);
  }
  async profiles({ includeDisabled = false } = {}) {
    await this.#loaded;
    const cacheKey = `${this.revision}:${includeDisabled}`;
    if (this.#profileCache?.key === cacheKey) return this.#profileCache.value;
    const profiles = {};
    for (const [id, entry] of Object.entries(this.#state.plugins)) {
      if (id === SERVICE || !includeDisabled && (!this.#state.plugins[SERVICE]?.enabled || !entry.enabled || !entry.detected)) continue;
      const config = configFor(id, entry.configuration), root = join(this.directory, 'plugins', id);
      await this.#verify(id, root);
      if (id === DART) {
        if (!config.sdkPath || !['dart', 'dart.exe'].includes(basename(config.sdkPath))) continue;
        const command = await realpath(config.sdkPath);
        profiles.dart = { command, args: ['language-server', '--protocol=lsp'], extensions: ['.dart'], languageId: 'dart', initializationOptions: config.initializationOptions, configuration: config.configuration };
      }
      if (id === TYPESCRIPT) {
        const init = { ...config.initializationOptions, disableAutomaticTypingAcquisition: true, tsserver: { useSyntaxServer: 'never', path: join(root, 'node_modules/typescript/lib/tsserver.js') } };
        for (const [language, extensions] of [['typescript', ['.ts']], ['typescriptreact', ['.tsx']], ['javascript', ['.js', '.mjs', '.cjs']], ['javascriptreact', ['.jsx']]]) profiles[language] = { command: process.execPath, args: [join(root, 'node_modules/typescript-language-server/lib/cli.mjs'), '--stdio'], extensions, languageId: language, initializationOptions: init, configuration: config.configuration };
      }
    }
    this.#profileCache = { key: cacheKey, value: profiles };
    return profiles;
  }
  async #modules() {
    if (!this.#state.plugins[SERVICE]) fail('PLUGIN_NOT_INSTALLED');
    const root = join(this.directory, 'plugins', SERVICE);
    await this.#verify(SERVICE, root);
    const core = await import(pathToFileURL(join(root, 'lib/service.js')).href);
    const backend = await import(pathToFileURL(join(root, 'lib/stdio.js')).href);
    return { ...core, ...backend };
  }
  async runtime(owner) {
    await this.#loaded;
    if (!this.#state.plugins[SERVICE]?.enabled) fail('SERVICE_DISABLED');
    if (this.#creating) return this.#creating;
    if (!this.#runtime) {
      this.#creating = (async () => {
      const modules = await this.#modules(), profiles = await this.profiles();
      const config = this.#state.plugins[SERVICE].configuration;
      this.#runtime = new modules.CodeNavigationService({ owner, backendFactory: modules.stdioBackendFactory(profiles), limits: { idleMs: config.idleSeconds * 1000, maxInstances: config.maxInstances } });
      this.#runtime.enable();
      return this.#runtime;
      })();
      try { return await this.#creating; } finally { this.#creating = undefined; }
    }
    return this.#runtime;
  }
  get revision() { return this.#state?.revision ?? -1; }
  get instances() { return (this.#runtime?.status.instances ?? 0) + [...this.#probeServices].reduce((sum, service) => sum + service.status.instances, 0); }
  async workspaceEnabled(root) { await this.#loaded; return this.#state.workspaces[sha(root)] === true; }
  async enableWorkspace(root, enabled) {
    return this.#exclusive(async () => {
      const next = clone(this.#state);
      if (enabled) { if (!next.plugins[SERVICE]?.enabled) fail('SERVICE_DISABLED'); next.workspaces[sha(root)] = true; }
      else { delete next.workspaces[sha(root)]; await this.#stop(); }
      next.revision++; await this.#save(next);
    });
  }
  async #detect(id) {
    const started = performance.now();
    this.#entry(id);
    if (!this.#state.plugins[id]) fail('PLUGIN_NOT_INSTALLED');
    const modules = await this.#modules();
    if (id === SERVICE) return { ...await this.#planUnlocked(id), detection: { ok: true, elapsedMs: performance.now() - started, instancesAfter: this.instances } };
    await this.#stop();
    const profiles = await this.profiles({ includeDisabled: true });
    const language = id === DART ? 'dart' : 'typescript', profile = profiles[language];
    if (!profile) fail('SDK_REQUIRED');
    const root = await mkdtemp(join(this.directory, 'probe-'));
    const controller = new AbortController(); this.#probes.add(controller);
    const text = id === DART ? 'String target() => "ok";\nvoid main() { print(target()); }\n' : 'function target() { return "ok"; }\nconsole.log(target());\n';
    const uri = pathToFileURL(join(root, `probe.${id === DART ? 'dart' : 'ts'}`)).href;
    await writeFile(new URL(uri), text);
    const service = new modules.CodeNavigationService({ backendFactory: modules.stdioBackendFactory(profiles), owner: {
      authorizeAndRead: async () => ({ partition: 'probe', workspaceKey: root, profile: language, rootUri: pathToFileURL(root).href, uri, languageId: language, version: 'probe', text }),
      assertCurrent: async () => {}, resolveLocation: async location => { if (location.uri !== uri) fail('DETECTION_FAILED'); return location; },
    } });
    this.#probeServices.add(service);
    try {
      service.enable();
      const result = await service.query({ reference: 'probe', version: 'probe', operation: 'definition', position: { line: 1, character: text.split('\n')[1].indexOf('target') + 2 } }, { access: {}, signal: controller.signal });
      if (!result.matches?.length) fail('DETECTION_FAILED');
      await service.disable();
      const next = clone(this.#state); next.plugins[id].detected = true; next.revision++; await this.#save(next);
      return { ...await this.#planUnlocked(id), detection: { ok: true, elapsedMs: performance.now() - started, instancesAfter: service.status.instances } };
    } catch (error) {
      const next = clone(this.#state); next.plugins[id].detected = false; next.revision++; await this.#save(next); throw error;
    } finally {
      try { await service.disable(); this.#probeServices.delete(service); }
      finally { this.#probes.delete(controller); await rm(root, { recursive: true, force: true }); }
    }
  }
  async #planUnlocked(id) {
    const entry = this.#state.plugins[id];
    return { ...this.#entry(id), installedVersion: entry?.version ?? null, enabled: entry?.enabled === true, configuration: clone(entry?.configuration ?? defaults(id)), revision: this.revision, detected: entry?.detected === true, instances: this.instances, action: entry?.enabled ? 'already-installed' : 'enable' };
  }
}
