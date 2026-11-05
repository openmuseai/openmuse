# 架构决策与待验证问题

状态：**决策台账 v1**。Accepted 项作为目标方案基线；Proposed 项须经 spike/评审后冻结。

## 1. 已接受决策

### ADR-D1：复用 Host Bridge 元协议

- 状态：Accepted
- 决策：Capability 的发现、绑定、调用、订阅、策略、取消和状态继续使用 `@muse/host-bridge`。
- 理由：当前已有 schema digest、limits、UDS/InProcess、Rust registry/policy/event；另建总线会导致权限和生命周期分叉。
- 后果：Resource/Ontology 是 Host Bridge 上的业务合同，不重复实现 transport。

### ADR-D2：不透明 ResourceRef

- 状态：Accepted
- 决策：跨 DSH/插件/Agent 只传 opaque `resourceRef`；locator、路径、AppFlowy view ID 和云凭据留在 authority Host。
- 理由：兼容现有合同并阻断任意路径/ID 枚举。
- 后果：调试依靠安全 descriptor/trace，不直接打印 locator。

### ADR-D3：Resource、Surface、Ontology 分域

- 状态：Accepted
- 决策：可访问内容、呈现引擎、业务语义使用独立合同与 Registry。
- 理由：同一资源可多种呈现，同一对象可多载体，新增格式不应增加 Ontology 类型。

### ADR-D4：格式与引擎由 contribution 注册

- 状态：Accepted
- 决策：核心不维护扩展名白名单；Format Provider 识别内容，Surface Adapter 声明并 probe 能力。
- 后果：manifest 声明是候选，runtime probe 才是可用事实。

### ADR-D5：所有业务写入 propose-first

- 状态：Accepted
- 决策：统一 `propose → validate/approve → apply → receipt/event`，带 base revision、幂等与审计。
- 例外：用户在已授权原生编辑 Surface 中的正常编辑可由该 Provider 聚合为 commit，但跨资源/Agent 自动写仍走 proposal。

### ADR-D6：Host 选择引擎

- 状态：Accepted
- 决策：Agent/内容只表达 presentation intent，不指定强制 engine；Host 依据能力、策略、placement 和用户偏好路由。

### ADR-D7：Anchor 为跨格式片段引用

- 状态：Accepted
- 决策：统一 Anchor envelope，selector 由格式 Provider 命名和解析。
- 后果：核心认识 resolved/ambiguous/orphaned 等状态，但不理解 Word run、CAD entity 等细节。

### ADR-D8：Ontology 建模业务世界

- 状态：Accepted
- 决策：用具体 ObjectType、Interface、Link、Claim、Action 建模；不按系统表复制，也不上线万能 `KnowledgePoint`。

### ADR-D9：ChangeSet 先于传播

- 状态：Accepted
- 决策：Resource 变化经语义确认形成 ChangeSet，再计算 ImpactSet/触发 Workflow。
- 理由：避免把自动保存、格式变化和低置信推测直接变成业务动作。

### ADR-D10：安全降级是协议能力

- 状态：Accepted
- 决策：按 `edit → annotate → view → metadata → download/system-open` 降级，并明确反馈实际模式。

### ADR-D11：Vendor 不承载 Muse 业务语义

- 状态：Accepted
- 决策：ioffice、Helix、open-file-viewer 保持 vendor 身份；Muse Adapter 管理合同、授权和生命周期。

### ADR-D12：一套合同，多种 placement

- 状态：Accepted
- 决策：Desktop UDS/FFI、Web postMessage/HTTPS、Mobile HTTPS/SSE 只做传输适配；Host capability matrix 决定可执行行为。

## 2. 待验证问题与默认建议

### OQ-1：ResourceRef 的持久性与轮换

- 状态：Proposed
- 问题：引用跨会话持久还是短期？移动/重命名后是否稳定？
- 默认建议：分离稳定 `resourceRef` 与短期 `accessHandle/materializationHandle`；稳定引用按 workspace + provider identity 解析，权限每次重校验。
- Spike：AppFlowy view、workspace file、remote object 三类在移动/删除/恢复/换设备下的矩阵。
- 冻结门：Phase 0 G0。

### OQ-2：Revision token 标准

- 状态：Proposed，阻塞合同冻结
- 现状：Document schema 要求 sha256 风格 token，但 TS 内存实现存在数字 revision。
- 默认建议：协议统一 opaque branded string，Provider 可用 `sha256:*`、`git:*`、`collab:*`；消费者只比较相等，不解析或排序。内存 Provider 改为合法字符串。
- 冻结门：跨 TS/Rust/Dart golden fixtures。

### OQ-3：Desktop DSH ↔ Flutter 的 transport

- 状态：Proposed
- 选择：Rust/FFI Host event、WebView JS channel、loopback HTTP。
- 默认建议：业务合同继续 Host Bridge；Desktop 优先通过 Rust/FFI/受控 Host event 进入 Flutter Orchestrator，WebView channel 仅承载 envelope，不暴露任意 Flutter method。
- Spike 指标：双向取消/事件、崩溃恢复、消息上限、身份绑定、调试性。

### OQ-4：DSH 默认打开的正式 seam

- 状态：Proposed，需上游实现
- 现状结论：`SessionControllerInternals.openPath` 是构造时测试注入，默认交付物点击由 ChatView 注入的 `openFile` 打开 DSH Sidebar；不能把覆盖 internals 当运行时方案。
- 默认建议：增加有契约的 `OpenResource` Service Definition/Provider/Consumer，所有 ProducedFiles/mentions/cards 走 consumer；旧 Sidebar 是 fallback provider。
- 冻结门：上游 spike 覆盖默认点击、显式 native open、插件生命周期与 model-visible error。

### OQ-5：Anchor 的格式标准

- 状态：Proposed
- 默认建议：核心只定义 envelope、revision、quote/fingerprint 和状态；各 Provider 命名 selector schema。优先采用已有稳定 ID/标准（AST/symbol/table/entity），文本引用作为回退。
- Spike：Word 重排/协同编辑、Markdown AST 变化、Git rename、PDF OCR 四套漂移数据集。

### OQ-6：Ontology MVP 存储

- 状态：Proposed
- 选择：关系型、专用图数据库、事件 + 混合投影。
- 默认建议：不可变事件 + 关系型 current projection + adjacency/search/vector read models；通过 repository/query 接口隐藏实现。只有基准证明需要才增加图后端。
- 基准：10M objects/50M links 的限定深度查询、权限过滤、更新、重建与运维成本；实际目标规模由产品数据校准。

### OQ-7：Ontology 与 local-first 的一致性

- 状态：Proposed
- 默认建议：Object/Claim proposal 可离线排队；确认/Action apply 在拥有 authority 的节点执行。采用 workspace-scoped event sync 和 deterministic merge，只对声明可合并的字段 CRDT 化。
- 不建议：把所有 Ontology 属性默认 LWW，可能覆盖审批事实。

### OQ-8：语义抽取策略

- 状态：Proposed
- 默认建议：结构化模板/确定性规则优先，模型只提出 Claim；记录 model/prompt/schema version 与证据。按 ObjectType 建立离线 gold set，达标后逐类开放。
- 冻结门：precision/recall、接受率、敏感数据路由、成本和延迟门槛由领域 owner 签字。

### OQ-9：自动审批边界

- 状态：Proposed
- 默认建议：MVP 仅允许低风险、确定性、可逆、单真源、writeSet 有界的 metadata 动作自动批准；正文、代码、外发和权限动作必须人工批准。
- 扩大条件：误动作率、补偿成功率、安全评审、workspace 管理员 opt-in。

### OQ-10：跨资源 Action 一致性

- 状态：Proposed
- 默认建议：显式 saga + provider receipt + 补偿/人工恢复，不尝试全局两阶段提交。
- Spike：Word + Git + test provider 的超时、重复响应、成功后断线、不可逆提交。

### OQ-11：插件签名与准入

- 状态：Proposed
- 默认建议：内部插件也使用 digest/来源记录；外部发布前要求签名、SBOM、权限差异、TCK 与远程禁用。高风险写 Provider 使用更严格认证层级。

### OQ-12：事件总线与保留

- 状态：Proposed
- 默认建议：Host Bridge event 负责进程/会话级订阅；持久 Ontology/Workflow event log 由 Runtime 管理，两者用 trace/causal ID 关联，不强求同一存储。
- 需决定：保留期、重放范围、租户隔离、顺序保证和 dead-letter owner。

### OQ-13：对象权限与来源权限的组合

- 状态：Proposed，安全阻塞项
- 默认建议：默认取交集；派生 Claim/摘要继承最敏感输入分类。任何“降级共享”需显式脱敏 Action 和审批，不能因写入 Ontology 自动变得可见。
- 冻结门：检索、关系计数、缓存和模型上下文的侧信道测试。

### OQ-14：用户偏好与组织策略优先级

- 状态：Proposed
- 默认顺序：安全/合规 policy → 端/设备 capability → workspace admin policy → 用户默认 → 内容 hint → 性能评分。
- 例外：用户当次显式“打开方式”可以覆盖默认偏好，但不能覆盖前三层。

## 3. 明确拒绝的方案

| 方案 | 拒绝原因 |
|---|---|
| 在所有点击处按扩展名调用不同编辑器 | 入口复制、无 capability probe、无法扩 CAD/3D |
| 把真实 path 作为 Agent 工具参数 | 越权、远端无意义、无法轮换和审计 |
| 覆盖 `SessionControllerInternals.openPath` 完成集成 | 非运行时服务 seam，也拦不住默认 ChatView 打开链 |
| 用一个 generic JSON patch 编辑所有格式 | 破坏 Word/表格/代码各自并发与语义 |
| 将全部原文复制进 Ontology/向量库 | 真源分叉、权限/删除困难、成本和陈旧性 |
| 一个万能 `KnowledgePoint` + `relatedTo` | 无约束、无法治理动作、最终成为数据垃圾场 |
| Agent 直接根据相似度修改下游 | 不可解释、误报会产生副作用、责任不清 |
| 为跨系统修改宣称 ACID | 多数 Provider 无共同事务；部分失败不可避免 |
| 让 manifest 声明等同运行能力 | 当前 Office 占位已证明声明可能先于实现 |

## 4. 决策流程

每个 Proposed 项的 owner 应提交轻量 ADR：上下文、选项、证据、性能/安全结果、决定、后果、回滚。决策状态为 `proposed → accepted/rejected → superseded`，并链接 spike 与 fixture。影响公开合同的决定必须在代码合入前更新本文和相应协议文档。

## 5. 下一次架构评审议程

按阻塞关系建议依次评审：

1. OQ-2 revision token 与合同生成/fixture。
2. OQ-4 DSH consumer seam、OQ-3 Desktop transport。
3. OQ-1 ResourceRef 生命周期、OQ-5 Anchor envelope。
4. Phase 1 垂直切片和 feature flag/telemetry。
5. OQ-6/OQ-7 Ontology 存储与 local-first。
6. OQ-13 权限继承、OQ-8 抽取评估、OQ-9 自动审批。

完成前四项即可开始统一资源 MVP；无需等待全部 Ontology 问题冻结。

