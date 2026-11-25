// Local process-cold / process-warm baseline, not Mobile or network acceptance.
import { writeFile, readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { platform, arch } from 'node:os';
import assert from 'node:assert/strict';
import { CodeNavigationService } from '../lib/service.js';
import { stdioBackendFactory } from '../lib/stdio.js';
import { fixture } from '../test/support/owner.js';

const cleanup = [], f = await fixture({ after: action => cleanup.push(action) });
const backendFactory = stdioBackendFactory({ typescript: {
  command: process.execPath,
  args: [fileURLToPath(new URL('../node_modules/typescript-language-server/lib/cli.mjs', import.meta.url)), '--stdio'],
  initializationOptions: { disableAutomaticTypingAcquisition: true, tsserver: {
    useSyntaxServer: 'never', path: fileURLToPath(new URL('../node_modules/typescript/lib/tsserver.js', import.meta.url)),
  } },
} });
const cold = [], warm = [];
let service;
try {
  for (let i = 0; i < 30; i++) {
    service = new CodeNavigationService({ owner: f.owner, backendFactory,
      trace: e => { if (e.stage === 'total') cold.push(e.elapsedMs); } });
    service.enable();
    const result = await service.query(f.input);
    assert.equal(result.matches[0]?.label, '定义.ts');
    await service.disable(); assert.equal(service.status.instances, 0);
  }
  let warmed = false;
  service = new CodeNavigationService({ owner: f.owner, backendFactory,
    trace: e => { if (warmed && e.stage === 'total') warm.push(e.elapsedMs); } });
  service.enable(); await service.query(f.input); warmed = true;
  for (let i = 0; i < 30; i++) {
    const result = await service.query(f.input);
    assert.equal(result.matches[0]?.label, '定义.ts');
  }
  await service.disable(); assert.equal(service.status.instances, 0);
  const stats = values => {
    const sorted = [...values].sort((a, b) => a - b);
    return { samples: values.length, p50Ms: sorted[Math.ceil(sorted.length * .50) - 1],
      p95Ms: sorted[Math.ceil(sorted.length * .95) - 1], maxMs: sorted.at(-1), valuesMs: values };
  };
  const result = { date: '2026-11-15', timezone: 'Asia/Shanghai', platform: platform(), arch: arch(), node: process.version,
    serviceSourceSha256: createHash('sha256').update(await readFile(new URL('../lib/service.js', import.meta.url))).digest('hex'),
    stdioSourceSha256: createHash('sha256').update(await readFile(new URL('../lib/stdio.js', import.meta.url))).digest('hex'),
    languageServer: 'typescript-language-server@6.1.0', typescript: '5.9.3',
    scope: 'local service only; OS filesystem caches are warm; process-cold and process-warm; no Mobile/network latency',
    fixture: { files: 2, sourceLines: 3, unicodePath: true, generated: true },
    cold: stats(cold), warm: stats(warm), disabledInstancesAtEnd: service.status.instances,
    targets: { serviceOnlyColdP95Under5000: stats(cold).p95Ms <= 5000, serviceOnlyWarmP95Under500: stats(warm).p95Ms <= 500 },
  };
  if (process.argv[2]) await writeFile(process.argv[2], JSON.stringify(result, null, 2) + '\n');
  console.log(JSON.stringify(result, null, 2));
} finally { await service?.disable(); await Promise.all(cleanup.map(action => action())); }
