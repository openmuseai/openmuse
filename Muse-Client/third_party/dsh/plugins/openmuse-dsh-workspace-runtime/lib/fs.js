import { FileSystem, FsError } from '@deepseek-ai/dsh-fs';
import { resolveControl } from './control.js';

const FS_ERROR_CODES = new Set([
  'FS_NOT_FOUND', 'FS_NOT_DIRECTORY', 'FS_NOT_TEXT', 'FS_NOT_REGULAR_FILE',
  'FS_TOO_LARGE', 'FS_PERMISSION_DENIED', 'FS_SANDBOX_DENIED', 'FS_IO_ERROR',
  'FS_STALE_VERSION', 'FS_NOT_OBSERVED', 'FS_AMBIGUOUS_EDIT',
  'FS_EDIT_NOT_FOUND', 'FS_ABORTED',
]);

function errorFromWire(error) {
  if (error instanceof FsError) return error;
  const code = error && FS_ERROR_CODES.has(error.code) ? error.code : 'FS_IO_ERROR';
  return new FsError(error instanceof Error ? error.message : String(error), code, { cause: error });
}

function bytesFromWire(value) {
  if (value instanceof Uint8Array) return value;
  if (typeof value === 'string') return Uint8Array.from(Buffer.from(value, 'base64'));
  throw new FsError('remote filesystem returned invalid byte payload', 'FS_IO_ERROR');
}

export class RemoteFileSystem extends FileSystem {
  constructor(ctx, config) {
    super(ctx);
    this.control = resolveControl(config);
    this.defaultSandboxMode = config.sandboxMode ?? 'workspace-write';
    this.targets = new Map();
  }

  get sandboxMode() { return this.defaultSandboxMode; }

  remember(wire) {
    if (!wire || typeof wire !== 'object' || !wire.target || typeof wire.processPath !== 'string'
      || typeof wire.fileUrl !== 'string' || !Array.isArray(wire.ancestorTargetKeys)) {
      throw new FsError('remote filesystem returned invalid target metadata', 'FS_IO_ERROR');
    }
    const target = Object.freeze({
      targetKey: String(wire.target.targetKey),
      displayPath: String(wire.target.displayPath),
    });
    this.targets.set(target.targetKey, Object.freeze({
      processPath: wire.processPath,
      fileUrl: wire.fileUrl,
      ancestorTargetKeys: Object.freeze(wire.ancestorTargetKeys.map(String)),
    }));
    return target;
  }

  metadata(target) {
    const metadata = this.targets.get(String(target?.targetKey));
    if (!metadata) throw new FsError('target was not resolved by this filesystem provider', 'FS_SANDBOX_DENIED');
    return metadata;
  }

  async invoke(method, params, signal) {
    try {
      return await this.control.call(method, params, signal);
    } catch (error) {
      throw errorFromWire(error);
    }
  }

  watch(target, changed, signal) {
    this.metadata(target);
    try {
      return Promise.resolve(this.control.watch({ targetKey: target.targetKey }, changed, signal));
    } catch (error) {
      return Promise.reject(errorFromWire(error));
    }
  }

  async resolve(path, opts = {}) {
    return this.remember(await this.invoke('fs.resolve', { path, cwd: opts.cwd }, opts.signal));
  }

  processPath(target) { return this.metadata(target).processPath; }
  processPathFromHostPath() { return undefined; }
  fileUrl(target) { return this.metadata(target).fileUrl; }

  contains(parent, child) {
    this.metadata(parent);
    return this.metadata(child).ancestorTargetKeys.includes(String(parent.targetKey));
  }

  stat(target, signal) {
    this.metadata(target);
    return this.invoke('fs.stat', { targetKey: target.targetKey }, signal);
  }

  lstat(path, opts = {}, signal) {
    return this.invoke('fs.lstat', { path, cwd: opts.cwd }, signal);
  }

  readText(target, signal) {
    this.metadata(target);
    return this.invoke('fs.readText', { targetKey: target.targetKey }, signal);
  }

  streamText(target, signal) {
    this.metadata(target);
    try {
      const chunks = this.control.stream('fs.streamText', { targetKey: target.targetKey }, signal);
      return Promise.resolve((async function* () {
        for await (const chunk of chunks) {
          if (typeof chunk !== 'string') throw new FsError('remote filesystem returned invalid text stream', 'FS_IO_ERROR');
          yield chunk;
        }
      })());
    } catch (error) {
      return Promise.reject(errorFromWire(error));
    }
  }

  async readBytes(target, signal, maxBytes) {
    this.metadata(target);
    return bytesFromWire(await this.invoke('fs.readBytes', { targetKey: target.targetKey, maxBytes }, signal));
  }

  async readByteRange(target, range, signal) {
    this.metadata(target);
    return bytesFromWire(await this.invoke('fs.readByteRange', { targetKey: target.targetKey, range }, signal));
  }

  async listDir(target, signal) {
    this.metadata(target);
    const entries = await this.invoke('fs.listDir', { targetKey: target.targetKey }, signal);
    if (!Array.isArray(entries)) throw new FsError('remote filesystem returned invalid directory listing', 'FS_IO_ERROR');
    return entries.map((wire) => ({
      name: wire.name,
      type: wire.type,
      target: this.remember(wire.targetMetadata),
      ...(wire.version === undefined ? {} : { version: wire.version }),
      ...(wire.size === undefined ? {} : { size: wire.size }),
    }));
  }

  writeText(target, content, expected, signal, sandboxPolicy) {
    this.metadata(target);
    return this.invoke('fs.writeText', { targetKey: target.targetKey, content, expected, sandboxPolicy }, signal);
  }

  editText(target, edit, expected, signal, sandboxPolicy) {
    this.metadata(target);
    return this.invoke('fs.editText', { targetKey: target.targetKey, edit, expected, sandboxPolicy }, signal);
  }
}

export default RemoteFileSystem;
