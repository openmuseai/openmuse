#!/usr/bin/env node
/**
 * Feature-scoped dependency topo for progressive Java→Rust rewrite (word-core parse).
 *
 * Workflow:
 *   1. List the *necessary core files* for one small feature in
 *      libs/word-core/parser/features/<feature-id>/seeds.csv
 *   2. Run this script → expands project deps (import/extends/implements + CSV depends)
 *      and Kahn-topo sorts them (inherits + CSV depends as hard edges).
 *   3. Rewrite in `order` ascending; re-run after each slice as seeds grow.
 *
 * Usage:
 *   node build-feature-dep-topo.mjs --feature wc-01-storage-write
 *   node build-feature-dep-topo.mjs --seeds-file ../features/wc-01-storage-write/seeds.csv
 *   node build-feature-dep-topo.mjs --from-mvp --batch parse-01-storage --tags mvp-must
 *   node build-feature-dep-topo.mjs --feature wc-01-storage-write --emit-queue
 *
 * Defaults:
 *   config = libs/word-core/rewrite_config.json
 *   out    = features/<id>/dep_topo.csv  (or beside seeds-file)
 */

import fs from 'fs';
import path from 'path';
import { createRequire } from 'module';
import { fileURLToPath } from 'url';

const require = createRequire(import.meta.url);
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const WORKSPACE = path.resolve(__dirname, '../../../..');
const PARSER_DIR = path.resolve(__dirname, '..');
const FEATURES_DIR = path.join(PARSER_DIR, 'features');
const WORD_CORE = path.resolve(__dirname, '../..');

const {
  initConfig,
  getConfig,
  javaAbsToRustAbs,
} = require(path.join(WORKSPACE, 'plugin_java2Rust/out/core/config.js'));
const {
  findJavaQualifiedInLibs,
  discoverJavaLibs,
} = require(path.join(WORKSPACE, 'plugin_java2Rust/out/codefile/java/javaCodeLib.js'));

const SKIP_PREFIXES = [
  'java.',
  'javax.',
  'jakarta.',
  'sun.',
  'jdk.',
  'org.w3c.',
  'org.xml.',
  'org.ietf.',
  'com.sun.',
  'android.',
  'kotlin.',
  'scala.',
];

function parseArgs(argv) {
  const out = {
    feature: '',
    featureName: '', // filter input CSV by feature_name column
    seedsFile: '',
    fromMvp: false,
    mvpCsv: path.join(PARSER_DIR, 'rewriting_files_parse_mvp.csv'),
    csv: '', // alias input csv (--csv); if set with --feature-name, filter that file
    batch: '',
    tags: '', // semicolon-separated, all must match (AND)
    state: '', // exact, or !pending / !=done style via prefix !
    outPath: '',
    errCsv: path.join(PARSER_DIR, 'typeoracle_missing_and_errors.csv'),
    config: path.join(WORD_CORE, 'rewrite_config.json'),
    emitQueue: false,
    queuePath: path.join(WORD_CORE, 'rewriting_files_parse_slice.csv'),
    includeDoneSeeds: true,
    // imports = full compile-oriented closure (large)
    // inherits = extends/implements + CSV depends only (minimal hierarchy)
    closure: 'inherits',
    // comma-separated rust_path prefixes; empty = no filter
    allowRustPrefix: '',
  };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--feature') out.feature = argv[++i];
    else if (a === '--feature-name') out.featureName = argv[++i];
    else if (a === '--seeds-file') out.seedsFile = path.resolve(argv[++i]);
    else if (a === '--from-mvp') out.fromMvp = true;
    else if (a === '--mvp') out.mvpCsv = path.resolve(argv[++i]);
    else if (a === '--csv') out.csv = path.resolve(argv[++i]);
    else if (a === '--batch') out.batch = argv[++i];
    else if (a === '--tags') out.tags = argv[++i];
    else if (a === '--state') out.state = argv[++i];
    else if (a === '--out') out.outPath = path.resolve(argv[++i]);
    else if (a === '--errors') out.errCsv = path.resolve(argv[++i]);
    else if (a === '--config') out.config = path.resolve(argv[++i]);
    else if (a === '--emit-queue') out.emitQueue = true;
    else if (a === '--queue') {
      out.emitQueue = true;
      out.queuePath = path.resolve(argv[++i]);
    } else if (a === '--pending-only') out.includeDoneSeeds = false;
    else if (a === '--closure') out.closure = argv[++i];
    else if (a === '--allow-rust-prefix') out.allowRustPrefix = argv[++i];
    else if (a === '--full-closure') {
      out.closure = 'imports';
      out.allowRustPrefix = '';
    } else if (a === '--help' || a === '-h') out.help = true;
  }

  // --csv implies reading that file as seed source (with optional --feature-name filter)
  if (out.csv) {
    out.mvpCsv = out.csv;
    out.fromMvp = true;
  }
  if (out.featureName && !out.fromMvp && !out.feature && !out.seedsFile) {
    out.fromMvp = true;
  }

  // Progressive default prefixes when using feature / feature-name and user did not override
  const useTightPrefix =
    (out.feature || out.featureName) &&
    out.allowRustPrefix === '' &&
    out.closure === 'inherits';
  if (useTightPrefix) {
    const name = out.featureName || out.feature;
    if (name === 'wave0-property') {
      out.allowRustPrefix = 'libs/core-libs/util/';
    } else if (name === 'wave0-xwpf') {
      out.allowRustPrefix = [
        'libs/core-libs/util/',
        'libs/core-libs/io/wps-io/',
      ].join(',');
    } else {
      out.allowRustPrefix = [
        'libs/word-core/',
        'libs/core-libs/util/',
        'libs/core-libs/io/wps-io/',
      ].join(',');
    }
  }
  return out;
}

function printHelp() {
  console.log(`build-feature-dep-topo.mjs — seed list → dep closure → topo CSV

Required (one of):
  --csv <path> --feature-name <name>
                          Filter CSV rows by feature_name, expand deps
  --feature <id>          Use features/<id>/seeds.csv
  --seeds-file <path>     Explicit seeds CSV
  --from-mvp              Filter rewriting_files_parse_mvp.csv

Optional filters (CSV / --from-mvp):
  --feature-name <name>   Keep rows whose feature_name equals <name>
  --batch <batch_id>      e.g. parse-01-storage
  --tags <a;b>            All tags must be present (AND)
  --state <s>             Exact state, or !done / !=done

Closure control (important for small slices):
  --closure inherits      DEFAULT: only extends/implements + CSV depends (small)
  --closure imports       Also BFS through every import (large, ~1k+)
  --allow-rust-prefix a,b Only keep nodes whose rust_path starts with a prefix
  --full-closure          Shortcut: --closure imports and no prefix filter

Output:
  --out <path>            Default for --feature-name:
                            features/<name>/dep_topo.csv
  --errors <path>         Optional typeoracle join (skip if missing)
  --emit-queue            Also write rewriting queue CSV for word-core
  --queue <path>          Queue path (implies --emit-queue)
  --pending-only          Drop seeds with state=done from seed set
  --config <path>         rewrite_config.json

Examples:
  node build-mvp-dep-topo.mjs --csv ../rewriting_files_parse_mvp.csv --feature-name wc-01-storage-write
  node build-feature-dep-topo.mjs --csv ../rewriting_files_parse_mvp.csv --feature-name wc-02-document-model
  node build-feature-dep-topo.mjs --feature wc-01-storage-write --emit-queue
`);
}

function splitCsvLine(line) {
  const cols = [];
  let cur = '';
  let inQ = false;
  for (let i = 0; i < line.length; i++) {
    const c = line[i];
    if (inQ) {
      if (c === '"' && line[i + 1] === '"') {
        cur += '"';
        i++;
      } else if (c === '"') inQ = false;
      else cur += c;
    } else if (c === '"') inQ = true;
    else if (c === ',') {
      cols.push(cur);
      cur = '';
    } else cur += c;
  }
  cols.push(cur);
  return cols;
}

function parseCsv(text) {
  const lines = text.replace(/^\uFEFF/, '').split(/\r?\n/).filter((l) => l.length > 0);
  const header = splitCsvLine(lines[0]);
  const rows = [];
  for (let i = 1; i < lines.length; i++) {
    const cols = splitCsvLine(lines[i]);
    const obj = {};
    header.forEach((h, idx) => {
      obj[h] = cols[idx] ?? '';
    });
    rows.push(obj);
  }
  return { header, rows };
}

function csvEscape(v) {
  const s = v == null ? '' : String(v);
  if (/[",\n\r]/.test(s)) return `"${s.replace(/"/g, '""')}"`;
  return s;
}

function writeCsv(filePath, header, rows) {
  const lines = [header.join(',')];
  for (const row of rows) {
    lines.push(header.map((h) => csvEscape(row[h])).join(','));
  }
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, lines.join('\n') + '\n', 'utf-8');
}

function relWs(abs) {
  return path.relative(WORKSPACE, abs).replace(/\\/g, '/');
}

function normRel(p) {
  return (p || '').replace(/\\/g, '/').replace(/^\.\//, '');
}

function isSkippedImport(qn) {
  return SKIP_PREFIXES.some((p) => qn === p.slice(0, -1) || qn.startsWith(p));
}

function stripJavaNoise(src) {
  return src
    .replace(/\/\*[\s\S]*?\*\//g, ' ')
    .replace(/\/\/[^\n]*/g, ' ');
}

function collectTypeRefs(javaAbs) {
  const raw = fs.readFileSync(javaAbs, 'utf-8');
  const src = stripJavaNoise(raw);
  const imports = [];
  const inherits = [];

  for (const m of src.matchAll(/\bimport\s+(?:static\s+)?([a-zA-Z_][\w.]*)\s*;/g)) {
    let qn = m[1];
    if (qn.endsWith('.*')) continue;
    if (m[0].includes('static')) {
      const parts = qn.split('.');
      if (parts.length >= 2 && /^[a-z]/.test(parts[parts.length - 1])) {
        qn = parts.slice(0, -1).join('.');
      }
    }
    if (!isSkippedImport(qn)) imports.push(qn);
  }

  const typeDecl = src.match(
    /\b(?:public\s+|protected\s+|private\s+|abstract\s+|final\s+|strictfp\s+|sealed\s+|non-sealed\s+)*(?:class|interface|enum|record)\s+\w+[^{]*\{/,
  );
  const header = typeDecl ? typeDecl[0] : src.slice(0, 4000);

  const ext = header.match(/\bextends\s+([^{]+?)(?:\bimplements\b|\{)/);
  if (ext) {
    for (const t of splitTypeList(ext[1])) inherits.push(t);
  }
  const impl = header.match(/\bimplements\s+([^{]+?)\{/);
  if (impl) {
    for (const t of splitTypeList(impl[1])) inherits.push(t);
  }
  const ifaceExt = header.match(/\binterface\s+\w+[^{]*?\bextends\s+([^{]+?)\{/);
  if (ifaceExt && !ext) {
    for (const t of splitTypeList(ifaceExt[1])) inherits.push(t);
  }

  return { imports, inherits };
}

function splitTypeList(s) {
  return s
    .split(',')
    .map((x) => x.trim())
    .map((x) => x.replace(/<.*>/g, '').trim())
    .map((x) => x.replace(/\.\.\.$/, '').trim())
    .filter((x) => x && /^[A-Za-z_][\w.]*$/.test(x));
}

function resolveTypeToJava(ref, cfg, fromJavaAbs, importQns, classIndex) {
  if (ref.includes('.')) {
    let hit = findJavaQualifiedInLibs(ref, cfg);
    if (hit) return hit.filePath;
    const parts = ref.split('.');
    for (let n = parts.length - 1; n >= 2; n--) {
      const q = parts.slice(0, n).join('.');
      hit = findJavaQualifiedInLibs(q, cfg);
      if (hit) return hit.filePath;
    }
    return null;
  }

  if (classIndex?.has(ref)) {
    const list = classIndex.get(ref);
    if (list.length === 1) return list[0];
  }

  const pkgMatch = fs.readFileSync(fromJavaAbs, 'utf-8').match(/^\s*package\s+([\w.]+)\s*;/m);
  if (pkgMatch) {
    const qn = `${pkgMatch[1]}.${ref}`;
    const hit = findJavaQualifiedInLibs(qn, cfg);
    if (hit) return hit.filePath;
    const sibling = path.join(path.dirname(fromJavaAbs), `${ref}.java`);
    if (fs.existsSync(sibling)) return sibling;
  }

  if (importQns) {
    for (const qn of importQns) {
      if (qn.endsWith('.' + ref) || qn === ref) {
        const hit = findJavaQualifiedInLibs(qn, cfg);
        if (hit) return hit.filePath;
      }
    }
  }

  return null;
}

function collectImportQns(javaAbs) {
  const src = stripJavaNoise(fs.readFileSync(javaAbs, 'utf-8'));
  const qns = [];
  for (const m of src.matchAll(/\bimport\s+(?:static\s+)?([a-zA-Z_][\w.]*)\s*;/g)) {
    let qn = m[1];
    if (qn.endsWith('.*')) continue;
    if (m[0].includes('static')) {
      const parts = qn.split('.');
      if (parts.length >= 2 && /^[a-z]/.test(parts[parts.length - 1])) {
        qn = parts.slice(0, -1).join('.');
      }
    }
    if (!isSkippedImport(qn)) qns.push(qn);
  }
  return qns;
}

function underProjectJava(javaAbs, cfg) {
  const n = javaAbs.replace(/\\/g, '/').toLowerCase();
  const main = cfg.javaBase.replace(/\\/g, '/').toLowerCase();
  const dep = cfg.javaDepBase.replace(/\\/g, '/').toLowerCase();
  return n.startsWith(main + '/') || n === main || n.startsWith(dep + '/') || n === dep;
}

function typeoracleBeside(rustRel) {
  const abs = path.join(WORKSPACE, rustRel);
  const dir = path.dirname(abs);
  const stem = path.basename(abs, '.rs');
  const p = path.join(dir, '.codeTrees', `${stem}.typeoracle.rs`);
  if (fs.existsSync(p)) return relWs(p);
  return '';
}

function matchState(actual, want) {
  if (!want) return true;
  if (want.startsWith('!') || want.startsWith('!=')) {
    const neg = want.replace(/^!=?/, '');
    return actual !== neg;
  }
  return actual === want;
}

function matchTags(rowTags, want) {
  if (!want) return true;
  const have = new Set((rowTags || '').split(';').map((s) => s.trim()).filter(Boolean));
  return want
    .split(';')
    .map((s) => s.trim())
    .filter(Boolean)
    .every((t) => have.has(t));
}

function resolveSeeds(args) {
  if (args.feature) {
    const seedsFile = path.join(FEATURES_DIR, args.feature, 'seeds.csv');
    if (!fs.existsSync(seedsFile)) {
      throw new Error(`feature seeds not found: ${seedsFile}`);
    }
    args.seedsFile = seedsFile;
    if (!args.outPath) args.outPath = path.join(FEATURES_DIR, args.feature, 'dep_topo.csv');
  }

  if (args.fromMvp || args.featureName || args.csv) {
    const mvpPath = args.mvpCsv;
    if (!fs.existsSync(mvpPath)) {
      throw new Error(`CSV not found: ${mvpPath}`);
    }
    const mvp = parseCsv(fs.readFileSync(mvpPath, 'utf-8'));
    let rows = mvp.rows;
    if (args.featureName) {
      rows = rows.filter((r) => (r.feature_name || '') === args.featureName);
    }
    if (args.batch) rows = rows.filter((r) => r.batch_id === args.batch);
    if (args.tags) rows = rows.filter((r) => matchTags(r.tags, args.tags));
    if (args.state) rows = rows.filter((r) => matchState(r.state, args.state));
    if (!args.includeDoneSeeds) rows = rows.filter((r) => r.state !== 'done');
    if (!args.outPath) {
      if (args.featureName) {
        const featDir = path.join(FEATURES_DIR, args.featureName);
        args.outPath = path.join(featDir, 'dep_topo.csv');
      } else {
        const parts = ['from-mvp'];
        if (args.batch) parts.push(args.batch);
        if (args.tags) parts.push(args.tags.replace(/;/g, '_'));
        args.outPath = path.join(FEATURES_DIR, '_generated', `${parts.join('__')}.dep_topo.csv`);
      }
    }
    return rows;
  }

  if (!args.seedsFile) {
    throw new Error('Need --csv/--feature-name, --feature, --seeds-file, or --from-mvp (see --help)');
  }
  if (!args.outPath) {
    args.outPath = path.join(path.dirname(args.seedsFile), 'dep_topo.csv');
  }
  let rows = parseCsv(fs.readFileSync(args.seedsFile, 'utf-8')).rows;
  if (args.featureName) {
    rows = rows.filter((r) => (r.feature_name || '') === args.featureName);
  }
  if (!args.includeDoneSeeds) rows = rows.filter((r) => r.state !== 'done');
  return rows;
}

function main() {
  const args = parseArgs(process.argv.slice(2));
  if (args.help) {
    printHelp();
    return;
  }

  if (!fs.existsSync(path.join(WORKSPACE, 'plugin_java2Rust/out/core/config.js'))) {
    throw new Error('Build plugin_java2Rust first (missing out/core/config.js)');
  }

  initConfig(args.config);
  const cfg = getConfig();
  discoverJavaLibs(cfg);

  const seedRows = resolveSeeds(args);
  if (!seedRows.length) {
    throw new Error('No seed rows after filters');
  }

  /** @type {Map<string, {typeoracle_rs_path:string, compile_error_count:string}>} */
  const errByRust = new Map();
  if (args.errCsv && fs.existsSync(args.errCsv)) {
    const errCsv = parseCsv(fs.readFileSync(args.errCsv, 'utf-8'));
    for (const r of errCsv.rows) {
      errByRust.set(normRel(r.rust_path), {
        typeoracle_rs_path: r.typeoracle_rs_path || '',
        compile_error_count: r.compile_error_count ?? '',
      });
    }
  }

  const nodes = new Map();
  const depsOf = new Map();

  const allowPrefixes = (args.allowRustPrefix || '')
    .split(',')
    .map((s) => s.trim())
    .filter(Boolean);

  function rustAllowed(rustRel) {
    if (!allowPrefixes.length) return true;
    return allowPrefixes.some((p) => rustRel.startsWith(p));
  }

  function ensureNode(javaAbs, mvpRow) {
    if (!underProjectJava(javaAbs, cfg)) return null;
    const javaRel = relWs(javaAbs);
    if (nodes.has(javaRel)) {
      if (mvpRow && !nodes.get(javaRel).seed) {
        const n = nodes.get(javaRel);
        n.seed = true;
        n.mvp = mvpRow;
        if (mvpRow.rust_path) n.rustRel = normRel(mvpRow.rust_path);
        if (mvpRow.class_name) n.className = mvpRow.class_name;
      }
      return javaRel;
    }
    let rustRel;
    if (mvpRow?.rust_path) {
      rustRel = normRel(mvpRow.rust_path);
    } else {
      rustRel = relWs(javaAbsToRustAbs(javaAbs));
    }
    // Seeds always kept; discovered deps filtered by rust prefix
    if (!mvpRow && !rustAllowed(rustRel)) return null;
    const className = mvpRow?.class_name || path.basename(javaAbs, '.java');
    nodes.set(javaRel, {
      javaAbs,
      javaRel,
      rustRel,
      className,
      seed: !!mvpRow,
      mvp: mvpRow,
    });
    depsOf.set(javaRel, new Set());
    return javaRel;
  }

  const classIndex = new Map();
  function indexClass(className, javaAbs) {
    if (!classIndex.has(className)) classIndex.set(className, []);
    const list = classIndex.get(className);
    if (!list.includes(javaAbs)) list.push(javaAbs);
  }

  for (const row of seedRows) {
    const javaAbs = path.join(WORKSPACE, normRel(row.file_path));
    if (!fs.existsSync(javaAbs)) {
      console.warn(`seed missing: ${row.file_path}`);
      continue;
    }
    ensureNode(javaAbs, row);
    indexClass(row.class_name || path.basename(javaAbs, '.java'), javaAbs);
  }

  const queue = [...nodes.keys()];
  const seenExpand = new Set(queue);
  while (queue.length) {
    const javaRel = queue.shift();
    const node = nodes.get(javaRel);
    if (!node || !fs.existsSync(node.javaAbs)) continue;

    const importQns = collectImportQns(node.javaAbs);
    const { imports, inherits } = collectTypeRefs(node.javaAbs);
    const csvDepends = node.mvp?.depends
      ? node.mvp.depends.split(';').map((s) => s.trim()).filter(Boolean)
      : [];

    const hardRefs = [...new Set([...inherits, ...csvDepends])];
    // inherits: hierarchy-only expansion; imports: full import BFS
    const discoverRefs =
      args.closure === 'imports'
        ? [...new Set([...imports, ...inherits, ...csvDepends])]
        : [...new Set([...inherits, ...csvDepends])];

    for (const ref of discoverRefs) {
      const depAbs = resolveTypeToJava(ref, cfg, node.javaAbs, importQns, classIndex);
      if (!depAbs) continue;
      if (!underProjectJava(depAbs, cfg)) continue;
      if (path.resolve(depAbs) === path.resolve(node.javaAbs)) continue;
      const depRel = ensureNode(depAbs, null);
      if (!depRel) continue;
      indexClass(nodes.get(depRel).className, depAbs);
      // Hard topo edges always inherits + csv depends when the ref is hard
      if (hardRefs.includes(ref)) depsOf.get(javaRel).add(depRel);
      if (!seenExpand.has(depRel)) {
        seenExpand.add(depRel);
        queue.push(depRel);
      }
    }
  }

  function cmpNode(a, b) {
    const na = nodes.get(a);
    const nb = nodes.get(b);
    const oa = na.mvp?.order ? Number(na.mvp.order) : 1e9;
    const ob = nb.mvp?.order ? Number(nb.mvp.order) : 1e9;
    if (oa !== ob) return oa - ob;
    return a.localeCompare(b);
  }

  const indeg = new Map();
  const dependents = new Map();
  for (const id of nodes.keys()) {
    indeg.set(id, 0);
    dependents.set(id, []);
  }
  for (const [id, deps] of depsOf) {
    for (const d of deps) {
      if (!nodes.has(d)) continue;
      indeg.set(id, (indeg.get(id) || 0) + 1);
      dependents.get(d).push(id);
    }
  }

  const ready = [...nodes.keys()]
    .filter((id) => (indeg.get(id) || 0) === 0)
    .sort(cmpNode);
  const ordered = [];
  const remaining = new Set(nodes.keys());

  while (ready.length) {
    const id = ready.shift();
    ordered.push(id);
    remaining.delete(id);
    for (const nid of dependents.get(id) || []) {
      indeg.set(nid, (indeg.get(nid) || 0) - 1);
      if (indeg.get(nid) === 0 && remaining.has(nid)) {
        ready.push(nid);
        ready.sort(cmpNode);
      }
    }
  }

  const cycleNodes = [...remaining].sort(cmpNode);
  for (const id of cycleNodes) ordered.push(id);

  const header = [
    'order',
    'file_path',
    'class_name',
    'rust_path',
    'depends',
    'typeoracle_rs_path',
    'compile_error_count',
    'rewrite_status',
    'rs_exists',
    'is_seed',
    'mvp_state',
    'mvp_order',
    'batch_id',
    'feature_name',
    'tags',
    'in_cycle',
    'crate',
  ];

  const outRows = [];
  let seedCount = 0;
  let missingRs = 0;
  let wordCoreNodes = 0;
  let coreLibsNodes = 0;

  for (let i = 0; i < ordered.length; i++) {
    const id = ordered[i];
    const n = nodes.get(id);
    const deps = [...(depsOf.get(id) || [])]
      .map((d) => nodes.get(d)?.className || path.basename(d, '.java'))
      .sort()
      .join(';');
    const rustAbs = path.join(WORKSPACE, n.rustRel);
    const rsExists = fs.existsSync(rustAbs);
    if (!rsExists) missingRs++;
    if (n.seed) seedCount++;

    const crate = n.rustRel.startsWith('libs/word-core/')
      ? 'word-core'
      : n.rustRel.startsWith('libs/core-libs/')
        ? 'core-libs'
        : 'other';
    if (crate === 'word-core') wordCoreNodes++;
    if (crate === 'core-libs') coreLibsNodes++;

    const err = errByRust.get(n.rustRel) || {};
    let torPath = err.typeoracle_rs_path || '';
    if (!torPath) torPath = typeoracleBeside(n.rustRel);

    let rewriteStatus;
    if (!rsExists) rewriteStatus = '未改写';
    else if (n.mvp?.state) rewriteStatus = n.mvp.state;
    else rewriteStatus = '已生成';

    outRows.push({
      order: String(i + 1),
      file_path: n.javaRel,
      class_name: n.className,
      rust_path: n.rustRel,
      depends: deps,
      typeoracle_rs_path: torPath,
      compile_error_count: err.compile_error_count ?? '',
      rewrite_status: rewriteStatus,
      rs_exists: rsExists ? 'yes' : 'no',
      is_seed: n.seed ? 'yes' : 'no',
      mvp_state: n.mvp?.state || '',
      mvp_order: n.mvp?.order || '',
      batch_id: n.mvp?.batch_id || '',
      feature_name: n.mvp?.feature_name || args.featureName || args.feature || '',
      tags: n.mvp?.tags || '',
      in_cycle: cycleNodes.includes(id) ? 'yes' : 'no',
      crate,
    });
  }

  writeCsv(args.outPath, header, outRows);

  if (args.emitQueue) {
    // Pipeline-compatible queue: only rows still needing work (no rs or not done)
    const qHeader = [
      'order',
      'file_path',
      'class_name',
      'state',
      'review',
      'lib_name',
      'batch_id',
      'rust_path',
      'depends',
      'session_id',
      'tags',
    ];
    const qRows = outRows
      .filter((r) => r.rs_exists === 'no' || (r.mvp_state && r.mvp_state !== 'done'))
      .map((r, idx) => ({
        order: String(idx + 1),
        file_path: r.file_path,
        class_name: r.class_name,
        state: r.mvp_state && r.mvp_state !== 'done' ? r.mvp_state : 'pending',
        review: '',
        lib_name: r.crate === 'word-core' ? 'main' : r.crate,
        batch_id: r.batch_id || args.feature || 'parse-slice',
        rust_path: r.rust_path,
        depends: r.depends,
        session_id: '',
        tags: r.tags || (r.is_seed === 'yes' ? 'feature-seed' : 'feature-dep'),
      }));
    writeCsv(args.queuePath, qHeader, qRows);
  }

  console.log(
    JSON.stringify(
      {
        feature: args.feature || null,
        feature_name: args.featureName || null,
        seeds_file: args.seedsFile ? relWs(args.seedsFile) : null,
        csv: args.csv || args.fromMvp ? relWs(args.mvpCsv) : null,
        out: relWs(args.outPath),
        queue: args.emitQueue ? relWs(args.queuePath) : null,
        seed_rows: seedRows.length,
        nodes: ordered.length,
        seeds_in_graph: seedCount,
        word_core_nodes: wordCoreNodes,
        core_libs_nodes: coreLibsNodes,
        missing_rs: missingRs,
        cycle_nodes: cycleNodes.length,
        closure: args.closure,
        allow_rust_prefix: allowPrefixes,
        pending_or_missing: outRows.filter(
          (r) => r.rs_exists === 'no' || (r.mvp_state && r.mvp_state !== 'done'),
        ).length,
      },
      null,
      2,
    ),
  );
}

try {
  main();
} catch (e) {
  console.error(String(e && e.stack ? e.stack : e));
  process.exit(1);
}
