import { digestSchema } from "@muse/host-bridge";
import { createMusePlugin } from "@muse/plugin-kit/dsh";
import type { MusePluginDefinition, MusePluginRuntimeConfig } from "@muse/plugin-kit";
import type { JsonSchemaNode, ObjectJsonSchema } from "@deepseek-ai/dsh-tools";
import type { Context } from "@deepseek-ai/cordis";
import {
  WORKSPACE_FAMILY,
  WORKSPACE_TREE_OPERATION,
  treeInputSchema,
  treeOutputSchema
} from "./cloud.js";

export const APPFLOWY_WORKSPACE_TREE_TOOL = "muse_workspace_list_views";

/** Tells the model not to glob the DSH cwd; pages live in Cloud folder collab. */
export const WORKSPACE_LIST_PROMPT =
  "AppFlowy pages live in the Host workspace plugin (sidebar catalog) and Cloud folder collab, "
  + "not in the DSH cwd. "
  + "The bound workspace directory only contains README.md by design; glob/ls will not list pages. "
  + "To list this workspace's files/pages, call muse_workspace_list_views. "
  + "To read a document-layout page, call muse_document_read with resourceRef set to that item's viewId. "
  + "muse_document_read_current only reads the Host-focused page. "
  + "Newly created or imported AppFlowy pages appear on the next muse_workspace_list_views call "
  + "(Host catalog first while Cloud folder collab is unwired; there is no cwd watcher).";

const expectedInputDigest = digestSchema(treeInputSchema);
const expectedOutputDigest = digestSchema(treeOutputSchema);

const modelInputSchema: ObjectJsonSchema = {
  type: "object",
  additionalProperties: false,
  properties: {
    workspaceId: { type: "string" },
    parentViewId: { type: "string" },
    cursor: { type: "string" },
    limit: { type: "integer", enum: [8, 16, 32, 64], default: 32 },
    depth: { type: "integer", enum: [1, 2, 3, 4], default: 3 }
  }
};

const modelItemSchema: JsonSchemaNode = {
  type: "object",
  additionalProperties: false,
  required: ["viewId", "title", "layout", "isSpace", "depth"],
  properties: {
    viewId: { type: "string" },
    parentViewId: { oneOf: [{ type: "string" }, { type: "null" }] },
    title: { type: "string" },
    layout: { type: "string", enum: ["document", "grid", "board", "calendar", "chat"] },
    isSpace: { type: "boolean" },
    depth: { type: "integer" }
  }
};

const modelOutputSchema: JsonSchemaNode = {
  type: "object",
  additionalProperties: false,
  required: ["protocol", "workspaceId", "rootViewId", "truncated", "items"],
  properties: {
    protocol: { type: "string" },
    workspaceId: { type: "string" },
    rootViewId: { type: "string" },
    truncated: { type: "boolean" },
    nextCursor: { type: "string" },
    items: { type: "array", items: modelItemSchema }
  }
};

export const appFlowyWorkspaceQueryDefinition: MusePluginDefinition = {
  pluginId: "muse.appflowy.workspace",
  version: "1.0.0",
  bridgeMajor: 1,
  targets: [{
    familyId: WORKSPACE_FAMILY,
    contract: { major: 1, minMinor: 0, maxMinor: 0 },
    requiredOperations: [WORKSPACE_TREE_OPERATION],
    tools: [{
      name: APPFLOWY_WORKSPACE_TREE_TOOL,
      description:
        "List bounded AppFlowy folder views (id, title, layout) from the Host sidebar catalog or Cloud collab. Does not read document bodies or DSH cwd files. Use cursor when truncated is true. Read a document page with muse_document_read (resourceRef = viewId).",
      operationId: WORKSPACE_TREE_OPERATION,
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

type PromptHost = Context & {
  systemPrompt?: { context(input: { name: string; order: number; text: () => string }): () => void };
};

export const createAppFlowyWorkspaceQueryPlugin = (
  config: MusePluginRuntimeConfig = {}
) => {
  const inner = createMusePlugin(appFlowyWorkspaceQueryDefinition, config);
  return {
    name: inner.name,
    inject: ["museHost", "tools", "systemPrompt"] as const,
    async apply(ctx: Context): Promise<void> {
      const prompt = (ctx as PromptHost).systemPrompt;
      if (prompt === undefined) {
        throw new Error("MUSE_SYSTEM_PROMPT_UNAVAILABLE");
      }
      ctx.effect(
        () => prompt.context({
          name: "muse.appflowy.workspace",
          order: 42,
          text: () => WORKSPACE_LIST_PROMPT
        }),
        "muse.appflowy.workspace.prompt"
      );
      await inner.apply(ctx);
    }
  };
};

const loaderPlugin = createAppFlowyWorkspaceQueryPlugin({
  ...(process.env.MUSE_PLUGIN_DIAGNOSTICS === "1"
    ? { onDiagnostic: diagnostic => process.stderr.write(`[muse-workspace] ${JSON.stringify(diagnostic)}\n`) }
    : {})
});

export const name = loaderPlugin.name;
export const inject = loaderPlugin.inject;
export const apply = (ctx: Context): Promise<void> => loaderPlugin.apply(ctx);
