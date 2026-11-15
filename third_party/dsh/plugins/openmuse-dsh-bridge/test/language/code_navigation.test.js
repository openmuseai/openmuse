import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, writeFile, readFile, stat, realpath, rm, mkdir } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve, relative, isAbsolute } from 'node:path';
import { resolveSessionFiles, readSessionFile } from '../../lib/resource_files.js';
import { codeNavigation } from '../../lib/code_navigation.js';

test('real Dart owner adapter: opt-in, cross-file function/method, version fence, release', async t => {
  const root = await realpath(await mkdtemp(join(tmpdir(), 'navigation-owner-')));
  const policy = join(root, 'config'); await mkdir(policy);
  const dart = process.env.OPENMUSE_TEST_DART;
  assert.ok(dart, 'Set OPENMUSE_TEST_DART to the installed Dart executable for this real language test.');
  await writeFile(join(policy, 'profiles.json'), JSON.stringify({ dart: { command: dart, args: ['language-server', '--protocol=lsp'], extensions: ['.dart'], languageId: 'dart', initializationOptions: { onlyAnalyzeProjectsWithOpenFiles: true } } }));
  await writeFile(join(root, '定义.dart'), 'String greeting() => "FUNCTION_OK";\nclass Service {\n  String greet() => "METHOD_OK";\n}\n');
  await writeFile(join(root, 'main.dart'), "import '定义.dart';\nvoid main() {\n  print(greeting());\n  print(Service().greet());\n}\n");
  const cleanups = [];
  const services = {
    codeNavigationConfig: { directory: policy },
    fs: { resolve: async (p, options = {}) => realpath(resolve(options.cwd ?? root, p)), processPath: x => x, contains: (a, b) => { const r = relative(a, b); return !r || !r.startsWith('..') && !isAbsolute(r); } },
    sessionPersistence: { stat: async () => ({ header: { cwd: root } }) },
    workspaceFiles: {
      stat: async (scope, p) => { const absolutePath = await realpath(resolve(scope.workspaceRoot, p)); return { absolutePath, bytes: (await stat(absolutePath)).size }; },
      readBytes: async (scope, p) => ({ absolutePath: await realpath(resolve(scope.workspaceRoot, p)), data: await readFile(resolve(scope.workspaceRoot, p)), eof: true }),
    },
  };
  const ctx = { get: key => services[key], effect: f => cleanups.push(f()) };
  t.after(async () => { for (const dispose of cleanups) await dispose(); await rm(root, { recursive: true, force: true }); });
  const resolved = await resolveSessionFiles(ctx, { locator: { kind: 'canonical', target: 'main.dart', context: { sessionId: 'test-session' } } });
  const ref = resolved.matches[0].ref;
  const content = await readSessionFile(ctx, { ref });
  const request = { ref, partition: 'account:test', generation: 1 };
  const call = (operation, extra = {}) => codeNavigation(ctx, { ...request, operation, ...extra });
  assert.equal((await call('status')).enabled, false);
  assert.equal((await call('status')).instances, 0);
  await assert.rejects(call('definition', { digest: content.digest, line: 2, character: 10 }), /DISABLED/);
  assert.equal((await call('enable')).enabled, true);
  assert.equal((await call('status')).instances, 0);
  const definition = await call('definition', { digest: content.digest, line: 2, character: 10 });
  assert.equal(definition.matches[0].label, '定义.dart');
  assert.equal(definition.matches[0].selectionRange.start.line, 0);
  const method = await call('definition', { digest: content.digest, line: 3, character: 19 });
  assert.equal(method.matches[0].label, '定义.dart');
  assert.equal(method.matches[0].selectionRange.start.line, 2);
  await assert.rejects(call('definition', { digest: '0'.repeat(64), line: 2, character: 10 }), /VERSION_MISMATCH/);
  const disabled = await call('disable');
  assert.equal(disabled.enabled, false); assert.equal(disabled.instances, 0);
});
