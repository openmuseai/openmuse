# 实施路线、工作包与验收门

状态：**建议路线 v1**；按依赖和风险分阶段，不承诺日历日期。每阶段在进入排期前由团队换算人力与工程周。

## 1. 实施策略

路线遵循“先收敛协议，再跑通垂直切片，再扩格式，最后扩大语义自动化”：

```text
0 基线与契约
      ↓
1 统一资源垂直切片（Markdown/Word/DSH）
      ↓
2 Surface 与引擎生态（Viewer/Helix/多端）
      ↓
3 Ontology MVP（Object/Link/Anchor/Claim）
      ↓
4 Impact + Action + Workflow
      ↓
5 规模化、开放生态、CAD/3D
```

不以重写现有 AppFlowy、Host Bridge、Document Contract 或 vendor 为前提。所有新路径都保留 feature flag、双读/影子验证和回退入口。

## 2. 建议代码边界

下列是目标包边界，名称在 Phase 0 RFC 中冻结：

以下路径相对于 `openmuse-io/` 工作区根目录：

```text
Muse-Clients/middlewares/dsh/core/
├─ protocol/host-bridge/              # [已实现] 元协议、policy、event、limits
├─ contract-resource/                 # [拟新增] Resource/Materialization 合同
├─ contract-presentation/             # [拟新增] SurfaceIntent/route receipt
├─ contract-ontology/                 # [拟新增] Object/Link/Claim/Action 合同
├─ resource-runtime/                  # [拟新增] Provider registry 与 TCK
└─ ontology-runtime/                  # [拟新增] store/projection/impact/action

Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/
├─ muse_ui_surface_runtime/           # [已实现] Surface/Facet kernel，扩展适配器
├─ muse_plugin_facets/                # [已实现] context/change/intent 合同
├─ muse_resource_contract/            # [拟新增] Dart 生成/手写薄类型层
├─ muse_surface_orchestrator/          # [拟新增] 路由、偏好、降级、receipt
└─ muse_knowledge_lens/                # [拟新增] Object/Impact/Approval UI
```

所有跨语言合同从同一 JSON Schema/fixture 生成或验证。TS、Rust、Dart 不分别发明字段和 revision 语义。

## 3. Phase 0：基线、ADR 与合同卫生

目标：在新增功能前消除会让双端协议分叉的高风险不一致。

### 工作包

- 冻结本文 D1–D12，并为 Resource/Surface/Ontology 划分限界上下文。
- 对 `@muse/host-bridge`、Plugin Graph v2、Facet、Document Contract 建立依赖图和 owner。
- 统一 revision token：合同若要求 `sha256:*`，测试/内存 Provider 不再返回裸整数；或经 ADR 改为显式 union，不能静默混用。
- 为 Host Bridge 加入 Resource/Presentation 计划所需的 schema fixtures、limits 和 error taxonomy。
- 对 DSH 打开链做 spike：默认点击经过 ChatView `openFile`，不是运行时替换 `SessionControllerInternals.openPath`。设计正式 Service Definition/Provider/Consumer seam。
- 对 Desktop Flutter ↔ DSH 建立 Native Capability Broker 方案；对 Web/Mobile 将现有 parent bridge 业务消息映射到中立 Host Bridge envelope。
- 建立匿名 telemetry 基线：当前打开成功率、格式、端、fallback、耗时、失败码。

### 退出条件 G0

- 三语言合同 golden fixture 全部通过。
- 有明确 ADR 说明 DSH seam、Desktop transport、revision 与 ownership。
- 没有把任意本地路径暴露给 DSH/Agent 的新接口。
- 可对旧路径进行基线回归，feature flag 默认关闭。

## 4. Phase 1：统一资源垂直切片

目标：Markdown、Word 和 DSH 交付物首次走同一 `resourceRef → resolve → route → materialize → surface` 链。

### 工作包

1. 实现 `muse.resource@1`：describe、capabilities、snapshot、materialize、commit/propose、subscribe。
2. 在 Rust Host 中建立 Resource Provider Registry，将 AppFlowy ViewReference、workspace file 与 Word view 适配为 Provider。
3. 实现 `muse.presentation@2` 与 Surface Orchestrator；先接 Markdown 和现有 Word Surface Adapter。
4. DSH 新增正式 `OpenResource` consumer；ProducedFiles、inline mention、card preview 统一请求 Host，保留旧 Sidebar 为 fallback。
5. 生成 route decision/receipt，接入现有 Surface lease/focus/disposal。
6. 把原始 path 输入留在 Host 边界：兼容层先注册为短期 resourceRef，再调用新协议。
7. 建立 Resource/Adapter TCK、端到端 golden path 与错误注入。

### 退出条件 G1

- 同一个 Word/Markdown resourceRef 从交付物、内联引用和最近列表打开结果一致。
- 默认路由不依赖扩展名 switch；格式支持来自注册的 Format/Surface Provider。
- 权限撤销、revision conflict、Surface 崩溃和 materialization 过期均有可验证行为。
- 旧 DSH Sidebar fallback 可通过配置启用，且新路径指标达到约定阈值后才默认开启。

## 5. Phase 2：Surface 与引擎生态

目标：证明“格式无特权”和 Adapter 扩展能力。

本阶段的可执行拆分、Host/DSH 协同、各引擎阶段门和测试矩阵以 [engine-delivery/README.md](engine-delivery/README.md) 为准；其中 Ontology Runtime 明确不属于引擎交付依赖。

### 工作包

- 将 `open-file-viewer` 包装为隔离 Web Surface Adapter，先支持 PDF/图片/常见只读格式。
- 接入 Helix PTY Surface：Host 受控 materialization、焦点/resize/lifecycle、保存 proposal；不把 TUI 协议泄露给核心。
- 完成 ioffice Word 真实能力探测；Excel/Slides/PDF 在 vendor 实现存在前只发布 honest manifest，不声明 `engineBound`。
- 实现用户/工作区打开偏好和 route explain UI。
- 为 Anchor Provider 建立 Markdown、Word、Code、PDF 首批 selector。
- 完成 Desktop/Web/Mobile placement matrix 和“发送到桌面继续”机制。
- 发布第三方 Adapter SDK 最小版与认证 TCK，但暂不开放高风险写入。

### 退出条件 G2

- 未修改核心协议即可增加一个示例新格式 Adapter。
- Viewer 隔离、Helix PTY、ioffice 生命周期和 crash recovery 通过安全/稳定性测试。
- 不支持编辑的端/引擎明确降级为 annotate/view/metadata，而不伪报成功。
- route decision 可解释且选择稳定，无 adapter 抢占抖动。

## 6. Phase 3：Ontology MVP

目标：让需求、代码、测试与其载体成为可查询、可溯源的工作图谱。

### 工作包

- 冻结首批 ObjectType、Interface、LinkType、Anchor、Claim schema 和治理流程。
- 建立事件日志、关系型投影、邻接查询与 workspace/字段权限过滤。
- 实现 Artifact/ArtifactRevision/Anchor，并接入 Phase 2 Anchor Provider。
- 提供 Object View、Knowledge Lens、搜索与 Claims Review Queue。
- Agent Semantic Extractor 仅在选区或显式导入中产生 proposed Claims；上线前离线评估。
- 建立 Requirement ↔ CodeSymbol ↔ TestCase 手工/半自动关联路径。
- 实现 orphan/ambiguous Anchor 检测和修复 UX。

### 退出条件 G3

- 可从 Requirement 查询到实现/测试并回到具体证据，权限过滤端到端有效。
- Agent 提议与人工确认事实在 API、UI、查询中清楚区分。
- schema、投影和索引可以重建；真源删除/权限撤销能正确传播。
- 不存在无界遍历、默认 `relatedTo` 或万能 `KnowledgePoint` 上线模型。

## 7. Phase 4：Impact、Action 与 Workflow

目标：交付用户描述的“产品变更影响代码/开发文档/测试”的完整闭环。

### 工作包

- 实现语义 ChangeSet 和确定性 Impact Engine；先只支持显式关系与小规模规则。
- 将 `muse.document@2` proposal/apply 适配为通用 Action effect，并接入代码/Test Provider 的安全动作。
- 实现 approval policy、submission criteria、幂等、saga receipt、补偿和人工恢复队列。
- 上线 Impact Inbox、ChangeSet review 与 action cards。
- Workflow Runtime 支持 trigger、condition、debounce、approval、wait、retry、budget 和 dead-letter。
- 首个模板：`requirement-change/v1`；先影子运行，只生成影响报告，再逐步允许低风险提议。
- 建立误报/漏报、接受率、冲突率、部分失败与恢复时间评估。

### 退出条件 G4

- PRD 验收标准变化可生成可解释 ImpactSet，并由各责任人独立审批。
- 未批准、过期批准、revision 变化和权限撤销无法执行写入。
- Workflow 重放、循环、fan-out、Provider 超时和部分失败均通过演练。
- 自动写入范围经安全/产品评审显式批准；默认仍是 propose-first。

## 8. Phase 5：规模化与开放生态

目标：在质量和治理数据证明可行后扩展业务域、格式与生态。

- CAD/3D 示例 Provider/Adapter/Anchor，验证 entity-level 知识关联。
- 更多业务对象和 Action，由 schema council 管理而非中央团队手工开发全部类型。
- 外部 SaaS/数据源 federation、数据驻留与跨 workspace 分享策略。
- 高规模图查询、索引和事件吞吐基准；依据证据决定是否增加图数据库/流处理。
- 插件签名、市场准入、权限 diff、远程 kill switch 与兼容认证。
- 自动化模板库、仿真/影子模式、组织级指标和合规导出。

退出条件不是“格式数量”，而是第三方可以只依赖公开合同和 TCK 安全接入，且平台 SLO/安全指标不因扩展下降。

## 9. 迁移映射

| 当前实现 | 目标 | 迁移方式 |
|---|---|---|
| DSH ChatView `openFile(path)` → Sidebar | `presentation.request(resourceRef)` | consumer seam + feature flag；失败回旧 Sidebar |
| `/api/present.open` 原生打开 | Host Presentation Provider 的 `system-open` | 保留其 session/path authority 校验，改为内部 Adapter |
| `muse.presentation-intent/v1` | `muse.presentation@2` | v1 adapter；补 request/decision/receipt/lifecycle |
| `muse.context-contribution/v1` metadata | 有类型 Context Bundle | 保留 UI context；Ontology 数据走新合同 |
| `muse.domain-change/v1` | ResourceEvent + semantic ChangeSet | v1 事件适配，避免改变现有插件 |
| `muse.document@2` | domain-specific edit contract | 继续使用；由 Action effect 包装，不被 generic patch 取代 |
| Office `engineBound` 占位 | capability probe 结果 | manifest 声明候选，runtime probe 决定可用性 |
| parent bridge 自定义消息 | Host Bridge transport adapter | 逐消息映射、双栈观测、再移除旧业务 switch |

## 10. 测试矩阵

### 合同测试

- JSON Schema 正反例、未知字段/版本、大小上限、错误码。
- TS/Rust/Dart round-trip 与 digest 一致。
- Resource/Anchor/Action 引用不可伪造、不可跨 workspace 重放。

### Adapter TCK

- probe/describe/open/ready/focus/close/dispose。
- materialization TTL、取消、崩溃、资源删除、权限撤销。
- Anchor 定位/重定位/ambiguous/orphaned。
- edit/save/conflict/readonly/fallback。

### 端到端

- Desktop 本地 AppFlowy、workspace file、Word/Helix/Viewer。
- Web/Mobile 远端资源、断线重连、SSE、placement 降级。
- DSH ProducedFile/mention/card 三个入口的一致性。
- Requirement 变更到代码/测试 proposal、审批、冲突、部分失败。

### 非功能

- 大文件、并发打开、Surface 泄漏、长会话、冷启动、低内存。
- prompt injection、renderer escape、路径穿越、zip bomb、approval replay。
- 图查询预算、自动化风暴、事件重放、索引重建、灾难恢复。

## 11. 发布与回滚

每个阶段使用 `off → internal → shadow → opt-in → cohort → default`。Shadow 只计算 route/impact，不产生副作用。发布看板至少分端、格式、adapter/provider 版本与 workspace cohort。

回滚原则：

- 合同 Provider 保持 N/N-1 兼容窗口；
- 新 Surface 失败可回到安全 Viewer/旧 Sidebar；
- Ontology 投影可从事件重建，错误规则停用不改写历史；
- Workflow/Action 在执行前可停，新提交拒绝；已部分执行的按 receipt 恢复，不能简单“回滚数据库”；
- schema migration 先 expand、双读验证，再 contract。

## 12. 团队与所有权建议

| 领域 | DRI 能力组合 |
|---|---|
| Protocol & Host Authority | Host Bridge、Rust、安全、跨语言合同 |
| Resource & Surface | Flutter、DSH、editor integration、多端 |
| Ontology Platform | 领域建模、数据、查询、权限、schema governance |
| Agent & Workflow | Agent runtime、eval、impact rules、action orchestration |
| Product Experience | Office/knowledge workflow、UX、research、analytics |
| Security & Reliability | threat model、plugin supply chain、SLO、incident response |

每个合同只有一个 owner，但 Provider/Adapter 分散所有。Ontology schema 由领域 owner 负责，平台团队负责护栏，不替业务定义概念。

## 13. 首个可演示里程碑

最有价值且风险可控的第一个 demo：

1. DSH 消息中的 Word 和 Markdown 交付物都以 resourceRef 打开。
2. Host 路由到现有 Word/Markdown Surface，并能解释选择原因。
3. 用户选择一个 PRD 段落，创建 Requirement 和 Anchor。
4. 手工关联一个 CodeSymbol 与 TestCase。
5. 修改并保存该段落后生成 ChangeSet/ImpactSet。
6. Agent 生成测试文档变更 proposal；用户看到 diff 后批准。
7. apply 返回新 revision，Lens 中关系和状态同步更新，审计链完整。

该切片同时验证两条主线，又避免一开始依赖 Excel/Slides/CAD、复杂自动抽取和跨系统自动提交。
