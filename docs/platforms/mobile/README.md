# Mobile

Flutter 单壳 + 共享 Host Shell 的移动端实现域：整体架构、Cloud Workspace 接入、Office 查看与 DOCX 引擎、ASR 语音插件。产品定义见 [product/MOBILE-PRODUCT-PRD](../../product/MOBILE-PRODUCT-PRD.zh-CN.md)，插件协议见 [plugins](../../plugins/README.md)。

## 文档

| 文档 | 内容 |
| --- | --- |
| [OpenMuse Mobile 架构与实现计划](MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md) | Mobile 分层、模块、依赖与实施顺序，域入口 |
| [Mobile App Root 与共享 Host Shell](MOBILE-APP-ROOT-HOST-SHELL.zh-CN.md) | App 根、Host Shell 的装配边界与共享方式 |
| [Mobile Cloud Workspace Service Integration](MOBILE-CLOUD-SERVICE-INTEGRATION.zh-CN.md) | 登录、Token、Cloud Authority 与 Workspace 的接入 |
| [Mobile Office DOCX Rust Engine](MOBILE-OFFICE-DOCX-ENGINE.zh-CN.md) | DOCX 解析/渲染引擎的 Rust 实现与接口 |
| [Mobile Office View Engines](MOBILE-OFFICE-VIEWERS.zh-CN.md) | PDF、表格、文档等查看引擎的选型与边界 |
| [Mobile ASR Plugin 架构、技术选型与实施计划](MOBILE-ASR-PLUGIN-ARCHITECTURE.zh-CN.md) | 语音识别插件架构、模型与运行时选型 |
| [Mobile ASR 实施计划与验证记录](MOBILE-ASR-IMPLEMENTATION-PLAN.zh-CN.md) | ASR 落地步骤、真机验证与问题记录 |

## 相关领域

- 资源打开与多端交接：[workspace/RESOURCE-REFERENCE-AVAILABILITY-AND-HANDOFF](../../workspace/RESOURCE-REFERENCE-AVAILABILITY-AND-HANDOFF.zh-CN.md)
- Mobile 发起 Desktop 插件任务：[plugins/MOBILE-DESKTOP-PLUGIN-INTERACTION-ROUTING](../../plugins/MOBILE-DESKTOP-PLUGIN-INTERACTION-ROUTING.zh-CN.md)
- 多端连接与中继：[devices-ctrl](../../devices-ctrl/README.md)
