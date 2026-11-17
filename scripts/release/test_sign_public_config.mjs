import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {generateKeyPairSync, verify} from 'node:crypto';
import {spawnSync} from 'node:child_process';

test('offline signer preserves each regional identity and authenticates its payload', () => {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'openmuse-signer-test-'));
  try {
    const keys = generateKeyPairSync('ed25519');
    const privatePath = path.join(directory, 'signer.pem');
    fs.writeFileSync(privatePath, keys.privateKey.export({type: 'pkcs8', format: 'pem'}), {mode: 0o600});
    for (const [market, region] of [['china', 'cn'], ['overseas', 'us']]) {
      const release = JSON.parse(fs.readFileSync(`packages/openmuse_runtime_config/assets/releases/${market}.json`));
      const bundle = release.regions.find(item => item.id === region).bundle;
      bundle.bootstrap.publicKeys = {test: keys.publicKey.export({format: 'jwk'}).x};
      const input = path.join(directory, region + '.json');
      const output = path.join(directory, region + '.jws');
      fs.writeFileSync(input, JSON.stringify(bundle));
      const args = ['scripts/release/sign-public-config.mjs', '--bundle', input,
        '--private-key', privatePath, '--kid', 'test', '--sequence', '2', '--output', output];
      const result = spawnSync(process.execPath, args, {encoding: 'utf8'});
      assert.equal(result.status, 0, result.stderr);
      const [header, body, signature] = fs.readFileSync(output, 'utf8').split('.');
      const snapshot = JSON.parse(Buffer.from(body, 'base64url'));
      assert.equal(snapshot.releaseId, `${region}-official-2`);
      assert.equal(snapshot.scope.dataRegion, region);
      assert.equal(snapshot.deployment.authBindingId, bundle.snapshot.deployment.authBindingId);
      assert.ok(verify(null, Buffer.from(header + '.' + body), keys.publicKey, Buffer.from(signature, 'base64url')));
      assert.ok(!verify(null, Buffer.from(header + '.' + body + 'tampered'), keys.publicKey, Buffer.from(signature, 'base64url')));
      assert.notEqual(spawnSync(process.execPath, args).status, 0, 'existing publications must not be overwritten');
    }
  } finally {
    fs.rmSync(directory, {recursive: true, force: true});
  }
});


test('document browser contract rejects unsafe locations', async () => {
  const {validateBundle} = await import('../../services/config-control/src/contracts.mjs');
  const release = JSON.parse(fs.readFileSync('packages/openmuse_runtime_config/assets/releases/overseas.json'));
  const original = release.regions.find(item => item.id === 'us').bundle;
  assert.doesNotThrow(() => validateBundle(original));
  for (const location of ['http://example.com/', 'https://user:password@example.com/',
    'https://example.com/?host=other', 'https://example.com/#token', 'https://example.com/app']) {
    const bundle = structuredClone(original);
    bundle.snapshot.web.documentBaseUrl = location;
    assert.throws(() => validateBundle(bundle), location);
  }
});
