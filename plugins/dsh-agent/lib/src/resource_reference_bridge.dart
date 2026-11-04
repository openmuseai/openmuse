/// First-party presentation adapter for the pinned DSH plain-text user bubble.
/// Keeps canonical references clickable without putting execution paths into
/// the journal or replacing the desktop conversation/settings surface.
const openMuseResourceReferenceScript = r'''
(function installOpenMuseResourceReferences() {
  if (!document.documentElement) { document.addEventListener('DOMContentLoaded',installOpenMuseResourceReferences,{once:true}); return; }
  if (window.__openMuseResourceReferences) return;
  window.__openMuseResourceReferences = true;
  const selector = '[class*="_userRow"] [class*="_bubble"]';
  const pattern = /\[([^\]\n]+)\]\((openmuse-resource:\/\/v1\/[A-Za-z0-9_-]+)\)/g;
  const style = document.createElement('style');
  style.textContent = '[data-openmuse-resource-message="1"]{font-size:0!important;line-height:0!important}';
  (document.head || document.documentElement).append(style);
  function valid(uri) {
    try {
      if (uri.length > 8192) return false;
      const encoded = uri.slice('openmuse-resource://v1/'.length);
      const binary = atob(encoded.replace(/-/g,'+').replace(/_/g,'/'));
      const value = JSON.parse(new TextDecoder().decode(Uint8Array.from(binary,c=>c.charCodeAt(0))));
      return value.protocol === 'openmuse.resource-reference/v1' && value.identity && typeof value.identity === 'object';
    } catch (_) { return false; }
  }
  function update() {
    for (const bubble of document.querySelectorAll(selector)) {
      let host = [...bubble.children].find(e=>e.hasAttribute('data-openmuse-resource-body'));
      const raw = [...bubble.childNodes].filter(n=>n!==host).map(n=>n.textContent || '').join('');
      if (raw.length > 131072) continue;
      const links = [...raw.matchAll(pattern)].filter(m=>valid(m[2])).slice(0,10);
      if (!links.length) {
        host?.remove(); delete bubble.dataset.openmuseResourceMessage; continue;
      }
      if (host?.__source === raw) continue;
      if (!host) { host=document.createElement('div'); host.setAttribute('data-openmuse-resource-body',''); bubble.append(host); }
      host.__source=raw; bubble.dataset.openmuseResourceMessage='1';
      const root=host.shadowRoot || host.attachShadow({mode:'open'}); root.replaceChildren();
      const css=document.createElement('style');
      css.textContent=':host{display:block;font:var(--dsh-content-font-size,14px)/1.6 var(--ds-font-family,system-ui);color:var(--dsw-alias-label-primary,#20242b)}p{white-space:pre-wrap;margin:0 0 8px}a{display:flex;align-items:center;gap:8px;min-height:44px;box-sizing:border-box;padding:8px 12px;margin-top:6px;border:1px solid var(--dsw-alias-border-l3,#c9ced7);border-radius:10px;color:inherit;text-decoration:none;background:var(--dsw-alias-bg-base,#f7f8fa)}a:focus-visible{outline:2px solid #2467d6;outline-offset:2px}svg{width:18px;height:18px;flex:none}span{overflow-wrap:anywhere}';
      root.append(css);
      const text=raw.replace(pattern,'').replace(/<resource-context digest="[a-f0-9]{64}">[\s\S]*?<\/resource-context>/g,'').trim();
      if(text){const p=document.createElement('p');p.textContent=text;root.append(p);}
      for(const link of links){
        const a=document.createElement('a');a.href=link[2];
        const svg=document.createElementNS('http://www.w3.org/2000/svg','svg');svg.setAttribute('viewBox','0 0 24 24');svg.setAttribute('aria-hidden','true');
        const path=document.createElementNS(svg.namespaceURI,'path');path.setAttribute('d','M10 13a5 5 0 0 0 7 0l3-3a5 5 0 0 0-7-7l-2 2M14 11a5 5 0 0 0-7 0l-3 3a5 5 0 0 0 7 7l2-2');path.setAttribute('fill','none');path.setAttribute('stroke','currentColor');path.setAttribute('stroke-width','1.8');svg.append(path);
        const label=document.createElement('span');label.textContent=link[1];a.append(svg,label);
        a.addEventListener('click',event=>{event.preventDefault();event.stopImmediatePropagation();window.MuseHostResource?.postMessage(JSON.stringify({type:'resource.reference.open',uri:link[2]}));});
        root.append(a);
      }
    }
  }
  let scheduled=false;
  const schedule=()=>{if(scheduled)return;scheduled=true;queueMicrotask(()=>{scheduled=false;update();});};
  new MutationObserver(schedule).observe(document.documentElement,{subtree:true,childList:true,characterData:true});
  schedule();
})();
''';
