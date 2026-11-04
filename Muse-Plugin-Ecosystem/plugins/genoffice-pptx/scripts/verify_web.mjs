/** Independent, standard MCP Apps test Host. No OpenMuse imports or fields. */
import fs from 'node:fs/promises'
import path from 'node:path'
import {createServer} from 'node:http'
import {fileURLToPath} from 'node:url'
import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {chromium} from 'playwright-core'
import {PNG} from 'pngjs'
import pixelmatch from 'pixelmatch'
import JSZip from 'jszip'
const root=path.dirname(path.dirname(fileURLToPath(import.meta.url)))
const sample=process.argv[2]
if(!sample)throw Error('Usage: node scripts/verify_web.mjs <sample.pptx> [output-dir] [reference-png-dir]')
const out=path.resolve(process.argv[3]||'dist/verification')
const reference=process.argv[4]
await fs.mkdir(out,{recursive:true})
const args={name:path.basename(sample),content:(await fs.readFile(sample)).toString('base64')}
const ui=await fs.readFile(process.env.MCP_APP_HTML?path.resolve(process.env.MCP_APP_HTML):path.join(root,'dist/web/index.html'),'utf8')
const host=`<!doctype html><meta charset="utf-8"><style>html,body,iframe{margin:0;width:100%;height:100%;border:0;overflow:hidden}</style><iframe id="view" sandbox="allow-scripts allow-downloads"></iframe><script>
const frame=document.getElementById('view');
window.pending=new Map();window.seq=0;window.callView=(method,params={})=>new Promise((resolve,reject)=>{const id="tck:"+(++window.seq);window.pending.set(id,{resolve,reject});frame.contentWindow.postMessage({jsonrpc:"2.0",id,method,params},"*")});window.input=${JSON.stringify(args).replaceAll('<','\\u003c')};window.download=null;
window.sendInput=()=>frame.contentWindow.postMessage({jsonrpc:'2.0',method:'ui/notifications/tool-input',params:{arguments:window.input}},'*');
addEventListener('message',event=>{if(event.source!==frame.contentWindow)return;const m=event.data;if(m?.jsonrpc!=='2.0')return;if(!m.method&&window.pending.has(m.id)){const p=window.pending.get(m.id);window.pending.delete(m.id);m.error?p.reject(m.error):p.resolve(m.result);return}const reply=result=>frame.contentWindow.postMessage({jsonrpc:'2.0',id:m.id,result},'*');
if(m.method==='ui/initialize')reply({protocolVersion:'2026-01-26',hostInfo:{name:'Independent MCP Apps TCK',version:'1.0.0'},hostCapabilities:{downloadFile:{},message:{text:{}},updateModelContext:{text:{}}},hostContext:{theme:'light',locale:'zh-Hans',platform:'desktop',displayMode:'inline'}});
if(m.method==='ui/notifications/initialized')window.sendInput();
if(m.method==='ui/update-model-context'){window.context=m.params;reply({});}if(m.method==='ui/message'){window.agentMessage=m.params;reply({});}if(m.method==='ui/download-file'){window.download=m.params;reply({});}
});frame.srcdoc=new TextDecoder().decode(Uint8Array.from(atob('${Buffer.from(ui).toString('base64')}'),c=>c.charCodeAt(0)));
</script>`
const server=createServer((req,res)=>res.end(host))
await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve))
const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true})
try{
 const page=await browser.newPage({viewport:{width:1351,height:904},deviceScaleFactor:1})
 const errors=[];page.on('pageerror',e=>errors.push(e.message))
 await page.goto(`http://127.0.0.1:${server.address().port}`)
 const view=page.frameLocator('#view')
 await view.locator('html[data-pptx-ready]').waitFor({timeout:60000})
 const count=Number(await view.locator('html').getAttribute('data-pptx-ready'))
 assert(count>0);await page.waitForTimeout(1000)
 const report=[]
 for(let i=0;i<count;i++){
  await view.locator('.thumb').nth(i).evaluate(el=>el.click())
  await view.getByText(`幻灯片 ${i+1} / ${count}`,{exact:true}).waitFor()
  await view.locator('.stage-rel').evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))))
  await page.waitForTimeout(150)
  const b64=await view.locator('.stage-rel').evaluate(el=>{
   const out=document.createElement('canvas');out.width=960;out.height=540;const c=out.getContext('2d')
   for(const layer of el.querySelectorAll('.konvajs-content canvas'))if(layer.style.display!=='none')c.drawImage(layer,160,160,1280,720,0,0,960,540)
   return out.toDataURL('image/png').split(',')[1]
  })
  const id=String(i+1).padStart(2,'0'),data=Buffer.from(b64,'base64')
  await fs.writeFile(path.join(out,`plugin-${id}.png`),data)
  const entry={page:i+1}
  if(reference){
   const a=PNG.sync.read(data),b=PNG.sync.read(await fs.readFile(path.join(reference,`page-${id}.png`))),diff=new PNG({width:960,height:540})
   entry.differentPixelRatio=pixelmatch(a.data,b.data,diff.data,960,540,{threshold:0.15})/(960*540)
   await fs.writeFile(path.join(out,`diff-${id}.png`),PNG.sync.write(diff))
  }
  report.push(entry)
 }
 // Discover and invoke the View's tools through the standard MCP Apps channel.
 const toolList=await page.evaluate(()=>window.callView('tools/list'))
 assert(toolList.tools.some(t=>t.name==='pptx_edit_text'))
 const callTool=async(name,args={})=>page.evaluate(({name,args})=>window.callView('tools/call',{name,arguments:args}),{name,args})
 const readDeck=async()=>JSON.parse((await callTool('pptx_read_deck')).content[0].text)
 const state=JSON.parse((await callTool('pptx_get_state')).content[0].text)
 assert.equal(state.pageCount,count)
 assert(!(await callTool('pptx_go_to_slide',{slideIndex:0})).isError)
 const first=JSON.parse((await callTool('pptx_read_page',{slideIndex:0})).content[0].text)
 assert.equal(first.index,0);assert(first.text.length>0)
 assert(!(await callTool('pptx_go_to_slide',{slideIndex:count-1})).isError)
 assert.equal(JSON.parse((await callTool('pptx_get_state')).content[0].text).slideIndex,count-1)
 const deck=await readDeck()
 const candidate=deck.slides.flatMap(slide=>slide.elements.map(el=>({...el,slideIndex:slide.index}))).find(el=>el.text?.includes('20XX'))
 assert(candidate,'Sample title element must be discoverable')
 const agentText='OpenMuse Agent verified'
 const changed=await callTool('pptx_edit_text',{slideIndex:candidate.slideIndex,elementId:candidate.elementId,expectedRevision:deck.revision,text:agentText})
 assert(!changed.isError,JSON.stringify(changed))
 const stale=await callTool('pptx_set_notes',{slideIndex:0,expectedRevision:deck.revision,text:'stale overwrite'})
 assert.equal(stale.isError,true,'A stale agent must not overwrite a concurrent edit')
 const current=await readDeck()
 assert(current.slides[candidate.slideIndex].elements.some(el=>el.text===agentText))
 const saveResult=await callTool('pptx_save',{expectedRevision:current.revision})
 assert(!saveResult.isError,JSON.stringify(saveResult))
 const agentBlob=await page.evaluate(()=>window.download.contents[0].resource.blob)
 const agentBytes=Buffer.from(agentBlob,'base64')
 const agentZip=await JSZip.loadAsync(agentBytes)
 assert((await agentZip.file(`ppt/slides/slide${candidate.slideIndex+1}.xml`).async('text')).includes(agentText))
 await fs.writeFile(path.join(out,'agent-edited.pptx'),agentBytes)
 await view.locator('.thumb').nth(candidate.slideIndex).evaluate(el=>el.click())
 await page.screenshot({path:path.join(out,'agent-edited.png')})
 await view.getByRole('button',{name:'Agent',exact:true}).click()
 await page.waitForFunction(()=>window.agentMessage!==undefined)
 assert((await page.evaluate(()=>window.agentMessage)).content.length>0,'Agent request must use standard ui/message')
 assert.equal(await view.locator('.ai-dock:visible').count(),0,'Original chat panel must be absent')
 const undo=await callTool('pptx_undo',{expectedRevision:current.revision})
 assert(!undo.isError)
 const restored=await readDeck()
 assert(restored.slides[candidate.slideIndex].elements.some(el=>el.text===candidate.text))
 await page.evaluate(()=>window.download=null)
 // Exercise the original text-edit overlay, then the standard download channel.
 await view.locator('.thumb').first().evaluate(el=>el.click())
 await view.getByText(`幻灯片 1 / ${count}`,{exact:true}).waitFor()
 const bounds=await view.locator('.stage-zoom-box').boundingBox()
 await page.mouse.dblclick(bounds.x+80*bounds.width/1280,bounds.y+350*bounds.width/1280)
 const editor=view.locator('.slide-text-editor');await editor.waitFor()
 const replacement='OpenMuse pipeline roundtrip'
 await editor.fill(replacement);await editor.press('Escape')
 await view.getByRole('button',{name:/^保存 \(/}).click()
 await page.waitForFunction(()=>window.download!==null)
 const download=await page.evaluate(()=>window.download.contents[0].resource)
 const saved=Buffer.from(download.blob,'base64')
 const zip=await JSZip.loadAsync(saved)
 const xml=await Promise.all(Object.values(zip.files).filter(f=>/^ppt\/slides\/slide\d+\.xml$/.test(f.name)).map(f=>f.async('text')))
 assert(xml.some(value=>value.includes(replacement)),'Saved OOXML must contain the text edit')
 await fs.writeFile(path.join(out,'edited.pptx'),saved)
 await page.evaluate(content=>{window.input.content=content;window.sendInput()},saved.toString('base64'))
 await view.getByText(`幻灯片 1 / ${count}`,{exact:true}).waitFor()
 await view.locator('html').evaluate(async(_,expected)=>{
  for(let attempt=0;attempt<100;attempt++) {
   const slide=(await window.slidesApi.consumePendingOpen(1280)).slides[0]
   const text=slide.nodes.flatMap(n=>n.text?.lines.flatMap(l=>l.runs.map(r=>r.text))||[]).join('')
   if(text.replace(/\s/g,'').includes(expected.replace(/\s/g,'')))return
   await new Promise(resolve=>setTimeout(resolve,100))
  }
  throw Error('Reopened render tree does not contain saved text')
 },replacement)
 await page.screenshot({path:path.join(out,'desktop-editor.png')})
 await page.setViewportSize({width:412,height:820})
 await view.locator('.thumb').nth(Math.min(3,count-1)).evaluate(el=>el.click())
 await view.getByText(`幻灯片 ${Math.min(4,count)} / ${count}`,{exact:true}).waitFor()
 await page.screenshot({path:path.join(out,'mobile-editor.png')})
 assert.deepEqual(errors,[])
 const result={sample:path.basename(sample),sampleSha256:createHash('sha256').update(await fs.readFile(sample)).digest('hex'),uiSha256:createHash('sha256').update(ui).digest('hex'),slides:count,renderedPages:report.length,textEditSavedAndReopened:true,viewToolsDiscovered:true,agentEditSaved:true,staleRevisionRejected:true,agentEditUndoable:true,standardAgentMessage:true,genSparkPanelRemoved:true,protocol:'MCP Apps',pages:report}
 await fs.writeFile(path.join(out,'result.json'),JSON.stringify(result,null,2)+'\n')
 console.log(JSON.stringify({...result,pages:undefined,maxDifferentPixelRatio:reference?Math.max(...report.map(p=>p.differentPixelRatio||0)):null},null,2))
}finally{await browser.close();server.close()}
