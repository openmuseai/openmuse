# OpenMuse 跨域合同基线 v1

- 状态：Accepted（C0）
- Owner：OpenMuse Platform Architecture
- Schema：`schemas/openmuse.contract.v1.schema.json`
- Wire fixtures：`schemas/fixtures/contract/v1/`

## 1. 决策

OpenMuse 使用 `openmuse.contract` v1 作为 Host、Plugin、Workspace、Storage、Sandbox、Mobile 和 DSH 边界共同依赖的控制面词汇。它只规定跨域语义，不承载任何领域的完整业务 DTO：`payload`、`value` 和 Descriptor 的 `value` 仍由各领域自己的协议定义。

Rust、Dart 和 TypeScript 分别由以下独立包投影同一份 wire contract：

- Rust：`crates/openmuse-contract`；
- Dart：`packages/openmuse_contract`；
- TypeScript/DSH：`third_party/dsh/plugins/openmuse-contract`。

现有 `openmuse-plugin-protocol`、`muse_resource_contract` 和 `openmuse-dsh-bridge` 不在 C0 中被隐式迁移。后续 C1/C2/C3 必须显式依赖或适配本基线，避免公共合同反向依赖 Plugin、Resource 或 DSH 实现。

## 2. Envelope 语义

每个 request/response envelope 必须包含：

| 字段 | 语义 |
|---|---|
| `protocol` | `{name, major, minor}`；name 固定为 `openmuse.contract` |
| `requestId` | 跨语言安全的 opaque string；用于幂等、关联和审计，不使用 JS 不安全的 64-bit number |
| `actor` | 发起用户/Agent 的主体身份 |
| `caller` | 当前调用边界的直接调用者；不能由 actor 推导 |
| `scope` | authority 必填，workspace/resource 可选；只允许稳定 Ref，不允许 path |
| `generation` | authority/capability epoch，从 1 开始；消费者必须与期望值精确相等 |
| `deadlineAtMs` | UTC Unix epoch milliseconds；`now >= deadline` 即过期，缺失不代表无限期 |
| `cancellationRef` | 可公开记录的取消协调标识，不是 bearer token 或 Secret |

整数必须位于 JavaScript safe-integer 范围内。所有 `*Ref` 都是非空、无空白的 opaque string；不得从其格式推断厂商、文件系统位置或授权能力。

Request 增加 `operation` 与领域 `payload`。Response 必须携带 `outcome`，且无论成功或失败都必须产生 Receipt。成功 Receipt 必须为 `committed`；错误 Receipt 不能为 `committed`；Receipt 的 request id 与 generation 必须和 envelope 一致。

解析与授权是两步：解析器验证结构、版本和 generation；Policy/Broker 仍负责判断 actor、caller、scope 和 operation 是否被授权。解析成功从不表示授权成功。

## 3. 版本与 fail-closed

本地实现只接受同一 major 且 `peer.minor <= local.minor`。未知 name、未知 major、未来 minor、未知字段、未知枚举值和 Receipt 不一致全部拒绝。

- 破坏既有字段语义、删除/重命名字段、改变安全默认值：提升 major；
- 仅向新实现增加可协商能力：提升 minor；旧实现会在版本检查阶段拒绝未来 minor；
- 文案、注释和不改变 wire 的实现修复：不改变版本。

这套规则有意牺牲“随意透传未知字段”，换取安全边界的确定性。领域层如需 extensibility，必须在自身 `payload` schema 内版本化。

## 4. Generation、deadline 与 cancellation

- generation 代表授权/租约/连接的当前 epoch，不是资源内容 revision；收到非当前 generation 时返回 `STALE_GENERATION`，不得自动降级或重放到新 generation；
- deadline 是端到端预算，上游转发时只能保持或缩短，不能延长；已经过期的工作不得开始，晚到结果不得提交；
- cancellationRef 只负责关联取消意图。取消是 best-effort 的执行信号，但已取消 generation 的晚到结果仍必须被丢弃；
- Resource `revision` 处理内容并发，generation 处理 authority freshness，两者不能互换。

## 5. 生命周期对象

### Descriptor

Descriptor 是不可变的描述快照，包含 `descriptorRef / generation / revision / issuedAtMs / value`。新事实产生新快照；不得原地改变已签发的快照。Descriptor 不授予访问权。

### Capability Handle

Handle 将 audience、scope、access、TTL 和 generation 绑定在一起。合法状态转换仅为：

```text
active -> revoked
active -> expired
```

`revoked` 与 `expired` 均为终态。Handle 是可撤销引用而非永久凭据，wire 中不得包含实际 Secret。

### Lease

Lease 表示对执行、materialization 或编辑时段的临时占用：

```text
active -> released
active -> revoked
active -> expired
```

三个目标状态均为终态。续租必须签发新的 generation 或新的 leaseRef，不能延长已终止 Lease。

### Receipt

Receipt 是请求结果的不可变证据，状态为 `committed / rejected / cancelled / expired / failed`。Receipt 只记录稳定 effects 名称，不记录 path、Secret、presigned URL 或厂商 SDK 对象。

## 6. 统一错误词汇

| Code | 含义 | 默认重试判断 |
|---|---|---|
| `DENIED` | Policy、grant、audience 或 scope 拒绝 | 不重试；需要新授权/新请求 |
| `NOT_FOUND` | authority 范围内对象不存在 | 不盲重试 |
| `CONFLICT` | revision、状态或幂等结果冲突 | 调用方重新读取后决定 |
| `EXPIRED` | deadline、Handle 或 Lease 已过期 | 原请求不重试；可签发新请求 |
| `STALE_GENERATION` | authority epoch 已更新 | 原 generation 永不重放 |
| `UNAVAILABLE` | Provider/运行面暂时不可用 | 可按 Policy 限制退避重试 |
| `TRANSIENT` | 短暂且未提交的失败 | 仅在幂等条件成立时重试 |
| `INTEGRITY_FAILED` | digest、签名、内容或 receipt 校验失败 | 禁止自动重试/降级 |

`retryable` 是产生错误一方在当前上下文中的提示，不能突破调用方幂等、deadline、重试预算或 Policy。错误 `details` 仅允许结构化诊断信息，不得携带 Secret。

## 7. Fixture 所有权与变更门禁

`schemas/fixtures/contract/v1/manifest.json` 是 fixture inventory，记录 Owner、schema digest、期望解析结果与 generation。任何 wire 变更必须同时：

1. 更新 schema 和 manifest digest；
2. 更新 canonical fixtures；
3. 让 Rust、Dart、TypeScript 三个投影对同一 fixture 通过 round-trip/fail-closed 测试；
4. 通过 fixture 安全扫描，确认没有 path、credential/Secret、S3 厂商字段或 SDK 类型；
5. 在 PR 中说明属于 patch、minor 或 major 变更。

统一验收入口为：

```bash
scripts/test_contract_baseline.sh
```

## 8. 非目标

C0 不定义 Plugin Manifest v2、Delegation Policy、Workspace Resource API、Storage Provider、Sandbox execution 或 Mobile session 的业务字段；这些分别属于 C1/C2/C3/ST/X/M 子需求。C0 也不让 Client 直接持有 S3 credential，不将本机绝对路径提升为跨域身份，不指定某个 transport（HTTP、IPC、WebSocket、DSH remote 均可承载同一语义）。
