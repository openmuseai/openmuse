// Embedded extension bridges must resolve against the authenticated DSH mount,
// rather than the Flutter page's /app/ base or the public origin root.

const bridgeMarkers = ['/dsh-market', '/model-capabilities']

export function rewriteEmbeddedMarketRequest(input, assetBase, documentBase) {
  const raw = typeof input === 'string'
    ? input
    : input instanceof URL
      ? input.href
      : input.url
  let url
  try {
    url = new URL(raw, documentBase)
  } catch {
    return input
  }
  const documentUrl = new URL(documentBase)
  const index = bridgeMarkers.reduce((found, marker) => {
    if (found >= 0) return found
    const at = url.pathname.indexOf(marker)
    const rest = at < 0 ? '' : url.pathname.slice(at + marker.length)
    return at >= 0 && (rest === '' || rest.startsWith('/')) ? at : -1
  }, -1)
  if (url.origin !== documentUrl.origin || index < 0) return input
  const target = new URL(url.pathname.slice(index).replace(/^\//, ''), assetBase)
  target.search = url.search
  target.hash = url.hash
  if (target.href === url.href) return input
  if (typeof input === 'string' || input instanceof URL) return target.href
  return new Request(target, input)
}
