import * as esbuild from 'esbuild'
import fs from 'node:fs'
import path from 'node:path'
const root=process.cwd(),upstream=path.join(root,'vendor/genoffice')
const packages=new Map()
for(const name of fs.readdirSync(path.join(upstream,'packages'))){const file=path.join(upstream,'packages',name,'package.json');if(fs.existsSync(file)){const p=JSON.parse(fs.readFileSync(file));packages.set(p.name,{root:path.dirname(file),exports:p.exports})}}
await esbuild.build({entryPoints:['web/src/reference.ts'],outfile:'dist/reference.cjs',bundle:true,platform:'node',format:'cjs',loader:{'.md':'text'},plugins:[{name:'upstream',setup(build){
 build.onResolve({filter:/harfbuzzjs.*harfbuzz\.js$/},()=>({path:path.join(root,'node_modules/harfbuzzjs/dist/harfbuzz.js'),external:true}))
 build.onResolve({filter:/^electron$/},()=>({path:'electron',namespace:'stub'}))
 build.onLoad({filter:/.*/,namespace:'stub'},()=>({contents:'export const BrowserWindow={};export const webContents={};'}))
 build.onResolve({filter:/^@genoffice\//},args=>{const parts=args.path.split('/'),p=packages.get(parts.slice(0,2).join('/'));if(!p)return;let sub=parts.length===2?'.':'./'+parts.slice(2).join('/');let rel=p.exports[sub];if(!rel){for(const [k,v] of Object.entries(p.exports)){if(k.endsWith('*')&&sub.startsWith(k.slice(0,-1)))rel=v.replace('*',sub.slice(k.length-1))}}if(!rel)throw Error(args.path);const target=path.resolve(p.root,rel.split('?')[0]);return {path:target,...(args.path.includes('?asset')?{namespace:'asset'}:{})}})
 build.onLoad({filter:/.*/,namespace:'asset'},args=>({contents:`export default ${JSON.stringify(args.path)}`}))
 build.onResolve({filter:/\?asset$/},args=>({path:path.resolve(args.resolveDir,args.path.split('?')[0]),namespace:'asset'}))
}}]})
