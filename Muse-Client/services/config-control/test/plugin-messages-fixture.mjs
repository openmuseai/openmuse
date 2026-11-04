// A generic plugin contract: the Host must never import a plugin implementation.
import {mkdtempSync,writeFileSync,rmSync,readFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {generateKeyPairSync,createPublicKey} from 'node:crypto';
import assert from 'node:assert/strict';
const directory=mkdtempSync(join(tmpdir(),'openmuse-plugin-contract-'));
try {
 const namespace='plugin.fixture',contractHash='a'.repeat(64);
 const catalog={metadata:{namespace,sourceRevision:3,contractHash},entries:{'plugin.fixture.title':{namespace,format:'interpolation.v1',parameters:['name'],messages:{en:'Document {name}', 'zh-Hans':'文稿 {name}'}}}};
 const file=join(directory,'catalog.json');writeFileSync(file,JSON.stringify(catalog));
 process.env.OPENMUSE_CONFIG_PLUGIN_REGISTRY_FILES=JSON.stringify([file]);
 const {ControlStore}=await import('../src/store.mjs');
 const bundle=JSON.parse(readFileSync(new URL('../../../packages/openmuse_runtime_config/assets/deployments/global-us.json',import.meta.url)));
 const {privateKey}=generateKeyPairSync('ed25519'),origin='https://config.example.test',now='2026-10-30T00:00:00Z';
 bundle.bootstrap.discoveryUrls=[origin+'/v1/runtime/scopes/plugin/active.jws'];
 bundle.bootstrap.publicKeys={test:createPublicKey(privateKey).export({format:'jwk'}).x};bundle.bootstrap.allowedResourceOrigins=[origin];
 const store=new ControlStore({database:':memory:',privateKey,kid:'test',publicOrigin:origin,clock:()=>new Date(now)});
 try {
  store.createScope('plugin',bundle,'admin');const snapshot=structuredClone(bundle.snapshot);
  snapshot.compatibility.requiredCapabilities.push('plugin-messages.v1','settings.v1');
  snapshot.settings={features:{'io.example.plugin.editing.enabled':false},parameters:{'io.example.plugin.limit':7}};
  const pack={schemaVersion:1,sourceRevision:3,contractHash,locale:'en',namespace,messages:{'plugin.fixture.title':'Updated {name}: {"safe":true}'}};
  const body={snapshot,note:'Plugin language contract',packs:[pack]};
  const bad=structuredClone(body);bad.packs[0].messages['plugin.fixture.title']='Lost argument';
  assert.throws(()=>store.createDraft('plugin',bad,'editor'),/arguments/);
  bad.packs[0]={...pack,contractHash:'0'.repeat(64)};assert.throws(()=>store.createDraft('plugin',bad,'editor'),/contract/);
  let draft=store.createDraft('plugin',body,'editor');draft=store.submit(draft.id,draft.version,'editor');store.approve(draft.id,draft.hash,'reviewer');
  const release=store.publish(draft.id,draft.hash,store.scope('plugin').revision,'publisher',600);
  const resource=release.snapshot.resources[0];assert.equal(resource.kind,'pluginMessages');assert.equal(resource.delivery,'onDemand');assert.deepEqual(release.snapshot.localization.bundleRefs,[]);
  console.log(JSON.stringify({bundle,now,pack,jws:release.jws,snapshot:release.snapshot,resources:Object.fromEntries(release.snapshot.resources.map(r=>[r.url,store.resource(r.sha256).toString('base64')]))}));
 } finally {store.close()}
} finally {rmSync(directory,{recursive:true,force:true})}
