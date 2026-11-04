import {validatePortal} from '../../../contracts/runtime-config/v1/portal.mjs';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import Ajv from 'ajv/dist/2020.js';
import formats from 'ajv-formats';
import {validatePack, parseMessage, parseInterpolation} from './messages.mjs';

const read = path => JSON.parse(readFileSync(new URL(path, import.meta.url), 'utf8'));
const defaultRegistry = read('../../../packages/openmuse_i18n/assets/catalog.json');
const registryFile = process.env.OPENMUSE_CONFIG_MESSAGE_REGISTRY_FILE;
const productRegistry = registryFile ? JSON.parse(readFileSync(registryFile,'utf8')) : defaultRegistry;
export const registry = registryFile ? {...productRegistry,entries:{...Object.fromEntries(Object.entries(defaultRegistry.entries).filter(([_,v])=>v.namespace==='admin')),...productRegistry.entries}} : defaultRegistry;
if (!registry.metadata || !Number.isInteger(registry.metadata.sourceRevision) || registry.metadata.sourceRevision < 1 || !Array.isArray(registry.metadata.locales) || !registry.metadata.locales.length || !registry.entries) throw Error('Invalid installed message registry');
const pluginRegistryFiles=JSON.parse(process.env.OPENMUSE_CONFIG_PLUGIN_REGISTRY_FILES??'[]');
if(!Array.isArray(pluginRegistryFiles) || pluginRegistryFiles.length>32)throw Error('Invalid plugin registry files');
registry.metadata.pluginContracts??={};
for(const file of pluginRegistryFiles){
 const plugin=JSON.parse(readFileSync(file,'utf8')),namespace=plugin.metadata?.namespace;
 if(typeof namespace!=='string'||!namespace.startsWith('plugin.')||!Number.isInteger(plugin.metadata.sourceRevision)||!(/^[a-f0-9]{64}$/.test(plugin.metadata.contractHash??''))||registry.metadata.pluginContracts[namespace])throw Error('Invalid or duplicate plugin registry');
 registry.metadata.pluginContracts[namespace]={sourceRevision:plugin.metadata.sourceRevision,contractHash:plugin.metadata.contractHash};
 for(const [id,definition] of Object.entries(plugin.entries??{})){
  if(!id.startsWith(namespace+'.') || definition.namespace!==namespace || definition.format!=='interpolation.v1' || Object.hasOwn(registry.entries,id))throw Error('Invalid plugin message ownership');
  registry.entries[id]=definition;
 }
}
const registeredLocales = new Set(registry.metadata.locales.map(l=>l.tag));
if (!registeredLocales.has(registry.metadata.templateLocale) || !registeredLocales.has(registry.metadata.fallbackLocale)) throw Error('Invalid message registry defaults');
for (const definition of Object.values(registry.entries)) {
  if (!definition.namespace || !Array.isArray(definition.parameters) || !definition.messages?.[registry.metadata.templateLocale]) throw Error('Invalid registered message');
  for (const source of Object.values(definition.messages)) if ((definition.format==='interpolation.v1'?parseInterpolation:parseMessage)(source).parameters.some(p=>!definition.parameters.includes(p))) throw Error('Undeclared registered message argument');
}
const ajv = new Ajv({allErrors: true, strict: true, strictRequired: false}); formats(ajv);
const snapshotSchema = read('../../../contracts/runtime-config/v1/snapshot.schema.json');
const validateSchema = ajv.compile(snapshotSchema);
ajv.addSchema(snapshotSchema, 'snapshot.schema.json');
const bundleSchema = ajv.compile(read('../../../contracts/runtime-config/v1/deployment-bundle.schema.json'));
export function validateBundle(bundle) {
  if (!bundleSchema(bundle)) throw Error(ajv.errorsText(bundleSchema.errors));
  for (const endpoint of bundle.bootstrap.discoveryUrls) https(endpoint);
  for (const origin of bundle.bootstrap.allowedResourceOrigins) https(origin,true);
  for (const origins of Object.values(bundle.bootstrap.allowedServiceOrigins)) for (const origin of origins) https(origin,true);
  validateSnapshot(bundle.snapshot,bundle);
  if (bundle.snapshot.localization.enabledLocales.some(locale=>!registeredLocales.has(locale))) throw Error('Deployment enables a language outside the installed registry');
}
export const digest = value => createHash('sha256').update(typeof value === 'string' || Buffer.isBuffer(value) ? value : JSON.stringify(value)).digest('hex');
export const identity = s => JSON.stringify([s.productId, s.environment, s.distributionId, s.deployment.id, s.deployment.authBindingId, s.scope.dataRegion, s.scope.platform, s.scope.variantId]);
export function https(value, originOnly = false) {
  const url = new URL(value);
  if (url.protocol !== 'https:' || !url.hostname || url.username || url.password || url.search || url.hash || (originOnly && url.pathname !== '/')) throw Error('Expected an HTTPS URL without credentials, query or fragment');
  // Check raw path as URL normalization would erase dot segments.
  if (value.split('/').slice(3).some(p => ['.', '..'].includes(decodeURIComponent(p)))) throw Error('Unsafe URL path');
  return url;
}
export function safePath(value) {
  if (typeof value !== 'string' || !value.startsWith('/') || value.startsWith('//') || /[\\?#\x00-\x20]/.test(value)) throw Error('Invalid relative path');
  for (const segment of value.split('/')) {
    const decoded = decodeURIComponent(segment);
    if (decoded === '.' || decoded === '..' || /[\\/?#\x00-\x20%]/.test(decoded)) throw Error('Unsafe path segment');
  }
}
export function validateSnapshot(s, bundle) {
  if (!validateSchema(s)) throw Error(ajv.errorsText(validateSchema.errors, {separator: '; '}));
  if (s.portal && s.scope.platform !== 'website') throw Error('Portal data belongs to website scopes');
  if (s.portal && !s.compatibility.requiredCapabilities.includes('portal.v1')) throw Error('Portal requires portal.v1');
  validatePortal(s, bundle.snapshot);
  const baseline = bundle.snapshot, locales = s.localization.enabledLocales;
  if (s.settings && !s.compatibility.requiredCapabilities.includes('settings.v1')) throw Error('Runtime settings require settings.v1 capability');
  if (identity(s) !== identity(baseline) || s.authentication.sessionStorageKey !== baseline.authentication.sessionStorageKey || s.web.sessionStorageKey !== baseline.web.sessionStorageKey) throw Error('Installed deployment identity cannot be changed remotely');
  const directory = s.localization.localeCatalog ?? [];
  const dynamic = s.compatibility.requiredCapabilities.includes('messages.dynamic.v1');
  if (new Set(directory.map(v=>v.tag)).size!==directory.length || directory.some(v=>!locales.includes(v.tag) || v.tag.split('-')[0]!==v.formatLocale.split('-')[0] || /[<>\x00-\x1f]/.test(v.label))) throw Error('Invalid dynamic locale directory');
  if (!locales.includes(s.localization.defaultLocale) || locales.some(l => !baseline.localization.enabledLocales.includes(l) && !(dynamic && directory.some(v=>v.tag===l)))) throw Error('Default or enabled locale is outside the installed registry');
  if (dynamic && baseline.localization.enabledLocales.some(l=>!locales.includes(l))) throw Error('Dynamic languages must retain installed fallback languages');
  for (const [alias, locale] of Object.entries(s.localization.aliases ?? {})) if (!locales.includes(locale) || (locales.includes(alias) && alias !== locale)) throw Error('Invalid locale alias');
  for (const [id, service] of Object.entries(s.deployment.services)) {
    const base = https(service.baseUrl);
    if (['cloud','relay'].includes(id) && base.pathname !== '/') throw Error('Cloud and relay require origin-only deployment endpoints');
    if (id === 'auth' && base.pathname.split('/').filter(Boolean).some(p=>!/^[A-Za-z0-9._~-]+$/.test(p))) throw Error('Unsupported authentication gateway path');
    if (!bundle.bootstrap.allowedServiceOrigins[id]?.includes(base.origin)) throw Error(`Untrusted ${id} service origin`);
  }
  for (const origin of Object.values(s.origins)) https(origin, true);
  for (const link of Object.values(s.links)) {
    if (!Object.hasOwn(s.origins, link.originRef) || !locales.includes(link.fallbackLocale) || !Object.hasOwn(link.pathByLocale, link.fallbackLocale)) throw Error('Unresolved link origin or fallback');
    for (const [locale, path] of Object.entries(link.pathByLocale)) { if (!locales.includes(locale)) throw Error('Link language is not enabled'); safePath(path); }
  }
  for (const path of Object.values(s.web.servicePaths)) safePath(path);
  if (s.web.servicePaths.relay && s.web.servicePaths.relay !== '/') throw Error('Paired relay gateway requires the origin root');
  if (s.navigation) {
    if (!Object.hasOwn(s.navigation.routes, s.navigation.entryRouteId)) throw Error('Missing entry route');
    const paths = new Set();
    for (const route of Object.values(s.navigation.routes)) {
      safePath(route.path);
      if (paths.has(route.path)) throw Error('Duplicate navigation route');
      paths.add(route.path);
    }
  }
  if (s.issuedAt && Date.parse(s.expiresAt) <= Date.parse(s.issuedAt)) throw Error('Invalid release lifetime');
  const ids = new Set(); let total = 0;
  for (const resource of s.resources ?? []) {
    if (ids.has(resource.id)) throw Error('Duplicate resource ID'); ids.add(resource.id);
    if (!bundle.bootstrap.allowedResourceOrigins.includes(https(resource.url).origin)) throw Error('Untrusted resource origin');
    if (resource.delivery && (!['messages','pluginMessages'].includes(resource.kind) || resource.delivery!=='onDemand' || (s.localization.bundleRefs??[]).includes(resource.id))) throw Error('Invalid optional message resource');
    total += resource.maxBytes;
  }
  if (total > 4 * 1024 * 1024 || (s.localization.bundleRefs ?? []).some(id => !ids.has(id))) throw Error('Resource budget exceeded or missing language resource');
  return s;
}
export function validateDraft(body, bundle) {
  if (!body || Object.keys(body).some(k => !['snapshot', 'packs', 'note'].includes(k)) || typeof body.note !== 'string' || body.note.length > 2000 || !Array.isArray(body.packs)) throw Error('Invalid draft');
  validateSnapshot(body.snapshot, bundle);
  if (body.snapshot.sequence !== 0 || body.snapshot.issuedAt || body.snapshot.expiresAt || (body.snapshot.resources?.length ?? 0) || (body.snapshot.localization.bundleRefs?.length ?? 0)) throw Error('Release metadata and resource URLs are generated by the publisher');
  if(body.packs.some(p=>p.namespace.startsWith('plugin.')) && !body.snapshot.compatibility.requiredCapabilities.includes('plugin-messages.v1'))throw Error('Plugin packs require plugin-messages.v1');
  const ids = new Set();
  for (const pack of body.packs) {
    const localeRegistry = {...registry,metadata:{...registry.metadata,locales:[...registry.metadata.locales,...(body.snapshot.localization.localeCatalog??[])]}};
    validatePack(pack, localeRegistry);
    const key = `${pack.locale}/${pack.namespace}`;
    if (ids.has(key) || !body.snapshot.localization.enabledLocales.includes(pack.locale)) throw Error('Duplicate or disabled language package'); ids.add(key);
    if (Buffer.byteLength(JSON.stringify(pack)) > 524288) throw Error('Language package is too large');
  }
  for (const locale of body.snapshot.localization.enabledLocales) {
    if (!registeredLocales.has(locale) && !body.packs.some(p=>p.locale===locale && Object.keys(p.messages).length)) throw Error('Downloadable language has no messages');
  }
  if (body.packs.length > 32 || Buffer.byteLength(JSON.stringify(body.packs)) > 4 * 1024 * 1024) throw Error('Language resource budget exceeded');
  return body;
}
