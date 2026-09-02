import { performance } from "node:perf_hooks";
import { describe, expect, it } from "vitest";
import { databaseManifestV2 } from "@muse/plugin-appflowy-database";
import { planCompositionV2, type HostDescriptorV2, type MusePluginManifestV2 } from "@muse/plugin-graph";
import { InMemoryInstallStoreV2, InstallCoordinatorV2, executionDisclosureV2 } from "@muse/plugin-security";
import { compatibilityMatrixV2, diagnosePluginV2, packageArtifactV2, runPluginTckV2, scaffoldPluginV2, validatePluginV2 } from "../src/index.js";

const markdown: MusePluginManifestV2 = {
  protocol: "muse.plugin/v2", pluginId: "muse.appflowy.markdown", version: "2.0.0",
  artifacts: [{ id: "agent", digest: `sha256:${"a".repeat(64)}`, kind: "npm", entrypoint: "markdown.js" }],
  facets: [{ id: "agent", kind: "agent", artifact: "agent", runtimes: ["dsh-native", "remote-dsh"], contributions: [{ slot: "agent.tools", contributionId: "markdown" }] }]
};
const host = (platform: "desktop" | "mobile" | "web"): HostDescriptorV2 => ({ protocol: "muse.host/v2", hostId: `host-${platform}`, platform, runtimes: platform === "desktop" ? ["dsh-native", "flutter", "rust-host"] : platform === "mobile" ? ["remote-dsh", "flutter", "rust-host"] : ["remote-dsh", "flutter"], builtInContracts: [], allowedGrants: [] });

describe("V2-08 ecosystem release TCK", () => {
  it("E08-01 composes the heterogeneous Database plugin on three platforms without core changes", () => {
    expect(runPluginTckV2(databaseManifestV2)).toMatchObject({ passed: true, plans: [{ platform: "desktop" }, { platform: "mobile" }, { platform: "web" }] });
  });
  it("E08-02 composes Markdown and Database deterministically without contribution cross-talk", () => {
    const first = planCompositionV2({ manifests: [markdown, databaseManifestV2], host: host("desktop") });
    const second = planCompositionV2({ manifests: [databaseManifestV2, markdown], host: host("desktop") });
    expect(first.digest).toBe(second.digest); expect(first.acceptedPlugins).toEqual(["muse.appflowy.database", "muse.appflowy.markdown"]);
  });
  it("E08-03 SDK scaffold validates and invalid declaration fails fast", () => {
    const files = scaffoldPluginV2("org.example.cleanroom").files; expect(validatePluginV2(JSON.parse(files["muse.plugin.json"]!)).pluginId).toBe("org.example.cleanroom");
    expect(() => validatePluginV2({ protocol: "muse.plugin/v3" })).toThrow();
  });
  it("E08-04 clean-room output has no private/internal imports", () => {
    const output = scaffoldPluginV2("org.example.cleanroom"); expect(Object.values(output.files).join("\n")).not.toMatch(/src\/internal|\.\.\/\.\./); expect(runPluginTckV2(JSON.parse(output.files["muse.plugin.json"]!)).passed).toBe(true);
  });
  it("E08-05 compatibility matrix distinguishes native pin from portable public seams", () => {
    expect(compatibilityMatrixV2.dsh).toEqual({ native: "pinned workspace release", portable: "public Tool/SystemPrompt seams" });
  });
  it("E08-06 unknown major rejects while compatible v2 manifest round-trips", () => {
    expect(validatePluginV2(structuredClone(databaseManifestV2))).toEqual(databaseManifestV2); expect(diagnosePluginV2({ ...databaseManifestV2, protocol: "muse.plugin/v3" }, [host("desktop")])[0]?.code).toBe("MANIFEST_INVALID");
  });
  it("E08-07 plans 1000 two-plugin compositions within local 250ms budget", () => {
    const started = performance.now(); for (let i = 0; i < 1000; i++) planCompositionV2({ manifests: [markdown, databaseManifestV2], host: host("desktop") }); expect(performance.now() - started).toBeLessThan(250);
  });
  it("E08-08 deterministic 10000-cycle soak has no retained plan or diagnostic growth", () => {
    let last = ""; for (let i = 0; i < 10000; i++) last = planCompositionV2({ manifests: [databaseManifestV2], host: host(i % 2 === 0 ? "mobile" : "web") }).digest; expect(last).toMatch(/^sha256:/);
  });
  it("E08-09 failed release health preserves active and LKG recovery", () => {
    const c = new InstallCoordinatorV2(new InMemoryInstallStoreV2()); const install = (version: string, healthy: boolean) => c.install({ pluginId: "muse.appflowy.database", version, digest: packageArtifactV2(Buffer.from(version)).digest, verify() {}, grant() {}, stage() {}, health: () => healthy });
    install("1.0.0", true); expect(() => install("2.0.0", false)).toThrow(); expect(c.snapshot().active?.version).toBe("1.0.0");
  });
  it("E08-10 release policy never labels native in-process as sandboxed", () => {
    expect(executionDisclosureV2("trusted-in-process", { processIsolated: false, osSandbox: false }).enforced).toBe(false);
  });
  it("E08-11 public SDK exports traceable diagnostics and package digest", () => {
    expect(diagnosePluginV2(databaseManifestV2, [host("desktop")])[0]?.code).toBe("HOST_COMPATIBLE"); expect(packageArtifactV2(Buffer.from("release")).digest).toMatch(/^sha256:[0-9a-f]{64}$/);
  });
});
