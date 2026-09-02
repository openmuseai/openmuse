import { describe, expect, it } from "vitest";
import { assertMuseToolSchemas } from "@muse/plugin-kit";
import {
  APPFLOWY_VIEW_RENAME_APPLY_TOOL,
  APPFLOWY_VIEW_RENAME_PROPOSE_TOOL,
  appFlowyViewRenameDefinition
} from "../src/index.js";

describe("AppFlowy current View rename Plugin", () => {
  it("exposes only opaque proposal DTOs and marks apply as a Host-policy write", () => {
    const tools = appFlowyViewRenameDefinition.targets[0]!.tools;
    expect(tools.map(tool => tool.name)).toEqual([
      APPFLOWY_VIEW_RENAME_PROPOSE_TOOL,
      APPFLOWY_VIEW_RENAME_APPLY_TOOL
    ]);
    expect(tools[0]!.policy).toBeUndefined();
    expect(tools[1]!.policy).toEqual({ effect: "local_write" });
    const publicContract = JSON.stringify(tools.map(tool => [tool.parameters, tool.output]));
    expect(publicContract).not.toMatch(/workspaceId|viewId|actorId|grantId|crdt/iu);
    for (const tool of tools) assertMuseToolSchemas(tool.parameters, tool.output);
  });
});
