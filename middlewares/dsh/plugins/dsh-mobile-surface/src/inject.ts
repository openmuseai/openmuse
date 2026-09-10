import { assertSafeScript } from "./html.js";

/** CSS only matches Muse-owned markers and frozen DSH data-* attributes. */
export const MOBILE_SURFACE_CSS =
  "html[data-muse-surface=mobile]{viewport-fit:cover;overflow-x:hidden;}" +
  "html[data-muse-surface=mobile] body{overflow-x:hidden;max-width:100%;}" +
  "html[data-muse-surface=mobile] [data-phase]{" +
  "--dsh-chat-content-width:min(100vw - 24px,748px);" +
  "--dsh-composer-card-max-width:calc(100% - 24px);" +
  "--dsh-composer-side-clearance:12px;" +
  "overflow-x:hidden;max-width:100%;}" +
  "html[data-muse-surface=mobile] [data-muse-mobile-frame]{" +
  "display:flex!important;flex-direction:column;min-height:100dvh;min-width:0;" +
  "grid-template-columns:none!important;overflow-x:hidden;}" +
  "html[data-muse-surface=mobile] [data-muse-mobile-header]{" +
  "position:fixed;z-index:30;top:0;left:0;right:0;height:48px;" +
  "display:flex;align-items:center;justify-content:space-between;" +
  "padding:4px 8px;pointer-events:none;" +
  "background:linear-gradient(180deg,var(--dsw-alias-bg-base) 70%,transparent);}" +
  "html[data-muse-surface=mobile] [data-muse-mobile-header] button{" +
  "pointer-events:auto;width:40px;height:40px;border:0;border-radius:20px;" +
  "background:transparent;color:var(--dsw-alias-label-primary);" +
  "font-size:20px;line-height:1;}" +
  "html[data-muse-surface=mobile] [data-muse-drawer-toggle]," +
  "html[data-muse-surface=mobile] [data-muse-new-session]{" +
  "font-size:22px;font-weight:500;}" +
  "html[data-muse-surface=mobile] [data-muse-drawer-backdrop]{" +
  "display:none;position:fixed;z-index:22;top:0;right:0;bottom:0;" +
  "left:min(86vw,320px);background:#0006;pointer-events:auto;}" +
  "html[data-muse-surface=mobile] [data-muse-mobile-frame]:not([data-sidebar-collapsed]) [data-muse-drawer-backdrop]{" +
  "display:block;}" +
  "html[data-muse-surface=mobile] [data-muse-region=sidebar]{" +
  "position:fixed;z-index:25;top:0;bottom:0;width:min(86vw,320px);" +
  "left:-110%;transition:left .2s ease;transform:none;" +
  "background:var(--dsw-specific-sidebar-fill,inherit);overflow:auto;" +
  "pointer-events:auto;touch-action:manipulation;" +
  "padding-top:env(safe-area-inset-top);" +
  "padding-bottom:env(safe-area-inset-bottom);}" +
  "html[data-muse-surface=mobile] [data-muse-mobile-frame]:not([data-sidebar-collapsed]) [data-muse-region=sidebar]{" +
  "left:0;transform:none;}" +
  "html[data-muse-surface=mobile] [data-muse-region=center]{" +
  "flex:1;min-width:0;min-height:0;padding-top:48px;overflow-x:hidden;}" +
  "html[data-muse-surface=mobile] [data-muse-region=details]{display:none;}" +
  "html[data-muse-surface=mobile] [data-conversation-scroll]{" +
  "overflow-x:hidden;overflow-y:auto;min-width:0;}" +
  "html[data-muse-surface=mobile] [data-chat-flow]," +
  "html[data-muse-surface=mobile] [data-turn-tail]{" +
  "max-width:100%;min-width:0;overflow-x:hidden;}" +
  "html[data-muse-surface=mobile] [data-time-hover-root]>div>span:not([id]){" +
  "display:none!important;}" +
  "html[data-muse-surface=mobile] [data-muse-token-chrome]{display:none!important;}" +
  "html[data-muse-surface=mobile] [data-composer-card] span:has(button svg circle){" +
  "display:none!important;}" +
  "html[data-muse-surface=mobile] [data-composer-seat]{" +
  "padding:0 0 max(10px,env(safe-area-inset-bottom));}" +
  "html[data-muse-surface=mobile] [data-composer-card]{" +
  "max-width:none!important;width:100%;border-radius:24px!important;" +
  "position:relative;padding-bottom:4px;}" +
  "html[data-muse-surface=mobile] [data-muse-native-input]{" +
  "position:absolute;right:52px;bottom:8px;z-index:2;" +
  "display:flex;gap:8px;align-items:center;}" +
  "html[data-muse-surface=mobile] [data-muse-native-input] button{" +
  "width:36px;height:36px;border:0;border-radius:18px;" +
  "background:var(--dsw-specific-selector,rgba(255,255,255,.08));" +
  "color:var(--dsw-alias-label-primary);font-size:18px;line-height:36px;" +
  "padding:0;text-align:center;}";

export const LAYOUT_ADAPTER_SCRIPT = (() => {
  const body =
    "(function(){" +
    "function adapt(){" +
    "var overlay=document.querySelector(\"[data-shell-overlay]\");" +
    "if(!overlay)return false;" +
    "var frame=overlay.parentElement;" +
    "if(!frame)return false;" +
    "frame.setAttribute(\"data-muse-mobile-frame\",\"true\");" +
    "var names=[\"sidebar\",\"center\",\"details\"];" +
    "var i=0,k=0;" +
    "while(i!==frame.children.length&&k!==3){" +
    "var el=frame.children[i];" +
    "i+=1;" +
    "if(el.hasAttribute(\"data-shell-overlay\"))continue;" +
    "el.setAttribute(\"data-muse-region\",names[k]);" +
    "k+=1;}" +
    "return true;}" +
    "function clickByAria(labels){" +
    "var i=0;" +
    "while(i!==labels.length){" +
    "var nodes=document.querySelectorAll(\"button[aria-label=\\\"\"+labels[i]+\"\\\"]\");" +
    "if(nodes.length){nodes[0].click();return true;}" +
    "i+=1;}" +
    "return false;}" +
    "function drawerOpen(){" +
    "var frame=document.querySelector(\"[data-muse-mobile-frame]\");" +
    "return !!(frame&&!frame.hasAttribute(\"data-sidebar-collapsed\"));}" +
    "function openDrawer(){return clickByAria([\"打开侧边栏\",\"Open sidebar\"]);}" +
    "function closeDrawer(){" +
    "if(!drawerOpen())return false;" +
    "clickByAria([\"收起侧边栏\",\"Collapse sidebar\"]);" +
    "return true;}" +
    "function newSession(){" +
    "return clickByAria([\"新建会话\",\"New session\",\"新会话\",\"New Session\"]);}" +
    "function mountChrome(){" +
    "var frame=document.querySelector(\"[data-muse-mobile-frame]\");" +
    "if(!frame||frame.getAttribute(\"data-muse-chrome\")===\"1\")return;" +
    "frame.setAttribute(\"data-muse-chrome\",\"1\");" +
    "var bar=document.createElement(\"div\");" +
    "bar.setAttribute(\"data-muse-mobile-header\",\"true\");" +
    "var menu=document.createElement(\"button\");" +
    "menu.type=\"button\";" +
    "menu.setAttribute(\"data-muse-drawer-toggle\",\"true\");" +
    "menu.setAttribute(\"aria-label\",\"打开侧边栏\");" +
    "menu.textContent=\"\\u2261\";" +
    "menu.addEventListener(\"click\",function(){" +
    "if(frame.hasAttribute(\"data-sidebar-collapsed\"))openDrawer();" +
    "else closeDrawer();});" +
    "var neu=document.createElement(\"button\");" +
    "neu.type=\"button\";" +
    "neu.setAttribute(\"data-muse-new-session\",\"true\");" +
    "neu.setAttribute(\"aria-label\",\"新建会话\");" +
    "neu.textContent=\"+\";" +
    "neu.addEventListener(\"click\",function(){newSession();});" +
    "bar.appendChild(menu);bar.appendChild(neu);" +
    "var veil=document.createElement(\"div\");" +
    "veil.setAttribute(\"data-muse-drawer-backdrop\",\"true\");" +
    "veil.addEventListener(\"click\",function(){closeDrawer();});" +
    "frame.appendChild(bar);frame.appendChild(veil);}" +
    "function hideTokenChrome(){" +
    "var card=document.querySelector(\"[data-composer-card]\");" +
    "if(!card||!card.parentElement)return;" +
    "var n=card.nextElementSibling;" +
    "while(n){" +
    "var keep=n.hasAttribute(\"data-queue-dock\")||n.querySelector(\"[data-queue-dock]\");" +
    "if(!keep)n.setAttribute(\"data-muse-token-chrome\",\"true\");" +
    "n=n.nextElementSibling;}}" +
    "function bindBack(){" +
    "if(window.__museDshBackBound)return;" +
    "window.__museDshBackBound=1;" +
    "window.addEventListener(\"muse-native-capability\",function(ev){" +
    "var d=ev&&ev.detail;if(!d||d.type!==\"back.request\")return;" +
    "var consumed=closeDrawer();" +
    "if(window.MuseNativeCapability&&typeof window.MuseNativeCapability.postMessage===\"function\"){" +
    "window.MuseNativeCapability.postMessage(JSON.stringify({" +
    "protocol:\"muse.native-capability/v1\",type:\"back.result\"," +
    "requestId:d.requestId||\"back\",consumed:consumed}));}" +
    "});}" +
    "function activate(){" +
    "if(!(window.MuseNativeCapability&&typeof window.MuseNativeCapability.postMessage===\"function\"))return;" +
    "document.documentElement.setAttribute(\"data-muse-surface\",\"mobile\");" +
    "if(!adapt())return;" +
    "mountChrome();" +
    "bindBack();" +
    "hideTokenChrome();" +
    "var frame=document.querySelector(\"[data-muse-mobile-frame]\");" +
    "if(!frame||frame.getAttribute(\"data-muse-observed\")===\"1\")return;" +
    "frame.setAttribute(\"data-muse-observed\",\"1\");" +
    "var obs=new MutationObserver(function(){adapt();hideTokenChrome();});" +
    "obs.observe(frame,{attributes:true,attributeFilter:[\"data-sidebar-collapsed\",\"data-details-collapsed\"],childList:true,subtree:true});" +
    "}" +
    "if(document.readyState===\"loading\")document.addEventListener(\"DOMContentLoaded\",activate);" +
    "else activate();" +
    "setInterval(activate,800);" +
    "})();";
  assertSafeScript(body);
  return "<script>" + body + "</script>";
})();

export const MOBILE_SURFACE_SNIPPET =
  "<style>" + MOBILE_SURFACE_CSS + "</style>" + LAYOUT_ADAPTER_SCRIPT;
