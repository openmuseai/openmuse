# F0.2 Host Resource 具体方案设计

## 1. 阶段目标

把 F0.1 的 `muse.resource/v1` 合同变成 Host 可执行的 authority 内核：资源只有一个稳定 `resourceRef`，真实 locator 和内容由注册 Provider 持有；Adapter 只能通过绑定 actor/workspace/session/consumer/revision/mode 的短期 handle 接触数据面；所有提交都带 expected revision 和 idempotency key。

阶段 Gate：

1. fake resource 可 describe、materialize、Host-local read/write、commit 并产生唯一 change event；
2. ACL、TTL、revoke、consumer/authority/revision/mode mismatch 均 fail closed；
3. duplicate commit 不重复写入或发事件，并发旧 revision 只能一个成功；
4. 创建失败、shutdown race、disposer fault 后所有可管理计数归零；
5. 公共返回值不含 bytes/path/URL；无 Ontology Runtime 也能完成全部测试。

## 2. 用户与系统场景

### HR-S01：任意文件成为资源

Host 将来自 workspace、DSH 交付物或未来远端对象的 locator 绑定到 `resourceRef`。调用方凭 authority 查询 descriptor，无法从 ref 推导路径。DOCX、源码和 PDF 在这里没有特殊分支。

### HR-S02：Adapter 获得短期数据面

Orchestrator 请求 `resource@revision` 的 materialization。Host 校验 read/write authority 后让 Provider 生成 bytes/file/working-copy/URL/stream，并只返回 opaque handle。Adapter 在 Host-local resolve 端再次提交 actor、workspace、session、adapter、resource、revision 和 mode，任一不符都拒绝。

### HR-S03：安全保存

可写 Adapter 在自己的数据面产生新内容，提交 `expectedRevision + content handle + digest + idempotencyKey`。Host 校验 handle 和 digest，Provider 原子比较 revision。成功产生 receipt 和一个 `resource.changed`；冲突保留 materialization，不覆盖真源。

### HR-S04：撤权、超时和关闭

用户关闭 Surface、policy revoke、TTL 到期或 Host shutdown 时，materialization dispose 可重复调用。多个 disposer 中某一个失败不能阻止后续清理；错误汇总交给上层报警。

### HR-S05：Provider reload

同一 providerRef 活跃时不能重复注册；卸载后只有更高 generation 可以重新注册。卸载同时删除其 resource bindings，旧对象不能被新 generation 解析。

## 3. 组件设计

```text
ResourceHost
 ├─ ResourceProviderRegistry
 │    ├─ providerRef + generation
 │    └─ resourceRef → providerRef locator binding
 ├─ MaterializationManager
 │    ├─ public opaque lease
 │    └─ private Host-local MaterializedData + disposer
 ├─ ResourceCommitCoordinator
 │    ├─ idempotency ledger
 │    └─ digest/revision/receipt
 └─ ResourceEventLog
      └─ cursor + bounded retention
```

### 3.1 Provider Port

`ResourceProvider` 必须实现 `describe()` 和 `materialize()`；可写 Provider 额外实现 `commit()`；`dispose()` 可选。Provider 负责真实 locator、格式读取和原子存储，不负责 UI 路由、Agent policy 或 Ontology。

Provider 产生的 `ProviderMaterialization` 包含私有 `MaterializedData` 和 disposer。Host 对 kind、schema 和句柄唯一性完成验证后才取得所有权；取得所有权前任何失败都立即调用 Provider disposer。

### 3.2 Registry

- resource binding 必须显式注册，不按 resourceRef 字符串前缀猜 Provider。
- providerRef 活跃重复注册返回 `PROVIDER_ALREADY_REGISTERED`。
- generation 必须为正整数且严格递增，防止 reload 后旧实例复活。
- provider disposer 按注册逆序运行；失败聚合但不中断后续 disposer。

### 3.3 Materialization

公开记录字段来自 Resource v1 schema；私有记录额外保存：

- actorRef、workspaceRef、sessionRef；
- ownerRef、consumerAdapterRef；
- resourceRef、revision、kind、accessMode、expiresAt；
- Host-local data 和 provider disposer；
- 创建 sequence，用于逆序清理。

`bytes-handle` 和 `working-copy` 可以 read-write；其他 kind 当前只能 read。Host-local `writeBytes()` 是 Adapter 数据面端口，不可映射为通用 DSH/Agent operation。

### 3.4 Commit

处理顺序固定：schema → authority/session → idempotency digest → handle binding/TTL/mode → content kind/digest → Provider compare-and-store → closed receipt → changed event。

Idempotency digest 包含 request、actor、workspace、consumerAdapter。相同 key + 相同输入返回原 receipt；相同 key + 不同输入返回 `IDEMPOTENCY_CONFLICT`。

### 3.5 Event

事件 cursor 为 opaque `resource-cursor.N` 实现细节。无 cursor 的订阅返回当前 retention window；携带过旧 cursor 返回 `CURSOR_EXPIRED + retentionFromCursor`，要求 Consumer 重新 snapshot。只有 `committed` 产生 `changed`；`no-change/conflict/failed` 不产生假变化。

## 4. 状态与并发

Materialization 内部状态只有 `active → disposing → removed`。从 Map 删除发生在调用外部 disposer 前，保证重复 revoke/close 不重复执行。创建与 shutdown race 中，若 Provider 在 Host disposed 后才返回，Host 不登记 handle，直接回收 Provider 输出。

Commit 并发最终由 Provider 的 expected revision 原子比较裁决。Host 不使用进程内全局锁伪造跨 Provider 原子性；fake Provider 的并发测试验证同一 base revision 只会得到一个 `committed`。

## 5. Authority 与安全边界

| 边界 | 校验 |
|---|---|
| describe | canRead、非空 actor/workspace/session/resource、显式 binding |
| materialize | describe capability、当前 revision、canRead，写模式再检查 canWrite |
| resolve | TTL + actor/workspace/session + adapter + resource + revision + accessMode |
| data-plane write | resolve 的全部校验 + bytes handle + read-write |
| commit | schema + canRead/canWrite + session/resource + handle + kind + digest |

公共 Host Bridge 日志只能观察 handle 元数据，不能记录 `MaterializedData`。`ResourceHostSnapshot` 仅给测试/诊断计数，不包含 locator 或内容。

`ResourceAuthority` 是 Host 内部已判定上下文，不是 wire DTO。F0.3 Provider 必须从 authenticated binding、workspace/session scope 和 policy decision 构造它；任何 Consumer 自报的 `canRead/canWrite` 字段都不得传入本 API。

## 6. 不同端的落地约束

| 端 | 可用 Provider/materialization | 本阶段产物 | 后续绑定 |
|---|---|---|---|
| Desktop | bytes、read-file、working-copy、loopback、stream | 完整 port 与安全校验 | F0.4 Flutter/native broker |
| Web | remote-url、stream | 相同 authority/lease 模型 | F0.5 HTTPS/range/origin broker |
| Mobile | remote-url、stream/read-only | 相同 TTL/revoke 模型 | F0.5 分页/低内存 broker |
| DSH local/remote | 只消费 describe/materialize/commit 控制面 | `ResourceHostApi` | F0.3 Host Bridge Provider |

当前 fake Provider 只实现 bytes，避免用虚假 path/URL 伪装生产 broker。

## 7. 错误与重试

- 配置/安全错误不可自动重试：`RESOURCE_DENIED`、consumer/authority/resource/revision/access mismatch、idempotency conflict。
- 状态刷新后可重试：`REVISION_CONFLICT`、`MATERIALIZATION_EXPIRED`、`CURSOR_EXPIRED`。
- Provider contract violation 会隔离本次产物并清理；未来 runtime 对 provider/version 熔断。
- 原始 Provider fault 向上保留给 adapter/telemetry 映射，本阶段不吞异常。
- disposer fault 使用 `AggregateError`；计数先移除，外部资源失败必须报警并进入 F0.6 orphan reconciliation。

## 8. 技术指标

| 指标 | F0.2 要求 | 当前证据 |
|---|---|---|
| authority/binding 负例 | 100% fail closed | 自动测试 Pass |
| duplicate commit side effects | 0 | 自动测试 Pass |
| concurrent stale commit overwrite | 0 | 自动测试 Pass |
| failure/shutdown 后 managed handle | 0 | 自动测试 Pass |
| public materialization raw data fields | 0 | 自动测试 Pass |
| event false positive | 0 | 自动测试 Pass |
| unit tests | 100% pass | 25/25 Pass |
| registry resolve p95 | ≤ 1 ms / 10k bindings | F0.6 benchmark |
| Host bookkeeping p95 | ≤ 5 ms，不含 Provider I/O | F0.6 benchmark |
| dispose p95 | ≤ 500 ms | F0.6 fault/perf |

## 9. 明确不做

- 不实现真实 AppFlowy/local/cloud Provider 和原生 locator。
- 不实现 Host Bridge capability descriptor；属于 F0.3 seam。
- 不实现 Surface route、Adapter probe 或引擎进程。
- 不实现 ioffice/Helix 保存能力；fake bytes commit 只验证平台语义。
- 不持久化 transient materialization/commit ledger；Host restart recovery 在生产 Provider 设计中补充。
- 不实现 Ontology Runtime、知识对象或影响传播。
