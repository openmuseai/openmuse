# Workspace、资源与同步

Workspace / Resource Authority 是共享语义层：稳定 Ref、临时 Handle、权限、路由与保留策略。这里同时收纳同步插件的冲突合同、草稿/检查点恢复、云 Workspace 元数据与多端资源交接。

## 文档

| 文档 | 内容 |
| --- | --- |
| [OpenMuse Workspace / Resource Authority v1](WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md) | Authority 边界、Ref 与 Handle 的 wire 语义，域入口 |
| [多端资源引用、可用性与对话交接](RESOURCE-REFERENCE-AVAILABILITY-AND-HANDOFF.zh-CN.md) | 资源身份跨端复用、可用性计算与对话内交接 |
| [Workspace Sync Plugin：策略、持久化编排与冲突合同](WORKSPACE-SYNC-PLUGIN.zh-CN.md) | 同步策略、持久化队列与冲突处理 |
| [Draft、Checkpoint、Quiescence 与恢复](DRAFT-CHECKPOINT-QUIESCENCE-RECOVERY.zh-CN.md) | 草稿保存、静默点、崩溃恢复语义 |
| [Cloud Workspace Metadata、CAS 与 Outbox](CLOUD-WORKSPACE-METADATA-CAS-OUTBOX.zh-CN.md) | 云端元数据、内容寻址与可靠外发 |

## 相关领域

- 存储与 Blob：[storage](../storage/README.md)
- 多端连接：[devices-ctrl](../devices-ctrl/README.md)
- 设计侧实现契约：[design/mobile-resource-center/IMPLEMENTED-CONTRACT](../design/mobile-resource-center/IMPLEMENTED-CONTRACT.zh-CN.md)
