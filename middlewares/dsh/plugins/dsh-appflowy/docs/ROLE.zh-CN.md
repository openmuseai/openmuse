# `@muse/dsh-appflowy` 是什么（以及不是什么）

> 对照：[对等插件架构](../../../../docs/platforms/PLUGIN_PEER_ARCHITECTURE.zh-CN.md)。

## 1. 结论

**`@muse/dsh-appflowy` 不是逻辑 Plugin。** 它是 DSH 进程里的 **装配包**。逻辑 Plugin 在 `packages/plugins/appflowy-*`。

## 2. Desktop 不需要 parent-bridge

Desktop 已经有运输：Flutter 写 hint 文件（workspace identity）+ Rust Host UDS（connector）。DSH sidecar 里的 HTTP parent-bridge **对 Desktop 是多余的**，也不该当业务 switch。

`parent-bridge.ts` 仍放在装配袋，是因为 **Web/Remote** 的浏览器不能连 UDS，iframe 需要 HTTP/SSE 管道。`apply()` 在没有 `MUSE_DOCUMENT_CLOUD_URL`（且未设 `MUSE_PARENT_BRIDGE=1`）时直接返回，Desktop 不会注册那些路由。

合法残留（仅 Web）：体积/来源校验、HTTP 路由、SSE、注入脚本。bind / token / contribute / 列树的 **实现** 已在 workspace Plugin 与 session 模块；parent-bridge 只做 Tx 入口。

## 3. Workspace 已是独立 Plugin

`packages/plugins/appflowy-workspace`（`muse.appflowy.workspace`）：

| 模块 | 职责 |
|---|---|
| `identity.ts` | cwd pin、hint 文件 |
| `cloud.ts` / `host.ts` | Domain：tree/current → Cloud 或 E2E fixture |
| `dsh.ts` | Tool `muse_workspace_list_views` |
| `views.ts` | 列页面业务（Web GET 由 parent-bridge 挂路由） |

## 4. connector 不再内嵌业务 fixture

InProcess Host 只多路复用 Plugin 注册的 provider：`e2eMarkdownProvider` / `createCloudMarkdownProvider`、`e2eWorkspaceProvider` / `createCloudWorkspaceProvider`。Desktop 走 UDS，不经过这些 fixture。
