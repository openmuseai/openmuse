import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const [bridgePath, fixturePath, orderPath] = process.argv.slice(2);
if (!bridgePath || !fixturePath || !orderPath) {
  throw new Error('usage: node web_bridge_smoke.mjs BRIDGE.mjs FIXTURE.pdf ORDER.pdf');
}
const { createBridge } = await import(pathToFileURL(resolve(bridgePath)));
const bridge = await createBridge();
const input = readFileSync(fixturePath);
assert.equal(bridge.pageCount(input), 2);
const session = bridge.open(input);
try {
  assert.equal(bridge.editPageCount(session), 2);
  bridge.loadPage(session, 0);
  const domains = bridge.domains(session);
  assert.ok(domains.length > 0);
  const size = bridge.pageSize(session, 0);
  assert.ok(size.width > 0 && size.height > 0);
  const firstPage = bridge.render(session, 0, 0, 0, 128, 128, 128, 128);
  const region = bridge.render(session, 0, 32, 40, 64, 48, 128, 128);
  assert.equal(firstPage.length, 128 * 128 * 4);
  assert.equal(region.length, 64 * 48 * 4);
  for (let row = 0; row < 48; row++) {
    assert.deepEqual(
      region.subarray(row * 64 * 4, (row + 1) * 64 * 4),
      firstPage.subarray(((row + 40) * 128 + 32) * 4,
        ((row + 40) * 128 + 96) * 4),
    );
  }
  const pageText = bridge.pageText(session, 0);
  assert.equal(pageText.units.length, pageText.boxes.length / 4);
  assert.ok(String.fromCharCode(...pageText.units).includes('Editable'));
  assert.deepEqual(bridge.pageLinks(session, 0), []);
  const original = bridge.domainText(session, 0);
  const focus = bridge.focusAt(session, 150, 680);
  assert.ok(focus.text.includes('Editable'));
  bridge.replaceDomain(session, 0, 'WASM');
  assert.equal(bridge.canUndo(session), true);
  assert.equal(bridge.pageCount(bridge.snapshot(session)), 2);
  bridge.undo(session);
  assert.equal(bridge.pageCount(bridge.snapshot(session)), 2);
  bridge.redo(session);
  const edited = bridge.snapshot(session);
  const reopened = bridge.open(edited);
  try {
    bridge.loadPage(reopened, 0);
    assert.equal(bridge.domainText(reopened, 0), 'WASM');
  } finally {
    bridge.close(reopened);
  }
  assert.ok(original.length > 0);
  const extracted = bridge.composeFlat([edited], [[1]]);
  assert.equal(bridge.pageCount(extracted), 1);
  const merged = bridge.composeFlat([extracted, edited], [[0], [0, 1]]);
  assert.equal(bridge.pageCount(merged), 3);
  const orderSource = readFileSync(orderPath);
  const ordered = bridge.composeFlat([orderSource], [[1, 0, 1]]);
  const orderedSession = bridge.open(ordered);
  try {
    assert.equal(bridge.pageSize(orderedSession, 0).width, 202);
    assert.equal(bridge.pageSize(orderedSession, 1).width, 101);
    assert.equal(bridge.pageSize(orderedSession, 2).width, 202);
  } finally {
    bridge.close(orderedSession);
  }
} finally {
  bridge.close(session);
}
console.log('Web bridge: render, text, focus, edit, undo, redo, compose passed');
