import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const [modulePath, fixturePath] = process.argv.slice(2);
if (!modulePath || !fixturePath) {
  throw new Error('usage: node web_product_smoke.mjs MODULE.mjs FIXTURE.pdf');
}

const { default: createModule } = await import(pathToFileURL(resolve(modulePath)));
const product = await createModule();
assert.equal(product._ksp_product_abi_version(), 2);

const fixture = readFileSync(fixturePath);
const pdf = product._malloc(fixture.length);
assert.notEqual(pdf, 0);
product.HEAPU8.set(fixture, pdf);
assert.equal(product._ksp_product_page_count(pdf, BigInt(fixture.length)), 2);

const indices = product._malloc(4);
const selection = product._malloc(24);
const output = product._malloc(16);
const error = product._malloc(256);
assert.ok(indices && selection && output && error);

function compose(pageIndex) {
  // wasm32 C layout: pointer, 8-byte-aligned uint64, pointer, uint32.
  const view = new DataView(product.HEAPU8.buffer);
  view.setInt32(indices, pageIndex, true);
  view.setUint32(selection, pdf, true);
  view.setBigUint64(selection + 8, BigInt(fixture.length), true);
  view.setUint32(selection + 16, indices, true);
  view.setUint32(selection + 20, 1, true);
  product.HEAPU8.fill(0, output, output + 16);
  product.HEAPU8.fill(0, error, error + 256);
  return product._ksp_product_compose_pages(selection, 1, output, error, 256);
}

assert.equal(compose(1), 0);
let view = new DataView(product.HEAPU8.buffer);
const result = view.getUint32(output, true);
const resultSize = Number(view.getBigUint64(output + 8, true));
assert.ok(result && resultSize > 100);
assert.equal(product._ksp_product_page_count(result, BigInt(resultSize)), 1);
assert.equal(String.fromCharCode(...product.HEAPU8.slice(result, result + 5)), '%PDF-');
product._ksp_product_free_bytes(output);
view = new DataView(product.HEAPU8.buffer);
assert.equal(view.getUint32(output, true), 0);
assert.equal(view.getBigUint64(output + 8, true), 0n);

assert.ok(compose(99) < 0);
view = new DataView(product.HEAPU8.buffer);
assert.equal(view.getUint32(output, true), 0);
assert.equal(view.getBigUint64(output + 8, true), 0n);

assert.equal(product._ksp_product_edit_abi_version(), 6);
const editError = product._malloc(4);
new DataView(product.HEAPU8.buffer).setInt32(editError, 0, true);
const session = product._ksp_product_edit_open_bytes(
  pdf,
  BigInt(fixture.length),
  editError,
);
assert.ok(session, `edit open failed: ${new DataView(product.HEAPU8.buffer).getInt32(editError, true)}`);
assert.equal(product._ksp_product_edit_page_count(session), 2);
assert.equal(product._ksp_product_edit_load_page(session, 0), 0);
assert.ok(product._ksp_product_edit_domain_count(session) > 0);

function domainText(handle) {
  const length = product._ksp_product_edit_domain_text(handle, 0, 0, 0);
  assert.ok(length >= 0);
  const pointer = product._malloc(length * 2 || 2);
  try {
    assert.equal(product._ksp_product_edit_domain_text(handle, 0, pointer, length), length);
    const chars = [];
    const memory = new DataView(product.HEAPU8.buffer);
    for (let index = 0; index < length; index++) {
      chars.push(memory.getUint16(pointer + index * 2, true));
    }
    return String.fromCharCode(...chars);
  } finally {
    product._free(pointer);
  }
}

const original = domainText(session);
assert.ok(original.length > 0);
function exportedDomainText(handle) {
  const bytes = product._malloc(16);
  product.HEAPU8.fill(0, bytes, bytes + 16);
  try {
    assert.equal(product._ksp_product_edit_export_bytes(handle, bytes), 0);
    const view = new DataView(product.HEAPU8.buffer);
    const pointer = view.getUint32(bytes, true);
    const size = Number(view.getBigUint64(bytes + 8, true));
    const reopened = product._ksp_product_edit_open_bytes(pointer, BigInt(size), editError);
    assert.ok(reopened);
    try {
      assert.equal(product._ksp_product_edit_load_page(reopened, 0), 0);
      return domainText(reopened);
    } finally {
      product._ksp_product_edit_close(reopened);
    }
  } finally {
    product._ksp_product_free_bytes(bytes);
    product._free(bytes);
  }
}
const replacement = product._malloc(8);
view = new DataView(product.HEAPU8.buffer);
for (let index = 0; index < 4; index++) {
  view.setUint16(replacement + index * 2, 'WASM'.charCodeAt(index), true);
}
assert.equal(product._ksp_product_edit_replace_domain(session, 0, replacement, 4), 0);
assert.equal(exportedDomainText(session), 'WASM');
assert.equal(product._ksp_product_edit_undo(session), 0);
assert.equal(exportedDomainText(session), original);
assert.equal(product._ksp_product_edit_redo(session), 0);
assert.equal(exportedDomainText(session), 'WASM');

assert.equal(product._ksp_product_edit_export_bytes(session, output), 0);
view = new DataView(product.HEAPU8.buffer);
const editedPtr = view.getUint32(output, true);
const editedSize = Number(view.getBigUint64(output + 8, true));
assert.ok(editedPtr && editedSize > 100);
assert.equal(product._ksp_product_page_count(editedPtr, BigInt(editedSize)), 2);
const reopened = product._ksp_product_edit_open_bytes(editedPtr, BigInt(editedSize), editError);
assert.ok(reopened);
assert.equal(product._ksp_product_edit_load_page(reopened, 0), 0);
assert.equal(domainText(reopened), 'WASM');
product._ksp_product_edit_close(reopened);
product._ksp_product_free_bytes(output);
product._ksp_product_edit_close(session);

for (const pointer of [replacement, editError, error, output, selection, indices, pdf]) {
  product._free(pointer);
}
console.log('Web product ABI: open, extract, edit, undo, redo, save, reopen passed');
