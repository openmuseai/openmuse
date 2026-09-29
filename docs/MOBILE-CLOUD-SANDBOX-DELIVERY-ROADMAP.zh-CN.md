# OpenMuse Mobile、Cloud Workspace 与 Workspace Sandbox 需求序列及交付计划

> 状态：Program Plan / 可拆分实施基线
>
> 日期：2026-09-29
>
> 输入：[Mobile 产品设计](MOBILE-PRODUCT-PRD.zh-CN.md)、[Mobile 架构与实现计划](MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md)、[S3 Storage ABI 与 MinIO/RustFS 选型](S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md)、[Workspace Sandbox 与 DSH Execution Plane](DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY.zh-CN.md)

## 1. 目的与总决策

本计划把两组需求合并为一个交付 Program，但不把它们实现成一个巨型模块：

1. **Mobile / Cloud Workspace / Storage**：Mobile 是 Agent-first 随身工作台，连接 Cloud Workspace 或已配对 Desktop；S3 是 Cloud 数据面的可替换 ABI。
2. **Workspace Sandbox / DSH / Plugin CLI**：Local、Cloud 和未来 Workspace 共享受控 Bash/CLI 执行语义；Office/领域插件以 headless worker 扩展 Agent 能力。

二者共享 Host/Plugin、权限、ResourceRef、Revision、Materialization 和 Receipt 合同；但 Mobile UI、S3 Provider、Workspace Sandbox、DSH 和 Office Engine 保持独立限界上下文。

推荐交付策略：

- 先冻结公共合同和测试夹具，再允许各工作流用 fake 独立推进；
- 先做 Cloud-only Mobile 内测，不等待 Paired Desktop、BYOS、RustFS qualification 或完整 Office 编辑；
- 先做 Local Sandbox PoC，不等待 S3 和 Mobile；
- Cloud Sandbox 可以先使用 fake/local blob adapter，生产接入才依赖 Cloud Resource Authority；
- RustFS 90 天 soak 是长期并行门禁，不进入客户端功能关键路径；
- Office Word/Sheet/Slides/PDF 分别立项、分别验收，不设置“全部完成才发布”的大门；
- 每个子需求必须拥有独立 Feature Flag、TCK/验收夹具和回滚边界。

## 2. 领域边界

| 领域 | 权威数据/职责 | 对外合同 | 明确不拥有 |
| --- | --- | --- | --- |
| Host/Plugin Platform | Manifest、Plugin 生命周期、Contribution、grants、distribution lock | Plugin Protocol、Broker | Workspace 内容、S3、DSH 推理 |
| Identity/Policy | actor、delegation chain、device、grant、decision | Capability/Policy receipt | 文件 bytes、Plugin 实现 |
| Workspace/Resource | tree、ResourceRef、revision、ACL、materialization、commit | Workspace/Resource Authority | 对象存储方言、进程执行 |
| Object Storage | immutable blob、range、multipart、storage receipt | BlobStorePort、S3 Profile | Workspace tree、当前 revision |
| Workspace Sandbox | Lease、执行世界、mount、进程、quota、draft、quiescence | `workspace.sandbox@1` | Plugin 安装真相、Workspace revision 真相 |
| DSH Agent | Session、推理、Tool 调度、事件日志 | Connector、`ctx.fs/subprocess/sandbox` | S3 凭据、Host path、Plugin 授权 |
| Mobile Host | 1/2/3 Window 投影、Remote presentation、审批、原生 capability bridge | Mobile Host/Window contracts | Node、本地 Bash、Cloud 存储实现 |
| Pairing/Relay | device trust、presence、Workspace grant、E2E attachment | Paired provider/connector | 自动上传、Cloud Workspace 替代 |
| Office/Domain Plugin | 格式语义、view/edit/export、Agent worker | Engine Adapter、Agent CLI ABI | Workspace/S3/DSH 权威 |

跨域调用必须经过版本化 Service/handle/receipt。禁止用绝对路径、永久 Secret、内部进程对象或厂商 SDK 类型跨越边界。

## 3. 优先级定义

| Priority | 含义 |
| --- | --- |
| P0 | 公共阻塞项或高风险验证；未完成会导致多个工作流返工 |
| P1 | Cloud-only Mobile 与基本 Workspace Sandbox 的 MVP 关键路径 |
| P2 | 数据自主、配对 Desktop、Plugin CLI 和完整生产能力 |
| P3 | 平台扩展、长期 qualification 或可延后的格式能力 |

依赖分两种：

- **硬依赖**：上游合同或能力未通过 Gate，目标需求不能进入集成/发布；
- **开发依赖**：可以使用 fake/fixture 开发，但合并到目标 Release 前必须替换为真实实现。

## 4. 总依赖图

```text
                              C0 Contract Baseline
                                      │
            ┌───────────────┬─────────┼───────────┬────────────────┐
            ▼               ▼         ▼           ▼                ▼
      C1 Manifest v2   C2 Delegation  C3 Resource C4 Distribution  X0 DSH Spike
            │               │         Authority       Lock              │
            │               │            │             │                │
            │        ┌──────┴─────┐      │             │                │
            │        ▼            ▼      ▼             ▼                ▼
            │      ST1          X1 Sandbox Service   M0 App Root     X2 Cloud Runtime
            │        ▲            │                    │                │
            ▼        │            ├──────────┐         │                │
      X4 CLI Registry│            ▼          ▼         ▼                ▼
            │      ST0 S3 TCK   X3 Draft    X5 Dispatcher       M1 Adaptive + M2/M3 Clients
            │        │            │          │                  │
            │        ▼            └────┬─────┘                  │
            │      ST2 Cloud Authority │                        │
            │        │                 ▼                        ▼
            └────────┼────────────── X6 Security ─────────── M4 Cloud Mobile Slice
                     │                                          │
              ┌──────┴────────┐                     ┌───────────┴──────────┐
              ▼               ▼                     ▼                      ▼
          ST3 Sync        ST4 BYOS/Migrate       M5 Paired Desktop      M7/M8 Releases
              │
              ▼
          ST5 RustFS qualification（90 天并行，不阻塞前期）
```

图中省略了部分交叉边，仅表达主关键路径；下表和各子需求定义为准。

## 5. 需求序列表

| ID | 子需求 | P | 硬依赖 | 可独立/并行 | 估算 |
| --- | --- | --- | --- | --- | ---: |
| C0 | 合同基线、错误词汇与跨语言 fixtures | P0 | 无 | 所有工作流的首个短任务 | 1～2 周 |
| C1 | Manifest v2、Artifact、Target、Agent CLI Schema | P0 | C0 | 与 C2/C3/ST0/X0 并行 | 3～5 周 |
| C2 | Broker Delegation、Policy Decision 与 Audit Context | P0 | C0 | 与 C1/C3 并行 | 3～5 周 |
| C3 | Workspace/Resource Authority v1 | P0 | C0 | 与 C1/C2 并行 | 4～6 周 |
| C4 | Distribution Lock、SBOM 与闭包门禁 | P0 | C1 | 可与 App Root/后端并行 | 2～3 周 |
| ST0 | BlobStorePort、S3 Profile 与 Provider TCK | P0 | C0 | 完全独立于 Mobile UI/DSH | 3～4 周 |
| ST1 | S3 Adapter、Credential/Vault 与 Endpoint 安全 | P1 | ST0；生产授权集成依赖 C2 | Adapter 可先用测试 Vault | 4～6 周 |
| ST2 | Cloud Workspace Metadata/CAS/Outbox | P1 | C3、ST0 | 先用 fake BlobStore | 4～6 周 |
| ST3 | Workspace Sync Plugin | P2 | C3、ST1、ST2 | 独立于 Mobile Window/Sandbox | 4～6 周 |
| ST4 | BYOS、Provider 迁移与数据可移植 | P2 | ST1、ST2；迁移 UI 依赖 ST3 | 与 Pairing/Office 并行 | 5～8 周 |
| ST5 | RustFS Qualification 与 Managed Rollout | P3 | ST0、ST1 | 长期独立泳道 | 3～5 周工程 + ≥90 天 soak |
| X0 | DSH `0.1.7-rc.1` Contract Spike 与 Provider TCK | P0 | C0 | 独立于 Storage/Mobile UI | 2～3 周 |
| X1 | Host Sandbox Service、Lease 与 Local Runtime | P1 | C2、C3、X0 | 可用本地 Workspace，不等 S3 | 5～8 周 |
| X2 | Cloud Execution Runtime 与 DSH Provider 组 | P1 | X1；生产集成依赖 ST2 | 可先接 fake Workspace | 6～10 周 |
| X3 | Draft、Checkpoint、Quiescence 与恢复 | P1 | C3、X1；Cloud 依赖 X2/ST2 | Local 先行 | 4～6 周 |
| X4 | Plugin CLI Registry 与 Artifact Resolver | P1/P2 | C1、C2 | 与 X2、Mobile 并行 | 3～5 周 |
| X5 | `office/openmuse` Dispatcher 与首个 Worker | P2 | X1、X4、C3 | Local 先行，不等 Cloud | 5～8 周 |
| X6 | Sandbox 生产安全与多租户 Gate | P1 | X2、X3、C2；CLI Gate 依赖 X5 | 安全测试可逐步前移 | 5～8 周 |
| M0 | Mobile App Root 与共享 Host Shell | P1 | C1；发行验收依赖 C4 | 与 M1/后端并行 | 3～4 周 |
| M1 | Adaptive Windows 与 Surface 生命周期 | P1 | C0 | 纯 Dart，可独立开展 | 3～4 周 |
| M2 | DSH Core/Connector 拆分与 Remote Presentation | P1 | C0、C2、X0 | 用 fake `session/open` 独立开发 | 4～6 周 |
| M3 | Mobile Resource Client、Viewer 与 Capability Bridge | P1 | C3 | 用 Resource fixtures 独立开发 | 3～5 周 |
| M4 | Cloud Workspace + Mobile + Remote DSH 纵切 | P1 | M0、M1、M2、M3、ST2、C2 | 集成 Gate | 4～6 周 |
| M5 | Paired Desktop、Relay 与 Desktop Workspace Provider | P2 | C2、C3、M2、M3 | 不依赖 S3/BYOS | 6～10 周 |
| M6 | Mobile Office Format Plugin（逐格式） | P2/P3 | C1、C3、M0 | 各格式相互独立 | 3～7+ 周/格式 |
| M7 | Android Alpha | P1/P2 | C4、M0、M1、M4 | iOS 可并行后半段 | 2～3 周 |
| M8 | iOS Beta | P2 | C4、M0、M1、M4 | Android 可并行后半段 | 3～4 周 |

这些估算包含设计、实现和本需求自己的验收，不包含底层 Office Engine 新增格式能力，也不应机械求和。

## 6. 公共基础子需求

### C0：合同基线、错误词汇与跨语言 Fixtures

**实施状态：已完成**（`feature/c0-contract-baseline`）

实现合同、canonical fixtures 与统一验收入口见 [`CONTRACT-BASELINE-V1.zh-CN.md`](CONTRACT-BASELINE-V1.zh-CN.md) 和 `scripts/test_contract_baseline.sh`。

**目标**

在任何大型实现前冻结跨域 identity、generation、deadline、cancellation、receipt 和错误分类，避免 Dart/Rust/TypeScript/Server 各自创造相似但不兼容的结构。

**方案**

- 建立 `schemas/fixtures/` 作为 wire fixture 真源；
- 每个 envelope 必须含 protocol version、request id、actor、scope、generation、deadline；
- 统一 `DENIED / NOT_FOUND / CONFLICT / EXPIRED / STALE_GENERATION / UNAVAILABLE / TRANSIENT / INTEGRITY_FAILED`；
- 明确 Descriptor、Handle、Lease、Receipt 四类对象的生命周期和序列化规则；
- 只冻结语义，不在 C0 设计所有业务字段。

**开发计划**

1. 盘点现有 Plugin、Resource、DSH bridge、Mobile 和 Sandbox 草案字段；
2. 输出 vocabulary ADR 与 JSON fixtures；
3. 为 Dart、Rust、TypeScript 添加 fixture parse/round-trip tests；
4. 建立 breaking-change 检查和 fixture ownership。

**验收**

- 三种语言解析同一成功/拒绝/过期/冲突 fixture；
- late generation 和未知 major version fail closed；
- fixture 中无 path、永久 Secret 或厂商 SDK 类型。

### C1：Manifest v2、Artifact、Target 与 Agent CLI Schema

**实施状态：已完成**（`feature/c1-manifest-v2`）

实现与验收入口见 [`PLUGIN-MANIFEST-V2.zh-CN.md`](PLUGIN-MANIFEST-V2.zh-CN.md) 和 `scripts/test_plugin_manifest_v2.sh`。

**目标**

一次完成 Mobile target 裁剪和 Sandbox Agent CLI 扩展，避免先做 Mobile manifest v2、随后再做不兼容的 CLI v3。

**方案**

- Manifest 分离 `ui_runtime`、execution connector、target compatibility、artifacts、presentation 和 contributions；
- artifact 包含 kind、target OS/arch/libc、digest、签名、license、ABI；
- `agent_cli` 包含 group/namespace/command/schema/required permissions/effects；
- v1 只通过纯迁移器读取；未知字段/target 默认拒绝；
- Manifest 声明请求能力，实际 grants 由 C2 决定。

**代码落点**

- `schemas/openmuse.plugin.v2.schema.json`；
- `crates/openmuse-plugin-protocol/`；
- `packages/openmuse_plugin_sdk/`；
- manifest migration/TCK。

**验收**

- 现有四个插件都有 v2 fixture 和明确 platform 决策；
- Android/iOS 不兼容 artifact 无法进入 resolution result；
- 未签名或 digest 不符的 sandbox worker 不注册；
- v1→v2 迁移确定性，原 v1 parser 回归测试不破坏。

### C2：Broker Delegation、Policy Decision 与 Audit Context

**实施状态：已完成**（`feature/c2-broker-delegation`）

安全模型、实现与验收入口见 [`BROKER-DELEGATION-POLICY-AUDIT.zh-CN.md`](BROKER-DELEGATION-POLICY-AUDIT.zh-CN.md) 和 `scripts/test_broker_delegation.sh`。

**目标**

支持“用户授权 DSH，DSH 代表用户调用 Sandbox，Sandbox 再启动目标 Plugin worker”的多主体调用，而不把某个 Plugin 的 ambient authority 借给另一个主体。

**方案**

```text
effective grants
 = tenant/host ceiling
 ∩ user/session grants
 ∩ caller plugin grants
 ∩ target plugin grants
 ∩ command required permissions
 ∩ workspace classification
```

- 每次决策记录 actor、caller plugin、target provider、workspace/revision、decision id；
- Capability handle 绑定 audience、scope、TTL、generation 和 revocation；
- Secret 只允许 brokered use，不导出明文；
- DSH 一次性 escalation 不能突破 Host ceiling；
- Mobile approval 只批准当前 action，不扩大 Desktop/Workspace grant。

**开发计划**

1. 扩展 `openmuse-plugin-protocol` request context；
2. 扩展 `openmuse-platform-runtime` policy hook 和 delegation validation；
3. 增加 handle registry/revocation port；
4. 添加审计 receipt 与 redaction；
5. 先用内存 policy provider，通过后再接账号/组织服务。

**验收**

- confused-deputy、过期 handle、错误 audience、跨 Workspace 重放全部失败；
- 审计能关联用户→DSH→Sandbox→Office worker；
- 日志不包含 token、presigned query 或 S3 key。

### C3：Workspace/Resource Authority v1

**实施状态：已完成**（`feature/c3-workspace-resource-authority`）

Provider port、事务边界与验收入口见 [`WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md`](WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md) 和 `scripts/test_workspace_resource_authority.sh`。

**目标**

建立 Mobile、Storage、Sync、Sandbox 和 Office 共用的资源真源，彻底把绝对 path 限制在 Local Provider 内。

**方案**

- `WorkspaceRef / MountRef / ResourceRef / Revision` 为稳定身份；
- `describe/list/materialize/commit/subscribe` 为 Provider Port；
- materialization 使用 TTL handle、audience、access mode、generation；
- commit 使用 `expectedRevision`、idempotency key、receipt；
- Working Copy/Draft 与 committed Revision 区分；
- Local、Cloud、Paired 三种 Provider 运行同一 TCK。

**代码落点**

- `packages/muse_resource_contract/`；
- `packages/muse_resource_bridge/`；
- 新增 Workspace Authority port/provider TCK；
- 当前 `workspace_controller` 只作为 Local adapter 输入，不继续扩展为通用真源。

**验收**

- Local 与 fake Cloud Provider 通过相同 list/materialize/commit/conflict 测试；
- Consumer 不能用 path 打开远端 Resource；
- stale revision、handle expiry、audience mismatch 有稳定错误；
- Blob 成功而 metadata 失败时不返回 commit success。

### C4：Distribution Lock、SBOM 与闭包门禁

**实施状态：已完成**（`feature/c4-distribution-lock`）

实现、发行流水线合同与统一验收入口见 [`DISTRIBUTION-LOCK-SBOM-CLOSURE-GATE.zh-CN.md`](DISTRIBUTION-LOCK-SBOM-CLOSURE-GATE.zh-CN.md) 和 `scripts/test_distribution_lock.sh`。

**目标**

让“插件是否打包”成为构建期事实，而不是运行时隐藏；同时支撑 Mobile 与 Sandbox worker artifact 的目标平台选择。

**方案**

- resolver 根据 Manifest v2 + Host capability snapshot 生成 `distribution-lock.json`；
- lock 固定 plugin/version/artifact/digest/license/target；
- APK/IPA 禁带 Node、Helix、Desktop dylib；Sandbox image 禁带未批准 worker；
- SBOM/notices 与最终 closure 一致。

**验收**

- 使用测试 Mobile app root 的制品扫描无 Desktop runtime；
- 错误 target/digest 使构建 fail closed；
- 同一 lock 可复现相同 artifact closure；
- Desktop 现有发行闭包无功能回归。

## 7. Storage 与 Cloud Workspace 子需求

### ST0：BlobStorePort、S3 Profile 与 Provider TCK

**实施状态：已完成（合同/TCK）**（`feature/st0-storage-contract-tck`）

ABI、reference fake、黑盒场景和统一验收入口见 [`STORAGE-CONTRACT-S3-PROFILE-TCK.zh-CN.md`](STORAGE-CONTRACT-S3-PROFILE-TCK.zh-CN.md) 和 `scripts/test_storage_contract.sh`。真实 AWS S3/MinIO/RustFS snapshot 必须由 ST1 adapter 针对具体环境运行同一 TCK；ST0 不在没有 adapter/凭据时伪造认证。

**目标**

冻结 OpenMuse 实际依赖的 S3 子集，并用黑盒测试替代厂商 feature table。

**方案与开发**

- 新增 provider-neutral `openmuse-storage-contract`；
- 定义 bounded stream、BlobRef、digest、multipart、receipt、错误归一化；
- 编写 fake provider 和 S3 TCK；
- 对 AWS S3、MinIO、RustFS 跑 put/head/get/range/delete/multipart/checksum/TLS/path-style；
- ListObjects 仅用于 GC/修复，不服务 Workspace tree。

**验收**

- reference fake 的完整基础矩阵可重复运行并生成 capability snapshot；
- 同一 TCK 可直接用于 AWS S3、MinIO、RustFS；三者的真实 snapshot 是 ST1 adapter 集成 Gate；
- trait 不泄漏 `aws_sdk_s3::ByteStream` 或厂商错误；
- ETag 不被当作内容 hash；
- 0B、5MiB 边界、100MiB+、中断/重试都有覆盖。

### ST1：S3 Adapter、Credential/Vault 与 Endpoint 安全

**实施状态：Accepted**（`feature/st1-s3-adapter-security`）

实现边界、风险和验收入口见 [`S3-ADAPTER-CREDENTIAL-ENDPOINT-SECURITY.zh-CN.md`](S3-ADAPTER-CREDENTIAL-ENDPOINT-SECURITY.zh-CN.md) 与 `scripts/test_storage_s3.sh`。MinIO/RustFS 已用 HTTPS、CA bundle 和固定 DNS connector 通过完整 TCK并固化报告；产品负责人于 2026-09-29 明确豁免本阶段真实 AWS S3 验证，详见 [`qualification/storage/ST1-ACCEPTANCE.zh-CN.md`](qualification/storage/ST1-ACCEPTANCE.zh-CN.md)。

**目标**

实现可生产化的 `aws-sdk-s3` adapter，同时确保 Mobile、DSH、Office 和普通 Sandbox 子进程永远不接收永久 S3 Secret。

**方案与开发**

- 实现 `openmuse-storage-s3`，封装 endpoint/path-style/checksum/presign quirks；
- 实现 Credential Store/Vault/STS port、rotation 和 revoke；
- endpoint 做 DNS rebinding/SSRF/private network policy；
- 连接测试使用随机 prefix 完成完整读写流程；
- URL/错误/日志统一脱敏；
- 建立 orphan multipart 和 unreferenced blob GC worker。

**验收**

- 凭据轮换期间已授权请求可控完成，新请求使用新 generation；
- metadata address、意外 loopback/private endpoint 被拒绝；
- presigned URL 不进入日志；
- Provider 不可用时返回归一化错误并保留上层 Draft。

### ST2：Cloud Workspace Metadata、CAS 与 Outbox

**实施状态：Accepted**（`feature/st2-cloud-resource-authority`）

领域边界、事务语义与统一验收入口见 [`CLOUD-WORKSPACE-METADATA-CAS-OUTBOX.zh-CN.md`](CLOUD-WORKSPACE-METADATA-CAS-OUTBOX.zh-CN.md) 和 `scripts/test_cloud_resource_authority.sh`。

**目标**

提供 Cloud Workspace 的 Resource Authority，而不是把 S3 bucket 直接当作文件树。

**方案与开发**

- Metadata Store 保存 Workspace tree、ACL、Resource Revision、BlobRef；
- immutable blob + metadata CAS 两阶段提交；
- outbox 发布 `resource.changed`，失败可重放；
- catalog/list/search 从 metadata 读取；
- stream/range handle 带 TTL/audience/generation；
- 初期可用 fake BlobStore，后接 ST1。

**验收**

- 并发 writer 必有一个 conflict，不发生 last-write-wins；
- blob 成功、metadata 失败产生可回收孤儿，不显示保存成功；
- metadata 成功、事件失败可由 outbox 恢复；
- 10k resources 不调用 ListObjects 构造 UI tree。

### ST3：Workspace Sync Plugin

**实施状态：Accepted**（`feature/st3-workspace-sync`）

策略状态机、Provider-neutral 端口和统一验收入口见 [`WORKSPACE-SYNC-PLUGIN.zh-CN.md`](WORKSPACE-SYNC-PLUGIN.zh-CN.md) 和 `scripts/test_workspace_sync.sh`。

**目标**

独立实现 local-only/snapshot/mirror/migrate，不让 Local Provider、Cloud Provider 或 S3 Provider互相拥有同步逻辑。

**方案与开发**

- 消费 Local change feed 与 Resource Service；
- durable plan/outbox/cursor/idempotency；
- digest 上传后执行 Cloud `expectedRevision` commit；
- 文本、Office、binary 冲突交给相应 Engine/merge provider；
- 未确认策略前零上传；
- UI 只显示 receipt/cursor/conflict，不推测同步成功。

**验收**

- 四种 policy 的上传方向与可写性符合 PRD；
- 断网/进程重启可续传；
- mirror split-brain 保留双方 Revision；
- snapshot 永远只读，migrate 完成前不切真源。

### ST4：BYOS、Provider 迁移与数据可移植

**实施状态：Accepted**（`feature/st4-byos-provider-portability`）

BYOS 执行放置、迁移状态机、可移植导出和统一验收入口见 [`BYOS-PROVIDER-MIGRATION-PORTABILITY.zh-CN.md`](BYOS-PROVIDER-MIGRATION-PORTABILITY.zh-CN.md) 和 `scripts/test_storage_portability.sh`。

**目标**

把“数据自主”做成可测试的配置、迁移、验证、撤销和删除能力。

**方案与开发**

- Server-mediated 与 Desktop-mediated BYOS 分开；
- capability snapshot 显示 Provider 兼容性；
- 按 metadata-owned BlobRefs 迁移，逐对象 digest；
- provider generation 原子切换，旧 generation 迟到写无效；
- observation window 后二次确认删源；
- 提供 manifest、revision、原件、digest 和验证工具导出。

**验收**

- 两种不同 S3 Provider 双向迁移一致；
- 可暂停/恢复/回滚；
- 凭据撤销后无后台继续访问；
- 删除产生 metadata 和 Provider 两侧 receipt。

### ST5：RustFS Qualification 与 Managed Rollout

**实施状态：Engineering Accepted；Managed 默认 Provider Gate 未通过（长期证据进行中）**（`feature/st5-rustfs-qualification`）

Fail-closed 判定器、证据账本和长期运行要求见 [`RUSTFS-QUALIFICATION-MANAGED-ROLLOUT.zh-CN.md`](RUSTFS-QUALIFICATION-MANAGED-ROLLOUT.zh-CN.md) 和 `scripts/test_rustfs_qualification.sh`。当前未伪造 ≥90 天 soak，RustFS 默认切换保持禁止；此长期 Gate 按计划不阻塞后续工作流。

**目标**

验证 RustFS 是否可成为 OpenMuse Managed Storage 的默认实现，不阻塞产品先使用已认证成熟 S3。

**方案与开发**

- 固定 stable 版本、SBOM、签名与升级策略；
- 4 节点/多盘故障、网络分区、磁盘满、恢复、滚动升级/回滚；
- 生产等价容量 ≥90 天 soak 和 digest scrub；
- shadow copy、双读验证、小租户 canary；
- MinIO/AWS S3 保持兼容与迁移回退。

**验收**

- 无未接受的高/严重安全 blocker；
- RPO/RTO 来自演练数据；
- canary 无 digest mismatch 且可回滚；
- 未过门禁时产品仍可在其他认证 Provider 上发布。

## 8. Workspace Sandbox 与 DSH 子需求

### X0：DSH Contract Spike 与 Provider TCK

**实施状态：Accepted**（`feature/x0-dsh-contract-tck`）

实际 `0.1.7-rc.1` 闭包合同、Provider seam 差异与统一验收入口见 [`DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md`](DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md) 和 `scripts/test_dsh_provider_contract.sh`。

**目标**

针对实际打包的 DSH `0.1.7-rc.1` 锁定 `ctx.fs/subprocess/shell/sandbox` 契约，消除 vendor source 与 npm closure 版本差异。

**方案与开发**

- 验证 Profile/Patch 替换 Base Bundle rows；
- 建立 FS↔Bash 双向可见、PTY、LSP、jobs、signal/cancel TCK；
- 验证 local sandbox 的 read-only/workspace-write/fail-closed；
- 明确远端 `fs-sandbox` 不可直接复用的接口差异；
- 形成 provider compatibility shim 和升级矩阵。

**验收**

- 不修改 Agent Loop 即可替换 Provider；
- 同一 Execution World TCK 全绿；
- DSH 升级时 contract diff 自动失败；
- 所有 PoC 代码不依赖未进入生产闭包的 E2B 包。

### X1：Host Sandbox Service、Lease 与 Local Runtime

**实施状态：Accepted**（`feature/x1-host-sandbox-local-runtime`）

实现与统一验收入口见 [`HOST-SANDBOX-SERVICE-LOCAL-RUNTIME.zh-CN.md`](HOST-SANDBOX-SERVICE-LOCAL-RUNTIME.zh-CN.md) 和 `scripts/test_workspace_sandbox.sh`。

**目标**

建立独立 `workspace.sandbox@1` Service Plugin，并先在 Desktop Local Workspace 跑通，不等待 Cloud/S3。

**方案与开发**

- `WorkspaceSandboxLease` 绑定 actor/workspace/baseRevision/policy ceiling/registry digest；
- Service 提供 create/attach/status/quiesce/prepareDraft/release；
- Local native 复用 DSH sandbox；Local isolated 预留 container/VM provider；
- DSH adapter 获取 opaque attachment，不把 Host path 写入通用协议；
- `/workspace /runtime /home/agent /tmp /run/openmuse` 使用固定挂载合同。

**验收**

- `rg/python/cargo` 在受控 `/workspace` 工作；
- 不能读取 Host home/SSH/其他 Workspace；
- Lease expiry/revoke 杀死完整 process range；
- Sandbox 插件停用不结束 Host，清理有 receipt。

### X2：Cloud Execution Runtime 与 DSH Provider 组

**实施状态：Accepted**（`feature/x2-cloud-execution-runtime`）

Runtime pool、同执行世界 Provider TCK 与验收入口见 [`CLOUD-EXECUTION-RUNTIME-DSH-PROVIDERS.zh-CN.md`](CLOUD-EXECUTION-RUNTIME-DSH-PROVIDERS.zh-CN.md) 和 `scripts/test_cloud_execution_runtime.sh`。

**目标**

把同一 Sandbox Lease 运行在 Cloud container/microVM，并让 DSH Bash、FS、PTY、LSP 位于同一执行世界。

**方案与开发**

- Runtime pool 分配 tenant/task 隔离实例；
- 实现 `@openmuse/dsh-workspace-runtime/fs/subprocess/sandbox`；
- outer ceiling 管 mount/network/device/quota；DSH policy 管每次文件效果；
- Provider attachment 走短期 audience-bound control channel；
- runtime image/version/SBOM 固定；
- 初期接 fake checkout，生产接 ST2 materialization。

**验收**

- FS 写入、Bash、PTY、LSP 双向一致；
- `danger-full-access` 不能逃出 outer Sandbox；
- 跨租户进程/卷/cache 不可见；
- 冷启动、取消、崩溃和 orphan cleanup 有指标与测试。

### X3：Draft、Checkpoint、Quiescence 与恢复

**实施状态：Accepted**（`feature/x3-draft-checkpoint-recovery`）

Draft Transaction、内容扫描、expected-base CAS 和崩溃恢复见 [`DRAFT-CHECKPOINT-QUIESCENCE-RECOVERY.zh-CN.md`](DRAFT-CHECKPOINT-QUIESCENCE-RECOVERY.zh-CN.md) 与 `scripts/test_draft_checkpoint.sh`。

**目标**

让 Bash 任意文件修改可以安全转成 Workspace Revision，而不是依赖不可靠 watcher 或每命令自动上传。

**方案与开发**

- checkout 是 Draft Transaction；
- Overlay/COW/journal + digest verification 跟踪变化；
- Provider 跟踪完整 process range；checkpoint 前 quiesce/freeze/flush；
- `prepareDraft` 产生 manifest digest，Resource Authority 做 expected-base CAS；
- 冲突保留 Draft，崩溃后恢复或可证明清理。

**验收**

- watcher 丢事件仍能发现变化；
- 活跃后台 writer 时 checkpoint 拒绝或等待；
- 并发 HEAD 变化不覆盖；
- crash/network failure 后不存在“显示成功但无 Revision”。

### X4：Plugin CLI Registry 与 Artifact Resolver

**实施状态：Accepted**（`feature/x4-plugin-cli-registry`）

冻结 registry、artifact admission 与 capability discovery 见 [`PLUGIN-CLI-REGISTRY-ARTIFACT-RESOLVER.zh-CN.md`](PLUGIN-CLI-REGISTRY-ARTIFACT-RESOLVER.zh-CN.md) 和 `scripts/test_cli_registry.sh`。

**目标**

把已安装 Plugin 的 Agent CLI contribution 解析成某次 Lease 的冻结能力快照。

**方案与开发**

- 消费 C1 Manifest/Artifact 和 C2 grants；
- 按 target/ABI/digest/signature/license/policy 选择 worker；
- namespace 冲突确定性拒绝或选择；
- Registry 带 digest/generation，Session 内不静默升级；
- Workspace 中的 `plugins.json` 只能是 requirement，不能是安装权威。

**验收**

- 无匹配 Linux artifact 的 macOS Plugin 在 Cloud 显示 unavailable；
- Plugin 更新不改变既有 Lease；
- 伪造 Workspace 清单不能注册命令；
- capability discovery 只返回当前可执行能力。

### X5：Dispatcher 与首个 Office Worker

**实施状态：Accepted**（`feature/x5-office-dispatcher-worker`）

稳定 CLI、Supervisor admission 与只读 DOCX worker 见 [`OFFICE-DISPATCHER-WORKER-SUPERVISOR.zh-CN.md`](OFFICE-DISPATCHER-WORKER-SUPERVISOR.zh-CN.md) 和 `scripts/test_worker_supervisor.sh`。

**目标**

只向 Agent 暴露稳定 `office/openmuse` CLI，通过 Supervisor 启动不可直接寻址的 headless worker。

**方案与开发**

- `/runtime/bin/office` 和 `openmuse` 通过 scoped socket 调 Supervisor；
- Supervisor 校验 registry、argv schema、cwd、deadline、permission intersection；
- worker 使用与 UI Plugin 共享的 Rust Core，但独立进程/身份/Secret；
- stdout JSON 版本化，日志 stderr，大结果写 Workspace/Blob；
- 首个 worker 建议选只读/可验证输出的 DOCX inspect/render，再扩展 replace-image。

**验收**

- Agent 不能直接执行/替换 worker bundle；
- argv 不进行第二层 shell 拼接；
- cancel/timeout 终止完整 worker process range；
- 同一输入在 Local/Cloud 产生一致业务输出；
- UI Plugin 权限不会泄漏给 worker。

### X6：Sandbox 生产安全与多租户 Gate

**目标**

把“任意程序执行”从功能 PoC 提升为可上线边界。

**方案与开发**

- rootless/microVM、read-only image、seccomp、cap drop、CPU/memory/PID/disk/time quota；
- egress default-deny + allowlist/proxy/audit；
- Secret broker、无 Secret child env、`/proc` 泄漏检查；
- artifact signing/SBOM/revocation；
- symlink/hardlink/path traversal、archive bomb、fork bomb、output injection fuzz；
- 渗透测试、故障注入、容量与成本监控。

**验收**

- 安全测试无未处置 P0/P1；
- 资源耗尽不会影响其他租户或 Host；
- outer ceiling 无法被 DSH escalation/Plugin worker 绕过；
- 所有执行都有 actor→caller→target→artifact→draft/revision 审计链。

## 9. Mobile 子需求

### M0：Mobile App Root 与共享 Host Shell

**目标**

建立不含 Desktop runtime 的 Mobile 壳，让 Desktop/Mobile composition 在源码依赖层真实分离。

**方案与开发**

- 抽取 `openmuse_host_shell`，创建 `app/openmuse_mobile`；
- Host Shell 只依赖 Broker、Window abstraction、theme 和 route contracts；
- Desktop/Mobile composition root 分别选择允许的 Plugin adapters；
- 接入 C4 distribution lock，但允许开发期使用 fixture lock；
- 登录/configuration/capability snapshot 通过注入 seam，不硬编码 Cloud SDK。

**验收**

- Android APK 与 iOS `--no-codesign` 空壳构建；
- 制品不含 Node、Helix、PTY 和 Desktop native artifact。
- Desktop app 对 Host Shell 抽取无功能回归；
- Mobile Host Shell 不 import 任一具体 Plugin 实现。

### M1：Adaptive Windows 与 Surface 生命周期

**目标**

独立实现 Compact/Medium/Expanded 的 1/2/3 Window 投影，不等待 Mobile App Root、Cloud API 或 DSH。

**方案与开发**

- `AdaptiveWindowProjector` 只投影 logical Window graph，不修改业务 session；
- 支持 hinge/display segments、safe area、IME、font scale；
- 定义 visible/warm/suspended 与稳定 `instanceRef`；
- Compact edge swipe + 显式切换，PlatformView 内容手势优先；
- Desktop layout store 与 Mobile projection/migration 分开；
- 用 fake Surface/PlatformView harness 做纯 Dart 测试。

**验收**

- 360/600/960dp 和折叠 segment golden 通过；
- 200 次旋转/切窗无重复 Surface/attachment；
- 高度不足、大字体、软键盘下无负尺寸/关键操作不可达；
- 单窗/双窗/三窗变化不重建逻辑 Session。

### M2：DSH Core/Connector 拆分与 Remote Presentation

**目标**

同一个 DSH Plugin Core 支持 local-sidecar、cloud-remote、paired-desktop，不出现两套业务状态机。

**方案与开发**

- 定义 `DshRuntimeConnector / CollaborationChannel / PresentationHandle`；
- 保留 Desktop local sidecar adapter；
- 实现 `session/open`、Queued、WebView loading/binding/ready、heartbeat/reconnect/close；
- 全链路 generation，旧 callback 丢弃；
- URL origin/path/TLS/navigation fail closed；
- 用 fake server fixture 先开发，不等待 ST2/X2。

**验收**

- local connector 无回归；
- 页面 loaded、bridge bound、Workspace attached 三个状态不混淆；
- token expiry/background/network change 可恢复；
- 日志、Crash report、analytics 无 token/URL Secret。

### M3：Mobile Resource Client、Viewer 与 Capability Bridge

**目标**

让 Mobile 全程使用 ResourceRef/handle 打开 Cloud/Paired 资源，并安全桥接 picker/share/camera 等原生能力。

**方案与开发**

- Cloud/Paired Workspace Provider client；
- 文本/Markdown bounded snapshot，图片/PDF range/short-lived handle；
- Markdown/文本/图片先行，PDF 按独立 gate；
- Agent context 只传最小片段和 revision；
- picker bytes 通过 Host stream handle，不把 `content://` 或本机 path 发给 WebView；
- 缓存明确标记 revision/离线状态。

**验收**

- 过期 handle、错误 audience/revision/generation fail closed；
- 大文件有 range/backpressure，不全量 collect；
- 切 Workspace 不闪现旧内容；
- 无 renderer 时明确 fallback/“在 Desktop 打开”。

### M4：Cloud Workspace + Mobile + Remote DSH 纵切

**目标**

形成第一个可用户验证的 Cloud-only 流程，而不等待 Pairing/BYOS/RustFS/全部 Office。

**方案与开发**

```text
login → Cloud catalog → choose Workspace
→ session/open → WebView → bind ResourceRef
→ ask Agent → proposal/approval → receipt
```

- ST2 提供 Cloud Authority；M2 提供 Remote DSH；M3 提供资源显示；
- 内测可先使用结构化 Tool/受控测试 Runtime；
- 若宣称“任意程序执行”，发布 Gate 必须再满足 X2/X3/X6；
- UI 显示数据真源、执行 placement、Storage 状态和只读/可写。

**验收**

- 真实测试账号完成完整路径；
- cross-workspace、stale revision、late callback 全拒绝；
- Storage unavailable、Queued、Binding、Degraded 有独立状态；
- Mobile 制品仍不包含 Node/DSH npm closure。

### M5：Paired Desktop、Relay 与 Desktop Workspace Provider

**目标**

让 Mobile 在 Desktop 在线时访问真实 Local Workspace 和本地 DSH，不自动上传数据。

**方案与开发**

- device registration/public key/presence；
- 同账号发现 + 人工 pairing challenge；
- 逐 Workspace read/propose/apply grant、TTL、revoke；
- Desktop outbound-only relay，业务 payload E2E；
- `PairedDesktopWorkspaceProvider` 与 `PairedDesktopDshConnector`；
- offline/sleep/reconnect/device replaced 状态。

**验收**

- 同账号未配对、已配对未授权、grant 过期全部失败；
- relay 不能读取明文业务 payload；
- Desktop 离线不 fallback 到同名 Cloud Workspace；
- 全程零自动上传，range/backpressure 可用。

### M6：Mobile Office Format Plugin（逐格式）

**目标**

按 Word、Sheet、Slides、PDF 独立接入 Flutter + Rust Engine，不把适配器排期当作底层 Engine 能力。

**方案与开发**

- `ResourceRef → bytes/range handle → engine session → export handle → expectedRevision commit`；
- Android `.so`/ABI 和 iOS XCFramework/signing 独立 artifact；
- view/edit/export capability 分别 admission；
- 无可靠原格式 export 就不注册 edit/save；
- corpus/字体/分页/公式/嵌入对象/损坏输入逐格式测试；
- 与 X5 共享 Rust Domain Core，但 UI adapter 与 Sandbox worker 分离。

**验收**

- 未实现能力不出现在 Manifest/菜单；
- Engine 不接触 S3/Workspace/DSH Secret；
- round-trip、重新解析和 concurrent revision conflict 通过；
- 低内存/崩溃隔离达到对应平台 gate。

### M7：Android Alpha

**目标与计划**

- release signing、applicationId、arm64/Play ABI；
- phone/tablet/foldable 矩阵、deep link/notification/background；
- SBOM/notices/privacy/permissions/accessibility；
- 安装、升级、回滚与 artifact verification。

**验收**

- M0/M1/M4 的 Alpha scope 全绿；
- 无 debug signing、无 Desktop runtime；
- 真机 30 分钟 Agent、50 次前后台、100 次 Workspace/Window 切换 soak 通过。

### M8：iOS Beta

**目标与计划**

- bundle ID、entitlements、TestFlight signing；
- WKWebView/picker/share/speech/background；
- iPhone/iPad、动态字体、VoiceOver；
- Privacy Manifest 和“无下载执行代码”审核说明。

**验收**

- 与 Android 使用同一 DSH/Resource fixtures；
- 没有运行时下载执行 Plugin/worker；
- TestFlight 真机和恢复场景通过。

## 10. 可并行工作流

### Wave 0：风险消除与合同准备（第 1～3 周）

可同时开展：

- C0 合同基线；
- X0 DSH contract spike；
- ST0 S3 Profile/TCK skeleton；
- M1 的 PlatformView/折叠/保活 spike；
- M2 的真实 `session/open` probe；
- M5 的 Pairing/Relay threat model；
- M6 的 Office artifact/license/ABI audit；
- ST5 RustFS qualification 环境准备。

只有 C0 是所有后续合同 PR 的合并前置；其他 spike 互不阻塞。

### Wave 1：四条基础泳道（第 3～8 周）

| 泳道 | 并行任务 |
| --- | --- |
| Platform | C1、C2、C3，随后 C4 |
| Storage/Server | ST0、ST1、ST2（先 fake blob） |
| Sandbox/DSH | X1（Local）、X4 schema/runtime 前半段 |
| Mobile | M0、M1、M2、M3，全部使用 fixtures/fake server |

Wave 1 的目标不是集成全部系统，而是让每条泳道独立通过自己的 TCK。

### Wave 2：首批纵切（第 8～16 周）

- M4 Cloud-only Mobile 内测；
- X2 Cloud Runtime；
- X3 Draft/Checkpoint；
- X4 Registry 完成；
- ST1 接入 ST2；
- M7 Android 内部 Alpha 准备。

M4 可以先使用受控 Runtime，不必等待 X5 Plugin CLI；但对外宣称通用 Bash/任意程序前必须通过 X6。

### Wave 3：产品扩展（第 14～24 周）

- X5 Dispatcher + 首个 Office worker；
- X6 Security Gate；
- M5 Paired Desktop；
- ST3 Sync；
- ST4 BYOS/Migration；
- M6 各格式按独立 Feature Flag 推进；
- M7/M8 平台发布门禁。

### Wave 4：生产演进（持续）

- ST5 RustFS 90 天 soak/canary；
- Windows Sandbox/Bash 对齐；
- 更多 Office/Domain CLI workers；
- E2EE execution placement；
- Remote editor/terminal 等非 Mobile V1 能力。

## 11. Release 序列与退出门禁

### R0：Contract/PoC Gate

包含 C0、X0、ST0 skeleton、Mobile PlatformView spike。

退出条件：关键接口有 fixture；高风险项没有未处置 P0；估算已按 spike 重排。

### R1：Independent Foundations Gate

包含 C1/C2/C3/C4、ST0/ST2 fake、X1 Local、M0/M1/M2/M3 fake。

退出条件：四条泳道各自 TCK 全绿；任何一条失败不阻止其他泳道继续，但不允许进入集成发布。

### R2：Cloud Mobile Internal Alpha

包含 ST1/ST2、M4、Android internal build；Sandbox 可先受控。

退出条件：真实账号完成 Cloud Workspace/Remote DSH/Resource/approval receipt；Mobile 无 Node；存储与执行位置可见。

### R3：Workspace Sandbox Beta

包含 X2/X3/X4、基本 X6；X5 可作为 Beta 子功能。

退出条件：Local/Cloud 同一 Provider TCK；Revision 提交正确；outer isolation/credential/quota 通过安全 Gate。

### R4：Data Autonomy & Desktop Continuity

包含 M5、ST3、ST4。

退出条件：Pairing/grant/E2E relay、local-only 零上传、snapshot/mirror/migrate 和 Provider migration 全部有 receipt 与冲突测试。

### R5：Platform Release

包含 M7、M8，以及被选入发行版的 M6/X5 capability。

退出条件：Android/iOS 真机、签名、隐私、SBOM、accessibility、恢复与 soak 门禁通过；未通过能力不打包/不注册。

### R6：Managed RustFS Rollout

只在 ST5 全部门禁完成后进行，与客户端版本号解耦。

## 12. 关键路径与非关键路径

### 12.1 Cloud-only Mobile 内测关键路径

```text
C0 → C2/C3 → ST2 ──────────────┐
      ├────────→ M2/M3 ─────────┤
C1 ─────────────→ M0 ───────────┼→ M4
C0 ─────────────→ M1 ───────────┘
```

不依赖：Paired Desktop、Sync、BYOS、RustFS、Plugin CLI、全部 Office 格式。

### 12.2 通用 Cloud Sandbox 关键路径

```text
C0 → X0 → X1 → X2 → X3 → X6
      C2/C3 ────────┘      │
      ST2 ─────────────────┘
```

不依赖：Mobile Adaptive Window、Pairing、RustFS、Office worker。

### 12.3 Plugin CLI 关键路径

```text
C1 + C2 → X4
C3 + X1 + X4 → X5
```

不依赖：Cloud Runtime、S3、Mobile。可以先在 Desktop Local Sandbox 验收。

### 12.4 数据自主关键路径

```text
ST0 → ST1 → ST2 → ST3/ST4
```

RustFS ST5 是 Provider qualification，不应阻塞 ABI、BYOS 或迁移能力。

### 12.5 Paired Desktop 关键路径

```text
C2 + C3 + M2 + M3 → M5
```

不依赖 Cloud Storage、Sync 和 BYOS；如果需要新 Sandbox 语义再软依赖 X1。

## 13. 团队与日历建议

建议最少四条稳定 owner 泳道：

| 团队 | 主要范围 | 建议配置 |
| --- | --- | --- |
| Platform Contracts | C0～C4、跨语言 fixtures/TCK | Rust/Dart 2 人 |
| Storage/Server | ST0～ST5、Cloud Authority | Rust/Backend/SRE 2 人 |
| Sandbox/DSH | X0～X6、Node/Rust/runtime security | TypeScript/Rust/Infra 2～3 人 |
| Mobile | M0～M8、Flutter/Android/iOS | Flutter + 平台 2～3 人 |
| Shared Security/QA | threat model、fuzz、release gate、device lab | 1～2 人共享 |

粗略日历目标，须在 R0 后重估：

- Cloud-only Mobile internal alpha：约 12～16 周；
- Workspace Sandbox beta：约 16～22 周；
- Paired Desktop + Sync/BYOS：约 20～28 周；
- Android Alpha + iOS Beta 的组合发布门禁：约 22～30 周；
- RustFS Managed 默认：以上时间之外，至少完成 90 天生产等价 soak。

这是 7～10 人并行的 Program 估算。若只有 3 名全栈工程师，应按价值逐条串行，合理范围约 40～60+ 周，不应把并行人周直接当作日历周。

## 14. 独立 PR 与验收纪律

每个子需求至少拆为：

1. Contract/fixture PR；
2. Provider/consumer implementation PR；
3. TCK/integration PR；
4. Feature Flag/composition PR；
5. Release Gate/observability PR。

共同规则：

- 一个 PR 不同时修改 Storage、Mobile UI 和 DSH Agent Loop；
- Consumer 先对 fake/TCK 开发，不直接等待真实 Provider；
- 新 Provider 通过 TCK 后才进入 distribution composition；
- 不支持的 capability 不注册，不能用 UI 隐藏模拟“未打包”；
- 合同变更先更新 fixture 和 compatibility policy；
- 每个 Release Gate 都必须能关闭新能力并回到上一稳定组合；
- 安全、数据完整性和迁移测试不是发布后的补充任务。

## 15. 当前建议的首批 Backlog

立即创建、可并行的第一批任务：

1. C0：跨语言 Contract Vocabulary 与 fixtures；
2. X0：DSH `0.1.7-rc.1` provider/profile contract spike；
3. ST0：BlobStorePort + fake provider + S3 TCK skeleton；
4. M1-spike：Android/iOS PlatformView suspend/restore/hinge 验证；
5. M2-spike：生产 `session/open`/Queued/bind fixture probe；
6. C1-design：统一 Manifest v2，合并 Mobile target 与 Agent CLI artifact；
7. C2-design：actor/caller/target delegation 与 handle revocation ADR；
8. C3-design：Workspace/Resource Authority v1 与 Local adapter migration；
9. M5-threat：Pairing/Relay/E2E threat model；
10. M6-audit：Word/Sheet/Slides/PDF artifact、license、ABI、export capability 审计；
11. ST5-lab：RustFS qualification 环境和测试数据生成器，不开始 90 天计时直至版本/拓扑冻结。

这些任务不存在实现级公共依赖，只共享 C0 的术语和 fixture 约束，可以在同一迭代独立开展。

## 16. 最终排序建议

按“先消除返工风险、再形成用户纵切、最后扩大生态”的顺序：

1. **先做 C0/C1/C2/C3 和 X0/ST0/Mobile spikes。**这是唯一值得集中冻结的公共基础。
2. **并行做 M0/M1/M2/M3、ST1/ST2、X1/X4。**各自用 fake 验收，不互相等待。
3. **优先集成 M4 Cloud-only Mobile。**它最快验证账号、Cloud Workspace、Remote DSH、ResourceRef 和 Mobile 产品价值。
4. **随后集成 X2/X3/X6。**把 Cloud Agent 从“可连接”提升为“可安全执行任意受允程序并正确提交”。
5. **X5 Plugin CLI 与 M5 Paired Desktop 并行。**前者扩展 Agent 生态，后者扩展数据 placement，彼此无硬依赖。
6. **再做 ST3/ST4。**在 Resource/Storage 基线稳定后开放 Sync/BYOS/迁移，避免早期双写扩大错误面。
7. **M6 各 Office 格式随 artifact 成熟度独立进入发行版。**不要阻塞 Mobile Agent/Workspace 主线。
8. **M7/M8 按真实 Release scope 收口。**只打包已经通过 target/artifact/capability Gate 的插件。
9. **ST5 始终并行。**RustFS 通过门禁后再切 Managed 默认；失败不改变上层产品 ABI。

该序列既共享必须共享的合同，又保留各领域独立设计、实现、验收和回滚能力，避免把 Mobile、Cloud Storage、DSH Sandbox、Plugin CLI 和 Office Engine绑成一次不可控的大爆炸交付。
