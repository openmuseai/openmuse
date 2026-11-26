// Reproducible, digest-checked adaptation of the layout owner, not DOM CSS.
import{readFile,writeFile,mkdir,mkdtemp,cp}from'node:fs/promises';import{tmpdir}from'node:os';import{join,resolve}from'node:path';import{createHash}from'node:crypto';import{execFileSync}from'node:child_process';
const root=resolve(import.meta.dirname,'..'),dir=join(root,'third_party/dsh');
const upstream=join(dir,'tarballs/deepseek-ai-dsh-client-ui-layout-0.2.0-rc.2.tgz');
const expectedUpstreamSha256='403c3ee38c258c121752ab0ba6f7ab980958517d7af982967864a6963037fdfb';
if(createHash('sha256').update(await readFile(upstream)).digest('hex')!==expectedUpstreamSha256)throw Error('Upstream archive digest mismatch');
const temp=await mkdtemp(join(tmpdir(),'muse-layout-')),pkg=join(temp,'package');
execFileSync('tar',['-xzf',upstream,'-C',temp]);
const file=join(pkg,'lib/client.js'),original=await readFile(file,'utf8');
let code=original;const patches=[
 ['const sidebarCollapsed = narrow ? !layoutInfo.narrowExpanded : layoutInfo.sidebar === 0;', 'const hostNavigation = window.__OpenMuseHostNavigation === 1;\n\t\t\tconst sidebarCollapsed = hostNavigation || (narrow ? !layoutInfo.narrowExpanded : layoutInfo.sidebar === 0);'],
 ['const collapsedWidth = darwin || document.documentElement.hasAttribute("data-windows-titlebar") ? 0 : 56;','const collapsedWidth = hostNavigation || darwin || document.documentElement.hasAttribute("data-windows-titlebar") ? 0 : 56;'],
 ['() => renderSlot("sidebar", {','() => hostNavigation ? null : renderSlot("sidebar", {'],
 ['const leadingMounted = darwin && sidebarCollapsed;','const leadingMounted = !hostNavigation && darwin && sidebarCollapsed;'],
];
for(const[a,b]of patches){if(code.split(a).length!==2)throw Error('Pinned layout source changed: '+a);code=code.replace(a,b);}
const sha=b=>createHash('sha256').update(b).digest('hex');
const manifest=JSON.parse(await readFile(join(pkg,'package.json'),'utf8'));if(manifest.version!=='0.2.0-rc.2')throw Error('Unexpected upstream version');manifest.version='0.2.0-rc.2-openmuse.1';await writeFile(join(pkg,'package.json'),JSON.stringify(manifest,null,2)+'\n');await writeFile(file,code);
await mkdir(join(dir,'patches'),{recursive:true});await writeFile(join(dir,'patches/layout-embedded-v1.json'),JSON.stringify({schema:'openmuse.vendor-patch/1',upstream:'@deepseek-ai/dsh-client-ui-layout@0.2.0-rc.2',repository:manifest.repository,license:manifest.license,upstreamArchiveSha256:expectedUpstreamSha256,sourceClientSha256:sha(original),patchedClientSha256:sha(code),patches,flag:'window.__OpenMuseHostNavigation === 1',default:'Unchanged standalone DSH',architecture:'A-01 / ADR-04'},null,2)+'\n');
const result=JSON.parse(execFileSync('npm',['pack','--json','--ignore-scripts','--pack-destination',join(dir,'tarballs')],{cwd:pkg,encoding:'utf8'}));console.log(JSON.stringify({file:result[0].filename,sha256:sha(await readFile(join(dir,'tarballs',result[0].filename)))}));
