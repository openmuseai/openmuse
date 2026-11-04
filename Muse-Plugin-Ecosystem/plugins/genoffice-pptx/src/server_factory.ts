import { readFile } from 'node:fs/promises'
import { McpServer } from '@modelcontextprotocol/server'
import { z } from 'zod'
import catalog from '../i18n/catalog.json'
import packageInfo from '../package.json'
import { inspectBytes } from './worker.js'
import { registerSlidesApi, decodePptxContent } from './public_api.js'

const uiResource = 'ui://genoffice-pptx/editor'
const mimeType = 'text/html;profile=mcp-app'

export function buildServer() {
  const server = new McpServer(
    { name: 'io.openmuse/genoffice-pptx', version: packageInfo.version },
    { capabilities: { tools: {}, resources: {} } },
  )
  server.registerResource(
    'OpenMuse Slides editor', uiResource,
    { title: 'OpenMuse Slides editor', mimeType },
    async () => {
      let text: string
      try {text=await readFile(new URL('./web/index.html',import.meta.url),'utf8')}
      catch {text=await readFile(new URL('../dist/web/index.html',import.meta.url),'utf8')}
      return {contents:[{uri:uiResource,mimeType,text}]}
    },
  )
  server.registerTool(
    'inspect_pptx',
    {
      title: 'Inspect PowerPoint slides',
      description: 'Inspect a user-selected PPTX document and return its slide structure.',
      inputSchema: z.object({ content: z.string().max(90*1024*1024).meta({
        contentEncoding: 'base64',
        contentMediaType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      }), name: z.string().optional() }),
      outputSchema: z.object({
        slides: z.number(),
        size: z.unknown(),
        sha256: z.string(),
        pages: z.array(z.object({ index: z.number(), elements: z.number() })),
      }),
      _meta: { ui: { resourceUri: uiResource, mimeType } },
    },
    async ({ content }) => {
      const output = await inspectBytes(decodePptxContent(content))
      return {
        content: [
          { type: 'text' as const, text: JSON.stringify(output) },

        ],
        structuredContent: output,
        _meta: { ui: { resourceUri: uiResource, mimeType } },
      }
    },
  )
  server.registerResource('Plugin language contract','i18n://genoffice-pptx/catalog',
    {mimeType:'application/vnd.openmuse.message-catalog+json'},async()=>({contents:[{uri:'i18n://genoffice-pptx/catalog',mimeType:'application/vnd.openmuse.message-catalog+json',text:JSON.stringify(catalog)}]}))
  registerSlidesApi(server)
  return server
}
