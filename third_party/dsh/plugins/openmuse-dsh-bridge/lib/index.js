/** OpenMuse-owned, token-gated bridge for Host-granted local workspaces. */
import { realpath, stat, readFile } from 'node:fs/promises';
import { isAbsolute } from 'node:path';
import { createRequire } from 'node:module';

export const inject = [];

const MAX_BODY = 64 * 1024;
const NATIVE_API_VERSION = 1;
const DSH_VERSION = '0.1.7-rc.1';
const require = createRequire(import.meta.url);
const NATIVE_SLOTS = new Set([
  'conversation.chat.node',
  'tool.call.toolview',
  'conversation.chat.assistant-actions',
  'conversation.chat.turnTail',
  'conversation.input.dock',
  'conversation.input.left',
  'conversation.input.right',
  'conversation.session.header.actions',
  'conversation.composer',
]);
const NATIVE_COMPONENTS = new Set([
  'text', 'markdown', 'code', 'badge', 'status', 'progress', 'keyValue',
  'table', 'image', 'gallery', 'fileLink', 'resourceLink', 'button',
  'iconButton', 'menu', 'segmentedControl', 'textField', 'select',
  'checkbox', 'disclosure', 'toolCard', 'messageAddon', 'turnTail',
  'approvalForm', 'questionForm', 'selectionForm', 'column', 'row', 'section',
]);
const NATIVE_TEMPLATES = new Set([
  'toolCard', 'messageAddon', 'turnTail', 'approvalForm', 'questionForm',
  'selectionForm', 'section',
]);
const BINDING_PATTERN = /^(session|turn|message|tool|plugin\.state)(\.[A-Za-z0-9_-]+)+$/;
const COMMAND_PATTERN = /^[A-Za-z0-9_-]+(?:\.[A-Za-z0-9_-]+)+$/;
const OFFICIAL_CONVERSATION_COVERAGE = new Map([
  ['@deepseek-ai/dsh-client-ui-conversation', 'native-shell'],
  ['@deepseek-ai/dsh-client-ui-chat', 'native'],
  ['@deepseek-ai/dsh-client-ui-tool', 'native-generic-tool'],
  ['@deepseek-ai/dsh-client-ui-user-questions', 'native-question'],
  ['@deepseek-ai/dsh-client-ui-model-selection', 'native-control'],
  ['@deepseek-ai/dsh-client-ui-permission-presets', 'native-control'],
  ['@deepseek-ai/dsh-client-ui-approval', 'element-fallback'],
  ['@deepseek-ai/dsh-client-ui-attachment', 'element-fallback'],
  ['@deepseek-ai/dsh-client-ui-deliverables', 'native-deliverables'],
  ['@deepseek-ai/dsh-client-ui-goal', 'element-fallback'],
  ['@deepseek-ai/dsh-client-ui-jobs', 'element-fallback'],
  ['@deepseek-ai/dsh-client-ui-plan', 'element-fallback'],
  ['@deepseek-ai/dsh-client-ui-schedule', 'element-fallback'],
  ['@deepseek-ai/dsh-client-ui-subagent', 'element-fallback'],
  ['@deepseek-ai/dsh-client-ui-trajectory', 'native-projection'],
  ['@deepseek-ai/dsh-client-ui-workflow-run', 'element-fallback'],
]);

export function createNativeInteractionCoordinator() {
  const followers = new Map();
  const pendingQuestions = new Map();

  const followerOpened = (sessionId) => {
    followers.set(sessionId, (followers.get(sessionId) ?? 0) + 1);
    return () => {
      const count = followers.get(sessionId) ?? 0;
      if (count <= 1) followers.delete(sessionId);
      else followers.set(sessionId, count - 1);
    };
  };

  const offerQuestion = (request, next) => {
    const sessionId = request?.agent?.id;
    if (typeof sessionId !== 'string' || (followers.get(sessionId) ?? 0) === 0 || pendingQuestions.has(sessionId)) {
      return next();
    }
    return new Promise((resolve, reject) => {
      let settled = false;
      const finish = (settle) => {
        if (settled) return;
        settled = true;
        request.signal?.removeEventListener('abort', onAbort);
        pendingQuestions.delete(sessionId);
        settle();
      };
      const onAbort = () => finish(() => reject(Object.assign(new Error('ask_user_question was aborted before the user answered'), {
        name: 'UserQuestionError', code: 'ASK_ABORTED',
      })));
      pendingQuestions.set(sessionId, {
        questions: request.questions,
        answer: (answer) => finish(() => resolve(answer)),
      });
      request.signal?.addEventListener('abort', onAbort, { once: true });
      if (request.signal?.aborted) onAbort();
    });
  };

  const answerQuestion = (sessionId, answers) => {
    const pending = pendingQuestions.get(sessionId);
    if (pending === undefined) return false;
    validateQuestionAnswers(pending.questions, answers);
    pending.answer({ answers });
    return true;
  };

  return { followerOpened, offerQuestion, answerQuestion };
}

function validateQuestionAnswers(questions, answers) {
  if (!Array.isArray(answers) || answers.length !== questions.length) throw new Error('invalid question answers');
  const byId = new Map(questions.map((question) => [question.id, question]));
  const seen = new Set();
  for (const answer of answers) {
    if (!answer || typeof answer !== 'object' || Array.isArray(answer)) throw new Error('invalid question answer');
    assertOnlyKeys(answer, ['id', 'selected', 'custom'], 'question answer');
    const question = byId.get(answer.id);
    if (question === undefined || seen.has(answer.id)) throw new Error('unknown or duplicate question id');
    seen.add(answer.id);
    if (!Array.isArray(answer.selected) || answer.selected.some((value) => typeof value !== 'string')) throw new Error('invalid selected answers');
    if (answer.custom !== undefined && (typeof answer.custom !== 'string' || answer.custom.trim().length === 0 || answer.custom.length > 8192)) {
      throw new Error('invalid custom answer');
    }
    const labels = new Set((question.options ?? []).map((option) => option.label));
    if (answer.selected.some((label) => !labels.has(label))) throw new Error('selected answer is not an offered option');
    if (question.multiSelect !== true && answer.selected.length > 1) throw new Error('single-select question has multiple answers');
  }
}
function reply(res, status, value) {
  res.writeHead(status, { 'content-type': 'application/json; charset=utf-8', 'cache-control': 'no-store' });
  res.end(JSON.stringify(value));
}
async function bodyOf(req) {
  let body = '';
  for await (const chunk of req) {
    body += chunk;
    if (body.length > MAX_BODY) throw new Error('request too large');
  }
  return JSON.parse(body);
}

function nativeFailure(error) {
  const candidate = typeof error?.code === 'string' ? error.code : '';
  return {
    error: 'native gateway request failed',
    code: /^[A-Z0-9_]{1,64}$/.test(candidate) ? candidate : 'NATIVE_GATEWAY_FAILED',
    detail: 'The Desktop host rejected the native conversation request.',
  };
}

function sessionAddress(sessionId) {
  if (typeof sessionId !== 'string' || sessionId.length === 0 || sessionId.length > 256) {
    throw new Error('invalid sessionId');
  }
  return { kind: 'session', sessionId };
}

function workspaceIdOf(value) {
  if (typeof value !== 'string' || value.length === 0 || value.length > 256) {
    throw new Error('invalid workspaceId');
  }
  return value;
}

function nativeWorkspaceView(workspace) {
  return {
    workspaceId: String(workspace.id),
    title: String(workspace.title),
    sessionIds: [...workspace.sessionIds].map(String),
    createdAt: String(workspace.createdAt),
    updatedAt: String(workspace.updatedAt),
  };
}

async function nativeCompatibility(ctx) {
  const inventory = ctx.get('pluginInventory');
  const snapshot = inventory?.list ? await inventory.list() : { entries: [] };
  const active = (snapshot.entries ?? [])
    .filter((entry) => entry.enabled && entry.fiberPhase !== 'failed')
    .map((entry) => ({
      entryId: String(entry.entryId),
      moduleName: entry.moduleName,
      fiberPhase: entry.fiberPhase,
    }));
  const forced = process.env.OPENMUSE_DSH_NATIVE_FORCE_WEBVIEW === '1';
  const compatible = !forced;
  return {
    nativeConversationCompatible: compatible,
    fallbackRequired: !compatible,
    requiresNegotiation: true,
    releaseReady: true,
    reason: forced ? 'forced-by-host' : null,
    activePlugins: active,
    unsupportedPlugins: [],
  };
}

export function nativePackageName(moduleName, entryId) {
  if (moduleName.startsWith('cordis:')) return '@deepseek-ai/cordis-runtime';
  if (moduleName.startsWith('file:')) {
    const match = moduleName.match(/\/node_modules\/((?:@[^/]+\/)?[^/]+)/);
    return match?.[1] ?? null;
  }
  if (moduleName === './lib/index.js' && entryId === 'model-capabilities') return 'dsh-model-capabilities';
  if (moduleName.includes('openmuse-dsh-bridge')) return 'openmuse-dsh-bridge';
  if (moduleName.startsWith('@')) return moduleName.split('/').slice(0, 2).join('/');
  if (!moduleName.startsWith('.') && !moduleName.startsWith('/')) return moduleName.split('/')[0];
  return null;
}

async function readNativeDescriptor(entry) {
  const name = nativePackageName(entry.moduleName, String(entry.entryId));
  if (name?.startsWith('@deepseek-ai/')) return officialNativeDescriptor(name);
  if (name === null) {
    return { pluginId: String(entry.entryId), impact: 'unknown-client', contributions: [] };
  }
  try {
    const manifestPath = require.resolve(`${name}/package.json`);
    const manifest = JSON.parse(await readFile(manifestPath, 'utf8'));
    const native = manifest.openmuse?.nativeConversation;
    if (native === undefined) {
      return { pluginId: manifest.name ?? name, impact: 'unknown-client', contributions: [] };
    }
    return validateNativeManifest(manifest.name ?? name, native);
  } catch {
    return { pluginId: name, impact: 'unknown-client', contributions: [] };
  }
}

export function officialNativeDescriptor(name) {
  const coverage = OFFICIAL_CONVERSATION_COVERAGE.get(name);
  if (coverage === undefined) return null;
  return {
    pluginId: name,
    impact: 'native-covered',
    coverage,
    contributions: [],
  };
}

export function validateNativeManifest(pluginId, manifest) {
  if (!manifest || manifest.schemaVersion !== 1) throw new Error(`${pluginId}: nativeConversation schemaVersion must be 1`);
  assertOnlyKeys(manifest, ['schemaVersion', 'impact', 'contributions'], `${pluginId}: nativeConversation`);
  const impact = manifest.impact;
  if (!['none', 'outside-conversation', 'native-covered', 'conversation'].includes(impact)) {
    throw new Error(`${pluginId}: invalid nativeConversation impact`);
  }
  const contributions = manifest.contributions;
  if (!Array.isArray(contributions) || contributions.length > 64) throw new Error(`${pluginId}: too many native contributions`);
  const keys = new Set();
  for (const contribution of contributions) {
    if (!contribution || typeof contribution !== 'object') throw new Error(`${pluginId}: invalid contribution`);
    assertOnlyKeys(
      contribution,
      ['slot', 'key', 'template', 'requires', 'fallback', 'title', 'body', 'actions', 'visibleWhen'],
      `${pluginId}: contribution`,
    );
    if (!NATIVE_SLOTS.has(contribution.slot)) throw new Error(`${pluginId}: unsupported slot ${contribution.slot}`);
    if (typeof contribution.key !== 'string' || contribution.key.length === 0 || contribution.key.length > 128) throw new Error(`${pluginId}: invalid contribution key`);
    if (!NATIVE_TEMPLATES.has(contribution.template)) throw new Error(`${pluginId}: unsupported native template ${contribution.template}`);
    const identity = `${contribution.slot}:${contribution.key}`;
    if (keys.has(identity)) throw new Error(`${pluginId}: duplicate contribution ${identity}`);
    keys.add(identity);
    if (!['genericToolCard', 'omit', 'web'].includes(contribution.fallback)) throw new Error(`${pluginId}: invalid fallback`);
    validateRequires(contribution.requires, pluginId);
    if (contribution.visibleWhen !== undefined) validateCondition(contribution.visibleWhen, 0);
    validateNativeValue(contribution.title, 0);
    const body = contribution.body ?? [];
    const actions = contribution.actions ?? [];
    if (!Array.isArray(body) || body.length > 64) throw new Error(`${pluginId}: invalid native body`);
    if (!Array.isArray(actions) || actions.length > 16) throw new Error(`${pluginId}: invalid native actions`);
    const budget = { count: 0 };
    for (const node of body) validateNativeNode(node, 0, budget);
    for (const action of actions) validateNativeAction(action);
  }
  return { pluginId, impact, contributions };
}

function validateNativeNode(node, depth, budget) {
  if (!node || typeof node !== 'object' || Array.isArray(node)) throw new Error('invalid native node');
  if (depth > 12 || ++budget.count > 256) throw new Error('native contribution exceeds structural limits');
  assertOnlyKeys(
    node,
    ['component', 'visibleWhen', 'tone', 'density', 'emphasis', 'text', 'label', 'value', 'children', 'command', 'arguments'],
    'native node',
  );
  const component = node.component;
  if (!NATIVE_COMPONENTS.has(component)) throw new Error(`unsupported native component ${component}`);
  if (node.visibleWhen !== undefined) validateCondition(node.visibleWhen, 0);
  validateNativeValue(node, depth);
  const children = node.children ?? [];
  if (!Array.isArray(children) || children.length > 64) throw new Error('invalid native children');
  for (const child of children) validateNativeNode(child, depth + 1, budget);
  if (node.command !== undefined && !COMMAND_PATTERN.test(node.command)) {
    throw new Error(`unsafe native command ${node.command}`);
  }
}

function validateNativeAction(action) {
  if (!action || typeof action !== 'object' || Array.isArray(action)) throw new Error('invalid native action');
  assertOnlyKeys(action, ['id', 'label', 'command', 'arguments'], 'native action');
  if (typeof action.id !== 'string' || !/^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$/.test(action.id)) throw new Error('invalid native action id');
  if (typeof action.label !== 'string' || action.label.length === 0 || action.label.length > 128) throw new Error('invalid native action label');
  if (typeof action.command !== 'string' || !COMMAND_PATTERN.test(action.command)) throw new Error(`unsafe native command ${String(action.command)}`);
  if (action.arguments !== undefined && (!action.arguments || typeof action.arguments !== 'object' || Array.isArray(action.arguments) || Object.keys(action.arguments).length > 32)) {
    throw new Error('invalid native action arguments');
  }
  validateNativeValue(action.arguments, 0);
}

function validateRequires(requires, pluginId) {
  if (!requires || typeof requires !== 'object' || Array.isArray(requires)) throw new Error(`${pluginId}: missing native requirements`);
  assertOnlyKeys(requires, ['nativeUiApi', 'components'], `${pluginId}: native requirements`);
  if (requires.nativeUiApi !== 1 || !Array.isArray(requires.components) || requires.components.length > 64) {
    throw new Error(`${pluginId}: invalid native requirements`);
  }
  if (new Set(requires.components).size !== requires.components.length || requires.components.some((item) => typeof item !== 'string' || !/^[A-Za-z][A-Za-z0-9]*@1$/.test(item))) {
    throw new Error(`${pluginId}: invalid native component requirements`);
  }
}

function validateCondition(condition, depth) {
  if (!condition || typeof condition !== 'object' || Array.isArray(condition) || depth > 8) throw new Error('invalid native condition');
  if (Object.hasOwn(condition, 'all')) {
    assertOnlyKeys(condition, ['all'], 'native condition');
    if (!Array.isArray(condition.all) || condition.all.length === 0 || condition.all.length > 16) throw new Error('invalid native condition group');
    for (const item of condition.all) validateCondition(item, depth + 1);
    return;
  }
  assertOnlyKeys(condition, ['field', 'equals'], 'native condition');
  if (typeof condition.field !== 'string' || !BINDING_PATTERN.test(condition.field)) throw new Error(`unsafe native condition ${String(condition.field)}`);
  if (condition.equals !== null && !['string', 'number', 'boolean'].includes(typeof condition.equals)) throw new Error('invalid native condition value');
}

function assertOnlyKeys(value, allowed, label) {
  const permitted = new Set(allowed);
  const unknown = Object.keys(value).find((key) => !permitted.has(key));
  if (unknown !== undefined) throw new Error(`${label}: unknown field ${unknown}`);
}

function validateNativeValue(value, depth) {
  if (depth > 16) throw new Error('native value exceeds structural limits');
  if (value === null || typeof value !== 'object') return;
  if (Array.isArray(value)) {
    if (value.length > 64) throw new Error('native value array exceeds structural limits');
    for (const item of value) validateNativeValue(item, depth + 1);
    return;
  }
  if (Object.hasOwn(value, 'bind')) {
    assertOnlyKeys(value, ['bind', 'fallback'], 'native binding');
    if (typeof value.bind !== 'string' || !BINDING_PATTERN.test(value.bind)) {
      throw new Error(`unsafe native binding ${String(value.bind)}`);
    }
    if (value.fallback !== null && value.fallback !== undefined && !['string', 'number', 'boolean'].includes(typeof value.fallback)) {
      throw new Error('invalid native binding fallback');
    }
    return;
  }
  for (const child of Object.values(value)) {
    validateNativeValue(child, depth + 1);
  }
}

export function negotiateNativeConversation(descriptors, capabilities) {
  const supportedSlots = new Set(capabilities?.slots ?? []);
  const supportedComponents = new Set(capabilities?.components ?? []);
  const api = capabilities?.nativeUiApi;
  const results = [];
  let mode = 'native';
  for (const descriptor of descriptors) {
    if (descriptor.impact === 'unknown-client') {
      results.push({ pluginId: descriptor.pluginId, mode: 'incompatible', reason: 'undeclared-client-side-effect', contributions: [] });
      mode = mode === 'native' ? 'generic' : mode;
      continue;
    }
    if (descriptor.impact === 'none' || descriptor.impact === 'outside-conversation') {
      results.push({ pluginId: descriptor.pluginId, mode: 'native', contributions: [] });
      continue;
    }
    const accepted = [];
    let pluginMode = 'native';
    for (const contribution of descriptor.contributions) {
      const required = contribution.requires ?? {};
      const components = required.components ?? [];
      const compatible = api === (required.nativeUiApi ?? 1) &&
        supportedSlots.has(contribution.slot) &&
        components.every((item) => supportedComponents.has(item));
      if (compatible) {
        accepted.push(contribution);
      } else if ((contribution.fallback ?? 'web') === 'genericToolCard' && contribution.slot === 'tool.call.toolview') {
        pluginMode = pluginMode === 'web' ? 'web' : 'generic';
      } else if ((contribution.fallback ?? 'web') === 'omit') {
        pluginMode = pluginMode === 'web' ? 'web' : 'generic';
      } else {
        // A Web-only contribution no longer takes the whole conversation away
        // from the native shell. The client renders an element-level notice and
        // lets the user opt into the Web compatibility surface explicitly.
        pluginMode = 'incompatible';
      }
    }
    if (pluginMode === 'incompatible') mode = mode === 'native' ? 'generic' : mode;
    else if (pluginMode === 'generic' && mode === 'native') mode = 'generic';
    results.push({
      pluginId: descriptor.pluginId,
      mode: pluginMode,
      ...(descriptor.coverage === undefined ? {} : { coverage: descriptor.coverage }),
      contributions: accepted,
    });
  }
  return { schemaVersion: 1, mode, plugins: results, contributions: results.flatMap((item) => item.contributions) };
}

async function nativeDescriptors(ctx) {
  const inventory = ctx.get('pluginInventory');
  if (!inventory?.list) return [];
  const snapshot = await inventory.list();
  const active = (snapshot.entries ?? []).filter((entry) => entry.enabled && entry.fiberPhase !== 'failed');
  const descriptors = await Promise.all(active.map(readNativeDescriptor));
  return descriptors.filter(Boolean);
}

async function handleNative(ctx, interactions, req, res, url) {
  const controller = ctx.get('sessionController');
  if (!controller) return reply(res, 503, { error: 'session controller unavailable' });
  const path = url.pathname;
  if (req.method === 'GET' && path === '/openmuse-native/v1/hello') {
    return reply(res, 200, {
      protocolVersion: NATIVE_API_VERSION,
      dshVersion: DSH_VERSION,
      stream: 'sse',
      capabilities: [
        'workspaces.list', 'sessions.list', 'session.create', 'session.follow',
        'session.page', 'session.prompt', 'session.cancel',
        'session.options', 'session.model.select', 'session.permission.select',
        'session.question.answer', 'session.changes.summary',
        'session.changes.preview',
        'session.changes.open-desktop',
      ],
      compatibility: await nativeCompatibility(ctx),
    });
  }
  if (req.method === 'GET' && path === '/openmuse-native/v1/workspaces') {
    const registry = ctx.get('workspaceRegistry');
    if (!registry?.list) return reply(res, 503, { error: 'workspace registry unavailable' });
    return reply(res, 200, { items: registry.list().map(nativeWorkspaceView) });
  }
  if (req.method === 'GET' && path === '/openmuse-native/v1/sessions') {
    const abort = new AbortController();
    res.on('close', () => abort.abort());
    return reply(res, 200, await controller.list({}, abort.signal));
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/session/create') {
    const payload = await bodyOf(req);
    const workspaceId = workspaceIdOf(payload.workspaceId);
    const registry = ctx.get('workspaceRegistry');
    if (!registry?.get) return reply(res, 503, { error: 'workspace registry unavailable' });
    if (registry.get(workspaceId) === undefined) {
      return reply(res, 404, { error: 'workspace not found', code: 'WORKSPACE_NOT_FOUND' });
    }
    if (payload.agentPreset !== undefined &&
        (typeof payload.agentPreset !== 'string' || payload.agentPreset.length === 0 || payload.agentPreset.length > 128)) {
      return reply(res, 400, { error: 'invalid agentPreset' });
    }
    return reply(res, 200, await controller.create({
      workspaceId,
      ...(payload.agentPreset === undefined ? {} : { agentPreset: payload.agentPreset }),
    }));
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/negotiate') {
    const capabilities = await bodyOf(req);
    const descriptors = await nativeDescriptors(ctx);
    return reply(res, 200, negotiateNativeConversation(descriptors, capabilities));
  }
  if (req.method === 'GET' && path === '/openmuse-native/v1/session/follow') {
    const address = sessionAddress(url.searchParams.get('sessionId'));
    const closeFollower = interactions.followerOpened(address.sessionId);
    const abort = new AbortController();
    res.on('close', () => abort.abort());
    res.writeHead(200, {
      'content-type': 'text/event-stream; charset=utf-8',
      'cache-control': 'no-store, no-transform',
      connection: 'keep-alive',
      'x-accel-buffering': 'no',
    });
    try {
      const frames = controller.follow({
        address,
        assistantStream: true,
        maxMessages: 500,
        turnWindow: { minMessages: 50, minTurns: 2 },
      }, abort.signal);
      for await (const frame of frames) {
        if (abort.signal.aborted) break;
        res.write(`event: frame\ndata: ${JSON.stringify(frame)}\n\n`);
      }
    } catch (error) {
      if (!abort.signal.aborted) {
        res.write(`event: error\ndata: ${JSON.stringify(nativeFailure(error))}\n\n`);
      }
    } finally {
      closeFollower();
      if (!res.writableEnded) res.end();
    }
    return;
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/session/page') {
    const payload = await bodyOf(req);
    const abort = new AbortController();
    res.on('close', () => abort.abort());
    const value = await controller.page({
      address: sessionAddress(payload.sessionId),
      throughSeq: payload.throughSeq,
      ...(payload.beforeSeq === undefined ? {} : { beforeSeq: payload.beforeSeq }),
      maxMessages: 500,
      turnWindow: { minMessages: 50, minTurns: 2 },
    }, abort.signal);
    return reply(res, 200, value);
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/session/prompt') {
    const payload = await bodyOf(req);
    sessionAddress(payload.sessionId);
    if (typeof payload.requestId !== 'string' || !['queue', 'steer'].includes(payload.mode) || !Array.isArray(payload.content)) {
      return reply(res, 400, { error: 'invalid prompt' });
    }
    const abort = new AbortController();
    res.on('close', () => abort.abort());
    return reply(res, 200, await controller.prompt(payload, abort.signal));
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/session/cancel') {
    const payload = await bodyOf(req);
    sessionAddress(payload.sessionId);
    return reply(res, 200, await controller.cancel({ sessionId: payload.sessionId }));
  }
  if (req.method === 'GET' && path === '/openmuse-native/v1/session/options') {
    sessionAddress(url.searchParams.get('sessionId'));
    const [models, permissions] = await Promise.all([
      controller.modelCatalog(),
      Promise.resolve(ctx.get('permissionPresets')?.catalog?.() ?? null),
    ]);
    return reply(res, 200, { models, permissions });
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/session/model') {
    const payload = await bodyOf(req);
    sessionAddress(payload.sessionId);
    if (typeof payload.provider !== 'string' || typeof payload.model !== 'string' ||
        (payload.reasoningEffort !== undefined && typeof payload.reasoningEffort !== 'string')) {
      return reply(res, 400, { error: 'invalid model selection' });
    }
    return reply(res, 200, await controller.selectModel({
      sessionId: payload.sessionId,
      provider: payload.provider,
      model: payload.model,
      ...(payload.reasoningEffort === undefined ? {} : { reasoningEffort: payload.reasoningEffort }),
    }));
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/session/permission') {
    const payload = await bodyOf(req);
    sessionAddress(payload.sessionId);
    if (typeof payload.preset !== 'string' || payload.preset.length === 0 || payload.preset.length > 128) {
      return reply(res, 400, { error: 'invalid permission preset' });
    }
    const permissionPresets = ctx.get('permissionPresets');
    const available = permissionPresets?.catalog?.().options ?? [];
    if (!available.some((option) => option.value === payload.preset)) {
      return reply(res, 400, { error: 'permission preset unavailable', code: 'PERMISSION_PRESET_UNAVAILABLE' });
    }
    const commands = ctx.get('commands');
    if (!commands?.execute) return reply(res, 503, { error: 'command service unavailable' });
    const resolved = await controller.resolveAgent(payload.sessionId);
    if (!resolved?.agent) throw resolved?.error ?? new Error('session agent unavailable');
    const abort = new AbortController();
    res.on('close', () => abort.abort());
    const execution = await commands.execute(resolved.agent, `/permission ${payload.preset}`, [], abort.signal);
    if (execution === undefined) return reply(res, 409, { error: 'permission command unavailable' });
    if (execution.kind === 'error') return reply(res, 409, { error: execution.text ?? 'permission switch failed' });
    return reply(res, 200, { selected: payload.preset });
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/session/question/answer') {
    const payload = await bodyOf(req);
    const address = sessionAddress(payload.sessionId);
    if (!Array.isArray(payload.answers)) return reply(res, 400, { error: 'invalid question answers' });
    if (!interactions.answerQuestion(address.sessionId, payload.answers)) {
      return reply(res, 409, { error: 'question is no longer pending', code: 'QUESTION_NOT_PENDING' });
    }
    return reply(res, 200, { accepted: true });
  }
  if (req.method === 'GET' && path === '/openmuse-native/v1/session/changes') {
    const address = sessionAddress(url.searchParams.get('sessionId'));
    const seq = Number(url.searchParams.get('seq'));
    if (!Number.isSafeInteger(seq) || seq < 1) return reply(res, 400, { error: 'invalid changes sequence' });
    const summary = ctx.get('workspaceChanges')?.summary?.(address.sessionId, seq);
    if (summary === undefined) return reply(res, 404, { error: 'change summary unavailable', code: 'CHANGES_UNAVAILABLE' });
    return reply(res, 200, {
      turn: summary.turn,
      total: summary.total,
      added: summary.added,
      deleted: summary.deleted,
      files: summary.files.map(({ display, added, deleted, binary, oversized }) => ({
        display, added, deleted,
        ...(binary === true ? { binary: true } : {}),
        ...(oversized === true ? { oversized: true } : {}),
      })),
    });
  }
  if (req.method === 'GET' && path === '/openmuse-native/v1/session/changes/preview') {
    const address = sessionAddress(url.searchParams.get('sessionId'));
    const seq = Number(url.searchParams.get('seq'));
    const index = Number(url.searchParams.get('index'));
    if (!Number.isSafeInteger(seq) || seq < 1 || !Number.isSafeInteger(index) || index < 0) {
      return reply(res, 400, { error: 'invalid changed file coordinates' });
    }
    const workspaceChanges = ctx.get('workspaceChanges');
    if (!workspaceChanges?.diff) return reply(res, 503, { error: 'workspace change preview unavailable' });
    const abort = new AbortController();
    res.on('close', () => abort.abort());
    const comparison = await workspaceChanges.diff(address.sessionId, seq, index, abort.signal);
    if (comparison === undefined) {
      return reply(res, 404, { error: 'changed file preview unavailable', code: 'CHANGED_FILE_PREVIEW_UNAVAILABLE' });
    }
    if (comparison.kind !== 'text') {
      return reply(res, 200, { kind: comparison.kind, display: comparison.display });
    }
    return reply(res, 200, {
      kind: 'text',
      display: comparison.display,
      before: comparison.before,
      after: comparison.after,
      coarse: comparison.coarse,
      hunks: comparison.hunks.map(({ oldStart, oldLines, newStart, newLines, lines }) => ({
        oldStart, oldLines, newStart, newLines, lines,
      })),
    });
  }
  if (req.method === 'POST' && path === '/openmuse-native/v1/session/changes/open') {
    const payload = await bodyOf(req);
    const address = sessionAddress(payload.sessionId);
    if (!Number.isSafeInteger(payload.seq) || payload.seq < 1 || !Number.isSafeInteger(payload.index) || payload.index < 0) {
      return reply(res, 400, { error: 'invalid changed file coordinates' });
    }
    const changes = ctx.get('workspaceChanges')?.summary?.(address.sessionId, payload.seq);
    const file = changes?.files?.[payload.index];
    if (changes === undefined || file === undefined) return reply(res, 404, { error: 'changed file unavailable', code: 'CHANGED_FILE_UNAVAILABLE' });
    if (controller.workspaceDesktop?.().available !== true) return reply(res, 409, { error: 'Desktop file opening is unavailable' });
    const workspaceFiles = ctx.get('workspaceFiles');
    const fs = ctx.get('fs');
    if (!workspaceFiles?.stat || !fs?.processPathFromHostPath || !fs?.resolve || !fs?.processPath) {
      return reply(res, 503, { error: 'workspace file service unavailable' });
    }
    const abort = new AbortController();
    res.on('close', () => abort.abort());
    const { absolutePath } = await workspaceFiles.stat({
      sessionId: address.sessionId,
      workspaceRoot: changes.cwd,
    }, file.path, abort.signal);
    const mapped = fs.processPathFromHostPath(absolutePath);
    if (mapped === undefined || fs.processPath(await fs.resolve(mapped, { signal: abort.signal })) !== absolutePath) {
      return reply(res, 422, { error: 'file has no verified Desktop path' });
    }
    await controller.openWorkspacePath({ path: absolutePath }, abort.signal);
    return reply(res, 200, { opened: true });
  }
  return reply(res, 404, { error: 'not found' });
}

export function apply(ctx) {
  const token = process.env.OPENMUSE_DSH_BRIDGE_TOKEN;
  if (typeof token !== 'string' || token.length < 32) return;
  let registered = false;
  const interactions = createNativeInteractionCoordinator();
  ctx.on('user-questions/request', function(request, next) {
    return interactions.offerQuestion(request, next);
  }, { global: true });
  const register = (webServer) => {
    if (registered || !webServer?.register) return;
    registered = true;
    ctx.effect(() => webServer.register({
      kind: 'prefix', path: '/openmuse-bridge',
      handler: async (req, res) => {
        if (req.headers['x-openmuse-bridge-token'] !== token) return reply(res, 403, { error: 'forbidden' });
        const path = new URL(req.url ?? '/', 'http://localhost').pathname;
        if (req.method !== 'POST' || path !== '/openmuse-bridge/workspaces') return reply(res, 404, { error: 'not found' });
        const registry = ctx.get('workspaceRegistry');
        if (!registry) return reply(res, 503, { error: 'workspace registry unavailable' });
        try {
          const payload = await bodyOf(req);
          if (!Array.isArray(payload.mounts) || payload.mounts.length > 128 || payload.mounts.some((item) => typeof item !== 'string' || item.length > 4096 || !isAbsolute(item))) {
            return reply(res, 400, { error: 'invalid mounts' });
          }
          const items = [];
          for (const requested of new Set(payload.mounts)) {
            const path = await realpath(requested);
            if (!(await stat(path)).isDirectory()) return reply(res, 400, { error: 'not a directory' });
            const workspace = await registry.resolveByPath(path) ?? await registry.create(path);
            items.push({ workspaceId: String(workspace.id), path: workspace.path });
          }
          // DSH registration is idempotent. Never delete workspaces created by DSH users.
          reply(res, 200, { items });
        } catch (error) {
          reply(res, 400, { error: String(error?.message ?? error) });
        }
      },
    }), 'openmuse-dsh-bridge: routes');
    ctx.effect(() => webServer.register({
      kind: 'prefix', path: '/openmuse-native',
      handler: async (req, res) => {
        if (req.headers['x-openmuse-bridge-token'] !== token) return reply(res, 403, { error: 'forbidden' });
        const url = new URL(req.url ?? '/', 'http://localhost');
        try {
          await handleNative(ctx, interactions, req, res, url);
        } catch (error) {
          if (!res.headersSent) reply(res, 400, nativeFailure(error));
          else if (!res.writableEnded) res.end();
        }
      },
    }), 'openmuse-dsh-bridge: native conversation routes');
  };
  register(ctx.get('webServer'));
  ctx.on('internal/service', (name, service) => {
    if (name === 'webServer') register(service);
  }, { global: true });
}
