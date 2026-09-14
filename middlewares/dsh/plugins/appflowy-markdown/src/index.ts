import { digestSchema, type JsonValue } from "@muse/host-bridge";
export { markdownPresentationContexts } from "./presentation.js";
export * from "./snapshot.js";
import {
  DOCUMENT_CONTRACT,
  documentApplyInputSchema,
  documentApplyOutputSchema,
  documentProposeInputSchema,
  documentProposeOutputSchema,
  documentQueryInputSchema,
  documentQueryOutputSchema,
  documentSnapshotInputSchema,
  documentSnapshotOutputSchema,
  documentStatusInputSchema,
  documentStatusOutputSchema,
  type JsonSchema
} from "@muse/contract-document";
import type { MusePluginDefinition } from "@muse/plugin-kit";
import type { JsonSchemaNode, ObjectJsonSchema } from "@deepseek-ai/dsh-tools";

export const APPFLOWY_MARKDOWN_FAMILY = DOCUMENT_CONTRACT.family;
export const APPFLOWY_MARKDOWN_READ_OPERATION = DOCUMENT_CONTRACT.operations.query;
export const APPFLOWY_MARKDOWN_PROPOSE_OPERATION = DOCUMENT_CONTRACT.operations.propose;
export const APPFLOWY_MARKDOWN_APPLY_OPERATION = DOCUMENT_CONTRACT.operations.apply;
export const APPFLOWY_MARKDOWN_STATUS_OPERATION = DOCUMENT_CONTRACT.operations.status;
export const APPFLOWY_MARKDOWN_SNAPSHOT_OPERATION = DOCUMENT_CONTRACT.operations.snapshot;
export const APPFLOWY_MARKDOWN_READ_TOOL = "muse_document_read_current";
export const APPFLOWY_MARKDOWN_SNAPSHOT_TOOL = "muse_document_read";
export const APPFLOWY_MARKDOWN_PROPOSE_TOOL = "muse_document_propose_markdown_edit";
export const APPFLOWY_MARKDOWN_APPLY_TOOL = "muse_document_apply_approved_edit";

export const providerInputSchema = documentQueryInputSchema as JsonValue;
export const providerOutputSchema = documentQueryOutputSchema as JsonValue;
export const proposeProviderInputSchema = documentProposeInputSchema as JsonValue;
export const proposeProviderOutputSchema = documentProposeOutputSchema as JsonValue;
export const applyProviderInputSchema = documentApplyInputSchema as JsonValue;
export const applyProviderOutputSchema = documentApplyOutputSchema as JsonValue;
export const statusProviderInputSchema = documentStatusInputSchema as JsonValue;
export const statusProviderOutputSchema = documentStatusOutputSchema as JsonValue;
export const snapshotProviderInputSchema = documentSnapshotInputSchema as JsonValue;
export const snapshotProviderOutputSchema = documentSnapshotOutputSchema as JsonValue;

const modelInputSchema: ObjectJsonSchema = { type: "object", additionalProperties: false, properties: {} };
const snapshotModelInputSchema: ObjectJsonSchema = {
  type: "object", additionalProperties: false, required: ["resourceRef"],
  properties: {
    resourceRef: {
      type: "string",
      description: "Opaque page id from muse_workspace_list_views. Only document-layout pages can be read."
    }
  }
};
const modelOutputSchema: JsonSchemaNode = {
  type: "object", additionalProperties: false, required: ["protocol", "resourceRef", "revision", "content"],
  properties: {
    protocol: { type: "string", const: "muse.document/snapshot/v2" }, resourceRef: { type: "string" }, revision: { type: "string" },
    content: { type: "object", additionalProperties: false, required: ["mediaType", "text", "truncated", "byteLength"], properties: {
      mediaType: { type: "string" }, text: { type: "string" }, truncated: { type: "boolean" }, byteLength: { type: "integer" }
    } }
  }
};
const proposeModelInputSchema: ObjectJsonSchema = {
  type: "object", additionalProperties: false, required: ["expectedRevision", "kind"],
  properties: {
    expectedRevision: { type: "string", description: "Opaque revision from muse_document_read_current." },
    kind: { type: "string", enum: ["insert", "replace", "delete"] },
    text: { type: "string", description: "Text for insert/replace." },
    find: { type: "string", description: "Exact span for targeted replace/delete." }
  }
};
const proposeModelOutputSchema: JsonSchemaNode = {
  type: "object", additionalProperties: false,
  required: ["protocol", "proposalRef", "resourceRef", "expectedRevision", "preview", "approvalRequired", "expiresAt"],
  properties: {
    protocol: { type: "string" }, proposalRef: { type: "string" }, resourceRef: { type: "string" }, expectedRevision: { type: "string" },
    preview: { type: "object", additionalProperties: false, required: ["before", "after", "changed"], properties: {
      before: { type: "string" }, after: { type: "string" }, changed: { type: "boolean" }
    } }, approvalRequired: { type: "boolean", const: true }, expiresAt: { type: "integer" }
  }
};
const applyModelInputSchema: ObjectJsonSchema = {
  type: "object", additionalProperties: false, required: ["proposalRef"],
  properties: { proposalRef: { type: "string", description: "Opaque proposalRef returned by propose." } }
};
const applyModelOutputSchema: JsonSchemaNode = {
  type: "object", additionalProperties: false,
  required: ["protocol", "commandRef", "status", "resourceRef", "previousRevision", "revision", "eventPublicationStatus", "idempotencyKey"],
  properties: {
    protocol: { type: "string" }, commandRef: { type: "string" }, status: { type: "string", enum: ["applied", "unchanged", "conflict"] },
    resourceRef: { type: "string" }, previousRevision: { type: "string" }, revision: { type: "string" },
    eventCursor: { oneOf: [{ type: "string" }, { type: "null" }] }, eventPublicationStatus: { type: "string", enum: ["published", "not-needed", "degraded"] },
    idempotencyKey: { type: "string" }
  }
};

const digest = (value: JsonSchema | JsonValue): string => digestSchema(value as JsonValue);

export const appFlowyMarkdownDefinition: MusePluginDefinition = {
  pluginId: "muse.appflowy.markdown", version: "2.0.0", bridgeMajor: 1,
  targets: [{
    familyId: DOCUMENT_CONTRACT.family,
    contract: { major: DOCUMENT_CONTRACT.major, minMinor: 0, maxMinor: DOCUMENT_CONTRACT.minor },
    requiredOperations: [
      DOCUMENT_CONTRACT.operations.query, DOCUMENT_CONTRACT.operations.propose,
      DOCUMENT_CONTRACT.operations.apply, DOCUMENT_CONTRACT.operations.status,
      DOCUMENT_CONTRACT.operations.snapshot
    ],
    tools: [
      {
        name: APPFLOWY_MARKDOWN_READ_TOOL,
        description: "Read the Host-selected document through muse.document@2 as a bounded Markdown projection.",
        operationId: DOCUMENT_CONTRACT.operations.query, parameters: modelInputSchema, output: modelOutputSchema,
        adapter: {
          accepts: contract => contract.operation.effect === "read" && contract.operation.inputSchema.sha256 === digest(documentQueryInputSchema) && contract.operation.outputSchema.sha256 === digest(documentQueryOutputSchema),
          toProviderInput: () => ({}), fromProviderOutput: value => value
        }
      },
      {
        name: APPFLOWY_MARKDOWN_SNAPSHOT_TOOL,
        description:
          "Read a workspace document page listed by muse_workspace_list_views as a bounded Markdown projection. "
          + "Pass resourceRef from that listing. Does not write.",
        operationId: DOCUMENT_CONTRACT.operations.snapshot, parameters: snapshotModelInputSchema, output: modelOutputSchema,
        adapter: {
          accepts: contract =>
            contract.operation.effect === "read"
            && contract.operation.inputSchema.sha256 === digest(documentSnapshotInputSchema)
            && contract.operation.outputSchema.sha256 === digest(documentSnapshotOutputSchema),
          toProviderInput: args => {
            const value = args as { resourceRef: string };
            return { resourceRef: value.resourceRef };
          },
          fromProviderOutput: value => value
        }
      },
      {
        name: APPFLOWY_MARKDOWN_PROPOSE_TOOL,
        description: "Create a non-mutating muse.document@2 Markdown edit proposal at an exact revision.",
        operationId: DOCUMENT_CONTRACT.operations.propose, parameters: proposeModelInputSchema, output: proposeModelOutputSchema,
        adapter: {
          accepts: contract => contract.operation.inputSchema.sha256 === digest(documentProposeInputSchema) && contract.operation.outputSchema.sha256 === digest(documentProposeOutputSchema),
          toProviderInput: args => {
            const value = args as { expectedRevision: string; kind: string; text?: string; find?: string };
            return { expectedRevision: value.expectedRevision, mutation: { kind: value.kind, ...(value.text === undefined ? {} : { text: value.text }), ...(value.find === undefined ? {} : { find: value.find }) } };
          },
          fromProviderOutput: value => value
        }
      },
      {
        name: APPFLOWY_MARKDOWN_APPLY_TOOL,
        description: "Apply a trusted-approved muse.document@2 proposal with Bridge idempotency.",
        operationId: DOCUMENT_CONTRACT.operations.apply, parameters: applyModelInputSchema, output: applyModelOutputSchema,
        policy: { effect: "local_write" },
        adapter: {
          accepts: contract => contract.operation.effect === "local_write" && contract.operation.idempotency === "required" && contract.operation.inputSchema.sha256 === digest(documentApplyInputSchema) && contract.operation.outputSchema.sha256 === digest(documentApplyOutputSchema),
          toProviderInput: args => args, fromProviderOutput: value => value
        }
      }
    ]
  }]
};
