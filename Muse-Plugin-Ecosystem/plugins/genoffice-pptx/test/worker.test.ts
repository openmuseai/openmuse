import test from 'node:test'
import assert from 'node:assert/strict'
import { join } from 'node:path'
import { mkdtemp, rm } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { inspect, roundtrip } from '../src/worker'

const fixture = join(import.meta.dirname, '../vendor/pptx-engine/tests/fixtures/01_standard_business.pptx')

test('GenOffice PPTX core inspects and round trips a real deck', async () => {
  const directory = await mkdtemp(join(tmpdir(), 'openmuse-pptx-'))
  try {
    const result = await inspect(fixture)
    assert.ok(result.slides > 0)
    const output = join(directory, 'copy.pptx')
    const saved = await roundtrip(fixture, output)
    assert.equal(saved.slidesPreserved, true)
    const reopened = await inspect(output)
    assert.equal(reopened.slides, result.slides)
    assert.deepEqual(reopened.pages, result.pages)
  } finally {
    await rm(directory, { recursive: true, force: true })
  }
})
