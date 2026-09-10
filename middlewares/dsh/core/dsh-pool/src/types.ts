export type InstanceState = "starting" | "ready" | "idle" | "suspended" | "destroyed";
export type ExecutorKind = "local" | "systemd" | "nomad";

export interface InstanceSpec {
  readonly tenantKey: string;
  readonly tenantHash: string;
  readonly homeDir: string;
  readonly port: number;
  readonly uid?: number;
  readonly memoryMaxBytes: number;
  readonly cpuWeight: number;
  readonly env: Record<string, string>;
  readonly command?: readonly string[];
  readonly cwd?: string;
  readonly envFile?: string;
}

export interface InstanceHandle {
  readonly instanceRef: string;
  readonly executor: ExecutorKind;
  readonly port: number;
  readonly startedAt: number;
  readonly pid?: number;
  readonly unitName?: string;
  readonly allocId?: string;
  readonly launchToken?: string;
  readonly nodeId?: string;
}

export interface InspectResult {
  readonly alive: boolean;
  readonly rssBytes?: number;
  readonly memoryCurrentBytes?: number;
  readonly cpuUsageUsec?: number;
}

export interface Executor {
  readonly kind: ExecutorKind;
  start(spec: InstanceSpec): Promise<InstanceHandle>;
  stop(handle: InstanceHandle): Promise<void>;
  inspect(handle: InstanceHandle): Promise<InspectResult>;
  waitReady(handle: InstanceHandle, timeoutMs: number): Promise<void>;
}

export interface SessionOpenInput {
  readonly accountRef: string;
  readonly workspaceRef: string;
  readonly deviceId: string;
}

export interface SessionOpenResult {
  readonly sessionRef: string;
  readonly instanceRef?: string;
  readonly webUrl?: string;
  readonly expiresAt?: number;
  readonly queuePosition?: number;
  readonly retryAfterMs?: number;
  readonly nodeId: string;
}

export interface PoolOptions {
  readonly readyQuota: number;
  readonly activeQuota: number;
  readonly idleTtlMs: number;
  readonly portBase: number;
  readonly homeRoot: string;
  readonly publicBase: string;
  readonly tenantSalt: string;
  readonly nodeId: string;
  readonly memoryMaxBytes: number;
  readonly cpuWeight: number;
  readonly now?: () => number;
  /** When false/omitted, systemd-run does not pass `--uid=` (users may not exist yet). */
  readonly assignUid?: boolean;
  readonly cwd?: string;
  readonly envFile?: string;
  readonly instanceEnv?: Record<string, string>;
}

export interface InstanceRecord {
  tenantKey: string;
  tenantHash: string;
  state: InstanceState;
  handle?: InstanceHandle;
  lastActiveAt: number;
  attachments: Set<string>;
  homeDir: string;
  port: number;
  nodeId: string;
}
