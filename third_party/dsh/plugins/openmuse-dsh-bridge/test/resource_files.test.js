import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, realpath, stat, readdir, readFile, symlink } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve, relative, isAbsolute } from 'node:path';
import { normalizeFileHint, resolveSessionFiles, readSessionFile } from '../lib/resource_files.js';
async function fixture() {
  const root = await mkdtemp(join(tmpdir(), 'openmuse-locator-'));
  await mkdir(join(root, 'src')); await mkdir(join(root, 'other'));
  await writeFile(join(root, 'src', 'main.dart'), 'void main() {}\n');
  await writeFile(join(root, 'other', 'main.dart'), '// other\n');
  const services = {
    fs: { resolve: async (path, opts = {}) => realpath(resolve(opts.cwd ?? root, path)), contains: (parent, child) => { const r = relative(parent, child); return r === '' || (!r.startsWith('..') && !isAbsolute(r)); }, processPath: p => p },
    sessionPersistence: { stat: async () => ({ header: { cwd: root } }) },
    sessionController: { page: async ({address}) => { assert.equal(address.kind, 'session'); return ({ header: { cwd: root }, records: [{ seq: 3, type: 'deliverables/presented', data: { files: [{ path: 'src/main.dart' }] } }] }); } },
    workspaceFiles: {
      stat: async (scope, path) => { const absolutePath = await realpath(resolve(scope.workspaceRoot, path)); const info = await stat(absolutePath); if (!info.isFile()) throw { code: 'workspace-file/not-regular-file' }; return { absolutePath, bytes: info.size }; },
      list: async (scope, path) => { assert.ok(path); return ({ entries: (await readdir(resolve(scope.workspaceRoot, path), { withFileTypes: true })).map(e => ({ name: e.name, type: e.isFile() ? 'file' : e.isDirectory() ? 'directory' : 'other' })), truncated: false }); },
      readBytes: async (scope, path) => ({ absolutePath: path, data: await readFile(path), eof: true }),
    },
  };
  return { root, get: name => services[name] };
}
const locate = (ctx, target, extra = {}) => resolveSessionFiles(ctx, { locator: { owner: 'dsh', kind: 'path', target, context: { sessionId: 's-1' }, ...extra } });
test('normalizes URI and line references at owner', () => {
  assert.deepEqual(normalizeFileHint('file:///tmp/a%20b.dart#L120', 's-1'), { path: '/tmp/a b.dart', line: 120, column: undefined });
  assert.deepEqual(normalizeFileHint('src/a.dart:12:8', 's-1'), { path: 'src/a.dart', line: 12, column: 8 });
  assert.equal(normalizeFileHint('dsh-resource://file/session/s-1/src/a.dart', 's-1').path, 'src/a.dart');
  assert.throws(() => normalizeFileHint('dsh-resource://file/session/other/src/a.dart', 's-1'), /DENIED/);
});
test('exact references return opaque handles and actual bytes', async () => {
  const ctx = await fixture(); const result = await locate(ctx, 'src/main.dart:1:2');
  assert.equal(result.matches.length, 1); assert.equal(result.line, 1); assert.equal(result.column, 2);
  assert.equal(JSON.stringify(result).includes(ctx.root), false);
  const content = await readSessionFile(ctx, { ref: result.matches[0].ref });
  assert.equal(Buffer.from(content.bytes, 'base64').toString(), 'void main() {}\n');
  assert.match(content.digest, /^[a-f0-9]{64}$/);
});
test('incomplete hints preserve all ambiguous matches', async () => {
  const result = await locate(await fixture(), 'main.dart');
  assert.deepEqual(result.matches.map(m => m.label).sort(), ['other/main.dart', 'src/main.dart']);
});
test('presented coordinates use the original journal declaration', async () => {
  const result = await locate(await fixture(), 'ignored', { kind: 'presented', context: { sessionId: 's-1', seq: 3, index: 0 } });
  assert.equal(result.matches[0].label, 'src/main.dart');
});
test('traversal, symlink escapes and invented handles are refused', async () => {
  const ctx = await fixture(); await symlink(tmpdir(), join(ctx.root, 'escape'));
  await assert.rejects(locate(ctx, '../'), /DENIED/);
  await assert.rejects(locate(ctx, 'escape/'), /DENIED/);
  assert.equal((await locate(ctx, 'absent.dart')).matches.length, 0);
  await assert.rejects(readSessionFile(ctx, { ref: 'invented' }), /EXPIRED/);
});

test('basename fallback is ambiguous and canonical identities never fuzzy-rebind', async () => {
  const ctx = await fixture();
  const result = await locate(ctx, 'missing-prefix/main.dart');
  assert.equal(result.matches.length, 2);
  const canonical = await locate(ctx, 'missing-prefix/main.dart', { kind: 'canonical' });
  assert.equal(canonical.matches.length, 0);
});
test('relative references can use a verified file context from the same Session', async () => {
  const ctx = await fixture();
  const file = (await locate(ctx, 'src/main.dart')).matches[0];
  const result = await locate(ctx, 'main.dart', { context: { sessionId: 's-1', baseRef: file.ref } });
  assert.deepEqual(result.matches.map(m => m.label), ['src/main.dart']);
});

test('document links prefer their verified source directory and decode URI paths', async () => {
  const ctx = await fixture();
  await writeFile(join(ctx.root, 'main.dart'), '// root decoy');
  await writeFile(join(ctx.root, 'src', '中文.dart'), '// unicode');
  const base = (await locate(ctx, 'src/main.dart')).matches[0];
  const relative = (target, sessionId = 's-1') => locate(ctx, target, { kind: 'relative', context: { sessionId, baseRef: base.ref } });
  assert.equal((await relative('main.dart')).matches[0].label, 'src/main.dart');
  assert.equal((await relative('../other/main.dart')).matches[0].label, 'other/main.dart');
  const encoded = await relative('%E4%B8%AD%E6%96%87.dart#L1');
  assert.equal(encoded.matches[0].label, 'src/中文.dart');
  assert.equal(encoded.line, 1);
  await assert.rejects(relative('../../escape.txt'), /DENIED/);
  await assert.rejects(relative('main.dart', 's-2'), /EXPIRED/);
});

test('tool results resolve original call arguments rather than caller-supplied path', async () => {
  const base = await fixture();
  const ctx = { get(name) { return name === 'sessionController' ? { page: async ({address, throughSeq}) => {
    assert.equal(address.kind, 'session'); assert.equal(throughSeq, 7);
    return { records: [
      { seq: 6, type: 'tool/call', data: { callId: 'c-1', arguments: JSON.stringify({ file_path: 'src/main.dart' }) } },
      { seq: 7, type: 'tool/result', data: { callId: 'c-1' } },
    ] };
  } } : base.get(name); } };
  const result = await locate(ctx, '../untrusted', { kind: 'tool', context: { sessionId: 's-1', seq: 7 } });
  assert.deepEqual(result.matches.map(m => m.label), ['src/main.dart']);
});
