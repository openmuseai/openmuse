# F0.3 开发说明与计划执行

## 1. 交付拆分

| Workstream | 实现位置 | 状态 | 完成定义 |
|---|---|---|---|
| F0.3-A Service Definition | `middlewares/dsh/core/dsh-resource-presentation` | Done | API、wire schema digest、journal/replay |
| F0.3-B Host Provider | `middlewares/dsh/plugins/dsh-resource-presentation-host` | Done | discover/bind/invoke/status/cancel/lifecycle |
| F0.3-C UI Consumer | `middlewares/dsh/plugins/dsh-client-ui-resource-open` | Done | 三入口同函数、recovery |
| F0.3-D Agent Consumer | `middlewares/dsh/plugins/dsh-tool-resource-present` | Done | recent allowlist、ToolRuntime、model-safe output |
| F0.3-E Bundle | build maps、wire scripts、`dsh-appflowy/cordis.patch.yml` | Done | 打包/运行时依赖可解析 |
| F0.3-F Docs/Gate | 本目录 | Done | 方案、开发、fixture、矩阵、结果、发布 |
| F0.3-G DSH Persistence Compatibility | `vendors/deepseek-harness/packages/core/session` | Done | 非 Surface 可忽略事件可写；Surface 事件拒绝该标记 |

## 2. 代码设计

### A. 公共 Service

- `service.ts`：Cordis `OpenResourceService` 抽象服务。
- `wire.ts`：family/operation/schema digest；request 直接复用 Presentation v2 schema。
- `journal.ts`：创建 requestRef、记录 discovery、统一 journaled request、replay projection。
- `types.ts`：source/status/cancel/recent policy；Ontology 只有 inert type。

三类 Muse journal event 通过 DSH `Session.append` 的非 Surface `ignorable:true` intent 写入。
它们缺失只会关闭 Muse recent-ref 或 UI recovery，不会改变 DSH Session 重建。DSH 上游的
类型签名阻止 Surface event 使用该 intent，运行时同时覆盖 widened/cast 调用；原有 persistence
contract 已验证 JSONL/SQLite 对未知且可忽略事件的 load。

公共包只依赖 contracts/Host Bridge 与 DSH Session，不引用 AppFlowy 或 vendor。

### B. Host Provider

- 连接 `ctx.museHost`；只发现 `muse.resource-presentation` major 2。
- 三个 operation 的 input/output digest 全部匹配才接受 descriptor。
- binding cache key=`sessionId + actorScopeRef + workspaceScopeRef`。
- request 的 idempotencyKey/cancellationId 使用语义 requestRef。
- terminal receipt cache 支持 duplicate/status/cancel 快路径。
- Host event 失效 binding；Cordis effect 负责 dispose。

### C. UI Consumer

三个公开函数调用同一个私有 `open(entry,target)`，再调用公共
`requestPresentation()`。无 Provider 构造立即失败。`recover()` 纯读 Session events。

### D. Agent Tool

- Tool schema 不声明 path/URL/engine/handle/token。
- Runtime 额外校验 opaque ref；必须 `exec.agent` 存在。
- 扫描当前 Session 的 durable discovery event，双 recent window fail-closed。
- 返回 receipt 的安全投影，主动删除 selectedAdapterRef/sessionRef/surfaceInstanceRef/revision。
- Tool 注册由 `ctx.effect()` disposer 回收。

## 3. F0.2 vertical slice

测试 Host 使用 `ResourceHost + FakeResourceProvider` 注册 `resource.fake`。收到 Presentation
invoke 后，Host 以自身 authenticated facts 构造 `ResourceAuthority` 并调用 `describe`，证明
DSH wire input 未携带权限字段也能跨越 F0.2 边界。

F0.4 将把这个 fake terminal receipt 替换为 Fake Orchestrator/Adapter/Surface；F0.3 API 不变。

## 4. Bundle 与加载顺序

```text
muse-host-bridge
  → muse-resource-presentation-host
      → muse-client-ui-resource-open
      → muse-tool-resource-present (+ tools)
```

四个包与 F0.1/F0.2 依赖已加入 middleware local build、客户端 macOS copy/link、Windows
portable `PACKAGE_DIRS` 和 symlink-free closure manifest；两套 wire scripts 新增 `@muse/*`
short name 和 `@deepseek-ai/dsh-session` harness alias。`dsh-appflowy` 声明直接 dependencies，
bundle patch 明确 inject。清单一致性脚本验证 Windows 与 closure 共 21 个 Muse packages 对齐。

## 5. 后续阶段接口

| 后续 | 复用接口 | 禁止反向修改 |
|---|---|---|
| F0.4 Client Orchestrator | Host capability 的 request/status/cancel | 不让 DSH 选择 Adapter |
| F0.5 Web/Mobile | 同 Presentation request + 端 capability | 不增加 `word-open` 私有消息 |
| F0.6 Hardening | counters、fixtures、replay | 不把日志移到 React state |
| Engine phases | receipt/EngineSession contracts | vendor 不认识 DSH/Ontology |

## 6. 开发命令

```bash
pnpm --dir middlewares/dsh/core/dsh-resource-presentation check
pnpm --dir middlewares/dsh/plugins/dsh-resource-presentation-host check
pnpm --dir middlewares/dsh/plugins/dsh-client-ui-resource-open check
pnpm --dir middlewares/dsh/plugins/dsh-tool-resource-present check
pnpm --dir vendors/deepseek-harness exec vitest run packages/core/session/tests/session.spec.ts
middlewares/scripts/build-muse-packages.sh --skip-tests
```
