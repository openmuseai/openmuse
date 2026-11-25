# 插件平台

插件协议、包格式与分发：Manifest v2 是入口，CLI 与 Registry 描述命令和产物解析，远程 Surface 协议覆盖 Mobile 预览/控制，Easel 与社媒插件是可选业务插件范例。

## 文档

| 文档 | 内容 |
| --- | --- |
| [OpenMuse Plugin Manifest v2](PLUGIN-MANIFEST-V2.zh-CN.md) | 清单字段、权限、能力与兼容约束，协议入口 |
| [Plugin CLI Registry 与 Artifact Resolver](PLUGIN-CLI-REGISTRY-ARTIFACT-RESOLVER.zh-CN.md) | 命令注册、产物寻址、版本与校验 |
| [OpenMuse CLI 插件、底部控制台与可扩展命令协议](OPENMUSE-CLI-PLUGIN-DESIGN.zh-CN.md) | 桌面 CLI 插件、控制台 UI 与命令协议设计 |
| [OpenMuse CLI 插件实施计划与 Easel 验收](OPENMUSE-CLI-PLUGIN-IMPLEMENTATION-PLAN.zh-CN.md) | CLI 插件落地步骤与验收结果 |
| [Mobile 远程插件预览与控制协议](MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md) | remote-surface 线协议、媒体句柄与权限边界 |
| [远程 Surface 与可选社媒插件：开发计划与测试矩阵](REMOTE-SURFACE-IMPLEMENTATION-PLAN.zh-CN.md) | RS0–RS7 阶段、测试矩阵与残留门禁 |
| [Mobile 插件市场：A2UI + QuickJS 实施方案](MOBILE-A2UI-QUICKJS-PLUGIN-DELIVERY.zh-CN.md) | 移动端不装包渲染插件界面的方案与限制 |
| [Mobile 发起的 Desktop 插件任务：窗口归属与交互路由](MOBILE-DESKTOP-PLUGIN-INTERACTION-ROUTING.zh-CN.md) | 跨端任务落到哪个窗口、如何回传与接管 |
| [Easel 能力复用：DSH 驱动的可选社媒插件设计](EASEL-DSH-OPTIONAL-SOCIAL-PLUGIN-DESIGN.zh-CN.md) | 社媒插件的能力边界与 DSH 驱动方式 |

## 相关领域

- 插件宿主与 DSH 接缝：[dsh/DSH-PLUGIN-HOST-MIGRATION](../dsh/DSH-PLUGIN-HOST-MIGRATION.zh-CN.md)
- Broker、权限与审计：[identity/BROKER-DELEGATION-POLICY-AUDIT](../identity/BROKER-DELEGATION-POLICY-AUDIT.zh-CN.md)
- 插件市场产品稿：[product/BUILTIN-PLUGIN-MARKET-PRD-AND-PLAN](../product/BUILTIN-PLUGIN-MARKET-PRD-AND-PLAN.zh-CN.md)
