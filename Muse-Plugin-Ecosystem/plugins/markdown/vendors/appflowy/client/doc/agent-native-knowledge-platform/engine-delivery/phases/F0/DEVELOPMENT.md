# F0 开发文档

## 1. 工作包与顺序

| 包 | 状态 | 设计/开发产物 | 退出条件 |
|---|---|---|---|
| F0.1 Contract Freeze | **Done** | 三合同 schema、TS/Rust API、golden、digest、状态机/receipt ledger | 三包 `pnpm check` 通过 |
| F0.2 Host Resource | **Done (Core)** | Registry、Provider binding、materialization manager、commit/event、lease/disposer | 25 tests；详情见 F0.2 子阶段 |
| F0.3 DSH Seam | Done | Service Definition、Host Provider、UI/Agent Consumer、durable session event | 20 Muse tests + 1 DSH Session test；三 UI 入口、agent snapshot、未知事件 reload 兼容通过 |
| F0.4 Client Orchestrator | Todo | Dart projection、Adapter Registry、route/probe、fake Adapter/Surface、receipt | fake 端到端与 leak=0 |
| F0.5 Web/Mobile Bridge | Todo | parent envelope adapter、remote materialization、端能力 profile | Desktop/Web/Mobile 矩阵通过 |
| F0.6 Hardening/Gate | Todo | chaos、perf、安全、flags、rollback drill、证据收敛 | F0 Gate 批准 |

依赖顺序：F0.1 → F0.2/F0.3/F0.4；F0.4 → F0.5；全部 → F0.6。F0.2 与 F0.3 可在 schema freeze 后并行；真实 Viewer/Word/Helix 接入必须等 F0 Gate。

## 2. F0.1 已实施内容

### 新增路径

```text
Muse-Clients/middlewares/dsh/core/
├─ contract-resource/
├─ contract-presentation/
└─ contract-engine-session/
```

每包包含：

- `schemas/`：权威 JSON Schema；
- `src/`：严格 TypeScript branded types、validator/digest；
- `fixtures/`：正负 golden 与 schema digest；
- `rust/`：Rust serde 投影、同 schema validator/digest；
- `tests/`：共享 fixture、round-trip 和行为不变量；
- `scripts/`：合同泄漏检查与 digest 生成；
- `README.md`、`TECH.zh-CN.md`：包级使用和技术限制。

关键实现：

- Resource 控制面拒绝 raw path/bearer URL；revision 保持 opaque。
- Presentation request 不接受 engine id；route/receipt 公开实际降级。
- `PresentationReceiptLedger` 参考实现保证一个 request 一个终态。
- Engine manifest 的 `edit` 必须同时声明 `commit`。
- EngineSession 状态机禁止只读进入 dirty/commit，禁止终态复活。
- TS/Rust 使用相同 schema digest 算法和 fixture。

## 3. 后续工作包的具体改动

### F0.2 Host Resource（已完成 Core）

已新建 `middlewares/dsh/core/resource-host/`，没有修改 vendor：

1. `ResourceProviderRegistry`：按 provider instance/generation 注册，重复 key 拒绝；effect/disposer 撤销。
2. `ResourceAuthorityService`：authorize + describe，binding 绑定 actor/workspace/session。
3. `MaterializationManager`：签发 handle，记录 audience/revision/mode/TTL；read/revoke/dispose。
4. `CommitCoordinator`：expected revision + idempotency，先只接 fake Provider。
5. `ResourceEventLog`：cursor/retention/snapshot recovery。
6. `LeakCounters`：registry/provider/subscription/materialization 全部可断言。

禁止在本包加入路由偏好或 vendor 内容解析。

### F0.3 DSH Seam

在 DSH 上游规则允许的包边界实现完整 seam：

1. Service Definition：`OpenResourceService.request/status/cancel`，schema 指向 Presentation v2。
2. Host Provider：Host Bridge client；从 DSH session coordinates 绑定 authority。
3. UI Consumer：deliverable、mention、card 只转换 intent，不解释扩展名。
4. Agent Consumer：`present_resource` 的 schema、recent-resource allowlist、model-visible request/result event。
5. Reload recovery：从 durable terminal receipt 恢复 UI。
6. Cordis effect/disposer 与缺 Provider 失败测试。

非平凡 DSH PR 单独提供 Agent Note、product-visible snapshot 和该仓库要求的覆盖率证据。

### F0.4 Client Orchestrator

建议 Flutter 包：`muse_resource_contract`、`muse_engine_adapter`、`muse_surface_orchestrator`。

1. 从 schema/fixture 手写或生成 Dart sealed types，禁止动态 Map 穿透业务层。
2. Adapter registry 按 `adapterId + version + generation` 管理。
3. Probe fan-out 可取消且有 concurrency cap；缓存 key 含 runtime/policy/placement digest。
4. Route score 仅基于 policy/capability/placement/quality，不读取 extension 直接选 engine。
5. Open saga 为每一步登记 disposer；首个 terminal receipt 原子提交。
6. Fake Resource/Adapter/Surface 可注入每个步骤失败、延迟、late event。

### F0.5 Web/Mobile Bridge

1. parent bridge 只转发 Host Bridge envelope；不增加 `word-open/viewer-open/helix-open` 私有消息。
2. Web 只接受 audience/origin/TTL 合法的 remote-url/stream handle。
3. Mobile capability profile 只发布 view + range/pagination。
4. 相同 request/cancel/status 语义做跨端契约测试。

### F0.6 Hardening

1. duplicate/cancel/ready race/plugin reload/Host restart/revoke fault matrix。
2. 1000 次 open-close soak，所有计数归零。
3. envelope size、日志脱敏、URL/path/token 静态与运行时扫描。
4. warm fake latency 与 disposer latency histogram。
5. feature flag shadow、cohort、rollback drill。
6. 汇总 `RESULTS.md`，由 Protocol/Host、DSH、Client Platform、QA、Security owner 签署。

## 4. Migration 与 feature flag

- `resourcePresentationV2` 默认 `internal/shadow`；shadow 只计算 route，不创建第二 Surface。
- 旧 `openFile(path)` 暂留，但只能由已验证的旧 Consumer fallback 调用；新协议/日志不得写 path。
- 三个 vendor flag 在 F0 均保持 off。
- schema package 首次发布只供 repo 内 link；F0 Gate 后再决定 workspace registry/versioning。

## 5. 构建与验证命令

```bash
cd Muse-Clients/middlewares/dsh/core/contract-resource && pnpm install && pnpm digest:update && pnpm check
cd Muse-Clients/middlewares/dsh/core/contract-presentation && pnpm install && pnpm digest:update && pnpm check
cd Muse-Clients/middlewares/dsh/core/contract-engine-session && pnpm install && pnpm digest:update && pnpm check
```

修改 schema 后必须先运行 `digest:update` 并审阅 digest diff；只改类型而不更新 schema 属于失败。生成目录 `dist/node_modules/rust/target` 不作为交付源文件。

## 6. Owner 与阻塞项

| 项 | Owner | 当前阻塞 |
|---|---|---|
| Schema/Host | Protocol/Host | F0.1/F0.2 Core 完成；生产 Provider 待后续绑定 |
| DSH seam | DSH | 需确认上游包落点与 coverage gate |
| Dart/Orchestrator | Client Platform | 尚未建立 Dart schema projection |
| Web/Mobile | Client Platform | 依赖 F0.4 |
| Gate | QA + Security/Release | 依赖 F0.2–F0.5 |

Ontology Runtime 不设 owner、不构成阻塞。
