# `@muse/plugin-appflowy-workspace` 技术说明

**路径：** `packages/plugins/appflowy-workspace`  
**逻辑 Plugin：** `muse.appflowy.workspace`  
**合同族：** `muse.workspace`

## 定位

AppFlowy **工作区** Plugin。不是 DSH 文件树，也不是嵌入 Attachment SM 的胶水。
cwd 只有 README；页面真源是 Host 侧栏 catalog（P0）与 Cloud folder collab（P1）。

| 面 | 模块 | 职责 |
|---|---|---|
| Identity | `identity.ts` | hint 文件 / Web bind 钉 DSH cwd，防删除；换工作区时清 catalog |
| Catalog | `catalog.ts` | Host `workspace.catalog` 有界投影（id/title/layout，≤64） |
| Domain 适配 | `cloud.ts` / `host.ts` | `workspace.current.query`、`workspace.tree.query`：Cloud 优先，未接线则 catalog |
| Agent Facet | `dsh.ts` | Tool `muse_workspace_list_views` + Prompt（禁止 glob/ls） |
| Web 列表 | `views.ts` | `GET /muse/v1/workspace/views`（HTTP 路由仍由 Web Tx 挂上） |

## Host ↔ DSH

1. **挂载（Identity）**：`workspace.bind` / hint 文件 → `applyWorkspaceHint` → `$DSH_HOME/appflowy-workspaces/<id>` + README。这只钉 scratch cwd。
2. **目录（Catalog Facet）**：HybridLive 之后 Host 投递 `workspace.catalog`。parent-bridge 在 inbox 之前 `rememberWorkspaceCatalogFromEnvelope`。
3. **工具（Domain）**：`muse_workspace_list_views` → `workspace.tree.query` → Cloud `POST /api/muse/workspace/tree`；`UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED` / `NOT_FOUND` 时投影 catalog。`SCOPE_MISMATCH` 不回退。
4. **`workspace.tree.ui` 不是目录**：那是侧栏展开 id 列表，不能拿来列页面。

## 与装配袋

`@muse/dsh-appflowy/connector` 只把 `e2eWorkspaceProvider` / `createCloudWorkspaceProvider` 插进 InProcess Host。Desktop UDS 走 Rust Host，不经过这些 provider。

## 验证

```bash
cd packages/plugins/appflowy-workspace && pnpm check
```
