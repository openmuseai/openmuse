# core

只定义公共，不定义具体的实现。这里的包可被任意业务 Plugin 依赖，本身不认识 Markdown、viewId、AppFlowy UI。

| 目录 | npm | 角色 |
|---|---|---|
| [`protocol/host-bridge`](protocol/host-bridge) | `@muse/host-bridge` | Host Bridge 元协议、codec、运输 |
| [`plugin-facets`](plugin-facets) | `@muse/plugin-facets` | Facet envelope 合同 |
| [`plugin-kit`](plugin-kit) | `@muse/plugin-kit` | discover / bind / Tool 生命周期 |
| [`plugin-graph`](plugin-graph) | `@muse/plugin-graph` | Composition Plan |
| [`context-broker`](context-broker) | `@muse/context-broker` | 上下文投影 → Prompt |
| [`contract-document`](contract-document) | `@muse/contract-document` | `muse.document` 合同 |
| [`dsh-runtime`](dsh-runtime) | `@muse/dsh-runtime` | DSH 运行时装配（无 AppFlowy 类型） |
| [`remote-runtime`](remote-runtime) | `@muse/remote-runtime` | `muse.remote` 会话契约 |
| [`plugin-security`](plugin-security) | `@muse/plugin-security` | 安装/策略校验 |
| [`ecosystem-sdk`](ecosystem-sdk) | `@muse/ecosystem-sdk` | 清单校验 CLI |

各包说明见对应目录 `TECH.zh-CN.md`，总表 [TECH_MAP.zh-CN.md](../TECH_MAP.zh-CN.md)。

`platforms/web`、`platforms/desktop` 预留端专用核心实现，当前为空。
