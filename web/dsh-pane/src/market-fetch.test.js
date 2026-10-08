import assert from 'node:assert/strict'
import test from 'node:test'

import { rewriteEmbeddedMarketRequest } from './market-fetch.js'

const assetBase = 'http://127.0.0.1:4174/dsh/'
const documentBase = 'http://127.0.0.1:4174/app/'

test('catalog requests leave the Flutter base and hit the DSH mount', () => {
  assert.equal(
    rewriteEmbeddedMarketRequest('/app/dsh-market/registry', assetBase, documentBase),
    'http://127.0.0.1:4174/dsh/dsh-market/registry',
  )
})

test('query strings stay on the rewritten market request', () => {
  assert.equal(
    rewriteEmbeddedMarketRequest('/app/dsh-market/updates?force=1', assetBase, documentBase),
    'http://127.0.0.1:4174/dsh/dsh-market/updates?force=1',
  )
})

test('requests that already target the DSH mount are left alone', () => {
  const current = 'http://127.0.0.1:4174/dsh/dsh-market/registry'
  assert.equal(
    rewriteEmbeddedMarketRequest(current, assetBase, documentBase),
    current,
  )
})

test('other DSH calls are not rewritten', () => {
  assert.equal(
    rewriteEmbeddedMarketRequest('/dsh/api/session', assetBase, documentBase),
    '/dsh/api/session',
  )
})
