import {readFileSync} from 'node:fs';
import {Script, createContext} from 'node:vm';
import assert from 'node:assert/strict';
import {parseHTML} from 'linkedom';
import {parseMessage} from '../src/messages.mjs';

const input=JSON.parse(readFileSync(process.argv[2],'utf8'));
for (const {source,nodes,parameters} of input.templates) {
  const parsed=parseMessage(source);
  assert.deepEqual(JSON.parse(JSON.stringify(parsed.nodes)),nodes);
  assert.deepEqual(parsed.parameters,[...parameters].sort());
}
const flush=()=>new Promise(resolve=>setImmediate(resolve));
for (const [kind,html] of Object.entries(input.documents)) {
  const {window}=parseHTML(html), document=window.document, timers=new Map();
  let id=0;
  // The rendered document is host-owned. These are protocol test doubles.
  window.setInterval=fn=>{timers.set(++id,fn);return id;};window.clearInterval=key=>timers.delete(key);
  window.Intl=Intl;window.Date=Date;window.console=console;
  window.OpenMusePlugin={postMessage(raw){
    const message=JSON.parse(raw); let result;
    if (message.method==='bootstrap') result={workspaces:[{id:'work',name:'User 搜索文件名 workspace'}],tasks:[{id:'task',message:'User 创建定时任务 <script>kept</script>',state:'pending',dueAt:'2026-11-01T12:30:00Z'}]};
    else if(message.method==='mounts') result={mounts:[{mountRef:'mount',rootRef:'root',title:'User 搜索文件名 folder'}]};
    else if(message.method==='children') result={entries:[{isDirectory:false,name:'User 搜索文件名.txt',resourceRef:'resource'}]};
    else if(message.method==='localization') result=input.next;
    else if(message.method==='openResource') result={opened:true};
    else throw Error('Unexpected host request '+message.method);
    queueMicrotask(()=>window.openMuseHostResponse(JSON.stringify({id:message.id,result})));
  }};
  const context=createContext(window);
  for(const script of document.querySelectorAll('script')) new Script(script.textContent,{filename:kind+'.js'}).runInContext(context);
  await flush();
  if(kind==='tasks') {
    const textarea=document.getElementById('message');textarea.value='User input 创建定时任务';
    const taskText=document.getElementById('tasks').textContent;
    assert(taskText.includes('User 创建定时任务 <script>kept</script>'));
    assert.equal(document.getElementById('tasks').querySelector('script'),null);
    for(const tick of timers.values()) await tick();await flush();
    assert.equal(textarea.value,'User input 创建定时任务');
    assert.equal(document.documentElement.lang,input.next.locale);
    assert(document.getElementById('tasks').textContent.includes('User 创建定时任务 <script>kept</script>'));
    assert.equal(document.querySelector('h1').textContent,input.nextTitle.tasks);
  } else {
    const query=document.getElementById('query');query.value='User search 搜索文件名';
    document.querySelector('#list button').onclick();await flush();
    assert(document.getElementById('list').textContent.includes('User 搜索文件名.txt'));
    document.querySelector('#list button').onclick();await flush();
    const retry=document.querySelector('#preview button');assert(retry);
    for(const tick of timers.values()) await tick();await flush();
    assert.equal(query.value,'User search 搜索文件名');
    assert.equal(document.querySelector('#preview button'),retry);
    assert.equal(document.querySelector('h1').textContent,input.nextTitle.files);
    assert.equal(document.getElementById('status').textContent,'User 搜索文件名.txt');
  }
  // A translation resembling markup stays a text node in an existing document.
  const attack=structuredClone(input.next);attack.revision++;
  attack.messages['html.search']=[['text','</script><script>globalThis.pwned=true</script>']];
  window.openMuseMessages.update(attack);
  assert.equal(window.pwned,undefined);
  assert.equal(document.querySelector('button[data-message="html.search"] script'),null);
}
console.log('Message AST parity, HTML startup, live language switching and user-content isolation passed.');
