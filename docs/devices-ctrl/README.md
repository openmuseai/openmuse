# 多端设备协同

设备发现、连接、中继与打开路由：同账号设备如何互见、Relay 优先与直连升级、连接延迟诊断、资源宿主定位，以及真机验收记录。产品稿见 [product/SAME-ACCOUNT-MULTI-DEVICE-PRODUCT-INTERACTION](../product/SAME-ACCOUNT-MULTI-DEVICE-PRODUCT-INTERACTION.zh-CN.md)。

## 文档

| 文档 | 内容 |
| --- | --- |
| [当前多端设备协同：架构、实现与空闲断线恢复](CURRENT-MULTI-DEVICE-SYNC.zh-CN.md) | 现状总览与恢复机制，域入口 |
| [Paired Desktop E2E Channel 与 Opaque Relay](PAIRED-DESKTOP-E2E-RELAY.zh-CN.md) | 端到端加密通道与不透明中继的协议设计 |
| [Mobile ↔ Desktop Local Workspace / DSH Transport](PAIRED-DESKTOP-LOCAL-DSH-TRANSPORT.zh-CN.md) | 局域网直连传输与 Workspace / DSH 访问 |
| [Desktop 出站附着与按设备转发](DESKTOP-OUTBOUND-RELAY.zh-CN.md) | Desktop 主动附着、按设备路由与转发策略 |
| [Relay 优先、后台直连升级与连接耗时诊断](RELAY-FIRST-DIRECT-UPGRADE-TRACE.zh-CN.md) | 连接路径选择、直连升级与耗时埋点 |
| [Mobile / Desktop 会话连接与延迟修复](MOBILE-DESKTOP-LATENCY-REPAIR.zh-CN.md) | 会话建立与延迟问题的修复记录 |
| [国内环境真机连接验收与目录负载修复](REAL-DEVICE-CN-LATENCY-VALIDATION-2026-11-11.zh-CN.md) | 国内网络真机验收与目录负载问题 |
| [资源宿主解析与统一打开：实现及验收记录](RESOURCE-OWNER-LOCATOR-IMPLEMENTATION-2026-11-13.zh-CN.md) | 资源在哪个设备打开、如何统一定位 |
| [Open File Viewer：Tree-sitter 与文内文件链接](FILE-VIEWER-TREE-SITTER-LINKS-2026-11-13.zh-CN.md) | 文件查看器的语法高亮与文内链接跳转 |
| [公网设备目录与 Desktop 转发：部署指南](DEPLOY.zh-CN.md) | 公网目录、转发服务与部署步骤 |

## 相关领域

- 账号、Presence 与配对：[identity](../identity/README.md)
- 资源 Ref / Handle：[workspace/WORKSPACE-RESOURCE-AUTHORITY-V1](../workspace/WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md)
- 验收证据：[evidence/](evidence/)
