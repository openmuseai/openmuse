import { digestSchema, type JsonValue } from "@muse/host-bridge";
import type { MusePluginDefinition } from "@muse/plugin-kit";
import type { JsonSchemaNode, ObjectJsonSchema } from "@deepseek-ai/dsh-tools";

export const APPFLOWY_VIEW_RENAME_FAMILY = "appflowy.view-rename";
export const APPFLOWY_VIEW_RENAME_PROPOSE = "view.rename.propose";
export const APPFLOWY_VIEW_RENAME_APPLY = "view.rename.apply";
export const APPFLOWY_VIEW_RENAME_STATUS = "view.rename.status";
export const APPFLOWY_VIEW_RENAME_PROPOSE_TOOL = "muse_appflowy_propose_view_rename";
export const APPFLOWY_VIEW_RENAME_APPLY_TOOL = "muse_appflowy_apply_view_rename";

const proposalIdSchema: JsonValue = { type: "string", minLength: 1, maxLength: 128 };
export const proposeProviderInputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object", additionalProperties: false, required: ["title"],
  properties: { title: { type: "string", minLength: 1, maxLength: 256 } }
};
export const proposeProviderOutputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object", additionalProperties: false, required: ["proposalId", "preview", "expiresAtMs"],
  properties: {
    proposalId: proposalIdSchema,
    preview: {
      type: "object", additionalProperties: false,
      required: ["currentTitle", "proposedTitle", "changed", "approvalRequired"],
      properties: {
        currentTitle: { type: "string", maxLength: 256 },
        proposedTitle: { type: "string", maxLength: 256 },
        changed: { type: "boolean" }, approvalRequired: { const: true }
      }
    },
    expiresAtMs: { type: "integer", minimum: 0 }
  }
};
export const applyProviderInputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object", additionalProperties: false, required: ["proposalId"],
  properties: { proposalId: proposalIdSchema }
};
export const applyProviderOutputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object", additionalProperties: false, required: ["commandId", "status"],
  properties: {
    commandId: { type: "string", minLength: 1, maxLength: 128 },
    status: { enum: ["applied", "unchanged", "conflict"] }
  }
};
export const statusProviderInputSchema = applyProviderInputSchema;
export const statusProviderOutputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object", additionalProperties: false, required: ["status", "expiresAtMs"],
  properties: { status: { const: "pending" }, expiresAtMs: { type: "integer", minimum: 0 } }
};

export const proposeModelInputSchema: ObjectJsonSchema = {
  type: "object", additionalProperties: false, required: ["title"],
  properties: { title: { type: "string", description: "New title for the currently selected view." } }
};
export const applyModelInputSchema: ObjectJsonSchema = {
  type: "object", additionalProperties: false, required: ["proposalId"],
  properties: { proposalId: { type: "string", description: "Opaque proposal ID returned by the propose tool." } }
};
const proposeModelOutputSchema: JsonSchemaNode = {
  type: "object", additionalProperties: false, required: ["proposalId", "preview", "expiresAtMs"],
  properties: {
    proposalId: { type: "string" },
    preview: {
      type: "object", additionalProperties: false,
      required: ["currentTitle", "proposedTitle", "changed", "approvalRequired"],
      properties: {
        currentTitle: { type: "string" }, proposedTitle: { type: "string" },
        changed: { type: "boolean" }, approvalRequired: { type: "boolean", const: true }
      }
    },
    expiresAtMs: { type: "integer" }
  }
};
const applyModelOutputSchema: JsonSchemaNode = {
  type: "object", additionalProperties: false, required: ["commandId", "status"],
  properties: {
    commandId: { type: "string" },
    status: { type: "string", enum: ["applied", "unchanged", "conflict"] }
  }
};
const digest = (value: JsonValue) => digestSchema(value);

export const appFlowyViewRenameDefinition: MusePluginDefinition = {
  pluginId: "appflowy-view-rename", version: "1.0.0", bridgeMajor: 1,
  targets: [{
    familyId: APPFLOWY_VIEW_RENAME_FAMILY,
    contract: { major: 1, minMinor: 0, maxMinor: 0 },
    requiredOperations: [APPFLOWY_VIEW_RENAME_PROPOSE, APPFLOWY_VIEW_RENAME_APPLY, APPFLOWY_VIEW_RENAME_STATUS],
    tools: [
      {
        name: APPFLOWY_VIEW_RENAME_PROPOSE_TOOL,
        description: "Create a short-lived, non-mutating rename proposal for the AppFlowy view currently selected by the Host.",
        operationId: APPFLOWY_VIEW_RENAME_PROPOSE,
        parameters: proposeModelInputSchema, output: proposeModelOutputSchema,
        adapter: {
          accepts: contract => digest(proposeProviderInputSchema) === contract.operation.inputSchema.sha256
            && digest(proposeProviderOutputSchema) === contract.operation.outputSchema.sha256,
          toProviderInput: args => args,
          fromProviderOutput: value => value
        }
      },
      {
        name: APPFLOWY_VIEW_RENAME_APPLY_TOOL,
        description: "After user approval, apply a previously created rename proposal to the Host-selected AppFlowy view.",
        operationId: APPFLOWY_VIEW_RENAME_APPLY,
        parameters: applyModelInputSchema, output: applyModelOutputSchema,
        policy: { effect: "local_write" },
        adapter: {
          accepts: contract => contract.operation.effect === "local_write"
            && contract.operation.idempotency === "required"
            && digest(applyProviderInputSchema) === contract.operation.inputSchema.sha256
            && digest(applyProviderOutputSchema) === contract.operation.outputSchema.sha256,
          toProviderInput: args => args,
          fromProviderOutput: value => value
        }
      }
    ]
  }]
};
