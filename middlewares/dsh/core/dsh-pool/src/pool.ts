import { mkdir } from "node:fs/promises";
import type {
  Executor,
  InstanceHandle,
  InstanceRecord,
  InstanceSpec,
  PoolOptions,
  SessionOpenInput,
  SessionOpenResult
} from "./types.js";
import { sessionRefOf, tenantHashOf, tenantKeyOf, uidForTenant, webUrlOf } from "./tenant.js";

interface Waiter {
  accountRef: string;
  workspaceRef: string;
  deviceId: string;
  enqueuedAt: number;
}

export class InstancePool {
  private readonly byKey = new Map<string, InstanceRecord>();
  private readonly waiters: Waiter[] = [];
  private ports = 0;
  constructor(
    private readonly executor: Executor,
    private readonly options: PoolOptions
  ) {}

  snapshot(): readonly InstanceRecord[] {
    return [...this.byKey.values()];
  }

  queueLength(): number {
    return this.waiters.length;
  }

  async open(input: SessionOpenInput): Promise<SessionOpenResult> {
    const now = this.now();
    const tenantKey = tenantKeyOf(input.accountRef, input.workspaceRef, this.options.tenantSalt);
    const tenantHash = tenantHashOf(tenantKey);
    const sessionRef = sessionRefOf(tenantKey);
    const existing = this.byKey.get(tenantKey);
    if (existing !== undefined && (existing.state === "ready" || existing.state === "idle" || existing.state === "starting")) {
      existing.attachments.add(input.deviceId);
      existing.state = "ready";
      existing.lastActiveAt = now;
      return this.readyResult(sessionRef, existing);
    }
    if (existing !== undefined && existing.state === "suspended") {
      await this.resume(existing);
      existing.attachments.add(input.deviceId);
      return this.readyResult(sessionRef, existing);
    }
    const readyCount = [...this.byKey.values()].filter(row => row.state === "ready" || row.state === "idle" || row.state === "starting").length;
    const activeCount = [...this.byKey.values()].filter(row => row.attachments.size > 0 && (row.state === "ready" || row.state === "idle")).length;
    if (readyCount >= this.options.readyQuota) {
      const evicted = await this.evictLru(now);
      if (!evicted) return this.enqueue(input, now);
    }
    if (activeCount >= this.options.activeQuota) {
      return this.enqueue(input, now);
    }
    const record = await this.spawn(tenantKey, tenantHash, input, now);
    return this.readyResult(sessionRef, record);
  }

  async close(sessionRef: string, deviceId: string): Promise<{ ok: true }> {
    const row = [...this.byKey.values()].find(item => sessionRefOf(item.tenantKey) === sessionRef);
    if (row === undefined) return { ok: true };
    row.attachments.delete(deviceId);
    if (row.attachments.size === 0 && (row.state === "ready" || row.state === "idle")) {
      row.state = "idle";
      row.lastActiveAt = this.now();
    }
    return { ok: true };
  }

  async heartbeat(sessionRef: string): Promise<{ ok: true }> {
    const row = [...this.byKey.values()].find(item => sessionRefOf(item.tenantKey) === sessionRef);
    if (row !== undefined) row.lastActiveAt = this.now();
    return { ok: true };
  }

  async tick(): Promise<void> {
    const now = this.now();
    for (const row of this.byKey.values()) {
      if (row.state === "idle" && row.attachments.size === 0 && now - row.lastActiveAt >= this.options.idleTtlMs) {
        await this.suspend(row);
      }
    }
    await this.drainWaiters();
  }

  lookupPort(tenantHash: string): number | undefined {
    const row = [...this.byKey.values()].find(item => item.tenantHash === tenantHash);
    if (row === undefined) return undefined;
    if (row.state === "ready" || row.state === "idle" || row.state === "starting") return row.port;
    return undefined;
  }

  private readyResult(sessionRef: string, row: InstanceRecord): SessionOpenResult {
    const token = row.handle?.launchToken ?? "pending";
    const instanceRef = row.handle?.instanceRef;
    return {
      sessionRef,
      webUrl: webUrlOf(this.options.publicBase, row.tenantHash, token),
      expiresAt: this.now() + this.options.idleTtlMs,
      nodeId: row.nodeId,
      ...(instanceRef === undefined ? {} : { instanceRef })
    };
  }

  private enqueue(input: SessionOpenInput, now: number): SessionOpenResult {
    this.waiters.push({ ...input, enqueuedAt: now });
    return {
      sessionRef: sessionRefOf(tenantKeyOf(input.accountRef, input.workspaceRef, this.options.tenantSalt)),
      queuePosition: this.waiters.length,
      retryAfterMs: 10_000,
      nodeId: this.options.nodeId
    };
  }

  private async evictLru(now: number): Promise<boolean> {
    const idle = [...this.byKey.values()]
      .filter(row => row.attachments.size === 0 && (row.state === "ready" || row.state === "idle"))
      .sort((a, b) => a.lastActiveAt - b.lastActiveAt)[0];
    if (idle === undefined) return false;
    idle.lastActiveAt = now - this.options.idleTtlMs;
    await this.suspend(idle);
    return true;
  }

  private async spawn(
    tenantKey: string,
    tenantHash: string,
    input: SessionOpenInput,
    now: number
  ): Promise<InstanceRecord> {
    this.ports += 1;
    const port = this.options.portBase + this.ports;
    const homeDir = `${this.options.homeRoot.replace(/\/+$/u, "")}/${tenantHash}/home`;
    await mkdir(homeDir, { recursive: true });
    const spec = this.specOf(tenantKey, tenantHash, homeDir, port, input.workspaceRef);
    const record: InstanceRecord = {
      tenantKey,
      tenantHash,
      state: "starting",
      lastActiveAt: now,
      attachments: new Set([input.deviceId]),
      homeDir,
      port,
      nodeId: this.options.nodeId
    };
    this.byKey.set(tenantKey, record);
    let handle: InstanceHandle | undefined;
    try {
      handle = await this.executor.start(spec);
      record.handle = handle;
      await this.executor.waitReady(handle, 180_000);
      record.state = "ready";
      return record;
    } catch (error) {
      if (handle !== undefined) await this.executor.stop(handle).catch(() => undefined);
      this.byKey.delete(tenantKey);
      throw error;
    }
  }

  private async resume(row: InstanceRecord): Promise<void> {
    row.state = "starting";
    const spec = this.specOf(row.tenantKey, row.tenantHash, row.homeDir, row.port);
    let handle: InstanceHandle | undefined;
    try {
      handle = await this.executor.start(spec);
      row.handle = handle;
      await this.executor.waitReady(handle, 180_000);
      row.state = "ready";
      row.lastActiveAt = this.now();
    } catch (error) {
      if (handle !== undefined) await this.executor.stop(handle).catch(() => undefined);
      row.state = "suspended";
      throw error;
    }
  }

  private specOf(
    tenantKey: string,
    tenantHash: string,
    homeDir: string,
    port: number,
    workspaceRef?: string
  ): InstanceSpec {
    const workspace = workspaceRef
      ? `${homeDir}/appflowy-workspaces/${workspaceRef}`
      : `${homeDir}/appflowy-workspaces`;
    return {
      tenantKey,
      tenantHash,
      homeDir,
      port,
      memoryMaxBytes: this.options.memoryMaxBytes,
      cpuWeight: this.options.cpuWeight,
      env: {
        ...(this.options.instanceEnv ?? {}),
        MUSE_APPFLOWY_DSH_WORKSPACE: workspace
      },
      ...(this.options.assignUid ? { uid: uidForTenant(tenantKey) } : {}),
      ...(this.options.cwd ? { cwd: this.options.cwd } : {}),
      ...(this.options.envFile ? { envFile: this.options.envFile } : {})
    };
  }

  private async suspend(row: InstanceRecord): Promise<void> {
    if (row.handle !== undefined) await this.executor.stop(row.handle);
    row.state = "suspended";
    delete row.handle;
    row.attachments.clear();
  }

  private async drainWaiters(): Promise<void> {
    if (this.waiters.length === 0) return;
    const readyCount = [...this.byKey.values()].filter(row => row.state === "ready" || row.state === "idle" || row.state === "starting").length;
    const activeCount = [...this.byKey.values()].filter(row => row.attachments.size > 0).length;
    if (readyCount >= this.options.readyQuota || activeCount >= this.options.activeQuota) return;
    const next = this.waiters.shift();
    if (next === undefined) return;
    await this.open(next);
  }

  private now(): number {
    return this.options.now?.() ?? Date.now();
  }
}
