import test from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { CodeNavigationService } from '../../lib/service.js';
import { stdioBackendFactory } from '../../lib/stdio.js';
import { fixture } from '../support/owner.js';

test('actual TypeScript language service: cross-file Unicode definition and warm reuse', async t => {
  const f = await fixture(t), traces = [];
  const service = new CodeNavigationService({ owner: f.owner, trace: e => traces.push(e),
    backendFactory: stdioBackendFactory({ typescript: {
      command: process.execPath,
      args: [fileURLToPath(new URL('../../node_modules/typescript-language-server/lib/cli.mjs', import.meta.url)), '--stdio'],
      initializationOptions: {
        disableAutomaticTypingAcquisition: true,
        tsserver: { useSyntaxServer: 'never',
          path: fileURLToPath(new URL('../../node_modules/typescript/lib/tsserver.js', import.meta.url)) },
      },
    } }),
  });
  t.after(() => service.disable()); service.enable();
  for (let i = 0; i < 2; i++) {
    const result = await service.query(f.input);
    assert.equal(result.kind, 'locations'); assert.equal(result.matches.length, 1);
    assert.equal(result.matches[0].selectionRange.start.line, 0);
    assert.equal(result.matches[0].label, '定义.ts');
  }
  assert.equal(traces.filter(e => e.stage === 'initialize').length, 1);
  assert.equal(traces.filter(e => e.stage === 'total')[1].cold, false);
  const refs = await service.query({ ...f.input, operation: 'references' });
  assert.ok(refs.matches.length >= 2);
  const hover = await service.query({ ...f.input, operation: 'hover' });
  assert.ok(JSON.stringify(hover.hover).includes('greet'));
  // UTF-16 offset is calculated from actual source, including emoji before
  // the identifier. The owner supplies a fresh matching content version.
  const line = 'const marker = "中文🙂"; const result = greet(marker);';
  await writeFile(join(f.root, 'main.ts'), 'import { greet } from "./定义";\n' + line + '\n');
  const read = f.owner.authorizeAndRead;
  f.owner.authorizeAndRead = async (...args) => ({ ...await read(...args), version: 'v2' });
  const unicode = await service.query({ ...f.input, version: 'v2', position: { line: 1, character: line.indexOf('greet') + 2 } });
  assert.equal(unicode.matches[0].label, '定义.ts');
  console.log('TypeScript smoke timings (not UI P95):', JSON.stringify(traces));
});
