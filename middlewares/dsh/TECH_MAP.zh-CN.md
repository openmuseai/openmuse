# 包技术地图

每个包一份 `TECH.zh-CN.md`。npm 名不变；目录见 [README](README.md)。

**先读：** 若困惑 `@muse/dsh-appflowy` 为何什么都有，见 [plugins/dsh-appflowy/docs/ROLE.zh-CN.md](plugins/dsh-appflowy/docs/ROLE.zh-CN.md)。它是装配袋，不是逻辑 Plugin。

## core（公共，无 AppFlowy 业务类型）

| 包 | 文档 |
|---|---|
| `@muse/host-bridge` | [core/protocol/host-bridge/TECH.zh-CN.md](core/protocol/host-bridge/TECH.zh-CN.md) |
| `@muse/plugin-facets` | [core/plugin-facets/TECH.zh-CN.md](core/plugin-facets/TECH.zh-CN.md) |
| `@muse/plugin-kit` | [core/plugin-kit/TECH.zh-CN.md](core/plugin-kit/TECH.zh-CN.md) |
| `@muse/plugin-graph` | [core/plugin-graph/TECH.zh-CN.md](core/plugin-graph/TECH.zh-CN.md) |
| `@muse/context-broker` | [core/context-broker/TECH.zh-CN.md](core/context-broker/TECH.zh-CN.md) |
| `@muse/contract-document` | [core/contract-document/TECH.zh-CN.md](core/contract-document/TECH.zh-CN.md) |
| `@muse/dsh-runtime` | [core/dsh-runtime/TECH.zh-CN.md](core/dsh-runtime/TECH.zh-CN.md) |
| `@muse/remote-runtime` | [core/remote-runtime/TECH.zh-CN.md](core/remote-runtime/TECH.zh-CN.md) |
| `@muse/plugin-security` | [core/plugin-security/TECH.zh-CN.md](core/plugin-security/TECH.zh-CN.md) |
| `@muse/ecosystem-sdk` | [core/ecosystem-sdk/TECH.zh-CN.md](core/ecosystem-sdk/TECH.zh-CN.md) |

## plugins（具体业务）

| 包 | 文档 |
|---|---|
| `@muse/plugin-appflowy-markdown` | [plugins/appflowy-markdown/TECH.zh-CN.md](plugins/appflowy-markdown/TECH.zh-CN.md) |
| `@muse/plugin-appflowy-workspace` | [plugins/appflowy-workspace/TECH.zh-CN.md](plugins/appflowy-workspace/TECH.zh-CN.md) |
| `@muse/plugin-appflowy-database` | [plugins/appflowy-database/TECH.zh-CN.md](plugins/appflowy-database/TECH.zh-CN.md) |
| `@muse/plugin-appflowy-view-reference` | [plugins/appflowy-view-reference/TECH.zh-CN.md](plugins/appflowy-view-reference/TECH.zh-CN.md) |
| `@muse/plugin-appflowy-view-rename` | [plugins/appflowy-view-rename/TECH.zh-CN.md](plugins/appflowy-view-rename/TECH.zh-CN.md) |
| `@muse/dsh-appflowy` | [plugins/dsh-appflowy/TECH.zh-CN.md](plugins/dsh-appflowy/TECH.zh-CN.md) · [ROLE](plugins/dsh-appflowy/docs/ROLE.zh-CN.md) |

依赖方向必须向下：业务 Facet → plugin-kit → host-bridge。Bridge 永不 import Plugin。
