import { assertSafeScript } from "./html.js";

/**
 * Native capability client: file input (WebView chooser) + speech channel.
 * Draft append uses the composer textarea native setter; it never calls submit.
 * Buttons sit inside [data-composer-card] as circular controls.
 */
export const MOBILE_INPUT_SCRIPT = (() => {
  const body =
    "(function(){" +
    "function ready(){" +
    "return document.documentElement.getAttribute(\"data-muse-surface\")===\"mobile\"" +
    "&& window.MuseNativeCapability" +
    "&& typeof window.MuseNativeCapability.postMessage===\"function\";}" +
    "function post(msg){window.MuseNativeCapability.postMessage(JSON.stringify(msg));}" +
    "function rid(){return \"muse.\"+Date.now().toString(36)+Math.random().toString(36).slice(2,8);}" +
    "var generation=0, sessionId=\"\", speechId=null, startDraft=\"\", listening=false;" +
    "function card(){return document.querySelector(\"[data-composer-card]\");}" +
    "function seat(){return document.querySelector(\"[data-composer-seat]\");}" +
    "function textarea(root){return root?root.querySelector(\"textarea\"):null;}" +
    "function setDraft(next){" +
    "var box=textarea(seat());" +
    "if(!box)return;" +
    "var desc=Object.getOwnPropertyDescriptor(window.HTMLTextAreaElement.prototype,\"value\");" +
    "if(desc&&desc.set)desc.set.call(box,next);" +
    "else box.value=next;" +
    "box.dispatchEvent(new Event(\"input\",{bubbles:true}));}" +
    "function appendFinal(text){" +
    "if(!text)return;" +
    "var box=textarea(seat());" +
    "var current=box?box.value:startDraft;" +
    "var glue=/[\\u4e00-\\u9fff]$/.test(current)||/^[\\u4e00-\\u9fff]/.test(text)?\"\":\" \";" +
    "setDraft((current||\"\")+(current&&!/\\s$/.test(current)?glue:\"\")+text);}" +
    "function circleBtn(label,glyph){" +
    "var b=document.createElement(\"button\");" +
    "b.type=\"button\";" +
    "b.textContent=glyph;" +
    "b.setAttribute(\"aria-label\",label);" +
    "return b;}" +
    "function mount(){" +
    "if(!ready())return;" +
    "var host=card()||seat();" +
    "if(!host||host.getAttribute(\"data-muse-input\")===\"1\")return;" +
    "host.setAttribute(\"data-muse-input\",\"1\");" +
    "var bar=document.createElement(\"div\");" +
    "bar.setAttribute(\"data-muse-native-input\",\"true\");" +
    "var file=document.createElement(\"input\");" +
    "file.type=\"file\";" +
    "file.accept=\"image/png,image/jpeg,image/webp,image/gif\";" +
    "file.multiple=true;" +
    "file.style.display=\"none\";" +
    "var attach=circleBtn(\"附件\",\"+\");" +
    "attach.setAttribute(\"data-muse-attach\",\"true\");" +
    "attach.addEventListener(\"click\",function(){file.click();});" +
    "var mic=circleBtn(\"麦克风\",\"\u25c9\");" +
    "mic.setAttribute(\"data-muse-mic\",\"true\");" +
    "mic.addEventListener(\"click\",function(){" +
    "if(listening&&speechId){" +
    "post({protocol:\"muse.native-capability/v1\",type:\"speech.cancel\",requestId:speechId,sessionId:sessionId,generation:generation});" +
    "listening=false;speechId=null;mic.setAttribute(\"aria-label\",\"麦克风\");return;}" +
    "var box=textarea(seat());" +
    "startDraft=box?box.value:\"\";" +
    "speechId=rid();" +
    "listening=true;" +
    "mic.setAttribute(\"aria-label\",\"停止\");" +
    "post({protocol:\"muse.native-capability/v1\",type:\"speech.start\",requestId:speechId,sessionId:sessionId,generation:generation,locale:\"zh-CN\"});" +
    "});" +
    "bar.appendChild(attach);bar.appendChild(mic);bar.appendChild(file);" +
    "host.appendChild(bar);" +
    "post({protocol:\"muse.native-capability/v1\",type:\"capabilities.get\",requestId:rid(),sessionId:sessionId,generation:generation});" +
    "}" +
    "window.addEventListener(\"muse-native-capability\",function(ev){" +
    "var detail=ev&&ev.detail;if(!detail||typeof detail!==\"object\")return;" +
    "if(detail.type===\"capabilities\"){" +
    "var attach=document.querySelector(\"[data-muse-attach]\");" +
    "var mic=document.querySelector(\"[data-muse-mic]\");" +
    "if(attach)attach.style.display=detail.file||detail.camera?\"\":\"none\";" +
    "if(mic)mic.style.display=detail.speech?\"\":\"none\";" +
    "}" +
    "if(detail.type===\"speech.final\"&&detail.requestId===speechId){" +
    "appendFinal(String(detail.text||\"\"));listening=false;speechId=null;" +
    "var mic=document.querySelector(\"[data-muse-mic]\");" +
    "if(mic)mic.setAttribute(\"aria-label\",\"麦克风\");}" +
    "if(detail.type===\"speech.error\"&&detail.requestId===speechId){" +
    "listening=false;speechId=null;" +
    "var mic=document.querySelector(\"[data-muse-mic]\");" +
    "if(mic)mic.setAttribute(\"aria-label\",\"麦克风\");}" +
    "});" +
    "if(document.readyState===\"loading\")document.addEventListener(\"DOMContentLoaded\",mount);" +
    "else mount();" +
    "setInterval(mount,800);" +
    "})();";
  assertSafeScript(body);
  return "<script>" + body + "</script>";
})();
