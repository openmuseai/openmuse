import { createHash } from 'node:crypto'
import { openPptx, getSlideNotes } from '@genoffice/pptx-engine'
import { McpServer } from '@modelcontextprotocol/server'
import { z } from 'zod'
import metadata from '../config/package-metadata.json'

export const slidesContract = {
  id: metadata.slidesContractId,
  version: '1.0.1', title: 'OpenMuse Slides · file snapshots',
  mediaType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
  operations: { readSnapshot: 'slides_read_snapshot' }, tools: ['slides_read_snapshot'],
  semantics: 'Immutable authorized file snapshot, not the unsaved active editor. Zero-based slide indices.',
}
const page = z.object({ index:z.number().int().nonnegative(), id:z.string(), text:z.string(), notes:z.string() })
export const slidesSnapshot = z.object({ name:z.string(), revision:z.string(), pageCount:z.number().int(),
  status:z.enum(['text','no-text-layer']), pages:z.array(page), source:z.literal('genoffice-pptx-engine') })
function textOf(element:any):string {
  const paragraphs=element.text?.paragraphs ?? element.textBody?.paragraphs ?? element.paragraphs
  const own=paragraphs?.map((p:any)=>p.runs?.map((r:any)=>r.text??'').join('')??'').join('\n')??''
  const child=(element.children??[]).map(textOf).join('\n')
  const rows=(element.rows??[]).flatMap((row:any)=>(row.cells??row).map?.(textOf)??[]).join('\n')
  return [own,child,rows].filter(Boolean).join('\n')
}
export function registerSlidesApi(server:McpServer) {
  server.registerResource('Public capability contracts','capability://genoffice-pptx/contracts',
    {mimeType:'application/vnd.openmuse.capabilities+json'},async()=>({contents:[{
      uri:'capability://genoffice-pptx/contracts',mimeType:'application/vnd.openmuse.capabilities+json',
      text:JSON.stringify({profile:'openmuse-capabilities/1',contracts:[slidesContract]})}]}))
  server.registerTool('slides_read_snapshot',{
    description:'Read plain text and notes from an authorized PPTX file. This does not read unsaved editor changes.',
    inputSchema:z.object({content:z.string().max(90*1024*1024).meta({contentEncoding:'base64',contentMediaType:slidesContract.mediaType}),name:z.string().max(180).optional()}),
    outputSchema:slidesSnapshot, annotations:{readOnlyHint:true,openWorldHint:false},
  },async({content,name})=>{
    const bytes=decodePptxContent(content)
    const opened=await openPptx(bytes)
    if(opened.deck.slides.length>500)throw Error('PAGE_BUDGET_EXCEEDED')
    const pages=opened.deck.slides.map((slide,index)=>({index,id:slide.path,
      text:slide.elements.map(textOf).filter(Boolean).join('\n'),notes:getSlideNotes(opened.archive,slide.path)}))
    const output=slidesSnapshot.parse({name:name??'Presentation',revision:createHash('sha256').update(bytes).digest('hex'),
      pageCount:pages.length,status:pages.some(p=>p.text.trim())?'text':'no-text-layer',pages,source:'genoffice-pptx-engine'})
    if(JSON.stringify(output).length>2*1024*1024)throw Error('TEXT_BUDGET_EXCEEDED')
    return {content:[{type:'text',text:JSON.stringify(output)}],structuredContent:output}
  })
}

// Avoid a recursive base64 regex in the public JSON Schema: large valid files
// can overflow otherwise conforming clients' regex stacks. Validate canonically
// in the provider with bounded native Buffer operations.
export function decodePptxContent(content:string):Buffer {
  if(content.length>90*1024*1024)throw Error('FILE_TOO_LARGE')
  const bytes=Buffer.from(content,'base64')
  if(bytes.length>64*1024*1024)throw Error('FILE_TOO_LARGE')
  if(bytes.toString('base64')!==content)throw Error('INVALID_BASE64')
  return bytes
}
