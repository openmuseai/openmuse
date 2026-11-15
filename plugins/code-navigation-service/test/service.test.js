import assert from 'node:assert/strict';
import test from 'node:test';
import { setTimeout as sleep } from 'node:timers/promises';
import { CodeNavigationService, createCodeNavigationPlugin, NavigationError } from '../lib/service.js';
import { fixture } from './support/owner.js';

const deferred = () => { let resolve; const promise = new Promise(r => resolve = r); return { promise, resolve }; };
const isCode = code => error => error.code === code;
async function setup(t, limits = {}, hooks = {}) {
  const f = await fixture(t);
  const metrics = { factories: 0, starts: 0, stops: 0, queries: 0, concurrent: 0, maxConcurrent: 0 };
  const trace = [];
  const service = new CodeNavigationService({ owner: f.owner, limits, trace: x => trace.push(x),
    backendFactory: () => {
      metrics.factories++;
      return {
        start: async (...args) => { metrics.starts++; await hooks.start?.(...args); },
        stop: async () => { metrics.stops++; await hooks.stop?.(); },
        query: async (...args) => {
          metrics.queries++; metrics.concurrent++; metrics.maxConcurrent = Math.max(metrics.maxConcurrent, metrics.concurrent);
          try {
            await hooks.query?.(...args);
            return { locations: [{ uri: f.rootUri + '/main.ts', range: { start: { line: 0, character: 0 }, end: { line: 0, character: 1 } } }] };
          } finally { metrics.concurrent--; }
        },
      };
    },
  });
  t.after(() => service.disable());
  return { ...f, service, metrics, trace };
}

test('default disabled and enabling/activation starts no process or timer', async t => {
  const f = await setup(t);
  await assert.rejects(f.service.query(f.input), isCode('DISABLED'));
  f.service.enable();
  assert.deepEqual(f.service.status, { enabled: true, stopping: false, instances: 0, pending: 0 });
  assert.equal(f.metrics.factories, 0);
  const plugin = createCodeNavigationPlugin({ owner: {}, backendFactory: () => assert.fail('must not start') });
  await plugin.activate(); await plugin.deactivate();
});
test('concurrent first queries single-flight one process and serialize documents', async t => {
  const f = await setup(t, {}, { query: () => sleep(10) });
  f.service.enable();
  const results = await Promise.all([f.service.query(f.input), f.service.query(f.input), f.service.query(f.input)]);
  assert.equal(f.metrics.starts, 1); assert.equal(f.metrics.maxConcurrent, 1);
  assert.equal(results[0].matches[0].reference.resource, 'authorized-target');
  assert.equal('uri' in results[0].matches[0], false);
  assert.equal(results[0].basis, 'semantic');
  assert.equal(JSON.stringify(f.trace).includes(f.rootUri), false);
});
test('version and position mismatch never start a server', async t => {
  const f = await setup(t); f.service.enable();
  await assert.rejects(f.service.query({ ...f.input, version: 'old' }), isCode('VERSION_MISMATCH'));
  await assert.rejects(f.service.query({ ...f.input, position: { line: 99, character: 0 } }), isCode('INVALID_POSITION'));
  assert.equal(f.metrics.starts, 0);
});
test('document bound is UTF-8 bytes, not character count', async t => {
  const f = await setup(t, { maxDocumentBytes: 1 }); f.service.enable();
  await assert.rejects(f.service.query(f.input), isCode('DOCUMENT_TOO_LARGE'));
  assert.equal(f.metrics.factories, 0);
});
test('global request admission is bounded', async t => {
  const entered = deferred(), release = deferred();
  const f = await setup(t, { maxPending: 1 }, { query: () => { entered.resolve(); return release.promise; } });
  f.service.enable(); const running = f.service.query(f.input); await entered.promise;
  await assert.rejects(f.service.query(f.input), isCode('BUSY'));
  release.resolve(); await running;
});
test('timeout resets an unresponsive backend and allows the next request', async t => {
  let first = true;
  const f = await setup(t, { startupTimeoutMs: 30, queryTimeoutMs: 30 }, { query: () => {
    if (first) { first = false; return new Promise(() => {}); }
  } });
  f.service.enable();
  await assert.rejects(f.service.query(f.input), isCode('TIMEOUT'));
  await f.service.stopAll();
  assert.equal(f.metrics.stops, 1);
  await f.service.query(f.input);
  assert.equal(f.metrics.starts, 2);
});
test('disable during startup fences results and releases the reserved process', async t => {
  const entered = deferred();
  const f = await setup(t, {}, { start: () => { entered.resolve(); return new Promise(() => {}); } });
  f.service.enable();
  const result = assert.rejects(f.service.query(f.input), isCode('DISABLED'));
  await entered.promise; await f.service.disable(); await result;
  assert.equal(f.service.status.instances, 0); assert.equal(f.metrics.stops, 1);
});
test('revocation after query denies results and resets the instance', async t => {
  let f;
  f = await setup(t, {}, { query: () => f.owner.revoke() }); f.service.enable();
  await assert.rejects(f.service.query(f.input), isCode('DENIED'));
  assert.equal(f.service.status.instances, 0);
});
test('idle processes are reclaimed without disabling service', async t => {
  const f = await setup(t, { idleMs: 15 }); f.service.enable();
  await f.service.query(f.input); await sleep(45);
  assert.equal(f.service.status.instances, 0); assert.equal(f.service.status.enabled, true);
  assert.equal(f.metrics.stops, 1);
});
test('caller cancellation is forwarded and prevents stale display', async t => {
  const entered = deferred();
  const f = await setup(t, {}, { query: () => { entered.resolve(); return new Promise(() => {}); } });
  f.service.enable(); const controller = new AbortController();
  const result = assert.rejects(f.service.query(f.input, { signal: controller.signal }), isCode('CANCELLED'));
  await entered.promise; controller.abort(); await result;
  await f.service.stopAll();
  assert.equal(f.metrics.stops, 1);
});
test('failed stop remains visible, blocks false success, and can be retried', async t => {
  let failStop = true;
  const f = await setup(t, {}, { stop: () => { if (failStop) throw new NavigationError('STOP_FAILED'); } });
  f.service.enable(); await f.service.query(f.input);
  await assert.rejects(f.service.disable(), isCode('STOP_FAILED'));
  assert.equal(f.service.status.instances, 1);
  failStop = false; await f.service.disable(); assert.equal(f.service.status.instances, 0);
});
test('busy instance cannot be evicted; idle one is evicted at the hard cap', async t => {
  const entered = deferred(), release = deferred();
  let block = true;
  const f = await setup(t, { maxInstances: 1 }, { query: () => {
    if (block) { entered.resolve(); return release.promise; }
  } });
  const read = f.owner.authorizeAndRead;
  f.owner.authorizeAndRead = async input => ({ ...await read(input), workspaceKey: input.workspace ?? 'workspace' });
  f.service.enable(); const running = f.service.query(f.input); await entered.promise;
  await assert.rejects(f.service.query({ ...f.input, workspace: 'other' }), isCode('BUSY'));
  release.resolve(); await running; block = false;
  await f.service.query({ ...f.input, workspace: 'other' });
  assert.equal(f.metrics.starts, 2); assert.equal(f.metrics.stops, 1);
  assert.equal(f.service.status.instances, 1);
});
test('generation partitions do not share a language process', async t => {
  const f = await setup(t);
  const read = f.owner.authorizeAndRead;
  f.owner.authorizeAndRead = async input => ({ ...await read(input), partition: input.partition ?? 'generation1' });
  f.service.enable(); await f.service.query(f.input);
  await f.service.query({ ...f.input, partition: 'generation2' });
  assert.equal(f.metrics.starts, 2);
});
test('result limits mark truncation before mapping additional targets', async t => {
  const f = await fixture(t);
  const location = { uri: f.rootUri + '/main.ts', range: { start: { line: 0, character: 0 }, end: { line: 0, character: 1 } } };
  let mapped = 0;
  const original = f.owner.resolveLocation;
  f.owner.resolveLocation = async (...args) => { mapped++; return original(...args); };
  const service = new CodeNavigationService({ owner: f.owner, limits: { maxResults: 2 }, backendFactory: () => ({
    start: async () => {}, stop: async () => {}, query: async () => ({ locations: Array(5).fill(location) }),
  }) });
  t.after(() => service.disable()); service.enable();
  const result = await service.query(f.input);
  assert.equal(result.truncated, true); assert.equal(result.matches.length, 2); assert.equal(mapped, 2);
});
test('target authorization failure never returns a raw LSP URI', async t => {
  const f = await setup(t); f.service.enable();
  f.owner.resolveLocation = () => { throw new NavigationError('DENIED'); };
  await assert.rejects(f.service.query(f.input), isCode('DENIED'));
});
test('queued cancellation returns before running query and retains bounded admission', async t => {
  const entered = deferred(), release = deferred();
  const f = await setup(t, { maxPending: 2 }, { query: () => { entered.resolve(); return release.promise; } });
  f.service.enable(); const running = f.service.query(f.input); await entered.promise;
  const controller = new AbortController();
  const cancelled = assert.rejects(f.service.query(f.input, { signal: controller.signal }), isCode('CANCELLED'));
  await sleep(5); controller.abort(); await cancelled;
  assert.equal(f.metrics.queries, 1);
  await assert.rejects(f.service.query(f.input), isCode('BUSY'));
  release.resolve(); await running;
});
