import {existsSync, mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {createPrivateKey, createPublicKey, generateKeyPairSync} from 'node:crypto';
import {ControlStore} from './store.mjs';
import {createControlServer} from './server.mjs';
import {https} from './contracts.mjs';

const settings = JSON.parse(readFileSync(new URL('../dev-settings.json', import.meta.url), 'utf8'));
const adminOrigin = process.env.OPENMUSE_CONFIG_ADMIN_ORIGIN ?? settings.adminOrigin;
const publicOrigin = https(process.env.OPENMUSE_CONFIG_PUBLIC_ORIGIN ?? settings.publicOrigin, true).origin;
const local = new URL(adminOrigin);
if (local.protocol !== 'http:' || local.hostname !== '127.0.0.1') throw Error('Development mode binds only to IPv4 loopback HTTP');
const data = new URL('../.local/', import.meta.url); mkdirSync(data, {recursive: true, mode: 0o700});
const keyPath = new URL('signing-key.pem', data);
if (!existsSync(keyPath)) writeFileSync(keyPath, generateKeyPairSync('ed25519').privateKey.export({type: 'pkcs8', format: 'pem'}), {mode: 0o600, flag: 'wx'});
const privateKey = createPrivateKey(readFileSync(keyPath));
const store = new ControlStore({database: new URL('control.sqlite', data), privateKey, kid: settings.signingKeyId, publicOrigin});
if (!store.listScopes().length) {
  const bundle = JSON.parse(readFileSync(new URL(`../${settings.deploymentBundle}`, import.meta.url), 'utf8'));
  bundle.snapshot.environment = 'development'; bundle.snapshot.distributionId = 'local-demo';
  bundle.snapshot.releaseId = 'bundled-local-demo';
  bundle.bootstrap.cohortRouting = true;
  bundle.bootstrap.discoveryUrls = [`${publicOrigin}/v1/runtime/scopes/${settings.scopeId}/active.jws`];
  bundle.bootstrap.publicKeys = {[settings.signingKeyId]: createPublicKey(privateKey).export({format: 'jwk'}).x};
  bundle.bootstrap.allowedResourceOrigins = [publicOrigin];
  store.createScope(settings.scopeId, bundle, 'development-bootstrap');
  writeFileSync(new URL('deployment.local.json', data), JSON.stringify(bundle, null, 2));
}
const server = createControlServer({store, localOrigin: adminOrigin, demo: true});
server.listen(Number(local.port), '127.0.0.1', () => console.log(`Local configuration admin ready at ${adminOrigin}`));
process.on('SIGTERM', () => server.close(() => {store.close(); process.exit(0);}));
