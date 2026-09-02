# `@muse/plugin-appflowy-workspace` 技术说明

**路径：** `packages/plugins/appflowy-workspace`  
**逻辑 Plugin：** `muse.appflowy.workspace`  
**合同族：** `muse.workspace`

## 定位

AppFlowy **工作区** Plugin。不是 DSH 文件树。cwd 只有 README；页面真源是 Cloud folder collab。

| 面 | 模块 | 职责 |
|---|---|---|
| Identity | `identity.ts` | hint 文件 / Web bind 钉 DSH cwd，防删除 |
| Domain 适配 | `cloud.ts` / `host.ts` | `workspace.current.query`、`workspace.tree.query` → Cloud HTTP 或 E2E fixture |
| Agent Facet | `dsh.ts` | Tool `muse_workspace_list_views` + Prompt（禁止 glob/ls） |
| Web 列表 | `views.ts` | `GET /muse/v1/workspace/views` 的业务实现（HTTP 路由仍由 Web Tx 挂上） |

## 与装配袋

`@muse/dsh-appflowy/connector` 只把 `e2eWorkspaceProvider` / `createCloudWorkspaceProvider` 插进 InProcess Host。Desktop UDS 走 Rust Host，不经过这些 provider。

## 验证

```bash
cd packages/plugins/appflowy-workspace && pnpm check
```
