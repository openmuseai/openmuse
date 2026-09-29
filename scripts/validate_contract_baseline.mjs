import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, readdir } from 'node:fs/promises';

import {
  OpenMuseContractError,
  parseContractEnvelope,
  parseLifecycleSnapshot,
} from '../third_party/dsh/plugins/openmuse-contract/lib/index.js';

const fixtureRoot = new URL('../schemas/fixtures/contract/v1/', import.meta.url);
const manifestUrl = new URL('manifest.json', fixtureRoot);
const manifest = JSON.parse(await readFile(manifestUrl, 'utf8'));

assert.equal(manifest.protocol, 'openmuse.contract');
assert.equal(manifest.major, 1);
assert.equal(typeof manifest.owner, 'string');
assert.ok(manifest.owner.length > 0, 'fixture owner is required');

const schemaUrl = new URL(manifest.schema, fixtureRoot);
const schemaBytes = await readFile(schemaUrl);
const schemaDigest = createHash('sha256').update(schemaBytes).digest('hex');
assert.equal(schemaDigest, manifest.schemaSha256, 'schema digest changed; review compatibility and update manifest');
JSON.parse(schemaBytes.toString('utf8'));

const inventory = new Set(manifest.fixtures.map((item) => item.file));
const actual = (await readdir(fixtureRoot))
  .filter((name) => name.endsWith('.json') && name !== 'manifest.json');
assert.deepEqual(new Set(actual), inventory, 'fixture manifest must own every JSON fixture');

for (const entry of manifest.fixtures) {
  const input = JSON.parse(await readFile(new URL(entry.file, fixtureRoot), 'utf8'));
  assertFixtureSafe(input, entry.file);

  if (entry.expectation === 'accept') {
    const parsed = entry.kind === 'lifecycle'
      ? parseLifecycleSnapshot(input)
      : parseContractEnvelope(input, { expectedGeneration: entry.generation });
    assert.deepEqual(parsed, input, `${entry.file} must round trip`);
    continue;
  }

  assert.throws(
    () => parseContractEnvelope(input, { expectedGeneration: entry.generation }),
    (error) => {
      if (!(error instanceof OpenMuseContractError)) return false;
      if (entry.expectation === 'reject-generation') return /stale generation/.test(error.message);
      if (entry.expectation === 'reject-protocol') return /incompatible protocol/.test(error.message);
      return false;
    },
    `${entry.file} must fail closed as ${entry.expectation}`,
  );
}

console.log(`Contract baseline: ${manifest.fixtures.length} fixtures validated; schema ${schemaDigest}`);

function assertFixtureSafe(value, location) {
  const forbiddenKeys = new Set([
    'path', 'cwd', 'secret', 'token', 'accesskey', 'secretkey', 'sessiontoken',
    'endpoint', 'bucket', 'providerclient', 'sdkclient',
  ]);
  if (Array.isArray(value)) {
    value.forEach((item, index) => assertFixtureSafe(item, `${location}[${index}]`));
    return;
  }
  if (value === null || typeof value !== 'object') {
    if (typeof value === 'string') {
      assert.ok(!/^(?:\/|[A-Za-z]:\\|file:\/\/|s3:\/\/)/u.test(value), `${location} contains a physical path`);
      assert.ok(!/(?:AmazonS3Client|S3Client|MinioClient|RustFSClient)/u.test(value), `${location} contains a vendor SDK type`);
    }
    return;
  }
  for (const [key, item] of Object.entries(value)) {
    assert.ok(!forbiddenKeys.has(key.toLowerCase()), `${location}.${key} is forbidden in a canonical fixture`);
    assertFixtureSafe(item, `${location}.${key}`);
  }
}
