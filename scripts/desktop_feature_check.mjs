// Run the complete Host suite with explicit, digest-pinned local product inputs.
import { readFile, readdir } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const args = process.argv.slice(2);
const option = name => { const index = args.indexOf(name); return index < 0 ? undefined : args[index + 1]; };
const configPath = option('--configuration');
const expected = option('--configuration-sha256');
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
if (!configPath || !/^[a-f0-9]{64}$/.test(expected ?? '')) throw Error('Explicit test configuration and SHA-256 required');
const bytes = await readFile(configPath);
if (hash(bytes) !== expected) throw Error('Test configuration changed');
const config = JSON.parse(bytes);
const allowed = new Set([
  'OPENMUSE_SAMPLE_DIR', 'OPENMUSE_SAMPLE_DOCX', 'OPENMUSE_SAMPLE_PDF', 'OPENMUSE_SAMPLE_XLSX_FILES',
  'PDF_PRODUCT_SAMPLE_PATH', 'PDF_PRODUCT_E2E_BUNDLE', 'PDF_PRODUCT_E2E_PUBLIC_KEY',
  'PDF_PRODUCT_E2E_KEY_ID', 'PDF_PRODUCT_E2E_VERSION', 'PDF_PRODUCT_E2E_SCREENSHOT_DIR',
  'PDF_PRODUCT_E2E_UI_FONT', 'PDF_PRODUCT_E2E_ICON_FONT',
  'WORD_STACK_BUNDLE', 'WORD_STACK_PUBLIC_KEY', 'WORD_STACK_KEY_ID', 'WORD_STACK_PLUGIN_VERSION', 'WORD_CORE_FFI',
  'WORD_CORE_ALLOW_HOST_FONTS_IN_TEST', 'WORD_CORE_FONT', 'WORD_CORE_CJK_FONT',
  'SPREADSHEET_PRODUCT_E2E_BUNDLE', 'SPREADSHEET_PRODUCT_E2E_PUBLIC_KEY',
  'SPREADSHEET_PRODUCT_E2E_KEY_ID', 'SPREADSHEET_PRODUCT_E2E_VERSION', 'OPENMUSE_GRIDCRAFT_DYLIB',
  'KSP_MAC_EDIT_DYLIB', 'KSP_PDFIUM_VIEWER_DYLIB',
]);
for (const [key, value] of Object.entries(config.environment ?? {})) {
  if (!allowed.has(key) || typeof value !== 'string') throw Error('Unexpected test environment field');
}
if (!Array.isArray(config.inputs) || config.inputs.length === 0) throw Error('Pinned input files required');
for (const input of config.inputs) {
  if (hash(await readFile(input.path)) !== input.sha256) throw Error('Test input changed: ' + input.name);
  console.log(JSON.stringify({ input: input.name, sha256: input.sha256 }));
}
const inherited = new Set(['HOME', 'PATH', 'TMPDIR', 'USER', 'LOGNAME', 'LANG', 'LC_ALL', 'SHELL']);
const environment = { ...Object.fromEntries(Object.entries(process.env).filter(([key]) => inherited.has(key))), ...config.environment };
// Run owner/bridge contracts as well as the complete configured Host suite.
const repository = fileURLToPath(new URL('../', import.meta.url));
const bridgeTests = (await readdir(new URL('../third_party/dsh/plugins/openmuse-dsh-bridge/test/', import.meta.url)))
  .filter(name => name.endsWith('.test.js')).map(name => 'third_party/dsh/plugins/openmuse-dsh-bridge/test/' + name);
const contracts = spawn(process.execPath, ['--test', 'scripts/test_dsh_embedded_layout.mjs', ...bridgeTests], {
  cwd: repository, env: environment, stdio: 'inherit',
});
const contractCode = await new Promise((resolve, reject) => {
  contracts.on('error', reject); contracts.on('exit', resolve);
});
if (contractCode !== 0) process.exit(contractCode ?? 1);
const child = spawn(option('--flutter') ?? 'flutter', ['test', '--no-pub', '--reporter', 'expanded'], {
  cwd: fileURLToPath(new URL('../app/openmuse_host/', import.meta.url)), env: environment, stdio: 'inherit',
});
child.on('error', () => { console.error('Unable to launch Flutter check'); process.exitCode = 1; });
child.on('exit', code => { process.exitCode = code ?? 1; });
