/**
 * Outbound HTTP for the market's own server-side calls.
 *
 * Node's global `fetch` ignores `HTTP_PROXY` / `HTTPS_PROXY` entirely
 * (measured on Node 25: a request with an unreachable proxy configured still
 * succeeds directly, and setting `NODE_USE_ENV_PROXY` at runtime changes
 * nothing — it is read at startup). On a machine whose route out is a local
 * proxy, that is not a slowdown but a different network: the catalog fetch
 * took 9.9s direct on a reporter's machine, seconds from the 15s timeout,
 * while their proxy sat unused a millisecond away.
 *
 * Every market request therefore calls undici's own fetch and carries a
 * dispatcher this module created. Two measurements say why, and they are
 * not the same fact:
 *
 * - On Node 25, `setGlobalDispatcher` from the undici package does not
 *   steer global fetch. With a dispatcher installed that way, a global
 *   fetch produced no CONNECT at a local proxy, while undici's own fetch
 *   produced `CONNECT awesome-dsh-plugin.com:443`.
 * - On Node 22 the two stacks share one symbol (#742). Global fetch reads
 *   `Symbol.for('undici.globalDispatcher.1')`. The host's first import of
 *   undici 8 (`web_fetch`) finds `.2` empty, installs its dispatcher, and
 *   writes a `Dispatcher1Wrapper` onto `.1`. After that, global fetch
 *   returns gzip bodies with null headers, and `JSON.parse` fails on the
 *   catalog. undici 7's fetch reads `.1` too, so calling it with no
 *   dispatcher fails the same way.
 *
 * The dispatcher is `EnvHttpProxyAgent` when a proxy is configured, and a
 * plain `Agent` otherwise. Only requests made here take it. The host's own
 * networking stays as the host configured it.
 */

import { Agent, EnvHttpProxyAgent, fetch as undiciFetch } from 'undici'

/**
 * The proxy this process would use for the catalog, if any.
 *
 * The standard variables mirror `EnvHttpProxyAgent`'s own resolution
 * deliberately, rather than picking the order that reads best, because the
 * same answer does two jobs: it decides whether the request goes through
 * the proxy agent or the direct one, and it is what the failure message
 * CLAIMS was tried. A helper that
 * named a proxy undici would not have used would put a false statement in
 * every bug report. `npm_config_*` is an additional source on top of that:
 * npm holds its proxy in its own config namespace (a machine set up with
 * `npm config set proxy` has the proxy in `npm_config_proxy` and nowhere
 * else), and undici does not read it — so it is resolved here and handed to
 * the agent explicitly in `marketFetch`.
 *
 * Three details are undici's, not ours (env-http-proxy-agent.js):
 *   - lowercase wins over uppercase (`https_proxy ?? HTTPS_PROXY`)
 *   - an https request falls back to the http proxy when no https one is set
 *   - the value is tested for truthiness, so `HTTPS_PROXY=` — which is how
 *     people turn a proxy off — falls through instead of masking HTTP_PROXY
 *
 * Blank-is-unset is ours, and only widens that last one: undici would hand a
 * whitespace-only value to `new URL()` and throw out of the constructor.
 * Scheme-less host:port values are also common in Windows proxy fields and
 * npm config; proxy agents require URLs, so those default to `http://`.
 */
export function configuredProxy(): string | null {
  const { http, https } = proxyFromEnv()
  return https ?? http
}

/**
 * Proxy URLs resolved from the process environment, standard variables
 * first and npm's own config (`npm_config_https_proxy` / `npm_config_proxy`)
 * as the fallback. npm's config is the second source, not a preference: a
 * standard variable always wins over npm's, and npm's https proxy falls
 * back to npm's http proxy the same way undici's does.
 */
function proxyFromEnv(): { http: string | null; https: string | null } {
  const pick = (raw: string | undefined): string | null => {
    const value = raw?.trim()
    if (value === undefined || value === '') return null
    return /^[a-z][a-z\d+.-]*:\/\//iu.test(value) ? value : `http://${value}`
  }
  const https =
    pick(process.env.https_proxy ?? process.env.HTTPS_PROXY) ??
    pick(process.env.npm_config_https_proxy)
  const http =
    pick(process.env.http_proxy ?? process.env.HTTP_PROXY) ??
    pick(process.env.npm_config_proxy)
  return { http, https }
}

/**
 * Built once and reused: an agent per request would drop connection reuse.
 * The proxy agent also reads NO_PROXY, so a host that excludes its own
 * registry mirror keeps being excluded.
 */
let proxyAgent: EnvHttpProxyAgent | null = null
let directAgent: Agent | null = null

/**
 * Fetch through the proxy this machine is configured to use, or directly
 * through this module's own agent when it has none. Both paths pass the
 * dispatcher described on this module.
 */
export async function marketFetch(
  url: string,
  init?: { signal?: AbortSignal; headers?: Record<string, string> },
): Promise<Response> {
  const { http, https } = proxyFromEnv()
  if (http === null && https === null) {
    directAgent ??= new Agent()
    return await undiciFetch(url, { ...init, dispatcher: directAgent }) as unknown as Response
  }
  // Pass the resolved proxies explicitly. EnvHttpProxyAgent itself reads
  // only http(s)_proxy out of the environment, so a proxy that lives in
  // npm_config_* must be handed over directly — otherwise the agent would
  // silently go direct while configuredProxy() claims a proxy was used.
  proxyAgent ??= new EnvHttpProxyAgent({
    httpProxy: http ?? undefined,
    httpsProxy: https ?? undefined,
  })
  return await undiciFetch(url, { ...init, dispatcher: proxyAgent }) as unknown as Response
}
