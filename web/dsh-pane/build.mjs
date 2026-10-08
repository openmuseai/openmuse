import { build } from 'esbuild'
import { resolve } from 'node:path'

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
