import { AppWebEntry } from '@deepseek-ai/dsh-client-web'

import { rewriteEmbeddedMarketRequest } from './market-fetch.js'

let bootstrap
let bootstrapPath
let activeEntry
let activeContainer
let pendingContainer
let mountToken = 0
let styleReady
let marketAssetBase
let nativeFetch

function installMarketFetchRewrite() {
  if (nativeFetch) return
  nativeFetch = globalThis.fetch.bind(globalThis)
  globalThis.fetch = (input, init) => {
    if (!marketAssetBase) return nativeFetch(input, init)
    const rewritten = rewriteEmbeddedMarketRequest(
      input,
      marketAssetBase,
      document.baseURI,
    )
    if (rewritten === input) return nativeFetch(input, init)
    return nativeFetch(rewritten, {
      ...init,
      credentials: init?.credentials ?? 'include',
    })
  }
}

async function installStyle() {
  if (styleReady) return styleReady
  styleReady = new Promise((resolve, reject) => {
    const link = document.createElement('link')
    link.rel = 'stylesheet'
    link.href = new URL('./dsh-pane.css', import.meta.url).href
    link.onload = resolve
    link.onerror = () => reject(new Error('DSH 样式加载失败。'))
    document.head.append(link)
  })
  return styleReady
}

async function installDshBootstrap(path) {
  if (bootstrap && bootstrapPath === path) return bootstrap
  bootstrapPath = path
  bootstrap = (async () => {
    if (path !== '/dsh/' && !/^\/u\/[a-fA-F0-9]{64}$/.test(path)) {
      throw new Error('DSH 会话路径无效。')
    }
    const response = await fetch(path, { credentials: 'include' })
    if (!response.ok) {
      throw new Error(
        response.status === 401 || response.status === 403
          ? '请先完成 DSH 身份验证。'
          : `DSH 启动失败：HTTP ${response.status}`,
      )
    }
    const html = new DOMParser().parseFromString(await response.text(), 'text/html')
    const documentUrl = new URL(response.url, location.href)
    const baseHref = html.querySelector('base[href]')?.getAttribute('href')
    const assetBase = new URL(baseHref || '.', documentUrl)
    if (assetBase.origin !== location.origin) {
      throw new Error('DSH 资源来源无效。')
    }
    // Desktop DSH addresses itself from the origin root. On this host that
    // root is the web app, so the bootstrap redirect is mounted at
    // /openmuse/dsh and every root-absolute URL has to stay there.
    const dshMount = documentUrl.pathname.startsWith('/openmuse/dsh/')
      ? '/openmuse/dsh/'
      : ''
    const sameOriginAsset = (reference, kind) => {
      const url = new URL(reference, assetBase)
      if (url.origin !== location.origin || !['http:', 'https:'].includes(url.protocol)) {
        throw new Error(`DSH ${kind}来源无效。`)
      }
      if (dshMount && !url.pathname.startsWith(dshMount)) {
        url.pathname = dshMount + url.pathname.replace(/^\//, '')
      }
      return url.href
    }
    for (const element of [...html.head.children, ...html.body.children]) {
      if (element.tagName === 'STYLE') {
        document.head.append(element.cloneNode(true))
      } else if (element.tagName === 'LINK' &&
          element.getAttribute('rel') === 'stylesheet') {
        const href = element.getAttribute('href')
        if (href) {
          const link = document.createElement('link')
          link.rel = 'stylesheet'
          link.href = sameOriginAsset(href, '样式')
          document.head.append(link)
        }
      } else if (element.tagName === 'SCRIPT' &&
          element.getAttribute('type') !== 'module') {
        const source = element.getAttribute('src')
        if (source) {
          await new Promise((resolve, reject) => {
            const script = document.createElement('script')
            script.src = sameOriginAsset(source, '脚本')
            script.onload = resolve
            script.onerror = () => reject(new Error('DSH 启动脚本加载失败。'))
            document.head.append(script)
          })
        } else {
          const script = document.createElement('script')
          script.textContent = element.textContent
          document.head.append(script)
        }
      }
    }
    if (!globalThis.__DSH_BOOT__ || !globalThis.__ModuleLoader__) {
      throw new Error('DSH 启动清单不完整。')
    }
    // Plugin URLs are relative to the DSH document, not the Flutter page.
    for (const entry of globalThis.__DSH_BOOT__.entries ?? []) {
      entry.url = sameOriginAsset(entry.url, '插件')
    }
    for (const batch of globalThis.__DSH_BOOT__.batches ?? []) {
      batch.url = sameOriginAsset(batch.url, '插件')
    }
    marketAssetBase = assetBase.href
    installMarketFetchRewrite()
    globalThis.__DSH_TRANSPORT__ = {
      ...globalThis.__DSH_TRANSPORT__,
      fetch: (input, init) => nativeFetch(sameOriginAsset(input, '接口'), {
        ...init,
        credentials: 'include',
      }),
      streamBaseUrl: assetBase.href,
    }
  })().catch((error) => {
    if (bootstrapPath === path) {
      bootstrap = undefined
      bootstrapPath = undefined
    }
    throw error
  })
  return bootstrap
}

export async function mount(container, path = '/dsh/') {
  if (!(container instanceof HTMLElement)) {
    throw new Error('DSH 面板容器无效。')
  }
  const token = ++mountToken
  pendingContainer = container
  await installDshBootstrap(path)
  if (token !== mountToken) return
  await installStyle()
  if (token !== mountToken) return
  if (activeEntry) await activeEntry.dispose()
  if (token !== mountToken) return
  activeContainer = container
  pendingContainer = undefined
  activeEntry = new AppWebEntry(container)
  await activeEntry.run()
}

export async function unmount(container) {
  if (activeContainer !== container && pendingContainer !== container) return
  ++mountToken
  activeContainer = undefined
  pendingContainer = undefined
  const entry = activeEntry
  activeEntry = undefined
  if (entry) await entry.dispose()
}
