import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

import {
  OpenMuseContractError,
  contractErrorCodes,
  ensureLiveAt,
  parseContractEnvelope,
  parseLifecycleSnapshot,
} from '../lib/index.js';
import type { ContractErrorCode, RequestEnvelope } from '../lib/index.js';

const fixtureRoot = new URL('../../../../../schemas/fixtures/contract/v1/', import.meta.url);

async function fixture(name: string): Promise<unknown> {
  return JSON.parse(await readFile(new URL(name, fixtureRoot), 'utf8'));
}

for (const name of [
  'request.success.json',
  'response.success.json',
  'response.denied.json',
  'response.expired.json',
  'response.conflict.json',
]) {
  test(`round trips ${name}`, async () => {
    const input = await fixture(name);
    assert.deepEqual(parseContractEnvelope(input, { expectedGeneration: 3 }), input);
  });
}

test('rejects a stale generation before exposing the outcome', async () => {
  const input = await fixture('response.stale-generation.json');
  assert.throws(
    () => parseContractEnvelope(input, { expectedGeneration: 3 }),
    (error) => error instanceof OpenMuseContractError && /stale generation/.test(error.message),
  );
});

test('rejects an unknown protocol major', async () => {
  const input = await fixture('request.unknown-major.json');
  assert.throws(
    () => parseContractEnvelope(input, { expectedGeneration: 3 }),
    (error) => error instanceof OpenMuseContractError && /incompatible protocol/.test(error.message),
  );
});

test('round trips descriptor, handle, lease and receipt lifecycle values', async () => {
  const input = await fixture('lifecycle.snapshot.json');
  assert.deepEqual(parseLifecycleSnapshot(input), input);
});

test('deadline checks fail closed and error vocabulary remains frozen', async () => {
  const request = parseContractEnvelope(await fixture('request.success.json'));
  assert.equal(request.kind, 'request');
  const typedRequest = request as RequestEnvelope;
  assert.throws(() => ensureLiveAt(typedRequest, typedRequest.deadlineAtMs));
  const expectedCodes: ContractErrorCode[] = [
    'DENIED', 'NOT_FOUND', 'CONFLICT', 'EXPIRED', 'STALE_GENERATION',
    'UNAVAILABLE', 'TRANSIENT', 'INTEGRITY_FAILED',
  ];
  assert.deepEqual(contractErrorCodes, expectedCodes);
});
