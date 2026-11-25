// Reproducible prototype QA using Node built-ins and an isolated headless Chrome.
// Does not connect to the user's browser, device, clipboard or accounts.
import {createServer} from 'node:http';
import {spawn} from 'node:child_process';
import {mkdtemp,readFile,writeFile,mkdir,rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import assert from 'node:assert/strict';

const root=path.dirname(fileURLToPath(import.meta.url));
const shots=path.join(root,'screens');await mkdir(shots,{recursive:true});
const profile=await mkdtemp(path.join(tmpdir(),'openmuse-resource-center-qa-'));
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
const mime={'.html':'text/html','.css':'text/css','.js':'text/javascript','.md':'text/plain','.png':'image/png'};
const server=createServer(async(req,res)=>{
  try{
    const route=decodeURIComponent(new URL(req.url,'http://localhost').pathname);
    const file=path.resolve(root,'.'+(route==='/'?'/index.html':route));
    if(!file.startsWith(root+path.sep)){res.writeHead(403).end();return;}
    const body=await readFile(file);res.writeHead(200,{'Content-Type':mime[path.extname(file)]||'application/octet-stream','Cache-Control':'no-store'}).end(body);
  }catch{res.writeHead(404).end();}
});
await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
const origin=`http://127.0.0.1:${server.address().port}`;
const executable=process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const chrome=spawn(executable,['--headless=new','--disable-gpu','--disable-background-networking','--disable-component-update','--disable-sync','--no-first-run','--no-default-browser-check','--remote-debugging-port=0',`--user-data-dir=${profile}`,'about:blank'],{stdio:'ignore'});
let socket;const pending=new Map();let seq=0;const runtimeErrors=[];const externalRequests=[];const checks=[];
async function call(method,params={}){const id=++seq;return new Promise((resolve,reject)=>{pending.set(id,{resolve,reject});socket.send(JSON.stringify({id,method,params}));});}
async function evaluate(expression){const r=await call('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});if(r.exceptionDetails)throw Error(r.exceptionDetails.text+': '+r.exceptionDetails.exception?.description);return r.result?.value;}
async function waitFor(expression,timeout=4000){const until=Date.now()+timeout;while(Date.now()<until){if(await evaluate(expression))return;await sleep(40);}throw Error('Timed out: '+expression);}
async function click(selector){const found=await evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});if(!e)return false;e.click();return true})()`);assert(found,'Missing control '+selector);}
async function load(screen='recent',theme='light',lang='zh',width=390,height=844,embed=true){
  await call('Emulation.setDeviceMetricsOverride',{width,height,deviceScaleFactor:2,mobile:embed});
  await call('Page.navigate',{url:`${origin}/index.html?screen=${screen}&theme=${theme}&lang=${lang}&embed=${embed?'1':'0'}`});
  await waitFor('Boolean(window.resourceCenterDemo)');await sleep(260);
}
const snapshot=()=>evaluate('window.resourceCenterDemo.snapshot()');
async function check(name,fn){await fn();checks.push({name,passed:true});console.log('PASS',name);}
async function shot(name){const png=await call('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});await writeFile(path.join(shots,name),Buffer.from(png.data,'base64'));}
try{
  let port;for(let i=0;i<100;i++){try{port=(await readFile(path.join(profile,'DevToolsActivePort'),'utf8')).split('\n')[0];break;}catch{await sleep(50);}}
  assert(port,'Headless Chrome did not start');
  const targets=await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
  const target=targets.find(t=>t.type==='page');assert(target);
  socket=new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((resolve,reject)=>{socket.addEventListener('open',resolve,{once:true});socket.addEventListener('error',reject,{once:true});});
  socket.addEventListener('message',event=>{const r=JSON.parse(event.data);if(r.id){const p=pending.get(r.id);pending.delete(r.id);if(r.error)p.reject(Error(JSON.stringify(r.error)));else p.resolve(r.result);}else if(r.method==='Runtime.exceptionThrown')runtimeErrors.push(r.params.exceptionDetails.text);else if(r.method==='Network.requestWillBeSent'&&/^https?:/.test(r.params.request.url)&&!r.params.request.url.startsWith(origin+'/'))externalRequests.push(r.params.request.url);});
  await call('Page.enable');await call('Runtime.enable');await call('Network.enable');
  await check('five tabs and a single-row source switcher',async()=>{
    await load();assert.equal(await evaluate('document.querySelectorAll(".bottom-nav button").length'),5);
    assert.equal(await evaluate('new Set([...document.querySelectorAll(".subtabs button")].map(e=>e.getBoundingClientRect().y)).size'),1);
  });
  await check('copy reference → paste into draft → send with a clickable card',async()=>{
    await click('[data-menu="proposal"]');await click('[data-action="copy-resource"]');assert((await snapshot()).clipboard);
    await click('[data-nav="tasks"]');await click('[data-action="paste"]');assert.equal((await snapshot()).draft.length,1);assert.equal((await snapshot()).messages,0);
    await click('[data-action="paste"]');assert.equal((await snapshot()).draft.length,1);
    await click('[data-action="send"]');assert.equal((await snapshot()).messages,1);assert.equal((await snapshot()).draft.length,0);
    await click('.message [data-open="proposal"]');assert.equal(await evaluate('Boolean(document.querySelector(".document-sheet"))'),true);
    await click('[data-action="close-editor"]');assert.equal((await snapshot()).messages,1);
  });
  await check('copying a path clears the previous typed resource reference',async()=>{
    await click('[data-nav="home"]');await click('[data-menu="cover"]');await click('[data-action="copy-path"]');assert.equal((await snapshot()).clipboard,null);
  });
  await check('remote hierarchy and offline recovery',async()=>{
    await evaluate('window.resourceCenterDemo.scene("remote")');
    for(const id of ['remote-workspace','remote-mount','remote-materials'])await click(`[data-folder="${id}"]`);
    assert.equal(await evaluate('document.querySelectorAll("[data-menu]").length'),2);
    await click('#pc-online');assert(await evaluate('document.querySelector(".empty-state").textContent.includes("离线")'));
    await click('[data-action="reconnect"]');assert.equal((await snapshot()).online,true);assert.equal(await evaluate('document.querySelectorAll("[data-menu]").length'),2);
  });
  await check('read-only PC resources cannot be renamed',async()=>{
    await click('[data-menu="proposal"]');assert(await evaluate('document.querySelector("[data-action=rename]").disabled'));await click('[data-action="close-sheet"]');
  });
  await check('workspace and mount navigation keep different sources separate',async()=>{
    await evaluate('window.resourceCenterDemo.scene("remote")');await click('[data-folder="remote-notes"]');assert(await evaluate('document.querySelector(".crumbs").textContent.includes("个人笔记")'));
    await evaluate('window.resourceCenterDemo.scene("remote")');await click('[data-folder="remote-workspace"]');await click('[data-folder="remote-shared"]');await click('[data-folder="remote-materials"]');
    assert(await evaluate('document.querySelector(".crumbs").textContent.includes("共享素材")'));assert(await evaluate('Boolean(document.querySelector("[data-menu=remote-asset]"))'));assert.equal(await evaluate('Boolean(document.querySelector("[data-menu=proposal]"))'),false);
  });
  await check('plugin install state dynamically changes creator entries',async()=>{
    await evaluate('window.resourceCenterDemo.scene("create")');assert.equal(await evaluate('document.querySelectorAll(".creator-tile").length'),4);
    await click('#notes-plugin');assert.equal(await evaluate('document.querySelectorAll(".creator-tile").length'),2);
    await click('#notes-plugin');assert.equal(await evaluate('document.querySelectorAll(".creator-tile").length'),4);
  });
  await check('create validates the name, creates once and opens the resource',async()=>{
    await click('[data-creator="markdown"]');await evaluate('document.querySelector("#resource-name").value=""');await click('[data-action="confirm-create"]');
    assert(await evaluate('document.querySelector("#form-error").textContent.length>0'));
    const count=(await snapshot()).resourceCount;await evaluate('document.querySelector("#resource-name").value="审核笔记.md"');await click('[data-action="confirm-create"]');
    await waitFor('Boolean(document.querySelector(".document-sheet"))');assert.equal((await snapshot()).resourceCount,count+1);
    await click('[data-action="close-editor"]');assert(await evaluate('document.querySelector("#app").textContent.includes("审核笔记.md")'));
  });
  await check('canceling an in-progress prototype create does not leave a resource',async()=>{
    await evaluate('window.resourceCenterDemo.scene("create")');await click('[data-creator="note"]');const count=(await snapshot()).resourceCount;
    await click('[data-action="confirm-create"]');await click('[data-action="close-sheet"]');await sleep(800);assert.equal((await snapshot()).resourceCount,count);
  });
  await check('rename local resource and remove recent with undo',async()=>{
    await evaluate('window.resourceCenterDemo.scene("recent")');await click('[data-menu="cover"]');await click('[data-action="rename"]');
    await evaluate('document.querySelector("#resource-name").value="新封面.png"');await click('[data-action="confirm-rename"]');assert(await evaluate('document.querySelector("#app").textContent.includes("新封面.png")'));
    await click('[data-menu="cover"]');await click('[data-action="remove-recent"]');assert.equal(await evaluate('Boolean(document.querySelector("[data-menu=cover]"))'),false);
    await click('#toast-action');assert.equal(await evaluate('Boolean(document.querySelector("[data-menu=cover]"))'),true);
  });
  await check('no handler is reported rather than pretending to play a video',async()=>{
    await load();await click('[data-open="video"]');assert.equal((await snapshot()).overlay,'no-handler');
  });
  await check('empty and error states have working recovery actions',async()=>{
    await load('empty');assert(await evaluate('Boolean(document.querySelector(".empty-state"))'));await click('[data-action="browse-local"]');assert.equal((await snapshot()).section,'local');
    await load('error');await click('[data-action="retry"]');assert(await evaluate('document.querySelectorAll(".resource-row").length>0'));
  });
  await check('long press opens the same resource menu',async()=>{
    await load();await evaluate('document.querySelector("[data-open=proposal]").dispatchEvent(new PointerEvent("pointerdown",{button:0,clientX:50,clientY:200,bubbles:true}))');
    await waitFor('window.resourceCenterDemo.snapshot().overlay==="menu"');assert.equal((await snapshot()).overlay,'menu');
  });
  await check('sheet uses modal semantics, traps focus and restores the triggering control',async()=>{
    await load();await evaluate('document.querySelector("[data-menu=proposal]").focus()');await click('[data-menu="proposal"]');
    assert(await evaluate('document.querySelector("#app").inert'));assert.equal(await evaluate('document.querySelector("[role=dialog]").getAttribute("aria-modal")'),'true');
    await evaluate('document.querySelector(".sheet-body button:last-child").focus()');
    await call('Input.dispatchKeyEvent',{type:'keyDown',key:'Tab',code:'Tab',windowsVirtualKeyCode:9});
    await call('Input.dispatchKeyEvent',{type:'keyUp',key:'Tab',code:'Tab',windowsVirtualKeyCode:9});
    assert.equal(await evaluate('document.activeElement.getAttribute("data-action")'),'close-sheet');
    await click('[data-action="close-sheet"]');assert.equal(await evaluate('document.activeElement.getAttribute("data-menu")'),'proposal');
  });
  await check('320 / 390 / 430px: themes and languages do not overflow the viewport',async()=>{
    for(const width of [320,390,430])for(const theme of ['light','dark'])for(const lang of ['zh','en']){
      await load('recent',theme,lang,width,844);assert(await evaluate('document.documentElement.scrollWidth <= innerWidth'),`${width}/${theme}/${lang}`);
      assert.equal(await evaluate('document.querySelectorAll(".bottom-nav button").length'),5);
      for(const screen of ['create','chat']){await evaluate(`window.resourceCenterDemo.scene(${JSON.stringify(screen)})`);assert(await evaluate('document.documentElement.scrollWidth <= innerWidth'),`${screen}/${width}/${theme}/${lang}`);}
    }
  });
  const screenshots=[
    ['01-home-recent-light.png','recent','light','zh'],['02-home-recent-dark-en.png','recent','dark','en'],
    ['03-remote-workspaces.png','remote','light','zh'],['04-remote-folder-dark.png','remote-folder','dark','zh'],
    ['05-create-light.png','create','light','zh'],['06-resource-menu-dark.png','menu','dark','zh'],
    ['07-open-with-light-en.png','open-with','light','en'],['08-chat-resource-light.png','chat','light','zh'],
    ['09-offline.png','offline','light','zh'],['10-empty-dark.png','empty','dark','en']
  ];
  for(const [name,screen,theme,lang] of screenshots){await load(screen,theme,lang);await shot(name);}
  await load('recent','light','zh',1440,1120,false);await call('Emulation.setDeviceMetricsOverride',{width:1440,height:1120,deviceScaleFactor:1,mobile:false});await sleep(100);await shot('00-review-workspace.png');
  assert.deepEqual(runtimeErrors,[]);assert.deepEqual(externalRequests,[]);
  await writeFile(path.join(root,'REVIEW-QA.json'),JSON.stringify({date:'2026-11-01',scope:'Offline HTML prototype only; not native-app acceptance',checks,runtimeErrors,externalRequests,screenshots:screenshots.length+1,externalDependencies:0,realDataOrClipboardOperations:false},null,2)+'\n');
  console.log(`Saved ${screenshots.length+1} screenshots and ${checks.length} passed checks.`);
}finally{
  socket?.close();chrome.kill('SIGTERM');await new Promise(resolve=>server.close(resolve));
  await sleep(250);await rm(profile,{recursive:true,force:true});
}
