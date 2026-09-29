# ST1 验收记录

- 状态：Accepted
- 日期：2026-09-29
- 分支：`feature/st1-s3-adapter-security`
- Adapter source digest：`1f55cb72fc026fa2771ce97d92618874b5001b5d21dd931dc82da02581536579`

## 通过项

- Rust 1.85 全目标编译、单元测试、Clippy 与 workspace 回归通过；
- credential rotation/revoke/audience/TTL、secret/URL 脱敏通过；
- HTTPS、SSRF、DNS generation、固定 connector 地址与自定义 CA 通过；
- MinIO `RELEASE.2025-09-07T16-13-09Z`：S3 Profile 9/9；
- RustFS `1.0.0`：S3 Profile 9/9；
- 两个 Provider 均覆盖 100 MiB+ multipart。

## Waiver

产品负责人于 2026-09-29 明确决定 ST1 不要求真实 AWS S3 验证，允许直接进入下一子需求。因此 AWS S3 live report 不作为本次退出条件；AWS 兼容性仍由相同 `aws-sdk-s3` adapter、S3 Profile 与后续可选认证入口覆盖，不得将本 waiver 表述为“已在 AWS S3 实测”。

正式报告：[`st1-minio.json`](st1-minio.json)、[`st1-rustfs.json`](st1-rustfs.json)。
