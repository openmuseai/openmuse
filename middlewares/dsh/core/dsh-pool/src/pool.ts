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
  tenantKey: string;
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

  status(): {
    ok: true;
    nodeId: string;
    queue: number;
    instances: ReadonlyArray<{
      tenantHash: string;
      state: InstanceRecord["state"];
      attachments: number;
      port: number;
    }>;
  } {
    return {
      ok: true,
      nodeId: this.options.nodeId,
      queue: this.waiters.length,
      instances: [...this.byKey.values()].map(row => ({
        tenantHash: row.tenantHash,
        state: row.state,
        attachments: row.attachments.size,
        port: row.port
      }))
    };
  }

  async open(input: SessionOpenInput): Promise<SessionOpenResult> {
    const now = this.now();
    const tenantKey = tenantKeyOf(input.accountRef, input.workspaceRef, this.options.tenantSalt);
    const tenantHash = tenantHashOf(tenantKey);
    const sessionRef = sessionRefOf(tenantKey);
    const existing = this.byKey.get(tenantKey);
    if (existing?.state === "starting") {
      existing.attachments.add(input.deviceId);
      existing.lastActiveAt = now;
      this.dropWaiters(tenantKey);
      return this.readyResult(sessionRef, existing);
    }
    if (existing === undefined) {
      const readyCount = this.occupying().length;
      const idleExists = [...this.byKey.values()].some(
        row => row.attachments.size === 0 && (row.state === "ready" || row.state === "idle")
      );
      if (readyCount >= this.options.readyQuota && !idleExists) {
        return this.enqueue(input, tenantKey, now);
      }
      const activeCount = this.occupying().filter(row => row.attachments.size > 0 && row.state !== "starting").length;
      if (activeCount >= this.options.activeQuota) {
        return this.enqueue(input, tenantKey, now);
      }
      const record = this.reserve(tenantKey, tenantHash, input, now);
      if (readyCount >= this.options.readyQuota) {
        await this.evictLru(now);
      }
      const adopted = await this.adoptOrphan(record, now);
      if (adopted !== undefined) return adopted;
      await this.startReserved(record, input.workspaceRef);
      return this.readyResult(sessionRef, record);
    }
    const attached = await this.attachIfLive(tenantKey, sessionRef, input.deviceId, now);
    if (attached !== undefined) return attached;
    return this.open(input);
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
    await this.reapDead();
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

  private occupying(): InstanceRecord[] {
    return [...this.byKey.values()].filter(
      row => row.state === "ready" || row.state === "idle" || row.state === "starting"
    );
  }

  private async attachIfLive(
    tenantKey: string,
    sessionRef: string,
    deviceId: string,
    now: number
  ): Promise<SessionOpenResult | undefined> {
    const existing = this.byKey.get(tenantKey);
    if (existing === undefined) return undefined;
    if (existing.state === "starting") {
      existing.attachments.add(deviceId);
      existing.lastActiveAt = now;
      this.dropWaiters(tenantKey);
      return this.readyResult(sessionRef, existing);
    }
    if (existing.state === "ready" || existing.state === "idle") {
      if (!(await this.alive(existing))) {
        await this.forget(existing);
        return undefined;
      }
      existing.attachments.add(deviceId);
      existing.state = "ready";
      existing.lastActiveAt = now;
      this.dropWaiters(tenantKey);
      return this.readyResult(sessionRef, existing);
    }
    if (existing.state === "suspended") {
      await this.resume(existing);
      existing.attachments.add(deviceId);
      this.dropWaiters(tenantKey);
      return this.readyResult(sessionRef, existing);
    }
    return undefined;
  }

  private reserve(
    tenantKey: string,
    tenantHash: string,
    input: SessionOpenInput,
    now: number
  ): InstanceRecord {
    this.ports += 1;
    const port = this.options.portBase + this.ports;
    const homeDir = `${this.options.homeRoot.replace(/\/+$/u, "")}/${tenantHash}/home`;
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
    this.dropWaiters(tenantKey);
    return record;
  }

  private async adoptOrphan(record: InstanceRecord, now: number): Promise<SessionOpenResult | undefined> {
    const findRunning = this.executor.findRunning;
    if (findRunning === undefined) return undefined;
    const found = await findRunning.call(this.executor, record.tenantHash);
    if (found === undefined) return undefined;
    record.port = found.port;
    record.handle = {
      instanceRef: `inst.${record.tenantHash}`,
      executor: this.executor.kind,
      port: found.port,
      startedAt: now,
      unitName: found.unitName,
      launchToken: "pending",
      nodeId: this.options.nodeId,
      ...(found.pid === undefined ? {} : { pid: found.pid })
    };
    try {
      await this.executor.waitReady(record.handle, 15_000);
      record.state = "ready";
      this.dropWaiters(record.tenantKey);
      return this.readyResult(sessionRefOf(record.tenantKey), record);
    } catch {
      delete record.handle;
      return undefined;
    }
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

  private enqueue(input: SessionOpenInput, tenantKey: string, now: number): SessionOpenResult {
    const existingIndex = this.waiters.findIndex(item => item.tenantKey === tenantKey);
    if (existingIndex >= 0) {
      const row = this.waiters[existingIndex];
      if (row !== undefined) row.deviceId = input.deviceId;
      return {
        sessionRef: sessionRefOf(tenantKey),
        queuePosition: existingIndex + 1,
        retryAfterMs: 10_000,
        nodeId: this.options.nodeId
      };
    }
    this.waiters.push({ ...input, tenantKey, enqueuedAt: now });
    return {
      sessionRef: sessionRefOf(tenantKey),
      queuePosition: this.waiters.length,
      retryAfterMs: 10_000,
      nodeId: this.options.nodeId
    };
  }

  private dropWaiters(tenantKey: string): void {
    for (let i = this.waiters.length - 1; i >= 0; i -= 1) {
      if (this.waiters[i]?.tenantKey === tenantKey) this.waiters.splice(i, 1);
    }
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

  private async startReserved(record: InstanceRecord, workspaceRef?: string): Promise<void> {
    let handle: InstanceHandle | undefined;
    try {
      await mkdir(record.homeDir, { recursive: true });
      const spec = this.specOf(record.tenantKey, record.tenantHash, record.homeDir, record.port, workspaceRef);
      handle = await this.executor.start(spec);
      record.handle = handle;
      await this.executor.waitReady(handle, 180_000);
      record.state = "ready";
    } catch (error) {
      if (handle !== undefined) await this.executor.stop(handle).catch(() => undefined);
      this.byKey.delete(record.tenantKey);
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

  private async alive(row: InstanceRecord): Promise<boolean> {
    if (row.handle === undefined) return row.state === "starting";
    try {
      return (await this.executor.inspect(row.handle)).alive;
    } catch {
      return false;
    }
  }

  private async forget(row: InstanceRecord): Promise<void> {
    if (row.handle !== undefined) await this.executor.stop(row.handle).catch(() => undefined);
    this.byKey.delete(row.tenantKey);
  }

  private async reapDead(): Promise<void> {
    for (const row of [...this.byKey.values()]) {
      if (row.state !== "ready" && row.state !== "idle") continue;
      if (await this.alive(row)) continue;
      await this.forget(row);
    }
  }

  private async drainWaiters(): Promise<void> {
    while (this.waiters.length > 0) {
      if (this.occupying().length >= this.options.readyQuota) return;
      const activeCount = this.occupying().filter(row => row.attachments.size > 0 && row.state !== "starting").length;
      if (activeCount >= this.options.activeQuota) return;
      const next = this.waiters.shift();
      if (next === undefined) return;
      if (this.byKey.has(next.tenantKey)) continue;
      await this.open(next);
      return;
    }
  }

  private now(): number {
    return this.options.now?.() ?? Date.now();
  }
}
