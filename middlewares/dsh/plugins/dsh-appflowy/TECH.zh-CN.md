# `@muse/dsh-appflowy` 技术说明

**路径：** `packages/plugins/dsh-appflowy`  
**角色：** DSH **装配包**，不是逻辑 Plugin。

职责与 Desktop vs Web：[docs/ROLE.zh-CN.md](docs/ROLE.zh-CN.md)。

## 留下的模块

| 模块 | 职责 |
|---|---|
| `cordis.patch.yml` | 插入清单 |
| `connector.ts` | 打开 UDS（Desktop）或把 **Plugin 提供的** InProcess provider 装进假 Host（Cloud/E2E） |
| `composition-host.ts` | 通用 hello/discover/bind/invoke 多路复用，无文档/树 fixture |
| `parent-bridge.ts` | **仅 Web Tx**。Desktop `apply()` 直接返回（无 `MUSE_DOCUMENT_CLOUD_URL`） |
| `webview.ts` | WKWebView `@scope` URL 补丁 |
| `approval.ts` | 部署方 HMAC |
| `session.ts` | Web device token / document focus 内存（Desktop 不用） |

业务已迁出：`@muse/plugin-appflowy-workspace`、`@muse/plugin-appflowy-markdown` 的 `host`/`cloud`。

## 验证

```bash
cd packages/plugins/dsh-appflowy && pnpm check
```
