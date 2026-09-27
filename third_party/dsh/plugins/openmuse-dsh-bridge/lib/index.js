/** OpenMuse-owned, token-gated bridge for Host-granted local workspaces. */
import { realpath, stat } from 'node:fs/promises';
import { isAbsolute } from 'node:path';

export const inject = [];

const MAX_BODY = 64 * 1024;
function reply(res, status, value) {
  res.writeHead(status, { 'content-type': 'application/json; charset=utf-8', 'cache-control': 'no-store' });
  res.end(JSON.stringify(value));
}
async function bodyOf(req) {
  let body = '';
  for await (const chunk of req) {
    body += chunk;
    if (body.length > MAX_BODY) throw new Error('request too large');
  }
  return JSON.parse(body);
}

export function apply(ctx) {
  const token = process.env.OPENMUSE_DSH_BRIDGE_TOKEN;
  if (typeof token !== 'string' || token.length < 32) return;
  let registered = false;
  const register = (webServer) => {
    if (registered || !webServer?.register) return;
    registered = true;
    ctx.effect(() => webServer.register({
      kind: 'prefix', path: '/openmuse-bridge',
      handler: async (req, res) => {
        if (req.headers['x-openmuse-bridge-token'] !== token) return reply(res, 403, { error: 'forbidden' });
        const path = new URL(req.url ?? '/', 'http://localhost').pathname;
        if (req.method !== 'POST' || path !== '/openmuse-bridge/workspaces') return reply(res, 404, { error: 'not found' });
        const registry = ctx.get('workspaceRegistry');
        if (!registry) return reply(res, 503, { error: 'workspace registry unavailable' });
        try {
          const payload = await bodyOf(req);
          if (!Array.isArray(payload.mounts) || payload.mounts.length > 128 || payload.mounts.some((item) => typeof item !== 'string' || item.length > 4096 || !isAbsolute(item))) {
            return reply(res, 400, { error: 'invalid mounts' });
          }
          const items = [];
          for (const requested of new Set(payload.mounts)) {
            const path = await realpath(requested);
            if (!(await stat(path)).isDirectory()) return reply(res, 400, { error: 'not a directory' });
            const workspace = await registry.resolveByPath(path) ?? await registry.create(path);
            items.push({ workspaceId: String(workspace.id), path: workspace.path });
          }
          // DSH registration is idempotent. Never delete workspaces created by DSH users.
          reply(res, 200, { items });
        } catch (error) {
          reply(res, 400, { error: String(error?.message ?? error) });
        }
      },
    }), 'openmuse-dsh-bridge: routes');
  };
  register(ctx.get('webServer'));
  ctx.on('internal/service', (name, service) => {
    if (name === 'webServer') register(service);
  }, { global: true });
}
