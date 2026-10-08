/**
 * The market's own settings section: what makes `allowRestart` a switch on
 * the plugin configuration page instead of a hand-edited YAML line.
 *
 * Only what a unit can honestly decide lives here: the schema's defaults,
 * and that the settings service is an OPTIONAL injection so a host without
 * one (every dsh before 0.1.0-rc.7) mounts everything else unchanged.
 *
 * Whether the namespace actually reaches a real host is asserted in layer 3
 * against real dsh, not against a hand-written stand-in of the settings
 * service — a fake would only prove this code agrees with my reading of a
 * contract I did not write.
 */

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { installMarketSettings, MarketSettings, settingsNamespaceState } from '../src/settings.ts'

/** Minimal cordis stand-in recording the optional `settings` injection. */
function fakeContext(hasSettings: boolean) {
  const injected: string[][] = []
  const ctx = {
    injected,
    inject(services: string[], callback: (scoped: unknown) => void) {
      injected.push(services)
      if (hasSettings && services.includes('settings')) callback(ctx)
    },
    settings: hasSettings ? {} : undefined,
    effect: (run: () => unknown) => { run() },
    on: () => () => {},
  }
  return ctx
}

/** Minimal cordis stand-in whose settings service DOES take a namespace. */
function fakeContextWithRegister() {
  const ctx = {
    inject(services: string[], callback: (scoped: unknown) => void) {
      if (services.includes('settings')) callback(ctx)
    },
    settings: {
      register: () => ({ get: () => ({ allowRestart: true }), watch: () => {} }),
    },
    effect: (run: () => unknown) => { run() },
    on: () => () => {},
  }
  return ctx
}

describe('what the market reports about its namespace (#677)', () => {
  // The two answers a reader can get, and they exist because a host
  // generation changed the contract underneath: 0.1.7 derives settings from
  // a plugin's Config schema and serves no third-party namespace, so the
  // market's plugin-configuration card cannot be dispatched there. Without
  // this the difference is invisible — a test (or a bug report) cannot tell
  // "the host cannot serve it" from "the market failed to register it".
  it('is unsupported-by-host when the settings service has no register()', () => {
    installMarketSettings(fakeContext(true) as never, { allowRestart: true })
    expect(settingsNamespaceState()).toBe('unsupported-by-host')
  })

  it('is registered when the host takes the namespace', () => {
    installMarketSettings(fakeContextWithRegister() as never, { allowRestart: true })
    expect(settingsNamespaceState()).toBe('registered')
  })
})

describe('MarketSettings schema', () => {
  it('defaults allowRestart to on', () => {
    expect(MarketSettings({}).allowRestart).toBe(true)
  })

  it('accepts an explicit off', () => {
    expect(MarketSettings({ allowRestart: false }).allowRestart).toBe(false)
  })

  it('claims only what this namespace actually stores', () => {
    // The release channel was in here for one version, and it made this a
    // SECOND writer for a value that lives in the market's state.json. The
    // routes read the saved channel off disk at mount and `onChange` — which
    // cannot see that file — assigned its own idea of the field straight
    // back over it, so the user's choice survived until the next settings
    // event and no further.
    //
    // A schema field is a claim of ownership, so this asserts the claim
    // stays narrow — widening it silently is exactly how that happened.
    // The consequence itself is caught in layer 3 (tests/web/channel.e2e.ts)
    // against a real settings service, per this file's own rule about not
    // hand-writing a stand-in for a contract we did not author.
    expect(Object.keys(MarketSettings({}))).toEqual(['allowRestart'])
  })
})

describe('installMarketSettings', () => {
  it('asks for the settings service optionally, never as a hard dependency', () => {
    const ctx = fakeContext(false)
    installMarketSettings(ctx as never, { allowRestart: true })
    // A host without the service must still mount everything else: the
    // registration rides its own scoped fiber.
    expect(ctx.injected.flat()).toContain('settings')
    expect(ctx.injected.flat()).not.toContain('webServer')
  })

  it('does not throw on a dsh 0.1.7 settings service, which has no register (#677)', () => {
    // 0.1.7's SettingsService exposes describe/update only; namespaces are
    // derived from a plugin's Config schema. The service still EXISTS, so the
    // inject callback runs — and calling register threw a TypeError that
    // cordis swallowed. The composed entry has to stand, and the host log
    // has to say why the switch is absent rather than stay silent.
    const warnings: string[] = []
    const ctx = {
      inject(services: string[], callback: (scoped: unknown) => void) {
        if (services.includes('settings')) callback(ctx)
      },
      settings: { describe: () => [], update: async () => {} },
      effect: (run: () => unknown) => { run() },
      logger: () => ({ warn: (message: string) => { warnings.push(message) } }),
    }
    const resolved = { allowRestart: false }
    expect(() => installMarketSettings(ctx as never, resolved)).not.toThrow()
    expect(resolved.allowRestart).toBe(false)
    expect(warnings).toHaveLength(1)
    expect(warnings[0]).toMatch(/no register\(\)/)
  })

  it('still registers on a pre-0.1.7 settings service', () => {
    // The guard must not swallow a host where register works: that would
    // quietly remove the switch from every host that has it today.
    const registered: string[] = []
    const ctx = {
      inject(services: string[], callback: (scoped: unknown) => void) {
        if (services.includes('settings')) callback(ctx)
      },
      settings: {
        register: (ns: string, _schema: unknown, options: { base: unknown }) => {
          registered.push(ns)
          return { get: () => options.base, watch: () => () => {} }
        },
      },
      effect: (run: () => unknown) => { run() },
    }
    installMarketSettings(ctx as never, { allowRestart: true })
    expect(registered).toEqual(['dsh-market'])
  })

  it('takes nothing from @deepseek-ai/dsh-settings at runtime', () => {
    // dsh 0.1.2-alpha.1 deleted `installSettingsSection` and moved
    // `settingsNamespace` elsewhere. This module imported both, and the
    // result was not a missing feature — it was the HOST FAILING TO BOOT:
    //
    //   SyntaxError: The requested module '@deepseek-ai/dsh-settings' does
    //   not provide an export named 'installSettingsSection'
    //
    // That is the distinction this guard exists for. `ctx.inject` degrades
    // quietly when a SERVICE is absent, which is the graceful path this file
    // already tests above. An ESM import of a missing EXPORT cannot degrade
    // at all: it throws while the module is being evaluated, cordis reports
    // a failed entry, and dsh exits 1 with the market installed. A plugin
    // must never be able to stop the host from starting.
    //
    // The service is the stable surface — `settings.register(ns, schema,
    // { base })` is byte-identical in 0.1.0-rc.7 and 0.1.2-alpha.2 — so the
    // rule is simply: reach the settings service through injection, never
    // through this package's exports. Scanned rather than mocked, because
    // this is a fact about our own source, not a claim about their contract
    // (see the note at the top of this file).
    const source = readFileSync(resolve('src/settings.ts'), 'utf8')
    const runtimeImports = [...source.matchAll(/^import\s+(?!type\b)(.+?)\s+from\s+'([^']+)'/gmu)]
      .filter(match => match[2]!.startsWith('@deepseek-ai/dsh-settings'))
    expect(runtimeImports.map(match => match[0]), 'import the settings SERVICE via ctx.inject instead').toEqual([])
  })
})
