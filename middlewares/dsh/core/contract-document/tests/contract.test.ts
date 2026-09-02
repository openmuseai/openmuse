import { describe, expect, it } from "vitest";
import { DocumentContractError, InMemoryDocumentProviderV2 } from "../src/index.js";

const authority = { actorRef: "actor.1", resourceRef: "document.1", canRead: true, canWrite: true };

describe("muse.document@2 TCK", () => {
  it("H04-03 traces query/propose/apply/event/receipt through one revision", () => {
    const provider = new InMemoryDocumentProviderV2("document.1", "Hello");
    const proposal = provider.propose(authority, "0", { kind: "insert", text: "Muse" }, 1000);
    const receipt = provider.apply(proposal.proposalRef, { authority, approved: true, idempotencyKey: "idem.1", origin: "agent", now: 1100 });
    expect(receipt).toMatchObject({ previousRevision: "0", revision: "1", status: "applied", eventCursor: "1" });
    expect(provider.query(authority).content.text).toBe("Hello\nMuse");
    expect(provider.subscribe("0")[0]).toMatchObject({ revision: "1", commandRef: receipt.commandRef });
  });
  it("H04-04 duplicate apply returns the same receipt and one event", () => {
    const provider = new InMemoryDocumentProviderV2("document.1", "Hello");
    const proposal = provider.propose(authority, "0", { kind: "replace", text: "Muse" }, 1000);
    const input = { authority, approved: true, idempotencyKey: "idem.1", origin: "agent" as const, now: 1100 };
    const first = provider.apply(proposal.proposalRef, input);
    expect(provider.apply(proposal.proposalRef, input)).toEqual(first);
    expect(provider.subscribe()).toHaveLength(1);
  });
  it("H04-05 concurrent expected revisions allow one commit", () => {
    const provider = new InMemoryDocumentProviderV2("document.1", "Hello");
    const one = provider.propose(authority, "0", { kind: "insert", text: "One" }, 1000);
    const two = provider.propose(authority, "0", { kind: "insert", text: "Two" }, 1000);
    provider.apply(one.proposalRef, { authority, approved: true, idempotencyKey: "one", origin: "agent", now: 1100 });
    expect(() => provider.apply(two.proposalRef, { authority, approved: true, idempotencyKey: "two", origin: "agent", now: 1101 }))
      .toThrowError(expect.objectContaining({ code: "REVISION_CONFLICT" }));
  });
  it("H04-06 cursor expiry requires a snapshot", () => {
    const provider = new InMemoryDocumentProviderV2("document.1", "", 1);
    for (let index = 0; index < 2; index++) {
      const revision = String(index);
      const proposal = provider.propose(authority, revision, { kind: "insert", text: String(index) }, 1000 + index);
      provider.apply(proposal.proposalRef, { authority, approved: true, idempotencyKey: `idem.${index}`, origin: "agent", now: 1100 + index });
    }
    expect(() => provider.subscribe("0")).toThrowError(expect.objectContaining({ code: "CURSOR_EXPIRED" }));
  });
  it("H04-08 rejects missing ACL and trusted approval independently", () => {
    const provider = new InMemoryDocumentProviderV2("document.1", "Hello");
    expect(() => provider.query({ ...authority, canRead: false })).toThrowError(DocumentContractError);
    const proposal = provider.propose(authority, "0", { kind: "replace", text: "Muse" }, 1000);
    expect(() => provider.apply(proposal.proposalRef, { authority, approved: false, idempotencyKey: "x", origin: "agent", now: 1100 }))
      .toThrowError(expect.objectContaining({ code: "APPROVAL_REQUIRED" }));
  });
});
