/// APK-side repair for the public DSH page.
///
/// Production DSH may still serve an older `@muse/dsh-mobile-surface` inject
/// (Chinese-only aria-labels, transform-based drawer). The WebView runs this
/// after each document load so the installed APK does not wait on a Host
/// deploy.
const dshMobileSurfaceRepairScript = r'''
(function(){
  function paint(){
    if(!(window.MuseNativeCapability&&typeof window.MuseNativeCapability.postMessage==="function"))return;
    document.documentElement.setAttribute("data-muse-surface","mobile");
    var css="html[data-muse-surface=mobile] [data-muse-region=sidebar]{"
      +"transform:none!important;left:-110%!important;pointer-events:auto!important;"
      +"touch-action:manipulation;}"
      +"html[data-muse-surface=mobile] [data-muse-mobile-frame]:not([data-sidebar-collapsed]) [data-muse-region=sidebar]{"
      +"left:0!important;transform:none!important;}"
      +"html[data-muse-surface=mobile] [data-muse-drawer-backdrop]{"
      +"left:min(86vw,320px)!important;pointer-events:auto!important;}";
    var tag=document.getElementById("muse-apk-mobile-surface");
    if(!tag){
      tag=document.createElement("style");
      tag.id="muse-apk-mobile-surface";
      document.documentElement.appendChild(tag);
    }
    tag.textContent=css;
    function clickByAria(labels){
      for(var i=0;i<labels.length;i++){
        var nodes=document.querySelectorAll('button[aria-label="'+labels[i]+'"]');
        if(nodes.length){nodes[0].click();return true;}
      }
      return false;
    }
    function drawerOpen(){
      var frame=document.querySelector("[data-muse-mobile-frame]");
      return !!(frame&&!frame.hasAttribute("data-sidebar-collapsed"));
    }
    function openDrawer(){return clickByAria(["打开侧边栏","Open sidebar"]);}
    function closeDrawer(){
      if(!drawerOpen())return false;
      clickByAria(["收起侧边栏","Collapse sidebar"]);
      return true;
    }
    function newSession(){
      return clickByAria(["新建会话","New session","新会话","New Session"]);
    }
    var menu=document.querySelector("[data-muse-drawer-toggle]");
    if(menu&&menu.getAttribute("data-muse-apk-bound")!=="1"){
      menu.setAttribute("data-muse-apk-bound","1");
      menu.addEventListener("click",function(){
        var frame=document.querySelector("[data-muse-mobile-frame]");
        if(!frame)return;
        if(frame.hasAttribute("data-sidebar-collapsed"))openDrawer();
        else closeDrawer();
      });
    }
    var neu=document.querySelector("[data-muse-new-session]");
    if(neu&&neu.getAttribute("data-muse-apk-bound")!=="1"){
      neu.setAttribute("data-muse-apk-bound","1");
      neu.addEventListener("click",function(){newSession();});
    }
    var veil=document.querySelector("[data-muse-drawer-backdrop]");
    if(veil&&veil.getAttribute("data-muse-apk-bound")!=="1"){
      veil.setAttribute("data-muse-apk-bound","1");
      veil.addEventListener("click",function(){closeDrawer();});
    }
    if(!window.__museDshBackBound){
      window.__museDshBackBound=1;
      window.addEventListener("muse-native-capability",function(ev){
        var d=ev&&ev.detail;if(!d||d.type!=="back.request")return;
        var consumed=closeDrawer();
        window.MuseNativeCapability.postMessage(JSON.stringify({
          protocol:"muse.native-capability/v1",
          type:"back.result",
          requestId:d.requestId||"back",
          consumed:consumed
        }));
      });
    }
  }
  paint();
  if(!window.__museApkSurfaceLoop){
    window.__museApkSurfaceLoop=1;
    setInterval(paint,800);
  }
})();
''';
