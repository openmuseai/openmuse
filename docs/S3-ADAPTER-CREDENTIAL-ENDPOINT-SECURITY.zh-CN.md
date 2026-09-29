# OpenMuse S3 Adapter、Credential 与 Endpoint 安全

- 状态：Accepted（ST1；AWS S3 live test 经产品负责人明确豁免）
- 分支：`feature/st1-s3-adapter-security`
- 实现：`crates/openmuse-storage-s3/`、`crates/openmuse-storage-credentials/`

## 1. 边界

ST1 是 ST0 `BlobStorePort` 的南向实现。只有 `openmuse-storage-s3` 可以 import `aws-sdk-s3`；Resource Authority、Workspace、Office、DSH 和 Mobile 只能看到 `BlobRef`、bounded stream、receipt 和归一化错误。

```text
Resource Authority
      │ BlobStorePort
      ▼
AwsS3BlobStore ── CredentialLease ── Vault
      │
      ├─ EndpointGuard（HTTPS / SSRF / DNS generation）
      ├─ aws-sdk-s3 1.88（与现有 Server 版本对齐）
      └─ AWS S3 / MinIO / RustFS
```

现有 Muse Server 的 legacy `BucketClient` 仍直接暴露 AWS `ByteStream`，部分 GET 会 collect 全对象，且存在输出 presigned URL 的 trace。新代码不复用该合同；Server 迁移必须删除这些调用点后才能宣称全产品 ST1 完成。

## 2. 数据面

`AwsS3BlobStore` 实现 immutable put/head/bounded get/range/delete、multipart create/part/complete/abort 和 maintenance list：

- 上传从 `BlobReadStream` 以不超过 1 MiB chunk 读取，边读边计算 SHA-256，并写入受控临时文件；不会在内存 collect 整对象；
- SDK 从临时文件流式发送，PUT 使用 `If-None-Match: *`；同 object ref 不允许不同内容覆盖；
- digest/size 写入 S3 user metadata，PUT/multipart complete 后 HEAD 复核；ETag 只保存在 opaque `ProviderValidator`；
- GET 将 SDK `ByteStream` 投影为 bounded stream，大 SDK chunk 会分片返回；
- multipart part 与 complete 在进程内幂等，非末尾 part 至少 5 MiB，最终 HEAD 校验整体 digest/size；
- HTTP 401/403/404/409/412/429/5xx 归一化为 C0 错误，不把 SDK/vendor error 字符串上抛。

临时 staging 是 bounded-memory 策略，不是持久 upload journal。Cloud worker 崩溃恢复、orphan multipart 扫描/abort 和 durable idempotency record 仍需 ST2 的 metadata/outbox worker；进程内 registry 不可当作跨重启真源。

## 3. Credential

上层配置只保存 `credentialRef`。`CredentialVaultPort` 按 audience 返回带 generation/TTL 的短期 lease：

- rotate 单调增加 generation；旧 lease 对新请求返回 `STALE_GENERATION`；
- revoke 后不能签发新 lease；
- `SecretBytes` 不可序列化，Debug 固定 `[REDACTED]`，Drop 使用 zeroize；
- `build_client_from_lease` 只接受 live 且 generation 匹配的 lease；新请求从新 lease 构建 client，已经开始的授权请求可使用其原 client 完成；
- Flutter、DSH、Office/Sandbox worker 永远不接收 access key/secret/session token。

当前 `InMemoryCredentialVault` 仅用于 TCK。生产 Vault/KMS/OS keychain adapter 必须实现同一 Port，不能把 secret 落入普通配置或数据库明文字段。

## 4. Endpoint 与日志安全

Public endpoint 必须是无 userinfo/query/fragment/path 的 HTTPS origin。DNS 结果只允许公网地址；loopback、RFC1918、link-local、metadata address、共享地址、文档/benchmark/reserved/multicast 均拒绝。HTTP 只允许显式 `LoopbackDevelopment`，且 DNS 结果必须全部是 loopback。

`EndpointGuard` 固定首次解析的 authority 与 address-set digest；`RevalidatingEndpointCheck` 在每次操作前复验，地址集合变化即拒绝。SDK HTTP connector 使用同一 `ValidatedEndpoint` 的地址集合构造固定 DNS resolver，不会在复验后再次采用未批准地址；伪造或与配置不一致的 endpoint snapshot 会被拒绝。TLS 默认使用平台信任根，也支持 adapter 侧注入 PEM CA bundle，CA 内容不进入 Workspace/Plugin 合同。

Presigned PUT：

- TTL 限制为 1～900 秒；绑定 content length/type、digest/size metadata 和 immutable 条件；
- URL 使用不可序列化、Debug 脱敏的 `SensitiveUrl`；
- `redact_url` 移除 userinfo/query/fragment；
- crate 禁止 ad-hoc trace/debug/info/warn/error，避免 URL、object key 或 credential 进入日志。

## 5. 验收

Hermetic Gate：

```bash
scripts/test_storage_s3.sh
```

覆盖 Rust 1.85 MSRV、credential rotation/revoke/audience/TTL、Secret redaction、HTTPS/SSRF、metadata IP、DNS 变化与 connector 地址固定、key prefix、URL redaction、bounded staging、checksum mismatch 和 HTTP 错误归一化。

真实 Provider 使用同一 ST0 TCK，测试入口默认 ignored，避免在没有隔离 bucket 时误删用户数据。固定镜像、临时 TLS、隔离 bucket 和报告生成可由统一入口执行：

```bash
scripts/run_storage_provider_tck.sh minio /tmp/minio-report.json
scripts/run_storage_provider_tck.sh rustfs /tmp/rustfs-report.json
scripts/run_storage_provider_tck.sh aws_s3 /tmp/aws-s3-report.json
```

本地 Provider 模式生成随机临时凭据，结束时删除精确命名的容器、匿名 volume 和证书目录；报告不包含 endpoint、object key、凭据或 presigned URL。报告同时记录 Git revision 与 adapter source digest，使“先验收、后提交”的结果仍可对应到精确源码。AWS 模式不创建 bucket，要求调用方预先注入下列环境变量：

```bash
OPENMUSE_TCK_S3_KIND=minio \
OPENMUSE_TCK_S3_ENDPOINT=http://localhost:9000 \
OPENMUSE_TCK_S3_REGION=us-east-1 \
OPENMUSE_TCK_S3_BUCKET=openmuse-tck \
OPENMUSE_TCK_S3_PREFIX="run/$(date +%s)" \
OPENMUSE_TCK_S3_ACCESS_KEY_ID=... \
OPENMUSE_TCK_S3_SECRET_ACCESS_KEY=... \
OPENMUSE_TCK_S3_CA_BUNDLE=/path/to/private-ca.pem \
OPENMUSE_TCK_PROVIDER_VERSION='exact provider version/image digest' \
OPENMUSE_TCK_ADAPTER_REVISION="$(git rev-parse HEAD)" \
OPENMUSE_TCK_ADAPTER_SOURCE_DIGEST=... \
OPENMUSE_TCK_REPORT_PATH=/isolated/output/provider-report.json \
cargo test -p openmuse-storage-s3 --test live_provider_tck -- --ignored --nocapture
```

`OPENMUSE_TCK_S3_CREATE_BUCKET=1` 只用于一次性隔离环境；默认不会创建 bucket。不得在共享或生产 bucket 运行。MinIO `RELEASE.2025-09-07T16-13-09Z` 与 RustFS `1.0.0` 已分别使用 HTTPS、自定义 CA 和固定 DNS connector 跑通完整 TCK，报告见 [`qualification/storage/ST1-ACCEPTANCE.zh-CN.md`](qualification/storage/ST1-ACCEPTANCE.zh-CN.md)。产品负责人明确豁免本阶段真实 AWS S3 验证；不得据此宣称 AWS 已实测。RustFS 的 90 天 soak 仍属于 ST5。
