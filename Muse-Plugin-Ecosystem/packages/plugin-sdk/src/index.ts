import { App } from '@modelcontextprotocol/ext-apps'
export type Provider = {id:string,version:string,sha256:string,contracts:Contract[],tools:any[]}
export type Contract = {id:string,version:string,title:string,mediaType:string,operations:Record<string,string>,tools:string[]}
export class CapabilityClient {
  readonly app=new App({name:'OpenMuse public capability client',version:'0.1.0'})
  async connect(){await this.app.connect();if(!this.app.getHostCapabilities()?.serverTools)throw Error('HOST_UPDATE_REQUIRED: serverTools unavailable')}
  async call(name:string,args:Record<string,unknown>={}) {
    const result=await this.app.callServerTool({name,arguments:args},{timeout:120000})
    if(result.isError)throw Error(result.content.filter(c=>c.type==='text').map((c:any)=>c.text).join('\n'))
    if(!result.structuredContent)throw Error('MISSING_STRUCTURED_RESULT')
    return result.structuredContent as any
  }
  async providers():Promise<Provider[]>{return (await this.call('capabilities_list')).providers}
  async bind(provider:Provider,contract:Contract) {
    const resource=await this.call('resources_choose',{title:'选择 '+contract.title,mediaType:contract.mediaType})
    return this.call('capabilities_bind',{provider:provider.id,contract:contract.id,resourceRef:resource.resourceRef,tools:contract.tools})
  }
  async invoke(binding:string,tool:string,args:Record<string,unknown>={}) {
    const result=await this.call('capabilities_call',{binding,tool,arguments:args})
    if(result.isError)throw Error(JSON.stringify(result.content))
    return result.structuredContent
  }
  async read(uri:string) {return this.app.readServerResource({uri})}
}
