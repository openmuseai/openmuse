# F0 具体方案设计

## 1. 目标、事实与非目标

F0 建立 Host、DSH、Flutter/Web Surface 与所有格式引擎之间唯一的公共控制面，使 Markdown、DOCX、代码、PDF 和未来 CAD/三维模型拥有相同的资源身份、打开事务、会话生命周期和终态回执。

当前事实：

- Host Bridge 已提供有界 JSON envelope、opaque ID、schema digest、invoke/subscribe/cancel 基础设施。
- `contract-document` 和 `plugin-facets` 已形成 TypeScript/Rust schema + golden fixture 模式。
- ioffice Word、Helix、open-file-viewer 的运行形态分别是 FFI/内存、PTY/进程、sandbox Web，不能共享数据面。
- ioffice Word 当前没有已验证 `toDocx`，因此只能声明 `view`/`ephemeral-edit`，不得声明 `edit`/`commit`。
- Ontology Runtime 尚未实现，也不是 F0 依赖。

F0 不做格式渲染、Office AST、PTY 协议、PDF 页面模型、真实写回，也不创建 Ontology Object/Link/Action/ImpactSet。

## 2. 用户场景与行为

### S-F0-01：DSH 交付物打开

用户点击 DSH 中的 DOCX、源码或 PDF。DSH Consumer 只提交 `resourceRef + requestedMode + placementHint`；Host 授权并描述资源，Orchestrator probe 候选 Adapter，创建短期 materialization，打开 Surface，最终返回唯一 receipt。UI 显示实际引擎、实际模式和降级原因。

### S-F0-02：Agent 请求呈现

Agent 只能使用本会话中已被授权工具发现的 `resourceRef`。工具请求和终态写入 DSH durable session event；模型看不到路径、bearer URL、materialization handle 或 engine secret。Agent 不能强制选择 engine。

### S-F0-03：重复、取消与恢复

同一 `requestRef` 重放不得产生第二个 Surface。取消后停止新 attempt，已分配对象逆序清理。DSH/Flutter reload 后使用 request/status ref 恢复终态，不重放打开副作用。旧 generation 的 late event 被丢弃。

### S-F0-04：能力降级

用户请求 edit，但 Adapter probe 仅返回 view 时，policy 可选择降级并在 route decision/receipt 中明确 `effectiveMode=view` 和 warning；如果用户明确要求不可降级的 edit，则返回 `unsupported`，不显示虚假 Save。

### S-F0-05：无 Ontology Runtime

完整打开链在没有任何 Ontology 包/服务的构建中工作。可选 `extensions` 或 event detail 只携带惰性事实标签；没有 consumer 时不得报错、排队或重试。

## 3. 领域责任

| 组件 | 必须提供 | 不得提供 |
|---|---|---|
| Resource Authority Host | authorize、describe、materialize、commit、subscribe、revoke | UI 路由偏好、vendor 文档语义 |
| Surface Orchestrator | enumerate/probe/score、attempt、lease、receipt、fallback | 解析路径、Office/PTY/PDF 内容 |
| Engine Adapter | manifest、probe、open/focus/navigate/context/close，可选 commit | 创建资源全局身份、扩大 authority |
| DSH Provider | Service Definition 到 Host Bridge 的绑定、session/actor/workspace scope | 最终授权、直接打开 path |
| DSH Consumer | UI/Agent 意图、持久化请求与终态、可解释 fallback | 根据扩展名选择 vendor、泄露 handle |
| Flutter/Web/Mobile | Surface、输入/焦点/错误、lease | 长期 token、静默写入 |

## 4. 权威合同

| 合同 | 协议版本 | 权威对象 | 关键不变量 |
|---|---|---|---|
| Resource | v1 | descriptor、materialization、commit request/receipt、event | revision opaque；控制面无 path/URL；commit optimistic |
| Presentation | v2 | request、route decision、terminal receipt | caller 不指定 engine；一个 request 一个 terminal receipt |
| EngineSession | v1 | manifest、probe、open request、handle、event | probe 无副作用；generation 隔离；只读无 dirty |

Schema 位于 `Muse-Clients/middlewares/dsh/core/contract-*/schemas/`，JSON Schema 是权威。TS/Rust 类型是投影，不得脱离 schema 单独扩展。

### 4.1 标识与 revision 决策

- 跨边界标识使用 Host Bridge 安全集：`[A-Za-z0-9._~-]{1,128}`。
- `resourceRef`、`revision`、`sessionRef` 均为 opaque branded string。
- 业务打开事务使用 `requestRef/attemptRef`，避免与 Host Bridge 单次传输 envelope 的 `requestId` 混淆；重试和 fallback 可跨多个 envelope 保持同一 root reference。
- versioned `adapterRef` 使用 `adapterId~version`，不把 `@` 扩展进 Host opaque ID 字符集。
- revision 不强制 sha256；Provider 可使用内容摘要、ETag 或数据库版本，但只能比较相等性。
- 内容 digest 与 schema/probe digest 使用 `sha256:<64 lowercase hex>`。

### 4.2 Materialization 决策

公共 envelope 只包含 `handleRef/kind/resourceRef/revision/consumerAdapterRef/accessMode/expiresAt`。真实 path、URL、reader、bytes 或 stream endpoint 由 Host 在已认证、限定 audience 的本地数据面解析。handle 必须绑定 actor/workspace/adapter/resource/revision/mode，并支持 TTL、revoke 和幂等 dispose。

### 4.3 打开事务与状态

```text
request
  → authorize/describe
  → probe candidates
  → route attempt
  → materialize
  → open EngineSession
  → acquire Surface lease
  → ready/focus
  → one terminal receipt
```

任何步骤失败均逆序 dispose。fallback 使用相同 `requestRef` 和新 `attemptRef`；最终 receipt 记录真正成功的 Adapter。

EngineSession 状态：

```text
allocating → materializing → starting → ready ↔ focused/background
                                      └→ dirty → committing → ready/conflict
任意活动态 → closing → closed | forced-terminated
启动阶段 → failed → closing/closed
```

`view` session 不得进入 dirty/committing/conflict。terminal state 不得重新进入 ready。

## 5. 不同端的 F0 功能集

| 端 | F0 必须实现 | F0 明确不做 | 技术验收 |
|---|---|---|---|
| Desktop macOS | Host-local registry、bytes/read-file/working-copy/loopback/stream handle；Flutter main/side Surface | 真正 vendor 编辑与保存 | intent 到 fake Surface；取消/崩溃清零；p95 open 控制面预算 300 ms（不含 engine/data） |
| Web | parent bridge 转发同一 Presentation envelope；remote-url/stream handle | local path、PTY、原生 FFI | schema/size/cancel 一致；CORS/audience 在 Host broker 校验 |
| Mobile | view-only Presentation 子集、remote-url/range、main Surface | Helix、Office edit、后台写入 | 超限明确 `RESOURCE_TOO_LARGE`；低内存路由降级 |
| DSH UI | deliverable/mention/card 共用 Consumer；终态入 session | React 本地私有 open 状态 | 三入口 snapshot 等价；reload 不重复打开 |
| DSH Agent | `present_resource` 仅接受已发现 resourceRef；结果 model-visible | path、engineId、PTY input、Viewer DOM | 输入来源校验；request/result 可重放 |

性能数值是 F0 runtime Gate，不适用于单纯 schema 包；未实现项在测试矩阵保持 Planned/Blocked。

## 6. 错误、取消与并发

错误使用稳定大写 code，owner 明确：Resource (`RESOURCE_NOT_FOUND/DENIED/REVISION_CONFLICT`)、Adapter (`ADAPTER_UNAVAILABLE/ENGINE_START_FAILED`)、Materialization (`MATERIALIZATION_EXPIRED/CONSUMER_MISMATCH`)、Session (`SESSION_OWNERSHIP_CONFLICT`)、通用 (`CANCELLED/DEADLINE_EXCEEDED/UNAVAILABLE`)。

- requestRef 幂等：完全相同的重复请求返回同一 receipt。
- terminal conflict：相同 requestRef 的第二个不同终态是协议错误。
- commit idempotency：相同 key 返回相同 Provider receipt。
- cancel 与 ready race：以 Host 首个原子 terminal 决定为准；败方只清理，不再发布第二终态。
- plugin reload：generation 增加；旧 generation 不能新建 Surface，也不能修改新 session 状态。

## 7. 安全、隐私和生命周期

- schema 默认 `additionalProperties: false`，阻止私加 path/token/vendor payload。
- Host Bridge 不承载完整 DOCX/PDF/模型/PTY 字节。
- Adapter manifest 是声明，不是权限；probe 成功也不替代 Host policy。
- 日志只记录 opaque ref、状态、耗时、error code、digest；不记录 display content、path、URL query 或 token。
- disposer 顺序：Surface lease → EngineSession → data-plane attachment → materialization → subscription。
- Host shutdown/revoke 强制关闭读取能力；可写 session 必须保留可恢复工作副本策略，F0 不实现写入。

## 8. 可观测与技术指标

| 指标 | F0 Gate |
|---|---|
| `presentation_terminal_receipts_total / requests_total` | 100% 最终一致；每 request 恰好 1 |
| duplicate open side effects | 0 |
| cancel 后新 Surface | 0 |
| fault test 后 registry/session/materialization/lease | 全部 0 |
| Host Bridge 大 payload | 0；所有 envelope 低于现有限制 |
| schema digest TS/Rust | 100% 相同 |
| invalid/unknown protocol acceptance | 0 |
| Ontology Runtime dependency | 0 |

运行时延迟目标在 F0.4 fake 链测量：warm fake open p95 ≤ 300 ms、cancel terminal p95 ≤ 200 ms、close/dispose p95 ≤ 500 ms。真实引擎启动指标在 F1/F2/F3 单独制定。

## 9. 回滚与兼容

新链由 `resourcePresentationV2=internal/shadow` 控制。旧 Sidebar open 保留为显式 fallback，不能由 Adapter 内部偷偷调用。任何 schema breaking change 增 major；只增加 optional 字段且两端容忍时增 minor。回滚只关闭新 flag，不改变 Resource 数据。

## 10. Ontology 预埋边界

允许：descriptor `extensions`、event `details` 中的 `ontologyProjectionEligible: boolean`，以及 resource/session/change 事实事件。禁止：Ontology 包依赖、业务类型枚举、对象持久化、关系计算、Action 执行、影响传播。未来 Runtime 只能作为事实流的可选 Consumer，不能改变 Adapter API。
