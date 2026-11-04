import test from 'node:test'
import assert from 'node:assert/strict'
import { ownsPairedHost } from './transport-policy.js'

const origin = 'https://us.openmuseai.com'
test('authenticated Desktop bootstrap can configure its own Host', () => {
  assert.equal(ownsPairedHost('/openmuse/dsh/', `${origin}/openmuse/dsh/`, origin), true)
  assert.equal(ownsPairedHost(`/u/${'a'.repeat(64)}`, `${origin}/openmuse/dsh/`, origin), true)
})
test('generic, unredirected and foreign DSH pages keep remote settings policy', () => {
  for (const [path, url] of [
    ['/dsh/', `${origin}/dsh/`],
    ['/dsh/', `${origin}/openmuse/dsh/`],
    ['/openmuse/dsh/', 'https://other.example/openmuse/dsh/'],
    [`/u/${'a'.repeat(64)}`, `${origin}/u/${'a'.repeat(64)}`],
    ['/u/invalid', `${origin}/openmuse/dsh/`],
  ]) assert.equal(ownsPairedHost(path, url, origin), false)
})
