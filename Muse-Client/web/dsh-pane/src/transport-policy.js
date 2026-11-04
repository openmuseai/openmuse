// Only the account-authorized Desktop mount represents the signed-in user's
// own Host. Generic DSH deployments retain their remote/read-only UI policy.
// This enables the Host settings UI; the paired cookie and DSH session still
// authorize every RPC on the server, including settings and credentials.
export function ownsPairedHost(path, documentUrl, pageOrigin) {
  const url = new URL(documentUrl)
  return url.origin === pageOrigin &&
    url.pathname.startsWith('/openmuse/dsh/') &&
    (path === '/openmuse/dsh/' || /^\/u\/[a-fA-F0-9]{64}$/.test(path))
}
