import { describe, expect, it } from "vitest";
import { Context } from "@deepseek-ai/cordis";
import ToolRuntime from "@deepseek-ai/dsh-tools";
import SystemPrompt from "@deepseek-ai/dsh-system-prompt";
import { MuseHostService } from "@muse/host-bridge/dsh";
import {
  APPFLOWY_MARKDOWN_APPLY_TOOL,
  APPFLOWY_MARKDOWN_PROPOSE_TOOL,
  APPFLOWY_MARKDOWN_READ_TOOL
} from "@muse/plugin-appflowy-markdown";
import { createAppFlowyMarkdownPlugin } from "@muse/plugin-appflowy-markdown/dsh";
import { InProcessAppFlowyConnector } from "../src/connector.js";

const execute = (ctx: Context, name: string, args: Record<string, unknown>) => ctx.tools.execute({
  callId: `call.${name}` as never,
  name,
  arguments: args,
  agent: { id: "agent.e2e" } as never,
  signal: new AbortController().signal
});

describe("M05 deterministic DSH → Bridge → document@2 vertical slice", () => {
  it("loads public Cordis services and completes read/propose/approve/apply with one receipt", async () => {
    const ctx = new Context();
    await ctx.plugin(SystemPrompt);
    await ctx.plugin(ToolRuntime);
    await ctx.plugin(InProcessAppFlowyConnector as never, {} as never);
    await ctx.plugin(MuseHostService as never, { autoStart: false } as never);
    await ctx.museHost.connect();
    const plugin = await ctx.plugin(createAppFlowyMarkdownPlugin({
      approvalProofRequester: { request: async () => ({ outcome: "approved", proofId: "proof.e2e" }) }
    }));
    expect(ctx.tools.schemas().map(value => value.name)).toEqual([
      APPFLOWY_MARKDOWN_READ_TOOL, APPFLOWY_MARKDOWN_PROPOSE_TOOL, APPFLOWY_MARKDOWN_APPLY_TOOL
    ]);
    const read = await execute(ctx, APPFLOWY_MARKDOWN_READ_TOOL, {});
    if (read.isError) throw new Error(JSON.stringify(read));
    expect(read).toMatchObject({ isError: false, value: { protocol: "muse.document/snapshot/v2", resourceRef: "document.e2e" } });
    const revision = (read.value as { revision: string }).revision;
    const proposed = await execute(ctx, APPFLOWY_MARKDOWN_PROPOSE_TOOL, { expectedRevision: revision, kind: "insert", text: "Muse" });
    expect(proposed).toMatchObject({ isError: false, value: { protocol: "muse.document/proposal/v2", proposalRef: "proposal.e2e" } });
    const applied = await execute(ctx, APPFLOWY_MARKDOWN_APPLY_TOOL, { proposalRef: "proposal.e2e" });
    expect(applied).toMatchObject({ isError: false, value: { protocol: "muse.document/receipt/v2", status: "applied", eventCursor: "1" } });
    await plugin.dispose();
    expect(ctx.tools.schemas()).toEqual([]);
    await ctx.fiber.dispose();
  });
});
