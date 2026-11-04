#!/usr/bin/env node
/**
 * Expand one feature_name from a parse MVP CSV into a dependency-sorted CSV.
 *
 * Input CSV must include column `feature_name` (see rewriting_files_parse_mvp.csv).
 *
 * Usage:
 *   node build-mvp-dep-topo.mjs --csv ../rewriting_files_parse_mvp.csv --feature-name wc-01-storage-write
 *   node build-mvp-dep-topo.mjs --feature-name wc-02-document-model
 *   node build-mvp-dep-topo.mjs --csv ../rewriting_files_parse_mvp.csv --feature-name wc-01-storage-write --full-closure
 *   node build-mvp-dep-topo.mjs --feature-name wc-01-storage-write --emit-queue
 *
 * Default output:
 *   features/<feature_name>/dep_topo.csv
 *
 * Implementation: thin CLI wrapper around build-feature-dep-topo.mjs
 * (inherits closure + rust path prefixes by default — not the old 1.8k full import dump).
 *
 * Full unfiltered MVP closure (legacy):
 *   node build-feature-dep-topo.mjs --from-mvp --full-closure --out ../rewriting_files_parse_mvp_dep_topo.csv
 */

import path from 'path';
import { fileURLToPath, pathToFileURL } from 'url';
import { spawnSync } from 'child_process';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const PARSER_DIR = path.resolve(__dirname, '..');
const IMPL = path.join(__dirname, 'build-feature-dep-topo.mjs');

function printHelp() {
  console.log(`build-mvp-dep-topo.mjs — CSV + feature_name → dependency topo CSV

Required:
  --feature-name <name>   feature_name column value to expand
                          (wave0-property | wave0-xwpf | wc-01-storage-write | …)

Optional:
  --csv <path>            Input CSV (default: ../rewriting_files_parse_mvp.csv)
  --out <path>            Default: features/<name>/dep_topo.csv
  --closure inherits|imports
  --full-closure          Expand all imports (large)
  --allow-rust-prefix a,b
  --emit-queue / --queue <path>
  --pending-only
  --tags / --state / --batch
  --errors <path>
  --config <path>

Examples:
  node build-mvp-dep-topo.mjs --csv ../rewriting_files_parse_mvp.csv --feature-name wc-01-storage-write
  # → ../features/wc-01-storage-write/dep_topo.csv
  node build-mvp-dep-topo.mjs --feature-name wc-03-property-convert --emit-queue
`);
}

function main() {
  const argv = process.argv.slice(2);
  if (argv.includes('--help') || argv.includes('-h')) {
    printHelp();
    return;
  }

  // Normalize aliases: --mvp → --csv for this wrapper's docs
  const forwarded = [];
  let hasCsv = false;
  let hasFeatureName = false;
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--mvp') {
      forwarded.push('--csv', argv[++i]);
      hasCsv = true;
    } else if (a === '--csv') {
      forwarded.push(a, argv[++i]);
      hasCsv = true;
    } else if (a === '--feature-name') {
      forwarded.push(a, argv[++i]);
      hasFeatureName = true;
    } else if (a === '--seeds' && argv[i + 1] === 'mvp-must') {
      // legacy: treat as tags filter
      i++;
      forwarded.push('--tags', 'mvp-must');
    } else {
      forwarded.push(a);
    }
  }

  if (!hasFeatureName) {
    console.error('Error: --feature-name <name> is required.\n');
    printHelp();
    process.exit(1);
  }

  if (!hasCsv) {
    forwarded.unshift('--csv', path.join(PARSER_DIR, 'rewriting_files_parse_mvp.csv'));
  }

  const r = spawnSync(process.execPath, [IMPL, ...forwarded], {
    stdio: 'inherit',
    cwd: __dirname,
  });
  process.exit(r.status ?? 1);
}

main();
