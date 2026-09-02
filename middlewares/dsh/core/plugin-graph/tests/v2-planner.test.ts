import { describe, expect, it } from "vitest";
import {
  GenerationControllerV2,
  InMemoryEffectLedgerV2,
  ManifestValidationError,
  MusePluginGraph,
  planCompositionV2,
  reconcileEffectsV2,
  satisfies,
  validateManifestV2,
  type GenerationSnapshotV2,
  type GenerationStoreV2,
  type HostDescriptorV2,
  type MusePluginManifestV2
} from "../src/index.js";

const sha = (value: string): `sha256:${string}` => `sha256:${value.repeat(64).slice(0, 64)}`;

const markdown = (): MusePluginManifestV2 => ({
  protocol: "muse.plugin/v2",
  pluginId: "muse.appflowy.markdown",
  version: "2.0.0",
  artifacts: [
    { id: "ui", digest: sha("a"), kind: "flutter-bundle", entrypoint: "MarkdownSurface" },
    { id: "domain", digest: sha("b"), kind: "rust-builtin", entrypoint: "muse.document" },
    { id: "agent", digest: sha("c"), kind: "npm", entrypoint: "index.js" }
  ],
  facets: [
    {
      id: "agent", kind: "agent", artifact: "agent", runtimes: ["remote-dsh", "dsh-native"],
      requires: [{ contract: "muse.document", range: "^2.0.0" }],
      provides: [{ contract: "muse.markdown.agent", version: "2.0.0" }]
    },
    {
      id: "domain", kind: "domain", artifact: "domain", runtimes: ["rust-host"],
      provides: [{ contract: "muse.document", version: "2.1.0", priority: 20 }]
    },
    {
      id: "ui", kind: "presentation", artifact: "ui", runtimes: ["flutter"],
      requires: [{ contract: "muse.document", range: "^2.0.0" }, { contract: "muse.selection", range: "^1.0.0", optional: true }],
      contributions: [{ slot: "workspace.editor", contributionId: "markdown", priority: 100 }]
    }
  ]
});

const host = (platform: HostDescriptorV2["platform"]): HostDescriptorV2 => ({
  protocol: "muse.host/v2",
  hostId: `muse-${platform}`,
  platform,
  runtimes: platform === "desktop" ? ["dsh-native", "flutter", "rust-host"] : ["remote-dsh", "flutter", "rust-host"],
  builtInContracts: [],
  allowedGrants: []
});

const shuffle = <T>(values: readonly T[], seed: number): T[] => {
  const output = [...values];
  let state = seed >>> 0;
  for (let index = output.length - 1; index > 0; index--) {
    state = (state * 1664525 + 1013904223) >>> 0;
    const selected = state % (index + 1);
    [output[index], output[selected]] = [output[selected]!, output[index]!];
  }
  return output;
};

describe("P01-01 manifest schema", () => {
  it("accepts v2 and rejects unknown major/missing/invalid digest structurally", () => {
    expect(validateManifestV2(markdown()).pluginId).toBe("muse.appflowy.markdown");
    for (const invalid of [
      { ...markdown(), protocol: "muse.plugin/v3" },
      { ...markdown(), facets: [] },
      { ...markdown(), artifacts: [{ ...markdown().artifacts[0]!, digest: "sha256:no" }] }
    ]) {
      expect(() => validateManifestV2(invalid)).toThrow(ManifestValidationError);
    }
  });
});

describe("P01-02 canonical property", () => {
  it("is invariant under manifest/facet/artifact input reordering for seeds 1..100", () => {
    const second: MusePluginManifestV2 = {
      ...markdown(), pluginId: "muse.auxiliary", artifacts: [{ id: "remote", digest: sha("d"), kind: "remote", entrypoint: "aux" }],
      facets: [{ id: "service", kind: "service", artifact: "remote", runtimes: ["remote-dsh", "dsh-native"], provides: [{ contract: "muse.aux", version: "1.0.0" }] }]
    };
    const baseline = planCompositionV2({ manifests: [markdown(), second], host: host("desktop") }).digest;
    for (let seed = 1; seed <= 100; seed++) {
      const reordered = [markdown(), second].map(value => ({ ...value, artifacts: shuffle(value.artifacts, seed), facets: shuffle(value.facets, seed + 100) }));
      expect(planCompositionV2({ manifests: shuffle(reordered, seed + 200), host: host("desktop") }).digest).toBe(baseline);
    }
  });
});

describe("P01-03 solver", () => {
  it("handles semver, required rejection, and optional degradation deterministically", () => {
    expect(satisfies("2.1.0", "^2.0.0")).toBe(true);
    expect(satisfies("3.0.0", "^2.0.0")).toBe(false);
    const plan = planCompositionV2({ manifests: [markdown()], host: host("desktop") });
    expect(plan.acceptedPlugins).toEqual(["muse.appflowy.markdown"]);
    expect(plan.facets.find(value => value.facetId === "ui")?.degradationCodes).toEqual(["OPTIONAL_PROVIDER_MISSING"]);
    const broken = markdown();
    const withoutDomain = { ...broken, facets: broken.facets.filter(value => value.id !== "domain") };
    const rejected = planCompositionV2({ manifests: [withoutDomain], host: host("desktop") });
    expect(rejected.rejectedPlugins).toEqual(["muse.appflowy.markdown"]);
    expect(rejected.diagnostics.some(value => value.code === "REQUIRED_PROVIDER_MISSING")).toBe(true);
  });
});

describe("P01-04 provider and contribution", () => {
  it("uses persisted selection and stable contribution priority, never discovery order", () => {
    const alternate: MusePluginManifestV2 = {
      protocol: "muse.plugin/v2", pluginId: "muse.document.alternate", version: "1.0.0",
      artifacts: [{ id: "domain", digest: sha("e"), kind: "rust-builtin", entrypoint: "alternate" }],
      facets: [{ id: "domain", kind: "domain", artifact: "domain", runtimes: ["rust-host"], provides: [{ contract: "muse.document", version: "2.2.0", priority: 1 }], contributions: [{ slot: "workspace.editor", contributionId: "markdown", priority: 1 }] }]
    };
    const selectedRef = "muse.document.alternate/domain";
    const plan = planCompositionV2({ manifests: [alternate, markdown()], host: host("desktop"), policy: { providerSelections: { "muse.document": selectedRef } } });
    expect(plan.providers.find(value => value.contract === "muse.document")?.providers).toEqual([selectedRef]);
    expect(plan.contributions.find(value => value.owner === "muse.appflowy.markdown/ui")?.status).toBe("selected");
    expect(plan.diagnostics.some(value => value.code === "CONTRIBUTION_CONFLICT")).toBe(true);
  });
});

describe("P01-05 platform slicing", () => {
  it("plans one plugin identity for desktop/mobile/web with runtime-specific agent facets", () => {
    for (const platform of ["desktop", "mobile", "web"] as const) {
      const plan = planCompositionV2({ manifests: [markdown()], host: host(platform) });
      expect(plan.acceptedPlugins).toEqual(["muse.appflowy.markdown"]);
      expect(plan.facets.find(value => value.facetId === "agent")?.runtime).toBe(platform === "desktop" ? "dsh-native" : "remote-dsh");
    }
  });
});

class MemoryGenerationStore implements GenerationStoreV2 {
  value: GenerationSnapshotV2 | undefined;
  load(): GenerationSnapshotV2 | undefined { return this.value === undefined ? undefined : structuredClone(this.value); }
  save(snapshot: GenerationSnapshotV2): void { this.value = structuredClone(snapshot); }
}

describe("P01-06/07 generation, LKG and ledger recovery", () => {
  it("stages, cuts over, drains and recovers only healthy LKG", () => {
    const store = new MemoryGenerationStore();
    const controller = new GenerationControllerV2(store);
    const first = controller.stage(planCompositionV2({ manifests: [markdown()], host: host("desktop") }));
    expect(() => controller.activate(first.generation)).toThrow("GENERATION_NOT_HEALTHY");
    controller.markHealthy(first.generation); controller.activate(first.generation);
    const second = controller.stage(planCompositionV2({ manifests: [markdown()], host: host("mobile") }));
    controller.markFailed(second.generation, "BOOT_FAILED");
    store.value = { ...store.value!, active: second.generation };
    const recovered = new GenerationControllerV2(store).recover();
    expect(recovered?.generation).toBe(first.generation);
    expect(recovered?.status).toBe("active");
  });

  it("reconciles append-only observed/disposed/cleanup-failed facts", () => {
    const ledger = new InMemoryEffectLedgerV2();
    const base = { generation: 1, activationRef: "a1", ownerRef: "p/f", effectKind: "surface", observedAt: 1 };
    ledger.append({ ...base, effectRef: "e1", state: "observed" });
    ledger.append({ ...base, effectRef: "e1", state: "disposed", observedAt: 2 });
    ledger.append({ ...base, effectRef: "e2", state: "cleanup-failed", reasonCode: "WINDOW_BUSY" });
    expect(reconcileEffectsV2(ledger.readAll(), 1).map(value => value.effectRef)).toEqual(["e2"]);
  });
});

describe("P01-08 v1 compatibility", () => {
  it("keeps the v1 activator available while v2 planner remains side-effect free", async () => {
    let activations = 0;
    const graph = new MusePluginGraph();
    await graph.activate({
      protocol: "muse.plugin-descriptor/v1", pluginId: "muse.compat", version: "1.0.0",
      facets: [{ facetKind: "agent", artifactRef: "compat", runtime: "dsh-cordis", requiresKernel: ">=1.0.0", publishes: [], consumes: [] }]
    }, { activate: async () => { activations++; return { healthy: true, dispose: () => undefined }; } });
    expect(activations).toBe(1);
    planCompositionV2({ manifests: [markdown()], host: host("desktop") });
    expect(activations).toBe(1);
  });
});
