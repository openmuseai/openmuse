import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, writeFile, readFile, stat, realpath, rm, mkdir } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve, relative, isAbsolute } from 'node:path';
import { resolveSessionFiles, readSessionFile } from '../../lib/resource_files.js';
import { codeNavigation, manageNavigationPlugins } from '../../lib/code_navigation.js';

test('real TypeScript owner adapter: opt-in, cross-file function/method, version fence, release', async t => {
  const root = await realpath(await mkdtemp(join(tmpdir(), 'navigation-owner-')));
  const policy = join(root, 'config'); await mkdir(policy);
  await writeFile(join(root, '定义.ts'), 'export function greeting() { return "FUNCTION_OK"; }\nexport class Service {\n  greet() { return "METHOD_OK"; }\n}\n');
  await writeFile(join(root, 'main.ts'), "import { greeting, Service } from './定义';\nconst value = greeting();\nconst method = new Service().greet();\n");
  const cleanups = [];
  const services = {
    codeNavigationConfig: { directory: policy, artifactsRoot: new URL('../../../../navigation-artifacts/', import.meta.url).pathname },
    fs: { resolve: async (p, options = {}) => realpath(resolve(options.cwd ?? root, p)), processPath: x => x, contains: (a, b) => { const r = relative(a, b); return !r || !r.startsWith('..') && !isAbsolute(r); } },
    sessionPersistence: { stat: async () => ({ header: { cwd: root } }) },
    workspaceFiles: {
      stat: async (scope, p) => { const absolutePath = await realpath(resolve(scope.workspaceRoot, p)); return { absolutePath, bytes: (await stat(absolutePath)).size }; },
      readBytes: async (scope, p) => ({ absolutePath: await realpath(resolve(scope.workspaceRoot, p)), data: await readFile(resolve(scope.workspaceRoot, p)), eof: true }),
    },
  };
  const ctx = { get: key => services[key], effect: f => cleanups.push(f()) };
  t.after(async () => { for (const dispose of cleanups) await dispose(); await rm(root, { recursive: true, force: true }); });
  const partition = 'account:test';
  const serviceId = 'com.openmuse.code-navigation', dartId = serviceId + '.typescript';
  const manage = (operation, pluginId, options = {}) => manageNavigationPlugins(ctx, { partition, operation, pluginId, options });
  await manage('install', serviceId);
  await manage('enable', serviceId);
  await manage('install', dartId);
  let config = await manage('plan', dartId);
  await manage('configure', dartId, { expectedRevision: config.revision, values: { sdkPath: '' } });
  await manage('detect', dartId);
  await manage('enable', dartId);
  const resolved = await resolveSessionFiles(ctx, { locator: { kind: 'canonical', target: 'main.ts', context: { sessionId: 'test-session' } } });
  const ref = resolved.matches[0].ref;
  const content = await readSessionFile(ctx, { ref });
  const request = { ref, partition: 'account:test', generation: 1 };
  const call = (operation, extra = {}) => codeNavigation(ctx, { ...request, operation, ...extra });
  assert.equal((await call('status')).enabled, false);
  assert.equal((await call('status')).instances, 0);
  await assert.rejects(call('definition', { digest: content.digest, line: 1, character: 17 }), /DISABLED/);
  assert.equal((await call('enable')).enabled, true);
  assert.equal((await call('status')).instances, 0);
  const definition = await call('definition', { digest: content.digest, line: 1, character: 17 });
  assert.equal(definition.matches[0].label, '定义.ts');
  assert.equal(definition.matches[0].selectionRange.start.line, 0);
  const method = await call('definition', { digest: content.digest, line: 2, character: 30 });
  assert.equal(method.matches[0].label, '定义.ts');
  assert.equal(method.matches[0].selectionRange.start.line, 2);
  await assert.rejects(call('definition', { digest: '0'.repeat(64), line: 1, character: 17 }), /VERSION_MISMATCH/);
  const disabled = await call('disable');
  assert.equal(disabled.enabled, false); assert.equal(disabled.instances, 0);
});