/**
 * E2E fixture: proves its own liveness, INCLUDING its death.
 *
 * The market's `activation[name].state` is an inference drawn from the
 * profile's bundle list and patch layers — the exact reasoning that was
 * wrong in #103, #135 and #147 — so a spec that asserts on it checks the
 * market against itself. This marker is ground truth instead.
 *
 * It is written from inside the webServer injection, so its presence means
 * cordis resolved the package, loaded this module, ran `apply()` AND
 * satisfied the injection. It is removed on dispose, so it also goes false
 * when the plugin is unloaded — an HTTP route cannot do that job: routes
 * registered here outlive the plugin's disposal, and a probe built on one
 * reports a disabled plugin as still alive.
 */
import { writeFileSync, rmSync, readFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

export const name = 'dshm-e2e-fixture-a'

/**
 * The version of the build that is actually RUNNING, read from the copy of
 * package.json this module was loaded beside.
 *
 * Written into the marker so an update can be observed from outside: after
 * the market replaces this package on disk under a running host, a marker
 * still naming the old version proves the old module is what the process is
 * serving, and one naming the new version proves the host reloaded (#491).
 * `activation[name].state` is the market's own inference and cannot settle
 * that question about itself.
 */
const version = JSON.parse(
  readFileSync(join(fileURLToPath(new URL('.', import.meta.url)), 'package.json'), 'utf8'),
).version

const marker = () => join(process.env.DSH_HOME ?? '.', `e2e-${name}.alive`)


/**
 * How many entries are currently mounting this module.
 *
 * ONE module can be mounted by MORE THAN ONE loader entry: a carrier bundle
 * (#156) inserts an entry naming another package, so a fixture is named twice
 * — by its own bundle's row and by the carrier's. Both resolve to the same
 * module URL, so the module instance is shared here and `apply` runs once per
 * entry.
 *
 * A plain write-on-apply / delete-on-dispose marker cannot survive that: one
 * entry's dispose deletes the file while the other entry is still running the
 * plugin, and the next spec reads "dead" about something the host reports as
 * Running (measured on dsh 0.1.7 — the dispose ordering there differed from
 * 0.1.2-alpha.2, which is all issue #717 was). Counting mounts makes the file
 * mean what every spec reads it as: at least one live entry.
 */
let mounts = 0
export function apply(ctx) {
  ctx.inject(['webServer'], (host) => {
    // ctx.effect is how this host models a disposable side effect: the
    // returned function runs when the plugin is unloaded.
    host.effect(() => {
      mounts += 1
      writeFileSync(marker(), version)
      return () => {
        mounts -= 1
        // Only the LAST mount leaving means the plugin is gone; a file that
        // vanishes while another entry still mounts this module says nothing
        // about liveness.
        if (mounts <= 0) rmSync(marker(), { force: true })
      }
    }, `${name}: e2e liveness marker`)
  })
}
