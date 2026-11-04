import test from 'node:test'
import assert from 'node:assert/strict'
import catalog from '../i18n/catalog.json'
import {PresentationSettings,matchLocale,pluginConfiguration} from '../web/src/presentation.js'
const namespace=pluginConfiguration.namespace,prefix=pluginConfiguration.settingsPrefix
function pack(locale:string,messages:Record<string,string>){return {schemaVersion:1,sourceRevision:1,contractHash:catalog.metadata.contractHash,namespace,locale,messages}}
const context=(locale:string,languagePacks:any[]=[],features:Record<string,boolean>={},parameters:Record<string,unknown>={})=>({locale,theme:'light',openmuse:{schemaVersion:1,languagePacks,settings:{features,parameters}}})
test('host locales preserve script boundaries; only bundled languages or provided packs activate',()=>{
 const state=new PresentationSettings()
 state.apply({locale:'en-GB'});assert.equal(state.locale,'en')
 state.apply({locale:'zh-CN'});assert.equal(state.locale,'zh-Hans');assert.equal(state.ui('ribbonTabHome'),'开始')
 state.apply({locale:'zh-Hant'});assert.equal(state.locale,'en')
 assert.equal(matchLocale('zh-TW')?.tag,'zh-Hant')
 state.apply(context('fr',[pack('fr',{[namespace+'.ui.ribbonTabHome']:'Accueil'})]));assert.equal(state.locale,'fr');assert.equal(state.ui('ribbonTabHome'),'Accueil')
})
test('incompatible packs and out-of-range configuration never partially activate',()=>{
 const state=new PresentationSettings();state.apply(context('en'))
 const before=state.revision
 assert.throws(()=>state.apply(context('fr',[pack('fr',{[namespace+'.ui.unknown']:'bad'})],{[prefix+'editing.enabled']:false})),/INCOMPATIBLE/)
 assert.equal(state.revision,before);assert.equal(state.features.editing,true)
 assert.throws(()=>state.apply(context('en',[],{},{[prefix+'maxFileBytes']:67108865})),/PARAMETER/)
 assert.throws(()=>state.apply(context('en',[],{},{[prefix+'defaultFileName']:'../file.pptx'})),/FILENAME/)
 const incompatible={...pack('fr',{[namespace+'.integration.unsupportedOperation']:'Missing required argument'}),contractHash:'0'.repeat(64)}
 assert.throws(()=>state.apply(context('fr',[incompatible])),/PACK/)
 assert.throws(()=>state.apply(context('fr',[pack('fr',{[namespace+'.integration.unsupportedOperation']:'{newArgument}'})])),/MESSAGE/)
})
test('remote translation and switches notify subscribers while interpolation stays inert',()=>{
 const state=new PresentationSettings();let notices=0
 const release=state.subscribe(()=>notices++)
 state.apply(context('en',[pack('en',{[namespace+'.integration.agent']:'<script>inert</script>'})],{[prefix+'agent.enabled']:false},{[prefix+'historyLimit']:3}))
 assert.equal(state.integration('agent'),'<script>inert</script>');assert.equal(state.features.agent,false);assert.equal(state.limits.historyLimit,3);assert.equal(notices,1)
 state.apply(context('en',[],{},{}));assert.equal(state.features.agent,true);assert.equal(state.limits.historyLimit,30)
 release();assert.equal(notices,2)
})
test('a signed Host locale directory can introduce another language without rebuilding the plugin',()=>{
 const state=new PresentationSettings(),host=context('fi-FI',[pack('fi',{[namespace+'.ui.ribbonTabHome']:'Aloitus'})])
 Object.assign(host.openmuse,{localeCatalog:[{tag:'fi',direction:'ltr',formatLocale:'fi-FI'}]})
 state.apply(host);assert.equal(state.locale,'fi');assert.equal(state.ui('ribbonTabHome'),'Aloitus');assert.equal(state.formatLocale,'fi-FI')
 assert.equal(state.ui('futureUpstreamKey'),'futureUpstreamKey')
 Object.assign(host.openmuse,{localeCatalog:[{tag:'fi',direction:'invalid',formatLocale:'fi-FI'}]})
 assert.throws(()=>state.apply(host),/LOCALE/);assert.equal(state.ui('ribbonTabHome'),'Aloitus')
})
