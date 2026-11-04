import test from 'node:test'
import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import { createServer } from 'node:net'
import { readFile } from 'node:fs/promises'
import { join } from 'node:path'
import { Client, StreamableHTTPClientTransport } from '@modelcontextprotocol/client'

async function freePort(): Promise<number> {
  const server = createServer()
  await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve))
  const port = (server.address() as { port: number }).port
  await new Promise<void>(resolve => server.close(() => resolve()))
  return port
}

test('remote MCP serves MCP Apps resource and PPTX tool over HTTP', async () => {
  const port = await freePort()
  const server = spawn(process.execPath, [join(import.meta.dirname, '../dist/http.mjs')], {
    env: { ...process.env, PORT: String(port) }, stdio: ['ignore', 'ignore', 'pipe'],
  })
  try {
    await new Promise<void>((resolve, reject) => {
      server.stderr.once('data', () => resolve())
      server.once('error', reject)
    })
    const client = new Client({ name: 'mobile-mcp-tck', version: '1.0.0' })
    const transport = new StreamableHTTPClientTransport(new URL(`http://127.0.0.1:${port}/mcp`))
    try {
      await client.connect(transport, { mode: 'auto' })
      const resources = await client.listResources()
      assert.ok(resources.resources.some(resource => resource.uri === 'ui://genoffice-pptx/editor'))
      const ui = await client.readResource({ uri: 'ui://genoffice-pptx/editor' })
      assert.equal(ui.contents[0].mimeType, 'text/html;profile=mcp-app')
      const fixture = join(import.meta.dirname, '../vendor/pptx-engine/tests/fixtures/01_standard_business.pptx')
      const result = await client.callTool({ name: 'inspect_pptx', arguments: {
        content: (await readFile(fixture)).toString('base64'),
      } })
      assert.ok(Number(result.structuredContent?.slides) > 0)
      assert.ok(ui.contents.some(item => 'text' in item && item.text.includes('OpenMuse Slides')))
    } finally {
      await transport.close()
    }
  } finally {
    server.kill()
  }
})
