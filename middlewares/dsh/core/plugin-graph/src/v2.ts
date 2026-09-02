import { createHash } from "node:crypto";

export type MusePlatform = "desktop" | "mobile" | "web";
export type MuseRuntime = "dsh-native" | "flutter" | "rust-host" | "remote-dsh";
export type FacetKindV2 = "agent" | "presentation" | "domain" | "service";

export interface ArtifactV2 {
  readonly id: string;
  readonly digest: `sha256:${string}`;
  readonly kind: "npm" | "flutter-bundle" | "rust-builtin" | "remote";
  readonly entrypoint: string;
}

export interface ContractRequirementV2 {
  readonly contract: string;
  readonly range: string;
  readonly optional?: boolean;
  readonly cardinality?: "one" | "many" | "pipeline";
}

export interface ContractProvisionV2 {
  readonly contract: string;
  readonly version: string;
  readonly priority?: number;
}

export interface ContributionV2 {
  readonly slot: string;
  readonly contributionId: string;
  readonly priority?: number;
  readonly replaces?: readonly string[];
}

export interface FacetManifestV2 {
  readonly id: string;
  readonly kind: FacetKindV2;
  readonly artifact: string;
  readonly runtimes: readonly MuseRuntime[];
  readonly platforms?: readonly MusePlatform[];
  readonly requires?: readonly ContractRequirementV2[];
  readonly provides?: readonly ContractProvisionV2[];
  readonly contributions?: readonly ContributionV2[];
  readonly grants?: readonly string[];
}

export interface MusePluginManifestV2 {
  readonly protocol: "muse.plugin/v2";
  readonly pluginId: string;
  readonly version: string;
  readonly artifacts: readonly ArtifactV2[];
  readonly facets: readonly FacetManifestV2[];
}

export interface HostDescriptorV2 {
  readonly protocol: "muse.host/v2";
  readonly hostId: string;
  readonly platform: MusePlatform;
  readonly runtimes: readonly MuseRuntime[];
  readonly builtInContracts: readonly ContractProvisionV2[];
  readonly allowedGrants: readonly string[];
}

export interface PlannerPolicyV2 {
  readonly disabledPlugins?: readonly string[];
  readonly deniedGrants?: readonly string[];
  readonly providerSelections?: Readonly<Record<string, string>>;
}

export interface PlanDiagnosticV2 {
  readonly severity: "info" | "warning" | "error";
  readonly code: string;
  readonly subject: string;
  readonly message: string;
}

export interface PlannedFacetV2 {
  readonly ref: string;
  readonly pluginId: string;
  readonly facetId: string;
  readonly kind: FacetKindV2;
  readonly runtime: MuseRuntime;
  readonly artifactDigest: string;
  readonly status: "accepted" | "degraded" | "rejected";
  readonly requiredGrants: readonly string[];
  readonly degradationCodes: readonly string[];
}

export interface ProviderSelectionV2 {
  readonly contract: string;
  readonly cardinality: "one" | "many" | "pipeline";
  readonly providers: readonly string[];
}

export interface ResolvedContributionV2 {
  readonly slot: string;
  readonly contributionId: string;
  readonly owner: string;
  readonly status: "selected" | "replaced" | "conflict";
  readonly replacedBy?: string;
}

export interface CompositionPlanV2 {
  readonly protocol: "muse.composition-plan/v2";
  readonly canonicalization: "muse-jcs-v1";
  readonly host: { readonly hostId: string; readonly platform: MusePlatform };
  readonly acceptedPlugins: readonly string[];
  readonly rejectedPlugins: readonly string[];
  readonly facets: readonly PlannedFacetV2[];
  readonly providers: readonly ProviderSelectionV2[];
  readonly contributions: readonly ResolvedContributionV2[];
  readonly activationOrder: readonly string[];
  readonly disposeOrder: readonly string[];
  readonly diagnostics: readonly PlanDiagnosticV2[];
  readonly restartRequired: boolean;
  readonly digest: `sha256:${string}`;
}

export class ManifestValidationError extends Error {
  constructor(readonly diagnostics: readonly PlanDiagnosticV2[]) {
    super(diagnostics.map(value => `${value.code}:${value.subject}`).join(", "));
  }
}

const ID = /^[a-z0-9](?:[a-z0-9._-]*[a-z0-9])?$/;
const SEMVER = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z.-]+)?$/;
const SHA256 = /^sha256:[0-9a-f]{64}$/;

export const validateManifestV2 = (raw: unknown): MusePluginManifestV2 => {
  const diagnostics: PlanDiagnosticV2[] = [];
  if (!isRecord(raw)) throw new ManifestValidationError([error("INVALID_MANIFEST", "/", "manifest must be an object")]);
  if (raw.protocol !== "muse.plugin/v2") diagnostics.push(error("UNKNOWN_PROTOCOL_MAJOR", "/protocol", "expected muse.plugin/v2"));
  if (typeof raw.pluginId !== "string" || !ID.test(raw.pluginId)) diagnostics.push(error("INVALID_PLUGIN_ID", "/pluginId", "invalid stable plugin id"));
  if (typeof raw.version !== "string" || !SEMVER.test(raw.version)) diagnostics.push(error("INVALID_VERSION", "/version", "version must be SemVer"));
  if (!Array.isArray(raw.artifacts) || raw.artifacts.length === 0) diagnostics.push(error("MISSING_ARTIFACT", "/artifacts", "at least one artifact is required"));
  if (!Array.isArray(raw.facets) || raw.facets.length === 0) diagnostics.push(error("MISSING_FACET", "/facets", "at least one facet is required"));
  const artifacts = Array.isArray(raw.artifacts) ? raw.artifacts : [];
  const artifactIds = new Set<string>();
  for (const [index, artifact] of artifacts.entries()) {
    if (!isRecord(artifact) || typeof artifact.id !== "string" || artifactIds.has(artifact.id)) {
      diagnostics.push(error("INVALID_ARTIFACT_ID", `/artifacts/${index}`, "artifact id must be unique"));
      continue;
    }
    artifactIds.add(artifact.id);
    if (typeof artifact.digest !== "string" || !SHA256.test(artifact.digest)) diagnostics.push(error("INVALID_ARTIFACT_DIGEST", `/artifacts/${index}/digest`, "expected lowercase sha256 digest"));
    if (typeof artifact.entrypoint !== "string" || artifact.entrypoint.length === 0) diagnostics.push(error("INVALID_ENTRYPOINT", `/artifacts/${index}/entrypoint`, "entrypoint is required"));
  }
  const facetIds = new Set<string>();
  const facets = Array.isArray(raw.facets) ? raw.facets : [];
  for (const [index, facet] of facets.entries()) {
    if (!isRecord(facet) || typeof facet.id !== "string" || facetIds.has(facet.id)) {
      diagnostics.push(error("INVALID_FACET_ID", `/facets/${index}`, "facet id must be unique"));
      continue;
    }
    facetIds.add(facet.id);
    if (typeof facet.artifact !== "string" || !artifactIds.has(facet.artifact)) diagnostics.push(error("UNKNOWN_ARTIFACT", `/facets/${index}/artifact`, "facet references an unknown artifact"));
    if (!Array.isArray(facet.runtimes) || facet.runtimes.length === 0) diagnostics.push(error("MISSING_RUNTIME", `/facets/${index}/runtimes`, "facet must declare a runtime"));
  }
  if (diagnostics.length > 0) throw new ManifestValidationError(sortDiagnostics(diagnostics));
  return structuredClone(raw) as unknown as MusePluginManifestV2;
};

export interface PlannerInputV2 {
  readonly manifests: readonly unknown[];
  readonly host: HostDescriptorV2;
  readonly policy?: PlannerPolicyV2;
}

export const planCompositionV2 = (input: PlannerInputV2): CompositionPlanV2 => {
  validateHost(input.host);
  const manifests = input.manifests.map(validateManifestV2).sort(compareManifest);
  const policy = input.policy ?? {};
  const deniedGrants = new Set(policy.deniedGrants ?? []);
  const allowedGrants = new Set(input.host.allowedGrants);
  const disabled = new Set(policy.disabledPlugins ?? []);
  const diagnostics: PlanDiagnosticV2[] = [];
  const rejected = new Set<string>();
  const facetCandidates: PlannedFacetV2[] = [];
  const facetSources = new Map<string, FacetManifestV2>();

  for (const manifest of manifests) {
    if (disabled.has(manifest.pluginId)) {
      rejected.add(manifest.pluginId);
      diagnostics.push(error("PLUGIN_DISABLED", manifest.pluginId, "plugin disabled by policy"));
      continue;
    }
    const artifacts = new Map(manifest.artifacts.map(value => [value.id, value]));
    for (const facet of [...manifest.facets].sort((a, b) => a.id.localeCompare(b.id))) {
      const ref = `${manifest.pluginId}/${facet.id}`;
      const runtime = selectRuntime(facet, input.host);
      const artifact = artifacts.get(facet.artifact)!;
      const denied = [...(facet.grants ?? [])].filter(value => deniedGrants.has(value) || !allowedGrants.has(value)).sort();
      const platformAllowed = facet.platforms === undefined || facet.platforms.includes(input.host.platform);
      if (runtime === undefined || !platformAllowed || denied.length > 0) {
        facetCandidates.push({
          ref, pluginId: manifest.pluginId, facetId: facet.id, kind: facet.kind,
          runtime: runtime ?? fallbackRuntime(facet), artifactDigest: artifact.digest,
          status: "rejected", requiredGrants: sorted(facet.grants ?? []),
          degradationCodes: sorted([
            ...(runtime === undefined ? ["RUNTIME_UNAVAILABLE"] : []),
            ...(!platformAllowed ? ["PLATFORM_UNAVAILABLE"] : []),
            ...(denied.length > 0 ? ["GRANT_DENIED"] : [])
          ])
        });
        rejected.add(manifest.pluginId);
        diagnostics.push(error(denied.length > 0 ? "GRANT_DENIED" : runtime === undefined ? "RUNTIME_UNAVAILABLE" : "PLATFORM_UNAVAILABLE", ref, "required facet cannot run on this host"));
      } else {
        const planned: PlannedFacetV2 = {
          ref, pluginId: manifest.pluginId, facetId: facet.id, kind: facet.kind, runtime,
          artifactDigest: artifact.digest, status: "accepted", requiredGrants: sorted(facet.grants ?? []), degradationCodes: []
        };
        facetCandidates.push(planned);
        facetSources.set(ref, facet);
      }
    }
  }

  const hostProviders = input.host.builtInContracts.map(value => ({
    ref: `host:${input.host.hostId}/${value.contract}`, provision: value
  }));
  const pluginProviders = facetCandidates.filter(value => value.status !== "rejected").flatMap(value =>
    (facetSources.get(value.ref)?.provides ?? []).map(provision => ({ ref: value.ref, provision }))
  );
  const allProviders = [...hostProviders, ...pluginProviders];
  const providers = new Map<string, ProviderSelectionV2>();
  const degraded = new Map<string, Set<string>>();

  for (const facet of facetCandidates.filter(value => value.status !== "rejected")) {
    for (const requirement of facetSources.get(facet.ref)?.requires ?? []) {
      const eligible = allProviders.filter(value => value.provision.contract === requirement.contract && satisfies(value.provision.version, requirement.range));
      const cardinality = requirement.cardinality ?? "one";
      const selected = selectProviders(requirement.contract, cardinality, eligible, policy.providerSelections?.[requirement.contract]);
      if (selected.length === 0) {
        const code = requirement.optional ? "OPTIONAL_PROVIDER_MISSING" : "REQUIRED_PROVIDER_MISSING";
        diagnostics.push({ severity: requirement.optional ? "warning" : "error", code, subject: facet.ref, message: `${requirement.contract} ${requirement.range}` });
        if (requirement.optional) addDegrade(degraded, facet.ref, code);
        else rejected.add(facet.pluginId);
      } else {
        providers.set(`${requirement.contract}:${cardinality}`, { contract: requirement.contract, cardinality, providers: selected });
      }
    }
  }

  const facets = facetCandidates.map(value => {
    if (rejected.has(value.pluginId)) return { ...value, status: "rejected" as const };
    const codes = sorted(degraded.get(value.ref) ?? []);
    return codes.length > 0 ? { ...value, status: "degraded" as const, degradationCodes: codes } : value;
  }).sort((a, b) => a.ref.localeCompare(b.ref));

  const contributions = resolveContributions(facets, facetSources, diagnostics);
  const acceptedPlugins = manifests.map(value => value.pluginId).filter(value => !rejected.has(value));
  const rejectedPlugins = sorted(rejected);
  const activationOrder = facets.filter(value => value.status !== "rejected").map(value => value.ref).sort();
  const unsigned = {
    protocol: "muse.composition-plan/v2" as const,
    canonicalization: "muse-jcs-v1" as const,
    host: { hostId: input.host.hostId, platform: input.host.platform },
    acceptedPlugins: sorted(acceptedPlugins), rejectedPlugins,
    facets,
    providers: [...providers.values()].sort((a, b) => `${a.contract}:${a.cardinality}`.localeCompare(`${b.contract}:${b.cardinality}`)),
    contributions,
    activationOrder,
    disposeOrder: [...activationOrder].reverse(),
    diagnostics: sortDiagnostics(diagnostics),
    restartRequired: facets.some(value => value.runtime === "dsh-native" && value.status !== "rejected")
  };
  return { ...unsigned, digest: digestCanonical(unsigned) };
};

export const canonicalJson = (value: unknown): string => JSON.stringify(canonicalValue(value));
export const digestCanonical = (value: unknown): `sha256:${string}` => `sha256:${createHash("sha256").update(canonicalJson(value)).digest("hex")}`;

const canonicalValue = (value: unknown): unknown => {
  if (Array.isArray(value)) return value.map(canonicalValue);
  if (isRecord(value)) return Object.fromEntries(Object.keys(value).sort().filter(key => value[key] !== undefined).map(key => [key, canonicalValue(value[key])]));
  return value;
};

const selectRuntime = (facet: FacetManifestV2, host: HostDescriptorV2): MuseRuntime | undefined =>
  sorted(facet.runtimes.filter(value => host.runtimes.includes(value)))[0];
const fallbackRuntime = (facet: FacetManifestV2): MuseRuntime => sorted(facet.runtimes)[0] ?? "remote-dsh";

const selectProviders = (
  contract: string,
  cardinality: "one" | "many" | "pipeline",
  providers: readonly { readonly ref: string; readonly provision: ContractProvisionV2 }[],
  persisted: string | undefined
): readonly string[] => {
  const ranked = [...providers].sort((a, b) => (b.provision.priority ?? 0) - (a.provision.priority ?? 0) || a.ref.localeCompare(b.ref));
  if (persisted !== undefined) {
    const selected = ranked.find(value => value.ref === persisted);
    if (selected !== undefined) return cardinality === "one" ? [selected.ref] : [selected, ...ranked.filter(value => value !== selected)].map(value => value.ref);
  }
  if (cardinality === "one") return ranked.slice(0, 1).map(value => value.ref);
  return ranked.map(value => value.ref);
};

const resolveContributions = (
  facets: readonly PlannedFacetV2[],
  sources: ReadonlyMap<string, FacetManifestV2>,
  diagnostics: PlanDiagnosticV2[]
): readonly ResolvedContributionV2[] => {
  const values = facets.filter(value => value.status !== "rejected").flatMap(facet =>
    (sources.get(facet.ref)?.contributions ?? []).map(contribution => ({ facet, contribution }))
  ).sort((a, b) => a.contribution.slot.localeCompare(b.contribution.slot) || a.contribution.contributionId.localeCompare(b.contribution.contributionId) || a.facet.ref.localeCompare(b.facet.ref));
  const result: ResolvedContributionV2[] = [];
  for (const item of values) {
    const owner = item.facet.ref;
    const existingIndex = result.findIndex(value => value.slot === item.contribution.slot && value.contributionId === item.contribution.contributionId && value.status === "selected");
    const existing = existingIndex < 0 ? undefined : result[existingIndex];
    if (existing === undefined) {
      result.push({ slot: item.contribution.slot, contributionId: item.contribution.contributionId, owner, status: "selected" });
      continue;
    }
    const existingSource = values.find(value => value.facet.ref === existing.owner && value.contribution.contributionId === existing.contributionId)!.contribution;
    const winner = compareContribution({ owner, contribution: item.contribution }, { owner: existing.owner, contribution: existingSource }) < 0
      ? { owner, contribution: item.contribution }
      : { owner: existing.owner, contribution: existingSource };
    const loser = winner.owner === owner ? existing.owner : owner;
    if (winner.owner !== existing.owner) {
      result[existingIndex] = { ...existing, status: "replaced", replacedBy: winner.owner };
    }
    result.push({ slot: item.contribution.slot, contributionId: item.contribution.contributionId, owner: loser, status: "conflict", replacedBy: winner.owner });
    diagnostics.push({ severity: "warning", code: "CONTRIBUTION_CONFLICT", subject: `${item.contribution.slot}/${item.contribution.contributionId}`, message: `selected ${winner.owner}` });
  }
  return result.sort((a, b) => `${a.slot}/${a.contributionId}/${a.owner}`.localeCompare(`${b.slot}/${b.contributionId}/${b.owner}`));
};

const compareContribution = (a: { owner: string; contribution: ContributionV2 }, b: { owner: string; contribution: ContributionV2 }): number =>
  (b.contribution.priority ?? 0) - (a.contribution.priority ?? 0) || a.owner.localeCompare(b.owner);

export type GenerationStatus = "staging" | "active" | "draining" | "disposed" | "failed";
export interface GenerationRecordV2 {
  readonly generation: number;
  readonly planDigest: string;
  readonly status: GenerationStatus;
  readonly healthy: boolean;
  readonly failureCode?: string;
}
export interface GenerationSnapshotV2 {
  readonly nextGeneration: number;
  readonly active?: number;
  readonly lkg?: number;
  readonly records: readonly GenerationRecordV2[];
}
export interface GenerationStoreV2 {
  load(): GenerationSnapshotV2 | undefined;
  save(snapshot: GenerationSnapshotV2): void;
}

export class GenerationControllerV2 {
  private snapshot: GenerationSnapshotV2;
  constructor(private readonly store: GenerationStoreV2) {
    this.snapshot = store.load() ?? { nextGeneration: 1, records: [] };
  }
  stage(plan: CompositionPlanV2): GenerationRecordV2 {
    const record: GenerationRecordV2 = { generation: this.snapshot.nextGeneration, planDigest: plan.digest, status: "staging", healthy: false };
    this.commit({ ...this.snapshot, nextGeneration: record.generation + 1, records: [...this.snapshot.records, record] });
    return record;
  }
  markHealthy(generation: number): void { this.replace(generation, value => ({ ...value, healthy: true })); }
  activate(generation: number): void {
    const target = this.require(generation);
    if (target.status !== "staging" || !target.healthy) throw new Error("GENERATION_NOT_HEALTHY");
    const records = this.snapshot.records.map(value => value.generation === generation
      ? { ...value, status: "active" as const }
      : value.status === "active" ? { ...value, status: "draining" as const } : value);
    this.commit({ ...this.snapshot, active: generation, lkg: generation, records });
  }
  markDisposed(generation: number): void { this.replace(generation, value => ({ ...value, status: "disposed" })); }
  markFailed(generation: number, failureCode: string): void {
    this.replace(generation, value => ({ ...value, status: "failed", healthy: false, failureCode }));
  }
  recover(): GenerationRecordV2 | undefined {
    const active = this.snapshot.active === undefined ? undefined : this.snapshot.records.find(value => value.generation === this.snapshot.active);
    if (active?.status === "active" && active.healthy) return active;
    const lkg = this.snapshot.lkg === undefined ? undefined : this.snapshot.records.find(value => value.generation === this.snapshot.lkg && value.healthy);
    if (lkg === undefined) return undefined;
    const records = this.snapshot.records.map(value => value.generation === lkg.generation ? { ...value, status: "active" as const } : value);
    this.commit({ ...this.snapshot, active: lkg.generation, records });
    return records.find(value => value.generation === lkg.generation);
  }
  current(): GenerationSnapshotV2 { return structuredClone(this.snapshot); }
  private require(generation: number): GenerationRecordV2 {
    const value = this.snapshot.records.find(record => record.generation === generation);
    if (value === undefined) throw new Error("UNKNOWN_GENERATION");
    return value;
  }
  private replace(generation: number, update: (value: GenerationRecordV2) => GenerationRecordV2): void {
    this.require(generation);
    this.commit({ ...this.snapshot, records: this.snapshot.records.map(value => value.generation === generation ? update(value) : value) });
  }
  private commit(snapshot: GenerationSnapshotV2): void { this.snapshot = snapshot; this.store.save(structuredClone(snapshot)); }
}

export type EffectStateV2 = "observed" | "disposed" | "cleanup-failed";
export interface EffectRecordV2 {
  readonly sequence: number;
  readonly generation: number;
  readonly activationRef: string;
  readonly ownerRef: string;
  readonly effectRef: string;
  readonly effectKind: string;
  readonly state: EffectStateV2;
  readonly observedAt: number;
  readonly reasonCode?: string;
}
export interface EffectLedgerPortV2 {
  append(record: Omit<EffectRecordV2, "sequence">): EffectRecordV2;
  readAll(): readonly EffectRecordV2[];
}
export class InMemoryEffectLedgerV2 implements EffectLedgerPortV2 {
  private readonly records: EffectRecordV2[] = [];
  append(record: Omit<EffectRecordV2, "sequence">): EffectRecordV2 {
    const value = { ...record, sequence: this.records.length + 1 };
    this.records.push(value);
    return value;
  }
  readAll(): readonly EffectRecordV2[] { return structuredClone(this.records); }
}
export const reconcileEffectsV2 = (records: readonly EffectRecordV2[], generation: number): readonly EffectRecordV2[] => {
  const latest = new Map<string, EffectRecordV2>();
  for (const record of records.filter(value => value.generation === generation).sort((a, b) => a.sequence - b.sequence)) latest.set(record.effectRef, record);
  return [...latest.values()].filter(value => value.state === "observed" || value.state === "cleanup-failed").sort((a, b) => a.effectRef.localeCompare(b.effectRef));
};

const validateHost = (host: HostDescriptorV2): void => {
  if (host.protocol !== "muse.host/v2") throw new Error("UNKNOWN_HOST_PROTOCOL_MAJOR");
  if (!ID.test(host.hostId)) throw new Error("INVALID_HOST_ID");
};
const compareManifest = (a: MusePluginManifestV2, b: MusePluginManifestV2): number => a.pluginId.localeCompare(b.pluginId) || a.version.localeCompare(b.version);
const sorted = <T>(values: Iterable<T>): T[] => [...values].sort((a, b) => String(a).localeCompare(String(b)));
const addDegrade = (map: Map<string, Set<string>>, ref: string, code: string): void => {
  const values = map.get(ref) ?? new Set<string>(); values.add(code); map.set(ref, values);
};
const sortDiagnostics = (values: readonly PlanDiagnosticV2[]): PlanDiagnosticV2[] => [...values].sort((a, b) => `${a.severity}/${a.code}/${a.subject}/${a.message}`.localeCompare(`${b.severity}/${b.code}/${b.subject}/${b.message}`));
const error = (code: string, subject: string, message: string): PlanDiagnosticV2 => ({ severity: "error", code, subject, message });
const isRecord = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);

const parseSemver = (value: string): readonly [number, number, number] | undefined => {
  const match = SEMVER.exec(value); return match === null ? undefined : [Number(match[1]), Number(match[2]), Number(match[3])];
};
export const satisfies = (version: string, range: string): boolean => {
  const actual = parseSemver(version); if (actual === undefined) return false;
  if (range === "*" || range === ">=0.0.0") return true;
  const operator = range.startsWith("^") ? "^" : range.startsWith("~") ? "~" : range.startsWith(">=") ? ">=" : "=";
  const expected = parseSemver(range.replace(/^(\^|~|>=)/, "")); if (expected === undefined) return false;
  const compare = actual[0] - expected[0] || actual[1] - expected[1] || actual[2] - expected[2];
  if (operator === "=") return compare === 0;
  if (operator === ">=") return compare >= 0;
  if (operator === "~") return actual[0] === expected[0] && actual[1] === expected[1] && compare >= 0;
  if (expected[0] > 0) return actual[0] === expected[0] && compare >= 0;
  if (expected[1] > 0) return actual[0] === 0 && actual[1] === expected[1] && compare >= 0;
  return actual[0] === 0 && actual[1] === 0 && actual[2] === expected[2];
};
