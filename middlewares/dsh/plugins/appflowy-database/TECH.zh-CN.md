# `@muse/plugin-appflowy-database` 技术说明

**路径：** `packages/plugins/appflowy-database`  
**逻辑 Plugin：** `muse.appflowy.database`  
**合同族：** `muse.table@1`

## 定位

表格领域的 **示例 Plugin**（TCK / SDK 作者指南），不是当前 Web/Desktop Agent 主链上的装配项。`cordis.patch.yml` **未**插入本包。

提供 `table.range.query` / `table.cell.update`、内存 Domain Port、v2 manifest。演示：异构领域数据仍走通用 envelope，Bridge 不出现单元格类型。

配套 Flutter：`muse_table_surface`。

## 验证

```bash
cd packages/plugins/appflowy-database && pnpm check
```
SDK：`packages/core/ecosystem-sdk` 对 `muse.plugin.json` validate/test。
