// Device E2E fixture for the paired Desktop transport. Production uses the
// Dart PairedDesktopGateway; this Node adapter avoids adb reverse + Dart
// HttpServer transport stalls while exercising the same trust boundaries.
import crypto from 'node:crypto';
import http from 'node:http';
import net from 'node:net';

const required = (name) => {
  const value = process.env[name];
  if (!value) throw new Error(`${name} is required`);
  return value;
};

const accountRef = required('OPENMUSE_DESKTOP_ACCOUNT_REF');
const gotrueOrigin = new URL(required('OPENMUSE_GOTRUE_ORIGIN'));
const dshEndpoint = new URL(required('OPENMUSE_DSH_ENDPOINT'));
const pairingCode = required('OPENMUSE_PAIRED_DESKTOP_PAIRING_CODE');
const port = Number(process.env.OPENMUSE_GATEWAY_PORT ?? '13180');
const grants = new Map();
let pairingArmed = true;

if (dshEndpoint.protocol !== 'http:' || !['127.0.0.1', 'localhost', '::1'].includes(dshEndpoint.hostname)) {
  throw new Error('OPENMUSE_DSH_ENDPOINT must be loopback HTTP');
}

const json = (response, statusCode, value) => {
  const body = Buffer.from(JSON.stringify(value));
  response.writeHead(statusCode, {
    'content-type': 'application/json; charset=utf-8',
    'content-length': body.length,
    connection: 'close',
  });
  response.end(body);
};

const cookies = (header = '') => Object.fromEntries(
  header.split(';').map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf('=');
    return at < 0 ? [part, ''] : [part.slice(0, at), part.slice(at + 1)];
  }),
);

const readBody = (request, limit = 16 * 1024) => new Promise((resolve, reject) => {
  const chunks = [];
  let length = 0;
  request.on('data', (chunk) => {
    length += chunk.length;
    if (length > limit) {
      reject(Object.assign(new Error('request too large'), {statusCode: 413, code: 'REQUEST_TOO_LARGE'}));
      request.destroy();
      return;
    }
    chunks.push(chunk);
  });
  request.on('end', () => resolve(Buffer.concat(chunks)));
  request.on('error', reject);
});

const validateMobileAccount = async (accessToken) => {
  const response = await fetch(new URL('/user', gotrueOrigin), {
    headers: {authorization: `Bearer ${accessToken}`},
    signal: AbortSignal.timeout(10_000),
  });
  if (!response.ok) throw Object.assign(new Error('token rejected'), {statusCode: 401, code: 'UNAUTHENTICATED'});
  const user = await response.json();
  if (typeof user.id !== 'string' || !user.id) throw new Error('invalid GoTrue user');
  return user.id;
};

const findGrant = (requestUrl, requestCookies) => {
  const match = requestUrl.pathname.match(/^\/u\/([a-f0-9]{64})$/);
  const ref = match?.[1] ?? requestCookies['OpenMuse-Paired'];
  const grant = ref ? grants.get(ref) : undefined;
  if (!grant || grant.expiresAtMs <= Date.now()) return undefined;
  return {grant, initial: Boolean(match)};
};

const upstreamCookie = (requestCookies) => Object.entries(requestCookies)
  .filter(([name]) => name !== 'OpenMuse-Paired')
  .map(([name, value]) => `${name}=${value}`)
  .join('; ');

const proxyHttp = (request, response, grantMatch) => {
  let stage = 'parse-incoming';
  try {
  const {grant, initial} = grantMatch;
  const incomingUrl = new URL(request.url, 'http://paired.invalid');
  stage = 'build-target';
  const upstreamBase = new URL(grant.upstream);
  const target = initial
    ? upstreamBase
    : new URL(`${incomingUrl.pathname}${incomingUrl.search}`, upstreamBase.origin);
  stage = 'prepare-headers';
  const requestCookies = cookies(request.headers.cookie);
  const headers = {...request.headers, host: target.host};
  delete headers.authorization;
  delete headers.connection;
  delete headers['content-length'];
  const cookie = upstreamCookie(requestCookies);
  if (cookie) headers.cookie = cookie;
  else delete headers.cookie;
  if (headers.origin) headers.origin = target.origin;
  if (headers.referer) headers.referer = `${target.origin}/`;

  stage = 'open-upstream';
  const outbound = http.request(target, {method: request.method, headers}, (upstream) => {
    const outgoingHeaders = {...upstream.headers};
    delete outgoingHeaders.connection;
    delete outgoingHeaders['transfer-encoding'];
    delete outgoingHeaders['content-length'];
    if (outgoingHeaders.location) {
      const resolved = new URL(outgoingHeaders.location, target);
      if (resolved.origin !== target.origin) {
        json(response, 502, {code: 'CROSS_ORIGIN_REDIRECT', message: 'Desktop DSH 返回了不安全的跳转。'});
        upstream.resume();
        return;
      }
      outgoingHeaders.location = `${resolved.pathname}${resolved.search}`;
    }
    if (initial) {
      const existing = outgoingHeaders['set-cookie'] ?? [];
      outgoingHeaders['set-cookie'] = [
        ...existing,
        `OpenMuse-Paired=${grant.grantRef}; Path=/; HttpOnly; SameSite=Strict; Max-Age=1800`,
      ];
    }
    response.writeHead(upstream.statusCode ?? 502, outgoingHeaders);
    upstream.pipe(response);
  });
  outbound.setTimeout(15_000, () => outbound.destroy(new Error('upstream timeout')));
  outbound.on('error', () => {
    if (!response.headersSent) json(response, 502, {code: 'PAIRED_DESKTOP_UNAVAILABLE', message: 'Desktop transport 暂时不可用。'});
    else response.destroy();
  });
  request.pipe(outbound);
  } catch (error) {
    process.stderr.write(`paired gateway proxy failure at ${stage}: ${error.code ?? error.name}\n`);
    throw error;
  }
};

const server = http.createServer(async (request, response) => {
  try {
    const requestUrl = new URL(request.url, 'http://paired.invalid');
    if (request.method === 'GET' && requestUrl.pathname === '/v1/status') {
      json(response, 200, {ready: true, pairingArmed});
      return;
    }
    if (request.method === 'POST' && requestUrl.pathname === '/v1/pair/open') {
      const authorization = request.headers.authorization ?? '';
      if (!authorization.startsWith('Bearer ')) {
        json(response, 401, {code: 'UNAUTHENTICATED', message: '请先登录。'});
        return;
      }
      const body = JSON.parse((await readBody(request)).toString('utf8'));
      if (!pairingArmed || body.pairingCode !== pairingCode) {
        json(response, 403, {code: 'PAIRING_CODE_DENIED', message: '配对码无效或已过期。'});
        return;
      }
      if (typeof body.deviceRef !== 'string' || !body.deviceRef || body.workspaceRef !== 'openmuse.local.default') {
        json(response, 403, {code: 'WORKSPACE_GRANT_DENIED', message: 'Desktop Workspace 未授权。'});
        return;
      }
      if (await validateMobileAccount(authorization.slice(7)) !== accountRef) {
        json(response, 403, {code: 'ACCOUNT_MISMATCH', message: 'Mobile 与 Desktop 必须登录同一个账号。'});
        return;
      }
      const grantRef = crypto.randomBytes(32).toString('hex');
      const expiresAtMs = Date.now() + 30 * 60 * 1000;
      grants.set(grantRef, {grantRef, expiresAtMs, upstream: dshEndpoint.toString()});
      pairingArmed = false;
      const host = `${request.socket.localAddress ?? '127.0.0.1'}:${port}`;
      json(response, 200, {
        accountRef,
        deviceRef: body.deviceRef,
        workspaceRef: 'openmuse.local.default',
        workspaceTitle: 'Project Workspace',
        grantRef,
        expiresAtMs,
        session: {
          sessionRef: `paired-dsh:${grantRef}`,
          origin: `http://${host}`,
          path: `/u/${grantRef}`,
          generation: 1,
          allowInsecureLoopback: true,
        },
      });
      return;
    }
    const grantMatch = findGrant(requestUrl, cookies(request.headers.cookie));
    if (!grantMatch) {
      json(response, 401, {code: 'GRANT_REQUIRED', message: 'Paired Desktop grant 无效或已过期。'});
      return;
    }
    proxyHttp(request, response, grantMatch);
  } catch (error) {
    const statusCode = error.statusCode ?? 502;
    json(response, statusCode, {code: error.code ?? 'PAIRED_DESKTOP_UNAVAILABLE', message: 'Desktop transport 暂时不可用。'});
  }
});

server.on('upgrade', (request, socket, head) => {
  const requestUrl = new URL(request.url, 'http://paired.invalid');
  const requestCookies = cookies(request.headers.cookie);
  const grantMatch = findGrant(requestUrl, requestCookies);
  if (!grantMatch) {
    socket.end('HTTP/1.1 401 Unauthorized\r\nConnection: close\r\n\r\n');
    return;
  }
  const target = new URL(`${requestUrl.pathname}${requestUrl.search}`, grantMatch.grant.upstream);
  const upstream = net.connect(Number(target.port || 80), target.hostname, () => {
    const headers = {...request.headers, host: target.host};
    const cookie = upstreamCookie(requestCookies);
    if (cookie) headers.cookie = cookie;
    else delete headers.cookie;
    if (headers.origin) headers.origin = target.origin;
    if (headers.referer) headers.referer = `${target.origin}/`;
    const lines = [`${request.method} ${target.pathname}${target.search} HTTP/${request.httpVersion}`];
    for (const [name, value] of Object.entries(headers)) {
      if (value !== undefined) lines.push(`${name}: ${Array.isArray(value) ? value.join(', ') : value}`);
    }
    upstream.write(`${lines.join('\r\n')}\r\n\r\n`);
    if (head.length) upstream.write(head);
    socket.pipe(upstream).pipe(socket);
  });
  upstream.on('error', () => socket.destroy());
});

server.listen(port, '127.0.0.1', () => process.stdout.write('PAIRED_DESKTOP_GATEWAY_READY\n'));
