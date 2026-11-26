import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {runInNewContext} from 'node:vm';
import {fileURLToPath} from 'node:url';
const dir=new URL('../third_party/dsh/',import.meta.url);
const sha=b=>createHash('sha256').update(b).digest('hex');
function loadOwner(name, presentation = 'navigation') {
  const receipt=JSON.parse(readFileSync(new URL('patches/layout-embedded-v2.json',dir))).find(r=>r.upstream.includes('ui-'+name+'@'));
  const archive=new URL('tarballs/'+receipt.file,dir);
  assert.equal(sha(readFileSync(archive)),receipt.sha256);
  let source=execFileSync('tar',['-xOf',fileURLToPath(archive),'package/lib/client.js'],{encoding:'utf8'});
  assert.equal(sha(source),receipt.patchedClientSha256);
  assert.equal(sha(readFileSync(new URL('node_modules/@deepseek-ai/dsh-client-ui-'+name+'/lib/client.js',dir))),receipt.patchedClientSha256);
  source=source.replace('exports.apply = apply;',`exports.testComponent = ${name==='sidebar'?'SidebarRoot':name==='layout'?'AppFrame':'SettingsRoot'};exports.apply = apply;`);
  const jsx=(type,props,key)=>({type,props,key});
  const react={useRef:current=>({current}),useState:v=>[typeof v==='function'?v():v,()=>{}],useEffect(){},useLayoutEffect(){},useMemo:fn=>fn(),useCallback:fn=>fn};
  const primitives=new Proxy({isDarwinDesktop:()=>false},{get:(o,k)=>o[k]??(()=>null)});
  let owner;
  const document={querySelector:()=>null,createElement:()=>({dataset:{}}),head:{appendChild(){}},documentElement:{dataset:{},hasAttribute:()=>false}};
  const window={__OpenMuseHostNavigation:1,__OpenMuseDshPresentation:presentation,__ModuleLoader__:{load:v=>{owner=v.factory(name=>name==='react'?react:name==='react/jsx-runtime'?{jsx,jsxs:jsx,Fragment:'fragment'}:name.includes('primitives')?primitives:{});}}};
  runInNewContext(source,{window,document,AbortController});
  return owner.testComponent;
}
const flatten=node=>!node||typeof node!=='object'?[]:[node,...[node.props?.children].flat(Infinity).flatMap(flatten)];
test('original DSH sidebar keeps workspace browser and all contribution slots at Host width',()=>{
  const Sidebar=loadOwner('sidebar'),calls=[];
  const root=Sidebar({collapsed:false,width:232,startSession(){},toggleSidebar(){throw Error('inner toggle is forbidden');},selectPanel(){},usePanels:f=>f([{id:'plugins',label:'Plugins'}]),useShortcuts:f=>f([]),usePanelInfo:f=>f({activePanelId:null}),t:v=>v,renderSlot:(name,props)=>{calls.push({name,props});return {type:'slot',props:{name}};}});
  assert.equal(root.props.style.width,232);
  assert.equal(calls.find(c=>c.name==='sidebar.workspaces').props.wide,true);
  for(const slot of ['sidebar.brand.mark','sidebar.brand.name','sidebar.workspaces','sidebar.footer.action','sidebar.settings'])assert.ok(calls.some(c=>c.name===slot),slot);
  assert.equal(flatten(root).some(n=>n.props?.['aria-label']==='toggle.collapse'),false);
});
test('DSH toolbar retains settings, brand, new-session and plugin slots without workspace or toggle',()=>{
  const Sidebar=loadOwner('sidebar'),calls=[];let created=0;
  const root=Sidebar({toolbar:true,collapsed:true,width:400,startSession(){created++;},toggleSidebar(){throw Error('inner toggle is forbidden');},selectPanel(){},usePanels:f=>f([{id:'plugins',label:'Plugins'}]),useShortcuts:f=>f([]),usePanelInfo:f=>f({activePanelId:null}),t:v=>v,renderSlot:(name,props)=>{calls.push({name,props});return {type:'slot',props:{name}};}});
  assert.equal(root.props['data-openmuse-dsh-toolbar'],true);
  for(const slot of ['sidebar.settings','sidebar.brand.mark','sidebar.footer.action'])assert.ok(calls.some(c=>c.name===slot),slot);
  assert.equal(calls.some(c=>c.name==='sidebar.workspaces'),false);
  const nodes=flatten(root);assert.ok(nodes.some(n=>n.props?.id==='plugins'));
  const button=nodes.find(n=>n.type==='button'&&n.props['aria-label']==='session.new.label');button.props.onClick();assert.equal(created,1);
});

test('navigation seat renders the original expanded sidebar without mounting a Conversation',()=>{
 const Frame=loadOwner('layout'),calls=[];
 const root=Frame({useStore:f=>f({layoutInfo:{viewportWidth:232,sidebar:280,narrowExpanded:false,rightbar:0,rightbarShown:false,rightbarTrack:false,rightbarFullscreen:false,rightbarInstant:false}}),useSessions(){},usePanelInfo(){},actions:{},renderSlot:(name,props)=>{calls.push({name,props});return {type:'slot',props:{name}};},t:v=>v});
 assert.equal(root.props['data-openmuse-dsh-navigation'],true);
 const sidebar=calls.find(c=>c.name==='sidebar');assert.equal(sidebar.props.collapsed,false);assert.equal(sidebar.props.width,232);
 assert.equal(flatten(root).some(n=>typeof n.type==='function'&&n.type.name==='MainPanel'),false);
});
test('only the Conversation seat owns automatic onboarding; no API key or completion state is fabricated',()=>{
 for(const presentation of ['navigation','conversation']) {
  const Settings=loadOwner('settings-general',presentation),calls=[];
  Settings({wide:true,reconnect(){},useConnectionState:f=>f('connected'),useSections:f=>f([]),useOnboardingSteps:f=>f([{id:'preview'},{id:'api-key'}]),useSessions:f=>f({phase:'ready',byId:{}}),renderSlot:(name,props)=>{calls.push({name,props});return null;},t:v=>v,useDesktopUpdate:f=>f({}),openDesktopUpdate(){},useStore:f=>f({open:false,activeId:'general'}),actions:{close(){},openSection(){}},useShortcuts:f=>f([])});
  assert.equal(calls.some(c=>c.name==='settings.onboarding'),presentation==='conversation');
 }
});
