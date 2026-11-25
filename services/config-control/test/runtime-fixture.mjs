// Cross-runtime contract fixture. Emits only public installation data and
// signed artifacts; the ephemeral private key never leaves this process.
import {readFileSync} from 'node:fs';
import {generateKeyPairSync, createPublicKey, sign} from 'node:crypto';
import {ControlStore} from '../src/store.mjs';
import {digest, registry} from '../src/contracts.mjs';
import {parseMessage} from '../src/messages.mjs';
const {privateKey}=generateKeyPairSync('ed25519');
const origin='https://config.example.test', now='2026-10-30T00:00:00.000Z';
const bundle=JSON.parse(readFileSync(new URL('../../../packages/openmuse_runtime_config/assets/deployments/default.json',import.meta.url)));
bundle.bootstrap.discoveryUrls=[`${origin}/v1/runtime/scopes/interop/active.jws`]; bundle.bootstrap.cohortRouting=true;
bundle.bootstrap.publicKeys={interop:createPublicKey(privateKey).export({format:'jwk'}).x}; bundle.bootstrap.allowedResourceOrigins=[origin];
const store=new ControlStore({database:':memory:',privateKey,kid:'interop',publicOrigin:origin,clock:()=>new Date(now)});
store.createScope('interop',bundle,'admin');
const pack={schemaVersion:1,sourceRevision:registry.metadata.sourceRevision,locale:registry.metadata.templateLocale,namespace:'common',messages:{'common.save':'Save globally'}};
const body={snapshot:structuredClone(bundle.snapshot),packs:[pack],note:'Interop fixture'};
body.snapshot.localization.defaultLocale=registry.metadata.templateLocale;
const frames=[];
function publish(ttl) {
  let draft=store.createDraft('interop',structuredClone(body),'editor'); draft=store.submit(draft.id,draft.version,'editor'); store.approve(draft.id,draft.hash,'reviewer');
  const release=store.publish(draft.id,draft.hash,store.scope('interop').revision,'publisher',ttl);
  frames.push({snapshot:release.snapshot,jws:release.jws,resources:Object.fromEntries(release.snapshot.resources.map(r=>[r.url,store.resource(r.sha256).toString('base64')]))});
}
publish(600);
body.snapshot.support.email='updated@example.test';body.packs[0].messages['common.save']='Save worldwide'; publish(600);
body.snapshot.deployment.services.auth.baseUrl+='/v2';publish(3600);
const bad=structuredClone(frames[2]);bad.snapshot.sequence=4;bad.snapshot.releaseId='invalid-message-contract';
const bytes=Buffer.from(JSON.stringify({...pack,messages:{'common.save':'{unexpected}'}})),hash=digest(bytes),resource=bad.snapshot.resources[0];
resource.sha256=hash;resource.url=`${origin}/v1/runtime/resources/${hash}.json`;resource.maxBytes=bytes.length;
const header=Buffer.from(JSON.stringify({alg:'EdDSA',typ:'openmuse.runtime-config+json',kid:'interop'})).toString('base64url'),payload=Buffer.from(JSON.stringify(bad.snapshot)).toString('base64url'),input=header+'.'+payload;
bad.jws=input+'.'+sign(null,Buffer.from(input),privateKey).toString('base64url');bad.resources={[resource.url]:bytes.toString('base64')};frames.push(bad);
const templates=Object.values(registry.entries).filter(definition=>definition.format!=='interpolation.v1').flatMap(definition=>Object.values(definition.messages).map(source=>({source,...parseMessage(source)})));
console.log(JSON.stringify({bundle,now,frames,templates}));store.close();
