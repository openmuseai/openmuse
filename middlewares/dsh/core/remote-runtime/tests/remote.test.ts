import { describe, expect, it } from "vitest";
import {
  DeviceTokenAuthorityV2, PairedProviderRouterV2, RemoteGenerationV2, ResourceStoreV2,
  ResumableStreamV2, SessionGatewayV2, decideOffline, enforcePlatformPlanV2, routeProviderV2
} from "../src/index.js";
import { planCompositionV2, type HostDescriptorV2, type MusePluginManifestV2 } from "@muse/plugin-graph";

const now = 1000;
const tokens = new DeviceTokenAuthorityV2(Buffer.alloc(32, 7));
const issue = (deviceId: string, expiresAt = 2000) => tokens.issue({ sessionRef: "session.1", deviceId, actorRef: "actor.1", expiresAt });

describe("R06 remote topology", () => {
  it("R06-01 attaches distinct presentation/control/runtime roles to one Session", () => {
    const gateway = new SessionGatewayV2(tokens);
    for (const deviceId of ["desktop", "mobile", "web"]) gateway.registerDevice({ deviceId, actorRef: "actor.1", platform: deviceId === "desktop" ? "desktop" : deviceId === "mobile" ? "mobile" : "web", registeredAt: now });
    gateway.attach(issue("desktop"), "runtime", 1, "runtime.1", now);
    gateway.attach(issue("mobile"), "presentation", 1, "runtime.1", now);
    gateway.attach(issue("web"), "control", 1, "runtime.1", now);
    expect(gateway.list("session.1").map(value => value.role).sort()).toEqual(["control", "presentation", "runtime"]);
  });

  it("R06-02 keeps one Markdown plugin identity while slicing agent runtime", () => {
    const manifest: MusePluginManifestV2 = {
      protocol: "muse.plugin/v2", pluginId: "muse.appflowy.markdown", version: "2.0.0",
      artifacts: [{ id: "agent", digest: `sha256:${"a".repeat(64)}`, kind: "npm", entrypoint: "same-agent.js" }],
      facets: [{ id: "agent", kind: "agent", artifact: "agent", runtimes: ["dsh-native", "remote-dsh"] }]
    };
    const plan = (platform: "desktop" | "mobile" | "web") => planCompositionV2({ manifests: [manifest], host: {
      protocol: "muse.host/v2", hostId: `host-${platform}`, platform,
      runtimes: platform === "desktop" ? ["dsh-native"] : ["remote-dsh"], builtInContracts: [], allowedGrants: []
    } satisfies HostDescriptorV2 });
    expect([plan("desktop"), plan("mobile"), plan("web")].map(value => value.acceptedPlugins)).toEqual([["muse.appflowy.markdown"], ["muse.appflowy.markdown"], ["muse.appflowy.markdown"]]);
    expect(plan("mobile").facets[0]?.artifactDigest).toBe(plan("desktop").facets[0]?.artifactDigest);
  });

  it("R06-03 resumes ordered reliable state, drops expired control, and rejects stale cursor", () => {
    const stream = new ResumableStreamV2<string>(2, 8);
    stream.publish("control", "old-selection", 900);
    stream.publish("state", "revision-1");
    expect(stream.resume("mobile", "0", now).map(value => value.payload)).toEqual(["revision-1"]);
    stream.publish("state", "revision-2");
    expect(() => stream.resume("mobile", "0", now)).toThrow("CURSOR_EXPIRED");
  });

  it("R06-04 rejects expired, wrong-device and revoked tokens", () => {
    const valid = issue("mobile");
    expect(() => tokens.verify(valid, "web", now)).toThrow();
    expect(() => tokens.verify(issue("mobile", 900), "mobile", now)).toThrow();
    tokens.revoke(valid); expect(() => tokens.verify(valid, "mobile", now)).toThrow("TOKEN_INVALID");
  });

  it("R06-05 rolling restart invalidates old runtime incarnation", () => {
    const runtime = new RemoteGenerationV2(); const old = runtime.cutover(1, true); const current = runtime.cutover(2, true);
    expect(() => runtime.assertCurrent(old)).toThrow("REMOTE_HANDLE_STALE"); expect(() => runtime.assertCurrent(current)).not.toThrow();
  });

  it("R06-06 paired provider is outbound, device/scope/capability/expiry scoped", () => {
    const router = new PairedProviderRouterV2(); router.setOnline("mobile", true);
    router.grant({ grantRef: "grant.1", deviceId: "mobile", scopeRef: "document.1", capability: "muse.document", expiresAt: 2000 });
    expect(router.route("grant.1", "mobile", "document.1", "muse.document", now)).toBe("paired-outbound");
    expect(() => router.route("grant.1", "desktop", "document.1", "muse.document", now)).toThrow();
  });

  it("R06-07 distinguishes local document continuity from Agent suspended/queue/reject", () => {
    expect(decideOffline({ domainLocal: true, agentOnline: false, operation: "document-edit", idempotent: false, started: false })).toBe("continue-local-document");
    expect(decideOffline({ domainLocal: false, agentOnline: false, operation: "agent-write", idempotent: true, started: false })).toBe("queue-idempotent");
    expect(decideOffline({ domainLocal: false, agentOnline: false, operation: "agent-write", idempotent: true, started: true })).toBe("suspend-unknown");
  });

  it("R06-08 enforces ack backpressure and chunks large resources", () => {
    const stream = new ResumableStreamV2<string>(100, 2); stream.publish("state", "1"); stream.publish("state", "2"); stream.publish("state", "3");
    expect(() => stream.resume("slow", "0", now)).toThrow("BACKPRESSURE");
    const resources = new ResourceStoreV2(); const ref = resources.put(new Uint8Array(2500));
    expect([...resources.read(ref, 1024)].map(value => value.length)).toEqual([1024, 1024, 452]);
  });

  it("R06-09 forbids dsh-native facets on mobile/web", () => {
    const fake = { facets: [{ ref: "p/a", runtime: "dsh-native", status: "accepted" }] } as never;
    expect(enforcePlatformPlanV2(fake, "mobile").rejected).toEqual([{ ref: "p/a", code: "DYNAMIC_EXECUTION_FORBIDDEN" }]);
    expect(enforcePlatformPlanV2(fake, "web").accepted).toEqual([]);
  });

  it("R06-10 routes only online policy-allowed Providers with stable latency tie break", () => {
    expect(routeProviderV2([
      { providerRef: "cloud.z", kind: "cloud", online: true, latencyMs: 20, policyAllowed: true },
      { providerRef: "paired.a", kind: "paired", online: true, latencyMs: 20, policyAllowed: true },
      { providerRef: "local", kind: "local", online: true, latencyMs: 1, policyAllowed: false }
    ])?.providerRef).toBe("cloud.z");
  });

  it("R06-11 attachment records separate session/device/role/generation/cursor semantics", () => {
    const gateway = new SessionGatewayV2(tokens); gateway.registerDevice({ deviceId: "mobile", actorRef: "actor.1", platform: "mobile", registeredAt: now });
    const value = gateway.attach(issue("mobile"), "presentation", 7, "inc.7", now);
    expect(value).toMatchObject({ sessionRef: "session.1", deviceId: "mobile", role: "presentation", generation: 7, stateCursor: "0" });
    expect(value.attachmentId).not.toBe(value.sessionRef);
  });
});
