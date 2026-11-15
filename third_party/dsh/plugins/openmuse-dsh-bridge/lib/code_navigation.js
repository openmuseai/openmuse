import { CodeNavigationService } from '@openmuse/code-navigation-service';
import { stdioBackendFactory } from '@openmuse/code-navigation-service/stdio';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { homedir } from 'node:os';
import { join, extname } from 'node:path';
import { readFile, writeFile, mkdir, rename, realpath } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { sessionFileContext, readSessionFile, resolveSessionFiles } from './resource_files.js';

const instances = new WeakMap();
const fail = code => { throw Object.assign(new Error(code), { resourceCode: code }); };
const sha = value => createHash('sha256').update(value).digest('hex');
const run = promisify(execFile);

async function create(ctx) {
  const directory = ctx.get('codeNavigationConfig')?.directory ?? join(homedir(), '.config', 'openmuse', 'code-navigation');
  let profiles = {};
  try {
    const bytes = await readFile(join(directory, 'profiles.json'));
    if (bytes.length > 65536) fail('INVALID_POLICY');
    profiles = JSON.parse(bytes);
  } catch (error) { if (error.code !== 'ENOENT') throw error; }
  if (!profiles.dart) {
    try {
      const { stdout } = await run(process.platform === 'win32' ? 'where' : 'which', ['dart']);
      const command = stdout.trim().split(/\r?\n/)[0];
      if (command) profiles.dart = { command, args: ['language-server', '--protocol=lsp'], extensions: ['.dart'], languageId: 'dart' };
    } catch { /* An unavailable language is shown as unconfigured. */ }
  }
  const configured = path => Object.entries(profiles).find(([, p]) => p.extensions?.includes(extname(path).toLowerCase()));
  let policies = {};
  try { policies = JSON.parse(await readFile(join(directory, 'workspaces.json'), 'utf8')); }
  catch (error) { if (error.code !== 'ENOENT') throw error; }
  let writes = Promise.resolve();
  const key = (partition, root) => sha(JSON.stringify([partition, root]));
  const owner = {
    async authorizeAndRead(input, access, signal) {
      const file = await sessionFileContext(ctx, input.reference, signal);
      const root = file.scope.workspaceRoot;
      const policy = key(access.partition, root);
      if (policies[policy] !== true) fail('DISABLED');
      const registered = configured(file.path);
      if (!registered) fail('LANGUAGE_UNCONFIGURED');
      const content = await readSessionFile(ctx, { ref: input.reference }, signal);
      // The built-in stdio adapter is explicitly local. A different execution
      // world must install its own backend rather than reinterpret host paths.
      if (await realpath(root) !== root || sha(await readFile(file.path)) !== content.digest) fail('UNSUPPORTED_EXECUTION_WORLD');
      return { partition: `${access.partition}:${access.generation}`, workspaceKey: sha(root),
        profile: registered[0], rootUri: pathToFileURL(root).href, uri: pathToFileURL(file.path).href,
        languageId: registered[1].languageId, version: content.digest,
        text: Buffer.from(content.bytes, 'base64').toString('utf8'), fileRef: input.reference, policy };
    },
    async assertCurrent(document, access, signal) {
      if (!policies[document.policy]) fail('DISABLED');
      const current = await readSessionFile(ctx, { ref: document.fileRef }, signal);
      if (current.digest !== document.version) fail('VERSION_MISMATCH');
    },
    async resolveLocation(location, document, access, signal) {
      if (!location.uri.startsWith('file:')) fail('DENIED');
      const file = await sessionFileContext(ctx, document.fileRef, signal);
      const resolved = await resolveSessionFiles(ctx, { locator: { owner: 'dsh', kind: 'canonical', target: fileURLToPath(location.uri), context: { sessionId: file.scope.sessionId } } }, signal);
      if (resolved.matches.length !== 1 || resolved.truncated) fail('DENIED');
      const target = resolved.matches[0];
      const current = await readSessionFile(ctx, { ref: target.ref }, signal);
      return { ref: target.ref, label: target.label, digest: current.digest, range: location.range, selectionRange: location.selectionRange };
    },
  };
  const service = new CodeNavigationService({ owner, backendFactory: stdioBackendFactory(profiles) });
  // Merely enabling the service does not start any language processes. Owner
  // policy still denies all workspaces until explicitly opted in.
  service.enable();
  ctx.effect?.(() => () => service.disable());
  return { service, configured, key, policies, async save() {
    const snapshot = JSON.stringify(policies);
    writes = writes.catch(() => {}).then(async () => {
      await mkdir(directory, { recursive: true, mode: 0o700 });
      const temp = join(directory, `workspaces-${process.pid}.tmp`);
      await writeFile(temp, snapshot, { mode: 0o600 });
      await rename(temp, join(directory, 'workspaces.json'));
    });
    await writes;
  } };
}

export async function codeNavigation(ctx, input, signal) {
  if (!input || typeof input.partition !== 'string' || !input.partition || input.partition.length > 256 || /[\s/\\]/.test(input.partition) || !Number.isSafeInteger(input.generation)) fail('DENIED');
  const allowed = ['status', 'enable', 'disable', 'definition', 'references', 'implementation', 'hover'];
  if (!allowed.includes(input.operation)) fail('INVALID_QUERY');
  const file = await sessionFileContext(ctx, input.ref, signal);
  let pending = instances.get(ctx);
  if (!pending) { pending = create(ctx); instances.set(ctx, pending); }
  const runtime = await pending;
  const policyKey = runtime.key(input.partition, file.scope.workspaceRoot);
  if (input.operation === 'enable') {
    if (!runtime.configured(file.path)) fail('LANGUAGE_UNCONFIGURED');
    runtime.policies[policyKey] = true;
    try { await runtime.save(); } catch (error) { delete runtime.policies[policyKey]; throw error; }
  }
  if (input.operation === 'disable') {
    delete runtime.policies[policyKey];
    await runtime.service.stopAll();
    await runtime.save();
  }
  const state = () => ({ enabled: runtime.policies[policyKey] === true, configured: !!runtime.configured(file.path), instances: runtime.service.status.instances });
  if (['status', 'enable', 'disable'].includes(input.operation)) return state();
  const result = await runtime.service.query({ reference: input.ref, version: input.digest,
    operation: input.operation, position: { line: input.line, character: input.character } },
    { access: { partition: input.partition, generation: input.generation }, signal });
  if (result.kind === 'hover') return { ...state(), hover: JSON.stringify(result.hover).slice(0, 65536) };
  return { ...state(), matches: result.matches, truncated: result.truncated };
}
