/** Local verification Host. It never executes plugin business code or accesses accounts. */
import {createServer} from 'node:http'
import {readFile,mkdir,writeFile} from 'node:fs/promises'
import path from 'node:path'
import {fileURLToPath} from 'node:url'
import JSZip from 'jszip'
const root=fileURLToPath(new URL('../',import.meta.url))
const config=JSON.parse(await readFile(path.join(root,'config/globalization.json'),'utf8'))
const catalog=JSON.parse(await readFile(path.join(root,'i18n/catalog.json'),'utf8'))
const ui=await readFile(path.join(root,'dist/web/index.html'),'utf8')
const fixture=process.argv[2]??path.join(root,'vendor/pptx-engine/tests/fixtures/01_standard_business.pptx')
const input={name:path.basename(fixture),content:(await readFile(fixture)).toString('base64')}
const settingsPrefix=config.settingsPrefix,namespace=config.namespace
const encode=value=>JSON.stringify(value).replaceAll('<','\\u003c')
const host=`<!doctype html><html lang="en"><meta charset="utf-8"><title>GenOffice globalization verification</title><style>body{margin:0;font:14px system-ui}header{padding:10px;background:#eef4f8;display:flex;flex-wrap:wrap;gap:6px;align-items:center}button,input{font:inherit;padding:6px}pre{margin:0;padding:8px;max-height:85px;overflow:auto;font-size:12px}iframe{width:100%;height:calc(100vh - 165px);border:0}</style><header>
<button id="en">English</button><button id="zh">中文</button><button id="fr">French pack</button><button id="ar">Arabic pack</button><button id="patch">Remote English label</button><button id="invalid">Reject invalid pack</button><button id="agent">Toggle Agent</button><button id="editing">Toggle editing</button><button id="export">Toggle export</button><input id="replacement" aria-label="Replacement title" value="Globalization verified 文稿原文"><button id="edit">Edit fixture title</button><button id="undo">Undo</button><button id="save">Save and verify</button></header><pre id="context" aria-label="Live document state">Waiting for plugin</pre><pre id="status" role="status">Ready</pre><iframe id="view" title="GenOffice PPTX" sandbox="allow-scripts allow-downloads"></iframe><script>
const frame=document.getElementById('view'),status=document.getElementById('status'),state=document.getElementById('context');
const input=${encode(input)},namespace=${encode(namespace)},prefix=${encode(settingsPrefix)},contractHash=${encode(catalog.metadata.contractHash)};
let hostContext={locale:'en',theme:'light',platform:'desktop',displayMode:'inline',openmuse:{schemaVersion:1,settings:{features:{},parameters:{}},languagePacks:[]}};
const pending=new Map();let sequence=0;
const send=(method,params)=>frame.contentWindow.postMessage({jsonrpc:'2.0',method,params},'*');
const request=(name,args={})=>new Promise((resolve,reject)=>{const id='verify:'+ ++sequence;pending.set(id,{resolve,reject});frame.contentWindow.postMessage({jsonrpc:'2.0',id,method:'tools/call',params:{name,arguments:args}},'*')});
const call=async(name,args={})=>{const result=await request(name,args);if(result.isError)throw Error(result.content[0].text);return JSON.parse(result.content[0].text)};
const pack=(locale,messages)=>({schemaVersion:1,sourceRevision:1,contractHash,namespace,locale,messages});
const update=()=>{send('ui/notifications/host-context-changed',hostContext);status.textContent='Host context: '+JSON.stringify(hostContext)};
for(const [id,locale]of [['en','en'],['zh','zh-Hans']])document.getElementById(id).onclick=()=>{hostContext.locale=locale;update()};
for(const [id,label]of [['fr','Accueil'],['ar','الرئيسية']])document.getElementById(id).onclick=()=>{hostContext.locale=id;hostContext.openmuse.languagePacks=[pack(id,{[namespace+'.ui.ribbonTabHome']:label,[namespace+'.integration.agent']:id==='fr'?'Agent FR':'مساعد'})];update()};
document.getElementById('patch').onclick=()=>{hostContext.locale='en';hostContext.openmuse.languagePacks=[pack('en',{[namespace+'.integration.agent']:'Assistant',[namespace+'.ui.ribbonTabHome']:'Home remote'})];update()};
document.getElementById('invalid').onclick=()=>{const bad=pack('fr',{[namespace+'.ui.ribbonTabHome']:'Broken {newArgument}'});send('ui/notifications/host-context-changed',{locale:'fr',openmuse:{...hostContext.openmuse,languagePacks:[bad]}});status.textContent='Sent incompatible pack; plugin must keep previous presentation state'};
for(const key of ['agent','editing','export'])document.getElementById(key).onclick=()=>{const id=prefix+key+'.enabled';hostContext.openmuse.settings.features[id]=!(hostContext.openmuse.settings.features[id]??true);update()};
const action=(id,fn)=>document.getElementById(id).onclick=async()=>{try{status.textContent=JSON.stringify(await fn())}catch(error){status.textContent='Rejected: '+error.message}};
action('edit',async()=>{const deck=await call('pptx_read_deck');const element=deck.slides.flatMap(s=>s.elements.map(e=>({...e,slideIndex:s.index}))).find(e=>e.text?.includes('20XX'))??deck.slides.flatMap(s=>s.elements.map(e=>({...e,slideIndex:s.index}))).find(e=>e.text);if(!element)throw Error('No text element');return call('pptx_edit_text',{slideIndex:element.slideIndex,elementId:element.elementId,expectedRevision:deck.revision,text:document.getElementById('replacement').value})});
action('undo',async()=>{const current=await call('pptx_get_state');return call('pptx_undo',{expectedRevision:current.revision})});
action('save',async()=>{const current=await call('pptx_get_state');return call('pptx_save',{expectedRevision:current.revision})});
addEventListener('message',async event=>{if(event.source!==frame.contentWindow)return;const m=event.data;if(m?.jsonrpc!=='2.0')return;if(!m.method&&pending.has(m.id)){const p=pending.get(m.id);pending.delete(m.id);m.error?p.reject(m.error):p.resolve(m.result);return}const reply=result=>frame.contentWindow.postMessage({jsonrpc:'2.0',id:m.id,result},'*');
 if(m.method==='ui/initialize')reply({protocolVersion:'2026-01-26',hostInfo:{name:'Globalization verification Host',version:'1.0.0'},hostCapabilities:{downloadFile:{},message:{text:{}},updateModelContext:{text:{}}},hostContext});
 if(m.method==='ui/notifications/initialized')send('ui/notifications/tool-input',{arguments:input});
 if(m.method==='ui/update-model-context'){state.textContent=JSON.stringify(m.params.structuredContent);reply({})}
 if(m.method==='ui/message'){status.textContent='Agent message: '+JSON.stringify(m.params);reply({})}
 if(m.method==='ui/download-file'){try{const blob=m.params.contents[0].resource.blob;const result=await fetch('/saved',{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify({blob})}).then(r=>r.json());status.textContent=JSON.stringify(result);reply(result.ok?{}:{isError:true})}catch(error){reply({isError:true});status.textContent=error.message}}
});frame.srcdoc=new TextDecoder().decode(Uint8Array.from(atob(${encode(Buffer.from(ui).toString('base64'))}),c=>c.charCodeAt(0)));
</script></html>`
const out=path.join(root,'dist/globalization-preview');await mkdir(out,{recursive:true})
const server=createServer(async(req,res)=>{
 try{
  if(req.url==='/saved'&&req.method==='POST'){
   const chunks=[];let size=0;for await(const chunk of req){size+=chunk.length;if(size>90*1024*1024)throw Error('Payload too large');chunks.push(chunk)}
   const bytes=Buffer.from(JSON.parse(Buffer.concat(chunks).toString()).blob,'base64');const zip=await JSZip.loadAsync(bytes)
   const slides=await Promise.all(Object.values(zip.files).filter(f=>/^ppt\/slides\/slide\d+\.xml$/.test(f.name)).map(f=>f.async('text')))
   const report={ok:true,bytes:bytes.length,pages:slides.length,editedMarkerPresent:slides.some(xml=>xml.includes('Globalization verified'))}
   await writeFile(path.join(out,'saved.pptx'),bytes);await writeFile(path.join(out,'saved-report.json'),JSON.stringify(report,null,2)+'\n');res.setHeader('content-type','application/json');res.end(JSON.stringify(report));return
  }
  res.setHeader('content-type','text/html; charset=utf-8');res.setHeader('cache-control','no-store');res.end(host)
 }catch(error){res.statusCode=400;res.end(String(error))}
})
server.listen(0,'127.0.0.1',()=>console.log(`Preview: http://127.0.0.1:${server.address().port}`))
