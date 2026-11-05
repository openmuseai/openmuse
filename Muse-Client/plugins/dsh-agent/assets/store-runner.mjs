// Fixed App Store DSH composition. User profiles and home patch files are
// deliberately never loaded; conversation/model data still uses DSH_HOME.
import { mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { boot, initProfile, loadProfileDirectory, loadOverlayPatches } from '@deepseek-ai/dsh-app-boot';
import { provideCmdline } from '@deepseek-ai/dsh-cmdline';
import { createLaunchEnvironmentSnapshot, DSH_LAUNCH_ENVIRONMENT_KEY }
  from '@deepseek-ai/dsh-launch-environment';

// The pinned Node supports execve. Use its built-in internal module loader
// instead of mapping the optional loader addon out of a writable cache.
// execve preserves PID, stdio, parentage and the inherited macOS sandbox.
if (!process.execArgv.includes('--expose-internals')) {
  process.execve(process.execPath, [process.execPath, '--expose-internals', ...process.argv.slice(1)], process.env);
}

const anchor = fileURLToPath(new URL('./@deepseek-ai/dsh/package.json', import.meta.url));
const scratch = mkdtempSync(join(tmpdir(), 'openmuse-store-dsh-'));
const forbidden = /@deepseek-ai\/dsh-(?:otel|session-telemetry-otel|host-product-telemetry-otel|client-product-analytics|plugin-manager|hmr|config-editor|tool-cordis|cordis-host-runner|tool-bash|tool-pwsh|terminal|api-terminal|ptc-|workflow-ptc|tool-workflow|host-open-in-app|client-ui-(?:cordis|plugin-manager|settings-plugins|sidebar-terminal|open-in-app))/;
function restrict(value) {
  if (Array.isArray(value)) return value.map(restrict);
  if (!value || typeof value !== 'object') return value;
  const result = Object.fromEntries(Object.entries(value).map(([k,v]) => [k,restrict(v)]));
  if (typeof result.name === 'string') {
    if (forbidden.test(result.name)) result.disabled = true;
    // Resolve the signed, closed package set here. No writable profile
    // resolver or native Node loader interception is needed.
    if (result.name.startsWith('@')) result.name = import.meta.resolve(result.name);
  }
  return result;
}
let ctx;
async function stop(code) {
  const timer = setTimeout(() => process.exit(code), 5000);
  timer.unref();
  try { await ctx?.fiber.dispose(); } finally {
    rmSync(scratch, {recursive:true, force:true}); process.exit(code);
  }
}
const parent = process.ppid;
const parentWatch = setInterval(() => { if (process.ppid === 1 || process.ppid !== parent) void stop(0); }, 1000);
parentWatch.unref();
process.on('SIGTERM', () => void stop(0));
process.on('SIGINT', () => void stop(0));
try {
  initProfile(scratch, ['@deepseek-ai/dsh-base', '@deepseek-ai/dsh-web-app']);
  const profile = loadProfileDirectory('OpenMuse Store', scratch, anchor, {userLayer:false});
  if (profile.skippedBundles.length) throw new Error(JSON.stringify(profile.skippedBundles));
  const overlays = [];
  for (let i=2; i<process.argv.length; i+=2) {
    if (process.argv[i] !== '--patch' || !process.argv[i+1]) throw new Error('Invalid Store launcher argument');
    overlays.push(...loadOverlayPatches('OpenMuse Store', process.argv[i+1]));
  }
  const patches = restrict([...profile.layers.flatMap(layer=>layer.patches), ...overlays]);
  if (process.env.OPENMUSE_STORE_DUMP === '1') {
    console.log(JSON.stringify(patches)); rmSync(scratch,{recursive:true,force:true});
  } else {
    const root = join(scratch,'cordis.yml'); writeFileSync(root,'[]\n');
    const listeners = new Set(); let ready = false;
    ctx = await boot('OpenMuse Store', root, patches, async host => {
      host.provide(DSH_LAUNCH_ENVIRONMENT_KEY, createLaunchEnvironmentSnapshot([{source:'process',values:process.env}]));
      // No profileContext: package management, HMR and configuration editing
      // cannot obtain a writable runtime composition.
      provideCmdline(host,{args:['--host','127.0.0.1','--port','0','--no-open'],exit:code=>void stop(code),
        ready:{onReady(fn){if(ready)fn();else listeners.add(fn);return()=>listeners.delete(fn);}}});
    });
    ready = true; for (const fn of listeners) fn(); listeners.clear();
  }
} catch(error) { console.error(error); await stop(1); }
