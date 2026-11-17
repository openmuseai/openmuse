# DSH

DSH 作为 OpenMuse 的对话与执行内核：对话流选型、Flutter 原生复刻、Provider 合同、插件宿主接缝、Workspace 执行平面与功能域迁移。Host 只提供受控资源、凭据、打开资源与审计，不内嵌 DSH 实现。

## 文档

| 文档 | 内容 |
| --- | --- |
| [DSH 对话流技术选型](DSH-CONVERSATION-SURFACE-SELECTION.zh-CN.md) | 对话界面的候选方案、取舍与决策 |
| [DSH 原生 Flutter 对话流复刻架构、开发计划与验收矩阵](DSH-NATIVE-FLUTTER-CONVERSATION-ARCHITECTURE-PLAN.zh-CN.md) | 在 Flutter 中复刻官方对话流的完整计划与验收项 |
| [DSH 0.1.7-rc.1 Provider Contract Spike 与 TCK](DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md) | Provider 合同探针、TCK 与兼容结论 |
| [DSH 插件与本地 Host 接缝迁移设计](DSH-PLUGIN-HOST-MIGRATION.zh-CN.md) | 插件宿主、sidecar 与 Host 接缝的迁移方案 |
| [Workspace Sandbox 与 DSH Execution Plane 设计评估](DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY.zh-CN.md) | 执行平面可行性、沙箱边界与资源流 |
| [Cloud Execution Runtime 与 DSH Provider 组](CLOUD-EXECUTION-RUNTIME-DSH-PROVIDERS.zh-CN.md) | 云端执行运行时与 Provider 组合方式 |
| [Workspace、版本 Diff 与 DSH 功能域迁移](WORKSPACE-VERSION-DSH-MIGRATION.zh-CN.md) | 从旧仓库迁移 Workspace、Diff 与 DSH Host Plane 的结论 |

## 相关领域

- 插件协议与宿主边界：[plugins](../plugins/README.md)
- Sandbox 与 Lease：[sandbox](../sandbox/README.md)
- 跨域合同：[architecture/CONTRACT-BASELINE-V1](../architecture/CONTRACT-BASELINE-V1.zh-CN.md)
