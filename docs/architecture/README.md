# 架构与跨域合同

本域回答三个问题：系统边界怎么划、跨域用什么词汇通信、代码与依赖从哪里来。具体实现与验收放在各端或各能力域，这里只保留跨域结论和入口。

建议阅读顺序：[架构评审](ARCHITECTURE-REVIEW.zh-CN.md) → [总体架构](PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md) → [跨域合同基线](CONTRACT-BASELINE-V1.zh-CN.md) → [功能对齐矩阵](FUNCTION-PARITY-MATRIX.zh-CN.md) / [PoC 结果](POC-RESULTS.zh-CN.md)。[代码来源审计](CODE-REUSE-PROVENANCE.zh-CN.md) 是发布门禁材料，随发行版本更新。

## 文档

| 文档 | 内容 |
| --- | --- |
| [OpenMuse 架构评审结论](ARCHITECTURE-REVIEW.zh-CN.md) | Host / Plugin / DSH 三条边界与长期演进建议，Accepted 结论 |
| [OpenMuse Host / Plugin / DSH 总体架构](PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md) | 组件、协议、性能、安全、UI 与发布的总体分析，架构入口 |
| [OpenMuse 跨域合同基线 v1](CONTRACT-BASELINE-V1.zh-CN.md) | `openmuse.contract` v1 envelope、Ref/Handle 与三语言投影 |
| [OpenMuse 本地工作台功能对齐矩阵](FUNCTION-PARITY-MATRIX.zh-CN.md) | 旧工作台能力与新工程的逐项对齐状态 |
| [OpenMuse PoC 结果](POC-RESULTS.zh-CN.md) | 关键技术假设的验证结论与残留风险 |
| [代码复用与来源审计](CODE-REUSE-PROVENANCE.zh-CN.md) | 第三方与历史代码来源、许可证与发布前审计要求 |

## 相关领域

- 插件协议与 Manifest：[plugins/](../plugins/README.md)
- DSH 运行时与对话流：[dsh/](../dsh/README.md)
- 存储、Sandbox 的领域合同：[storage/](../storage/README.md)、[sandbox/](../sandbox/README.md)
