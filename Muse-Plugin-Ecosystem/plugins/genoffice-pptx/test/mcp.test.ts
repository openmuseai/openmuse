import test from 'node:test'
import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { join } from 'node:path'
import { Client } from '@modelcontextprotocol/client'
import { StdioClientTransport } from '@modelcontextprotocol/client/stdio'

test('standard MCP client discovers and calls the packaged PPTX tool', async () => {
  const server = join(import.meta.dirname, '../dist/server.mjs')
  const fixture = join(import.meta.dirname, '../vendor/pptx-engine/tests/fixtures/01_standard_business.pptx')
  const client = new Client({ name: 'generic-mcp-tck', version: '1.0.0' })
  const transport = new StdioClientTransport({ command: process.execPath, args: [server] })
  try {
    await client.connect(transport, { mode: 'auto' })
    const listed = await client.listTools()
    const tool = listed.tools.find(tool => tool.name === 'inspect_pptx')
    assert.ok(tool)
    assert.equal((tool.inputSchema as any).properties?.content?.contentEncoding, 'base64')
    const result = await client.callTool({
      name: 'inspect_pptx',
      arguments: { content: (await readFile(fixture)).toString('base64') },
    })
    assert.equal(result.isError, undefined)
    assert.ok(Number(result.structuredContent?.slides) > 0)
  } finally {
    await transport.close()
  }
})
