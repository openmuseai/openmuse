import * as esbuild from 'esbuild'
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
const root = path.dirname(path.dirname(fileURLToPath(import.meta.url)))
const upstream = path.join(root, 'vendor/genoffice')
const output = path.join(root,process.env.GENOFFICE_WEB_OUT || 'dist/web')
const packages = new Map()
for (const name of fs.readdirSync(path.join(upstream, 'packages'))) {
  const file = path.join(upstream, 'packages', name, 'package.json')
  if (fs.existsSync(file)) { const p = JSON.parse(fs.readFileSync(file)); packages.set(p.name, {root:path.dirname(file),exports:p.exports}) }
}
fs.mkdirSync(output, {recursive:true})
const result = await esbuild.build({
  absWorkingDir: root, entryPoints:{main:process.env.GENOFFICE_WEB_ENTRY || 'web/src/main.ts'}, bundle:true, outdir:output,
  platform:'browser', format:'iife', target:['safari17','chrome120'],
  inject:[path.join(root,'web/shims/globals.ts')], jsx:'automatic', minify:true,
  loader:{'.md':'text','.png':'dataurl','.jpg':'dataurl','.jpeg':'dataurl','.svg':'dataurl','.woff':'dataurl','.ttf':'dataurl','.woff2':'dataurl','.wasm':'dataurl'},
  assetNames:'assets/[name]-[hash]', metafile:true,
  alias:{'electron':path.join(root,'web/shims/electron.ts'),'node:path':path.join(root,'web/shims/path.ts'),'pngjs':path.join(root,'node_modules/pngjs/browser.js'),'node:crypto':path.join(root,'web/shims/crypto.ts'),'node:zlib':path.join(root,'web/shims/zlib.ts'),'node:fs':path.join(root,'web/shims/unavailable.ts'),'node:stream/promises':path.join(root,'web/shims/unavailable.ts')},
  plugins:[{name:'upstream-packages',setup(build){
    // Keep the upstream checkout unchanged; substitute only its presentation adapter.
    build.onLoad({filter:/\/renderer\/i18n\/locale\.tsx$/},async()=>({contents:await fs.promises.readFile(path.join(root,'web/src/locale-adapter.tsx'),'utf8'),loader:'tsx',resolveDir:path.join(root,'web/src')}))
    // Product integration overlays only AI surfaces; parser, session builder,
    // original React editor and Konva rendering are unchanged.
    if (!process.env.GENOFFICE_WEB_ENTRY) {
      // Add only an integration callback to the existing editor state setter.
      build.onLoad({filter:/\/renderer\/App\.tsx$/},async args=>{
        let source=await fs.promises.readFile(args.path,'utf8')
        source=source.replaceAll('<GensparkMark', '<OpenMuseMark').replaceAll('>Genspark AI<', '>OpenMuse<')
        source=`import {OpenMuseMark} from ${JSON.stringify(path.join(root,'web/src/brand-mark.tsx'))};\n`+source
        if(!source.includes('<AiPanel\n'))throw Error('Upstream navigation contract changed')
        return {contents:source.replace('<AiPanel\n','<AiPanel onNavigate={setCurrent}\n'),loader:'tsx',resolveDir:path.dirname(args.path)}
      })
      build.onResolve({filter:/^\.\/ai\/AiPanel$/},()=>({path:path.join(root,'web/src/agent-panel.tsx')}))
      build.onLoad({filter:/\/components\/RibbonHomeTab\.tsx$/},async args=>{
        let source=await fs.promises.readFile(args.path,'utf8')
        const start=source.indexOf('<Group label="Genspark AI">')
        const end=source.indexOf('</Group>',start)
        if(start<0||end<0)throw new Error('Upstream AI toolbar contract changed')
        source=source.slice(0,start)+'<Group label="Agent"><AgentButton /></Group>'+source.slice(end+8)
        return {contents:`import {AgentButton} from ${JSON.stringify(path.join(root,'web/src/agent-button.tsx'))};\n`+source,loader:'tsx',resolveDir:path.dirname(args.path)}
      })
    }
    build.onResolve({filter:/^\.\/fonts$/},args=>args.importer.endsWith('/main/session-state.ts')?{path:path.join(root,'web/shims/desktop-fonts.ts')}:undefined)
    build.onResolve({filter:/^@genoffice\//},args=>{
      const parts=args.path.split('/'); const name=parts.slice(0,2).join('/'); const p=packages.get(name)
      if(!p) return
      const sub=parts.length===2?'.':'./'+parts.slice(2).join('/')
      let rel=typeof p.exports==='string'?p.exports:p.exports[sub]
      if(!rel) {for(const [key,value] of Object.entries(p.exports)){if(key.endsWith('*')&&sub.startsWith(key.slice(0,-1))) rel=value.replace('*',sub.slice(key.length-1))}}
      if(!rel) throw new Error(`Unknown upstream export ${args.path}`)
      return {path:path.resolve(p.root,rel)}
    })
  }}],
})
fs.writeFileSync(path.join(root,'dist/web-metafile.json'),JSON.stringify(result.metafile,null,2))
const html = fs.readFileSync(path.join(root,'web/index.html'),'utf8').replace('<link rel="stylesheet" href="./main.css">', ()=>'<style>'+fs.readFileSync(path.join(output,'main.css'),'utf8')+'</style>').replace('<script src="./main.js"></script>', ()=>'<script>try{'+fs.readFileSync(path.join(output,'main.js'),'utf8').replaceAll('</script','<\\/script')+'}catch(error){document.getElementById("root").textContent=String(error.stack||error);console.error(error)}</script>')
fs.writeFileSync(path.join(output,'index.html'),html)
