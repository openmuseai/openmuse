import { createHmac, randomBytes, randomUUID, timingSafeEqual } from "node:crypto";
import type { CompositionPlanV2, MusePlatform, PlannedFacetV2 } from "@muse/plugin-graph";

export type AttachmentRole = "runtime" | "presentation" | "control";
export interface DeviceV2 { readonly deviceId: string; readonly actorRef: string; readonly platform: MusePlatform; readonly registeredAt: number; }
export interface SessionAttachmentV2 {
  readonly attachmentId: string; readonly sessionRef: string; readonly deviceId: string; readonly role: AttachmentRole;
  readonly generation: number; readonly incarnation: string; readonly attachedAt: number; readonly stateCursor: string;
}
export interface DeviceTokenClaimsV2 { readonly sessionRef: string; readonly deviceId: string; readonly actorRef: string; readonly expiresAt: number; readonly nonce: string; }

export class DeviceTokenAuthorityV2 {
  private readonly revoked = new Set<string>();
  constructor(private readonly secret = randomBytes(32)) {}
  issue(claims: Omit<DeviceTokenClaimsV2, "nonce">): string {
    const value: DeviceTokenClaimsV2 = { ...claims, nonce: randomBytes(12).toString("base64url") };
    const payload = Buffer.from(JSON.stringify(value)).toString("base64url");
    return `${payload}.${createHmac("sha256", this.secret).update(payload).digest("base64url")}`;
  }
  verify(token: string, expectedDevice: string, now: number): DeviceTokenClaimsV2 {
    const [payload, signature, extra] = token.split(".");
    if (payload === undefined || signature === undefined || extra !== undefined || this.revoked.has(token)) throw new Error("TOKEN_INVALID");
    const expected = createHmac("sha256", this.secret).update(payload).digest();
    const actual = Buffer.from(signature, "base64url");
    if (actual.length !== expected.length || !timingSafeEqual(actual, expected)) throw new Error("TOKEN_INVALID");
    const claims = JSON.parse(Buffer.from(payload, "base64url").toString("utf8")) as DeviceTokenClaimsV2;
    if (claims.deviceId !== expectedDevice || claims.expiresAt <= now) throw new Error("TOKEN_EXPIRED_OR_DEVICE_MISMATCH");
    return claims;
  }
  revoke(token: string): void { this.revoked.add(token); }
}

export interface StreamFrameV2<T = unknown> { readonly cursor: string; readonly channel: "state" | "control" | "receipt"; readonly payload: T; readonly expiresAt?: number; }
export class ResumableStreamV2<T = unknown> {
  private next = 1; private readonly frames: StreamFrameV2<T>[] = []; private readonly acked = new Map<string, number>();
  constructor(readonly retention = 1024, readonly maxUnacked = 64) {
    if (retention < 1 || maxUnacked < 1) throw new Error("INVALID_STREAM_LIMITS");
  }
  publish(channel: StreamFrameV2["channel"], payload: T, expiresAt?: number): StreamFrameV2<T> {
    const frame: StreamFrameV2<T> = { cursor: String(this.next++), channel, payload, ...(expiresAt === undefined ? {} : { expiresAt }) };
    this.frames.push(frame); while (this.frames.length > this.retention) this.frames.shift(); return frame;
  }
  resume(consumer: string, afterCursor: string | undefined, now: number): readonly StreamFrameV2<T>[] {
    const after = afterCursor === undefined ? 0 : Number(afterCursor); const first = this.frames[0];
    if (first !== undefined && after < Number(first.cursor) - 1) throw new Error("CURSOR_EXPIRED");
    const values = this.frames.filter(value => Number(value.cursor) > after && (value.expiresAt === undefined || value.expiresAt > now));
    const ack = this.acked.get(consumer) ?? after;
    if (values.filter(value => Number(value.cursor) > ack).length > this.maxUnacked) throw new Error("BACKPRESSURE");
    return values.map(value => structuredClone(value));
  }
  ack(consumer: string, cursor: string): void {
    const value = Number(cursor); if (!Number.isSafeInteger(value) || value < (this.acked.get(consumer) ?? 0) || value >= this.next) throw new Error("ACK_INVALID");
    this.acked.set(consumer, value);
  }
}

export class SessionGatewayV2 {
  private readonly devices = new Map<string, DeviceV2>(); private readonly attachments = new Map<string, SessionAttachmentV2>();
  readonly stream = new ResumableStreamV2();
  constructor(readonly tokens = new DeviceTokenAuthorityV2()) {}
  registerDevice(device: DeviceV2): void { this.devices.set(device.deviceId, structuredClone(device)); }
  attach(token: string, role: AttachmentRole, generation: number, incarnation: string, now: number): SessionAttachmentV2 {
    const payload = token.split(".")[0]; if (payload === undefined) throw new Error("TOKEN_INVALID");
    const unsafe = JSON.parse(Buffer.from(payload, "base64url").toString("utf8")) as DeviceTokenClaimsV2;
    const claims = this.tokens.verify(token, unsafe.deviceId, now); const device = this.devices.get(claims.deviceId);
    if (device === undefined || device.actorRef !== claims.actorRef) throw new Error("DEVICE_NOT_REGISTERED");
    const value: SessionAttachmentV2 = {
      attachmentId: `attachment.${randomUUID()}`, sessionRef: claims.sessionRef, deviceId: claims.deviceId, role,
      generation, incarnation, attachedAt: now, stateCursor: "0"
    };
    this.attachments.set(value.attachmentId, value); return structuredClone(value);
  }
  detach(id: string): void { this.attachments.delete(id); }
  list(sessionRef: string): readonly SessionAttachmentV2[] { return [...this.attachments.values()].filter(value => value.sessionRef === sessionRef).sort((a, b) => a.attachmentId.localeCompare(b.attachmentId)); }
}

export interface PairedProviderGrantV2 { readonly grantRef: string; readonly deviceId: string; readonly scopeRef: string; readonly capability: string; readonly expiresAt: number; }
export class PairedProviderRouterV2 {
  private readonly grants = new Map<string, PairedProviderGrantV2>(); private readonly online = new Set<string>();
  setOnline(deviceId: string, online: boolean): void { online ? this.online.add(deviceId) : this.online.delete(deviceId); }
  grant(value: PairedProviderGrantV2): void { this.grants.set(value.grantRef, structuredClone(value)); }
  route(grantRef: string, deviceId: string, scopeRef: string, capability: string, now: number): "paired-outbound" {
    const value = this.grants.get(grantRef);
    if (value === undefined || value.deviceId !== deviceId || value.scopeRef !== scopeRef || value.capability !== capability || value.expiresAt <= now || !this.online.has(deviceId)) throw new Error("PAIRED_PROVIDER_FORBIDDEN");
    return "paired-outbound";
  }
}

export type AgentOfflineDecision = "continue-local-document" | "queue-idempotent" | "suspend-unknown" | "reject";
export const decideOffline = (input: { readonly domainLocal: boolean; readonly agentOnline: boolean; readonly operation: "document-edit" | "agent-read" | "agent-write"; readonly idempotent: boolean; readonly started: boolean }): AgentOfflineDecision => {
  if (input.operation === "document-edit" && input.domainLocal) return "continue-local-document";
  if (input.agentOnline) return input.idempotent ? "queue-idempotent" : "reject";
  if (input.started) return "suspend-unknown";
  return input.idempotent ? "queue-idempotent" : "reject";
};

export interface PlatformPlanVerdictV2 { readonly accepted: readonly PlannedFacetV2[]; readonly rejected: readonly { readonly ref: string; readonly code: string }[]; }
export const enforcePlatformPlanV2 = (plan: CompositionPlanV2, platform: MusePlatform): PlatformPlanVerdictV2 => {
  const accepted: PlannedFacetV2[] = []; const rejected: { ref: string; code: string }[] = [];
  for (const facet of plan.facets.filter(value => value.status !== "rejected")) {
    const dynamicForbidden = platform !== "desktop" && facet.runtime === "dsh-native";
    if (dynamicForbidden) rejected.push({ ref: facet.ref, code: "DYNAMIC_EXECUTION_FORBIDDEN" }); else accepted.push(facet);
  }
  return { accepted, rejected };
};

export interface ProviderCandidateV2 { readonly providerRef: string; readonly kind: "cloud" | "local" | "paired"; readonly online: boolean; readonly latencyMs: number; readonly policyAllowed: boolean; }
export const routeProviderV2 = (values: readonly ProviderCandidateV2[]): ProviderCandidateV2 | undefined =>
  [...values].filter(value => value.online && value.policyAllowed).sort((a, b) => a.latencyMs - b.latencyMs || a.providerRef.localeCompare(b.providerRef))[0];

export class RemoteGenerationV2 {
  private active: { generation: number; incarnation: string } | undefined;
  cutover(generation: number, healthy: boolean): { generation: number; incarnation: string } {
    if (!healthy) throw new Error("REMOTE_GENERATION_UNHEALTHY");
    this.active = { generation, incarnation: randomBytes(16).toString("hex") }; return { ...this.active };
  }
  assertCurrent(value: { generation: number; incarnation: string }): void {
    if (this.active?.generation !== value.generation || this.active.incarnation !== value.incarnation) throw new Error("REMOTE_HANDLE_STALE");
  }
}

export class ResourceStoreV2 {
  private readonly values = new Map<string, Uint8Array>();
  put(value: Uint8Array): string { const ref = `resource.${randomUUID()}`; this.values.set(ref, value.slice()); return ref; }
  *read(ref: string, chunkSize: number): Iterable<Uint8Array> {
    if (!Number.isInteger(chunkSize) || chunkSize < 1 || chunkSize > 1024 * 1024) throw new Error("INVALID_CHUNK_SIZE");
    const value = this.values.get(ref); if (value === undefined) throw new Error("RESOURCE_NOT_FOUND");
    for (let offset = 0; offset < value.length; offset += chunkSize) yield value.slice(offset, offset + chunkSize);
  }
}
