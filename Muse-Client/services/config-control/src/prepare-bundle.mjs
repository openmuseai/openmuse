import {readFileSync, writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {parseArgs} from 'node:util';
import {https, validateBundle} from './contracts.mjs';

const {values}=parseArgs({options:Object.fromEntries(['input','output','scope','origin','public-keys'].map(key=>[key,{type:'string'}]))});
if (Object.keys(values).length !== 5 || Object.values(values).some(v=>!v) || !/^[a-z0-9][a-z0-9-]{0,63}$/.test(values.scope ?? '')) throw Error('Use --input deployment.json --output installed.json --scope scope-id --origin HTTPS-origin --public-keys public.json');
if (resolve(values.input)===resolve(values.output) || resolve(values['public-keys'])===resolve(values.output)) throw Error('Output must differ from source files');
const bundle=JSON.parse(readFileSync(values.input,'utf8')), keys=JSON.parse(readFileSync(values['public-keys'],'utf8'));
const origin=https(values.origin,true).origin;
bundle.bootstrap.publicKeys={...bundle.bootstrap.publicKeys,...keys};
bundle.bootstrap.discoveryUrls=[`${origin}/v1/runtime/scopes/${values.scope}/active.jws`];
bundle.bootstrap.cohortRouting=true;
bundle.bootstrap.allowedResourceOrigins=[...new Set([...bundle.bootstrap.allowedResourceOrigins,origin])];
validateBundle(bundle);
writeFileSync(values.output,JSON.stringify(bundle,null,2)+'\n',{flag:'wx'});
console.log(`Validated public installation bundle saved to ${resolve(values.output)}`);
