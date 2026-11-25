import test from 'node:test';
import {request as httpRequest} from 'node:http';
import assert from 'node:assert/strict';
import {readFileSync, mkdtempSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {generateKeyPairSync, createPublicKey, verify} from 'node:crypto';
import {ControlStore} from '../src/store.mjs';
import {createControlServer} from '../src/server.mjs';
import {digest, registry, validateDraft} from '../src/contracts.mjs';
import {parseMessage,parseInterpolation} from '../src/messages.mjs';

function fixture(database = ':memory:') {
  const {privateKey} = generateKeyPairSync('ed25519'), publicOrigin = 'https://config.example.test';
  const bundle = JSON.parse(readFileSync(new URL('../../../packages/openmuse_runtime_config/assets/deployments/default.json', import.meta.url)));
  bundle.bootstrap.discoveryUrls = [`${publicOrigin}/v1/runtime/scopes/test/active.jws`];
  bundle.bootstrap.publicKeys = {test: createPublicKey(privateKey).export({format:'jwk'}).x};
  bundle.bootstrap.allowedResourceOrigins = [publicOrigin];
  const store = new ControlStore({database, privateKey, kid:'test', publicOrigin});
  store.createScope('test',bundle,'admin');
  const body = {snapshot:structuredClone(bundle.snapshot),packs:[],note:'Enable the template language'};
  body.snapshot.localization.defaultLocale = registry.metadata.templateLocale;
  return {store,body,bundle,privateKey,publicOrigin};
}
function published(store, body) {
  let draft = store.createDraft('test',body,'editor'); draft = store.submit(draft.id,draft.version,'editor');
  store.approve(draft.id,draft.hash,'reviewer');
  return store.publish(draft.id,draft.hash,store.scope('test').revision,'publisher');
}
test('frozen reviewed content produces a verifiable immutable release and matching resources', () => {
  const {store, body, privateKey} = fixture();
  body.packs = [{schemaVersion:1,sourceRevision:registry.metadata.sourceRevision,locale:registry.metadata.templateLocale,namespace:'common',messages:{'common.save':'Save safely'}}];
  const draft = store.createDraft('test',body,'editor');
  assert.throws(()=>store.updateDraft(draft.id,0,body,'editor'),/changed/);
  assert.throws(()=>store.approve(draft.id,draft.hash,'reviewer'),/submitted/);
  const submitted = store.submit(draft.id,draft.version,'editor');
  assert.throws(()=>store.updateDraft(draft.id,submitted.version,body,'editor'),/frozen/);
  assert.throws(()=>store.approve(draft.id,draft.hash,'editor'),/own change/);
  assert.throws(()=>store.approve(draft.id,'wrong','reviewer'),/content/);
  assert.throws(()=>store.publish(draft.id,draft.hash,0,'publisher'),/approval/);
  store.approve(draft.id,draft.hash,'reviewer');
  const release = store.publish(draft.id,draft.hash,0,'publisher');
  const [h,p,s] = release.jws.split('.');
  assert.equal(JSON.parse(Buffer.from(h,'base64url')).typ,'openmuse.runtime-config+json');
  assert(verify(null,Buffer.from(h+'.'+p),createPublicKey(privateKey),Buffer.from(s,'base64url')));
  assert.deepEqual(JSON.parse(Buffer.from(p,'base64url')),release.snapshot);
  assert.equal(release.snapshot.sequence,1);
  const resource = release.snapshot.resources[0], bytes = store.resource(resource.sha256);
  assert.equal(digest(bytes),resource.sha256); assert.equal(bytes.length,resource.maxBytes);
  assert.deepEqual(JSON.parse(bytes),body.packs[0]);
  assert.equal(store.publish(draft.id,draft.hash,0,'publisher').jws,release.jws);
  assert(store.audits('test').some(a=>a.action==='release.publish'));
  store.close();
});
test('rollout is deterministic and rollback content is republished at a higher sequence', () => {
  const {store,body} = fixture();
  const first = published(store,body);
  const nextBody = structuredClone(body); nextBody.snapshot.support.email = 'new@example.test';
  const next = published(store,nextBody);
  assert.equal(store.active('test').id,first.id);
  assert.equal(store.active('test',0).id,next.id); assert.equal(store.active('test',1).id,first.id);
  assert.throws(()=>store.rollout('test',25,0,'publisher'),/pointer/);
  store.rollout('test',25,store.scope('test').revision,'publisher');
  assert.equal(store.active('test',24).id,next.id); assert.equal(store.active('test',25).id,first.id);
  store.rollout('test',0,store.scope('test').revision,'publisher'); assert.equal(store.active('test',0).id,first.id);
  const rollback = store.rollback('test',first.id,'editor'); store.submit(rollback.id,rollback.version,'editor'); store.approve(rollback.id,rollback.hash,'reviewer');
  const rolled = store.publish(rollback.id,rollback.hash,store.scope('test').revision,'publisher');
  assert.equal(rolled.sequence,3); assert.equal(rolled.snapshot.support.email,first.snapshot.support.email);
  store.rollout('test',100,store.scope('test').revision,'publisher'); assert.equal(store.active('test').sequence,3); store.close();
});
test('untrusted endpoints, paths, identity and message parameters are rejected before saving', () => {
  const {store,body,bundle} = fixture();
  for (const mutate of [
    s=>s.deployment.services.auth.baseUrl='https://attacker.example.test',
    s=>s.deployment.authBindingId='different',
    s=>s.links['legal.terms'].pathByLocale[s.localization.defaultLocale]='/x/%252e%252e',
    s=>s.localization.defaultLocale='unsupported',
    s=>s.web.servicePaths.auth='//attacker.example.test',
    s=>s.extra='unsupported',
  ]) { const invalid=structuredClone(body); mutate(invalid.snapshot); assert.throws(()=>validateDraft(invalid,bundle)); }
  const bad=structuredClone(body); bad.packs=[{schemaVersion:1,sourceRevision:registry.metadata.sourceRevision,locale:registry.metadata.templateLocale,namespace:'common',messages:{'common.save':'{unknown}'}}];
  assert.throws(()=>store.createDraft('test',bad,'editor'),/arguments/);
  assert.equal(store.listDrafts('test').length,0); store.close();
});
test('database reopen preserves release, high-water sequence and audit', () => {
  const dir=mkdtempSync(join(tmpdir(),'openmuse-control-'));
  try {
    const {store,body,privateKey,publicOrigin}=fixture(join(dir,'control.db')); const release=published(store,body); store.close();
    const reopened=new ControlStore({database:join(dir,'control.db'),privateKey,kid:'test',publicOrigin});
    assert.equal(reopened.active('test').jws,release.jws); assert.equal(reopened.scope('test').sequence,1); assert.equal(reopened.audits('test')[0].action,'release.publish'); reopened.close();
  } finally {rmSync(dir,{recursive:true,force:true});}
});
test('approval cannot overwrite an intervening release and new drafts preserve current content', () => {
  const {store,body}=fixture();
  let stale=store.createDraft('test',body,'editor');stale=store.submit(stale.id,stale.version,'editor');store.approve(stale.id,stale.hash,'reviewer');
  const current=structuredClone(body);current.snapshot.support.email='active@example.test';published(store,current);
  assert.throws(()=>store.publish(stale.id,stale.hash,store.scope('test').revision,'publisher'),/pointer/);
  assert.throws(()=>store.approve(stale.id,stale.hash,'another-reviewer'),/changed/);
  assert.equal(store.template('test').snapshot.support.email,'active@example.test');
  assert.equal(store.template('test').snapshot.sequence,0);store.close();
});
test('HTTP roles, origin/host protections, public CORS and frozen changes work together', async t => {
  const {store,body}=fixture();
  const server=createControlServer({store,localOrigin:'http://127.0.0.1:18879',demo:true});
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const base=`http://127.0.0.1:${server.address().port}`;
  t.after(()=>{server.close();store.close();});
  // Host is the configured external authority, even behind a forwarding proxy.
  const call=(path,actor,method='GET',payload,extra={})=>new Promise((resolve,reject)=>{
    const req=httpRequest(base+path,{method,headers:{host:'127.0.0.1:18879',...(actor?{'x-demo-actor':actor}:{}),...(payload?{'content-type':'application/json'}:{}),...extra}},res=>{
      const chunks=[];res.on('data',c=>chunks.push(c));res.on('end',()=>resolve(new Response(Buffer.concat(chunks),{status:res.statusCode,headers:res.headers})));
    });req.on('error',reject);req.end(payload?JSON.stringify(payload):undefined);
  });
  assert.equal((await call('/v1/admin/scopes')).status,401);
  assert.equal((await call('/v1/admin/scopes','demo-editor','GET',null,{origin:'https://evil.example.test'})).status,403);
  assert.equal((await fetch(base+'/healthz')).status,421);
  let response=await call('/v1/admin/scopes/test/drafts','demo-editor','POST',body); assert.equal(response.status,201); let draft=await response.json();
  response=await call(`/v1/admin/drafts/${draft.id}/submit`,'demo-editor','POST',{version:draft.version}); draft=await response.json();
  assert.equal((await call(`/v1/admin/drafts/${draft.id}/approve`,'demo-editor','POST',{hash:draft.hash})).status,403);
  assert.equal((await call(`/v1/admin/drafts/${draft.id}/approve`,'demo-reviewer','POST',{hash:draft.hash})).status,200);
  assert.equal((await call(`/v1/admin/drafts/${draft.id}/publish`,'demo-publisher','POST',{hash:draft.hash,revision:0})).status,200);
  response=await call('/v1/runtime/scopes/test/active.jws',null,'GET',null,{origin:'https://client.example.test'});
  assert.equal(response.status,200); assert.equal(response.headers.get('access-control-allow-origin'),'*'); assert.equal((await response.text()).split('.').length,3);
  assert.equal((await call('/v1/runtime/scopes/test/active/100.jws')).status,400);
  assert.equal((await call('/v1/runtime/scopes/test/active.jws',null,'GET',null,{host:'config.example.test'})).status,200);
  assert.equal((await call('/v1/admin/scopes','demo-editor','GET',null,{host:'config.example.test'})).status,421);
  const index=await call('/'); assert.match(index.headers.get('content-security-policy'),/frame-ancestors 'none'/);
});
test('production credentials have explicit scope access and ignore demo identities', async t => {
  const {store}=fixture();
  const principals=[{id:'scoped-reader',token:'a'.repeat(40),roles:['reader'],scopes:['other']}, {id:'test-reader',token:'b'.repeat(40),roles:['reader'],scopes:['test']}];
  const server=createControlServer({store,localOrigin:'http://127.0.0.1:18879',principals});
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve)); t.after(()=>{server.close();store.close();});
  const call=(path,token)=>new Promise((resolve,reject)=>{
    const req=httpRequest(`http://127.0.0.1:${server.address().port}${path}`,{headers:{host:'127.0.0.1:18879',...(token?{authorization:'Bearer '+token}:{}),'x-demo-actor':'demo-admin'}},res=>{
      const bytes=[];res.on('data',c=>bytes.push(c));res.on('end',()=>resolve({status:res.statusCode,json:JSON.parse(Buffer.concat(bytes))}));
    });req.on('error',reject);req.end();
  });
  assert.equal((await call('/v1/admin/scopes')).status,401);
  assert.deepEqual((await call('/v1/admin/scopes',principals[0].token)).json,[]);
  assert.equal((await call('/v1/admin/scopes/test',principals[0].token)).status,403);
  assert.equal((await call('/v1/admin/scopes/test',principals[1].token)).status,200);
});
test('every registered translation parses with unchanged argument contracts', () => {
  for (const [id, definition] of Object.entries(registry.entries)) for (const source of Object.values(definition.messages)) {
    const {parameters}=(definition.format==='interpolation.v1'?parseInterpolation:parseMessage)(source); assert(parameters.every(p=>definition.parameters.includes(p)),id);
  }
  assert.throws(()=>parseMessage('{count, plural, one{one}}'),/other/);
  assert.throws(()=>parseMessage('{value, date}'),/Unsupported/);
});

test('runtime switches and parameter bounds are reviewed signed data, not a build-time change', () => {
  const {store,body}=fixture();
  body.snapshot.compatibility.requiredCapabilities.push('settings.v1');
  body.snapshot.settings={features:{'plugins.market.enabled':false},parameters:{'runtime.refreshIntervalSeconds':60,'plugins.example.limit':17}};
  const malformed=structuredClone(body);malformed.snapshot.settings.parameters['runtime.refreshIntervalSeconds']=0;
  assert.throws(()=>store.createDraft('test',malformed,'editor'),/60/);
  const invalidFlag=structuredClone(body);invalidFlag.snapshot.settings.features['plugins.market.enabled']='false';
  assert.throws(()=>store.createDraft('test',invalidFlag,'editor'),/boolean/);
  const release=published(store,body);
  assert.equal(release.snapshot.settings.features['plugins.market.enabled'],false);
  assert.equal(release.snapshot.settings.parameters['runtime.refreshIntervalSeconds'],60);
  store.close();
});
