import { afterEach, describe, expect, it } from "vitest";
import { Context, Service } from "@deepseek-ai/cordis";
import Loader from "@deepseek-ai/cordis-plugin-loader";
import {
  DesktopPublicServicesAdapter,
  DSH_COMPATIBILITY,
  MuseCompositionService,
  MuseDiagnosticsService,
  MuseDshSupervisor,
  authorizeCarrierRequest,
  createPortableDshAdapter,
  createWebCarrierSecurity,
  materializeDshProfile,
  type MaterializedProfile,
  type MuseDshRuntimeDescriptor,
  type RuntimeLauncher,
  type RuntimeProcess
} from "../src/index.js";
import { planCompositionV2, type HostDescriptorV2, type MusePluginManifestV2 } from "@muse/plugin-graph";

const digest = `sha256:${"a".repeat(64)}` as const;
const manifest: MusePluginManifestV2 = {
  protocol: "muse.plugin/v2", pluginId: "muse.runtime.fixture", version: "2.0.0",
  artifacts: [{ id: "agent", digest, kind: "npm", entrypoint: "fixture" }],
  facets: [{ id: "agent", kind: "agent", artifact: "agent", runtimes: ["dsh-native", "remote-dsh"], provides: [{ contract: "muse.fixture", version: "1.0.0" }] }]
};
const host: HostDescriptorV2 = {
  protocol: "muse.host/v2", hostId: "runtime-test", platform: "desktop",
  runtimes: ["dsh-native"], builtInContracts: [], allowedGrants: []
};
const plan = planCompositionV2({ manifests: [manifest], host });
const descriptor = (generation = 1): MuseDshRuntimeDescriptor => ({
  protocol: "muse.dsh-runtime/v2", runtimeId: "dsh.local", generation, planDigest: plan.digest,
  incarnation: `incarnation-${generation}`, mode: "local", compatibility: DSH_COMPATIBILITY
});
const profile = materializeDshProfile({ plan, generation: 1, moduleByFacet: { "muse.runtime.fixture/agent": "@fixture/native" } });

let context: Context | undefined;
afterEach(async () => { await context?.fiber.dispose(); context = undefined; });

describe("D02-01/02 official Loader and service lifecycle", () => {
  it("loads unchanged function and Muse services through the pinned real Loader, then disposes exactly once", async () => {
    let applied = 0; let disposed = 0;
    const native = () => { applied++; return () => { disposed++; }; };
    context = new Context();
    await context.plugin(Loader);
    context.loader.internal = {
      version: "v2",
      async import(specifier: string) {
        if (specifier === "@muse/composition") return MuseCompositionService;
        if (specifier === "@muse/diagnostics") return MuseDiagnosticsService;
        if (specifier === "@fixture/native") return native;
        throw new Error(`unexpected ${specifier}`);
      }
    } as unknown as NonNullable<typeof context.loader.internal>;
    const runtimeConfig = { descriptor: descriptor(), plan };
    await context.loader.create({ name: "@muse/composition", config: runtimeConfig });
    await context.loader.create({ name: "@muse/diagnostics" });
    const nativeId = await context.loader.create({ name: "@fixture/native" });
    await context.loader.await();
    expect(applied).toBe(1);
    expect(context.museComposition.plan().digest).toBe(plan.digest);
    expect(context.museDiagnostics.snapshot()).toEqual(plan.diagnostics);
    await context.loader.remove(nativeId);
    expect(disposed).toBe(1);
  });
});

class ToolsRuntime extends Service {
  registrations = 0; disposals = 0;
  constructor(ctx: Context) { super(ctx, "tools"); }
  register(): () => void { this.registrations++; return () => { this.registrations--; this.disposals++; }; }
}
class PromptRuntime extends Service {
  registrations = 0; disposals = 0;
  constructor(ctx: Context) { super(ctx, "systemPrompt"); }
  context(): () => void { this.registrations++; return () => { this.registrations--; this.disposals++; }; }
}

describe("D02-03/04 client and portable public seams", () => {
  it("registers Tool and Prompt via public services and removes both on Fiber dispose", async () => {
    context = new Context();
    await context.plugin(MuseCompositionService, { descriptor: descriptor(), plan });
    await context.plugin(ToolsRuntime);
    await context.plugin(PromptRuntime);
    let ownerDisposed = 0;
    const fiber = await context.plugin(createPortableDshAdapter({
      name: "muse-portable-fixture", tool: { name: "fixture" },
      prompt: { order: 80, render: () => "fixture context" }, onDispose: () => { ownerDisposed++; }
    }));
    expect((context.tools as ToolsRuntime).registrations).toBe(1);
    expect((context.systemPrompt as PromptRuntime).registrations).toBe(1);
    await fiber.dispose();
    expect((context.tools as ToolsRuntime).disposals).toBe(1);
    expect((context.systemPrompt as PromptRuntime).disposals).toBe(1);
    expect(ownerDisposed).toBe(1);
  });
});

describe("D02-05 profile and Desktop public adapters", () => {
  it("materializes only planned DSH facets and refuses private Desktop API", async () => {
    expect(profile.rows.map(value => value.name)).toEqual(["@muse/dsh-runtime", "@fixture/native"]);
    let selected = "default";
    const adapter = new DesktopPublicServicesAdapter({
      current: { profileId: selected, generation: 1 },
      select: async profileId => { selected = profileId; return { restartRequired: true }; }
    }, {
      installPlugin: async () => ({ receiptId: "receipt-1", rollback: async () => undefined }),
      runPlugin: async () => ({ processId: "process-1", stop: async () => undefined })
    });
    expect((await adapter.profiles?.select("next"))?.restartRequired).toBe(true);
    expect(selected).toBe("next");
    expect((await adapter.pnpm?.installPlugin("fixture"))?.receiptId).toBe("receipt-1");
    expect(() => adapter.requirePrivate("desktopRuntime")).toThrow("PRIVATE_DESKTOP_API_DENIED");
  });
});

describe("D02-06/07 supervisor generation and stale handles", () => {
  it("health-checks staging, drains old process, and invalidates its handle", async () => {
    const lifecycle: string[] = [];
    const launcher: RuntimeLauncher = {
      start: async (_profile: MaterializedProfile, value: MuseDshRuntimeDescriptor): Promise<RuntimeProcess> => ({
        incarnation: value.incarnation, endpoint: `uds://${value.generation}`,
        health: async () => true,
        drain: async () => { lifecycle.push(`drain:${value.generation}`); },
        stop: async () => { lifecycle.push(`stop:${value.generation}`); }
      })
    };
    const supervisor = new MuseDshSupervisor(launcher);
    const first = await supervisor.stageAndCutover(profile);
    const secondProfile = { ...profile, generation: 2 };
    const second = await supervisor.stageAndCutover(secondProfile);
    expect(lifecycle).toEqual(["drain:1", "stop:1"]);
    expect(() => supervisor.assertCurrent(first)).toThrow("STALE_RUNTIME_HANDLE");
    expect(() => supervisor.assertCurrent(second)).not.toThrow();
    await supervisor.dispose();
    expect(lifecycle.slice(-2)).toEqual(["drain:2", "stop:2"]);
  });

  it("does not cut over an unhealthy staging process", async () => {
    let stopped = 0;
    const supervisor = new MuseDshSupervisor({ start: async (_profile, value) => ({
      incarnation: value.incarnation, endpoint: "uds://bad", health: async () => false,
      drain: async () => undefined, stop: async () => { stopped++; }
    }) });
    await expect(supervisor.stageAndCutover(profile)).rejects.toThrow("DSH_STAGING_UNHEALTHY");
    expect(stopped).toBe(1);
  });
});

describe("D02-08 carrier security", () => {
  it("binds an isolated loopback origin and requires both exact origin and bearer token", () => {
    const config = createWebCarrierSecurity(43121, 7);
    expect(config.origin).toBe("http://127.0.0.1:43121");
    expect(config.csp).toContain("frame-ancestors 'none'");
    expect(authorizeCarrierRequest(config, config.origin, config.bearerToken)).toBe(true);
    expect(authorizeCarrierRequest(config, "http://localhost:43121", config.bearerToken)).toBe(false);
    expect(authorizeCarrierRequest(config, config.origin, "wrong")).toBe(false);
  });
});
