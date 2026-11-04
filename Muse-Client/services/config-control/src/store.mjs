import {DatabaseSync} from 'node:sqlite';
import {createPublicKey, randomUUID, sign} from 'node:crypto';
import {digest, identity, validateDraft, validateSnapshot, validateBundle} from './contracts.mjs';

export class ControlError extends Error {
  constructor(status, message) { super(message); this.status = status; }
}
const fail = (status, message) => { throw new ControlError(status, message); };
const parse = value => value == null ? null : JSON.parse(value);
const clone = value => structuredClone(value);
export class ControlStore {
  constructor({database, privateKey, kid, publicOrigin, clock = () => new Date()}) {
    this.db = new DatabaseSync(database); this.privateKey = privateKey; this.kid = kid; this.publicOrigin = publicOrigin; this.clock = clock;
    this.publicKey = createPublicKey(privateKey).export({format: 'jwk'}).x;
    this.db.exec(`PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON; PRAGMA busy_timeout=5000;
      CREATE TABLE IF NOT EXISTS scopes(id TEXT PRIMARY KEY, bundle TEXT NOT NULL, sequence INTEGER NOT NULL, current TEXT, previous TEXT, percent INTEGER NOT NULL DEFAULT 100, revision INTEGER NOT NULL DEFAULT 0);
      CREATE TABLE IF NOT EXISTS drafts(id TEXT PRIMARY KEY, scope TEXT NOT NULL REFERENCES scopes(id), body TEXT NOT NULL, hash TEXT NOT NULL, version INTEGER NOT NULL, state TEXT NOT NULL, contributors TEXT NOT NULL, created TEXT NOT NULL, updated TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS approvals(draft TEXT NOT NULL REFERENCES drafts(id), hash TEXT NOT NULL, actor TEXT NOT NULL, at TEXT NOT NULL, PRIMARY KEY(draft,actor));
      CREATE TABLE IF NOT EXISTS releases(id TEXT PRIMARY KEY, scope TEXT NOT NULL REFERENCES scopes(id), sequence INTEGER NOT NULL, draft TEXT NOT NULL UNIQUE REFERENCES drafts(id), snapshot TEXT NOT NULL, jws TEXT NOT NULL, at TEXT NOT NULL, UNIQUE(scope,sequence));
      CREATE TABLE IF NOT EXISTS resources(hash TEXT PRIMARY KEY, bytes BLOB NOT NULL);
      CREATE TABLE IF NOT EXISTS audit(id INTEGER PRIMARY KEY, at TEXT NOT NULL, actor TEXT NOT NULL, action TEXT NOT NULL, scope TEXT NOT NULL, target TEXT NOT NULL, detail TEXT NOT NULL);`);
    if (!this.db.prepare('PRAGMA table_info(drafts)').all().some(column => column.name === 'base_revision')) {
      this.db.exec('ALTER TABLE drafts ADD COLUMN base_revision INTEGER NOT NULL DEFAULT 0');
    }
  }
  close() { this.db.close(); }
  transaction(fn) {
    this.db.exec('BEGIN IMMEDIATE');
    try { const result = fn(); this.db.exec('COMMIT'); return result; }
    catch (error) { this.db.exec('ROLLBACK'); throw error; }
  }
  audit(actor, action, scope, target, detail = {}) {
    this.db.prepare('INSERT INTO audit(at,actor,action,scope,target,detail) VALUES(?,?,?,?,?,?)').run(this.clock().toISOString(), actor, action, scope, target, JSON.stringify(detail));
  }
  scope(id) {
    const row = this.db.prepare('SELECT * FROM scopes WHERE id=?').get(id);
    if (!row) fail(404, 'Unknown scope'); return {...row, bundle: parse(row.bundle)};
  }
  createScope(id, bundle, actor) {
    if (!/^[a-z0-9][a-z0-9-]{0,63}$/.test(id)) fail(400, 'Invalid scope ID');
    if (this.db.prepare('SELECT id FROM scopes WHERE id=?').get(id)) fail(409, 'Scope already exists');
    const bootstrap = bundle?.bootstrap;
    if (!bootstrap || !Array.isArray(bootstrap.discoveryUrls) || !bootstrap.publicKeys || !bootstrap.allowedServiceOrigins || !Array.isArray(bootstrap.allowedResourceOrigins)) fail(400, 'Invalid installed bootstrap');
    if (bootstrap.publicKeys[this.kid] !== this.publicKey) fail(400, 'Installed bundle must trust the configured signing key');
    const discovery = `${this.publicOrigin}/v1/runtime/scopes/${id}/active.jws`;
    if (!bootstrap.discoveryUrls.includes(discovery) || !bootstrap.allowedResourceOrigins.includes(this.publicOrigin)) fail(400, 'Installed bundle must authorize this distribution origin');
    validateBundle(bundle);
    return this.transaction(() => {
      this.db.prepare('INSERT INTO scopes(id,bundle,sequence) VALUES(?,?,?)').run(id, JSON.stringify(bundle), bundle.snapshot.sequence);
      this.audit(actor, 'scope.create', id, id, {identity: identity(bundle.snapshot)}); return this.scope(id);
    });
  }
  listScopes() { return this.db.prepare('SELECT id,sequence,current,previous,percent,revision FROM scopes ORDER BY id').all(); }
  draft(id) {
    const row = this.db.prepare('SELECT * FROM drafts WHERE id=?').get(id);
    if (!row) fail(404, 'Unknown draft');
    return {...row, body: parse(row.body), contributors: parse(row.contributors), approvals: this.db.prepare('SELECT actor,hash,at FROM approvals WHERE draft=?').all(id)};
  }
  listDrafts(scope) { this.scope(scope); return this.db.prepare('SELECT id,hash,version,state,created,updated FROM drafts WHERE scope=? ORDER BY updated DESC LIMIT 100').all(scope); }
  createDraft(scope, body, actor) {
    validateDraft(body, this.scope(scope).bundle);
    return this.transaction(() => {
      const id = randomUUID(), now = this.clock().toISOString(), hash = digest(body);
      this.db.prepare('INSERT INTO drafts(id,scope,body,hash,version,state,contributors,created,updated,base_revision) VALUES(?,?,?,?,?,?,?,?,?,?)').run(id, scope, JSON.stringify(body), hash, 1, 'draft', JSON.stringify([actor]), now, now, this.scope(scope).revision);
      this.audit(actor, 'draft.create', scope, id, {hash}); return this.draft(id);
    });
  }
  updateDraft(id, version, body, actor) {
    return this.transaction(() => {
      const draft = this.draft(id);
      if (draft.version !== version || draft.state !== 'draft') fail(409, 'Draft changed or is frozen');
      validateDraft(body, this.scope(draft.scope).bundle);
      const hash = digest(body), contributors = [...new Set([...draft.contributors, actor])];
      this.db.prepare('UPDATE drafts SET body=?,hash=?,version=version+1,contributors=?,updated=? WHERE id=?').run(JSON.stringify(body), hash, JSON.stringify(contributors), this.clock().toISOString(), id);
      this.audit(actor, 'draft.update', draft.scope, id, {hash}); return this.draft(id);
    });
  }
  submit(id, version, actor) {
    return this.transaction(() => {
      const d = this.draft(id);
      if (d.version !== version || d.state !== 'draft') fail(409, 'Draft changed or is frozen');
      validateDraft(d.body, this.scope(d.scope).bundle);
      this.db.prepare("UPDATE drafts SET state='submitted',version=version+1,updated=?,base_revision=? WHERE id=?").run(this.clock().toISOString(), this.scope(d.scope).revision, id);
      this.audit(actor, 'draft.submit', d.scope, id, {hash: d.hash}); return this.draft(id);
    });
  }
  approve(id, hash, actor) {
    return this.transaction(() => {
      const d = this.draft(id);
      if (d.state !== 'submitted' || d.hash !== hash) fail(409, 'Approval must bind to the submitted content');
      if (d.base_revision !== this.scope(d.scope).revision) fail(409, 'Deployment changed; clone and review a fresh draft');
      if (d.contributors.includes(actor)) fail(403, 'Contributors cannot approve their own change');
      this.db.prepare('INSERT OR REPLACE INTO approvals VALUES(?,?,?,?)').run(id, hash, actor, this.clock().toISOString());
      this.audit(actor, 'draft.approve', d.scope, id, {hash}); return this.draft(id);
    });
  }
  publish(id, hash, expectedRevision, actor, ttlSeconds = 86400) {
    if (!Number.isInteger(ttlSeconds) || ttlSeconds < 300 || ttlSeconds > 7 * 86400) fail(400, 'Lifetime must be between 5 minutes and 7 days');
    return this.transaction(() => {
      const d = this.draft(id), scope = this.scope(d.scope);
      if (d.state === 'published') return this.release(this.db.prepare('SELECT id FROM releases WHERE draft=?').get(id).id);
      if (scope.revision !== expectedRevision || d.base_revision !== scope.revision) fail(409, 'Active pointer changed; clone and review a fresh draft');
      if (scope.bundle.bootstrap.publicKeys[this.kid] !== this.publicKey) fail(409, 'The active signer is outside the installed deployment trust policy');
      if (d.state !== 'submitted' || d.hash !== hash || !d.approvals.some(a => a.hash === hash && !d.contributors.includes(a.actor))) fail(409, 'A matching independent approval is required');
      validateDraft(d.body, scope.bundle);
      const snapshot = clone(d.body.snapshot), releaseId = randomUUID(), now = this.clock();
      snapshot.releaseId = releaseId; snapshot.sequence = scope.sequence + 1;
      snapshot.issuedAt = now.toISOString(); snapshot.expiresAt = new Date(now.getTime() + ttlSeconds * 1000).toISOString();
      snapshot.resources = d.body.packs.map(pack => {
        const bytes = Buffer.from(JSON.stringify(pack)), hash = digest(bytes);
        this.db.prepare('INSERT OR IGNORE INTO resources VALUES(?,?)').run(hash, bytes);
        return {id: `messages.${pack.locale}.${pack.namespace}`, kind: pack.namespace.startsWith('plugin.')?'pluginMessages':'messages', ...((pack.namespace.startsWith('plugin.') || !scope.bundle.snapshot.localization.enabledLocales.includes(pack.locale)) && (pack.namespace.startsWith('plugin.') || pack.locale!==snapshot.localization.defaultLocale) ? {delivery:'onDemand'} : {}), locale: pack.locale, namespace: pack.namespace, url: `${this.publicOrigin}/v1/runtime/resources/${hash}.json`, sha256: hash, maxBytes: bytes.length};
      });
      snapshot.localization.bundleRefs = snapshot.resources.filter(r=>r.delivery!=='onDemand').map(r => r.id);
      validateSnapshot(snapshot, scope.bundle);
      const header = Buffer.from(JSON.stringify({alg: 'EdDSA', typ: 'openmuse.runtime-config+json', kid: this.kid})).toString('base64url');
      const payload = Buffer.from(JSON.stringify(snapshot)).toString('base64url'), signed = `${header}.${payload}`;
      const jws = `${signed}.${sign(null, Buffer.from(signed), this.privateKey).toString('base64url')}`;
      this.db.prepare('INSERT INTO releases VALUES(?,?,?,?,?,?,?)').run(releaseId, d.scope, snapshot.sequence, id, JSON.stringify(snapshot), jws, now.toISOString());
      this.db.prepare('UPDATE scopes SET sequence=?,previous=current,current=?,percent=?,revision=revision+1 WHERE id=?').run(snapshot.sequence, releaseId, scope.current ? 1 : 100, d.scope);
      this.db.prepare("UPDATE drafts SET state='published',version=version+1,updated=? WHERE id=?").run(now.toISOString(), id);
      this.audit(actor, 'release.publish', d.scope, releaseId, {sequence: snapshot.sequence, draftHash: hash}); return this.release(releaseId);
    });
  }
  release(id) {
    const row = this.db.prepare('SELECT * FROM releases WHERE id=?').get(id); if (!row) fail(404, 'Unknown release');
    return {...row, snapshot: parse(row.snapshot)};
  }
  listReleases(scope) { this.scope(scope); return this.db.prepare('SELECT id,sequence,draft,at FROM releases WHERE scope=? ORDER BY sequence DESC LIMIT 100').all(scope); }
  rollout(id, percent, expectedRevision, actor) {
    if (![0, 1, 5, 25, 100].includes(percent)) fail(400, 'Unsupported rollout percentage');
    return this.transaction(() => {
      const s = this.scope(id);
      if (s.revision !== expectedRevision || !s.current || (!s.previous && percent !== 100)) fail(409, 'Active pointer changed or no stable fallback exists');
      if (percent < s.percent && percent !== 0) fail(409, 'Rollout can only advance or stop at zero');
      const release = this.release(s.current);
      if (percent > 0 && Date.parse(release.snapshot.expiresAt) <= this.clock().getTime()) fail(409, 'Release expired; publish a new sequence');
      this.db.prepare('UPDATE scopes SET percent=?,revision=revision+1 WHERE id=?').run(percent, id);
      this.audit(actor, 'release.rollout', id, s.current, {percent}); return this.scope(id);
    });
  }
  active(id, bucket = null) {
    const s = this.scope(id);
    if (bucket !== null && (!Number.isInteger(bucket) || bucket < 0 || bucket > 99)) fail(400, 'Invalid public cohort');
    // Clients without a cohort stay on the stable release during rollout.
    const release = s.percent === 100 || (bucket !== null && bucket < s.percent) ? s.current : s.previous;
    if (!release) fail(404, 'No release published'); return this.release(release);
  }
  resource(hash) {
    const row = this.db.prepare('SELECT bytes FROM resources WHERE hash=?').get(hash); if (!row) fail(404, 'Unknown resource'); return Buffer.from(row.bytes);
  }
  rollback(scope, releaseId, actor) {
    const release = this.release(releaseId);
    if (release.scope !== scope) fail(400, 'Release belongs to a different scope');
    const snapshot = clone(release.snapshot), packs = snapshot.resources.map(r => JSON.parse(this.resource(r.sha256)));
    delete snapshot.issuedAt; delete snapshot.expiresAt; snapshot.sequence = 0; snapshot.releaseId = 'draft'; snapshot.resources = []; snapshot.localization.bundleRefs = [];
    // A rollback is a new reviewed release; pointers never move backwards.
    return this.createDraft(scope, {snapshot, packs, note: `Rollback content from release ${releaseId}`}, actor);
  }
  template(scope) {
    const current = this.scope(scope);
    const snapshot = current.current ? clone(this.release(current.current).snapshot) : clone(current.bundle.snapshot);
    const packs = (snapshot.resources ?? []).map(r => JSON.parse(this.resource(r.sha256)));
    delete snapshot.issuedAt; delete snapshot.expiresAt; snapshot.sequence = 0; snapshot.releaseId = 'draft'; snapshot.resources = []; snapshot.localization.bundleRefs = [];
    return {snapshot,packs,note:''};
  }
  audits(scope) { this.scope(scope); return this.db.prepare('SELECT * FROM audit WHERE scope=? ORDER BY id DESC LIMIT 200').all(scope).map(r => ({...r, detail: parse(r.detail)})); }
}
