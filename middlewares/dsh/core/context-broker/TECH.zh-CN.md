# `@muse/context-broker` 技术说明

**路径：** `packages/core/context-broker`  
**角色：** 把 Context Contribution **投影进 `systemPrompt.context`**，带预算、TTL、pin。

## 定位

DSH Plugin Inbox 的一类消费者：入站 context 合同 **不进 agent-loop 聊天记录**，只更新投影库存；下一次模型请求才看见文本。

工作方式：

1. Facet `registerProjection({ pluginId, contextType, schemaDigest, render })`
2. `ingestContribution(envelope)` 校验 `@muse/plugin-facets` 信封后入库
3. 按 priority / token 预算拼进 Cordis `systemPrompt.context`
4. `pinSurface` / `removeSurface` 管焦点与关闭

**认：** envelope 与投影函数。  
**不认：** viewId 业务含义、iframe、cwd。

## 公共表面

| 导出 | 内容 |
|---|---|
| `@muse/context-broker` | `MuseContextBroker` 纯逻辑 |
| `@muse/context-broker/dsh` | Cordis 插件，`inject: [museHost, systemPrompt]`；监听 Host `provider.event` |

Web 现状：parent-bridge 调用 `ingestContribution` 时伪造 subscription，承认入站本应是协议事件。目标是 Inbox 直接投递。

## 验证

```bash
cd packages/core/context-broker && pnpm check
```
