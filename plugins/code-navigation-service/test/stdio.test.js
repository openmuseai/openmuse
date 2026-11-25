import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { CodeNavigationService } from '../lib/service.js';
import { stdioBackendFactory, normalizeLocations } from '../lib/stdio.js';
import { fixture } from './support/owner.js';

for (const mode of ['normal', 'worker', 'hang', 'utf8']) {
  test(`real subprocess: ${mode}, cleanup is awaited`, async t => {
    const f = await fixture(t), pidFile = join(f.root, 'pid');
    const service = new CodeNavigationService({ owner: f.owner,
      limits: { startupTimeoutMs: 800, queryTimeoutMs: 100 },
      backendFactory: stdioBackendFactory({ typescript: { command: process.execPath,
        args: [fileURLToPath(new URL('./support/lsp-server.js', import.meta.url)), mode, pidFile] } }, { shutdownMs: 30, killMs: 500 }),
    });
    t.after(() => service.disable()); service.enable();
    if (mode === 'normal' || mode === 'worker') {
      const result = await service.query(f.input);
      assert.equal(result.matches[0].selectionRange.start.line, 1);
    } else await assert.rejects(service.query(f.input), error => error.code === (mode === 'hang' ? 'TIMEOUT' : 'UNSUPPORTED_POSITION_ENCODING'));
    await service.disable();
    const pid = Number(await readFile(pidFile, 'utf8'));
    assert.throws(() => process.kill(pid, 0), { code: 'ESRCH' });
    if (mode === 'worker') {
      const worker = Number(await readFile(pidFile + '.worker', 'utf8'));
      assert.throws(() => process.kill(worker, 0), { code: 'ESRCH' });
    }
    assert.equal(service.status.instances, 0);
  });
}
test('malformed ranges are rejected before owner target lookup', () => {
  assert.throws(() => normalizeLocations({ uri: 'file:///x', range: { start: { line: 1, character: 0 }, end: { line: 0, character: 0 } } }), { code: 'MALFORMED_RESPONSE' });
});
