# 存储

对象存储与数据可移植：S3 ABI 与 MinIO / RustFS 选型、Adapter 的凭据与端点安全、BlobStorePort / S3 Profile / Provider TCK，以及 BYOS 与数据迁出。资质验收的原始记录在 [qualification/storage](../qualification/storage/ST1-ACCEPTANCE.zh-CN.md)。

## 文档

| 文档 | 内容 |
| --- | --- |
| [OpenMuse Office S3 Storage ABI 与 MinIO / RustFS 技术选型](S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md) | 存储 ABI、候选实现与选型结论，域入口 |
| [OpenMuse S3 Adapter、Credential 与 Endpoint 安全](S3-ADAPTER-CREDENTIAL-ENDPOINT-SECURITY.zh-CN.md) | 凭据管理、端点校验与攻击面约束 |
| [OpenMuse BlobStorePort、S3 Profile 与 Provider TCK](STORAGE-CONTRACT-S3-PROFILE-TCK.zh-CN.md) | 跨实现合同与一致性测试套件 |
| [RustFS Qualification 与 Managed Rollout Gate](RUSTFS-QUALIFICATION-MANAGED-ROLLOUT.zh-CN.md) | RustFS 资质门槛与受控放量条件 |
| [BYOS、Provider 迁移与数据可移植](BYOS-PROVIDER-MIGRATION-PORTABILITY.zh-CN.md) | 自带存储、Provider 切换与迁出路径 |

## 相关领域

- Workspace 元数据与 CAS：[workspace/CLOUD-WORKSPACE-METADATA-CAS-OUTBOX](../workspace/CLOUD-WORKSPACE-METADATA-CAS-OUTBOX.zh-CN.md)
- 跨域合同：[architecture/CONTRACT-BASELINE-V1](../architecture/CONTRACT-BASELINE-V1.zh-CN.md)
- 资质验收材料：[qualification/storage](../qualification/storage/)
