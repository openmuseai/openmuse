import { randomUUID, createHash } from 'node:crypto';
import { posix, win32 } from 'node:path';

const owners = new WeakMap();
const fail = (code) => { throw Object.assign(new Error(code), { resourceCode: code }); };
const missing = (e) => ['ENOENT', 'FS_NOT_FOUND', 'workspace-file/not-found', 'workspace-file/not-regular-file'].includes(e?.code) || ['workspace-file/not-found', 'workspace-file/not-regular-file'].includes(e?.details?.code);
function paths(root) { return /^[A-Za-z]:[\\/]|^\\\\/.test(root) ? win32 : posix; }
export function normalizeFileHint(raw, sessionId) {
  let value = raw.trim().replace(/^`+|`+$/g, '');
  let line, column;
  const location = value.match(/(?:#L(\d+)(?:C(\d+))?(?:-L?\d+)?|:(\d+)(?::(\d+))?)$/);
  if (location) { line = Number(location[1] ?? location[3]); column = Number(location[2] ?? location[4]) || undefined; value = value.slice(0, location.index); }
  if (value.startsWith('dsh-resource://')) {
    const u = new URL(value);
    if (u.host !== 'file') fail('INVALID_REFERENCE');
    const parts = u.pathname.slice(1).split('/').map(decodeURIComponent);
    if (parts[0] === 'session') {
      if (parts[1] !== sessionId) fail('DENIED');
      value = parts.slice(2).join('/');
    } else if (parts[0] === 'absolute') {
      value = parts.slice(1).join('/');
      if (!/^[A-Za-z]:[\\/]/.test(value)) value = `/${value}`;
    } else fail('INVALID_REFERENCE');
  } else if (value.startsWith('file://')) {
    const u = new URL(value);
    if (u.hostname && u.hostname !== 'localhost') fail('DENIED');
    value = decodeURIComponent(u.pathname);
    if (/^\/[A-Za-z]:\//.test(value)) value = value.slice(1);
  } else if (value.includes('://')) fail('INVALID_REFERENCE');
  else {
    // Markdown hrefs use URI escaping even when they are relative paths.
    const fragment = value.indexOf('#');
    if (fragment >= 0) value = value.slice(0, fragment);
    try { value = decodeURIComponent(value); } catch { fail('INVALID_REFERENCE'); }
  }
  if (!value || value.length > 4096 || /[\x00-\x1f]/.test(value)) fail('INVALID_REFERENCE');
  return { path: value.replaceAll('\\', '/'), line, column };
}

async function scopeOf(ctx, sessionId, signal) {
  const metadata = await ctx.get('sessionPersistence').stat(sessionId, { signal });
  const root = metadata?.header?.cwd;
  if (typeof root !== 'string' || !root) fail('SOURCE_UNAVAILABLE');
  const fs = ctx.get('fs');
  const rootTarget = await fs.resolve(root, { signal });
  return { sessionId, workspaceRoot: fs.processPath(rootTarget), rootTarget };
}
async function checked(ctx, scope, path, signal) {
  const fs = ctx.get('fs');
  const root = await fs.resolve(scope.workspaceRoot, { signal });
  if (fs.processPath(root) !== scope.workspaceRoot || (scope.rootTarget && (!fs.contains(scope.rootTarget, root) || !fs.contains(root, scope.rootTarget)))) fail('DENIED');
  const target = await fs.resolve(path, { cwd: scope.workspaceRoot, signal });
  if (!fs.contains(root, target)) fail('DENIED');
  const result = await ctx.get('workspaceFiles').stat(scope, fs.processPath(target), signal);
  // Check the canonical target again: a symlink must not widen the grant.
  const canonical = await fs.resolve(result.absolutePath, { signal });
  if (!fs.contains(root, canonical)) fail('DENIED');
  return result;
}
function store(ctx) {
  let map = owners.get(ctx);
  if (!map) { map = new Map(); owners.set(ctx, map); }
  for (const [key, value] of map) if (value.expires <= Date.now()) map.delete(key);
  return map;
}
export async function resolveSessionFiles(ctx, input, signal) {
  const locator = input.locator;
  const sessionId = locator?.context?.sessionId;
  if (typeof sessionId !== 'string' || !/^[\w.-]{1,256}$/.test(sessionId)) fail('INVALID_REFERENCE');
  const scope = await scopeOf(ctx, sessionId, signal);
  let raw = locator.target;
  const { seq, index } = locator.context;
  if (locator.kind === 'presented' || locator.kind === 'tool') {
    if (!Number.isSafeInteger(seq) || seq < 1) fail('INVALID_REFERENCE');
    const page = await ctx.get('sessionController').page({ address: { kind: 'session', sessionId }, throughSeq: seq, maxMessages: 80, turnWindow: { minMessages: 1, minTurns: 1 } }, signal);
    const record = page?.records?.find(r => r.seq === seq);
    if (locator.kind === 'presented') {
      if (!Number.isSafeInteger(index) || index < 0 || record?.type !== 'deliverables/presented') fail('NOT_FOUND');
      raw = record.data?.files?.[index]?.path;
    } else {
      const call = record?.type === 'tool/result' ? page.records.find(r => r.type === 'tool/call' && r.data?.callId === record.data?.callId) : record;
      let args = call?.data?.arguments ?? call?.data?.tool?.arguments;
      if (typeof args === 'string') { try { args = JSON.parse(args); } catch { fail('NOT_FOUND'); } }
      raw = args?.path ?? args?.file_path ?? args?.filePath ?? args?.target_file;
    }
  } else if (!['path', 'canonical', 'relative'].includes(locator.kind)) fail('INVALID_REFERENCE');
  if (typeof raw !== 'string') fail('NOT_FOUND');
  const hint = normalizeFileHint(raw, sessionId), p = paths(scope.workspaceRoot);
  const found = new Map(), basenameMatches = new Map();
  async function add(path) {
    try { const file = await checked(ctx, scope, path, signal); found.set(file.absolutePath, file); return true; }
    catch (e) { if (missing(e)) return false; throw e; }
  }
  const base = store(ctx).get(locator.context.baseRef);
  if (locator.kind === 'relative' && (!base || base.scope.sessionId !== sessionId)) fail('EXPIRED');
  let exact = locator.kind === 'relative' && !p.isAbsolute(hint.path)
    ? await add(p.join(p.dirname(base.path), hint.path))
    : await add(hint.path);
  if (!exact && base && base.scope.sessionId === sessionId && !p.isAbsolute(hint.path)) {
    exact = await add(p.join(p.dirname(base.path), hint.path));
  }
  let truncated = false;
  if (!exact && locator.kind !== 'canonical' && !p.isAbsolute(hint.path)) {
    // Suffix search is bounded and remains in this Session's execution world.
    const needle = p.normalize(hint.path).replaceAll('\\', '/').replace(/^\.\//, '');
    if (needle.startsWith('../')) fail('DENIED');
    const queue = ['.']; let visited = 0;
    const deadline = Date.now() + 1500;
    while (queue.length && visited < 3000 && Date.now() < deadline && found.size < 20) {
      const dir = queue.shift();
      const listing = await ctx.get('workspaceFiles').list(scope, dir, signal);
      truncated ||= listing.truncated === true;
      for (const entry of listing.entries) {
        if (++visited > 3000 || Date.now() >= deadline) { truncated = true; break; }
        if (entry.name === '.' || entry.name === '..' || /[/\\]/.test(entry.name)) continue;
        const path = dir && dir !== '.' ? `${dir}/${entry.name}` : entry.name;
        if (entry.type === 'directory' && !['.git', 'node_modules', '.dart_tool', 'build', '.openmuse-trash'].includes(entry.name)) queue.push(path);
        if (entry.type === 'file') {
          if (path === needle || path.endsWith(`/${needle}`)) await add(path);
          else if (entry.name === p.basename(needle)) {
            try { const file = await checked(ctx, scope, path, signal); if (basenameMatches.size < 20) basenameMatches.set(file.absolutePath, file); else truncated = true; }
            catch (e) { if (!missing(e)) throw e; }
          }
        }
        if (found.size >= 20) { truncated = true; break; }
      }
    }
    truncated ||= queue.length > 0;
    if (found.size === 0) for (const [key, value] of basenameMatches) found.set(key, value);
  }
  const handles = store(ctx), matches = [];
  for (const file of found.values()) {
    const ref = randomUUID();
    const label = p.relative(scope.workspaceRoot, file.absolutePath).replaceAll('\\', '/');
    handles.set(ref, { scope, path: file.absolutePath, label, expires: Date.now() + 30 * 60 * 1000 });
    matches.push({ ref, label });
  }
  while (handles.size > 512) handles.delete(handles.keys().next().value);
  return { matches, truncated, ...(hint.line ? { line: hint.line } : {}), ...(hint.column ? { column: hint.column } : {}) };
}
export async function readSessionFile(ctx, input, signal) {
  const entry = store(ctx).get(input.ref);
  if (!entry) fail('EXPIRED');
  const file = await checked(ctx, entry.scope, entry.path, signal);
  if (file.bytes > 16 * 1024 * 1024) fail('TRANSFER_TOO_LARGE');
  const current = await scopeOf(ctx, entry.scope.sessionId, signal);
  const fs = ctx.get('fs');
  if (current.workspaceRoot !== entry.scope.workspaceRoot || !fs.contains(entry.scope.rootTarget, current.rootTarget) || !fs.contains(current.rootTarget, entry.scope.rootTarget)) fail('DENIED');
  const content = await ctx.get('workspaceFiles').readBytes(entry.scope, entry.path, {}, signal);
  if (content.absolutePath !== file.absolutePath) fail('DENIED');
  await checked(ctx, entry.scope, content.absolutePath, signal);
  const bytes = Buffer.from(content.data);
  if (bytes.length > 16 * 1024 * 1024 || content.eof !== true) fail('TRANSFER_TOO_LARGE');
  const digest = createHash('sha256').update(bytes).digest('hex');
  return { ref: input.ref, label: entry.label, digest, bytes: bytes.toString('base64') };
}
