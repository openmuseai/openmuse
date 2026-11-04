import {mkdirSync, writeFileSync, unlinkSync} from 'node:fs';
import {generateKeyPairSync} from 'node:crypto';
import {resolve, join} from 'node:path';
import {parseArgs} from 'node:util';

const {values}=parseArgs({options:{directory:{type:'string'},kid:{type:'string'}}});
if (!values.directory || !/^[A-Za-z0-9][A-Za-z0-9_.-]{0,63}$/.test(values.kid ?? '')) throw Error('Use --directory /private/key-directory --kid signing-key-id');
const directory=resolve(values.directory);mkdirSync(directory,{recursive:true,mode:0o700});
const pair=generateKeyPairSync('ed25519');
const privatePath=join(directory,values.kid+'.private.pem'), publicPath=join(directory,values.kid+'.public.json');
let createdPrivate=false;
try {
  writeFileSync(privatePath,pair.privateKey.export({type:'pkcs8',format:'pem'}),{flag:'wx',mode:0o600});
  createdPrivate=true;
  writeFileSync(publicPath,JSON.stringify({[values.kid]:pair.publicKey.export({format:'jwk'}).x},null,2)+'\n',{flag:'wx',mode:0o644});
} catch (error) {
  // A pre-existing public output must not leave a newly generated mismatched
  // private signer behind. Never remove a file that this invocation did not own.
  if (createdPrivate) unlinkSync(privatePath);
  throw error;
}
console.log(`Private signer saved to ${privatePath}\nPublic installation keys saved to ${publicPath}`);
