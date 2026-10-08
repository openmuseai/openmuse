// Local vertical slice only. Production needs account/session authorization,
// CSRF policy, audited DSH bootstrap and the paired E2E transport.
import { createServer, request as httpRequest } from 'node:http'
import { request as httpsRequest } from 'node:https'
import { createReadStream } from 'node:fs'
import { stat } from 'node:fs/promises'
import { dirname, extname, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const buildRoot = resolve(process.env.OPENMUSE_WEB_BUILD ?? resolve(repoRoot, 'app/openmuse_web/build/web'))
const upstreamText = process.env.OPENMUSE_DSH_ORIGIN
if (!upstreamText) throw new Error('Set OPENMUSE_DSH_ORIGIN to the local DSH web origin')
const upstream = new URL(upstreamText)
const productionE2e = process.env.OPENMUSE_E2E_PRODUCTION === '1'
const productionRelay = new URL('https://openmuseai.com:8443')
const productionCloud = new URL('https://openmuseai.com')
const localGateway = new URL(process.env.OPENMUSE_PAIRED_GATEWAY_ORIGIN ?? 'http://127.0.0.1:13180')
if (localGateway.protocol !== 'http:' || !['127.0.0.1', 'localhost', '[::1]'].includes(localGateway.hostname)) {
  throw new Error('The local paired gateway must be a loopback HTTP origin')
}
if (productionE2e) {
  if (upstream.origin !== productionRelay.origin) throw new Error('Production E2E relay origin mismatch')
} else if (upstream.protocol !== 'http:' || !['127.0.0.1', 'localhost', '[::1]'].includes(upstream.hostname)) {
  throw new Error('The dev edge accepts only a loopback HTTP DSH origin')
}
const port = Number(process.env.OPENMUSE_WEB_PORT ?? 4174)
if (!Number.isInteger(port) || port < 1 || port > 65535) throw new Error('Invalid OPENMUSE_WEB_PORT')
const allowedHosts = new Set([`127.0.0.1:${port}`, `localhost:${port}`])

function allowedBrowserRequest(req) {
  const host = req.headers.host
  if (!allowedHosts.has(host)) return false
  return req.headers.origin === undefined || req.headers.origin === `http://${host}`
}

const types = {
  '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8', '.json': 'application/json; charset=utf-8',
  '.svg': 'image/svg+xml', '.png': 'image/png', '.ico': 'image/x-icon',
  '.webp': 'image/webp', '.wasm': 'application/wasm', '.otf': 'font/otf',
  '.ttf': 'font/ttf', '.woff2': 'font/woff2',
}

function dshTarget(url) {
  const destination = url.pathname.startsWith('/gotrue/') ||
    url.pathname.startsWith('/api/muse/devices')
    ? productionCloud
    : !productionE2e && !url.pathname.startsWith('/dsh/')
      ? localGateway : upstream
  const target = new URL(destination.origin)
  target.pathname = !productionE2e && url.pathname.startsWith('/dsh/')
    ? url.pathname.slice('/dsh'.length)
    : url.pathname
  target.search = url.search
  return target
}

function upstreamHeaders(req, target) {
  const headers = { ...req.headers, host: target.host }
  const browserOrigin = `http://${req.headers.host}`
  if (headers.origin === browserOrigin) headers.origin = target.origin
  if (typeof headers.referer === 'string' && headers.referer.startsWith(browserOrigin)) {
    headers.referer = `${target.origin}${headers.referer.slice(browserOrigin.length)}`
  }
  return headers
}

function proxy(req, res, url) {
  const target = dshTarget(url)
  const request = target.protocol === 'https:' ? httpsRequest : httpRequest
  const outgoing = request(target, {
    method: req.method,
    headers: upstreamHeaders(req, target),
  }, incoming => {
    const headers = { ...incoming.headers }
    if (typeof headers.location === 'string') {
      const redirect = new URL(headers.location, target)
      if (redirect.origin === target.origin) {
        headers.location = `${url.protocol}//${req.headers.host}${redirect.pathname}${redirect.search}${redirect.hash}`
      }
    }
    res.writeHead(incoming.statusCode ?? 502, headers)
    incoming.pipe(res)
  })
  outgoing.on('error', error => {
    if (!res.headersSent) res.writeHead(502, { 'content-type': 'text/plain; charset=utf-8' })
    res.end(`Local DSH unavailable: ${error.message}`)
  })
  req.pipe(outgoing)
}

async function serveFlutter(req, res, url) {
  if (req.method !== 'GET' && req.method !== 'HEAD') {
    res.writeHead(405).end()
    return
  }
  let name
  try { name = decodeURIComponent(url.pathname.slice('/app/'.length)) }
  catch { res.writeHead(400).end(); return }
  const filename = resolve(buildRoot, name || 'index.html')
  const within = relative(buildRoot, filename)
  if (within.startsWith('..') || within.startsWith('/')) {
    res.writeHead(403).end()
    return
  }
  let info
  try { info = await stat(filename) }
  catch { res.writeHead(404).end(); return }
  if (!info.isFile()) { res.writeHead(404).end(); return }
  res.writeHead(200, {
    'content-type': types[extname(filename)] ?? 'application/octet-stream',
    'cache-control': 'no-store',
  })
  if (req.method === 'HEAD') res.end()
  else createReadStream(filename).pipe(res)
}

const server = createServer((req, res) => {
  if (!allowedBrowserRequest(req)) { res.writeHead(403).end(); return }
  const url = new URL(req.url ?? '/', 'http://local.invalid')
  if (url.pathname === '/app' || url.pathname === '/dsh') {
    res.writeHead(308, { location: `${url.pathname}/${url.search}` }).end()
  } else if (url.pathname.startsWith('/app/')) {
    void serveFlutter(req, res, url).catch(error => {
      if (!res.headersSent) res.writeHead(500)
      res.end(error.message)
    })
  } else {
    proxy(req, res, url)
  }
})

server.on('upgrade', (req, socket, head) => {
  if (!allowedBrowserRequest(req)) { socket.destroy(); return }
  const url = new URL(req.url ?? '/', 'http://local.invalid')
  if (url.pathname.startsWith('/app/')) { socket.destroy(); return }
  const target = dshTarget(url)
  const request = target.protocol === 'https:' ? httpsRequest : httpRequest
  const outgoing = request(target, { method: req.method, headers: upstreamHeaders(req, target) })
  outgoing.on('upgrade', (response, upstreamSocket, upstreamHead) => {
    const lines = [`HTTP/${response.httpVersion} ${response.statusCode} ${response.statusMessage}`]
    for (const [key, value] of Object.entries(response.headers)) {
      if (value !== undefined) lines.push(`${key}: ${value}`)
    }
    socket.write(`${lines.join('\r\n')}\r\n\r\n`)
    if (upstreamHead.length) socket.write(upstreamHead)
    if (head.length) upstreamSocket.write(head)
    upstreamSocket.pipe(socket).pipe(upstreamSocket)
  })
  outgoing.on('response', response => {
    socket.end(`HTTP/1.1 ${response.statusCode ?? 502} ${response.statusMessage}\r\n\r\n`)
  })
  outgoing.on('error', () => socket.destroy())
  outgoing.end()
})

server.listen(port, '127.0.0.1', () => {
  console.log(`OpenMuse Web dev edge: http://127.0.0.1:${port}/app/`)
  console.log(`DSH Web (direct page): http://127.0.0.1:${port}/dsh/`)
})
