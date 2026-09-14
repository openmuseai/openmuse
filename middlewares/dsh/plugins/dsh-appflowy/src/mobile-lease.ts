import { createHash } from "node:crypto";

export interface MobileIdentity { token: string; deviceId: string; connectionId: string }
export interface HostIdentity { token: string; deviceId: string }
interface Lease { fingerprint: string; workspaceId: string; expiresAt: number; verifiedAt: number }

export const HOST_VERIFY_TTL_MS = 20_000;
const hostVerifyCache = new Map<string, number>();

export const resetHostVerifyCache = (): void => {
  hostVerifyCache.clear();
};

const httpCloudAllowed = (hostname: string): boolean =>
  hostname === "127.0.0.1" ||
  hostname === "localhost" ||
  hostname === "host.docker.internal" ||
  hostname.endsWith(".test") ||
  hostname.endsWith(".invalid");

const hostCacheKey = (identity: HostIdentity, workspaceId: string): string =>
  createHash("sha256").update(JSON.stringify([identity.token, identity.deviceId, workspaceId])).digest("hex");

/** Explicitly single-controller TEST carrier. Not a multi-tenant session gateway. */
export class ExclusiveMobileLease {
  private lease: Lease | undefined;
  private pending = false;
  private timer: ReturnType<typeof setTimeout> | undefined;
  constructor(private readonly deps: {
    verify(identity: MobileIdentity, workspaceId: string): Promise<void>;
    clearContext(): void;
    clock?: () => number;
  }) {}
  private now(): number { return this.deps.clock?.() ?? Date.now(); }
  private fingerprint(identity: MobileIdentity): string {
    return createHash("sha256").update(JSON.stringify([identity.token, identity.deviceId, identity.connectionId])).digest("hex");
  }
  get occupied(): boolean {
    if (this.lease !== undefined && this.lease.expiresAt <= this.now()) this.release();
    return this.pending || this.lease !== undefined;
  }
  get workspaceId(): string | undefined { return this.occupied ? this.lease?.workspaceId : undefined; }
  matches(identity: MobileIdentity): boolean {
    return this.occupied && !this.pending && this.lease?.fingerprint === this.fingerprint(identity);
  }
  async authorize(identity: MobileIdentity, workspaceId?: string): Promise<void> {
    const occupied = this.occupied;
    if (this.pending || (occupied && !this.matches(identity))) throw new Error("HOST_IN_USE");
    const scope = workspaceId ?? this.lease?.workspaceId;
    if (!scope || (this.lease !== undefined && scope !== this.lease.workspaceId)) throw new Error("SCOPE_MISMATCH");
    if (this.lease !== undefined && this.now() - this.lease.verifiedAt < 20_000) {
      this.renew(); return;
    }
    this.pending = true;
    try {
      await this.deps.verify(identity, scope);
      if (!occupied) this.deps.clearContext();
      this.lease = { fingerprint: this.fingerprint(identity), workspaceId: scope,
        expiresAt: this.now() + 60_000, verifiedAt: this.now() };
      this.renew();
    } catch {
      this.release();
      throw new Error("DEVICE_AUTH_REJECTED");
    } finally { this.pending = false; }
  }
  private renew(): void {
    if (this.lease === undefined) return;
    this.lease.expiresAt = this.now() + 60_000;
    clearTimeout(this.timer);
    this.timer = setTimeout(() => this.release(), 60_000);
    this.timer.unref();
  }
  release(): void {
    clearTimeout(this.timer);
    this.timer = undefined;
    if (this.lease !== undefined) this.deps.clearContext();
    this.lease = undefined;
  }
}

const SHARED_HOST_MAX = 8;
const SHARED_HOST_TTL_MS = 60_000;

interface SharedHost {
  fingerprint: string;
  expiresAt: number;
  verifiedAt: number;
}

/**
 * Same tenant instance, many Hosts (Web inject + N mobile connections).
 * HOST_IN_USE only means the attachment cap is full — never "Web vs Mobile mutex".
 */
export class SharedHostSession {
  private workspaceId_: string | undefined;
  private readonly hosts = new Map<string, SharedHost>();
  private pending = 0;
  private timer: ReturnType<typeof setTimeout> | undefined;
  constructor(private readonly deps: {
    verify(identity: MobileIdentity, workspaceId: string): Promise<void>;
    clock?: () => number;
  }) {}

  private now(): number { return this.deps.clock?.() ?? Date.now(); }
  private fingerprint(identity: MobileIdentity): string {
    return createHash("sha256").update(JSON.stringify([identity.token, identity.deviceId, identity.connectionId])).digest("hex");
  }
  private gc(): void {
    const now = this.now();
    for (const [key, host] of this.hosts) {
      if (host.expiresAt <= now) this.hosts.delete(key);
    }
  }
  pinWorkspace(workspaceId: string, options?: { replace?: boolean }): void {
    if (workspaceId.length === 0) return;
    if (this.workspaceId_ !== undefined && this.workspaceId_ !== workspaceId) {
      if (!options?.replace || this.hosts.size > 0) throw new Error("SCOPE_MISMATCH");
    }
    this.workspaceId_ = workspaceId;
  }
  get occupied(): boolean {
    this.gc();
    return this.pending > 0 || this.hosts.size > 0;
  }
  get workspaceId(): string | undefined {
    this.gc();
    return this.workspaceId_;
  }
  get size(): number {
    this.gc();
    return this.hosts.size;
  }
  matches(identity: MobileIdentity): boolean {
    this.gc();
    return this.hosts.has(this.fingerprint(identity));
  }
  async authorize(identity: MobileIdentity, workspaceId?: string): Promise<void> {
    this.gc();
    const scope = workspaceId ?? this.workspaceId_;
    if (!scope) throw new Error("SCOPE_MISMATCH");
    if (this.workspaceId_ !== undefined && this.workspaceId_ !== scope) throw new Error("SCOPE_MISMATCH");
    const fp = this.fingerprint(identity);
    const existing = this.hosts.get(fp);
    if (existing !== undefined && this.now() - existing.verifiedAt < HOST_VERIFY_TTL_MS) {
      this.renew(fp);
      return;
    }
    if (existing === undefined && this.hosts.size >= SHARED_HOST_MAX) throw new Error("HOST_IN_USE");
    this.pending += 1;
    if (this.workspaceId_ === undefined) this.workspaceId_ = scope;
    try {
      await this.deps.verify(identity, scope);
      this.hosts.set(fp, { fingerprint: fp, expiresAt: this.now() + SHARED_HOST_TTL_MS, verifiedAt: this.now() });
      this.renew(fp);
    } catch (error) {
      if (existing === undefined) this.hosts.delete(fp);
      if (error instanceof Error && error.message === "SCOPE_MISMATCH") throw error;
      throw new Error("DEVICE_AUTH_REJECTED");
    } finally {
      this.pending -= 1;
    }
  }
  detach(identity: MobileIdentity): void {
    this.hosts.delete(this.fingerprint(identity));
    this.armTimer();
  }
  releaseAll(): void {
    this.hosts.clear();
    this.workspaceId_ = undefined;
    this.pending = 0;
    clearTimeout(this.timer);
    this.timer = undefined;
  }
  private renew(fp: string): void {
    const host = this.hosts.get(fp);
    if (host === undefined) return;
    host.expiresAt = this.now() + SHARED_HOST_TTL_MS;
    this.armTimer();
  }
  private armTimer(): void {
    clearTimeout(this.timer);
    if (this.hosts.size === 0) {
      this.timer = undefined;
      return;
    }
    const next = Math.min(...[...this.hosts.values()].map(host => host.expiresAt)) - this.now();
    this.timer = setTimeout(() => this.gc(), Math.max(1, next));
    this.timer.unref();
  }
}

/** Cloud already validates the token + device header and workspace membership.
 * Shared by mobile exclusive lease and web parent-bridge (P0 host channel). */
export async function verifyMobileWorkspace(identity: MobileIdentity, workspaceId: string): Promise<void> {
  const base = process.env.MUSE_DOCUMENT_CLOUD_URL;
  if (!base) throw new Error("CLOUD_UNAVAILABLE");
  const url = new URL("/api/muse/workspace/current", base);
  if (url.protocol !== "https:" && !(url.protocol === "http:" && httpCloudAllowed(url.hostname))) {
    throw new Error("CLOUD_TLS_REQUIRED");
  }
  const response = await fetch(url, { method: "POST", redirect: "error", signal: AbortSignal.timeout(10_000),
    headers: { "Content-Type": "application/json", Authorization: `Bearer ${identity.token}`, "X-Muse-Device-Id": identity.deviceId },
    body: JSON.stringify({ workspaceId }) });
  if (!response.ok) throw new Error("DEVICE_AUTH_REJECTED");
  const body = await response.json() as { code?: number; data?: { workspaceId?: string } };
  if (body.code !== 0 || body.data?.workspaceId !== workspaceId) throw new Error("SCOPE_MISMATCH");
}

/** Fail-closed Cloud membership check with a 20s cache (same window as ExclusiveMobileLease). */
export async function verifyHostWorkspace(
  identity: HostIdentity,
  workspaceId: string,
  now = Date.now()
): Promise<void> {
  const key = hostCacheKey(identity, workspaceId);
  const hit = hostVerifyCache.get(key);
  if (hit !== undefined && now - hit < HOST_VERIFY_TTL_MS) return;
  await verifyMobileWorkspace(
    { token: identity.token, deviceId: identity.deviceId, connectionId: "web" },
    workspaceId
  );
  hostVerifyCache.set(key, now);
}
