import {
  createHash, createHmac, randomBytes, randomUUID, sign, timingSafeEqual, verify,
  type KeyObject
} from "node:crypto";
import type { EffectRecordV2 } from "@muse/plugin-graph";

export type ExecutionTrust = "trusted-in-process" | "isolated-local" | "remote-sandbox" | "precompiled-native" | "declarative-only";
export type EvidenceLevel = "listed" | "declared" | "tested" | "signed" | "reviewed" | "sandboxed";

export interface ArtifactEvidenceV2 {
  readonly protocol: "muse.artifact-evidence/v2";
  readonly pluginId: string;
  readonly version: string;
  readonly digest: `sha256:${string}`;
  readonly publisherKeyId: string;
  readonly signature: string;
  readonly sbomDigest: `sha256:${string}`;
  readonly provenanceDigest: `sha256:${string}`;
  readonly tested?: boolean;
  readonly reviewed?: boolean;
  readonly sandboxEvidence?: { readonly runner: string; readonly policyDigest: `sha256:${string}` };
}

const sha256 = (value: Uint8Array | string): `sha256:${string}` =>
  `sha256:${createHash("sha256").update(value).digest("hex")}`;

export const artifactSigningStatementV2 = (evidence: Omit<ArtifactEvidenceV2, "signature" | "tested" | "reviewed" | "sandboxEvidence">): Uint8Array =>
  Buffer.from([
    evidence.protocol, evidence.pluginId, evidence.version, evidence.digest,
    evidence.publisherKeyId, evidence.sbomDigest, evidence.provenanceDigest
  ].join("\n"), "utf8");

export interface VerifiedArtifactV2 { readonly evidence: ArtifactEvidenceV2; readonly levels: readonly EvidenceLevel[]; }
export const verifyArtifactV2 = (input: {
  readonly bytes: Uint8Array;
  readonly evidence: ArtifactEvidenceV2;
  readonly publisherKeys: ReadonlyMap<string, KeyObject>;
  readonly revokedPublisherKeys?: ReadonlySet<string>;
  readonly hostSandboxEnforced?: boolean;
}): VerifiedArtifactV2 => {
  const { evidence } = input;
  if (sha256(input.bytes) !== evidence.digest) throw new Error("ARTIFACT_DIGEST_MISMATCH");
  if (input.revokedPublisherKeys?.has(evidence.publisherKeyId)) throw new Error("PUBLISHER_KEY_REVOKED");
  const key = input.publisherKeys.get(evidence.publisherKeyId);
  if (key === undefined) throw new Error("PUBLISHER_UNKNOWN");
  const unsigned = { protocol: evidence.protocol, pluginId: evidence.pluginId, version: evidence.version, digest: evidence.digest,
    publisherKeyId: evidence.publisherKeyId, sbomDigest: evidence.sbomDigest, provenanceDigest: evidence.provenanceDigest } as const;
  if (!verify(null, artifactSigningStatementV2(unsigned), key, Buffer.from(evidence.signature, "base64url"))) throw new Error("ARTIFACT_SIGNATURE_INVALID");
  const levels: EvidenceLevel[] = ["listed", "declared", "signed"];
  if (evidence.tested === true) levels.push("tested");
  if (evidence.reviewed === true) levels.push("reviewed");
  if (evidence.sandboxEvidence !== undefined && input.hostSandboxEnforced === true) levels.push("sandboxed");
  return { evidence: structuredClone(evidence), levels };
};

export interface PermissionDiffV2 { readonly added: readonly string[]; readonly removed: readonly string[]; readonly unchanged: readonly string[]; readonly requiresReapproval: boolean; }
export const diffPermissionsV2 = (previous: readonly string[], next: readonly string[]): PermissionDiffV2 => {
  const before = new Set(previous); const after = new Set(next);
  const added = [...after].filter(value => !before.has(value)).sort();
  const removed = [...before].filter(value => !after.has(value)).sort();
  const unchanged = [...after].filter(value => before.has(value)).sort();
  return { added, removed, unchanged, requiresReapproval: added.length > 0 };
};

export interface CapabilityGrantV2 {
  readonly protocol: "muse.capability-grant/v2";
  readonly grantRef: string; readonly pluginId: string; readonly facetId: string; readonly actorRef: string;
  readonly deviceId: string; readonly resourceScope: string; readonly capability: string; readonly constraints: Readonly<Record<string, unknown>>;
  readonly expiresAt: number; readonly policyVersion: string; readonly signature: string;
}
export class GrantStoreV2 {
  private readonly grants = new Map<string, CapabilityGrantV2>(); private readonly revoked = new Set<string>();
  constructor(private readonly authoritySecret = randomBytes(32)) {}
  issue(input: Omit<CapabilityGrantV2, "protocol" | "grantRef" | "signature">): CapabilityGrantV2 {
    const unsigned = { protocol: "muse.capability-grant/v2" as const, grantRef: `grant.${randomUUID()}`, ...input };
    const signature = createHmac("sha256", this.authoritySecret).update(canonical(unsigned)).digest("base64url");
    const grant = { ...unsigned, signature }; this.grants.set(grant.grantRef, grant); return structuredClone(grant);
  }
  revoke(grantRef: string): void { this.revoked.add(grantRef); }
  assert(input: { readonly grantRef: string; readonly pluginId: string; readonly facetId: string; readonly actorRef: string; readonly deviceId: string; readonly resourceScope: string; readonly capability: string; readonly policyVersion: string; readonly now: number }): CapabilityGrantV2 {
    const grant = this.grants.get(input.grantRef);
    if (grant === undefined || this.revoked.has(input.grantRef)) throw new Error("GRANT_REVOKED_OR_MISSING");
    const { signature, ...unsigned } = grant; const expected = createHmac("sha256", this.authoritySecret).update(canonical(unsigned)).digest();
    const actual = Buffer.from(signature, "base64url");
    if (actual.length !== expected.length || !timingSafeEqual(actual, expected)) throw new Error("GRANT_SIGNATURE_INVALID");
    if (grant.expiresAt <= input.now) throw new Error("GRANT_EXPIRED");
    for (const key of ["pluginId", "facetId", "actorRef", "deviceId", "resourceScope", "capability", "policyVersion"] as const)
      if (grant[key] !== input[key]) throw new Error(`GRANT_${key.toUpperCase()}_MISMATCH`);
    return structuredClone(grant);
  }
}

export interface SecretLeaseV2 { readonly leaseRef: string; readonly secretRef: string; readonly pluginId: string; readonly operation: string; readonly expiresAt: number; }
export class SecretBrokerV2 {
  private readonly secrets = new Map<string, string>(); private readonly leases = new Map<string, SecretLeaseV2>(); private readonly revoked = new Set<string>();
  register(secretRef: string, value: string): void { if (value.length < 1) throw new Error("SECRET_EMPTY"); this.secrets.set(secretRef, value); }
  lease(input: Omit<SecretLeaseV2, "leaseRef">): SecretLeaseV2 {
    if (!this.secrets.has(input.secretRef)) throw new Error("SECRET_NOT_FOUND");
    const value = { ...input, leaseRef: `lease.${randomUUID()}` }; this.leases.set(value.leaseRef, value); return structuredClone(value);
  }
  revoke(leaseRef: string): void { this.revoked.add(leaseRef); }
  proxy<T>(input: { readonly leaseRef: string; readonly pluginId: string; readonly operation: string; readonly now: number }, adapter: (secret: string) => T): T {
    const lease = this.leases.get(input.leaseRef);
    if (lease === undefined || this.revoked.has(input.leaseRef) || lease.expiresAt <= input.now || lease.pluginId !== input.pluginId || lease.operation !== input.operation) throw new Error("SECRET_LEASE_FORBIDDEN");
    return adapter(this.secrets.get(lease.secretRef)!);
  }
}

export interface ApprovalRequestV2 { readonly requestRef: string; readonly nonce: string; readonly pluginId: string; readonly operation: string; readonly target: string; readonly impactDigest: string; readonly expiresAt: number; }
export class TrustedApprovalHostV2 {
  private readonly pending = new Map<string, ApprovalRequestV2>();
  constructor(readonly trustedOrigin = "muse://trusted-approval") {}
  request(value: ApprovalRequestV2): void { this.pending.set(value.requestRef, structuredClone(value)); }
  approve(input: { readonly origin: string; readonly requestRef: string; readonly nonce: string; readonly now: number }): string {
    if (input.origin !== this.trustedOrigin) throw new Error("APPROVAL_ORIGIN_UNTRUSTED");
    const value = this.pending.get(input.requestRef);
    if (value === undefined || value.nonce !== input.nonce || value.expiresAt <= input.now) throw new Error("APPROVAL_INVALID");
    this.pending.delete(input.requestRef); return `approval.${randomUUID()}`;
  }
}

export const executionDisclosureV2 = (trust: ExecutionTrust, hostEvidence: { readonly processIsolated: boolean; readonly osSandbox: boolean }): { readonly enforced: boolean; readonly label: string } => {
  if (trust === "trusted-in-process") return { enforced: false, label: "full host process access; publisher must be trusted" };
  if (trust === "isolated-local") return hostEvidence.processIsolated && hostEvidence.osSandbox ? { enforced: true, label: "process and OS sandbox enforced" } : { enforced: false, label: "isolation declared but not proven" };
  if (trust === "remote-sandbox") return { enforced: hostEvidence.processIsolated, label: hostEvidence.processIsolated ? "remote process boundary enforced" : "remote isolation unavailable" };
  return { enforced: true, label: trust === "precompiled-native" ? "application-signing boundary" : "declarative renderer only" };
};

export type InstallStepV2 = "intent" | "verified" | "granted" | "staged" | "healthy" | "activated" | "failed";
export interface InstallWalEntryV2 { readonly sequence: number; readonly transactionRef: string; readonly pluginId: string; readonly version: string; readonly digest: string; readonly step: InstallStepV2; readonly reason?: string; }
export interface InstallSnapshotV2 { readonly nextSequence: number; readonly active?: { readonly pluginId: string; readonly version: string; readonly digest: string }; readonly lkg?: { readonly pluginId: string; readonly version: string; readonly digest: string }; readonly wal: readonly InstallWalEntryV2[]; }
export interface InstallStoreV2 { load(): InstallSnapshotV2 | undefined; save(value: InstallSnapshotV2): void; }
export class InMemoryInstallStoreV2 implements InstallStoreV2 {
  value: InstallSnapshotV2 | undefined;
  load(): InstallSnapshotV2 | undefined { return this.value === undefined ? undefined : structuredClone(this.value); }
  save(value: InstallSnapshotV2): void { this.value = structuredClone(value); }
}
export class InstallCoordinatorV2 {
  private state: InstallSnapshotV2;
  constructor(private readonly store: InstallStoreV2) { this.state = store.load() ?? { nextSequence: 1, wal: [] }; }
  install(input: { readonly pluginId: string; readonly version: string; readonly digest: string; readonly verify: () => void; readonly grant: () => void; readonly stage: () => void; readonly health: () => boolean }): InstallSnapshotV2 {
    const tx = `install.${randomUUID()}`; this.append(tx, input, "intent");
    try {
      input.verify(); this.append(tx, input, "verified"); input.grant(); this.append(tx, input, "granted");
      input.stage(); this.append(tx, input, "staged"); if (!input.health()) throw new Error("HEALTH_CHECK_FAILED");
      this.append(tx, input, "healthy");
      const target = { pluginId: input.pluginId, version: input.version, digest: input.digest };
      this.state = { ...this.state, active: target, ...(this.state.active === undefined ? {} : { lkg: this.state.active }) };
      this.append(tx, input, "activated"); return this.snapshot();
    } catch (error) {
      this.append(tx, input, "failed", error instanceof Error ? error.message : "UNKNOWN"); throw error;
    }
  }
  recover(): InstallSnapshotV2 {
    const lastByTx = new Map<string, InstallWalEntryV2>(); for (const value of this.state.wal) lastByTx.set(value.transactionRef, value);
    for (const value of lastByTx.values()) if (value.step !== "activated" && value.step !== "failed")
      this.append(value.transactionRef, value, "failed", "RECOVERED_INCOMPLETE_TRANSACTION");
    return this.snapshot();
  }
  rollback(): InstallSnapshotV2 { if (this.state.lkg === undefined) throw new Error("LKG_MISSING"); this.state = { ...this.state, active: this.state.lkg }; this.save(); return this.snapshot(); }
  snapshot(): InstallSnapshotV2 { return structuredClone(this.state); }
  private append(tx: string, input: { pluginId: string; version: string; digest: string }, step: InstallStepV2, reason?: string): void {
    const entry: InstallWalEntryV2 = { sequence: this.state.nextSequence, transactionRef: tx, pluginId: input.pluginId, version: input.version, digest: input.digest, step, ...(reason === undefined ? {} : { reason }) };
    this.state = { ...this.state, nextSequence: this.state.nextSequence + 1, wal: [...this.state.wal, entry] }; this.save();
  }
  private save(): void { this.store.save(structuredClone(this.state)); }
}

export const reconcileUninstallV2 = (effects: readonly EffectRecordV2[], cleanup: (effect: EffectRecordV2) => boolean): { readonly complete: boolean; readonly unresolved: readonly string[] } => {
  const latest = new Map<string, EffectRecordV2>(); for (const value of effects) latest.set(value.effectRef, value);
  const unresolved = [...latest.values()].filter(value => value.state === "observed" && !cleanup(value)).map(value => value.effectRef).sort();
  return { complete: unresolved.length === 0, unresolved };
};

const forbiddenAuditKeys = /(?:secret|prompt|content|body|markdown|token|password|api.?key)/i;
export class PrivacyAuditLogV2 {
  private readonly records: Readonly<Record<string, unknown>>[] = [];
  append(value: Readonly<Record<string, unknown>>): void {
    for (const [key, item] of Object.entries(value)) {
      if ((key !== "contentHash" && forbiddenAuditKeys.test(key)) || (typeof item === "string" && /sk-[A-Za-z0-9_-]{12,}/.test(item))) throw new Error("AUDIT_SENSITIVE_DATA_FORBIDDEN");
    }
    this.records.push(structuredClone(value));
  }
  all(): readonly Readonly<Record<string, unknown>>[] { return structuredClone(this.records); }
  static contentRef(value: string): { readonly contentHash: string } { return { contentHash: sha256(value) }; }
}

export const signEvidenceV2 = (unsigned: Omit<ArtifactEvidenceV2, "signature" | "tested" | "reviewed" | "sandboxEvidence">, privateKey: KeyObject): string =>
  sign(null, artifactSigningStatementV2(unsigned), privateKey).toString("base64url");

const canonical = (value: unknown): string => JSON.stringify(sortValue(value));
const sortValue = (value: unknown): unknown => Array.isArray(value) ? value.map(sortValue) : value !== null && typeof value === "object"
  ? Object.fromEntries(Object.entries(value as Record<string, unknown>).sort(([a], [b]) => a.localeCompare(b)).map(([key, item]) => [key, sortValue(item)])) : value;
