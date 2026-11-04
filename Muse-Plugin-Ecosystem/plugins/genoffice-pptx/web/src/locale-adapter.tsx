import {useSyncExternalStore,type ReactNode} from 'react'
import {platformShortcuts,type Lang,type Params} from '../../vendor/genoffice/packages/i18n/src/index'
import {presentation} from './presentation'
export type StringKey = string
export type TFunc = (key:StringKey,params?:Params)=>string
export const getLang=()=>presentation.upstream as Lang
export const setModuleLang=(_lang:Lang)=>{syncDocument()}
export const t:TFunc=(key,params)=>platformShortcuts(presentation.ui(key,params)).replaceAll('GenOffice', 'OpenMuse').replaceAll('Genspark', 'OpenMuse')
export const aiLangDirective=()=>'' // Host Agent owns its locale-aware instructions.
export const DATE_LOCALES = new Proxy({} as Record<Lang,string>,{get:()=>presentation.formatLocale})
function syncDocument(){document.documentElement.lang=presentation.locale;document.documentElement.dir=presentation.direction;document.documentElement.dataset.pptxLocale=presentation.locale}
presentation.subscribe(syncDocument)
syncDocument()
export function LocaleProvider({children}:{initial:Lang,children:ReactNode}){
  useSyncExternalStore(presentation.subscribe,()=>presentation.revision)
  syncDocument()
  return children
}
export function useI18n(){useSyncExternalStore(presentation.subscribe,()=>presentation.revision);return {lang:getLang(),t,dateLocale:presentation.formatLocale}}
