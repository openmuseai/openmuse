/**
 * Theme lifecycle: classifying installed packages as themes (by the
 * registry's theme category), live-toggling bundle-layer entries through
 * the loader, and keeping exactly one theme active with the choice
 * persisted across restarts.
 */

import { join } from 'node:path'
import { loadRegistry, pluginCategories } from './registry.ts'
import { hotMount, hotUnmount, listHotMounts, writeDisabled } from './hot.ts'
import { logEvent } from './log.ts'
import { LifecyclePendingError, LifecycleWaitError, waitForLifecycle } from './lifecycle.ts'
import { nameMatchesPackage } from './entry-identity.ts'
import { bundlePatchInsertedIds, profileDir, readInstalled } from './profile.ts'
import { repoOf } from './sources.ts'

/** The slice of a cordis loader entry the market needs for live enable/disable. */
export interface LoaderEntry {
  options: { id?: string; name?: string; disabled?: boolean | null }
  fiber?: unknown
  update(options: { disabled: boolean | null }, create?: boolean, force?: boolean): Promise<void>
}

/** The host surface the theme manager needs (loader entries + hot-mount context). */
export interface ThemeHost {
  loader: { entries(): Iterable<LoaderEntry> }
  plugin(plugin: unknown, config: unknown): { await(): Promise<unknown>; dispose(): Promise<unknown> | void }
}

/** Manages theme exclusivity for one profile. */
export interface ThemeManager {
  installedThemeNames(): Promise<Set<string>>
  setEntryDisabled(name: string, disabledFlag: boolean, requireLive?: boolean, rejectPending?: boolean): Promise<boolean>
  activateTheme(name: string): Promise<boolean>
}

/**
 * Whether a loader entry belongs to `packageName`.
 *
 * A bundle patch need not name its own package. Two shapes exist (#619, the
 * toggle-side half of #71):
 *
 *  - a SUBPATH entry — `aegis` mounts `aegis/extensions/dsh/index.js`,
 *    `toolshrink` mounts `toolshrink/harness`. Matched with the same `name/`
 *    bound `liveIncludes()` uses, so a differently-suffixed package
 *    (`toolshrink-extra`) can never match.
 *  - a CARRIER bundle — `@deepseek-ai/dsh-experimental-agent-team-profile`
 *    mounts entries named `@deepseek-ai/dsh-experimental-agent-team` and
 *    `@deepseek-ai/dsh-experimental-tool-agent-team`. There is no name
 *    relation at all, so this falls back to the entry id the package's own
 *    patch inserts — the rule `carriedRowLive()` uses (#156).
 */
function ownsLoaderEntry(
  entry: LoaderEntry,
  packageName: string,
  ownedIds: ReadonlySet<string>,
): boolean {
  const entryName = entry.options.name
  if (nameMatchesPackage(entryName, packageName)) return true
  const id = entry.options.id
  if (id === undefined || id === '') return false
  // Loader ids may carry an include prefix (`include:<key>:<id>`); the bare
  // id is what a bundle patch declares.
  return ownedIds.has(id.split(':').pop() ?? id)
}

/**
 * Create the theme manager. `disabledThemes` is the live, shared set of
 * themes the user switched off — the caller owns reading it at boot and
 * replaying it; the manager mutates and persists it on switches.
 */
export function createThemeManager(
  host: ThemeHost,
  profile: string,
  disabledThemes: Set<string>,
  explicitDir?: string,
): ThemeManager {
  const activeProfileDir = profileDir(profile, explicitDir)
  /** Installed package names classified as themes by the registry's theme category. */
  async function installedThemeNames(): Promise<Set<string>> {
    const names = new Set<string>()
    try {
      const registry = await loadRegistry()
      const themeEntries = registry.plugins.filter(p => pluginCategories(p).includes('theme'))
      const themeNames = new Set(themeEntries.map(p => p.name))
      const themeRepos = new Set(
        themeEntries.map(p => repoOf(p.url)).filter((r): r is string => r !== null).map(r => r.toLowerCase()),
      )
      for (const [name, spec] of Object.entries(readInstalled(profile, activeProfileDir))) {
        if (themeNames.has(name)) {
          names.add(name)
          continue
        }
        const match = /github:([^#\s]+)/.exec(String(spec).toLowerCase())
        if (match !== null && themeRepos.has(match[1])) names.add(name)
      }
    } catch { /* registry unavailable — nothing classifies as a theme */ }
    return names
  }

  /**
   * Live-toggle a bundle-layer plugin through its loader entry. Bundle trees
   * are in-memory (write is a no-op), so this never touches any file — the
   * market persists the choice itself and replays it at boot.
   * @returns true when a matching live entry was found and updated.
   */
  /** Every loader entry that belongs to this package (its own rows and names). */
  function matchingEntries(name: string): LoaderEntry[] {
    const ownedIds = new Set(bundlePatchInsertedIds(join(activeProfileDir, 'node_modules', name)))
    return [...host.loader.entries()].filter(entry => ownsLoaderEntry(entry, name, ownedIds))
  }

  /**
   * Entries whose last update did not settle.
   *
   * A rejected update leaves the loader object in a state nobody can describe,
   * so a later ENABLE of the same entry is refused until a disable (which
   * cannot make things worse) has been seen through. Without this an enable
   * after a crashed disable reports success over a fiber that is not there.
   */
  const uncertainEntries = new WeakSet<LoaderEntry>()
  async function updateEntry(entry: LoaderEntry, disabled: boolean | null, restoring = false): Promise<void> {
    if (!disabled && !restoring && uncertainEntries.has(entry)) {
      throw new LifecycleWaitError('loader entry: runtime state is uncertain; retry disabling before enabling')
    }
    try {
      await waitForLifecycle(entry, () => entry.update({ disabled }, false, true),
        `${entry.options.name ?? entry.options.id ?? 'entry'}: loader update`)
      uncertainEntries.delete(entry)
    } catch (error) {
      uncertainEntries.add(entry)
      throw error
    }
  }

  /**
   * Live-toggle a bundle-layer plugin through its loader entry.
   *
   * `requireLive` is the strict mode: the caller needs the requested state to
   * be true when this returns, so an update that does not settle is an error
   * rather than a log line, and every entry touched on the way is restored to
   * the state it had before (#575, #582). The legacy theme and boot-replay
   * callers keep the best-effort semantics they were written against.
   *
   * @returns true when a matching live entry was found and updated.
   */
  async function setEntryDisabled(name: string, disabledFlag: boolean, requireLive = false, rejectPending = false): Promise<boolean> {
    const previous: { entry: LoaderEntry; disabled: boolean | null | undefined; live: boolean }[] = []
    try {
      let found = false
      // Whether ANY loader entry was selected, as opposed to whether an update
      // landed. The log conflated the two (#788): `found` only turns true after
      // a successful update, so an entry that WAS matched but whose update threw
      // was reported as "no loader entry matched" — which sent the reporter
      // looking for a naming bug (#619's shape) in a profile where the match
      // was fine and the runtime was simply still busy.
      let matched = false
      for (const entry of matchingEntries(name)) {
        matched = true
        if (!disabledFlag && uncertainEntries.has(entry)) {
          throw new LifecycleWaitError('loader entry: runtime state is uncertain; retry disabling before enabling')
        }
        if (requireLive) previous.push({ entry, disabled: entry.options.disabled, live: entry.fiber !== undefined })
        // Whether THIS entry's update landed, so the "-> off/on" line below is
        // only written for a change that happened. It used to be written after
        // a thrown update too, reading "off: fiber=true" for an entry that was
        // still running.
        let landed = false
        // A disable can land while the entry's init is still in flight: the
        // options flip but the finishing init brings the fiber up anyway, and a
        // plain re-update no-ops on the empty diff. Force the update and verify
        // the live state, retrying until reality matches the flag.
        for (let attempt = 0; attempt < 3; attempt++) {
          try {
            await updateEntry(entry, disabledFlag ? true : null)
            found = true
            landed = true
          } catch (error) {
            // A strict caller cannot accept an update that did not settle; a
            // pending one is also fatal for a caller that asked to be told.
            if (requireLive || (rejectPending && error instanceof LifecycleWaitError)) throw error
            // "Still pending" is the SAME update that already timed out, not a
            // new failure: the first `did not settle within 10s` stays in the
            // log, and every retry that merely bumps into it would otherwise
            // add a line that reads as one more thing going wrong (#788 — 25 of
            // them across restarts for one stuck entry).
            if (!(error instanceof LifecyclePendingError)) {
              logEvent('warn', 'toggle', `${name}: entry update failed — ${error instanceof Error ? error.message : String(error)}`)
            }
            break
          }
          const live = entry.fiber !== undefined
          if (live !== disabledFlag) break
          await new Promise(resolvePromise => setTimeout(resolvePromise, 200))
        }
        if (requireLive && (entry.fiber !== undefined) === disabledFlag) {
          throw new Error(`${name}: loader entry ${disabledFlag ? 'did not stop' : 'did not become live'}`)
        }
        if (landed) {
          logEvent('info', 'toggle',
            `${name} -> ${disabledFlag ? 'off' : 'on'}: fiber=${String(entry.fiber !== undefined)}`)
        }
      }
      if (!matched) logEvent('info', 'toggle', `${name}: no loader entry matched`)
      return found
    } catch (error) {
      // Put back what this call changed, in reverse, and say so when that
      // fails too: a rollback that reports success while the runtime is
      // somewhere else is the failure mode this exists for.
      const rollbackErrors: string[] = []
      for (const { entry, disabled, live } of previous.reverse()) {
        try {
          await updateEntry(entry, disabled ?? null, true)
          if (disabled === undefined) delete entry.options.disabled
          if ((entry.fiber !== undefined) !== live) throw new Error(`${name}: loader entry did not restore its previous live state`)
        } catch (rollbackError) {
          rollbackErrors.push(rollbackError instanceof Error ? rollbackError.message : String(rollbackError))
        }
      }
      if (rollbackErrors.length) {
        throw new Error(`${error instanceof Error ? error.message : String(error)}; entry restoration failed: ${rollbackErrors.join('; ')}`)
      }
      throw error
    }
  }

  /**
   * Make `name` the one active theme: deactivate every other installed theme
   * (market hot mounts unmount; bundle-layer entries live-disable) and bring
   * it up. The choice persists in state.json and is replayed at boot.
   */
  /**
   * Make `name` the one active theme.
   *
   * Switching stops every other theme first, because they are mutually
   * exclusive by construction. That makes the failure case the interesting
   * one: if the new theme then cannot come up, the user is left with NO theme
   * — the old one stopped, the new one never started, and the tab shows the
   * plugin as enabled while the interface has lost its skin (#582, where the
   * reported assertion was simply "the previous theme's fiber is undefined").
   *
   * So what this stopped is remembered, and a failed switch puts it back —
   * both the live fiber and the disable flag that followed it. The attempted
   * theme keeps neither: it never came up, and leaving it out of the disable
   * set is what lets the next attempt try it again.
   */
  async function activateTheme(name: string): Promise<boolean> {
    const themes = await installedThemeNames()
    const stopped: { name: string; kind: 'hot' | 'entry' }[] = []
    for (const other of themes) {
      if (other === name) continue
      if (listHotMounts().includes(other)) {
        if (await hotUnmount(other)) stopped.push({ name: other, kind: 'hot' })
        disabledThemes.add(other)
      } else if (await setEntryDisabled(other, true)) {
        stopped.push({ name: other, kind: 'entry' })
        disabledThemes.add(other)
      }
    }
    disabledThemes.delete(name)
    writeDisabled(activeProfileDir, disabledThemes)
    const live = listHotMounts().includes(name)
      || await setEntryDisabled(name, false)
      || (await hotMount(host, activeProfileDir, name)).ok
    if (live) return true
    for (const previous of stopped) {
      const back = previous.kind === 'hot'
        ? (await hotMount(host, activeProfileDir, previous.name)).ok
        : await setEntryDisabled(previous.name, false)
      if (back) disabledThemes.delete(previous.name)
      else logEvent('warn', 'theme', `${previous.name}: could not be restored after ${name} failed to start`)
    }
    writeDisabled(activeProfileDir, disabledThemes)
    return false
  }

  return { installedThemeNames, setEntryDisabled, activateTheme }
}
