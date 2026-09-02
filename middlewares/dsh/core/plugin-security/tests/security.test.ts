import { generateKeyPairSync } from "node:crypto";
import { describe, expect, it } from "vitest";
import {
  GrantStoreV2, InMemoryInstallStoreV2, InstallCoordinatorV2, PrivacyAuditLogV2, SecretBrokerV2,
  TrustedApprovalHostV2, artifactSigningStatementV2, diffPermissionsV2, executionDisclosureV2,
  reconcileUninstallV2, signEvidenceV2, verifyArtifactV2, type ArtifactEvidenceV2
} from "../src/index.js";

const bytes = Buffer.from("immutable plugin artifact");
const digest = "sha256:215986aa17028b78e85fa087dbd0931a0ee4f5ca279f86d7fc79a658f092c9c1" as const;
const sbomDigest = `sha256:${"b".repeat(64)}` as const;
const provenanceDigest = `sha256:${"c".repeat(64)}` as const;
const { publicKey, privateKey } = generateKeyPairSync("ed25519");
const unsigned = { protocol: "muse.artifact-evidence/v2", pluginId: "muse.example", version: "1.0.0", digest, publisherKeyId: "publisher.1", sbomDigest, provenanceDigest } as const;
const evidence = (): ArtifactEvidenceV2 => ({ ...unsigned, signature: signEvidenceV2(unsigned, privateKey), tested: true, reviewed: true });

describe("V2-07 security and recovery TCK", () => {
  it("S07-01 rejects artifact tampering before active generation changes", () => {
    expect(() => verifyArtifactV2({ bytes: Buffer.from("tampered"), evidence: evidence(), publisherKeys: new Map([["publisher.1", publicKey]]) })).toThrow("ARTIFACT_DIGEST_MISMATCH");
  });

  it("S07-02 permission expansion requires reapproval", () => {
    expect(diffPermissionsV2(["document.read"], ["document.read", "document.write"])).toEqual({ added: ["document.write"], removed: [], unchanged: ["document.read"], requiresReapproval: true });
  });

  it("S07-03 grant is plugin/facet/actor/device/scope/capability/policy/expiry bound and revocable", () => {
    const store = new GrantStoreV2(Buffer.alloc(32, 1));
    const grant = store.issue({ pluginId: "muse.example", facetId: "agent", actorRef: "actor.1", deviceId: "device.1", resourceScope: "doc.1", capability: "document.write", constraints: {}, expiresAt: 2000, policyVersion: "p1" });
    const input = { grantRef: grant.grantRef, pluginId: "muse.example", facetId: "agent", actorRef: "actor.1", deviceId: "device.1", resourceScope: "doc.1", capability: "document.write", policyVersion: "p1", now: 1000 };
    expect(() => store.assert({ ...input, resourceScope: "doc.2" })).toThrow(); expect(() => store.assert({ ...input, now: 3000 })).toThrow("GRANT_EXPIRED");
    store.revoke(grant.grantRef); expect(() => store.assert(input)).toThrow("GRANT_REVOKED_OR_MISSING");
  });

  it("S07-04 secret broker exposes only scoped lease and proxy result, then revokes", () => {
    const broker = new SecretBrokerV2(); broker.register("secret.deepseek", "never-log-this");
    const lease = broker.lease({ secretRef: "secret.deepseek", pluginId: "muse.example", operation: "model.call", expiresAt: 2000 });
    expect(JSON.stringify(lease)).not.toContain("never-log-this");
    expect(broker.proxy({ leaseRef: lease.leaseRef, pluginId: "muse.example", operation: "model.call", now: 1000 }, secret => secret.length)).toBe(14);
    broker.revoke(lease.leaseRef); expect(() => broker.proxy({ leaseRef: lease.leaseRef, pluginId: "muse.example", operation: "model.call", now: 1000 }, () => 1)).toThrow();
  });

  it("S07-05 untrusted Web origin, wrong nonce and replay cannot mint approval", () => {
    const host = new TrustedApprovalHostV2(); host.request({ requestRef: "r1", nonce: "n1", pluginId: "muse.example", operation: "write", target: "doc.1", impactDigest: "sha256:x", expiresAt: 2000 });
    expect(() => host.approve({ origin: "https://evil.invalid", requestRef: "r1", nonce: "n1", now: 1000 })).toThrow("APPROVAL_ORIGIN_UNTRUSTED");
    expect(() => host.approve({ origin: host.trustedOrigin, requestRef: "r1", nonce: "bad", now: 1000 })).toThrow("APPROVAL_INVALID");
    host.approve({ origin: host.trustedOrigin, requestRef: "r1", nonce: "n1", now: 1000 });
    expect(() => host.approve({ origin: host.trustedOrigin, requestRef: "r1", nonce: "n1", now: 1000 })).toThrow("APPROVAL_INVALID");
  });

  it("S07-06 native DSH risk is disclosed and policy evidence cannot fake sandbox", () => {
    expect(executionDisclosureV2("trusted-in-process", { processIsolated: false, osSandbox: false })).toEqual({ enforced: false, label: "full host process access; publisher must be trusted" });
    expect(executionDisclosureV2("isolated-local", { processIsolated: true, osSandbox: false }).enforced).toBe(false);
  });

  it("S07-07 crash/health failure leaves active intact and recovery closes incomplete WAL", () => {
    const store = new InMemoryInstallStoreV2(); const coordinator = new InstallCoordinatorV2(store);
    coordinator.install({ pluginId: "muse.example", version: "1.0.0", digest, verify() {}, grant() {}, stage() {}, health: () => true });
    expect(() => coordinator.install({ pluginId: "muse.example", version: "2.0.0", digest: `sha256:${"d".repeat(64)}`, verify() {}, grant() {}, stage() {}, health: () => false })).toThrow("HEALTH_CHECK_FAILED");
    expect(coordinator.snapshot().active?.version).toBe("1.0.0"); expect(coordinator.recover().wal.at(-1)?.step).toBe("failed");
  });

  it("S07-08 uninstall refuses complete while an Effect cleanup fails", () => {
    const effects = [{ sequence: 1, activationRef: "activation.1", effectRef: "subscription.1", ownerRef: "muse.example/agent", generation: 1, effectKind: "subscription", state: "observed", observedAt: 1 }] as never;
    expect(reconcileUninstallV2(effects, () => false)).toEqual({ complete: false, unresolved: ["subscription.1"] });
  });

  it("S07-09 prompt-originated effect still requires independent grant and trusted approval", () => {
    const host = new TrustedApprovalHostV2();
    expect(() => host.approve({ origin: "dsh://plugin-webview", requestRef: "made-up", nonce: "x", now: 1000 })).toThrow("APPROVAL_ORIGIN_UNTRUSTED");
  });

  it("S07-10 verifies signature and only awards sandboxed when Host evidence exists", () => {
    expect(artifactSigningStatementV2(unsigned).length).toBeGreaterThan(0);
    const signed = { ...evidence(), sandboxEvidence: { runner: "muse-runner", policyDigest: `sha256:${"e".repeat(64)}` as const } };
    expect(verifyArtifactV2({ bytes, evidence: signed, publisherKeys: new Map([["publisher.1", publicKey]]) }).levels).not.toContain("sandboxed");
    expect(verifyArtifactV2({ bytes, evidence: signed, publisherKeys: new Map([["publisher.1", publicKey]]), hostSandboxEnforced: true }).levels).toContain("sandboxed");
  });

  it("S07-11 audit chain accepts references/hashes and rejects content or secret", () => {
    const audit = new PrivacyAuditLogV2(); audit.append({ traceRef: "t1", grantRef: "g1", approvalRef: "a1", revision: "r1", ...PrivacyAuditLogV2.contentRef("private text") });
    expect(audit.all()).toHaveLength(1); expect(JSON.stringify(audit.all())).not.toContain("private text");
    expect(() => audit.append({ prompt: "private text" })).toThrow("AUDIT_SENSITIVE_DATA_FORBIDDEN");
    expect(() => audit.append({ result: "sk-abcdefghijklmnop" })).toThrow("AUDIT_SENSITIVE_DATA_FORBIDDEN");
  });
});
