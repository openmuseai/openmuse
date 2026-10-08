/**
 * The operations model: queue ordering, the counts the panel entry reports,
 * and the two rules that keep a decision reachable — `input` survives a
 * clear, and a card can find the record that explains its own state.
 */

import { describe, expect, it } from 'vitest'
import {
  bucketOf, clearSettled, drop, enqueue, isSettled, needsUser, patch,
  queuePosition, recordForUrl, sortForPanel, summarize,
} from '../src/client/operations.ts'
import type { OperationRecord, OperationState } from '../src/client/operations.ts'

const rec = (id: string, state: OperationState, extra: Partial<OperationRecord> = {}): OperationRecord =>
  ({ id, kind: 'install', name: id, state, ...extra })

describe('operation state grouping', () => {
  it('collapses six states into the three the panel renders', () => {
    // The panel distinguishes buckets by icon and color; the status line
    // carries the rest. Six colors would not be readable.
    expect(bucketOf('queued')).toBe('busy')
    expect(bucketOf('running')).toBe('busy')
    expect(bucketOf('done')).toBe('ok')
    expect(bucketOf('warned')).toBe('ok')
    expect(bucketOf('input')).toBe('attention')
    expect(bucketOf('failed')).toBe('attention')
  })

  it('does not count a decision as finished', () => {
    // `input` means the host is done and the user is not. Treating it as
    // settled would sweep it out of the panel on the next clear.
    expect(isSettled(rec('a', 'input'))).toBe(false)
    expect(needsUser(rec('a', 'input'))).toBe(true)
    expect(isSettled(rec('b', 'failed'))).toBe(true)
    expect(needsUser(rec('b', 'failed'))).toBe(false)
  })
})

describe('the queue', () => {
  it('reports how many are ahead, so "queued" cannot read as stuck', () => {
    const list = [rec('a', 'running'), rec('b', 'queued'), rec('c', 'queued')]
    expect(queuePosition(list, 'b')).toBe(0)
    expect(queuePosition(list, 'c')).toBe(1)
    // A record that is not queued has no position, which is not the same as 0.
    expect(queuePosition(list, 'a')).toBeNull()
  })

  it('keeps enqueue order within a group so the panel reads as the run order', () => {
    const list = [rec('first', 'queued'), rec('second', 'queued'), rec('third', 'queued')]
    expect(sortForPanel(list).map(record => record.id)).toEqual(['first', 'second', 'third'])
  })

  it('floats what needs the user above what is still moving', () => {
    const list = [
      rec('done', 'done'), rec('running', 'running'),
      rec('clash', 'input'), rec('queued', 'queued'),
    ]
    expect(sortForPanel(list).map(record => record.id))
      .toEqual(['clash', 'running', 'queued', 'done'])
  })
})

describe('clearing', () => {
  it('sweeps finished records but never an unanswered decision', () => {
    // Clearing an `input` record would delete the only route back to that
    // choice: the install is already reverted, so nothing else would
    // re-raise it.
    const list = [
      rec('ok', 'done'), rec('bad', 'failed'), rec('warn', 'warned'),
      rec('clash', 'input'), rec('busy', 'running'),
    ]
    expect(clearSettled(list).map(record => record.id)).toEqual(['clash', 'busy'])
  })

  it('drops exactly one record by id', () => {
    const list = [rec('a', 'done'), rec('b', 'done')]
    expect(drop(list, 'a').map(record => record.id)).toEqual(['b'])
  })
})

describe('summarize', () => {
  it('counts the batch the entry reports, not individual rows', () => {
    const summary = summarize([
      rec('1', 'done'), rec('2', 'done'), rec('3', 'running'),
      rec('4', 'queued'), rec('5', 'queued'), rec('6', 'input'), rec('7', 'failed'),
    ])
    expect(summary.running).toBe(1)
    expect(summary.queued).toBe(2)
    // Neither queued record carries blockers here: `blocked` counts only the
    // ones the agent guard refused (#752).
    expect(summary.blocked).toBe(0)
    expect(summary.attention).toBe(1)
    // failed is settled; input is not, so it is excluded from the total the
    // "3 / 7" progress line divides.
    expect(summary.settled).toBe(3)
    expect(summary.total).toBe(6)
    expect(summary.progressed).toBe(4)
  })

  it('counts the queued records the agent guard is holding (#752)', () => {
    // The aggregate line needs this apart from `queued`: with nothing running,
    // "installing 0/3" states a contradiction, while "3 waiting for the agents
    // to go idle" names the obstacle. A record with no blockers keeps the
    // ordinary queue copy.
    const summary = summarize([
      rec('1', 'queued', { blockedBy: ['session-a', 'session-b'] }),
      rec('2', 'queued', { blockedBy: ['session-a'] }),
      rec('3', 'queued'),
      rec('4', 'running'),
    ])
    expect(summary.queued).toBe(3)
    expect(summary.blocked).toBe(2)
  })

  it('is all zeroes for an empty panel', () => {
    expect(summarize([])).toEqual({
      running: 0, queued: 0, blocked: 0, attention: 0, settled: 0, total: 0, progressed: 0,
    })
  })
})

describe('patch and card lookup', () => {
  it('replaces only the addressed record', () => {
    const list = [rec('a', 'running'), rec('b', 'running')]
    const next = patch(list, 'a', { state: 'done', needsRefresh: true })
    expect(next[0]).toMatchObject({ id: 'a', state: 'done', needsRefresh: true })
    expect(next[1]).toEqual(list[1])
  })

  it('gives a card its LATEST record, so a retry supersedes the rejection', () => {
    // Without newest-first the card would keep showing the clash it already
    // resolved, and its Install button would never come back.
    const list = enqueue(
      [rec('old', 'input', { url: 'u' }), rec('other', 'done', { url: 'v' })],
      rec('new', 'running', { url: 'u' }),
    )
    expect(recordForUrl(list, 'u')?.id).toBe('new')
    expect(recordForUrl(list, 'v')?.id).toBe('other')
    expect(recordForUrl(list, 'missing')).toBeNull()
  })
})
