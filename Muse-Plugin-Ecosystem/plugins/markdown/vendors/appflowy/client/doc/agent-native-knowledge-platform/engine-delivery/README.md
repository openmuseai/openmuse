# 可扩展格式引擎交付方案

状态：**实施设计 v1**  
审计基线：2026-09-15 当前工作区  
范围：`vendors/helix`、`vendors/ioffice`、`vendors/open-file-viewer` 与 Muse Host、DSH、Flutter/Web Surface 的协同。

## 1. 本轮目标

本轮把三个 vendor 从“仓库里存在的引擎/源码”交付为可发现、可路由、可授权、可观测、可测试的 Muse 格式引擎：

```text
DSH / User Intent
        │
        ▼
Host Bridge control plane
Resource Provider → Route → Materialization → Engine Session → Receipt/Event
                                      │
                    ┌─────────────────┼─────────────────┐
                    ▼                 ▼                 ▼
                ioffice            Helix       open-file-viewer
              native/wasm       PTY process      sandbox web
```

三个引擎只共享资源、呈现、生命周期、权限和观测合同，不共享内部文档模型：

- ioffice 拥有 Office 解析、排版、编辑与格式导出。
- Helix 拥有模态文本编辑、终端 UI、语法和可选 LSP。
- open-file-viewer 拥有浏览器端只读预览及格式插件。
- Host 拥有资源身份、locator、权限、路由、物化和提交 authority。
- DSH 是 Intent/Context/Action 的 Consumer，不持有本地路径，也不选择未经授权的引擎。

## 2. 明确排除 Ontology Runtime

本轮**不实现**：

- Ontology Object/Link/Claim 存储和查询；
- Resource 片段到语义对象的抽取；
- ImpactSet、Action Runtime、Workflow/Automation；
- 需求变化到代码/测试的自动传播。

本轮只预埋稳定、无业务语义的接口：

- opaque `resourceRef` 和 revision；
- 可选 `anchorHint/selection` 上下文；
- `resource.changed`、`surface.selection.changed` 等事实事件；
- Adapter 可选的 `describeSelection()`；
- trace/causal ID 和扩展字段保留规则。

这些挂钩不得让引擎依赖 Ontology 包，也不得在事件中写入 Requirement、Decision 等业务类型。详见 [08-ontology-reserved-hooks.md](08-ontology-reserved-hooks.md)。

## 3. 文档地图

| 文档 | 内容 |
|---|---|
| [00-current-state-and-scope.md](00-current-state-and-scope.md) | 当前源码事实、边界、目标能力与非目标 |
| [01-host-dsh-engine-collaboration.md](01-host-dsh-engine-collaboration.md) | Host、DSH、Surface、Provider、Adapter 的控制流与状态机 |
| [02-engine-adapter-contracts.md](02-engine-adapter-contracts.md) | 公共 Manifest、probe、session、materialization、commit 和事件合同 |
| [03-helix-implementation-plan.md](03-helix-implementation-plan.md) | Helix 每阶段设计、开发任务、测试矩阵和退出门 |
| [04-ioffice-implementation-plan.md](04-ioffice-implementation-plan.md) | ioffice Word 及 Excel/Slides/PDF 的条件式实施计划 |
| [05-open-file-viewer-implementation-plan.md](05-open-file-viewer-implementation-plan.md) | Viewer 隔离、资源读取、格式认证和多端计划 |
| [06-integrated-roadmap.md](06-integrated-roadmap.md) | 三引擎依赖图、PR/工作包拆分、灰度、回滚和责任边界 |
| [07-cross-engine-test-release.md](07-cross-engine-test-release.md) | 公共 TCK、E2E、性能、安全、打包和发布矩阵 |
| [08-ontology-reserved-hooks.md](08-ontology-reserved-hooks.md) | Ontology 延后期间必须保留和禁止提前实现的接口 |
| [phases/F0/](phases/F0/README.md) | F0 当前实施包：设计、开发、测试矩阵、fixtures、发布与实测结果 |
| [phases/F0.2-host-resource/](phases/F0.2-host-resource/README.md) | F0.2 Host Resource Core 实施、25 项自动测试与 Gate 证据 |
| [phases/F0.3-dsh-seam/](phases/F0.3-dsh-seam/README.md) | F0.3 DSH Service/Provider/UI+Agent Consumers、20 项自动测试与 Gate 证据 |

既有总体方案仍为上位约束：

- [统一资源协议](../03-universal-resource-protocol.md)
- [Surface 路由与引擎适配](../04-surface-routing-and-engine-adapters.md)
- [产品 PRD](../01-product-prd.md)

ioffice Word 既有阶段规格是当前事实依据，而不是被本目录替代：

- `Muse-Clients/local/docs/office/word/`
- `Muse-Clients/local/docs/office/word/E2E-ACCEPTANCE.zh-CN.md`

## 4. 交付原则

1. **先公共链路，后引擎功能。** 没有 Resource/Presentation/Session TCK，不并行制造三个私有打开协议。
2. **能力必须运行时可证。** Manifest 只声明候选，`probe` 决定当前设备和产物是否真正可用。
3. **控制面统一，数据面按引擎选择。** JSON Host Bridge 不承载 docx、大文件或 PTY 重绘字节。
4. **只读先于写入。** 先交付可打开、可关闭、可回退，再开放保存；没有真实导出/提交能力就显示只读。
5. **Vendor 保持 Muse 无知。** Muse 适配代码进入 Muse-Clients；只有通用能力修复才回馈 vendor。
6. **每阶段可独立回滚。** Adapter 可被 feature flag 禁用，资源仍可降级到 Viewer、下载或系统应用。
7. **每阶段都有设计、开发清单、自动测试、手工验收和 no-go 条件。**

## 5. 全局交付门

| Gate | 名称 | 通过条件 |
|---|---|---|
| G0 | 合同冻结 | TS/Rust/Dart fixtures 一致；错误、取消、revision 和 receipt 语义冻结 |
| G1 | 公共打开链 | DSH 三类入口统一进入 Host route；未知格式与权限失败有明确回执 |
| G2 | 引擎只读可用 | 三引擎各至少一个受支持场景通过 open/ready/focus/close/fallback |
| G3 | 安全写入 | Helix/iOffice 仅在真实 commit 能力和冲突测试通过后开放 edit |
| G4 | 多端和打包 | 目标端产物、签名、sandbox、内存/性能、崩溃恢复通过 |
| G5 | 默认启用 | 灰度 SLO 达标，旧路径可回滚，无 P0 安全与数据损坏缺陷 |

## 6. 推荐实施顺序

1. 公共合同、Host registry、DSH OpenResource Consumer、Flutter Surface Orchestrator。
2. open-file-viewer Wave 1，只读链最容易验证公共协议和 fallback。
3. ioffice Word 统一只读打开，复用当前 Flutter/FFI 实现。
4. Helix PTY spike 与只读/工作副本打开。
5. ioffice `toDocx` 硬门通过后开放 Word Save。
6. Helix commit/conflict 完成后开放 edit。
7. Viewer 复杂格式、Web/Mobile、ioffice 多平台。
8. Excel/Slides/PDF 只有真实 vendor 引擎通过 admission gate 后立项。

## 7. 完成定义

一个引擎只能在产品中标记“可用”，当且仅当：

- manifest、runtime probe 和打包产物一致；
- Resource/Surface TCK 通过；
- 对应端 E2E、性能和安全门通过；
- error/fallback/取消/崩溃有用户可理解行为；
- 写入引擎具有真实序列化、revision、冲突和恢复证据；
- DSH 入口产生 model-visible、可重放的安全结果事件；
- 没有依赖尚未实现的 Ontology Runtime。
