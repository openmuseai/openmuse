import { createHash } from "node:crypto";

export interface MobileIdentity { token: string; deviceId: string; connectionId: string }
interface Lease { fingerprint: string; workspaceId: string; expiresAt: number; verifiedAt: number }

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

/** Cloud already validates the token + device header and workspace membership.
 * This read-only check is for the exclusive test carrier, not introspection v2. */
export async function verifyMobileWorkspace(identity: MobileIdentity, workspaceId: string): Promise<void> {
  const base = process.env.MUSE_DOCUMENT_CLOUD_URL;
  if (!base) throw new Error("CLOUD_UNAVAILABLE");
  const url = new URL("/api/muse/workspace/current", base);
  if (url.protocol !== "https:" && !(url.protocol === "http:" && ["127.0.0.1", "localhost", "host.docker.internal"].includes(url.hostname))) {
    throw new Error("CLOUD_TLS_REQUIRED");
  }
  const response = await fetch(url, { method: "POST", redirect: "error", signal: AbortSignal.timeout(10_000),
    headers: { "Content-Type": "application/json", Authorization: `Bearer ${identity.token}`, "X-Muse-Device-Id": identity.deviceId },
    body: JSON.stringify({ workspaceId }) });
  if (!response.ok) throw new Error("DEVICE_AUTH_REJECTED");
  const body = await response.json() as { code?: number; data?: { workspaceId?: string } };
  if (body.code !== 0 || body.data?.workspaceId !== workspaceId) throw new Error("SCOPE_MISMATCH");
}
