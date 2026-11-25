import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {generateKeyPairSync,createPublicKey,verify} from 'node:crypto';
import {ControlStore} from '../src/store.mjs';

test('regional media is reviewed and signed without changing the App deployment boundary',()=>{
 const bundle=JSON.parse(readFileSync(new URL('../../../packages/openmuse_runtime_config/assets/deployments/default.json',import.meta.url)));
 bundle.snapshot.scope.platform='website';bundle.snapshot.compatibility.requiredCapabilities=['config.v1','portal.v1'];
 const s=bundle.snapshot;s.origins.us='https://us.example.test';s.origins.cn='https://cn.example.test';
 const link=(id,originRef,path)=>s.links[id]={originRef,pathByLocale:{en:path,'zh-Hans':path},fallbackLocale:'en'};
 s.portal={schemaVersion:1,defaultRegion:'us',regions:{}};
 for(const id of ['cn','us']){
  link(id+'.portal',id,'/');link(id+'.app',id,'/app/');link(id+'.image',id,'/media/versioned/poster-v1.jpg');
  s.portal.regions[id]={dataRegion:id,labels:{en:id,'zh-Hans':id},portalLinkId:id+'.portal',appLinkId:id+'.app',downloads:[],media:{hero:{kind:'image',fallbackLocale:'en',byLocale:{en:{linkId:id+'.image'}}}}};
 }
 const {privateKey}=generateKeyPairSync('ed25519');const origin='https://config.example.test';
 bundle.bootstrap.discoveryUrls=[origin+'/v1/runtime/scopes/portal/active.jws'];bundle.bootstrap.allowedResourceOrigins=[origin];bundle.bootstrap.publicKeys={test:createPublicKey(privateKey).export({format:'jwk'}).x};
 const store=new ControlStore({database:':memory:',privateKey,kid:'test',publicOrigin:origin});
 try{
  store.createScope('portal',bundle,'admin');
  const body={snapshot:structuredClone(s),packs:[],note:'Update only the US English poster'};
  body.snapshot.links['us.image'].pathByLocale.en='/media/versioned/poster-v2.jpg';
  let d=store.createDraft('portal',body,'editor');d=store.submit(d.id,d.version,'editor');store.approve(d.id,d.hash,'reviewer');
  const release=store.publish(d.id,d.hash,store.scope('portal').revision,'publisher',600);
  const parts=release.jws.split('.');assert(verify(null,Buffer.from(parts.slice(0,2).join('.')),createPublicKey(privateKey),Buffer.from(parts[2],'base64url')));
  const published=JSON.parse(Buffer.from(parts[1],'base64url'));assert.equal(published.links['us.image'].pathByLocale.en,'/media/versioned/poster-v2.jpg');assert.equal(published.links['cn.image'].pathByLocale.en,'/media/versioned/poster-v1.jpg');
  for(const mutate of [v=>v.origins.us='https://cn.example.test',v=>v.portal.regions.us.dataRegion='cn',v=>v.portal.defaultRegion='cn',v=>v.links['us.image'].pathByLocale.en='/payload.html']){
   const invalid=structuredClone(body);mutate(invalid.snapshot);assert.throws(()=>store.createDraft('portal',invalid,'editor'));
  }
 }finally{store.close();}
});
