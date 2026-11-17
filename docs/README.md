# OpenMuse 文档中心

本目录按**领域**组织：每个一级目录对应一个相对稳定的产品/技术域，域内文档保持扁平，不再按“设计 / 计划 / 验收”二次切分。跨域语义放 `architecture/`，实现与验收跟随所属领域。

## 目录树

```text
docs/
├── README.md                 本索引：领域地图、命名约定、路径迁移
├── architecture/             总体架构、跨域合同、来源审计、功能对齐
├── product/                  产品 PRD、首发范围、交付排期与多端协同产品稿
├── platforms/
│   ├── mobile/               Flutter Mobile 架构、Cloud 接入、Office / ASR
│   ├── desktop/              Desktop 工作台、Helix 编辑器、工程门禁
│   └── web/                  Web 技术选型、共源工作台与验收矩阵
├── dsh/                      DSH 运行时、对话流、Provider、插件宿主与执行平面
├── plugins/                  插件平台、Manifest、插件市场、CLI、远程 Surface
├── identity/                 账号、认证、Presence、设备配对、Broker 委派审计
├── devices-ctrl/             多端设备协同、中继部署、连接延迟与真机验收
├── workspace/                Workspace / Resource Authority、同步、草稿与版本
├── storage/                  S3 / MinIO / RustFS、存储合同、TCK 与可移植
├── sandbox/                  Sandbox、Host 服务、Lease 与本地运行时
├── globalization/            多端地区化、语言、动态参数与运行时配置
├── operations/               macOS / Windows CI 与增量构建
├── release/                  上架流水线、渠道规范、各次发行归档
│   └── gates/                发行门禁（SBOM / 闭包、iOS 未签名构建）
├── design/                   UX 设计稿、交互原型与人工审核包
├── meterails/                对外图文 / 视频内容与生成脚本
├── media/                    README 与演示用截图、视频
├── evidence/                 跨域验收证据（按任务与日期归档）
└── qualification/            存储资质与验收记录
```

## 领域索引

| 领域 | 解决什么问题 | 入口 |
| --- | --- | --- |
| [architecture](architecture/README.md) | 系统边界、跨域合同、PoC 结论、代码来源 | [总体架构](architecture/PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md) |
| [product](product/README.md) | 产品定义、首发范围、版本更新与交付顺序 | [Mobile PRD](product/MOBILE-PRODUCT-PRD.zh-CN.md) |
| [platforms/mobile](platforms/mobile/README.md) | Mobile 架构、Cloud Workspace、Office 与 ASR | [Mobile 架构](platforms/mobile/MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md) |
| [platforms/desktop](platforms/desktop/README.md) | Desktop 工作台、Helix、工程门禁 | [Workbench 规格](platforms/desktop/WORKBENCH-PARITY-SPEC.zh-CN.md) |
| [platforms/web](platforms/web/README.md) | Web 技术选型、开发切片与共源验收 | [Web 技术选型](platforms/web/WEB-ARCHITECTURE-TECH-SELECTION-IMPLEMENTATION-PLAN.zh-CN.md) |
| [dsh](dsh/README.md) | DSH 对话流、Provider、插件宿主与执行平面 | [DSH 对话流选型](dsh/DSH-CONVERSATION-SURFACE-SELECTION.zh-CN.md) |
| [plugins](plugins/README.md) | 插件协议、Manifest、市场、CLI 与远程 Surface | [Manifest v2](plugins/PLUGIN-MANIFEST-V2.zh-CN.md) |
| [identity](identity/README.md) | 账号、鉴权、设备 Presence、配对与委派审计 | [账号与配对](identity/ACCOUNT-DEVICE-PRESENCE-PAIRING.zh-CN.md) |
| [devices-ctrl](devices-ctrl/README.md) | 多端协同、Relay、延迟修复与真机验收 | [当前多端协同](devices-ctrl/CURRENT-MULTI-DEVICE-SYNC.zh-CN.md) |
| [workspace](workspace/README.md) | 资源授权、同步、草稿恢复与云元数据 | [Resource Authority](workspace/WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md) |
| [storage](storage/README.md) | 对象存储选型、安全、TCK 与迁移 | [S3 技术选型](storage/S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md) |
| [sandbox](sandbox/README.md) | Sandbox 服务、Lease、安全与本地运行时 | [Host Sandbox](sandbox/HOST-SANDBOX-SERVICE-LOCAL-RUNTIME.zh-CN.md) |
| [globalization](globalization/README.md) | 地区、语言与运行时配置整改 | [全球化整改](globalization/GLOBALIZATION-MULTIPLATFORM-REMEDIATION-PLAN.zh-CN.md) |
| [operations](operations/README.md) | CI 与构建流水线 | [macOS CI](operations/macos-ci.md) |
| [release](release/README.zh-CN.md) | 上架流水线、渠道、发行归档与门禁 | [发布与上架](release/README.zh-CN.md) |
| [design](design/) | UX 设计、原型与审核包 | [资源中心](design/mobile-resource-center/README.zh-CN.md) |
| [meterails](meterails/) | 对外内容制作 | [能力对比报告](meterails/OPENMUSE-VS-DSH-CAPABILITY-REPORT.zh-CN.md) |
| [media](media/) | 截图与演示视频 | — |
| [evidence](evidence/) | 跨域验收证据 | — |
| [qualification](qualification/) | 资质验收记录 | [ST1 验收](qualification/storage/ST1-ACCEPTANCE.zh-CN.md) |

## 命名与状态约定

- 文件名：`UPPER-KEBAB-TOPIC.zh-CN.md`；带日期的执行记录追加 `-YYYY-MM-DD`（如 `REAL-DEVICE-CN-LATENCY-VALIDATION-2026-11-11.zh-CN.md`）。
- 每篇文档开头保留 `状态：`（Proposal / Accepted / Implemented / Superseded）与 `日期：`；已 Accepted 的文档不静默改写结论，新增结论另起文档或在文末追加“变更记录”。
- 新增文档：先确定唯一所属领域 → 放入对应目录 → 在该目录 `README.md` 增加一行。跨域内容在 `architecture/` 保留入口链接，不复制正文。
- 验收证据跟随所属任务（`devices-ctrl/evidence/`、`design/*/evidence/`、`release/runs/*/evidence/`）；无法归属单一领域的放 `evidence/`。

## 路径迁移（2026-11）

原 `docs/` 根目录的 72 篇文档已下沉到上述领域目录；`design/`、`release/`、`devices-ctrl/`、`meterails/`、`media/`、`evidence/`、`qualification/` 目录保持原位，避免破坏 CI、`config/release/pipeline.json` 与脚本中的既有路径。设备协同与发行门禁相关文档分别归入 `devices-ctrl/` 和新增的 `release/gates/`。仓库内的相对链接、根目录 README 与代码注释已同步更新。

旧路径的完整对照如下（新文档请直接使用新路径）：

<details>
<summary>旧路径 → 新路径（72 项）</summary>

| 旧路径 | 新路径 |
| --- | --- |
| `docs/ARCHITECTURE-REVIEW.zh-CN.md` | `docs/architecture/ARCHITECTURE-REVIEW.zh-CN.md` |
| `docs/CODE-REUSE-PROVENANCE.zh-CN.md` | `docs/architecture/CODE-REUSE-PROVENANCE.zh-CN.md` |
| `docs/CONTRACT-BASELINE-V1.zh-CN.md` | `docs/architecture/CONTRACT-BASELINE-V1.zh-CN.md` |
| `docs/FUNCTION-PARITY-MATRIX.zh-CN.md` | `docs/architecture/FUNCTION-PARITY-MATRIX.zh-CN.md` |
| `docs/PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md` | `docs/architecture/PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md` |
| `docs/POC-RESULTS.zh-CN.md` | `docs/architecture/POC-RESULTS.zh-CN.md` |
| `docs/APP-UPDATE-PRD-AND-DELIVERY.zh-CN.md` | `docs/product/APP-UPDATE-PRD-AND-DELIVERY.zh-CN.md` |
| `docs/BUILTIN-PLUGIN-MARKET-PRD-AND-PLAN.zh-CN.md` | `docs/product/BUILTIN-PLUGIN-MARKET-PRD-AND-PLAN.zh-CN.md` |
| `docs/MOBILE-CLOUD-SANDBOX-DELIVERY-ROADMAP.zh-CN.md` | `docs/product/MOBILE-CLOUD-SANDBOX-DELIVERY-ROADMAP.zh-CN.md` |
| `docs/MOBILE-PRODUCT-PRD.zh-CN.md` | `docs/product/MOBILE-PRODUCT-PRD.zh-CN.md` |
| `docs/OPENMUSE-LOCAL-PLAN.zh-CN.md` | `docs/product/OPENMUSE-LOCAL-PLAN.zh-CN.md` |
| `docs/SAME-ACCOUNT-MULTI-DEVICE-PRODUCT-INTERACTION.zh-CN.md` | `docs/product/SAME-ACCOUNT-MULTI-DEVICE-PRODUCT-INTERACTION.zh-CN.md` |
| `docs/MOBILE-APP-ROOT-HOST-SHELL.zh-CN.md` | `docs/platforms/mobile/MOBILE-APP-ROOT-HOST-SHELL.zh-CN.md` |
| `docs/MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md` | `docs/platforms/mobile/MOBILE-ARCHITECTURE-IMPLEMENTATION-PLAN.zh-CN.md` |
| `docs/MOBILE-ASR-IMPLEMENTATION-PLAN.zh-CN.md` | `docs/platforms/mobile/MOBILE-ASR-IMPLEMENTATION-PLAN.zh-CN.md` |
| `docs/MOBILE-ASR-PLUGIN-ARCHITECTURE.zh-CN.md` | `docs/platforms/mobile/MOBILE-ASR-PLUGIN-ARCHITECTURE.zh-CN.md` |
| `docs/MOBILE-CLOUD-SERVICE-INTEGRATION.zh-CN.md` | `docs/platforms/mobile/MOBILE-CLOUD-SERVICE-INTEGRATION.zh-CN.md` |
| `docs/MOBILE-OFFICE-DOCX-ENGINE.zh-CN.md` | `docs/platforms/mobile/MOBILE-OFFICE-DOCX-ENGINE.zh-CN.md` |
| `docs/MOBILE-OFFICE-VIEWERS.zh-CN.md` | `docs/platforms/mobile/MOBILE-OFFICE-VIEWERS.zh-CN.md` |
| `docs/OFFICE-DISPATCHER-WORKER-SUPERVISOR.zh-CN.md` | `docs/sandbox/OFFICE-DISPATCHER-WORKER-SUPERVISOR.zh-CN.md` |
| `docs/DESKTOP-GATE-RESULTS.zh-CN.md` | `docs/platforms/desktop/DESKTOP-GATE-RESULTS.zh-CN.md` |
| `docs/HELIX-NONMODAL-EDITOR-DESIGN.zh-CN.md` | `docs/platforms/desktop/HELIX-NONMODAL-EDITOR-DESIGN.zh-CN.md` |
| `docs/HELIX-WINDOWS-OPEN-PERF.zh-CN.md` | `docs/platforms/desktop/HELIX-WINDOWS-OPEN-PERF.zh-CN.md` |
| `docs/WORKBENCH-FREE-LAYOUT.zh-CN.md` | `docs/platforms/desktop/WORKBENCH-FREE-LAYOUT.zh-CN.md` |
| `docs/WORKBENCH-PARITY-SPEC.zh-CN.md` | `docs/platforms/desktop/WORKBENCH-PARITY-SPEC.zh-CN.md` |
| `docs/WEB-ARCHITECTURE-TECH-SELECTION-IMPLEMENTATION-PLAN.zh-CN.md` | `docs/platforms/web/WEB-ARCHITECTURE-TECH-SELECTION-IMPLEMENTATION-PLAN.zh-CN.md` |
| `docs/WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md` | `docs/platforms/web/WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md` |
| `docs/WEB-DEVELOPMENT-PLAN.zh-CN.md` | `docs/platforms/web/WEB-DEVELOPMENT-PLAN.zh-CN.md` |
| `docs/CLOUD-EXECUTION-RUNTIME-DSH-PROVIDERS.zh-CN.md` | `docs/dsh/CLOUD-EXECUTION-RUNTIME-DSH-PROVIDERS.zh-CN.md` |
| `docs/DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md` | `docs/dsh/DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md` |
| `docs/DSH-CONVERSATION-SURFACE-SELECTION.zh-CN.md` | `docs/dsh/DSH-CONVERSATION-SURFACE-SELECTION.zh-CN.md` |
| `docs/DSH-NATIVE-FLUTTER-CONVERSATION-ARCHITECTURE-PLAN.zh-CN.md` | `docs/dsh/DSH-NATIVE-FLUTTER-CONVERSATION-ARCHITECTURE-PLAN.zh-CN.md` |
| `docs/DSH-PLUGIN-HOST-MIGRATION.zh-CN.md` | `docs/dsh/DSH-PLUGIN-HOST-MIGRATION.zh-CN.md` |
| `docs/DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY.zh-CN.md` | `docs/dsh/DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY.zh-CN.md` |
| `docs/WORKSPACE-VERSION-DSH-MIGRATION.zh-CN.md` | `docs/dsh/WORKSPACE-VERSION-DSH-MIGRATION.zh-CN.md` |
| `docs/EASEL-DSH-OPTIONAL-SOCIAL-PLUGIN-DESIGN.zh-CN.md` | `docs/plugins/EASEL-DSH-OPTIONAL-SOCIAL-PLUGIN-DESIGN.zh-CN.md` |
| `docs/MOBILE-A2UI-QUICKJS-PLUGIN-DELIVERY.zh-CN.md` | `docs/plugins/MOBILE-A2UI-QUICKJS-PLUGIN-DELIVERY.zh-CN.md` |
| `docs/MOBILE-DESKTOP-PLUGIN-INTERACTION-ROUTING.zh-CN.md` | `docs/plugins/MOBILE-DESKTOP-PLUGIN-INTERACTION-ROUTING.zh-CN.md` |
| `docs/MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md` | `docs/plugins/MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md` |
| `docs/OPENMUSE-CLI-PLUGIN-DESIGN.zh-CN.md` | `docs/plugins/OPENMUSE-CLI-PLUGIN-DESIGN.zh-CN.md` |
| `docs/OPENMUSE-CLI-PLUGIN-IMPLEMENTATION-PLAN.zh-CN.md` | `docs/plugins/OPENMUSE-CLI-PLUGIN-IMPLEMENTATION-PLAN.zh-CN.md` |
| `docs/PLUGIN-CLI-REGISTRY-ARTIFACT-RESOLVER.zh-CN.md` | `docs/plugins/PLUGIN-CLI-REGISTRY-ARTIFACT-RESOLVER.zh-CN.md` |
| `docs/PLUGIN-MANIFEST-V2.zh-CN.md` | `docs/plugins/PLUGIN-MANIFEST-V2.zh-CN.md` |
| `docs/REMOTE-SURFACE-IMPLEMENTATION-PLAN.zh-CN.md` | `docs/plugins/REMOTE-SURFACE-IMPLEMENTATION-PLAN.zh-CN.md` |
| `docs/ACCOUNT-DEVICE-PRESENCE-PAIRING.zh-CN.md` | `docs/identity/ACCOUNT-DEVICE-PRESENCE-PAIRING.zh-CN.md` |
| `docs/AUTH-CROSS-DEVICE-TEST-MATRIX.zh-CN.md` | `docs/identity/AUTH-CROSS-DEVICE-TEST-MATRIX.zh-CN.md` |
| `docs/AUTH-GOTRUE-PLUGIN-MIGRATION.zh-CN.md` | `docs/identity/AUTH-GOTRUE-PLUGIN-MIGRATION.zh-CN.md` |
| `docs/AUTH-RELEASE-GATE.zh-CN.md` | `docs/identity/AUTH-RELEASE-GATE.zh-CN.md` |
| `docs/BROKER-DELEGATION-POLICY-AUDIT.zh-CN.md` | `docs/identity/BROKER-DELEGATION-POLICY-AUDIT.zh-CN.md` |
| `docs/DEVICE-PRESENCE-REALTIME-ARCHITECTURE.zh-CN.md` | `docs/identity/DEVICE-PRESENCE-REALTIME-ARCHITECTURE.zh-CN.md` |
| `docs/CLOUD-WORKSPACE-METADATA-CAS-OUTBOX.zh-CN.md` | `docs/workspace/CLOUD-WORKSPACE-METADATA-CAS-OUTBOX.zh-CN.md` |
| `docs/DRAFT-CHECKPOINT-QUIESCENCE-RECOVERY.zh-CN.md` | `docs/workspace/DRAFT-CHECKPOINT-QUIESCENCE-RECOVERY.zh-CN.md` |
| `docs/RESOURCE-REFERENCE-AVAILABILITY-AND-HANDOFF.zh-CN.md` | `docs/workspace/RESOURCE-REFERENCE-AVAILABILITY-AND-HANDOFF.zh-CN.md` |
| `docs/WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md` | `docs/workspace/WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md` |
| `docs/WORKSPACE-SYNC-PLUGIN.zh-CN.md` | `docs/workspace/WORKSPACE-SYNC-PLUGIN.zh-CN.md` |
| `docs/BYOS-PROVIDER-MIGRATION-PORTABILITY.zh-CN.md` | `docs/storage/BYOS-PROVIDER-MIGRATION-PORTABILITY.zh-CN.md` |
| `docs/RUSTFS-QUALIFICATION-MANAGED-ROLLOUT.zh-CN.md` | `docs/storage/RUSTFS-QUALIFICATION-MANAGED-ROLLOUT.zh-CN.md` |
| `docs/S3-ADAPTER-CREDENTIAL-ENDPOINT-SECURITY.zh-CN.md` | `docs/storage/S3-ADAPTER-CREDENTIAL-ENDPOINT-SECURITY.zh-CN.md` |
| `docs/S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md` | `docs/storage/S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md` |
| `docs/STORAGE-CONTRACT-S3-PROFILE-TCK.zh-CN.md` | `docs/storage/STORAGE-CONTRACT-S3-PROFILE-TCK.zh-CN.md` |
| `docs/HOST-SANDBOX-SERVICE-LOCAL-RUNTIME.zh-CN.md` | `docs/sandbox/HOST-SANDBOX-SERVICE-LOCAL-RUNTIME.zh-CN.md` |
| `docs/SANDBOX-PRODUCTION-SECURITY-GATE.zh-CN.md` | `docs/sandbox/SANDBOX-PRODUCTION-SECURITY-GATE.zh-CN.md` |
| `docs/GLOBALIZATION-MULTIPLATFORM-REMEDIATION-PLAN.zh-CN.md` | `docs/globalization/GLOBALIZATION-MULTIPLATFORM-REMEDIATION-PLAN.zh-CN.md` |
| `docs/GLOBALIZATION-RUNTIME-CONFIG-PLATFORM.zh-CN.md` | `docs/globalization/GLOBALIZATION-RUNTIME-CONFIG-PLATFORM.zh-CN.md` |
| `docs/REGIONAL-APPS-RUNTIME-REMEDIATION.zh-CN.md` | `docs/globalization/REGIONAL-APPS-RUNTIME-REMEDIATION.zh-CN.md` |
| `docs/macos-ci.md` | `docs/operations/macos-ci.md` |
| `docs/windows-ci.md` | `docs/operations/windows-ci.md` |
| `docs/windows-incremental-build.md` | `docs/operations/windows-incremental-build.md` |
| `docs/DISTRIBUTION-LOCK-SBOM-CLOSURE-GATE.zh-CN.md` | `docs/release/gates/DISTRIBUTION-LOCK-SBOM-CLOSURE-GATE.zh-CN.md` |
| `docs/IOS-UNSIGNED-BUILD-GATE.zh-CN.md` | `docs/release/gates/IOS-UNSIGNED-BUILD-GATE.zh-CN.md` |
| `docs/PAIRED-DESKTOP-E2E-RELAY.zh-CN.md` | `docs/devices-ctrl/PAIRED-DESKTOP-E2E-RELAY.zh-CN.md` |
| `docs/PAIRED-DESKTOP-LOCAL-DSH-TRANSPORT.zh-CN.md` | `docs/devices-ctrl/PAIRED-DESKTOP-LOCAL-DSH-TRANSPORT.zh-CN.md` |

</details>
