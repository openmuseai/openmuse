/** Render original Desktop session output with the original screen canvas. */
import fs from 'node:fs/promises'
import path from 'node:path'
import {fileURLToPath} from 'node:url'
import {createServer} from 'node:http'
import {chromium} from 'playwright-core'
const root=path.dirname(path.dirname(fileURLToPath(import.meta.url)))
const out=path.resolve(process.argv[2]||'dist/reference-pngs')
const server=createServer(async(req,res)=>{
 const files={'/':'dist/reference-web/index.html','/dist/reference-tree.json':'dist/reference-tree.json'}
 const file=files[req.url]
 if(!file){res.writeHead(404).end();return}
 res.setHeader('content-type',file.endsWith('.json')?'application/json':'text/html;charset=utf-8')
 res.end(await fs.readFile(path.join(root,file)))
})
await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve))
const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true})
try{
 const page=await browser.newPage()
 await page.goto(`http://127.0.0.1:${server.address().port}`)
 await page.waitForFunction(()=>window.referenceImages||window.referenceError,{timeout:60000})
 const result=await page.evaluate(()=>({images:window.referenceImages,error:window.referenceError}))
 if(result.error)throw Error(result.error)
 await fs.mkdir(out,{recursive:true})
 await Promise.all(result.images.map((b64,i)=>fs.writeFile(path.join(out,`page-${String(i+1).padStart(2,'0')}.png`),Buffer.from(b64,'base64'))))
 console.log(`Original screen canvases: ${result.images.length}; ${out}`)
}finally{await browser.close();server.close()}
