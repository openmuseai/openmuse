// Test-only renderer: use GenOffice's unchanged export canvas on RenderSlides
// produced by its unchanged Desktop session builder and system-font metrics.
import './environment'
import {renderSlidesToPngBase64} from '../../vendor/genoffice/apps/slides/src/renderer/export-render'
import '../../vendor/genoffice/apps/slides/src/renderer/styles.css'
async function main(){
 const slides=await(await fetch('/dist/reference-tree.json')).json()
 const urls=new Set<string>()
 function collect(v:any){if(!v||typeof v!=='object')return;for(const value of Object.values(v)){if(typeof value==='string'&&value.startsWith('data:image/'))urls.add(value);else if(value&&typeof value==='object')collect(value)}}
 collect(slides)
 const images=new Map<string,HTMLImageElement>()
 await Promise.all(Array.from(urls).map(url=>new Promise<void>(resolve=>{const img=new Image();img.onload=()=>{images.set(url,img);resolve()};img.onerror=()=>resolve();img.src=url})))
 await document.fonts.ready
 ;(window as any).referenceImages=await renderSlidesToPngBase64(slides,images,0.75)
}
void main().catch(e=>{(window as any).referenceError=String(e)})
