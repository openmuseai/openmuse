import { createHash, randomUUID } from "node:crypto";

export * from "./schemas.js";

export const DOCUMENT_CONTRACT = {
  family: "muse.document",
  major: 2,
  minor: 0,
  operations: {
    query: "document.current.query",
    propose: "document.current.propose",
    apply: "document.current.apply",
    status: "document.command.status",
    snapshot: "document.resource.snapshot"
  },
  events: { committed: "document.commit.v2" }
} as const;

export interface DocumentContentV2 {
  readonly mediaType: "text/markdown" | "text/plain" | string;
  readonly text: string;
  readonly truncated: boolean;
  readonly byteLength: number;
}
export interface DocumentSnapshotV2 {
  readonly protocol: "muse.document/snapshot/v2";
  readonly resourceRef: string;
  readonly revision: string;
  readonly content: DocumentContentV2;
}
export type DocumentMutationV2 =
  | { readonly kind: "insert"; readonly text: string }
  | { readonly kind: "replace"; readonly text: string; readonly find?: string }
  | { readonly kind: "delete"; readonly find: string };
export interface DocumentProposalV2 {
  readonly protocol: "muse.document/proposal/v2";
  readonly proposalRef: string;
  readonly resourceRef: string;
  readonly expectedRevision: string;
  readonly preview: { readonly changed: boolean; readonly before: string; readonly after: string };
  readonly approvalRequired: true;
  readonly expiresAt: number;
}
export interface DocumentCommitReceiptV2 {
  readonly protocol: "muse.document/receipt/v2";
  readonly commandRef: string;
  readonly status: "applied" | "unchanged" | "conflict";
  readonly resourceRef: string;
  readonly previousRevision: string;
  readonly revision: string;
  readonly eventCursor?: string;
  readonly idempotencyKey: string;
}
export interface DocumentCommitEventV2 {
  readonly protocol: "muse.document/event/v2";
  readonly eventId: string;
  readonly cursor: string;
  readonly resourceRef: string;
  readonly revision: string;
  readonly commandRef: string;
  readonly origin: "ui" | "agent" | "collaboration" | "recovery";
  readonly change: { readonly kind: "content-replaced" | "content-inserted" | "content-deleted" | "unchanged" };
  readonly occurredAt: number;
}
export type DocumentErrorCode = "INVALID_INPUT" | "FORBIDDEN" | "APPROVAL_REQUIRED" | "REVISION_CONFLICT" |
  "PROPOSAL_EXPIRED" | "PROPOSAL_NOT_FOUND" | "CURSOR_EXPIRED" | "CANCELLED" | "DEADLINE_EXCEEDED" | "UNAVAILABLE";
export class DocumentContractError extends Error {
  constructor(readonly code: DocumentErrorCode, readonly details: Readonly<Record<string, string>> = {}) { super(code); }
}

export interface DocumentAuthorityV2 {
  readonly actorRef: string;
  readonly resourceRef: string;
  readonly canRead: boolean;
  readonly canWrite: boolean;
}
export interface DocumentApplyContextV2 {
  readonly authority: DocumentAuthorityV2;
  readonly approved: boolean;
  readonly idempotencyKey: string;
  readonly origin: DocumentCommitEventV2["origin"];
  readonly now: number;
}
interface PendingProposal { readonly actorRef: string; readonly value: DocumentProposalV2; readonly mutation: DocumentMutationV2; }

export class InMemoryDocumentProviderV2 {
  private text: string;
  private revision = 0;
  private cursor = 0;
  private readonly proposals = new Map<string, PendingProposal>();
  private readonly receipts = new Map<string, DocumentCommitReceiptV2>();
  private readonly events: DocumentCommitEventV2[] = [];
  constructor(readonly resourceRef: string, initialText = "", private readonly retention = 128) { this.text = initialText; }

  query(authority: DocumentAuthorityV2): DocumentSnapshotV2 {
    this.authorize(authority, false);
    return { protocol: "muse.document/snapshot/v2", resourceRef: this.resourceRef, revision: String(this.revision), content: { mediaType: "text/markdown", text: this.text, truncated: false, byteLength: Buffer.byteLength(this.text) } };
  }
  propose(authority: DocumentAuthorityV2, expectedRevision: string, mutation: DocumentMutationV2, now: number): DocumentProposalV2 {
    this.authorize(authority, true);
    if (expectedRevision !== String(this.revision)) throw new DocumentContractError("REVISION_CONFLICT", { expectedRevision, actualRevision: String(this.revision) });
    const after = applyMutation(this.text, mutation);
    const value: DocumentProposalV2 = {
      protocol: "muse.document/proposal/v2", proposalRef: `proposal.${randomUUID()}`, resourceRef: this.resourceRef,
      expectedRevision, preview: { changed: after !== this.text, before: this.text, after }, approvalRequired: true, expiresAt: now + 300_000
    };
    this.proposals.set(value.proposalRef, { actorRef: authority.actorRef, value, mutation });
    return value;
  }
  apply(proposalRef: string, context: DocumentApplyContextV2): DocumentCommitReceiptV2 {
    this.authorize(context.authority, true);
    const duplicate = this.receipts.get(context.idempotencyKey); if (duplicate !== undefined) return structuredClone(duplicate);
    if (!context.approved) throw new DocumentContractError("APPROVAL_REQUIRED");
    const pending = this.proposals.get(proposalRef); if (pending === undefined) throw new DocumentContractError("PROPOSAL_NOT_FOUND");
    if (pending.value.expiresAt <= context.now) { this.proposals.delete(proposalRef); throw new DocumentContractError("PROPOSAL_EXPIRED"); }
    if (pending.actorRef !== context.authority.actorRef) throw new DocumentContractError("FORBIDDEN");
    if (pending.value.expectedRevision !== String(this.revision)) throw new DocumentContractError("REVISION_CONFLICT", { expectedRevision: pending.value.expectedRevision, actualRevision: String(this.revision) });
    const previousRevision = String(this.revision);
    const next = applyMutation(this.text, pending.mutation);
    const changed = next !== this.text;
    if (changed) { this.text = next; this.revision++; }
    const commandRef = `command.${randomUUID()}`;
    let eventCursor: string | undefined;
    if (changed) {
      eventCursor = String(++this.cursor);
      this.events.push({
        protocol: "muse.document/event/v2", eventId: `event.${randomUUID()}`, cursor: eventCursor,
        resourceRef: this.resourceRef, revision: String(this.revision), commandRef, origin: context.origin,
        change: { kind: mutationEventKind(pending.mutation) }, occurredAt: context.now
      });
      while (this.events.length > this.retention) this.events.shift();
    }
    const receipt: DocumentCommitReceiptV2 = {
      protocol: "muse.document/receipt/v2", commandRef, status: changed ? "applied" : "unchanged", resourceRef: this.resourceRef,
      previousRevision, revision: String(this.revision), ...(eventCursor === undefined ? {} : { eventCursor }), idempotencyKey: context.idempotencyKey
    };
    this.proposals.delete(proposalRef); this.receipts.set(context.idempotencyKey, receipt); return structuredClone(receipt);
  }
  subscribe(afterCursor?: string): readonly DocumentCommitEventV2[] {
    const after = afterCursor === undefined ? 0 : Number(afterCursor);
    const first = this.events[0];
    if (afterCursor !== undefined && first !== undefined && after < Number(first.cursor) - 1) throw new DocumentContractError("CURSOR_EXPIRED", { snapshotRevision: String(this.revision) });
    return this.events.filter(value => Number(value.cursor) > after).map(value => structuredClone(value));
  }
  private authorize(authority: DocumentAuthorityV2, write: boolean): void {
    if (authority.resourceRef !== this.resourceRef || !authority.canRead || (write && !authority.canWrite)) throw new DocumentContractError("FORBIDDEN");
  }
}

export const documentContractDigest = (value: unknown): `sha256:${string}` =>
  `sha256:${createHash("sha256").update(JSON.stringify(value)).digest("hex")}`;
const applyMutation = (current: string, mutation: DocumentMutationV2): string => {
  if (mutation.kind === "insert") return current.length === 0 ? mutation.text : `${current}\n${mutation.text}`;
  if (mutation.kind === "delete") {
    if (mutation.find.length === 0 || !current.includes(mutation.find)) throw new DocumentContractError("INVALID_INPUT");
    return current.replace(mutation.find, "");
  }
  if (mutation.find === undefined) return mutation.text;
  if (mutation.find.length === 0 || !current.includes(mutation.find)) throw new DocumentContractError("INVALID_INPUT");
  return current.replace(mutation.find, mutation.text);
};
const mutationEventKind = (mutation: DocumentMutationV2): DocumentCommitEventV2["change"]["kind"] =>
  mutation.kind === "insert" ? "content-inserted" : mutation.kind === "delete" ? "content-deleted" : "content-replaced";
