# 全球化与运行时配置

多端地区化整改：地区选择、语言包下发、动态参数与开关、签名配置发布，以及不发版即可调整的运行时能力。执行入口是整改计划，平台级详细设计记录契约与实现。

## 文档

| 文档 | 内容 |
| --- | --- |
| [多端全球化整改范围与实施计划](GLOBALIZATION-MULTIPLATFORM-REMEDIATION-PLAN.zh-CN.md) | 范围、优先级与端到端执行顺序，域入口 |
| [多端 App 地区、动态参数与语言整改](REGIONAL-APPS-RUNTIME-REMEDIATION.zh-CN.md) | 地区/语言判定、开关下发与代码入口清单 |
| [OpenMuse 国际化与动态配置平台详细设计](GLOBALIZATION-RUNTIME-CONFIG-PLATFORM.zh-CN.md) | 配置控制面、分发面、SDK 与安全模型 |

## 相关领域

- 配置实现入口：`packages/openmuse_runtime_config/`、`packages/openmuse_i18n/`、`services/config-control/`
- 上架渠道的地区要求：[release/pipeline/channels](../release/pipeline/channels/)
- Mobile 地区化交付：[product/MOBILE-CLOUD-SANDBOX-DELIVERY-ROADMAP](../product/MOBILE-CLOUD-SANDBOX-DELIVERY-ROADMAP.zh-CN.md)
