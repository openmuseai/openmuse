import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { runInNewContext } from 'node:vm';
import test from 'node:test';

test('DSH file-address click goes to Host, non-file resources remain in DSH', async () => {
  let module;
  const messages = [];
  const workspaceMessages = [];
  const clipboardMessages = [];
  const events = new Map();
  let selection = '/workspace/docs/architecture.md';
  const appended = [];
  const document = {
    body: { appendChild: (item) => appended.push(item) },
    createElement: (tag) => ({
      tag, style: {}, children: [], listeners: {},
      setAttribute() {},
      appendChild(item) { this.children.push(item); },
      addEventListener(name, handler) { this.listeners[name] = handler; },
      remove() { this.removed = true; },
    }),
  };
  const openedWorkspaces = [];
  const original = [];
  const sidebarRight = { openResource: (...args) => original.push(args) };
  const window = {
    __ModuleLoader__: { load: (value) => { module = value.factory(); } },
    MuseHostResource: { postMessage: (value) => messages.push(JSON.parse(value)) },
    MuseHostWorkspace: { postMessage: (value) => workspaceMessages.push(JSON.parse(value)) },
    MuseHostClipboard: { postMessage: (value) => clipboardMessages.push(JSON.parse(value)) },
    getSelection: () => ({ toString: () => selection }),
    addEventListener: (name, handler) => events.set(name, handler),
    removeEventListener: (name) => events.delete(name),
    innerWidth: 1000, innerHeight: 800,
  };
  runInNewContext(readFileSync(new URL('../lib/client.js', import.meta.url), 'utf8'), { window, document });
  const ctx = {
    sidebarRight,
    sessions: { list: { getSnapshot: () => ({ byId: { s1: { cwd: '/workspace' } } }) } },
    uiWorkspace: {
      openWorkspace(id) {
        openedWorkspaces.push(id);
        this.mainReference = { sessionId: 's1' };
      },
      openSession(id) { this.mainReference = { sessionId: id }; },
    },
    workspaces: { list: {
      getSnapshot: () => ({ items: [{ workspaceId: 'w1', path: '/workspace' }] }),
      subscribe: () => () => {},
    } },
    effect: () => {},
  };
  module.apply(ctx);
  sidebarRight.openResource('dsh-resource://file/session/s1/docs/hello%20world.md', { params: { line: 12 } });
  assert.deepEqual(messages, [{ type: 'resource.open', path: 'docs/hello world.md', cwd: '/workspace', line: 12 }]);
  sidebarRight.openResource('dsh-resource://plan/s1/a');
  assert.equal(original.length, 1);
  sidebarRight.openResource('dsh-resource://file/session/unknown/private.txt');
  assert.equal(original.length, 1, 'embedded file links without an authorized session must not fall back to DSH');
  await ctx.uiWorkspace.openWorkspace('w1');
  assert.deepEqual(workspaceMessages, [{ type: 'workspace.activate', path: '/workspace' }]);
  window.OpenMuseDshWorkspace.activate('/workspace');
  assert.deepEqual(openedWorkspaces, ['w1'], 'Host activation must not re-open the already selected DSH workspace');
  ctx.uiWorkspace.openSession('s1');
  assert.deepEqual(workspaceMessages.at(-1), { type: 'workspace.activate', path: '/workspace' });
  let prevented = false;
  events.get('keydown')({
    key: 'c', metaKey: true, target: {},
    preventDefault: () => { prevented = true; },
    stopImmediatePropagation: () => {},
  });
  assert.equal(prevented, true);
  assert.deepEqual(clipboardMessages, [{ type: 'clipboard.write', text: '/workspace/docs/architecture.md' }]);
  events.get('contextmenu')({
    target: {}, clientX: 120, clientY: 130,
    preventDefault: () => {}, stopImmediatePropagation: () => {},
  });
  assert.deepEqual(appended.at(-1).children.map((item) => item.textContent), ['复制选中内容', '复制路径']);
  appended.at(-1).children[1].listeners.click();
  assert.deepEqual(clipboardMessages.at(-1), { type: 'clipboard.write', text: '/workspace/docs/architecture.md' });
  selection = 'architecture.md';
  events.get('contextmenu')({
    target: {}, clientX: 120, clientY: 130,
    preventDefault: () => {}, stopImmediatePropagation: () => {},
  });
  assert.deepEqual(appended.at(-1).children.map((item) => item.textContent), ['复制选中内容', '复制路径']);
  selection = '';
  prevented = false;
  events.get('keydown')({
    key: 'c', metaKey: true, target: {},
    preventDefault: () => { prevented = true; },
    stopImmediatePropagation: () => {},
  });
  assert.equal(prevented, false, 'empty selection must retain DSH/WebKit default copy');
  selection = 'input text';
  events.get('keydown')({
    key: 'c', metaKey: true, target: { closest: () => ({}) },
    preventDefault: () => { prevented = true; },
    stopImmediatePropagation: () => {},
  });
  assert.equal(prevented, false, 'editing inputs must retain their native copy behavior');
});
