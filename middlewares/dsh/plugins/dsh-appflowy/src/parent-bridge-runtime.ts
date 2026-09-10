export const FRAME_SOURCE = "muse.dsh-web";

/**
 * Same-origin `/u/<hash>/` (and `/dsh/`) must not load root-absolute `/assets/*`
 * or `/muse/v1/*` off the Cloud apex — those 404 / 307 to the marketing site.
 * Relative `./…` resolves against the iframe URL instead.
 */
const EMBEDDED_ROOT = /(["'(])\/(?!u\/[a-f0-9]{32}\/)(assets|plugins|muse)\//g;
const EMBEDDED_MANIFEST = /(["'(])\/manifest\.webmanifest/g;

export const relativizeEmbeddedRootPaths = (html: string): string =>
  html.replace(EMBEDDED_ROOT, "$1./$2/").replace(EMBEDDED_MANIFEST, "$1./manifest.webmanifest");

/** Public path prefix for a DSH iframe (`/u/<32hex>` or `/dsh`). Empty on loopback `/`. */
export const embeddedPublicPrefix = (pathname: string): string => {
  const tenant = pathname.match(/^\/u\/[a-f0-9]{32}/);
  if (tenant) return tenant[0];
  if (pathname === "/dsh" || pathname.startsWith("/dsh/")) return "/dsh";
  return "";
};

const EMBEDDED_FIRST = new Set(["assets", "plugins", "muse"]);

/** Prefix DSH-owned root paths so they stay under the tenant iframe. */
export const rewriteEmbeddedPath = (path: string, prefix: string): string => {
  if (!prefix) return path;
  if (path === prefix || path.startsWith(`${prefix}/`)) return path;
  const parts = path.split("/");
  const first = parts[1];
  if (first === "api") {
    if (parts[2] === "muse") return path;
    return `${prefix}${path}`;
  }
  if (first !== undefined && EMBEDDED_FIRST.has(first)) return `${prefix}${path}`;
  return path;
};

/** Treat `ws`/`wss` as the same site as `http`/`https` on the same host. */
export const asHttpOrigin = (origin: string): string =>
  origin.replace(/^ws:/i, "http:").replace(/^wss:/i, "https:");

export const rewriteEmbeddedHref = (href: string, pathname: string, origin: string): string => {
  try {
    const url = new URL(href, origin);
    if (asHttpOrigin(url.origin) !== asHttpOrigin(origin)) return href;
    const next = rewriteEmbeddedPath(url.pathname, embeddedPublicPrefix(pathname));
    if (next === url.pathname) return href;
    return `${url.origin}${next}${url.search}${url.hash}`;
  } catch {
    return href;
  }
};

/**
 * Patches fetch / XHR / WebSocket / EventSource before DSH modules run.
 * DSH posts `new URL("/api/host.listDirectory", origin)` which would otherwise
 * hit Cloud nginx `/api` (404). Must not contain a raw `<`.
 */
export const EMBEDDED_PATH_REWRITE_SCRIPT =
  "<script>(function(){" +
  "if(window.__MUSE_EMBED_REWRITE__)return;" +
  "window.__MUSE_EMBED_REWRITE__=1;" +
  "function prefix(){" +
  "var m=location.pathname.match(/^\\/u\\/[a-f0-9]{32}/);" +
  "if(m)return m[0];" +
  "if(location.pathname===\"/dsh\"||location.pathname.indexOf(\"/dsh/\")===0)return \"/dsh\";" +
  "return \"\";" +
  "}" +
  "function rewritePath(path){" +
  "var p=prefix();" +
  "if(!p||path.indexOf(p+\"/\")===0||path===p)return path;" +
  "var parts=path.split(\"/\");" +
  "var first=parts[1];" +
  "if(first===\"api\"){" +
  "if(parts[2]===\"muse\")return path;" +
  "return p+path;" +
  "}" +
  "if(first===\"assets\"||first===\"plugins\"||first===\"muse\")return p+path;" +
  "return path;" +
  "}" +
  "function httpish(o){return String(o).replace(/^ws:/i,\"http:\").replace(/^wss:/i,\"https:\");}" +
  "function rewriteHref(href){" +
  "try{" +
  "var u=new URL(href,location.href);" +
  "if(httpish(u.origin)!==location.origin)return href;" +
  "var next=rewritePath(u.pathname);" +
  "if(next===u.pathname)return href;" +
  "return u.origin+next+u.search+u.hash;" +
  "}catch(err){return href;}" +
  "}" +
  "function rewriteInput(input){" +
  "if(typeof input===\"string\")return rewriteHref(input);" +
  "if(typeof URL!==\"undefined\"&&input instanceof URL)return new URL(rewriteHref(input.href));" +
  "if(typeof Request!==\"undefined\"&&input instanceof Request)return new Request(rewriteHref(input.url),input);" +
  "return input;" +
  "}" +
  "var fetch0=window.fetch;" +
  "window.fetch=function(input,init){return fetch0.call(this,rewriteInput(input),init);};" +
  "var open0=XMLHttpRequest.prototype.open;" +
  "XMLHttpRequest.prototype.open=function(method,url){" +
  "if(typeof url===\"string\"||(typeof URL!==\"undefined\"&&url instanceof URL))arguments[1]=rewriteInput(url);" +
  "return open0.apply(this,arguments);" +
  "};" +
  "var WS=window.WebSocket;" +
  "window.WebSocket=function(url,protocols){" +
  "var next=rewriteInput(url);" +
  "return protocols===undefined?new WS(next):new WS(next,protocols);" +
  "};" +
  "window.WebSocket.prototype=WS.prototype;" +
  "window.WebSocket.CONNECTING=WS.CONNECTING;" +
  "window.WebSocket.OPEN=WS.OPEN;" +
  "window.WebSocket.CLOSING=WS.CLOSING;" +
  "window.WebSocket.CLOSED=WS.CLOSED;" +
  "var ES=window.EventSource;" +
  "if(ES){" +
  "window.EventSource=function(url,config){" +
  "var next=rewriteInput(url);" +
  "return config===undefined?new ES(next):new ES(next,config);" +
  "};" +
  "window.EventSource.prototype=ES.prototype;" +
  "}" +
  "})();</script>";


export type BridgeReplyMessage = {
  source: typeof FRAME_SOURCE;
  type: "bridge.reply";
  requestId?: string;
  status: number;
  body: unknown;
};

/** XHR onload/onerror → parent `bridge.reply` (E1-T4). */
export function makeBridgeReply(
  requestId: string | undefined,
  status: number,
  body: unknown
): BridgeReplyMessage {
  return {
    source: FRAME_SOURCE,
    type: "bridge.reply",
    status,
    body,
    ...(requestId ? { requestId } : {})
  };
}

type ParentWin = { postMessage: (data: unknown, origin: string) => void };

/**
 * Mirrors PARENT_BRIDGE_SCRIPT: SSE intents buffer until parentOrigin is set,
 * then flush (E1-T10).
 */
export class ParentOriginBuffer {
  parentOrigin = "";
  parentWin: ParentWin | null = null;
  readonly pending: unknown[] = [];

  onParentHello(origin: string, win: ParentWin): void {
    this.parentOrigin = origin;
    this.parentWin = win;
    this.flush();
  }

  onSsePayload(parsed: unknown): void {
    if (this.parentWin && this.parentOrigin) {
      this.parentWin.postMessage(parsed, this.parentOrigin);
      return;
    }
    this.pending.push(parsed);
  }

  flush(): void {
    if (!this.parentWin || !this.parentOrigin) return;
    while (this.pending.length > 0) {
      this.parentWin.postMessage(this.pending.shift(), this.parentOrigin);
    }
  }
}
