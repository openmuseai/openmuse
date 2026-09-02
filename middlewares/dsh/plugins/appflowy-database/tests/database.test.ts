import { Context } from "@deepseek-ai/cordis";
import ToolRuntime from "@deepseek-ai/dsh-tools";
import SystemPrompt from "@deepseek-ai/dsh-system-prompt";
import { describe, expect, it } from "vitest";
import { InMemoryTableDomainV1, TABLE_READ_TOOL, TABLE_UPDATE_TOOL, createAppFlowyDatabasePlugin } from "../src/index.js";

const execute = (ctx: Context, name: string, args: Record<string, unknown>) => ctx.tools.execute({ callId: `call.${name}` as never, name, arguments: args, agent: { id: "agent.table" } as never, signal: new AbortController().signal });

describe("AppFlowy Database DSH facet", () => {
  it("loads/unloads real Cordis tools and performs revision-checked range/cell flow", async () => {
    const ctx = new Context(); await ctx.plugin(SystemPrompt); await ctx.plugin(ToolRuntime);
    const domain = new InMemoryTableDomainV1("database.1", new Map([["row.1", { name: "Ada", done: false }]]));
    const plugin = await ctx.plugin(createAppFlowyDatabasePlugin(domain));
    expect(ctx.tools.schemas().map(value => value.name)).toEqual([TABLE_READ_TOOL, TABLE_UPDATE_TOOL]);
    const read = await execute(ctx, TABLE_READ_TOOL, { columns: ["name", "done"], rowRefs: ["row.1"] });
    expect(read).toMatchObject({ isError: false, value: { protocol: "muse.table/range-snapshot/v1", revision: "r1" } });
    const updated = await execute(ctx, TABLE_UPDATE_TOOL, { expectedRevision: "r1", rowRef: "row.1", columnRef: "done", value: true, approvalProofRef: "approval.1" });
    expect(updated).toMatchObject({ isError: false, value: { status: "applied", revision: "r2" } });
    const conflict = await execute(ctx, TABLE_UPDATE_TOOL, { expectedRevision: "r1", rowRef: "row.1", columnRef: "done", value: false, approvalProofRef: "approval.2" });
    expect(conflict).toMatchObject({ isError: false, value: { status: "conflict", revision: "r2" } });
    await plugin.dispose(); expect(ctx.tools.schemas()).toEqual([]); await ctx.fiber.dispose();
  });
});
