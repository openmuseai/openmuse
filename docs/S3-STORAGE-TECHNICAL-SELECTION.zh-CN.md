# OpenMuse Office S3 Storage ABI 与 MinIO / RustFS 技术选型

状态：ADR 提案 v1（2026-09-29）

关联：[MOBILE-PRODUCT-PRD.zh-CN.md](MOBILE-PRODUCT-PRD.zh-CN.md)、[MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md](MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md)、[PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md](PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md)

## 1. 决策

OpenMuse Office 不把 MinIO 或 RustFS 作为产品领域依赖，而把一个经过 OpenMuse 认证的 **S3 Data Plane Profile** 定义为 Storage ABI。

选型结论：

1. **产品协议：**标准 Amazon S3 API 的受控子集，不依赖 MinIO Admin API、RustFS Admin API、私有磁盘格式或厂商 SDK。
2. **Rust 服务端客户端：**`aws-sdk-s3`，集中在 Object Storage Provider 内；Flutter、Office Engine、Workspace 和 DSH 都不直接 import S3 SDK。
3. **官方新建自托管存储：**RustFS 是战略首选，但 1.0 初期必须先通过 OpenMuse conformance、故障恢复、安全版本和 90 天 soak 门禁，不能因为实现语言相同就直接进入唯一生产环境。
4. **MinIO：**继续作为必须兼容的 BYOS Provider、已有部署迁移源和短期回退环境；不作为新建 OpenMuse 托管集群的长期默认。
5. **若今天必须承载高价值生产数据且 RustFS 尚未过门禁：**优先使用已通过相同 Profile 的成熟托管 S3 服务；不能在 RustFS 与已归档的 MinIO Community 之间被迫二选一。
6. **数据自主：**用户可以选择 OpenMuse 托管 Storage、自己的 S3-compatible endpoint，或仅保留 Desktop 本地数据。迁出、删除、凭据撤销、完整性验证和恢复演练是产品能力，不是文档承诺。

一句话：**OpenMuse 依赖自己的 Storage Port 和 S3 Profile；RustFS/MinIO 只是可替换 Provider。**

## 2. 官方状态核对

截至本 ADR 日期：

- [MinIO Community 仓库](https://github.com/minio/minio)采用 AGPL-3.0；GitHub 已将其标为 2026-04-25 归档只读。[最后公开 release](https://github.com/minio/minio/releases)为 `RELEASE.2025-10-15T17-29-55Z`，安装说明要求从源码构建容器。
- [RustFS 仓库](https://github.com/rustfs/rustfs)采用 Apache-2.0，提供分布式、Erasure Coding、Versioning、Object Lock、IAM/Policy、KMS、Replication 等能力；[官方 Releases](https://github.com/rustfs/rustfs/releases)显示 1.0.0 于 2026-09-16 发布，后续 1.0.1 仍是 preview 线。
- [RustFS 官方兼容矩阵](https://github.com/rustfs/rustfs/blob/main/docs/architecture/s3-compatibility-matrix.md)明确不声称覆盖全部标准或 vendor-specific S3 行为。其文档列出了 bucket logging、ownership controls、POST checksum、multipart 边界和 ACL 等差异。
- [RustFS Security Advisories](https://github.com/rustfs/rustfs/security)在 2026 年披露过多项 IAM、Object Lock、Console 和非 S3 协议相关安全问题。安全披露本身是好信号，但说明 OpenMuse 仍需固定版本、快速升级和自己的回归门禁，不能只看 feature table。
- [AWS SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/endpoints.html)支持 custom endpoint 和 `force_path_style`，适合用一个客户端实现 AWS S3 与 S3-compatible Provider；presigned GET/PUT 也有官方支持。

这些事实支持“RustFS 作为未来默认候选”，但不支持“RustFS 已经是无条件的 100% MinIO 替代品”。

## 3. 选型标准

### 3.1 硬门槛

- S3 Profile 所需操作通过真实黑盒测试；
- Apache-2.0/商业分发边界可接受，第三方 notices 完整；
- 支持固定版本、校验和、SBOM、可复现安装和离线升级；
- 单节点与分布式模式都具备备份/恢复、扩容、坏盘和滚动升级剧本；
- 安全公告有明确响应窗口，OpenMuse 能在规定时间完成升级；
- 不要求 Office/Workspace 了解 Provider 私有概念；
- 数据可迁出到另一个通过 Profile 的 S3 Provider，且 digest 一致。

### 3.2 权重

| 维度 | 权重 | 说明 |
|---|---:|---|
| 数据正确性与恢复 | 25% | Office 原件不可静默损坏 |
| S3 Profile 兼容 | 20% | 只评价 OpenMuse 实际操作，不按宣传页打分 |
| 安全与升级 | 15% | IAM/KMS/凭据/补丁周期 |
| 运维成熟度 | 15% | 监控、扩容、故障演练、runbook |
| 许可证与可持续性 | 10% | 分发、修改和长期维护风险 |
| 性能与成本 | 10% | p95/p99、资源、容量效率 |
| 技术栈一致性 | 5% | Rust 是加分项，不是正确性替代品 |

## 4. MinIO 与 RustFS 的结论性比较

| 维度 | MinIO Community | RustFS | OpenMuse 判断 |
|---|---|---|---|
| License | AGPL-3.0 | Apache-2.0 | RustFS 更适合作为可分发默认组件；仍需法律审查 |
| 上游状态 | 仓库归档、最后 release 为 2025-10 | 活跃，刚进入 1.0 | MinIO 长期性弱；RustFS 新版本风险高 |
| 历史生产经验 | 很强 | 相对较新 | 短期成熟度 MinIO 占优 |
| S3 兼容 | 历史成熟 | 经过测试的广泛子集 | RustFS 必须以 Profile 实测，不接受“完全兼容”假设 |
| 分布式/EC | 成熟 | 已提供 | 均需在 OpenMuse 拓扑上做破坏测试 |
| IAM/KMS/Object Lock | 成熟但版本停滞 | 功能存在，近期仍有安全修复 | 两者都不能替代最小权限与升级门禁 |
| 官方二进制/供应链 | CE 后期转 source-only | 1.0 有二进制、校验和、SBOM/provenance | RustFS 更利于当前自动化，但需验证签名链 |
| 技术栈 | Go | Rust | RustFS 与团队技能更一致，但 Provider 必须保持进程边界 |
| 新建官方托管 | 不推荐长期默认 | **有条件推荐** | RustFS 通过 Gate 后启用 |
| BYOS 兼容 | 必须支持 | 必须支持 | 两者都只是 Provider |

### 4.1 为什么不是立即无条件切 RustFS

RustFS 1.0.0 距本 ADR 很近，1.0.1 仍处 preview；兼容矩阵存在明确边界，且 pre-1.0 曾出现升级/恢复风险报告。Office 原件的风险函数不是“服务能启动”，而是：

```text
写入确认后可读
+ 并发提交不覆盖
+ multipart 可恢复/清理
+ 节点/盘故障后 digest 不变
+ 升级/回滚不破坏旧数据
+ 凭据和策略不能越权
```

因此推荐的是一条路线，而不是跳过验证的品牌替换。

## 5. S3 不是 Workspace 数据库

S3 只保存不可变 bytes、预览、导出和可选备份。它不拥有：

- Workspace/成员/设备身份；
- 文件树、重命名、移动和排序语义；
- Office 文档当前 revision；
- 协作 CRDT/操作日志；
- 插件注册、DSH session 或审批；
- 跨对象事务和并发写锁。

[Amazon S3 官方一致性说明](https://docs.aws.amazon.com/AmazonS3/latest/userguide/Welcome.html)保证单 key 的强 read-after-write，但不提供跨 key 原子事务，也不替应用解决并发 writer。OpenMuse 必须把 revision/CAS、引用计数、commit receipt 和目录事务放在 Resource/Workspace 领域。

目标分层：

```text
Workspace / Office Domain
  └─ Resource Authority (resourceRef, revision, CAS, ACL)
       ├─ Metadata Store (PostgreSQL / Desktop SQLite)
       ├─ Collaboration Log Provider
       └─ BlobStorePort
            └─ S3StorageProvider (aws-sdk-s3)
                 ├─ OpenMuse Managed / RustFS
                 ├─ User BYOS / MinIO or RustFS
                 ├─ AWS S3 / R2 / other certified provider
                 └─ test fake
```

## 6. Office 对象模型

### 6.1 不可变 blob + 应用 revision

Office 保存采用两阶段 commit：

```text
ioffice.toDocx/toXlsx/toPptx
  → validate format
  → sha256 + size + mediaType
  → BlobStore.putIfAbsent(digest, stream)
  → verify HEAD/checksum
  → ResourceAuthority.commit(expectedRevision, BlobRef)
  → committed new ResourceRevision
  → resource.changed receipt
```

对象 key 由 OpenMuse 生成，用户文件名不直接成为权限边界：

```text
v1/blobs/sha256/{digest-prefix}/{digest}
v1/previews/{workspaceRef}/{resourceRef}/{revision}/{variant}
v1/exports/{workspaceRef}/{jobRef}/{artifact}
```

同一 key 永不原地覆盖。并发控制由 metadata CAS 完成，S3 ETag 不能充当通用内容哈希：multipart、加密和不同 Provider 下 ETag 语义不稳定。

### 6.2 写入失败边界

- blob 成功、metadata 失败：形成未引用 blob，由延迟 GC 在 grace period 后回收；
- metadata 成功前不得向用户返回保存成功；
- metadata 成功、事件失败：用 outbox 重放 `resource.changed`；
- multipart 未完成：后台 abort；
- Provider 暂时不可用：保留本地草稿/导出，不把 dirty 清零；
- digest/size 不一致：隔离对象并返回 `STORAGE_INTEGRITY_FAILED`。

## 7. OpenMuse S3 Data Plane Profile v1

### 7.1 必需能力

| 能力 | 用途 |
|---|---|
| SigV4 + custom endpoint/region | 通用连接 |
| path-style 与 virtual-host-style 可配置 | 自建和公有云差异 |
| Put/Get/Head/Delete Object | 核心 blob CRUD |
| Range GET | Mobile/PDF/大文件 |
| ListObjectsV2 + prefix + pagination | 修复/GC；不用于用户文件树 |
| multipart create/upload/complete/abort | 大型 Office/附件 |
| presigned GET/PUT | 受限直传/下载 |
| user metadata + content type | digest/格式辅助验证 |
| checksum 或 OpenMuse 端到端 SHA-256 | 完整性 |
| read-after-write probe | 写后验证 |
| TLS + hostname validation | 非本机 endpoint 必需 |

### 7.2 增强能力

- Versioning：官方托管默认开启，BYOS 推荐；OpenMuse revision 不依赖它；
- Object Lock：合规/备份 profile 可启用，普通 Workspace 不强制；
- SSE-S3/SSE-KMS/SSE-C：按 provider capability 协商；
- lifecycle：清理 preview/export/未引用 blob；无此能力时由 OpenMuse GC；
- replication/notification：运维增强，不进入核心正确性；
- STS/OIDC：优先于长期 access key。

### 7.3 明确不依赖

- ACL；统一使用 IAM/bucket policy；
- MinIO/RustFS Admin API；
- bucket ownership controls、access points、S3 Select；
- Provider 的本地磁盘布局；
- POST form upload；首发使用 presigned PUT/multipart；
- ETag=MD5 假设；
- ListObjects 作为 Workspace catalog。

## 8. Provider 配置与凭据

`S3ProviderConfig` 建议包含：

```text
providerRef
endpoint
region
bucket
prefix
addressingStyle = auto | path | virtualHost
credentialRef
tlsPolicy
encryptionProfile
capabilitySnapshot + testedAt
```

安全规则：

- secret 不写进 Workspace manifest、Flutter settings、DSH context 或日志；
- Server-mediated BYOS 的密钥进入 Credential Vault，使用 envelope encryption；
- 优先支持用户创建的 scoped IAM/STS role，仅允许指定 bucket/prefix；
- Mobile 只拿短期 resource handle/presigned request，不拿永久 S3 secret；
- Desktop-only BYOS 可选择本机 Credential Store，此时云端和 Mobile 只有 Desktop 在线时才能访问；
- endpoint 必须防 SSRF：解析 DNS、阻止意外 loopback/private/metadata address，私有 endpoint 需通过用户部署的 Storage Agent 显式连接；
- “连接测试”必须使用专用随机 prefix，完成 put/head/range/get/delete/multipart 后清理，不能只做 ListBuckets。

## 9. 用户数据自主的三种模式

| 模式 | 凭据位置 | Mobile/Cloud 可用性 | 适合用户 |
|---|---|---|---|
| OpenMuse Managed | OpenMuse 服务端；官方 Storage Provider | 始终可用 | 希望开箱即用 |
| BYOS Server-mediated | OpenMuse Vault 中的 scoped role/key | 始终可用；OpenMuse 数据面可访问 bytes | 自选 S3、接受服务端代理 |
| BYOS Desktop-mediated | 用户 Desktop Credential Store | Desktop 在线时可用；可端到端中继 | 不愿把 S3 凭据交给 OpenMuse |

“自带存储”不等于“端到端不可见”。UI 必须明确谁持有密钥、OpenMuse/DSH 是否能读取明文、离线时谁可访问。

### 9.1 加密等级

1. Baseline：TLS + Provider SSE，OpenMuse Resource Authority 可读取明文以支持预览、索引与 DSH。
2. Customer-managed KMS：bucket 使用用户 KMS key，OpenMuse role 获得限定 decrypt 权限。
3. Client E2EE（后续独立 profile）：Office bytes 在客户端加密；S3 和 OpenMuse Cloud 不持明文。此模式下远端 DSH/索引/预览默认不可用，除非用户向某次 session 授予短期解密 capability。

不能用“使用用户 S3”暗示已经 E2EE。

## 10. Plugin 与领域隔离

### 10.1 插件边界

```text
Office Engine Plugin
  consumes: resource.materialize / resource.commit
  knows nothing about: S3 endpoint, bucket, access key

Workspace Sync Plugin
  consumes: workspace.changeFeed / resource.read / resource.commit
  coordinates: local ↔ cloud revisions
  knows BlobStore only through scoped Host service

Object Storage Provider Plugin (Server/Desktop infrastructure)
  implements: blob.put/get/head/range/delete/multipart
  owns: aws-sdk-s3 client and provider quirks

DSH Workspace Plugin
  consumes: resource query/propose/apply capabilities
  never receives: S3 credentials or raw provider config
```

Host 只拥有 Broker、Resource Authority、permission/policy 和 provider selection，不包含 S3 API 分支或 iOffice 保存实现。DSH 的 Everything is Plugin 不意味着插件可以跨过 Host 直接访问对象存储。

### 10.2 Provider quirk 隔离

不同 S3 实现的差异只允许存在于 `S3StorageProvider`：endpoint、path style、checksum、presign、multipart、错误归一化。上层只看到：

```text
BlobRef / BlobDigest / BlobLease
StorageReceipt
StorageErrorCode
ProviderCapabilitySnapshot
```

## 11. Rust 实现建议

```text
crates/openmuse-storage-contract/       # provider-neutral traits/types/errors
crates/openmuse-storage-s3/             # aws-sdk-s3 implementation
crates/openmuse-storage-tck/            # black-box conformance kit
crates/openmuse-storage-credentials/    # vault/STS/rotation ports
crates/openmuse-resource-store/         # metadata CAS/outbox/GC
plugins/workspace-sync/                 # sync policy/orchestration
```

核心 trait 不暴露 `aws_sdk_s3::Client`、`ByteStream` 或厂商错误类型。流使用领域自己的 bounded stream/reader；所有操作带 workspace scope、deadline、cancellation、request id 和 audit context。

当前 Muse Server 已使用 `aws-sdk-s3` 和 `BucketClient`，可以演进而不是重写；但现有接口把 AWS `ByteStream` 泄漏到 trait、错误归一化较粗、部分读取会 collect 全对象、presigned URL 曾直接记录 URL。新合同必须修复这些边界，尤其禁止记录带签名 query 的 URL。

## 12. Conformance 与生产 Gate

### 12.1 Provider TCK

每个 provider/version/配置组合必须运行：

- 单对象 put/head/get/range/delete，立即 read/list；
- 0B、小对象、5MiB 边界、100MiB+ multipart；
- multipart abort、重复 part、乱序/重试 complete；
- UTF-8、空格、保留字符和长 key；
- content-type/user metadata/checksum round trip；
- presigned GET/PUT 过期、header、path-style；
- 并发 putIfAbsent/CAS 的应用层语义；
- versioning on/off；
- 401/403/404/409/429/5xx/timeout/DNS/TLS 错误归一化；
- 中断上传、进程重启、磁盘满、坏盘、节点丢失、网络分区；
- 升级 N-1→N、失败回滚、备份恢复、全量 digest scrub；
- 凭据轮换与撤销；
- 10k/1M object prefix 的 list/GC 性能。

### 12.2 RustFS 进入官方生产的附加门禁

- 固定 stable release，不使用 preview/nightly；
- 所有已知高/严重安全公告已修复，安全扫描无未接受 blocker；
- 4 节点/多盘拓扑完成故障注入与恢复；
- 生产等价数据量连续 soak ≥90 天；
- 完成跨版本滚动升级和回滚演练；
- 从当前 MinIO/目标托管 S3 的双读校验迁移通过；
- RPO/RTO、监控、容量、on-call 和 runbook 由实际演练确认。

RustFS 未过 Gate 时，OpenMuse Managed 使用已认证的托管 S3 或保留现有 MinIO 部署；产品 ABI 不变。

## 13. 迁移与退出策略

迁移不复制 Provider 的磁盘目录，始终走 S3 API：

```text
freeze/dual-write policy
  → enumerate metadata-owned BlobRefs
  → source GET + digest verify
  → destination putIfAbsent
  → destination HEAD/GET sample or full scrub
  → switch provider generation
  → observation window
  → retire source after explicit approval
```

Provider selection 带 generation。切换后旧 generation 的迟到写不能成为当前 revision。任何迁移都可暂停、续传和生成逐对象 receipt；禁止按 `ListObjects` 猜测 Workspace 真源。

用户退出必须能够获得：

- Workspace manifest、resource/revision 元数据导出；
- 原始 Office bytes 和 digest 清单；
- 对象 key/version mapping；
- 验证工具；
- 删除请求及 Provider/metadata 两侧 receipt。

## 14. 分阶段落地

| 阶段 | 交付 | Gate |
|---|---|---|
| S0 Contract | Storage contract、S3 Profile、TCK fixtures | fake + AWS SDK local contract 绿 |
| S1 Existing Server Adapter | 收敛当前 `BucketClient`、流式读取、错误/日志安全 | 现有 MinIO/AWS 测试无回归 |
| S2 BYOS Preview | Provider 配置、Vault、连接测试、capability snapshot | 两个不同 S3 实现通过 |
| S3 RustFS Qualification | 固定 RustFS stable、故障/升级/安全/soak | §12.2 全通过 |
| S4 Managed Rollout | shadow copy、双读校验、小租户 canary | 无 digest mismatch、可回滚 |
| S5 Data Portability | 导入/导出/迁移/删除 receipts | 用户可独立验证并迁出 |

## 15. 最终建议

- **现在冻结：**S3 是 Storage ABI；`aws-sdk-s3` 是 Rust adapter；应用层 revision/CAS 是正确性真源。
- **现在实现：**Provider TCK、BYOS、凭据/SSRF/secret-log 安全、immutable blob 模型。
- **现在选择：**RustFS 作为官方新建自托管方向；MinIO 作为兼容与迁移 Provider。
- **不要现在承诺：**未经 90 天生产等价验证就称 RustFS 为唯一生产默认；也不要因 MinIO 历史成熟而把新产品长期锁在已归档 Community 线上。
- **用户价值：**同一个 Workspace 可以在 OpenMuse 托管、用户 BYOS 或 Desktop-mediated 模式间迁移，Office/DSH/Flutter UI 无需知道底层是哪家对象存储。
