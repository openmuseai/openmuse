import { NavigationPluginManager } from '@openmuse/code-navigation-manager';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { homedir } from 'node:os';
import { join, extname } from 'node:path';
import { readFile, realpath } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { sessionFileContext, readSessionFile, resolveSessionFiles } from './resource_files.js';

const instances = new WeakMap();
const fail = code => { throw Object.assign(new Error(code), { resourceCode: code }); };
const sha = value => createHash('sha256').update(value).digest('hex');

function managerFor(ctx, partition) {
  let accounts = instances.get(ctx);
  if (!accounts) { accounts = new Map(); instances.set(ctx, accounts); }
  let manager = accounts.get(partition);
  if (!manager) {
    const config = ctx.get('codeNavigationConfig');
    manager = new NavigationPluginManager({
      directory: config?.directory ?? join(homedir(), '.config', 'openmuse', 'code-navigation'),
      artifactsRoot: config?.artifactsRoot ?? fileURLToPath(new URL('../../../navigation-artifacts/', import.meta.url)), partition,
    });
    accounts.set(partition, manager);
    ctx.effect?.(() => () => manager.stop());
  }
  return manager;
}
function checkPartition(input) {
  if (!input || typeof input.partition !== 'string' || !input.partition || input.partition.length > 256 || /[\s/\\]/.test(input.partition)) fail('DENIED');
}
// Called only by the trusted Desktop management dispatcher; gateway proxy denies
// this private path. A Mobile request never supplies its account partition here.
export async function manageNavigationPlugins(ctx, input) {
  checkPartition(input);
  return managerFor(ctx, input.partition).manage(input.operation, input.pluginId, input.options);
}
async function ownerFor(ctx, manager) {
  return {
    async authorizeAndRead(input, access, signal) {
      const file = await sessionFileContext(ctx, input.reference, signal);
      const root = file.scope.workspaceRoot;
      if (!await manager.workspaceEnabled(root)) fail('DISABLED');
      const profiles = await manager.profiles();
      const registered = Object.entries(profiles).find(([, p]) => p.extensions.includes(extname(file.path).toLowerCase()));
      if (!registered) fail('LANGUAGE_UNCONFIGURED');
      const content = await readSessionFile(ctx, { ref: input.reference }, signal);
      if (await realpath(root) !== root || sha(await readFile(file.path)) !== content.digest) fail('UNSUPPORTED_EXECUTION_WORLD');
      return { partition: `${access.partition}:${access.generation}`, workspaceKey: sha(root),
        profile: registered[0], rootUri: pathToFileURL(root).href, uri: pathToFileURL(file.path).href,
        languageId: registered[1].languageId, version: content.digest,
        text: Buffer.from(content.bytes, 'base64').toString('utf8'), fileRef: input.reference, root, revision: manager.revision };
    },
    async assertCurrent(document, access, signal) {
      if (!await manager.workspaceEnabled(document.root) || document.revision !== manager.revision) fail('DISABLED');
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
}

export async function codeNavigation(ctx, input, signal) {
  checkPartition(input);
  if (!Number.isSafeInteger(input.generation)) fail('DENIED');
  const allowed = ['status', 'enable', 'disable', 'definition', 'references', 'implementation', 'hover'];
  if (!allowed.includes(input.operation)) fail('INVALID_QUERY');
  const file = await sessionFileContext(ctx, input.ref, signal);
  const manager = managerFor(ctx, input.partition);
  const profiles = await manager.profiles();
  const configured = Object.values(profiles).some(p => p.extensions.includes(extname(file.path).toLowerCase()));
  if (input.operation === 'enable') {
    if (!configured) fail('LANGUAGE_UNCONFIGURED');
    await manager.enableWorkspace(file.scope.workspaceRoot, true);
  }
  if (input.operation === 'disable') await manager.enableWorkspace(file.scope.workspaceRoot, false);
  const state = async () => ({ enabled: configured && await manager.workspaceEnabled(file.scope.workspaceRoot), configured, instances: manager.instances });
  if (['status', 'enable', 'disable'].includes(input.operation)) return state();
  const service = await manager.runtime(await ownerFor(ctx, manager));
  const result = await service.query({ reference: input.ref, version: input.digest,
    operation: input.operation, position: { line: input.line, character: input.character } },
    { access: { partition: input.partition, generation: input.generation }, signal });
  if (result.kind === 'hover') return { ...await state(), hover: JSON.stringify(result.hover).slice(0, 65536) };
  return { ...await state(), matches: result.matches, truncated: result.truncated };
}
