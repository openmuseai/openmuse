export type JsonSchema = Readonly<Record<string, unknown>>;
const draft = "https://json-schema.org/draft/2020-12/schema";
const opaqueRef = { type: "string", minLength: 1, maxLength: 128 } as const;
const revision = { type: "string", pattern: "^sha256:[0-9a-f]{64}$" } as const;

export const documentQueryInputSchema: JsonSchema = { $schema: draft, type: "object", additionalProperties: false };
export const documentQueryOutputSchema: JsonSchema = {
  $schema: draft, type: "object", additionalProperties: false,
  required: ["protocol", "resourceRef", "revision", "content"],
  properties: {
    protocol: { const: "muse.document/snapshot/v2" }, resourceRef: opaqueRef, revision,
    content: { type: "object", additionalProperties: false, required: ["mediaType", "text", "truncated", "byteLength"], properties: {
      mediaType: { const: "text/markdown" }, text: { type: "string", maxLength: 65536 }, truncated: { type: "boolean" },
      byteLength: { type: "integer", minimum: 0, maximum: 65536 }
    } }
  }
};
export const documentProposeInputSchema: JsonSchema = {
  $schema: draft, type: "object", additionalProperties: false, required: ["expectedRevision", "mutation"],
  properties: {
    expectedRevision: revision,
    mutation: { type: "object", required: ["kind"], properties: {
      kind: { enum: ["insert", "replace", "delete"] }, text: { type: "string", maxLength: 65536 },
      find: { type: "string", minLength: 1, maxLength: 65536 }
    } }
  }
};
export const documentProposeOutputSchema: JsonSchema = {
  $schema: draft, type: "object", additionalProperties: false,
  required: ["protocol", "proposalRef", "resourceRef", "expectedRevision", "preview", "approvalRequired", "expiresAt"],
  properties: {
    protocol: { const: "muse.document/proposal/v2" }, proposalRef: opaqueRef, resourceRef: opaqueRef, expectedRevision: revision,
    preview: { type: "object", additionalProperties: false, required: ["before", "after", "changed"], properties: {
      before: { type: "string", maxLength: 65536 }, after: { type: "string", maxLength: 65536 }, changed: { type: "boolean" }
    } }, approvalRequired: { const: true }, expiresAt: { type: "integer", minimum: 0 }
  }
};
export const documentApplyInputSchema: JsonSchema = {
  $schema: draft, type: "object", additionalProperties: false, required: ["proposalRef"], properties: { proposalRef: opaqueRef }
};
export const documentApplyOutputSchema: JsonSchema = {
  $schema: draft, type: "object", additionalProperties: false,
  required: ["protocol", "commandRef", "status", "resourceRef", "previousRevision", "revision", "eventPublicationStatus", "idempotencyKey"],
  properties: {
    protocol: { const: "muse.document/receipt/v2" }, commandRef: opaqueRef,
    status: { enum: ["applied", "unchanged", "conflict"] }, resourceRef: opaqueRef,
    previousRevision: revision, revision, eventCursor: { type: ["string", "null"] },
    eventPublicationStatus: { enum: ["published", "not-needed", "degraded"] }, idempotencyKey: { type: "string" }
  }
};
export const documentStatusInputSchema = documentApplyInputSchema;
export const documentStatusOutputSchema: JsonSchema = {
  $schema: draft, type: "object", additionalProperties: false, required: ["status", "expiresAt"],
  properties: { status: { const: "pending" }, expiresAt: { type: "integer", minimum: 0 } }
};
