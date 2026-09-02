# plugins

具体的业务。这些包认识 AppFlowy 领域对象，通过 `@muse/host-bridge` 说话，不在 DSH 里写 CRDT。

| 目录 | npm | 角色 |
|---|---|---|
| [`appflowy-workspace`](appflowy-workspace) | `@muse/plugin-appflowy-workspace` | 工作区身份绑定 + 列页面 Tool |
| [`appflowy-markdown`](appflowy-markdown) | `@muse/plugin-appflowy-markdown` | 文档读/写 Tool + 投影 |
| [`appflowy-database`](appflowy-database) | `@muse/plugin-appflowy-database` | 表格合同示例 |
| [`appflowy-view-reference`](appflowy-view-reference) | `@muse/plugin-appflowy-view-reference` | 当前 View 引用 |
| [`appflowy-view-rename`](appflowy-view-rename) | `@muse/plugin-appflowy-view-rename` | View 重命名 |
| [`dsh-appflowy`](dsh-appflowy) | `@muse/dsh-appflowy` | **装配包**：`cordis.patch.yml` + connector；parent-bridge 仅 Web Tx |

`platforms/` 预留端专用业务实现。Web Tx（parent-bridge）仍由装配包按环境开关加载，不在 Desktop 注册 HTTP。
