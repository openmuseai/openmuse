import {spawn} from 'node:child_process';
import {mkdtemp,readFile,writeFile,mkdir,rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
// Runs the production DSH web frontend in an isolated test browser profile.
// Input contains a private QA bridge token; it is never included in evidence.
const repo=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const options=new Map();for(let i=2;i<process.argv.length;i+=2)options.set(process.argv[i],process.argv[i+1]);
assert(options.get('--runtime') && options.get('--evidence'),'Use --runtime PRIVATE --evidence TARGET');
const input=JSON.parse(await readFile(options.get('--runtime'),'utf8'));
const output=path.resolve(options.get('--evidence'));await mkdir(output,{recursive:true});
const source=(await readFile(path.join(repo,'plugins/dsh-agent/lib/src/resource_reference_bridge.dart'),'utf8')).split("r'''")[1].split("''';")[0];
const profile=await mkdtemp(path.join(tmpdir(),'openmuse-resource-web-'));
const chrome=spawn('/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',['--headless=new','--disable-gpu','--disable-background-networking','--disable-component-update','--no-first-run','--no-default-browser-check','--remote-debugging-port=0',`--user-data-dir=${profile}`,'about:blank'],{stdio:'ignore'});
const sleep=ms=>new Promise(r=>setTimeout(r,ms));let socket;let seq=0;const pending=new Map();const errors=[];
async function call(method,params={}){const id=++seq;return new Promise((resolve,reject)=>{pending.set(id,{resolve,reject});socket.send(JSON.stringify({id,method,params}));});}
async function evaluate(expression){const r=await call('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});if(r.exceptionDetails)throw Error(r.exceptionDetails.exception?.description||r.exceptionDetails.text);return r.result?.value;}
async function waitFor(expression,timeout=10000){const end=Date.now()+timeout;while(Date.now()<end){if(await evaluate(expression))return;await sleep(100);}throw Error('Timed out waiting for production DSH UI');}
try{
 let port;for(let i=0;i<100;i++){try{port=(await readFile(path.join(profile,'DevToolsActivePort'),'utf8')).split('\n')[0];break;}catch{await sleep(50);}}
 assert(port);const targets=await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();socket=new WebSocket(targets.find(t=>t.type==='page').webSocketDebuggerUrl);await new Promise(r=>socket.addEventListener('open',r,{once:true}));socket.addEventListener('message',e=>{const r=JSON.parse(e.data);if(r.id){const p=pending.get(r.id);pending.delete(r.id);r.error?p.reject(Error(JSON.stringify(r.error))):p.resolve(r.result);}else if(r.method==='Runtime.exceptionThrown')errors.push(r.params.exceptionDetails.text);});
 await call('Page.enable');await call('Runtime.enable');
 await call('Page.addScriptToEvaluateOnNewDocument',{source:'window.__resourceOpenMessages=[];window.MuseHostResource={postMessage:raw=>window.__resourceOpenMessages.push(JSON.parse(raw))};'+source});
 await call('Page.navigate',{url:input.dshEndpoint});await waitFor('document.body?.innerText.length>30');await sleep(3000);
 await evaluate('[...document.querySelectorAll("button")].find(x=>["继续","Continue"].includes(x.textContent.trim()))?.click()');
 const session=await evaluate(`fetch('/openmuse-native/v1/sessions',{headers:{'x-openmuse-bridge-token':${JSON.stringify(input.bridgeToken)}}}).then(r=>r.json()).then(j=>j.items[0].sessionId)`);
 assert(session,'No real QA conversation exists');
 await waitFor('typeof window.OpenMuseDshWorkspace?.openSession === "function"');
 await evaluate(`window.OpenMuseDshWorkspace.openSession(${JSON.stringify(session)})`);
 await waitFor('[...document.querySelectorAll("[data-openmuse-resource-body]")].filter(x=>x.shadowRoot?.querySelector("a")).length>=2',20000);
 const info=await evaluate(`(()=>{
 const hosts=[...document.querySelectorAll('[data-openmuse-resource-body]')];
 return hosts.flatMap(host=>[...host.shadowRoot.querySelectorAll('a')].map(a=>({label:a.textContent,uri:a.getAttribute('href'),height:a.getBoundingClientRect().height,text:host.shadowRoot.textContent,hidden:getComputedStyle(host.parentElement).fontSize})))
 })()`);
 assert.equal(info.length,2);assert(info.every(x=>x.label==='Field notes reviewed.md'&&x.height>=44&&x.hidden==='0px'));
 assert(info.every(x=>!x.text.includes('<resource-context')&&!x.text.includes('Verification phrase:')),'Raw execution context must stay out of user bubble presentation');
 const refs=info.map(x=>JSON.parse(Buffer.from(x.uri.split('/').at(-1),'base64url').toString('utf8')));
 assert(refs.every(x=>x.protocol==='openmuse.resource-reference/v1'&&x.version?.digest?.length===64));
 assert.equal(refs.filter(x=>x.retained).length,1);
 await evaluate(`document.querySelector('[data-openmuse-resource-body]').shadowRoot.querySelector('a').click()`);
 const messages=await evaluate('window.__resourceOpenMessages');
 assert.equal(messages.length,1);assert.equal(messages[0].type,'resource.reference.open');assert.equal(messages[0].uri,info[0].uri);
 await writeFile(path.join(output,'01-conversation-light.png'),Buffer.from((await call('Page.captureScreenshot',{format:'png'})).data,'base64'));
 await call('Emulation.setEmulatedMedia',{features:[{name:'prefers-color-scheme',value:'dark'}]});await sleep(500);
 await writeFile(path.join(output,'02-conversation-dark.png'),Buffer.from((await call('Page.captureScreenshot',{format:'png'})).data,'base64'));
 assert.equal(errors.length,0,'Unexpected browser runtime error');
 await writeFile(path.join(output,'result.json'),JSON.stringify({passed:true,surface:'real DSH frontend + production resource-reference adapter; isolated headless Chrome',nativeWebViewUI:false,links:info.length,canonicalHostClick:true,minimumTargetHeight:Math.min(...info.map(x=>x.height)),runtimeErrors:errors},null,2));
 console.log('PASS: real DSH journal renders two canonical resource cards; retained version, safe presentation and Host click bridge verified.');

}finally{socket?.close();chrome.kill();await sleep(200);await rm(profile,{recursive:true,force:true});}
