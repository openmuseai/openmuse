/** Export ordinary MCP discovery and ResourceContents from a built stdio server.
 * Public offline profile documentation: contracts/offline-mcp-app-v1/.
 * Never execute a downloaded Node server on Mobile. */
import {Client} from '@modelcontextprotocol/client'
import {StdioClientTransport} from '@modelcontextprotocol/client/stdio'
import {writeFile} from 'node:fs/promises'
export async function exportOffline(server,output){
 const client=new Client({name:'offline-discovery-export',version:'1.0.0'})
 const transport=new StdioClientTransport({command:process.execPath,args:[server],maxBufferSize:64*1024*1024})
 try{
  await client.connect(transport,{mode:'auto'})
  const tools=(await client.listTools()).tools,resources=[]
  for(const resource of (await client.listResources()).resources){resources.push(...(await client.readResource({uri:resource.uri})).contents)}
  await writeFile(output,JSON.stringify({tools,resources}))
 }finally{await transport.close()}
}
if(process.argv[1]?.endsWith('/export-offline.mjs'))await exportOffline(process.argv[2],process.argv[3])
