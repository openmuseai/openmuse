import { Context } from "@deepseek-ai/cordis";
import SystemPrompt from "@deepseek-ai/dsh-system-prompt";
import ToolRuntime from "@deepseek-ai/dsh-tools";
import { MuseHostService } from "@muse/host-bridge/dsh";
import { describe, expect, it } from "vitest";
import { InProcessAppFlowyConnector } from "../src/connector.js";
import {
  APPFLOWY_WORKSPACE_TREE_TOOL,
  WORKSPACE_LIST_PROMPT,
  createAppFlowyWorkspaceQueryPlugin,
  inject as workspaceQueryInject
} from "@muse/plugin-appflowy-workspace/dsh";

describe("AppFlowy workspace tree Tool (W7)", () => {
  it("tells the model to list Cloud folder views, not DSH cwd files", () => {
    expect(workspaceQueryInject).toEqual(["museHost", "tools", "systemPrompt"]);
    expect(WORKSPACE_LIST_PROMPT).toContain("muse_workspace_list_views");
    expect(WORKSPACE_LIST_PROMPT).toContain("glob/ls will not list pages");
    expect(WORKSPACE_LIST_PROMPT).toContain("README.md");
  });
  it("lists bounded folder views from the workspace family, not document bodies", async () => {
    const ctx = new Context();
    await ctx.plugin(SystemPrompt);
    await ctx.plugin(ToolRuntime);
    await ctx.plugin(InProcessAppFlowyConnector as never, {} as never);
    await ctx.plugin(MuseHostService as never, { autoStart: false } as never);
    await ctx.museHost.connect();
    const diagnostics: { phase: string; code: string }[] = [];
    const plugin = await ctx.plugin(createAppFlowyWorkspaceQueryPlugin({
      onDiagnostic: diagnostic => {
        diagnostics.push({ phase: diagnostic.phase, code: diagnostic.code });
      }
    }));
    expect(diagnostics, JSON.stringify(diagnostics)).toContainEqual({ phase: "active", code: "BOUND" });
    expect(ctx.tools.schemas().map(value => value.name)).toEqual([APPFLOWY_WORKSPACE_TREE_TOOL]);
    const listed = await ctx.tools.execute({
      callId: "call.list" as never,
      name: APPFLOWY_WORKSPACE_TREE_TOOL,
      arguments: {},
      agent: { id: "agent.e2e" } as never,
      signal: new AbortController().signal
    });
    if (listed.isError) throw new Error(JSON.stringify(listed));
    expect(listed).toMatchObject({
      isError: false,
      value: { protocol: "muse.workspace/tree/v1", truncated: false }
    });
    expect(JSON.stringify(listed.value)).toContain("Getting started");
    expect(JSON.stringify(listed.value)).not.toContain("text/markdown");
    await plugin.dispose();
    await ctx.fiber.dispose();
  });
});
