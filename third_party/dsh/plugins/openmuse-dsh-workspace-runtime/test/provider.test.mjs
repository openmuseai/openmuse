import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtemp, mkdir, readFile, readdir, realpath, stat, lstat, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { PassThrough } from 'node:stream';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

import { Context } from '@deepseek-ai/cordis';
import {
  CONTROL_SCHEMA,
  REMOTE_RUNNER,
  RemoteControlError,
  apply,
} from '@openmuse/dsh-workspace-runtime';
import RemoteFileSystem from '@openmuse/dsh-workspace-runtime/fs';
import RemoteSandboxProvider from '@openmuse/dsh-workspace-runtime/sandbox';
import RemoteSubprocessRuntime from '@openmuse/dsh-workspace-runtime/subprocess';

const audience = 'dsh-provider-tck';
const attachment = Object.freeze({
  tokenRef: 'opaque-test-attachment',
  runtimeRef: 'runtime-tck-1',
  audience,
  generation: 7,
  expiresAtMs: Date.now() + 60_000,
});

function outputReader(buffer) {
  return {
    readFrom(fromByte) {
      const value = Buffer.concat(buffer);
      return { text: value.subarray(fromByte).toString('utf8'), nextOffset: value.length, lossy: false };
    },
  };
}

class FixtureTransport {
  constructor(root) {
    this.root = root;
    this.targets = new Map();
    this.launches = new Map();
    this.nextLaunch = 0;
  }

  authenticate(envelope) {
    assert.equal(envelope.schema, CONTROL_SCHEMA);
    const value = envelope.attachment;
    if (value.tokenRef !== attachment.tokenRef || value.runtimeRef !== attachment.runtimeRef
      || value.audience !== audience || value.generation !== attachment.generation) {
      throw Object.assign(new Error('attachment rejected by execution service'), { code: 'FS_PERMISSION_DENIED' });
    }
    if (Date.now() >= value.expiresAtMs) throw Object.assign(new Error('attachment expired'), { code: 'FS_PERMISSION_DENIED' });
  }

  workspacePath(requested, cwd = '/workspace') {
    if (typeof requested !== 'string' || requested.length === 0) throw Object.assign(new Error('path required'), { code: 'FS_NOT_FOUND' });
    const base = cwd.startsWith('/workspace') ? cwd : '/workspace';
    const processPath = path.posix.normalize(requested.startsWith('/') ? requested : path.posix.join(base, requested));
    if (processPath !== '/workspace' && !processPath.startsWith('/workspace/')) {
      throw Object.assign(new Error('path escapes workspace'), { code: 'FS_SANDBOX_DENIED' });
    }
    const relative = processPath === '/workspace' ? '' : processPath.slice('/workspace/'.length);
    const hostPath = path.resolve(this.root, relative);
    if (hostPath !== this.root && !hostPath.startsWith(`${this.root}${path.sep}`)) {
      throw Object.assign(new Error('path escapes workspace'), { code: 'FS_SANDBOX_DENIED' });
    }
    return { processPath, hostPath, relative };
  }

  keyFor(processPath) {
    return `omfs:${createHash('sha256').update(`${attachment.runtimeRef}\0${processPath}`).digest('base64url')}`;
  }

  metadata(requested, cwd) {
    const resolved = this.workspacePath(requested, cwd);
    const targetKey = this.keyFor(resolved.processPath);
    this.targets.set(targetKey, resolved);
    const pieces = resolved.processPath.split('/').filter(Boolean);
    const ancestors = [];
    for (let index = 1; index <= pieces.length; index += 1) {
      const processPath = `/${pieces.slice(0, index).join('/')}`;
      const key = this.keyFor(processPath);
      ancestors.push(key);
      if (!this.targets.has(key)) this.targets.set(key, this.workspacePath(processPath));
    }
    return {
      target: { targetKey, displayPath: resolved.processPath },
      processPath: resolved.processPath,
      fileUrl: pathToFileURL(resolved.processPath).href,
      ancestorTargetKeys: ancestors,
    };
  }

  target(targetKey) {
    const value = this.targets.get(String(targetKey));
    if (!value) throw Object.assign(new Error('unknown opaque target'), { code: 'FS_SANDBOX_DENIED' });
    return value;
  }

  async info(target, noFollow = false) {
    try {
      const value = await (noFollow ? lstat(target.hostPath) : stat(target.hostPath));
      return {
        version: `${value.dev}:${value.ino}:${value.size}:${value.mtimeMs}`,
        type: value.isSymbolicLink() ? 'symlink' : value.isFile() ? 'file' : value.isDirectory() ? 'directory' : 'other',
        size: value.isFile() ? value.size : undefined,
      };
    } catch (error) {
      if (error?.code === 'ENOENT') return undefined;
      throw error;
    }
  }

  async call(envelope, { signal } = {}) {
    this.authenticate(envelope);
    signal?.throwIfAborted();
    const p = envelope.params ?? {};
    switch (envelope.method) {
      case 'fs.resolve': return this.metadata(p.path, p.cwd);
      case 'fs.stat': return this.info(this.target(p.targetKey));
      case 'fs.lstat': return this.info(this.workspacePath(p.path, p.cwd), true);
      case 'fs.readText': return readFile(this.target(p.targetKey).hostPath, 'utf8');
      case 'fs.readBytes': {
        const value = await readFile(this.target(p.targetKey).hostPath);
        if (value.length > p.maxBytes) throw Object.assign(new Error('file too large'), { code: 'FS_TOO_LARGE' });
        return value.toString('base64');
      }
      case 'fs.readByteRange': {
        const value = await readFile(this.target(p.targetKey).hostPath);
        return value.subarray(p.range.offset, p.range.offset + p.range.length).toString('base64');
      }
      case 'fs.listDir': {
        const directory = this.target(p.targetKey);
        const names = (await readdir(directory.hostPath)).sort();
        return Promise.all(names.map(async (name) => {
          const targetMetadata = this.metadata(path.posix.join(directory.processPath, name));
          const info = await this.info(this.target(targetMetadata.target.targetKey));
          return { name, type: info.type, version: info.version, size: info.size, targetMetadata };
        }));
      }
      case 'fs.writeText': {
        const target = this.target(p.targetKey);
        if (p.sandboxPolicy?.mode === 'read-only') throw Object.assign(new Error('read-only policy'), { code: 'FS_SANDBOX_DENIED' });
        const beforeInfo = await this.info(target);
        if (p.expected?.kind === 'createIfAbsent' && beforeInfo) throw Object.assign(new Error('already exists'), { code: 'FS_NOT_OBSERVED' });
        if (p.expected?.kind === 'replaceIfVersion' && beforeInfo?.version !== p.expected.version) throw Object.assign(new Error('stale'), { code: 'FS_STALE_VERSION' });
        const before = beforeInfo ? await readFile(target.hostPath, 'utf8') : null;
        await mkdir(path.dirname(target.hostPath), { recursive: true });
        await writeFile(target.hostPath, p.content, 'utf8');
        return { operation: beforeInfo ? 'update' : 'create', version: (await this.info(target)).version, before, after: p.content };
      }
      case 'fs.editText': {
        const target = this.target(p.targetKey);
        if (p.sandboxPolicy?.mode === 'read-only') throw Object.assign(new Error('read-only policy'), { code: 'FS_SANDBOX_DENIED' });
        const info = await this.info(target);
        if (!info || (p.expected && p.expected.version !== info.version)) throw Object.assign(new Error('stale'), { code: 'FS_STALE_VERSION' });
        const before = await readFile(target.hostPath, 'utf8');
        const count = before.split(p.edit.oldString).length - 1;
        if (count === 0) throw Object.assign(new Error('not found'), { code: 'FS_EDIT_NOT_FOUND' });
        if (count > 1 && !p.edit.replaceAll) throw Object.assign(new Error('ambiguous'), { code: 'FS_AMBIGUOUS_EDIT' });
        const after = before.split(p.edit.oldString).join(p.edit.newString);
        await writeFile(target.hostPath, after, 'utf8');
        return { version: (await this.info(target)).version, before, after };
      }
      case 'subprocess.resolveExecutable': {
        if (!['bash', 'sh', 'node', 'cat'].includes(p.command) && !p.command.startsWith('/runtime/bin/')) throw new Error('executable unavailable');
        return p.command.startsWith('/') ? p.command : `/runtime/bin/${p.command}`;
      }
      case 'subprocess.terminalEnvironment': return { platform: 'posix', defaultShell: '/runtime/bin/bash' };
      case 'sandbox.prepare': {
        if (!['read-only', 'workspace-write'].includes(p.policy?.mode) || p.policy.workspaceRoot !== '/workspace') throw new Error('unsupported sandbox policy');
        const launchToken = `launch.${++this.nextLaunch}`;
        this.launches.set(launchToken, { argv: p.argv, policy: p.policy });
        return { launchToken, enforcement: 'full', denialSignatures: ['OPENMUSE_SANDBOX_DENIED'] };
      }
      default: throw new Error(`unsupported control method ${envelope.method}`);
    }
  }

  async *stream(envelope, { signal } = {}) {
    this.authenticate(envelope);
    signal?.throwIfAborted();
    if (envelope.method !== 'fs.streamText') throw new Error(`unsupported stream method ${envelope.method}`);
    const value = await readFile(this.target(envelope.params.targetKey).hostPath, 'utf8');
    for (let offset = 0; offset < value.length; offset += 3) {
      signal?.throwIfAborted();
      yield value.slice(offset, offset + 3);
    }
  }

  watch(envelope, _changed, signal) {
    this.authenticate(envelope);
    signal.throwIfAborted();
    return async () => {};
  }

  executable(value) {
    if (value.startsWith('/runtime/bin/')) return value.slice('/runtime/bin/'.length);
    return value;
  }

  launchSpec(envelope) {
    this.authenticate(envelope);
    const spec = structuredClone(envelope.params.spec);
    if (spec.argv[0] === REMOTE_RUNNER) {
      const token = spec.argv[1];
      const launch = this.launches.get(token);
      if (!launch || spec.argv[2] !== '--' || JSON.stringify(spec.argv.slice(3)) !== JSON.stringify(launch.argv)) {
        throw new Error('OPENMUSE_REMOTE_RUNNER_FAILURE: invalid or replayed launch token');
      }
      this.launches.delete(token);
      spec.argv = launch.argv;
    }
    const cwd = this.workspacePath(spec.cwd).hostPath;
    const argv = spec.argv.map((item, index) => {
      if (index === 0) return this.executable(item);
      return typeof item === 'string' && item.startsWith('/workspace') ? this.workspacePath(item).hostPath : item;
    });
    return { ...spec, argv, cwd };
  }

  openProcess(envelope) {
    const spec = this.launchSpec(envelope);
    if (envelope.method === 'subprocess.spawnTerminal') return this.openTerminal(spec);
    if (envelope.method !== 'subprocess.spawn') throw new Error('unsupported streaming method');
    const stdoutChunks = [];
    const stderrChunks = [];
    const stdoutMode = spec.stdio.stdout;
    const stderrMode = spec.stdio.stderr;
    const child = spawn(spec.argv[0], spec.argv.slice(1), {
      cwd: spec.cwd,
      env: { ...process.env, ...spec.env },
      stdio: [spec.stdio.stdin === 'ignore' ? 'ignore' : 'pipe', stdoutMode === 'inherit' ? 'inherit' : 'pipe', stderrMode === 'inherit' ? 'inherit' : 'pipe'],
    });
    if (stdoutMode !== 'pipe' && stdoutMode !== 'inherit') child.stdout.on('data', (chunk) => stdoutChunks.push(Buffer.from(chunk)));
    if (stderrMode !== 'pipe' && stderrMode !== 'inherit') child.stderr.on('data', (chunk) => stderrChunks.push(Buffer.from(chunk)));
    if (typeof spec.stdio.stdin === 'object') child.stdin.end(spec.stdio.stdin.data);
    const done = new Promise((resolve, reject) => {
      child.once('error', reject);
      child.once('close', (exitCode, signal) => resolve({ exitCode, signal }));
    });
    return {
      stdin: spec.stdio.stdin === 'pipe' ? child.stdin : undefined,
      stdout: stdoutMode === 'pipe' ? child.stdout : undefined,
      stderr: stderrMode === 'pipe' ? child.stderr : undefined,
      control: undefined,
      collected: {
        ...(typeof stdoutMode === 'object' ? { stdout: outputReader(stdoutChunks) } : {}),
        ...(typeof stderrMode === 'object' ? { stderr: outputReader(stderrChunks) } : {}),
      },
      done,
      terminate: () => { if (child.exitCode === null) child.kill('SIGKILL'); },
      waitForExit: async (signal) => {
        if (!signal) { await done.catch(() => {}); return true; }
        return Promise.race([done.then(() => true, () => true), new Promise((resolve) => signal.addEventListener('abort', () => resolve(false), { once: true }))]);
      },
    };
  }

  openTerminal(spec) {
    const output = new PassThrough();
    const child = spawn(spec.argv[0], spec.argv.slice(1), { cwd: spec.cwd, env: { ...process.env, ...spec.env, TERM: spec.terminalType }, stdio: ['pipe', 'pipe', 'pipe'] });
    child.stdout.pipe(output, { end: false });
    child.stderr.pipe(output, { end: false });
    const done = new Promise((resolve, reject) => {
      child.once('error', reject);
      child.once('close', (exitCode, signal) => { output.end(); resolve({ exitCode, signal }); });
    });
    return {
      pid: child.pid,
      output,
      done,
      write: async (data) => { child.stdin.write(data); },
      resize: async () => {},
      inspectForeground: async () => ({ processGroupId: child.pid, inputWaiting: true }),
      inspectActivity: async () => ({ state: child.exitCode === null ? 'busy' : 'idle', revision: 1 }),
      signalForeground: async (signal) => { child.kill(signal); return child.pid; },
      terminate: async () => { if (child.exitCode === null) child.kill('SIGKILL'); await done.catch(() => {}); },
    };
  }
}

function collect(stream) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    stream.on('data', (chunk) => chunks.push(Buffer.from(chunk)));
    stream.once('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
    stream.once('error', reject);
  });
}

test('remote Provider group shares one execution world and preserves DSH seams', async (t) => {
  const root = await realpath(await mkdtemp(path.join(tmpdir(), 'openmuse-dsh-remote-')));
  const ctx = new Context();
  const transport = new FixtureTransport(root);
  apply(ctx, { transport, attachment, audience, sandboxMode: 'workspace-write' });
  t.after(async () => { await ctx.fiber.dispose(); });

  const workspace = await ctx.fs.resolve('/workspace');
  const fromFs = await ctx.fs.resolve('from-fs.txt', { cwd: '/workspace' });
  assert.equal(ctx.fs.processPath(fromFs), '/workspace/from-fs.txt');
  assert.equal(ctx.fs.fileUrl(fromFs), 'file:///workspace/from-fs.txt');
  assert.equal(ctx.fs.contains(workspace, fromFs), true);
  assert.equal(ctx.fs.processPathFromHostPath(path.join(root, 'from-fs.txt')), undefined);
  await ctx.fs.writeText(fromFs, 'alpha\n', undefined, undefined, { mode: 'workspace-write', workspaceRoot: '/workspace' });

  const cat = ctx.subprocess.spawn({
    argv: ['/runtime/bin/cat', ctx.fs.processPath(fromFs)], cwd: '/workspace',
    stdio: { stdin: 'ignore', stdout: { maxBytes: 1024 }, stderr: { maxBytes: 1024 } }, graceMs: 100,
  });
  assert.equal((await cat.done).exitCode, 0);
  assert.equal(cat.collected.stdout.readFrom(0).text, 'alpha\n', 'FS writes must be immediately visible to Bash');

  const confined = await ctx.sandbox.confine(['/runtime/bin/bash', '-c', 'printf beta > from-bash.txt'], {
    mode: 'workspace-write', workspaceRoot: '/workspace',
  });
  assert.equal(confined.enforcement, 'full');
  const bash = ctx.subprocess.spawn({
    argv: confined.argv, cwd: '/workspace',
    stdio: { stdin: 'ignore', stdout: { maxBytes: 1024 }, stderr: { maxBytes: 1024 } }, graceMs: 100,
  });
  assert.equal((await bash.done).exitCode, 0);
  const fromBash = await ctx.fs.resolve('/workspace/from-bash.txt');
  assert.equal(await ctx.fs.readText(fromBash), 'beta', 'Bash writes must be immediately visible to FS');
  let streamed = '';
  for await (const chunk of await ctx.fs.streamText(fromFs)) streamed += chunk;
  assert.equal(streamed, 'alpha\n', 'large text reads must stay on the streaming transport seam');
  await assert.rejects(() => ctx.fs.resolve('/workspace/../../etc/passwd'), (error) => error.code === 'FS_SANDBOX_DENIED');
  await assert.rejects(() => ctx.fs.writeText(fromBash, 'denied', undefined, undefined, { mode: 'read-only', workspaceRoot: '/workspace' }), (error) => error.code === 'FS_SANDBOX_DENIED');

  const lsp = ctx.subprocess.spawn({
    argv: ['/runtime/bin/node', '-e', 'process.stdin.pipe(process.stdout)'], cwd: '/workspace',
    stdio: { stdin: 'pipe', stdout: 'pipe', stderr: { maxBytes: 1024 } }, graceMs: 100,
  });
  const lspOutput = collect(lsp.stdout);
  const frame = 'Content-Length: 18\r\n\r\n{"jsonrpc":"2.0"}';
  lsp.stdin.end(frame);
  assert.equal(await lspOutput, frame, 'raw pipe must preserve LSP framing');
  assert.equal((await lsp.done).exitCode, 0);

  const terminal = await ctx.subprocess.spawnTerminal({
    argv: ['/runtime/bin/sh', '-c', 'IFS= read -r line; printf "pty:%s\\n" "$line"'], cwd: '/workspace',
    rows: 24, cols: 80, terminalType: 'xterm-256color', graceMs: 100,
  });
  const terminalOutput = collect(terminal.output);
  await terminal.write('hello\n');
  assert.equal((await terminal.done).exitCode, 0);
  assert.match(await terminalOutput, /pty:hello/);

  const controller = new AbortController();
  const sleeper = ctx.subprocess.spawn({
    argv: ['/runtime/bin/sh', '-c', 'sleep 20'], cwd: '/workspace',
    stdio: { stdin: 'ignore', stdout: { maxBytes: 16 }, stderr: { maxBytes: 16 } }, graceMs: 50, signal: controller.signal,
  });
  controller.abort();
  assert.equal(await sleeper.waitForExit(), true);
  assert.notEqual((await sleeper.done).signal, null, 'abort must terminate the managed process');
});

test('attachments are audience-bound, expiring, and authenticated again by the server', async () => {
  assert.throws(() => apply(new Context(), {
    transport: { call() {} }, attachment, audience: 'wrong-audience',
  }), (error) => error instanceof RemoteControlError && error.code === 'ATTACHMENT_AUDIENCE_MISMATCH');

  let now = Date.now();
  const expiring = { ...attachment, expiresAtMs: now + 10 };
  const ctx = new Context();
  apply(ctx, {
    transport: new FixtureTransport(await realpath(await mkdtemp(path.join(tmpdir(), 'openmuse-dsh-auth-')))),
    attachment: expiring,
    audience,
    now: () => now,
  });
  now += 11;
  await assert.rejects(() => ctx.fs.resolve('/workspace'), (error) => error.code === 'FS_IO_ERROR' && error.cause?.code === 'ATTACHMENT_EXPIRED');
  await ctx.fiber.dispose();

  const serverCtx = new Context();
  apply(serverCtx, {
    transport: new FixtureTransport(await realpath(await mkdtemp(path.join(tmpdir(), 'openmuse-dsh-server-auth-')))),
    attachment: { ...attachment, tokenRef: 'client-valid-but-server-unknown' },
    audience,
  });
  await assert.rejects(() => serverCtx.fs.resolve('/workspace'), (error) => error.code === 'FS_PERMISSION_DENIED');
  await serverCtx.fiber.dispose();
});

test('Cordis row entrypoints accept the same deployment config independently', async () => {
  const ctx = new Context();
  const transport = new FixtureTransport(await realpath(await mkdtemp(path.join(tmpdir(), 'openmuse-dsh-rows-'))));
  const config = { transport, attachment, audience };
  new RemoteFileSystem(ctx, config);
  new RemoteSandboxProvider(ctx, config);
  new RemoteSubprocessRuntime(ctx, config);
  assert.ok(ctx.fs instanceof RemoteFileSystem);
  assert.ok(ctx.sandbox instanceof RemoteSandboxProvider);
  assert.ok(ctx.subprocess instanceof RemoteSubprocessRuntime);
  assert.equal((await ctx.fs.resolve('/workspace')).displayPath, '/workspace');
  await ctx.fiber.dispose();
});
