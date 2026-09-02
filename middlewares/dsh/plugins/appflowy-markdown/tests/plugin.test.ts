import { describe, expect, it } from "vitest";
import { assertMuseToolSchemas } from "@muse/plugin-kit";
import {
  APPFLOWY_MARKDOWN_APPLY_TOOL,
  APPFLOWY_MARKDOWN_PROPOSE_TOOL,
  APPFLOWY_MARKDOWN_READ_TOOL,
  appFlowyMarkdownDefinition
} from "../src/index.js";

describe("AppFlowy Markdown Plugin", () => {
  it("exposes a target-free read Tool and policy-gated propose/apply writes", () => {
    const tools = appFlowyMarkdownDefinition.targets[0]!.tools;
    expect(tools.map(tool => tool.name)).toEqual([
      APPFLOWY_MARKDOWN_READ_TOOL,
      APPFLOWY_MARKDOWN_PROPOSE_TOOL,
      APPFLOWY_MARKDOWN_APPLY_TOOL
    ]);
    expect(tools[0]!.policy).toBeUndefined();
    expect(tools[1]!.policy).toBeUndefined();
    expect(tools[2]!.policy).toEqual({ effect: "local_write" });
    const publicContract = JSON.stringify(tools.map(tool => [tool.parameters, tool.output]));
    expect(publicContract).not.toMatch(/workspaceId|viewId|actorId|grantId|documentId|crdt/iu);
    for (const tool of tools) assertMuseToolSchemas(tool.parameters, tool.output);
  });
});
