import http from 'node:http';
import {readFileSync, statSync} from 'node:fs';
import {createHash, createPrivateKey, timingSafeEqual} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {ControlError, ControlStore} from './store.mjs';
import {https, registry, validateDraft} from './contracts.mjs';

const roleActions = {
  reader: [], editor: ['draft.create', 'draft.update', 'draft.submit', 'draft.rollback'],
  approver: ['draft.approve'], publisher: ['release.publish', 'release.rollout'], admin: ['scope.create'],
};
export const demoActors = [
  {id: 'demo-editor', roles: ['reader', 'editor']}, {id: 'demo-reviewer', roles: ['reader', 'approver']},
  {id: 'demo-publisher', roles: ['reader', 'publisher']}, {id: 'demo-admin', roles: ['reader', 'admin']},
];
const tokenHash = token => createHash('sha256').update(token).digest();
function authenticate(req, {demo, principals}) {
  if (demo) {
    const actor = demoActors.find(a => a.id === req.headers['x-demo-actor']);
    if (actor) return {...actor, scopes: ['*']};
  } else {
    const header = req.headers.authorization || '';
    if (header.startsWith('Bearer ')) {
      const candidate = tokenHash(header.slice(7));
      const actor = principals.find(p => timingSafeEqual(candidate, tokenHash(p.token)));
      if (actor) return {id: actor.id, roles: actor.roles, scopes: actor.scopes};
    }
  }
  throw new ControlError(401, 'Authentication required');
}
function authorize(actor, action) {
  if (!actor.roles.some(role => roleActions[role]?.includes(action))) throw new ControlError(403, 'Role does not allow this action');
}
function checkScope(actor, scope) {
  if (!actor.scopes.includes('*') && !actor.scopes.includes(scope)) throw new ControlError(403, 'Deployment scope is not authorized');
}
async function body(req) {
  if (!String(req.headers['content-type'] ?? '').startsWith('application/json')) throw new ControlError(415, 'Expected application/json');
  const parts = []; let size = 0;
  for await (const chunk of req) { size += chunk.length; if (size > 5 * 1024 * 1024) throw new ControlError(413, 'Request is too large'); parts.push(chunk); }
  try { return JSON.parse(Buffer.concat(parts).toString('utf8')); } catch { throw new ControlError(400, 'Invalid JSON'); }
}
export function createControlServer({store, localOrigin, demo = false, principals = []}) {
  const local = new URL(localOrigin);
  if (local.username || local.password || local.search || local.hash || local.pathname !== '/') throw Error('Admin origin must not contain credentials, path, query or fragment');
  if (!demo && local.protocol !== 'https:' && !(local.protocol === 'http:' && ['127.0.0.1','[::1]'].includes(local.hostname))) throw Error('Non-loopback admin access requires an HTTPS ingress origin');
  if (demo && !['127.0.0.1', '[::1]'].includes(local.hostname)) throw Error('Demo mode requires a loopback origin');
  if (!demo && (!principals.length || principals.some(p => !p.id || typeof p.token !== 'string' || p.token.length < 32 || !Array.isArray(p.roles) || p.roles.some(r => !Object.hasOwn(roleActions, r)) || !Array.isArray(p.scopes) || !p.scopes.length || p.scopes.some(s => s !== '*' && !/^[a-z0-9][a-z0-9-]{0,63}$/.test(s))))) throw Error('Configure principals with strong tokens, explicit roles and deployment scopes');
  if (!demo && (new Set(principals.map(p=>p.id)).size !== principals.length || new Set(principals.map(p=>p.token)).size !== principals.length)) throw Error('Principal IDs and tokens must be unique');
  const staticFiles = new Map([['/', ['index.html', 'text/html']], ['/admin.js', ['admin.js', 'text/javascript']], ['/admin.css', ['admin.css', 'text/css']]]);
  const server = http.createServer(async (req, res) => {
    let publicRequest = false;
    const send = (status, payload, type = 'application/json', headers = {}) => {
      res.writeHead(status, {'content-type': `${type}; charset=utf-8`, 'x-content-type-options': 'nosniff', 'referrer-policy': 'no-referrer', 'cache-control': 'no-store', ...headers});
      res.end(type === 'application/json' && !Buffer.isBuffer(payload) ? JSON.stringify(payload) : payload);
    };
    try {
      const runtimePath = req.url.startsWith('/v1/runtime/');
      publicRequest = runtimePath;
      const allowedHost = runtimePath ? [local.host, new URL(store.publicOrigin).host] : [local.host];
      if (!allowedHost.includes(req.headers.host)) throw new ControlError(421, 'Unexpected host');
      if (!req.url.startsWith('/v1/runtime/') && req.headers.origin && req.headers.origin !== local.origin) throw new ControlError(403, 'Cross-origin requests are not allowed');
      const url = new URL(req.url, localOrigin), path = url.pathname;
      if (url.search) throw new ControlError(400, 'Query parameters are not supported');
      if (req.method === 'GET' && staticFiles.has(path)) {
        const [name, type] = staticFiles.get(path);
        return send(200, readFileSync(new URL(`../public/${name}`, import.meta.url)), type, {'referrer-policy': 'no-referrer', 'content-security-policy': "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' https:; media-src https:; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'"});
      }
      if (req.method === 'GET' && path === '/admin-messages.json') return send(200, {metadata: registry.metadata, entries: Object.fromEntries(Object.entries(registry.entries).filter(([_, v]) => v.namespace === 'admin'))});
      if (req.method === 'GET' && path === '/healthz') return send(200, {ok: true});
      if (req.method === 'GET' && path === '/v1/admin/session') {
        const actor = demo ? null : authenticate(req, {demo, principals});
        return send(200, {demo, actors: demo ? demoActors : [], actor, publicOrigin: store.publicOrigin, kid: store.kid, publicKey: store.publicKey});
      }
      let match;
      if (req.method === 'GET' && (match = path.match(/^\/v1\/runtime\/scopes\/([a-z0-9-]+)\/active(?:\/(\d{1,3}))?\.jws$/))) {
        publicRequest = true;
        const release = store.active(match[1], match[2] === undefined ? null : Number(match[2]));
        // Short pointer caching; immutable resources and releases can use a CDN.
        return send(200, release.jws, 'application/jose', {'cache-control': 'public, max-age=15, must-revalidate', 'etag': `"${release.id}"`, 'access-control-allow-origin': '*'});
      }
      if (req.method === 'GET' && (match = path.match(/^\/v1\/runtime\/resources\/([a-f0-9]{64})\.json$/))) {
        publicRequest = true;
        return send(200, store.resource(match[1]), 'application/json', {'cache-control': 'public, max-age=31536000, immutable', 'etag': `"${match[1]}"`, 'access-control-allow-origin': '*'});
      }
      if (req.method === 'GET' && (match = path.match(/^\/v1\/runtime\/releases\/([a-f0-9-]{36})\.jws$/))) {
        publicRequest = true;
        return send(200, store.release(match[1]).jws, 'application/jose', {'cache-control': 'public, max-age=31536000, immutable', 'access-control-allow-origin': '*'});
      }
      if (runtimePath) throw new ControlError(404, 'Distribution route not found');
      const actor = authenticate(req, {demo, principals});
      if (req.method === 'GET' && path === '/v1/admin/catalog') return send(200, registry);
      if (req.method === 'GET' && path === '/v1/admin/scopes') return send(200, store.listScopes().filter(s=>actor.scopes.includes('*') || actor.scopes.includes(s.id)));
      if (req.method === 'POST' && path === '/v1/admin/scopes') {
        authorize(actor, 'scope.create'); const b = await body(req); checkScope(actor,b.id); return send(201, store.createScope(b.id, b.bundle, actor.id));
      }
      if ((match = path.match(/^\/v1\/admin\/scopes\/([a-z0-9-]+)(?:\/(drafts|releases|audit|rollout|validate|rollback|template))?$/))) {
        const id = match[1], action = match[2];
        checkScope(actor,id);
        if (req.method === 'GET') {
          if (!action) return send(200, store.scope(id));
          if (action === 'drafts') return send(200, store.listDrafts(id));
          if (action === 'releases') return send(200, store.listReleases(id));
          if (action === 'audit') return send(200, store.audits(id));
          if (action === 'template') return send(200, store.template(id));
        }
        if (req.method === 'POST') {
          const b = await body(req);
          if (action === 'validate') { validateDraft(b, store.scope(id).bundle); return send(200, {valid: true}); }
          if (action === 'drafts') { authorize(actor, 'draft.create'); return send(201, store.createDraft(id, b, actor.id)); }
          if (action === 'rollout') { authorize(actor, 'release.rollout'); return send(200, store.rollout(id, b.percent, b.revision, actor.id)); }
          if (action === 'rollback') { authorize(actor, 'draft.rollback'); return send(201, store.rollback(id, b.releaseId, actor.id)); }
        }
      }
      if ((match = path.match(/^\/v1\/admin\/drafts\/([a-f0-9-]{36})(?:\/(submit|approve|publish))?$/))) {
        const id = match[1], action = match[2];
        checkScope(actor,store.draft(id).scope);
        if (req.method === 'GET' && !action) return send(200, store.draft(id));
        if (req.method === 'PUT' && !action) { authorize(actor, 'draft.update'); const b = await body(req); return send(200, store.updateDraft(id, b.version, b.body, actor.id)); }
        if (req.method === 'POST' && action) {
          authorize(actor, action === 'publish' ? 'release.publish' : `draft.${action}`); const b = await body(req);
          if (action === 'submit') return send(200, store.submit(id, b.version, actor.id));
          if (action === 'approve') return send(200, store.approve(id, b.hash, actor.id));
          if (action === 'publish') return send(200, store.publish(id, b.hash, b.revision, actor.id, b.ttlSeconds));
        }
      }
      if (req.method === 'GET' && (match = path.match(/^\/v1\/admin\/releases\/([a-f0-9-]{36})$/))) { const release = store.release(match[1]); checkScope(actor,release.scope); return send(200,release); }
      throw new ControlError(404, 'Route not found');
    } catch (error) {
      const status = error instanceof ControlError ? error.status : 400;
      // Never echo request bodies, tokens, signing material, or SQL errors.
      send(status, {error: error instanceof ControlError || !/SQLITE|constraint|database/i.test(error.message) ? error.message : 'Configuration operation failed'}, 'application/json', publicRequest ? {'access-control-allow-origin': '*'} : {});
    }
  });
  server.requestTimeout = 15000; server.headersTimeout = 10000;
  return server;
}

export async function startFromEnvironment(env = process.env) {
  const required = name => { if (!env[name]) throw Error(`Missing ${name}`); return env[name]; };
  const keyFile = required('OPENMUSE_CONFIG_SIGNING_KEY_FILE');
  if ((statSync(keyFile).mode & 0o077) !== 0) throw Error('Signing key file must be readable only by its owner');
  const privateKey = createPrivateKey(readFileSync(keyFile));
  if (privateKey.asymmetricKeyType !== 'ed25519') throw Error('Expected Ed25519 signing key');
  const publicOrigin = https(required('OPENMUSE_CONFIG_PUBLIC_ORIGIN'), true).origin;
  const localOrigin = required('OPENMUSE_CONFIG_ADMIN_ORIGIN'), local = new URL(localOrigin);
  const principalsFile = required('OPENMUSE_CONFIG_PRINCIPALS_FILE');
  if ((statSync(principalsFile).mode & 0o077) !== 0) throw Error('Principal secret file must be readable only by its owner');
  const principals = JSON.parse(readFileSync(principalsFile, 'utf8'));
  const store = new ControlStore({database: required('OPENMUSE_CONFIG_DATABASE'), privateKey, kid: required('OPENMUSE_CONFIG_SIGNING_KEY_ID'), publicOrigin});
  const server = createControlServer({store, localOrigin, principals});
  await new Promise((resolve,reject) => { server.once('error',reject); server.listen(Number(env.OPENMUSE_CONFIG_LISTEN_PORT ?? (local.port || 8790)), env.OPENMUSE_CONFIG_BIND ?? '127.0.0.1', resolve); });
  return {server, store};
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  startFromEnvironment().then(({server, store}) => {
    console.log(`Configuration service listening on ${server.address().address}:${server.address().port}`);
    process.on('SIGTERM', () => server.close(() => { store.close(); process.exit(0); }));
  }).catch(error => { console.error(error.message); process.exitCode = 1; });
}
