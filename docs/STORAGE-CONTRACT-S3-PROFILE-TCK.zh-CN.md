# OpenMuse BlobStorePort、S3 Profile 与 Provider TCK

- 状态：Accepted（ST0）
- Owner：Object Storage Domain
- 合同：`crates/openmuse-storage-contract/`
- 黑盒 TCK / reference fake：`crates/openmuse-storage-tck/`

## 1. 决策与边界

OpenMuse 依赖自己的 immutable `BlobStorePort`，不依赖 RustFS、MinIO、AWS S3 或某个 SDK。S3-compatible API 是 ST1 Provider adapter 的南向协议；本合同是 Resource Authority、Sync 和 GC 的北向 ABI。

```text
Resource Authority / Sync / GC
              │ BlobRef + digest + bounded stream + receipt
              ▼
         BlobStorePort                 BlobMaintenancePort
              │                                │
              └──────── Provider adapter ──────┘
                         │
                 AWS / MinIO / RustFS / fake
```

领域隔离规则：

- Workspace tree、文件名、revision、CAS 和 ACL 不属于 Object Storage；
- Object Storage 只保存 opaque immutable bytes 和辅助 metadata；
- endpoint、bucket、region、credential、presigned URL 与厂商错误只存在于 ST1 adapter/credential 领域；
- `ListObjects` 只通过独立的 `BlobMaintenancePort` 暴露给 GC/修复，不可用于构造 Workspace UI tree；
- DSH、Mobile 和 Office Plugin 不直接持有 `BlobStorePort`，只消费 Resource Authority 的 scoped handle。

## 2. BlobStore ABI v1

核心对象是 `BlobRef { providerRef, objectRef, digest, size }`。`objectRef` 是 Provider 内的稳定逻辑引用，不是本地 path，也不携带 bucket/endpoint。内容身份只有 lowercase SHA-256 `BlobDigest`；`ProviderValidator`（S3 中通常来自 ETag）是 opaque 并发/缓存提示，永远不解释为 MD5 或内容 hash。

每次操作携带 `StorageRequestContext`：request id、C0 actor/caller、C2 policy decision ref、workspace scope、provider generation、deadline 和 cancellation ref。ST1 adapter 必须在 I/O 前后检查 deadline/cancellation，并把晚到的旧 generation 响应拒绝为 `STALE_GENERATION`；审计字段只传身份引用，不携带 token 或 Secret。

`BlobStorePort` 提供：

- immutable `put`、`head`、`read`/range、幂等 `delete`；
- `begin_multipart`、`upload_part`、`complete_multipart`、`abort_multipart`；
- capability snapshot。

写入必须预先声明 size、SHA-256、content type、metadata 和 idempotency key。相同 key/相同内容返回 replay receipt；相同 key 或 object ref 对应不同内容返回 `CONFLICT`，不能覆盖已有 blob。

### Bounded stream

输入/输出均使用领域自己的 `BlobReadStream`，consumer 每次显式传入上限，v1 最大 chunk 为 1 MiB。Adapter 不得把 SDK `ByteStream` 暴露给调用者，也不得为了 range/普通 GET 在 adapter 内 collect 整个对象。reference fake 会 collect，这是测试实现，不是生产 adapter 的实现许可。

### Multipart

v1 profile 采用 5 MiB 非末尾 part 下限、1～10000 连续 part number。每个 part 独立校验 size/SHA-256；同 part 的相同重试稳定返回同一 receipt，不同内容返回 `CONFLICT`。Complete 接受乱序 receipt，但在提交前排序并验证连续性、完整集合、总 size 和最终 SHA-256；失败不得产生可见对象，成功后的相同 complete 可幂等重放。Abort 可重复。

## 3. 错误与 Receipt

Provider 错误归一化到 C0 跨域词汇：

`DENIED / NOT_FOUND / CONFLICT / EXPIRED / STALE_GENERATION / UNAVAILABLE / TRANSIENT / INTEGRITY_FAILED`。

厂商 request id、HTTP 状态和原始异常可进入 adapter 私有 telemetry，但不能成为上层分支条件，也不能把 endpoint、query signature 或 credential 写入错误 message。`retryable` 是稳定提示；只有 `TRANSIENT/UNAVAILABLE` 等经过 adapter 判定的失败才允许上层按幂等语义重试。

`StorageReceipt` 固定 request、operation、object、digest、size 与 replay 状态。Receipt 只证明 Provider 操作完成，不代表 Workspace revision 已提交；Resource Authority 仍需自己的 metadata CAS receipt。

## 4. S3 Data Plane Profile v1

可认证 Provider 必须声明并实测：

| 能力 | v1 Gate |
| --- | --- |
| SHA-256 | 必需；写入与读后验证不可依赖 ETag |
| Range read | 必需 |
| Multipart | 必需，非末尾 part 下限不得高于 5 MiB |
| TLS | 远端认证必需；local fixture 只验证 capability 语义 |
| Addressing | 至少 path / virtual-host 之一；ST1 按配置实测 |
| Maintenance list | 必需，但只能经维护 Port 使用 |

Versioning、Object Lock、replication、lifecycle 和 KMS 是后续 capability，不进入 v1 核心正确性。Presigned transfer、Vault/STS、endpoint SSRF/TLS 探测属于 ST1，因为它们需要 credential 和网络配置；不会塞进纯 Blob ABI。

## 5. TCK

`run_provider_tck` 接受 `dyn BlobStorePort + dyn BlobMaintenancePort`，因此 fake、AWS SDK adapter、MinIO 和 RustFS 运行完全相同的场景。只有 `failures` 为空的报告才可形成已认证 capability snapshot。

当前自动 Gate 覆盖：

- put/head/get/range/delete 与读后可见性；
- UTF-8、空格和保留字符 object ref；
- content type、metadata、SHA-256 与 opaque validator；
- immutable/idempotency conflict；
- 0 byte、5 MiB 边界和真实 100 MiB+ multipart；
- part/complete 重试、乱序 complete、abort；
- 上传中断不产生部分可见对象，随后可重试；
- checksum mismatch fail closed；
- maintenance list 分页。

运行：

```bash
scripts/test_storage_contract.sh
```

### 真实 Provider 认证交接

ST0 交付可复用 TCK 和 fake 基准，不伪造真实网络认证结果。AWS S3、MinIO、RustFS 的真实 snapshot 必须在 ST1 `openmuse-storage-s3` adapter 接入后，针对精确的 provider version、endpoint policy、addressing style 与 TLS 配置运行同一 TCK；没有凭据/endpoint 的 CI 不得生成“通过”记录。ST1 至少保存：

- provider kind/version 与 adapter commit；
- profile version、addressing style、TLS/checksum 行为；
- TCK report、运行时间和环境引用（不含 Secret）；
- 失败场景和 waiver（核心正确性项不允许 waiver）。

RustFS 的故障、升级与 90 天 soak 仍属于 ST5，不因通过功能 TCK 自动成为生产默认。

## 6. 后续实施

- ST1：实现 `aws-sdk-s3` adapter、Vault/STS、endpoint/SSRF/TLS、presign，并对 AWS/MinIO/RustFS 跑本 TCK；
- ST2：Resource Authority 用 BlobStorePort 完成 immutable blob + metadata CAS/outbox；
- ST3/ST4：Sync/BYOS/迁移只依赖此 ABI 和 Resource Authority；
- ST5：在相同 digest/TCK 基线上执行 RustFS 长期 qualification。
