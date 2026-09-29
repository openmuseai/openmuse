import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, readdir } from 'node:fs/promises';

const fixtureRoot = new URL('../schemas/fixtures/plugin/v2/', import.meta.url);
const manifest = JSON.parse(await readFile(new URL('manifest.json', fixtureRoot), 'utf8'));
assert.equal(typeof manifest.owner, 'string');
assert.ok(manifest.owner.length > 0, 'fixture owner is required');

const schemaBytes = await readFile(new URL(manifest.schema, fixtureRoot));
const digest = createHash('sha256').update(schemaBytes).digest('hex');
assert.equal(digest, manifest.schemaSha256, 'v2 schema changed; update compatibility review and digest');
JSON.parse(schemaBytes.toString('utf8'));

const actual = (await readdir(fixtureRoot))
  .filter((name) => name.endsWith('.json') && name !== 'manifest.json');
assert.deepEqual(new Set(actual), new Set(manifest.fixtures), 'fixture inventory mismatch');

for (const name of manifest.fixtures) {
  const value = JSON.parse(await readFile(new URL(name, fixtureRoot), 'utf8'));
  assert.equal(value.manifest_version, 2, `${name}: manifest version`);
  assert.deepEqual(
    new Set(value.compatibility.targets.map((item) => item.target.os)),
    new Set(['macos', 'windows', 'linux', 'android', 'ios']),
    `${name}: explicit native platform decisions`,
  );
  for (const os of ['android', 'ios']) {
    const decision = value.compatibility.targets.find((item) => item.target.os === os);
    assert.equal(decision.status, 'unsupported', `${name}: ${os} must not be selected before its implementation ships`);
    assert.ok(decision.reason.length > 0, `${name}: ${os} reason`);
  }
  const supported = new Set(
    value.compatibility.targets
      .filter((item) => item.status === 'supported')
      .map((item) => JSON.stringify(item.target)),
  );
  for (const artifact of value.artifacts) {
    assert.ok(supported.has(JSON.stringify(artifact.target)), `${name}: artifact target must be supported`);
    assert.match(artifact.digest.value, /^[0-9a-f]{64}$/u, `${name}: artifact sha256`);
  }
  assertNoPhysicalAuthority(value, name);
}

console.log(`Plugin manifest v2: ${manifest.fixtures.length} fixtures; schema ${digest}`);

function assertNoPhysicalAuthority(value, location) {
  if (Array.isArray(value)) {
    value.forEach((item, index) => assertNoPhysicalAuthority(item, `${location}[${index}]`));
    return;
  }
  if (value === null || typeof value !== 'object') {
    if (typeof value === 'string') {
      assert.ok(!/^(?:\/|[A-Za-z]:\\|file:\/\/)/u.test(value), `${location} contains an absolute path`);
    }
    return;
  }
  for (const [key, item] of Object.entries(value)) {
    assert.ok(!['secret', 'token', 'private_key', 'credential'].includes(key.toLowerCase()), `${location}.${key} embeds authority`);
    assertNoPhysicalAuthority(item, `${location}.${key}`);
  }
}
