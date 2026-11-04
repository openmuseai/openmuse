/**
 * Host-compatibility guard: on hosts whose injected primitives module
 * predates rc.6, the named exports the market renders with are undefined
 * (the module itself resolves, so the bundle factory succeeds). apply()
 * must detect the gap and skip registration instead of throwing mid-render.
 */
import { describe, expect, it } from 'vitest'
import { apply, missingPrimitives, REQUIRED_PRIMITIVES } from '../../src/client/index.ts'
import { ICON_ALIASES, missingIcons } from '../../src/client/icons.ts'

describe('missingPrimitives', () => {
  it('reports no gaps when every required export exists', () => {
    const mod: Record<string, unknown> = {}
    for (const name of REQUIRED_PRIMITIVES) mod[name] = () => null
    expect(missingPrimitives(mod)).toEqual([])
  })

  it('names the missing exports on an old host', () => {
    const mod: Record<string, unknown> = { Menu: () => null, Toast: () => null }
    expect(missingPrimitives(mod)).toEqual(['DisclosureRow', 'Tooltip'])
  })

  it('reports every requirement when the module is empty', () => {
    expect(missingPrimitives({})).toEqual([...REQUIRED_PRIMITIVES])
  })

  it('accepts a custom requirement list', () => {
    expect(missingPrimitives({ A: 1 }, ['A', 'B', 'C'])).toEqual(['B', 'C'])
  })
})

describe('apply() icon gaps (#671)', () => {
  it('treats a 0.1.7-only Regular icon table as complete for missingIcons', () => {
    const mod: Record<string, unknown> = {}
    for (const name of REQUIRED_PRIMITIVES) mod[name] = () => null
    for (const [, newer] of ICON_ALIASES) mod[newer] = () => null
    expect(missingPrimitives(mod)).toEqual([])
    expect(missingIcons(mod)).toEqual([])
  })

  it('does not treat icon gaps as a hard apply() disable', () => {
    // Missing Menu still disables; missing icons alone must not — icons.ts
    // skips the glyph so a rename costs one icon, not the settings page.
    const mod: Record<string, unknown> = {}
    for (const name of REQUIRED_PRIMITIVES) mod[name] = () => null
    expect(missingPrimitives(mod)).toEqual([])
    expect(missingIcons(mod).length).toBeGreaterThan(0)
  })
})

/** A host that declares only the slots it has, and records what the market did. */
function hostWith(declared: readonly string[]) {
  const registrations: Record<string, unknown>[] = []
  const injections: string[][] = []
  const slots = new Set(declared)
  apply({
    effect: (run: () => unknown) => { run() },
    on: () => () => {},
    locale: {
      register: () => () => {}, bind: () => (key: string) => key,
      subscribe: () => () => {}, getSnapshot: () => ({ active: 'en' }),
    },
    theme: { getTheme: () => null, setTheme: () => {} },
    slots: {
      // A real host fires this only for the slots it declares — which is what
      // makes the slot itself the feature detection for the newer card seat.
      inject: (slot: string, register: () => unknown) => { if (slots.has(slot)) register() },
      register: (options: Record<string, unknown>) => { registrations.push(options); return () => {} },
    },
    // The service injections are RECORDED but never fired on this host: that
    // is the "host without settingsScope" case the market must survive.
    inject: (services: string[]) => { injections.push(services) },
  } as Parameters<typeof apply>[0])
  return { registrations: registrations.map(entry => entry.name), injections }
}

it('keeps the main market on hosts without settingsScope (#516)', () => {
  const { registrations, injections } = hostWith(['settings.section', 'shell.overlay'])
  expect(injections).toEqual([['settingsScope']])
  expect(registrations).toEqual(['settings.section', 'shell.overlay'])
})

it('offers the market as a Plugins-page tab on the line that declares one (#722)', () => {
  // 0.1.7 removed `settingsScope` and `settings.plugin.item`, and no host
  // declares `plugins.bundle.config` — so the card had no seat on that line at
  // all and vanished silently. The tab slot is what its contract offers for a
  // page inside the Plugins section, and it is the one the market must follow.
  const { registrations, injections } = hostWith(['settings.section', 'shell.overlay', 'settings.plugins.tab'])
  expect(injections).toEqual([['settingsScope']])
  expect(registrations).toEqual(['settings.section', 'settings.plugins.tab', 'shell.overlay'])
})

it('places the settings card on the bundle page a 0.1.7 host declares (#677)', () => {
  // The newer line has no plugin-configuration page and no settingsScope; it
  // declares `plugins.bundle.config`, which its own contract names as where a
  // third-party bundle's configuration belongs. The card must follow the
  // slot, and the market must still register everything else.
  const { registrations, injections } = hostWith(['settings.section', 'shell.overlay', 'plugins.bundle.config'])
  expect(injections).toEqual([['settingsScope']])
  expect(registrations).toEqual(['settings.section', 'plugins.bundle.config', 'shell.overlay'])
})
