# 端实现

按交付端划分：Mobile、Desktop、Web 各自保留架构、实现计划、性能分析与验收矩阵。跨端共享的合同、资源与同步语义在 [architecture](../architecture/README.md)、[workspace](../workspace/README.md) 和 [storage](../storage/README.md)。

| 目录 | 范围 | 入口 |
| --- | --- | --- |
| [mobile](mobile/README.md) | Flutter Mobile、Host Shell、Cloud 接入、Office、ASR | [Mobile 架构](mobile/MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md) |
| [desktop](desktop/README.md) | Desktop 工作台、Helix、工程门禁 | [Workbench 规格](desktop/WORKBENCH-PARITY-SPEC.zh-CN.md) |
| [web](web/README.md) | Web 技术选型、开发切片、共源验收 | [Web 技术选型](web/WEB-ARCHITECTURE-TECH-SELECTION-IMPLEMENTATION-PLAN.zh-CN.md) |

共用材料：UX 设计稿见 [design](../design/)，发布执行见 [release](../release/README.zh-CN.md)，地区化配置见 [globalization](../globalization/README.md)。
