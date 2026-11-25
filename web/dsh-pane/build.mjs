import { build } from 'esbuild'
import { resolve } from 'node:path'
import { readFile, writeFile } from 'node:fs/promises'
import { createHash } from 'node:crypto'

await build({
  entryPoints: [resolve('src/main.js')],
  outfile: resolve('../../app/openmuse_web/web/dsh-pane.js'),
  bundle: true,
  minify: true,
  format: 'esm',
  target: 'es2022',
  define: {
    'process.env.CORDIS_SHARED': 'undefined',
    'process.versions.node': '"0.0.0"',
    'process.execArgv': '[]',
  },
  alias: { 'node:module': resolve('src/node-module-stub.js') },
  loader: { '.woff2': 'dataurl', '.woff': 'dataurl', '.ttf': 'dataurl' },
})

// Runtime-loaded plugins require the APIs exported by this pinned kernel.
const panePath = resolve('../../app/openmuse_web/web/dsh-pane.js')
const pane = await readFile(panePath)
for (const api of ['observeComposition', 'modalSelector']) {
  if (!pane.includes(Buffer.from(api))) throw new Error(`DSH kernel missing ${api}; rebuild from the pinned lockfile`)
}
const revision = createHash('sha256').update(pane).digest('hex').slice(0, 16)
const loaderPath = resolve('../../app/openmuse_web/web/dsh-pane-loader.js')
const loader = await readFile(loaderPath, 'utf8')
await writeFile(loaderPath, loader.replace(/import\('\.\/dsh-pane\.js(?:\?v=[a-f0-9]+)?'\)/,
  `import('./dsh-pane.js?v=${revision}')`))
