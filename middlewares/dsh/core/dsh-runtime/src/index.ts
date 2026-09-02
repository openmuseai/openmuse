import { randomBytes } from "node:crypto";
import { Context, Service } from "@deepseek-ai/cordis";
import type { CompositionPlanV2, PlanDiagnosticV2, PlannedFacetV2 } from "@muse/plugin-graph";

export const DSH_COMPATIBILITY = {
  dsh: "0.1.0-rc.7",
  cordis: "4.0.1",
  loader: "1.0.2",
  node: "^22.19.0 || >=24.0.0"
} as const;

export interface MuseDshRuntimeDescriptor {
  readonly protocol: "muse.dsh-runtime/v2";
  readonly runtimeId: string;
  readonly generation: number;
  readonly planDigest: string;
  readonly incarnation: string;
  readonly mode: "local" | "remote";
  readonly compatibility: typeof DSH_COMPATIBILITY;
}

export interface MaterializedProfileRow {
  readonly name: string;
  readonly config?: Readonly<Record<string, unknown>>;
}

export interface ProfileMaterializationInput {
  readonly plan: CompositionPlanV2;
  readonly generation: number;
  readonly moduleByFacet: Readonly<Record<string, string>>;
  readonly nativeRows?: readonly MaterializedProfileRow[];
}

export interface MaterializedProfile {
  readonly generation: number;
  readonly planDigest: string;
  readonly rows: readonly MaterializedProfileRow[];
  readonly diagnostics: readonly PlanDiagnosticV2[];
}

export const materializeDshProfile = (input: ProfileMaterializationInput): MaterializedProfile => {
  const facets = input.plan.facets.filter(isDshFacet).sort((a, b) => a.ref.localeCompare(b.ref));
  const diagnostics: PlanDiagnosticV2[] = [];
  const rows: MaterializedProfileRow[] = [...(input.nativeRows ?? [])];
  rows.push({
    name: "@muse/dsh-runtime",
    config: { generation: input.generation, planDigest: input.plan.digest }
  });
  for (const facet of facets) {
    const name = input.moduleByFacet[facet.ref];
    if (name === undefined) {
      diagnostics.push({ severity: "error", code: "DSH_MODULE_UNRESOLVED", subject: facet.ref, message: "no DSH module mapping for planned facet" });
      continue;
    }
    rows.push({ name, config: { muse: { facetRef: facet.ref, generation: input.generation, planDigest: input.plan.digest } } });
  }
  if (diagnostics.length > 0) throw new ProfileMaterializationError(diagnostics);
  return { generation: input.generation, planDigest: input.plan.digest, rows, diagnostics };
};

export class ProfileMaterializationError extends Error {
  constructor(readonly diagnostics: readonly PlanDiagnosticV2[]) { super(diagnostics.map(value => `${value.code}:${value.subject}`).join(", ")); }
}

export interface MuseRuntimeConfig {
  readonly descriptor: MuseDshRuntimeDescriptor;
  readonly plan: CompositionPlanV2;
}

declare module "@deepseek-ai/cordis" {
  interface Context {
    museComposition: MuseCompositionService;
    museDiagnostics: MuseDiagnosticsService;
    tools?: { register(definition: unknown): () => void };
    systemPrompt?: { context(input: { name: string; order: number; text: () => string }): () => void };
  }
}

export class MuseCompositionService extends Service {
  static inject: string[] = [];
  constructor(ctx: Context, readonly config: MuseRuntimeConfig) { super(ctx, "museComposition"); }
  descriptor(): MuseDshRuntimeDescriptor { return structuredClone(this.config.descriptor); }
  plan(): CompositionPlanV2 { return structuredClone(this.config.plan); }
  slice(runtime: "dsh-native" | "remote-dsh"): readonly PlannedFacetV2[] {
    return this.config.plan.facets.filter(value => value.runtime === runtime && value.status !== "rejected").map(value => structuredClone(value));
  }
}

export class MuseDiagnosticsService extends Service {
  static inject = ["museComposition"];
  constructor(ctx: Context) { super(ctx, "museDiagnostics"); }
  snapshot(): readonly PlanDiagnosticV2[] { return this.ctx.museComposition.plan().diagnostics; }
}

export interface PortableDshContribution {
  readonly name: string;
  readonly tool?: unknown;
  readonly prompt?: { readonly order: number; readonly render: () => string };
  readonly onDispose?: () => void;
}

export const createPortableDshAdapter = (contribution: PortableDshContribution) => {
  const plugin = (ctx: Context): (() => void) => {
    const disposers: Array<() => void> = [];
    if (contribution.tool !== undefined) {
      if (ctx.tools === undefined) throw new Error("MUSE_TOOLS_UNAVAILABLE");
      disposers.push(ctx.tools.register(contribution.tool));
    }
    if (contribution.prompt !== undefined) {
      if (ctx.systemPrompt === undefined) throw new Error("MUSE_SYSTEM_PROMPT_UNAVAILABLE");
      disposers.push(ctx.systemPrompt.context({ name: contribution.name, order: contribution.prompt.order, text: contribution.prompt.render }));
    }
    return () => {
      for (const dispose of [...disposers].reverse()) dispose();
      contribution.onDispose?.();
    };
  };
  plugin.inject = [
    "museComposition",
    ...(contribution.tool === undefined ? [] : ["tools"]),
    ...(contribution.prompt === undefined ? [] : ["systemPrompt"])
  ];
  Object.defineProperty(plugin, "name", { value: contribution.name });
  return plugin;
};

export interface RuntimeProcess {
  readonly incarnation: string;
  readonly endpoint: string;
  health(): Promise<boolean>;
  drain(): Promise<void>;
  stop(): Promise<void>;
}
export interface RuntimeLauncher {
  start(profile: MaterializedProfile, descriptor: MuseDshRuntimeDescriptor): Promise<RuntimeProcess>;
}
export interface RuntimeHandle {
  readonly generation: number;
  readonly incarnation: string;
  readonly endpoint: string;
}

export class MuseDshSupervisor {
  private active: { readonly process: RuntimeProcess; readonly descriptor: MuseDshRuntimeDescriptor } | undefined;
  constructor(private readonly launcher: RuntimeLauncher) {}
  async stageAndCutover(profile: MaterializedProfile, mode: "local" | "remote" = "local"): Promise<RuntimeHandle> {
    const descriptor: MuseDshRuntimeDescriptor = {
      protocol: "muse.dsh-runtime/v2", runtimeId: `dsh.${mode}`, generation: profile.generation,
      planDigest: profile.planDigest, incarnation: randomBytes(16).toString("hex"), mode, compatibility: DSH_COMPATIBILITY
    };
    const staged = await this.launcher.start(profile, descriptor);
    if (staged.incarnation !== descriptor.incarnation || !await staged.health()) {
      await staged.stop();
      throw new Error("DSH_STAGING_UNHEALTHY");
    }
    const previous = this.active;
    this.active = { process: staged, descriptor };
    if (previous !== undefined) { await previous.process.drain(); await previous.process.stop(); }
    return { generation: descriptor.generation, incarnation: descriptor.incarnation, endpoint: staged.endpoint };
  }
  assertCurrent(handle: RuntimeHandle): void {
    if (this.active?.descriptor.generation !== handle.generation || this.active.descriptor.incarnation !== handle.incarnation) throw new Error("STALE_RUNTIME_HANDLE");
  }
  async dispose(): Promise<void> {
    const active = this.active; this.active = undefined;
    if (active !== undefined) { await active.process.drain(); await active.process.stop(); }
  }
}

export interface CarrierSecurityConfig {
  readonly origin: string;
  readonly csp: string;
  readonly bearerToken: string;
  readonly storageNamespace: string;
}
export const createWebCarrierSecurity = (port: number, generation: number): CarrierSecurityConfig => {
  if (!Number.isInteger(port) || port < 1024 || port > 65535) throw new Error("INVALID_CARRIER_PORT");
  return {
    origin: `http://127.0.0.1:${port}`,
    csp: "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self' ws://127.0.0.1:*; frame-ancestors 'none'; base-uri 'none'",
    bearerToken: randomBytes(32).toString("base64url"),
    storageNamespace: `muse-dsh-g${generation}`
  };
};
export const authorizeCarrierRequest = (config: CarrierSecurityConfig, origin: string | undefined, token: string | undefined): boolean =>
  origin === config.origin && token !== undefined && token === config.bearerToken;

export interface DesktopProfilesPublic {
  readonly current: { readonly profileId: string; readonly generation: number };
  select(profileId: string): Promise<{ readonly restartRequired: true }>;
}
export interface DesktopPnpmPublic {
  installPlugin(specifier: string): Promise<{ readonly receiptId: string; rollback(): Promise<void> }>;
  runPlugin(specifier: string): Promise<{ readonly processId: string; stop(): Promise<void> }>;
}
export class DesktopPublicServicesAdapter {
  constructor(readonly profiles?: DesktopProfilesPublic, readonly pnpm?: DesktopPnpmPublic) {}
  requirePrivate(name: string): never { throw new Error(`PRIVATE_DESKTOP_API_DENIED:${name}`); }
}

const isDshFacet = (value: PlannedFacetV2): boolean =>
  value.status !== "rejected" && (value.runtime === "dsh-native" || value.runtime === "remote-dsh");

export default MuseCompositionService;
