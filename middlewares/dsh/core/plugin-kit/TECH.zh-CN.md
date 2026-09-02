# `@muse/plugin-kit` 技术说明

**路径：** `packages/core/plugin-kit`  
**角色：** 把 Host capability **发布成 DSH Tool** 的通用运行时。业务 Plugin 的 DSH Facet 几乎都通过它创建。

## 定位

`createMusePlugin(definition, config)`：

1. `inject: ["museHost", "tools"]`
2. discover → 按 `familyId` + 合同范围选 descriptor
3. bind（scope hint 如 `appflowy.selection=current`，**模型不能选 viewId**）
4. 把 definition 里的 operation 注册为 Tool；invoke 走 `museHost.invoke`
5. 写操作先走 Host policy；需要时向 Host 要 grant

**认：** Plugin definition、scope hint、Tool 适配器。  
**不认：** AppFlowy、CRDT、具体 document schema（那是 Plugin + `contract-document`）。

Bridge **永不** import 本包。

## 公共表面

| 导出 | 内容 |
|---|---|
| `@muse/plugin-kit` | `MusePluginDefinition`、`InProcessDomainProvider`（Cloud/E2E composition）、错误、manifest 辅助 |
| `@muse/plugin-kit/dsh` | `createMusePlugin`、`MusePluginRuntime` |

核心实现：`src/dsh/runtime.ts`（prepare / bind / invoke / dispose）。

## 边界

`scripts/check-boundaries.mjs`：禁止产品域名、禁止 `from "@muse/host-bridge/src"`。

## 验证

```bash
cd packages/core/plugin-kit && pnpm check
```
