// Digest-checked source adaptation at the two DSH layout owners. Standalone is unchanged.
import {readFile,writeFile,mkdir,mkdtemp} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
const dir=resolve(import.meta.dirname,'../third_party/dsh');
const sha=bytes=>createHash('sha256').update(bytes).digest('hex');
const toolbar=`if (toolbar) return (0, react_jsx_runtime.jsxs)("div", {
  "data-openmuse-dsh-toolbar": true,
  style: {display:"flex",alignItems:"center",gap:8,height:44,minHeight:44,padding:"4px 38px 4px 10px",boxSizing:"border-box",borderBottom:".5px solid var(--dsw-alias-border-l3)",overflowX:"auto"},
  children: [
    (0, react_jsx_runtime.jsx)("div", {style:{display:"flex",flexShrink:0},children:renderSlot("sidebar.settings",{wide:false})}),
    renderSlot("sidebar.brand.mark",{size:24},{fallback:(0,react_jsx_runtime.jsx)(_deepseek_ai_dsh_client_ui_primitives.FishLogo,{size:24})}),
    (0,react_jsx_runtime.jsx)(_deepseek_ai_dsh_client_ui_primitives.Tooltip,{label:t("session.new.label"),children:(0,react_jsx_runtime.jsx)("button",{type:"button",className:SidebarRoot_module_css_default.iconButton,"aria-label":t("session.new.label"),onClick:()=>startSession(),children:(0,react_jsx_runtime.jsx)(_deepseek_ai_dsh_client_ui_primitives.IconNewChatOutlineRegular,{size:18})})}),
    ...panels.map(({id,label})=>(0,react_jsx_runtime.jsx)(PanelRow,{id,label,wide:false,usePanelInfo,selectPanel,renderSlot},id)),
    (0,react_jsx_runtime.jsx)("div",{style:{display:"flex",flexShrink:0},children:renderSlot("sidebar.footer.action",{wide:false})})
  ]
});
`;
const specs=[{
 name:'layout',sha256:'403c3ee38c258c121752ab0ba6f7ab980958517d7af982967864a6963037fdfb',version:'0.2.0-rc.2-openmuse.3',patches:[
 ['const sidebarCollapsed = narrow ? !layoutInfo.narrowExpanded : layoutInfo.sidebar === 0;','const hostNavigation = window.__OpenMuseHostNavigation === 1;\nconst hostSidebar = hostNavigation && window.__OpenMuseDshPresentation === "navigation";\nconst sidebarCollapsed = hostNavigation || (narrow ? !layoutInfo.narrowExpanded : layoutInfo.sidebar === 0);'],
 ['const collapsedWidth = darwin || document.documentElement.hasAttribute("data-windows-titlebar") ? 0 : 56;','const collapsedWidth = hostNavigation || darwin || document.documentElement.hasAttribute("data-windows-titlebar") ? 0 : 56;'],
 ['() => renderSlot("sidebar", {\n\t\t\t\tcollapsed: sidebarCollapsed,\n\t\t\t\twidth: cols.sidebar','() => hostNavigation && !hostSidebar ? null : renderSlot("sidebar", {\ncollapsed: hostSidebar ? false : sidebarCollapsed,\nwidth: hostSidebar ? viewport : cols.sidebar'],
 ['\t\t\t\tcols.sidebar\n\t\t\t]);','\t\t\t\tcols.sidebar, viewport\n\t\t\t]);'],
 ['const leadingMounted = darwin && sidebarCollapsed;','const toolbar = (0, react.useMemo)(() => hostNavigation && !hostSidebar ? renderSlot("sidebar", {collapsed:true,width:viewport,toolbar:true}) : null, [renderSlot,viewport]);\nconst leadingMounted = !hostNavigation && darwin && sidebarCollapsed;\nif (hostSidebar) return (0,react_jsx_runtime.jsxs)("div",{ref:frameRef,style:{height:"100%",width:"100%",position:"relative",overflow:"hidden"},"data-openmuse-dsh-navigation":true,children:[sidebar,overlays]});'],
 ['(0, react_jsx_runtime.jsx)(CenterColumn, { children: main })','(0, react_jsx_runtime.jsx)(CenterColumn, { children: (0,react_jsx_runtime.jsxs)(react_jsx_runtime.Fragment,{children:[toolbar,main]}) })'],
 ]
},{name:'sidebar',sha256:'5476ce59f5bbcfee3bee5930ea5db2bc671b2b77bd0ffa51c94d48e5330e4e7a',version:'0.2.0-rc.2-openmuse.1',patches:[
 ['function SidebarRoot({ collapsed, width,','function SidebarRoot({ toolbar = false, collapsed, width,'],
 ['const toggle = (0, react_jsx_runtime.jsx)',''+toolbar+'const toggle = window.__OpenMuseHostNavigation === 1 ? null : (0, react_jsx_runtime.jsx)'],
 ['darwinDesktop && (0, react_jsx_runtime.jsx)("div", {','darwinDesktop && window.__OpenMuseHostNavigation !== 1 && (0, react_jsx_runtime.jsx)("div", {'],
 ]},{name:'settings-general',sha256:'77408b888afbc88226ebd9337ab209c434c784a89e5360ec5f22ea9ae2db439b',version:'0.2.0-rc.2-openmuse.1',patches:[
 ['const onboardingActive = useSessions((state) => {','const onboardingActive = useSessions((state) => {\nif (window.__OpenMuseHostNavigation === 1 && window.__OpenMuseDshPresentation === \"navigation\") return false;'],
 ]}];
const receipts=[];
for(const spec of specs){const upstream=join(dir,`tarballs/deepseek-ai-dsh-client-ui-${spec.name}-0.2.0-rc.2.tgz`);if(sha(await readFile(upstream))!==spec.sha256)throw Error('Upstream archive changed: '+spec.name);const temp=await mkdtemp(join(tmpdir(),'muse-dsh-'));execFileSync('tar',['-xzf',upstream,'-C',temp]);const pkg=join(temp,'package'),file=join(pkg,'lib/client.js'),original=await readFile(file,'utf8');let code=original;for(const[a,b]of spec.patches){if(code.split(a).length!==2)throw Error('Pinned source changed: '+spec.name+' / '+a);code=code.replace(a,b);}const manifest=JSON.parse(await readFile(join(pkg,'package.json'),'utf8'));if(manifest.version!=='0.2.0-rc.2')throw Error('Unexpected upstream version');manifest.version=spec.version;await writeFile(join(pkg,'package.json'),JSON.stringify(manifest,null,2)+'\n');await writeFile(file,code);const packed=JSON.parse(execFileSync('npm',['pack','--json','--ignore-scripts','--pack-destination',join(dir,'tarballs')],{cwd:pkg,encoding:'utf8'}))[0];receipts.push({schema:'openmuse.vendor-patch/2',upstream:manifest.name+'@0.2.0-rc.2',repository:manifest.repository,license:manifest.license,upstreamArchiveSha256:spec.sha256,sourceClientSha256:sha(original),patchedClientSha256:sha(code),patches:spec.patches,file:packed.filename,sha256:sha(await readFile(join(dir,'tarballs',packed.filename))),flags:['window.__OpenMuseHostNavigation === 1','window.__OpenMuseDshPresentation === "navigation"'],default:'Unchanged standalone DSH',architecture:'ADR-07 / AC-13–14'});}
await mkdir(join(dir,'patches'),{recursive:true});await writeFile(join(dir,'patches/layout-embedded-v2.json'),JSON.stringify(receipts,null,2)+'\n');console.log(JSON.stringify(receipts.map(({file,sha256})=>({file,sha256}))));
