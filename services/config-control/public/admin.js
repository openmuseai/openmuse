const $ = id => document.getElementById(id);
const state = {scope: null, draft: null, body: null, base: null, scopes: [], catalog: null, session: null, view: 'overview', locale: null, token: ''};
let ui;
const t = key => ui.entries[`admin.${key}`]?.messages[state.locale] ?? ui.entries[`admin.${key}`]?.messages[ui.metadata.templateLocale] ?? key;
const text = (tag, value, className) => { const node = document.createElement(tag); node.textContent = value; if (className) node.className = className; return node; };
function translate() {
  document.documentElement.lang = state.locale;
  document.querySelectorAll('[data-i18n]').forEach(el => el.textContent = t(el.dataset.i18n));
  document.querySelectorAll('[data-placeholder]').forEach(el => el.placeholder = t(el.dataset.placeholder));
  $('page-title').textContent = t(state.view === 'editor' ? 'configuration' : state.view);
}
function notice(value, error = false) { $('notice').hidden = false; $('notice').textContent = value; $('notice').classList.toggle('error', error); }
async function request(path, method = 'GET', body) {
  const headers = {};
  if (state.session?.demo) headers['x-demo-actor'] = $('actor').value;
  else if (state.token) headers.authorization = `Bearer ${state.token}`;
  if (body !== undefined) headers['content-type'] = 'application/json';
  const response = await fetch(path, {method, headers, credentials: 'omit', ...(body === undefined ? {} : {body: JSON.stringify(body)})});
  const result = await response.json(); if (!response.ok) throw Error(result.error || String(response.status)); return result;
}
const run = action => async () => { try { await action(); } catch (error) { notice(error.message, true); } };
function show(view) {
  state.view = view; document.querySelectorAll('.view').forEach(el => el.hidden = el.id !== view);
  document.querySelectorAll('[data-view]').forEach(el => el.classList.toggle('selected', el.dataset.view === view)); translate();
}
function options(element, values, selected) {
  element.replaceChildren(...values.map(({value, label}) => { const option = text('option', label); option.value = value; return option; }));
  if (selected !== undefined) element.value = selected;
}
function table(container, columns, rows) {
  container.replaceChildren();
  if (!rows.length) { container.append(text('div', t('empty'), 'empty')); return; }
  const table = document.createElement('table'); table.className = 'table';
  const head = document.createElement('tr'); columns.forEach(c => head.append(text('th', t(c))));
  const thead = document.createElement('thead'); thead.append(head); table.append(thead);
  const tbody = document.createElement('tbody');
  rows.forEach(row => { const tr = document.createElement('tr'); row.forEach(value => { const td = document.createElement('td'); if (value instanceof Node) td.append(value); else td.textContent = String(value ?? '—'); tr.append(td); }); tbody.append(tr); });
  table.append(tbody); container.append(table);
}
function button(key, action) { const b = text('button', t(key)); b.addEventListener('click', run(action)); return b; }
const scopePath = action => `/v1/admin/scopes/${encodeURIComponent(state.scope.id)}${action ? '/' + action : ''}`;
const draftPath = action => `/v1/admin/drafts/${state.draft.id}${action ? '/' + action : ''}`;
const date = value => new Date(value).toLocaleString(state.locale);
function roles() { return state.session.demo ? state.session.actors.find(a => a.id === $('actor').value).roles : state.session.actor.roles; }
async function refresh() {
  state.scopes = await request('/v1/admin/scopes');
  const id = $('scope').value || state.scopes[0]?.id;
  options($('scope'), state.scopes.map(s => ({value: s.id, label: s.id})), id);
  if (!id) { $('new-draft').disabled = true; return; }
  state.scope = await request(`/v1/admin/scopes/${encodeURIComponent(id)}`);
  state.base = await request(scopePath('template'));
  const s = state.scope, baseline = s.bundle.snapshot;
  $('scope-description').textContent = [baseline.environment, baseline.distributionId, baseline.scope.dataRegion, baseline.scope.platform].join(' / ');
  $('sequence').textContent = s.sequence; $('active-release').textContent = s.current ?? '—';
  $('rollout-value').textContent = `${s.percent}%`; $('percent').value = s.percent;
  $('language-count').textContent = baseline.localization.enabledLocales.length;
  $('language-list').textContent = baseline.localization.enabledLocales.join(' / ');
  const [drafts, releases, audit] = await Promise.all(['drafts', 'releases', 'audit'].map(action => request(scopePath(action))));
  $('draft-count').textContent = drafts.filter(d => d.state !== 'published').length;
  table($('draft-list'), ['id', 'state', 'updated', 'actions'], drafts.map(d => [d.id.slice(0, 8), t(d.state === 'submitted' ? 'submittedState' : d.state === 'published' ? 'publishedState' : 'draft'), date(d.updated), button('open', () => openDraft(d.id))]));
  table($('release-list'), ['sequence', 'id', 'publishedAt', 'actions'], releases.map(r => [r.sequence, r.id.slice(0, 8), date(r.at), button('rollback', async () => { const draft = await request(scopePath('rollback'), 'POST', {releaseId: r.id}); await refresh(); await openDraft(draft.id); })]));
  table($('audit-list'), ['updated', 'actor', 'action', 'detail'], audit.map(a => [date(a.at), a.actor, a.action, JSON.stringify(a.detail)]));
  updateButtons();
}
function updateButtons() {
  const role = roles(), d = state.draft;
  $('new-draft').disabled = !role.includes('editor');
  $('save').disabled = !role.includes('editor') || !state.body || (d && d.state !== 'draft');
  $('submit').disabled = !role.includes('editor') || !d || d.state !== 'draft';
  $('approve').disabled = !role.includes('approver') || !d || d.state !== 'submitted';
  $('publish').disabled = !role.includes('publisher') || !d || d.state !== 'submitted';
  $('apply-rollout').disabled = !role.includes('publisher');
  $('attach-pack').disabled = !role.includes('editor') || !state.body || (d && d.state !== 'draft');
  for (const id of ['snapshot','note','channel']) $(id).readOnly = !!d && d.state !== 'draft';
  for (const id of ['portal-media-url','portal-poster-url','portal-apply','portal-downloads','portal-downloads-apply']) $(id).disabled=!role.includes('editor') || !!d && d.state!=='draft';
  $('default-locale').disabled = !!d && d.state !== 'draft';
  for(const id of ['flag-market','flag-updates','refresh-interval']) $(id).disabled=!!d && d.state!=='draft';
}
function blank() {
  const snapshot = structuredClone(state.scope.bundle.snapshot);
  snapshot.releaseId = 'draft'; snapshot.sequence = 0; delete snapshot.issuedAt; delete snapshot.expiresAt;
  snapshot.resources = []; snapshot.localization.bundleRefs = [];
  return {snapshot, packs: [], note: ''};
}
function diff(before, after, path = '') {
  const lines = [];
  for (const key of new Set([...Object.keys(before ?? {}), ...Object.keys(after ?? {})])) {
    const a = before?.[key], b = after?.[key], at = path ? `${path}.${key}` : key;
    if (JSON.stringify(a) === JSON.stringify(b)) continue;
    if (a && b && typeof a === 'object' && typeof b === 'object' && !Array.isArray(a) && !Array.isArray(b)) lines.push(...diff(a,b,at));
    else lines.push(`${at}\n− ${JSON.stringify(a)}\n+ ${JSON.stringify(b)}`);
  }
  return lines;
}
function renderEditor() {
  const b = state.body, d = state.draft;
  $('note').value = b.note; $('snapshot').value = JSON.stringify(b.snapshot, null, 2);
  $('channel').value = b.snapshot.updates.channel;
  $('runtime-settings').hidden=state.scope.bundle.snapshot.scope.platform==='website';
  renderPortal();
  const settings=b.snapshot.settings??{};
  $('flag-market').checked=settings.features?.['plugins.market.enabled']??true;
  $('flag-updates').checked=settings.features?.['updates.prompts.enabled']??true;
  $('refresh-interval').value=settings.parameters?.['runtime.refreshIntervalSeconds']??300;
  const locales=[...state.catalog.metadata.locales,...(b.snapshot.localization.localeCatalog??[])];
  options($('pack-locale'), [...new Map(locales.map(v=>[v.tag,v])).values()].map(v=>({value:v.tag,label:v.label})), $('pack-locale').value);

  options($('default-locale'), b.snapshot.localization.enabledLocales.map(value => ({value, label: locales.find(l => l.tag === value)?.label ?? value})), b.snapshot.localization.defaultLocale);
  $('draft-state').textContent = d ? `${t(d.state === 'submitted' ? 'submittedState' : d.state === 'published' ? 'publishedState' : 'draft')} · v${d.version}` : t('unsaved');
  $('draft-hash').textContent = d?.hash ?? '';
  const changes = diff(state.base.snapshot, b.snapshot);
  for (const pack of b.packs) {
    const previous = state.base.packs.find(p => p.locale === pack.locale && p.namespace === pack.namespace);
    const baseline = previous?.messages ?? Object.fromEntries(Object.entries(state.catalog.entries).filter(([_,v]) => v.namespace === pack.namespace).map(([id,v]) => [id,v.messages[pack.locale] ?? v.messages[state.catalog.metadata.templateLocale]]));
    changes.push(...diff(baseline, pack.messages, `${pack.locale}/${pack.namespace}`));
  }
  $('diff').textContent = changes.join('\n\n') || t('noDiff');
  $('approval-list').replaceChildren(...(d?.approvals ?? []).map(a => text('p', `${a.actor} · ${date(a.at)}`)));
  $('pack-list').replaceChildren(...b.packs.map(p => text('p', `${p.locale} / ${p.namespace} · ${Object.keys(p.messages).length}`)));
  updateButtons();
}
async function openDraft(id) { state.draft = await request(`/v1/admin/drafts/${id}`); state.body = structuredClone(state.draft.body); renderEditor(); show('editor'); }
function edited() {
  if (!state.body) throw Error(t('needDraft'));
  const snapshot = JSON.parse($('snapshot').value);
  snapshot.localization.defaultLocale = $('default-locale').value; snapshot.updates.channel = $('channel').value;
  if (state.scope.bundle.snapshot.scope.platform!=='website') {
    snapshot.compatibility.requiredCapabilities=[...new Set([...snapshot.compatibility.requiredCapabilities,'settings.v1'])];
  if (state.body.packs.some(p=>p.namespace.startsWith('plugin.'))) snapshot.compatibility.requiredCapabilities=[...new Set([...snapshot.compatibility.requiredCapabilities,'plugin-messages.v1'])];
  snapshot.settings??={};snapshot.settings.features??={};snapshot.settings.parameters??={};
  snapshot.settings.features['plugins.market.enabled']=$('flag-market').checked;
  snapshot.settings.features['updates.prompts.enabled']=$('flag-updates').checked;
  snapshot.settings.parameters['runtime.refreshIntervalSeconds']=Number($('refresh-interval').value);
  }
  state.body = {snapshot, packs: state.body.packs, note: $('note').value};
  return state.body;
}
async function save() {
  const b = edited();
  state.draft = state.draft ? await request(draftPath(), 'PUT', {version: state.draft.version, body: b}) : await request(scopePath('drafts'), 'POST', b);
  state.body = structuredClone(state.draft.body); renderEditor(); await refresh(); notice(t('saved'));
}
function loadPack() {
  const locale = $('pack-locale').value, namespace = $('pack-namespace').value;
  const contract=state.catalog.metadata.pluginContracts?.[namespace];
  const pack = state.body?.packs.find(p => p.locale === locale && p.namespace === namespace) ?? {schemaVersion: 1, sourceRevision: contract?.sourceRevision??state.catalog.metadata.sourceRevision, ...(contract?{contractHash:contract.contractHash}:{}), locale, namespace, messages: Object.fromEntries(Object.entries(state.catalog.entries).filter(([_,v]) => v.namespace === namespace).map(([id,v]) => [id,v.messages[locale] ?? v.messages[state.catalog.metadata.templateLocale]]))};
  $('pack').value = JSON.stringify(pack,null,2);
}
async function connect() {
  state.token = $('token').value;
  state.session = await request('/v1/admin/session');
  $('demo-banner').hidden = !state.session.demo;
  if (state.session.demo) options($('actor'), state.session.actors.map(a => ({value: a.id, label: `${a.id} · ${a.roles.filter(r => r !== 'reader').join(', ')}`})), $('actor').value || undefined);
  $('actor').hidden = !state.session.demo; $('token').hidden = state.session.demo; $('connect').hidden = state.session.demo;
  $('origin').textContent = state.session.publicOrigin;
  state.catalog = await request('/v1/admin/catalog');
  options($('pack-locale'), state.catalog.metadata.locales.map(l => ({value: l.tag, label: l.label})));
  options($('pack-namespace'), [...new Set(Object.values(state.catalog.entries).map(e => e.namespace))].sort().map(value => ({value,label:value})));
  await refresh(); loadPack();
}
document.querySelectorAll('[data-view]').forEach(el => el.addEventListener('click', () => show(el.dataset.view)));
$('ui-locale').addEventListener('change', run(async () => { state.locale = $('ui-locale').value; translate(); if (state.session) await refresh(); }));
$('actor').addEventListener('change', run(async () => { await refresh(); if (state.draft) { state.draft = await request(draftPath()); updateButtons(); } }));
$('connect').addEventListener('click', run(connect));
$('refresh').addEventListener('click', run(refresh));
$('scope').addEventListener('change', run(async () => { state.draft = null; state.body = null; await refresh(); show('overview'); }));
$('new-draft').addEventListener('click', run(async () => { await refresh(); state.draft = null; state.body = structuredClone(state.base); renderEditor(); show('editor'); }));
$('validate').addEventListener('click', run(async () => { await request(scopePath('validate'), 'POST', edited()); notice(t('valid')); }));
$('save').addEventListener('click', run(save));
$('submit').addEventListener('click', run(async () => { await save(); state.draft = await request(draftPath('submit'), 'POST', {version: state.draft.version}); renderEditor(); await refresh(); notice(t('submitted')); }));
$('approve').addEventListener('click', run(async () => { state.draft = await request(draftPath('approve'), 'POST', {hash: state.draft.hash}); renderEditor(); await refresh(); notice(t('approved')); }));
$('publish').addEventListener('click', run(async () => { await request(draftPath('publish'), 'POST', {hash: state.draft.hash, revision: state.scope.revision, ttlSeconds: Number($('ttl').value)}); state.draft = await request(draftPath()); renderEditor(); await refresh(); notice(t('published')); show('releases'); }));
$('apply-rollout').addEventListener('click', run(async () => { await request(scopePath('rollout'), 'POST', {revision: state.scope.revision, percent: Number($('percent').value)}); await refresh(); notice(t('rolloutApplied')); }));
for (const id of ['flag-market','flag-updates','refresh-interval','default-locale','channel']) $(id).addEventListener('change',run(async()=>{edited();renderEditor();}));
$('snapshot').addEventListener('change', run(async()=>{if(state.body){state.body.note=$('note').value;state.body.snapshot=JSON.parse($('snapshot').value);renderEditor();}}));
$('load-pack').addEventListener('click', run(loadPack));
$('attach-pack').addEventListener('click', run(async () => { edited(); const pack = JSON.parse($('pack').value); const packs = state.body.packs.filter(p => p.locale !== pack.locale || p.namespace !== pack.namespace); packs.push(pack); const b = {...state.body,packs}; if(pack.namespace.startsWith('plugin.'))b.snapshot.compatibility.requiredCapabilities=[...new Set([...b.snapshot.compatibility.requiredCapabilities,'plugin-messages.v1'])]; await request(scopePath('validate'), 'POST', b); state.body = b; renderEditor(); notice(t('attached')); }));
$('simulate').addEventListener('click', run(async () => { const bucket = Number($('cohort').value); if (!Number.isInteger(bucket) || bucket < 0 || bucket > 99) throw Error('Invalid cohort'); const response = await fetch(`/v1/runtime/scopes/${state.scope.id}/active/${bucket}.jws`, {credentials:'omit'}); if (!response.ok) throw Error((await response.json()).error); const compact = await response.text(); const bytes = Uint8Array.from(atob(compact.split('.')[1].replace(/-/g,'+').replace(/_/g,'/')), c=>c.charCodeAt(0)); const snapshot = JSON.parse(new TextDecoder().decode(bytes)); $('simulation').textContent = t('previewOnly')+'\n\n'+JSON.stringify(snapshot,null,2); }));
try {
  ui = await (await fetch('/admin-messages.json', {credentials:'omit'})).json();
  state.locale = ui.metadata.locales.find(l => l.tag === navigator.language || l.aliases.includes(navigator.language))?.tag ?? ui.metadata.fallbackLocale;
  options($('ui-locale'), ui.metadata.locales.map(l => ({value:l.tag,label:l.label})),state.locale); translate();
  try { await connect(); } catch (error) { $('token').hidden = false; $('connect').hidden = false; notice(error.message, true); }
} catch (error) { notice(error.message,true); }

function portalLink(snapshot, id, locale) {
 const link=snapshot.links[id]; return new URL(link.pathByLocale[locale]??link.pathByLocale[link.fallbackLocale],snapshot.origins[link.originRef]).toString();
}
function renderPortal() {
 const snapshot=state.body?.snapshot, portal=snapshot?.portal;
 $('portal-settings').hidden=!portal;
 if(!portal)return;
 options($('portal-region'),Object.entries(portal.regions).map(([value,r])=>({value,label:r.labels[state.locale]??value})),portal.regions[$('portal-region').value]?$('portal-region').value:portal.defaultRegion);
 const region=portal.regions[$('portal-region').value];
 options($('portal-locale'),snapshot.localization.enabledLocales.map(value=>({value,label:value})),snapshot.localization.enabledLocales.includes($('portal-locale').value)?$('portal-locale').value:snapshot.localization.defaultLocale);
 options($('portal-slot'),Object.keys(region.media).map(value=>({value,label:value})),region.media[$('portal-slot').value]?$('portal-slot').value:Object.keys(region.media)[0]);
 loadPortalMedia();
}
function loadPortalMedia() {
 if(!state.body?.snapshot.portal)return;
 const s=state.body.snapshot, r=s.portal.regions[$('portal-region').value], locale=$('portal-locale').value, media=r.media[$('portal-slot').value], variant=media.byLocale[locale]??media.byLocale[media.fallbackLocale];
 $('portal-entrypoints').textContent=[portalLink(s,r.portalLinkId,locale),portalLink(s,r.appLinkId,locale)].join(' → ');
 $('portal-kind').value=media.kind;$('portal-media-url').value=portalLink(s,variant.linkId,locale);$('portal-poster-url').value=variant.posterLinkId?portalLink(s,variant.posterLinkId,locale):'';
 $('portal-poster-url').closest('label').hidden=media.kind!=='video';
 $('portal-downloads').value=JSON.stringify(r.downloads.map(v=>({platform:v.platform,version:v.version,sha256:v.sha256,url:portalLink(s,v.linkId,locale)})),null,2);
 $('portal-media-preview').replaceChildren();
}
function putPortalLink(snapshot,id,source) {
 const url=new URL(source);
 if(url.protocol!=='https:'||url.username||url.password||url.search||url.hash)throw Error(t('portalInvalidUrl'));
 const originRef=id+'.origin';snapshot.origins[originRef]=url.origin;
 snapshot.links[id]={originRef,pathByLocale:Object.fromEntries(snapshot.localization.enabledLocales.map(l=>[l,url.pathname])),fallbackLocale:snapshot.localization.defaultLocale};
 return id;
}
$('portal-region').addEventListener('change',renderPortal);
$('portal-locale').addEventListener('change',loadPortalMedia);
$('portal-slot').addEventListener('change',loadPortalMedia);
$('portal-preview').addEventListener('click',run(async()=>{
 const url=new URL($('portal-media-url').value);if(url.protocol!=='https:'||url.username||url.password)throw Error(t('portalInvalidUrl'));
 const el=document.createElement($('portal-kind').value==='video'?'video':'img');el.src=url.toString();el.setAttribute('aria-label',t('portalPreview'));el.referrerPolicy='no-referrer';
 if(el.tagName==='VIDEO'){el.controls=true;el.preload='none';el.poster=$('portal-poster-url').value;}
 else el.alt=t('portalPreview');
 $('portal-media-preview').replaceChildren(el);
}));
$('portal-apply').addEventListener('click',run(async()=>{
 edited();const body=structuredClone(state.body),s=body.snapshot,id=$('portal-region').value,locale=$('portal-locale').value,slot=$('portal-slot').value,r=s.portal.regions[id],media=r.media[slot],prefix=`portal.material.${id}.${slot}.${locale}`;
 const variant={linkId:putPortalLink(s,prefix,$('portal-media-url').value)};
 if(media.kind==='video')variant.posterLinkId=putPortalLink(s,prefix+'.poster',$('portal-poster-url').value);
 media.byLocale[locale]=variant;
 await request(scopePath('validate'),'POST',body);state.body=body;renderEditor();notice(t('portalApplied'));
}));
$('portal-downloads-apply').addEventListener('click',run(async()=>{
 edited();const body=structuredClone(state.body),s=body.snapshot,id=$('portal-region').value,rows=JSON.parse($('portal-downloads').value);
 if(!Array.isArray(rows)||rows.length>32)throw Error(t('portalDownloadsHelp'));
 s.portal.regions[id].downloads=rows.map(v=>({platform:v.platform,version:v.version,sha256:v.sha256,linkId:putPortalLink(s,`portal.installer.${id}.${v.platform}`,v.url)}));
 await request(scopePath('validate'),'POST',body);state.body=body;renderEditor();notice(t('portalApplied'));
}));
