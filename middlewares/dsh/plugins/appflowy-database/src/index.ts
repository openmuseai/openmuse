import type { Context } from "@deepseek-ai/cordis";
import type {} from "@deepseek-ai/dsh-tools";
import type { MusePluginManifestV2 } from "@muse/plugin-graph";

export const TABLE_CONTRACT = { family: "muse.table", major: 1, minor: 0, operations: { readRange: "table.range.query", updateCell: "table.cell.update" } } as const;
export const TABLE_READ_TOOL = "muse_table_read_range";
export const TABLE_UPDATE_TOOL = "muse_table_update_cell";

export type CellValue = string | number | boolean | null;
export interface TableRangeSnapshotV1 { readonly protocol: "muse.table/range-snapshot/v1"; readonly resourceRef: string; readonly revision: string; readonly columns: readonly string[]; readonly rows: readonly { readonly rowRef: string; readonly cells: Readonly<Record<string, CellValue>> }[]; }
export interface TableUpdateReceiptV1 { readonly protocol: "muse.table/update-receipt/v1"; readonly status: "applied" | "conflict"; readonly resourceRef: string; readonly previousRevision: string; readonly revision: string; readonly rowRef: string; readonly columnRef: string; }
export interface TableDomainPortV1 { invoke(operation: string, input: Readonly<Record<string, unknown>>): Promise<unknown>; }

export class InMemoryTableDomainV1 implements TableDomainPortV1 {
  private revision = 1;
  constructor(readonly resourceRef: string, private readonly rows: Map<string, Record<string, CellValue>>) {}
  async invoke(operation: string, input: Readonly<Record<string, unknown>>): Promise<unknown> {
    if (operation === TABLE_CONTRACT.operations.readRange) {
      const columns = Array.isArray(input.columns) ? input.columns.filter((v): v is string => typeof v === "string").slice(0, 64) : [];
      const rowRefs = Array.isArray(input.rowRefs) ? input.rowRefs.filter((v): v is string => typeof v === "string").slice(0, 256) : [...this.rows.keys()].slice(0, 256);
      return { protocol: "muse.table/range-snapshot/v1", resourceRef: this.resourceRef, revision: `r${this.revision}`, columns,
        rows: rowRefs.flatMap(rowRef => { const row = this.rows.get(rowRef); return row === undefined ? [] : [{ rowRef, cells: Object.fromEntries(columns.map(column => [column, row[column] ?? null])) }]; }) } satisfies TableRangeSnapshotV1;
    }
    if (operation === TABLE_CONTRACT.operations.updateCell) {
      const expected = String(input.expectedRevision); const rowRef = String(input.rowRef); const columnRef = String(input.columnRef);
      if (expected !== `r${this.revision}`) return { protocol: "muse.table/update-receipt/v1", status: "conflict", resourceRef: this.resourceRef, previousRevision: `r${this.revision}`, revision: `r${this.revision}`, rowRef, columnRef } satisfies TableUpdateReceiptV1;
      if (typeof input.approvalProofRef !== "string") throw new Error("TRUSTED_APPROVAL_REQUIRED");
      const row = this.rows.get(rowRef); if (row === undefined) throw new Error("ROW_NOT_FOUND");
      const previousRevision = `r${this.revision}`; row[columnRef] = input.value as CellValue; this.revision += 1;
      return { protocol: "muse.table/update-receipt/v1", status: "applied", resourceRef: this.resourceRef, previousRevision, revision: `r${this.revision}`, rowRef, columnRef } satisfies TableUpdateReceiptV1;
    }
    throw new Error("TABLE_OPERATION_UNKNOWN");
  }
}

export const databaseManifestV2: MusePluginManifestV2 = {
  protocol: "muse.plugin/v2", pluginId: "muse.appflowy.database", version: "1.0.0",
  artifacts: [
    { id: "agent", digest: `sha256:${"d".repeat(64)}`, kind: "npm", entrypoint: "@muse/plugin-appflowy-database" },
    { id: "presentation", digest: `sha256:${"e".repeat(64)}`, kind: "flutter-bundle", entrypoint: "MuseDatabaseUiFacet" },
    { id: "domain", digest: `sha256:${"f".repeat(64)}`, kind: "rust-builtin", entrypoint: "appflowy.database.local" }
  ],
  facets: [
    { id: "agent", kind: "agent", artifact: "agent", runtimes: ["dsh-native", "remote-dsh"], requires: [{ contract: TABLE_CONTRACT.family, range: "^1.0.0" }] },
    { id: "presentation", kind: "presentation", artifact: "presentation", runtimes: ["flutter"], requires: [{ contract: TABLE_CONTRACT.family, range: "^1.0.0" }], contributions: [{ slot: "workspace.editor", contributionId: "database-grid", priority: 90 }] },
    { id: "domain", kind: "domain", artifact: "domain", runtimes: ["rust-host", "remote-dsh"], provides: [{ contract: TABLE_CONTRACT.family, version: "1.0.0", priority: 100 }] }
  ]
};

export const createAppFlowyDatabasePlugin = (domain: TableDomainPortV1) => ({
  name: "muse-appflowy-database", inject: ["tools"] as const,
  apply(ctx: Context): void {
    ctx.effect(() => ctx.tools.register({
      name: TABLE_READ_TOOL, description: "Read a bounded row/column range from the active AppFlowy database.",
      parameters: { type: "object", additionalProperties: false, required: ["columns"], properties: { columns: { type: "array", items: { type: "string" }, maxItems: 64 }, rowRefs: { type: "array", items: { type: "string" }, maxItems: 256 } } },
      output: { schema: { type: "object" }, render: (_args, value) => [{ type: "text", text: JSON.stringify(value) }] },
      execute: args => domain.invoke(TABLE_CONTRACT.operations.readRange, args as Record<string, unknown>)
    }));
    ctx.effect(() => ctx.tools.register({
      name: TABLE_UPDATE_TOOL, description: "Update one AppFlowy database cell at an exact revision with trusted approval.",
      parameters: { type: "object", additionalProperties: false, required: ["expectedRevision", "rowRef", "columnRef", "value", "approvalProofRef"], properties: {
        expectedRevision: { type: "string" }, rowRef: { type: "string" }, columnRef: { type: "string" }, value: { oneOf: [{ type: "string" }, { type: "number" }, { type: "boolean" }, { type: "null" }] }, approvalProofRef: { type: "string" }
      } },
      output: { schema: { type: "object" }, render: (_args, value) => [{ type: "text", text: JSON.stringify(value) }] },
      execute: args => domain.invoke(TABLE_CONTRACT.operations.updateCell, args as Record<string, unknown>)
    }));
  }
});
