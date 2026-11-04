import fs from 'node:fs/promises'
import {createServer} from 'node:http'
import {fileURLToPath} from 'node:url'
import path from 'node:path'
import assert from 'node:assert/strict'
import {chromium} from 'playwright-core'
const root=path.dirname(path.dirname(fileURLToPath(import.meta.url)))
const sample=process.argv[2]
if(!sample)throw Error('Pass a reviewed PPTX fixture')
const ui=await fs.readFile(path.join(root,'dist/web/index.html'),'utf8')
const input={name:'acceptance.pptx',content:(await fs.readFile(sample)).toString('base64')}
const host=`<!doctype html><iframe id="view" sandbox="allow-scripts allow-downloads" style="width:100vw;height:100vh;border:0"></iframe><script>
const frame=document.getElementById('view');
window.locale=tag=>frame.contentWindow.postMessage({jsonrpc:'2.0',method:'ui/notifications/host-context-changed',params:{locale:tag,theme:'light'}},'*');
addEventListener('message',event=>{if(event.source!==frame.contentWindow)return;const m=event.data;const reply=result=>frame.contentWindow.postMessage({jsonrpc:'2.0',id:m.id,result},'*');
if(m.method==='ui/initialize')reply({protocolVersion:'2026-01-26',hostInfo:{name:'Language acceptance',version:'1.0.0'},hostCapabilities:{downloadFile:{}},hostContext:{locale:'en',theme:'light',platform:'mobile',displayMode:'inline'}});
if(m.method==='ui/notifications/initialized')frame.contentWindow.postMessage({jsonrpc:'2.0',method:'ui/notifications/tool-input',params:{arguments:${JSON.stringify(input)}}},'*');});
frame.srcdoc=new TextDecoder().decode(Uint8Array.from(atob('${Buffer.from(ui).toString('base64')}'),c=>c.charCodeAt(0)));
</script>`
const server=createServer((req,res)=>res.end(host));await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve))
const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true})
try{
 const page=await browser.newPage({viewport:{width:420,height:900}})
 await page.goto(`http://127.0.0.1:${server.address().port}`)
 const frame=page.frameLocator('#view')
 await frame.locator('html[data-pptx-ready]').waitFor({timeout:60000})
 const count=await frame.locator('html').getAttribute('data-pptx-ready')
 await frame.locator('.ribbon-tab').filter({hasText:/^Home$/}).waitFor({timeout:15000})
 await page.evaluate(()=>window.locale('zh-Hans'))
 await frame.locator('.ribbon-tab').filter({hasText:/^开始$/}).waitFor({timeout:5000})
 assert.equal(await frame.locator('html').getAttribute('data-pptx-ready'),count)
 await page.evaluate(()=>window.locale('en'))
 await frame.locator('.ribbon-tab').filter({hasText:/^Home$/}).waitFor({timeout:5000})
 console.log('GenOffice live language switch: PASS (same mounted deck)')
}finally{await browser.close();await new Promise(resolve=>server.close(resolve))}
