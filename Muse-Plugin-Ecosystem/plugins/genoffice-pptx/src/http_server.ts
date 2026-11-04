import { createServer } from 'node:http'
import { createMcpHandler } from '@modelcontextprotocol/server'
import { localhostHostValidation, localhostOriginValidation, toNodeHandler } from '@modelcontextprotocol/node'
import { buildServer } from './server_factory.js'

const handler = toNodeHandler(createMcpHandler(buildServer))
const validHost = localhostHostValidation()
const validOrigin = localhostOriginValidation()
const publicOrigin = process.env.PPTX_PUBLIC_ORIGIN
const publicHost = publicOrigin ? new URL(publicOrigin).host : null
const port = Number(process.env.PORT || 8767)
createServer((request, response) => {
  if (request.url?.split('?')[0] !== '/mcp') {
    response.writeHead(404).end()
    return
  }
  if (publicHost) {
    if (request.headers.host !== publicHost || (request.headers.origin && request.headers.origin !== publicOrigin)) {
      response.writeHead(403).end(); return
    }
  } else if (!validHost(request, response) || !validOrigin(request, response)) return
  void handler(request, response)
}).listen(port, '127.0.0.1', () => {
  process.stderr.write(`MCP PPTX remote on http://127.0.0.1:${port}/mcp\n`)
})
