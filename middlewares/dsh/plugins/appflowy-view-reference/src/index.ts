import { digestSchema, type JsonValue } from "@muse/host-bridge";
import type { MusePluginDefinition } from "@muse/plugin-kit";
import type { JsonSchemaNode, ObjectJsonSchema } from "@deepseek-ai/dsh-tools";

export const APPFLOWY_VIEW_REFERENCE_FAMILY = "appflowy.view-reference";
export const APPFLOWY_VIEW_REFERENCE_OPERATION = "view.reference.read";
export const APPFLOWY_VIEW_REFERENCE_TOOL = "muse_appflowy_get_view_reference";

export const providerInputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  additionalProperties: false,
  properties: {
    limit: { type: "integer", minimum: 1, maximum: 32 },
    cursor: { type: "string", minLength: 1, maxLength: 256 }
  }
};

const providerViewSchema: JsonValue = {
  type: "object",
  additionalProperties: false,
  required: ["title", "titleTruncated", "layout", "locked", "childCount"],
  properties: {
    title: { type: "string", maxLength: 256 },
    titleTruncated: { type: "boolean" },
    layout: { enum: ["document", "grid", "board", "calendar", "chat"] },
    locked: { type: ["boolean", "null"] },
    childCount: { type: "integer", minimum: 0, maximum: 256 }
  }
};

const providerChildSchema: JsonValue = {
  type: "object",
  additionalProperties: false,
  required: ["title", "titleTruncated", "layout", "locked"],
  properties: {
    title: { type: "string", maxLength: 256 },
    titleTruncated: { type: "boolean" },
    layout: { enum: ["document", "grid", "board", "calendar", "chat"] },
    locked: { type: ["boolean", "null"] }
  }
};

export const providerOutputSchema: JsonValue = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  additionalProperties: false,
  required: ["view", "children", "page"],
  properties: {
    view: providerViewSchema,
    children: { type: "array", maxItems: 32, items: providerChildSchema },
    page: {
      type: "object",
      additionalProperties: false,
      required: ["returned", "hasMore"],
      properties: {
        returned: { type: "integer", minimum: 0, maximum: 32 },
        hasMore: { type: "boolean" },
        nextCursor: { type: "string", minLength: 1, maxLength: 256 }
      }
    }
  }
};

const nullableBoolean: JsonSchemaNode = {
  oneOf: [{ type: "boolean" }, { type: "null" }]
};
const layoutSchema: JsonSchemaNode = {
  type: "string",
  enum: ["document", "grid", "board", "calendar", "chat"]
};
const modelChildSchema: JsonSchemaNode = {
  type: "object",
  additionalProperties: false,
  required: ["title", "titleTruncated", "layout", "locked"],
  properties: {
    title: { type: "string" },
    titleTruncated: { type: "boolean" },
    layout: layoutSchema,
    locked: nullableBoolean
  }
};

export const modelInputSchema: ObjectJsonSchema = {
  type: "object",
  additionalProperties: false,
  properties: {
    limit: { type: "integer", enum: [5, 10, 20, 32], default: 20 },
    cursor: { type: "string" }
  }
};

export const modelOutputSchema: JsonSchemaNode = {
  type: "object",
  additionalProperties: false,
  required: ["view", "children", "page"],
  properties: {
    view: {
      type: "object",
      additionalProperties: false,
      required: ["title", "titleTruncated", "layout", "locked", "childCount"],
      properties: {
        title: { type: "string" },
        titleTruncated: { type: "boolean" },
        layout: layoutSchema,
        locked: nullableBoolean,
        childCount: { type: "integer" }
      }
    },
    children: { type: "array", items: modelChildSchema },
    page: {
      type: "object",
      additionalProperties: false,
      required: ["returned", "hasMore"],
      properties: {
        returned: { type: "integer" },
        hasMore: { type: "boolean" },
        nextCursor: { type: "string" }
      }
    }
  }
};

const expectedInputDigest = digestSchema(providerInputSchema);
const expectedOutputDigest = digestSchema(providerOutputSchema);

export const appFlowyViewReferenceDefinition: MusePluginDefinition = {
  pluginId: "appflowy-view-reference",
  version: "1.0.0",
  bridgeMajor: 1,
  targets: [{
    familyId: APPFLOWY_VIEW_REFERENCE_FAMILY,
    contract: { major: 1, minMinor: 0, maxMinor: 0 },
    requiredOperations: [APPFLOWY_VIEW_REFERENCE_OPERATION],
    tools: [{
      name: APPFLOWY_VIEW_REFERENCE_TOOL,
      description: "Read bounded metadata and immediate child summaries for the AppFlowy view currently bound by the Host. Does not read document or block content.",
      operationId: APPFLOWY_VIEW_REFERENCE_OPERATION,
      parameters: modelInputSchema,
      output: modelOutputSchema,
      adapter: {
        accepts: contract =>
          contract.operation.inputSchema.sha256 === expectedInputDigest
          && contract.operation.outputSchema.sha256 === expectedOutputDigest,
        toProviderInput: args => args,
        fromProviderOutput: value => value
      }
    }]
  }]
};
