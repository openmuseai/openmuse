import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import test from 'node:test';

import { apply, createNativeInteractionCoordinator, nativePackageName, negotiateNativeConversation, officialNativeDescriptor, validateNativeManifest } from '../lib/index.js';

function context(controller, inventory = { entries: [] }, workspaceRegistry, extras = {}) {
  const routes = [];
  const services = {
    sessionController: controller,
    pluginInventory: { list: async () => inventory },
    workspaceRegistry,
    webServer: {
      register(route) {
        routes.push(route);
        return () => {};
      },
    },
    ...extras,
  };
  return {
    routes,
    get: (name) => services[name],
    effect: (operation) => operation(),
    on: () => {},
  };
}

class Response extends EventEmitter {
  headersSent = false;
  writableEnded = false;
  chunks = [];
  writeHead(status, headers) {
    this.status = status;
    this.headers = headers;
    this.headersSent = true;
  }
  write(value) {
    this.chunks.push(value);
  }
  end(value = '') {
    this.chunks.push(value);
    this.writableEnded = true;
    this.emit('close');
  }
}

function request(url, method = 'GET', body) {
  const value = new EventEmitter();
  value.url = url;
  value.method = method;
  value.headers = { 'x-openmuse-bridge-token': process.env.OPENMUSE_DSH_BRIDGE_TOKEN };
  value[Symbol.asyncIterator] = async function* () {
    if (body !== undefined) yield Buffer.from(JSON.stringify(body));
  };
  return value;
}

test('native hello reports plugin compatibility and pinned DSH contract', async () => {
  process.env.OPENMUSE_DSH_BRIDGE_TOKEN = 'x'.repeat(32);
  delete process.env.OPENMUSE_DSH_NATIVE_FORCE_WEBVIEW;
  const ctx = context({}, {
    entries: [{ entryId: 'third-party', moduleName: 'custom-chat-theme', enabled: true, fiberPhase: 'active' }],
  });
  apply(ctx);
  const route = ctx.routes.find((value) => value.path === '/openmuse-native');
  const res = new Response();
  await route.handler(request('/openmuse-native/v1/hello'), res);
  const body = JSON.parse(res.chunks.join(''));
  assert.equal(res.status, 200);
  assert.equal(body.dshVersion, '0.1.7-rc.1');
  assert.equal(body.compatibility.fallbackRequired, false);
  assert.equal(body.compatibility.reason, null);
  assert.equal(body.compatibility.requiresNegotiation, true);
  assert.equal(body.compatibility.releaseReady, true);

  process.env.OPENMUSE_DSH_NATIVE_FORCE_WEBVIEW = '1';
  const forced = new Response();
  await route.handler(request('/openmuse-native/v1/hello'), forced);
  const forcedBody = JSON.parse(forced.chunks.join(''));
  assert.equal(forcedBody.compatibility.fallbackRequired, true);
  assert.equal(forcedBody.compatibility.reason, 'forced-by-host');
  delete process.env.OPENMUSE_DSH_NATIVE_FORCE_WEBVIEW;
});

test('native follow streams the DSH snapshot and ordered event frames', async () => {
  process.env.OPENMUSE_DSH_BRIDGE_TOKEN = 'y'.repeat(32);
  const controller = {
    async *follow(value) {
      assert.equal(value.address.sessionId, 's-1');
      yield { type: 'snapshot', cursor: 1, records: [], hasMore: false, header: { id: 's-1' }, projections: { asOfSeq: 1, values: {} } };
      yield { type: 'event', event: { type: 'turn/end', seq: 2, time: 2, data: { turn: 1, reason: { kind: 'completed' } } } };
    },
  };
  const ctx = context(controller);
  apply(ctx);
  const route = ctx.routes.find((value) => value.path === '/openmuse-native');
  const res = new Response();
  await route.handler(request('/openmuse-native/v1/session/follow?sessionId=s-1'), res);
  assert.equal(res.status, 200);
  assert.match(res.chunks.join(''), /event: frame/);
  assert.match(res.chunks.join(''), /\"cursor\":1/);
  assert.match(res.chunks.join(''), /\"seq\":2/);
});

test('native prompt delegates without reimplementing DSH admission', async () => {
  process.env.OPENMUSE_DSH_BRIDGE_TOKEN = 'z'.repeat(32);
  let observed;
  const ctx = context({
    async prompt(value) {
      observed = value;
      return { accepted: true };
    },
  });
  apply(ctx);
  const route = ctx.routes.find((value) => value.path === '/openmuse-native');
  const res = new Response();
  await route.handler(request('/openmuse-native/v1/session/prompt', 'POST', {
    sessionId: 's-1', requestId: 'mobile-1', mode: 'queue', content: [{ type: 'text', text: 'hello' }],
  }), res);
  assert.equal(observed.content[0].text, 'hello');
  assert.deepEqual(JSON.parse(res.chunks.join('')), { accepted: true });
});

test('native workspace catalog omits Desktop paths and creates a session in the selected workspace', async () => {
  process.env.OPENMUSE_DSH_BRIDGE_TOKEN = 'w'.repeat(32);
  const workspace = {
    id: 'workspace-1',
    path: '/Users/private/project',
    title: 'Project One',
    sessionIds: ['session-old'],
    createdAt: '2026-09-01T00:00:00.000Z',
    updatedAt: '2026-10-01T00:00:00.000Z',
  };
  let created;
  const ctx = context({
    async create(value) {
      created = value;
      return { sessionId: 'session-new', agentPreset: value.agentPreset };
    },
  }, { entries: [] }, {
    list: () => [workspace],
    get: (id) => id === workspace.id ? workspace : undefined,
  });
  apply(ctx);
  const route = ctx.routes.find((value) => value.path === '/openmuse-native');

  const listResponse = new Response();
  await route.handler(request('/openmuse-native/v1/workspaces'), listResponse);
  const listBody = JSON.parse(listResponse.chunks.join(''));
  assert.equal(listResponse.status, 200);
  assert.deepEqual(listBody.items[0], {
    workspaceId: 'workspace-1',
    title: 'Project One',
    sessionIds: ['session-old'],
    createdAt: '2026-09-01T00:00:00.000Z',
    updatedAt: '2026-10-01T00:00:00.000Z',
  });
  assert.equal(JSON.stringify(listBody).includes('/Users/private'), false);

  const createResponse = new Response();
  await route.handler(request('/openmuse-native/v1/session/create', 'POST', {
    workspaceId: 'workspace-1', agentPreset: 'standard',
  }), createResponse);
  assert.equal(createResponse.status, 200);
  assert.deepEqual(created, { workspaceId: 'workspace-1', agentPreset: 'standard' });
  assert.equal(JSON.parse(createResponse.chunks.join('')).sessionId, 'session-new');
});

test('declarative native UI keeps incompatible plugins element-scoped', () => {
  const weather = validateNativeManifest('weather', {
    schemaVersion: 1,
    impact: 'conversation',
    contributions: [{
      slot: 'tool.call.toolview', key: 'weather', template: 'toolCard',
      requires: { nativeUiApi: 1, components: ['toolCard@1', 'keyValue@1'] },
      body: [{ component: 'keyValue', label: 'temperature', value: { bind: 'tool.result.temperature' } }],
      fallback: 'genericToolCard',
    }],
  });
  const native = negotiateNativeConversation([weather], {
    nativeUiApi: 1, slots: ['tool.call.toolview'], components: ['toolCard@1', 'keyValue@1'],
  });
  assert.equal(native.mode, 'native');
  assert.equal(native.contributions.length, 1);

  const generic = negotiateNativeConversation([weather], {
    nativeUiApi: 1, slots: ['tool.call.toolview'], components: ['toolCard@1'],
  });
  assert.equal(generic.mode, 'generic');

  const incompatible = negotiateNativeConversation([{ pluginId: 'legacy-react', impact: 'unknown-client', contributions: [] }], {
    nativeUiApi: 1, slots: [], components: [],
  });
  assert.equal(incompatible.mode, 'generic');
  assert.equal(incompatible.plugins[0].mode, 'incompatible');
});

test('native session options, model, and permission routes delegate to DSH owners', async () => {
  process.env.OPENMUSE_DSH_BRIDGE_TOKEN = 'o'.repeat(32);
  const agent = { id: 's-1' };
  let modelRequest;
  let permissionLine;
  const controller = {
    modelCatalog: async () => ({
      default: { provider: 'p', model: 'm' }, routableProviders: ['p'],
      groups: [{ id: 'p', name: 'Provider', models: [{ id: 'm', name: 'Model' }] }], failures: [],
    }),
    selectModel: async (value) => {
      modelRequest = value;
      return { selected: value };
    },
    resolveAgent: async () => ({ agent }),
  };
  const ctx = context(controller, { entries: [] }, undefined, {
    permissionPresets: { catalog: () => ({ options: [{ value: 'workspace-write', name: 'Workspace write' }], defaultOptions: [], defaultPreset: 'workspace-write' }) },
    commands: { execute: async (value, line) => {
      assert.equal(value, agent);
      permissionLine = line;
      return { kind: 'success' };
    } },
  });
  apply(ctx);
  const route = ctx.routes.find((value) => value.path === '/openmuse-native');

  const optionsResponse = new Response();
  await route.handler(request('/openmuse-native/v1/session/options?sessionId=s-1'), optionsResponse);
  const options = JSON.parse(optionsResponse.chunks.join(''));
  assert.equal(options.models.groups[0].models[0].id, 'm');
  assert.equal(options.permissions.options[0].value, 'workspace-write');

  const modelResponse = new Response();
  await route.handler(request('/openmuse-native/v1/session/model', 'POST', {
    sessionId: 's-1', provider: 'p', model: 'm', reasoningEffort: 'high',
  }), modelResponse);
  assert.equal(modelRequest.reasoningEffort, 'high');

  const permissionResponse = new Response();
  await route.handler(request('/openmuse-native/v1/session/permission', 'POST', {
    sessionId: 's-1', preset: 'workspace-write',
  }), permissionResponse);
  assert.equal(permissionLine, '/permission workspace-write');
  assert.equal(JSON.parse(permissionResponse.chunks.join('')).selected, 'workspace-write');
});

test('native question coordinator claims only followed sessions and resolves structured answers', async () => {
  const coordinator = createNativeInteractionCoordinator();
  let delegated = false;
  const request = {
    agent: { id: 's-question' },
    questions: [{
      id: 'confirm', question: 'Continue?', multiSelect: false,
      options: [{ label: 'Yes' }, { label: 'No' }],
    }],
  };
  const delegatedResult = await coordinator.offerQuestion(request, async () => {
    delegated = true;
    return { answers: [] };
  });
  assert.equal(delegated, true);
  assert.deepEqual(delegatedResult, { answers: [] });

  const close = coordinator.followerOpened('s-question');
  const pending = coordinator.offerQuestion(request, async () => {
    throw new Error('must not delegate a natively followed session');
  });
  assert.equal(coordinator.answerQuestion('s-question', [{ id: 'confirm', selected: ['Yes'] }]), true);
  assert.deepEqual(await pending, { answers: [{ id: 'confirm', selected: ['Yes'] }] });
  assert.equal(coordinator.answerQuestion('s-question', [{ id: 'confirm', selected: ['No'] }]), false);
  close();
});

test('native workspace changes route returns mobile-safe summary and opens verified Desktop file', async () => {
  process.env.OPENMUSE_DSH_BRIDGE_TOKEN = 'c'.repeat(32);
  let opened;
  const controller = {
    workspaceDesktop: () => ({ available: true }),
    openWorkspacePath: async (value) => { opened = value.path; },
  };
  const summary = {
    turn: 2, cwd: '/workspace', total: 1, added: 3, deleted: 1,
    files: [{ path: 'README.md', display: 'README.md', added: 3, deleted: 1 }],
  };
  const ctx = context(controller, { entries: [] }, undefined, {
    workspaceChanges: {
      summary: (sessionId, seq) => sessionId === 's-1' && seq === 9 ? summary : undefined,
      diff: async (sessionId, seq, index) => sessionId === 's-1' && seq === 9 && index === 0 ? {
        kind: 'text', path: '/workspace/README.md', display: 'README.md',
        before: false, after: true, coarse: false,
        hunks: [{ oldStart: 1, oldLines: 0, newStart: 1, newLines: 2, lines: ['+# Title', '+Body'] }],
      } : undefined,
    },
    workspaceFiles: { stat: async () => ({ absolutePath: '/workspace/README.md' }) },
    fs: {
      processPathFromHostPath: (value) => value,
      resolve: async (value) => value,
      processPath: (value) => value,
    },
  });
  apply(ctx);
  const route = ctx.routes.find((value) => value.path === '/openmuse-native');
  const listResponse = new Response();
  await route.handler(request('/openmuse-native/v1/session/changes?sessionId=s-1&seq=9'), listResponse);
  const listed = JSON.parse(listResponse.chunks.join(''));
  assert.equal(listed.files[0].display, 'README.md');
  assert.equal(Object.hasOwn(listed.files[0], 'path'), false);

  const previewResponse = new Response();
  await route.handler(request('/openmuse-native/v1/session/changes/preview?sessionId=s-1&seq=9&index=0'), previewResponse);
  const preview = JSON.parse(previewResponse.chunks.join(''));
  assert.equal(preview.before, false);
  assert.equal(preview.after, true);
  assert.deepEqual(preview.hunks[0].lines, ['+# Title', '+Body']);
  assert.equal(Object.hasOwn(preview, 'path'), false);

  const openResponse = new Response();
  await route.handler(request('/openmuse-native/v1/session/changes/open', 'POST', {
    sessionId: 's-1', seq: 9, index: 0,
  }), openResponse);
  assert.equal(opened, '/workspace/README.md');
  assert.deepEqual(JSON.parse(openResponse.chunks.join('')), { opened: true });
});

test('declarative native UI rejects unsafe bindings and arbitrary components', () => {
  assert.throws(() => validateNativeManifest('unsafe', {
    schemaVersion: 1,
    impact: 'conversation',
    contributions: [{
      slot: 'tool.call.toolview', key: 'unsafe', template: 'toolCard',
      requires: { nativeUiApi: 1, components: ['toolCard@1'] },
      fallback: 'web',
      body: [{ component: 'absolutePosition', text: { bind: 'process.env.SECRET' } }],
    }],
  }), /unsupported native component|unsafe native binding/);

  assert.throws(() => validateNativeManifest('unsafe-action', {
    schemaVersion: 1,
    impact: 'conversation',
    contributions: [{
      slot: 'tool.call.toolview', key: 'unsafe-action', template: 'toolCard',
      requires: { nativeUiApi: 1, components: ['toolCard@1'] },
      fallback: 'web',
      actions: [{
        id: 'open', label: 'open', command: 'https://attacker.invalid/run',
        arguments: { secret: { bind: 'process.env.SECRET' } },
      }],
    }],
  }), /unsafe native binding|unsafe native command/);

  assert.throws(() => validateNativeManifest('unknown-field', {
    schemaVersion: 1,
    impact: 'conversation',
    contributions: [{
      slot: 'tool.call.toolview', key: 'unknown-field', template: 'toolCard',
      requires: { nativeUiApi: 1, components: ['toolCard@1'] },
      fallback: 'web',
      body: [{ component: 'text', text: 'safe', css: 'display:none' }],
    }],
  }), /unknown field css/);
});

test('runtime file URL plugins resolve their package names', () => {
  assert.equal(
    nativePackageName(
      'file:///runtime/node_modules/openmuse-dsh-bridge/lib/index.js',
      'include:openmuse-host-bridge',
    ),
    'openmuse-dsh-bridge',
  );
  assert.equal(nativePackageName('cordis:include', 'include'), '@deepseek-ai/cordis-runtime');
});

test('official conversation plugins have explicit native coverage classes', () => {
  assert.equal(
    officialNativeDescriptor('@deepseek-ai/dsh-client-ui-model-selection').coverage,
    'native-control',
  );
  assert.equal(
    officialNativeDescriptor('@deepseek-ai/dsh-client-ui-approval').coverage,
    'element-fallback',
  );
  assert.equal(
    officialNativeDescriptor('@deepseek-ai/dsh-client-ui-settings'),
    null,
  );
});
