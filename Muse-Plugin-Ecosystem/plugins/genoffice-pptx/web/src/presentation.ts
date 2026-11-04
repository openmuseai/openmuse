import configuration from '../../config/globalization.json'
import catalog from '../../i18n/catalog.json'

type Parameters = Record<string,string|number>
type Pack = {schemaVersion:number,sourceRevision:number,contractHash:string,locale:string,namespace:string,messages:Record<string,string>}
type LocaleDefinition = typeof configuration.locales[number]
const object=(v:unknown):v is Record<string,any>=>v!==null&&typeof v==='object'&&!Array.isArray(v)
const parameters=(s:string)=>[...new Set([...s.matchAll(/\{(\w+)\}/g)].map(m=>m[1]))].sort().join(',')
const entries:Record<string,{messages:Record<string,string>}> = catalog.entries
export const pluginConfiguration=configuration
export function matchLocale(raw:unknown,locales:LocaleDefinition[]=configuration.locales) {
  if(typeof raw!=='string')return undefined
  let tag:string
  try {tag=Intl.getCanonicalLocales(raw.replaceAll('_','-'))[0]}catch{return undefined}
  const direct=locales.find(v=>v.tag===tag||v.aliases.includes(tag))
  if(direct)return direct
  const parent=tag.replace(/-(?:[A-Z]{2}|[0-9]{3})$/,'')
  return locales.find(v=>v.tag===parent)
}
export class PresentationSettings {
  private listeners=new Set<()=>void>()
  private overlays:Record<string,Record<string,string>>={}
  private _locale=configuration.defaultLocale
  private _theme:'light'|'dark'='light'
  private _features={...configuration.features}
  private _parameters={...configuration.parameters}
  private _revision=0
  private _locales=configuration.locales
  get locale(){return this._locale}
  get upstream(){return matchLocale(this.locale,this._locales)!.upstream}
  get direction(){return matchLocale(this.locale,this._locales)!.direction as 'ltr'|'rtl'}
  get formatLocale(){return matchLocale(this.locale,this._locales)!.formatLocale}
  get theme(){return this._theme}
  get features(){return Object.freeze({...this._features})}
  get limits(){return Object.freeze({...this._parameters})}
  get revision(){return this._revision}
  subscribe=(listener:()=>void)=>{this.listeners.add(listener);return()=>this.listeners.delete(listener)}
  // Only the MCP SDK's authenticated peer delivers this context. This adapter
  // never fetches URLs, consumes account credentials or changes a service region.
  apply(context:unknown):void {
    if(!object(context))throw Error('INVALID_HOST_CONTEXT')
    const extension=context[configuration.hostExtension]
    let overlays=this.overlays,features=this._features,limits=this._parameters,locales=this._locales
    if(extension!==undefined){
      if(!object(extension)||extension.schemaVersion!==1)throw Error('INVALID_HOST_EXTENSION')
      const directory=extension.localeCatalog??[]
      if(!Array.isArray(directory)||directory.length>64)throw Error('INVALID_LOCALE_DIRECTORY')
      locales=[...configuration.locales]
      for(const value of directory){
        if(!object(value)||typeof value.tag!=='string'||Intl.getCanonicalLocales(value.tag)[0]!==value.tag||typeof value.formatLocale!=='string'||Intl.getCanonicalLocales(value.formatLocale)[0]!==value.formatLocale||value.tag.split('-')[0]!==value.formatLocale.split('-')[0]||!['ltr','rtl'].includes(value.direction))throw Error('INVALID_LOCALE_DEFINITION')
        // Built-in aliases and script boundaries remain authoritative. A newly
        // signed Host locale can add translations without a plugin release.
        if(locales.some(v=>v.tag===value.tag))continue
        const upstream=matchLocale(value.tag)?.upstream??matchLocale(configuration.defaultLocale)!.upstream
        locales.push({tag:value.tag,upstream,aliases:[],direction:value.direction,formatLocale:value.formatLocale})
      }
      const settings=extension.settings??{}
      if(!object(settings)||!object(settings.features??{})||!object(settings.parameters??{}))throw Error('INVALID_PLUGIN_SETTINGS')
      features={...configuration.features};limits={...configuration.parameters}
      for(const key of Object.keys(features) as (keyof typeof features)[]){
        const value=settings.features?.[configuration.settingsPrefix+key+'.enabled']
        if(value!==undefined){if(typeof value!=='boolean')throw Error('INVALID_PLUGIN_SWITCH');features[key]=value}
      }
      for(const key of ['maxFileBytes','historyLimit'] as const){
        const value=settings.parameters?.[configuration.settingsPrefix+key]
        const [min,max]=configuration.limits[key]
        if(value!==undefined){if(!Number.isInteger(value)||value<min||value>max)throw Error('INVALID_PLUGIN_PARAMETER');limits[key]=value}
      }
      const name=settings.parameters?.[configuration.settingsPrefix+'defaultFileName']
      if(name!==undefined){if(typeof name!=='string'||name.length>180||!name.endsWith('.pptx')||/[\\/\x00-\x1f]/.test(name))throw Error('INVALID_DEFAULT_FILENAME');limits.defaultFileName=name}
      const packs=extension.languagePacks??[]
      if(!Array.isArray(packs)||packs.length>32)throw Error('INVALID_LANGUAGE_DIRECTORY')
      overlays={}
      for(const raw of packs){
        if(!object(raw)||raw.namespace!==configuration.namespace)continue
        const pack=raw as Pack
        if(pack.schemaVersion!==1||pack.sourceRevision!==configuration.sourceRevision||pack.contractHash!==catalog.metadata.contractHash||!locales.some(v=>v.tag===pack.locale)||!object(pack.messages)||Object.keys(pack.messages).length>4000||new TextEncoder().encode(JSON.stringify(pack)).length>configuration.limits.languagePackBytes)throw Error('INCOMPATIBLE_PLUGIN_PACK')
        if(overlays[pack.locale])throw Error('DUPLICATE_PLUGIN_LANGUAGE')
        const messages:Record<string,string>=Object.create(null)
        for(const [id,source] of Object.entries(pack.messages)){
          const definition=Object.hasOwn(entries,id)?entries[id]:undefined
          const template=definition?.messages[pack.locale]??definition?.messages[configuration.defaultLocale]
          if(!definition||typeof source!=='string'||source.length>16000||parameters(source)!==parameters(template))throw Error('INCOMPATIBLE_PLUGIN_MESSAGE')
          messages[id]=source
        }
        if(!Object.keys(messages).length)throw Error('EMPTY_PLUGIN_PACK')
        overlays[pack.locale]=messages
      }
    }
    const requested=matchLocale(context.locale,locales)?.tag??this._locale
    const locale=configuration.bundledLocales.includes(requested)||overlays[requested]?requested:configuration.defaultLocale
    const theme=context.theme===undefined?this._theme:context.theme
    if(theme!=='light'&&theme!=='dark')throw Error('INVALID_THEME')
    if(JSON.stringify([locale,theme,features,limits,overlays,locales])===JSON.stringify([this._locale,this._theme,this._features,this._parameters,this.overlays,this._locales]))return
    this._locale=locale;this._theme=theme;this._features=features;this._parameters=limits;this.overlays=overlays;this._locales=locales
    this._revision++;for(const listener of this.listeners)listener()
  }
  text(id:string,params?:Parameters):string {
    const definition=Object.hasOwn(entries,id)?entries[id]:undefined
    if(!definition)throw Error('UNKNOWN_PLUGIN_MESSAGE')
    const source=this.overlays[this.locale]?.[id]??definition.messages[this.locale]??definition.messages[configuration.defaultLocale]
    return source.replace(/\{(\w+)\}/g,(match,name)=>params&&Object.hasOwn(params,name)?String(params[name]):match)
  }
  ui(key:string,params?:Parameters){const id=`${configuration.namespace}.ui.${key}`;return Object.hasOwn(entries,id)?this.text(id,params):key}
  integration(key:string,params?:Parameters){return this.text(`${configuration.namespace}.integration.${key}`,params)}
}
export const presentation=new PresentationSettings()
