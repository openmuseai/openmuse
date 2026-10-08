// The market client resolves `/dsh-market/…` against document.baseURI. Inside
// the Flutter shell that base is `/app/`, so the catalog request never reaches
// the DSH mount and the static host answers 404.

const marketMarker = '/dsh-market'

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
  const index = url.pathname.indexOf(marketMarker)
  if (url.origin !== documentUrl.origin || index < 0) return input
  const target = new URL(url.pathname.slice(index).replace(/^\//, ''), assetBase)
  target.search = url.search
  target.hash = url.hash
  if (target.href === url.href) return input
  if (typeof input === 'string' || input instanceof URL) return target.href
  return new Request(target, input)
}
