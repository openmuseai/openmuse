import type { Context } from "@deepseek-ai/cordis";
import { EMBEDDED_PATH_REWRITE_SCRIPT, relativizeEmbeddedRootPaths } from "./parent-bridge-runtime.js";

/**
 * WKWebView treats `/plugins/@scope/...` as a malformed URL (the `@` looks
 * like a userinfo delimiter), so classic `<script src>` loads fail.
 *
 * DSH's client-modules tap injects `window.__DSH_BOOT__` after `<head>`. This
 * plugin only depends on `webServer`, so our tap often runs *before* that
 * injection — a `replaceAll` on the raw index.html never sees the URLs.
 *
 * The inline script is therefore the real fix: it rewrites `__DSH_BOOT__`
 * entries and intercepts `HTMLScriptElement.src` / `setAttribute("src")`.
 * `replaceAll` still runs for the case where we are registered after the boot
 * tap. Node's `/plugins` handler `decodeURIComponent`s, so Chromium stays
 * compatible.
 *
 * The script body must not contain a raw `<` (HTML parser would treat it as a
 * tag) or `</script>`.
 */
export const SCOPED_PLUGIN_URL_PATCH_SCRIPT =
  "<script>(function(){" +
  "function encode(u){return String(u).split(\"/plugins/@\").join(\"/plugins/%40\");}" +
  "function patchBoot(){" +
  "var boot=window.__DSH_BOOT__;" +
  "if(!boot||!boot.entries||!boot.entries.length)return;" +
  "var i=0;while(i!==boot.entries.length){" +
  "var row=boot.entries[i];" +
  "if(row&&typeof row.url===\"string\")row.url=encode(row.url);" +
  "i+=1;}}" +
  "patchBoot();" +
  "var desc=Object.getOwnPropertyDescriptor(HTMLScriptElement.prototype,\"src\");" +
  "if(desc&&desc.set&&desc.get){" +
  "Object.defineProperty(HTMLScriptElement.prototype,\"src\",{" +
  "configurable:true,enumerable:true," +
  "get:function(){return desc.get.call(this);}," +
  "set:function(v){desc.set.call(this,encode(v));}" +
  "});}" +
  "var setAttr=Element.prototype.setAttribute;" +
  "Element.prototype.setAttribute=function(name,value){" +
  "if(String(name).toLowerCase()===\"src\")value=encode(value);" +
  "return setAttr.call(this,name,value);" +
  "};" +
  "})();</script>";

export const encodeScopedPluginUrls = (html: string): string => {
  const encoded = relativizeEmbeddedRootPaths(html.replaceAll("/plugins/@", "/plugins/%40"));
  const head = encoded.indexOf("<head>");
  const injected = `${EMBEDDED_PATH_REWRITE_SCRIPT}${SCOPED_PLUGIN_URL_PATCH_SCRIPT}`;
  if (head === -1) return `${injected}${encoded}`;
  return `${encoded.slice(0, head + 6)}${injected}${encoded.slice(head + 6)}`;
};

type WebServer = {
  tapIndex(transform: (html: string) => string): () => void;
};

type HostContext = Context & { webServer: WebServer };

export const name = "@muse/dsh-appflowy/webview";
export const inject = ["webServer"];

export const apply = (ctx: Context): void => {
  const server = (ctx as HostContext).webServer;
  if (server?.tapIndex === undefined) {
    throw new Error("webServer.tapIndex is required to encode scoped plugin URLs for WKWebView");
  }
  ctx.effect(
    () => server.tapIndex(encodeScopedPluginUrls),
    "muse.appflowy.encodeScopedPluginUrls"
  );
};
