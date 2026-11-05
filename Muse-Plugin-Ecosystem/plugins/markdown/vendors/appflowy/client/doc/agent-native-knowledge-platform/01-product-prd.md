# 产品需求文档：Muse Agent 原生知识与工作平台

状态：**目标 PRD v2**  
基线：基于 2026-09-14 工作区实现审计  
适用端：Desktop（macOS / Windows / Linux）、Web、Mobile（iOS / Android）  
说明：本文定义产品范围、逐端能力和可验收指标；技术实现分别见统一资源、Surface、Ontology、Workflow 与安全设计文档。

## 1. 产品定义

Muse 是以 Agent 为核心的知识构建、浏览、传播与行动平台。用户仍然使用适合内容的编辑器，但文件、代码、应用视图和业务对象不再被各自的格式与应用隔离：

- 任意内容通过统一资源协议被引用、授权、打开、保存和订阅变化；
- 内容片段可以成为需求、决策、代码符号、测试义务等知识对象的证据；
- 变化先形成可解释的业务影响，再由人和 Agent 共同提出、审批和执行动作；
- Desktop、Web、Mobile 使用同一对象和动作语义，但按设备能力提供不同功能集。

一句话体验：**用户可以从任何任务或知识对象进入最合适的内容工具，并让有证据、受治理的 Agent 将变化安全传播到下一项工作。**

## 2. 背景与问题

### 2.1 用户问题

1. **打开割裂**：同一个交付物在聊天、文件树和工作区中的打开方式不同；Markdown 有内置路径，Office、代码和新格式各自接线。
2. **语义割裂**：PRD、代码、测试和发布说明表达同一个功能，却只能靠文件名、链接和人的记忆维持一致。
3. **变化失联**：产品经理修改验收标准后，开发和测试很难知道“具体改了什么、为什么与我有关、需要采取什么动作”。
4. **Agent 缺少可治理行动面**：Agent 能生成内容，却经常不知道真实对象、当前 revision、允许执行的动作和审批要求。
5. **跨端能力含混**：桌面可以启动原生引擎，Web/Mobile 不可以；如果产品只做同一套按钮，会产生无法完成或误导性的体验。

### 2.2 当前实现约束

- Host Bridge、Plugin Graph v2、Facet/Surface Runtime、Document proposal/apply、审批 proof 已有基础。
- DSH 默认交付物点击仍由 ChatView 的 `openFile(path)` 打开 Sidebar；尚无统一的运行时 Resource Presentation consumer。
- ioffice 当前只有 macOS arm64 Word 引擎具备实际绑定；Excel、Slides、PDF 仍是槽位/占位状态。
- open-file-viewer 与 Helix 尚未接入 Muse Surface。
- Web/Mobile 已有 parent bridge 和部分 AppFlowy 打开能力；Desktop WebView 尚无通用 Host → Flutter Intent 下行。
- Ontology Object/Link/Action/Impact/Workflow Runtime 当前尚未实现。

因此本 PRD 的“目标功能”不等于“当前已有功能”。每阶段交付必须使用第 15 节的版本门槛。

## 3. 产品目标与非目标

### 3.1 目标

| ID | 目标 | 用户结果 | 衡量方式 |
|---|---|---|---|
| G1 | 任意资源一等化 | 用户点击资源总能获得正确打开、可解释降级或明确下一步 | 有效打开结果率、正确路由率 |
| G2 | 知识与载体双向可达 | 从需求能找到文档、代码、测试；从内容选区能回到对象 | 可追溯覆盖率、Anchor 定位率 |
| G3 | 变化可解释传播 | 变更能生成原因明确、责任清楚的 ImpactSet | 影响处置率、漏报/误报率 |
| G4 | Agent 行动受治理 | Agent 只提出可验证动作，写入经过权限、revision 和审批 | 审计完整率、越权写入数、冲突保护率 |
| G5 | 各端任务闭环 | Desktop 重创作、Web 协作、Mobile 审批/捕获各自完成核心任务 | 分端任务完成率、跨端接续成功率 |
| G6 | 可扩展格式生态 | 新格式通过 Provider/Adapter 接入，无需修改核心路由 | Adapter 接入工时、核心零修改验证 |

### 3.2 非目标

- 不重新实现一套通用 Office 内核，不保证不同引擎间二进制格式完全无损。
- 不把 Git、AppFlowy Collab、文件系统或企业系统的数据全部复制为新真源。
- 不让所有自然语言抽取结果自动成为事实。
- 不在首版提供任意图数据库探索、完整低代码应用平台或无审批的跨系统自动写入。
- 不要求 Mobile 具备 Desktop 的原生编辑器、终端和大文件处理能力。
- 不让远端 Host 操作用户本机路径、Finder、Explorer 或未授权应用。

## 4. 用户、场景和核心任务

| 角色 | 高频场景 | 必须完成的任务 | 主要端 |
|---|---|---|---|
| 产品经理 | 写 PRD、改验收标准、确认影响 | 编辑载体；将段落关联需求；发起影响分析；确认变更 | Desktop / Web |
| 开发工程师 | 从需求理解实现、处理影响 | 从代码/任务追溯需求与决策；审阅代码动作；解决 revision 冲突 | Desktop |
| 测试工程师 | 更新用例、确认覆盖 | 查看受影响测试；审阅 Agent 草案；回写结果与证据 | Desktop / Web / Mobile |
| 知识工作者 | 收集、撰写、关联资料 | 打开多格式资源；从选区创建对象/Claim；分享对象视图 | Desktop / Web |
| 负责人/审批者 | 看风险、审批变更、跟进阻塞 | 在 Impact Inbox 聚合审阅；批准/拒绝；查看完成状态 | Web / Mobile |
| 自动化设计者 | 定义触发、条件和动作 | 模拟 Workflow；设置审批和预算；查看失败/重放 | Desktop / Web |
| 管理员/安全负责人 | 控制接入、权限与审计 | 管理 Provider/Adapter；策略配置；调查动作和数据访问 | Web / Desktop |
| 插件开发者 | 接入新格式/系统 | 声明 capability；通过合同与 Adapter TCK；验证安全降级 | Desktop |

## 5. 产品对象与用户语言

| 产品词汇 | 用户理解 | 技术映射 |
|---|---|---|
| 资源 | 可以打开、查看或操作的内容 | `ResourceRef + ResourceDescriptor` |
| 工作面 | 当前设备承载资源的查看/编辑区域 | Surface + Engine Adapter |
| 知识对象 | 跨文件存在、有业务意义的事物 | Ontology Object |
| 关联 | 两个对象之间含义明确的关系 | Link instance |
| 锚点 | 内容中的准确位置 | Anchor：段落、页、单元格、符号、CAD 实体等 |
| 声明 | 有来源、可确认或驳回的事实主张 | Claim + Evidence |
| 变更集 | 一组具有共同意图的对象/资源修改 | ChangeSet |
| 影响集 | 某次变化波及的对象、路径和责任 | ImpactSet |
| 动作 | 对真实对象或系统进行受控改变 | Action Type / Action Run |
| 自动化 | 事件满足条件后运行的受治理流程 | Workflow Definition / Run |

产品界面可使用“知识点”作为通俗称呼，但创建时必须选择需求、决策、风险、测试等具体类型，不能生成无约束的万能对象。

## 6. 详细业务场景

### S1：从 Agent 交付物打开 Word、代码和未知格式

**参与者**：知识工作者；**入口**：DSH 消息卡片/正文引用；**端**：Desktop、Web、Mobile。

前置条件：Agent 已通过结构化交付声明提供 `resourceRef`，用户具有至少 metadata 权限。

主流程：

1. 消息展示资源名、类型、大小、来源、最近 revision 和可用模式，不展示 Host 私有路径。
2. 用户单击资源，系统在 100 ms 内提供按压/加载反馈并发出 Presentation Request。
3. Host 校验引用和权限，Format Provider 描述内容，Orchestrator 根据端能力、策略、用户偏好和运行状态选择 Adapter。
4. Desktop 上 Word 路由至 ioffice Word；代码可路由至 Helix；常见只读格式进入安全 Viewer。
5. Web/Mobile 若无原生编辑能力，自动选择只读预览或对象摘要，并提示“在桌面继续”。
6. Surface ready 后定位到指定 Anchor；Agent 只接收有界的当前选区上下文。

异常与降级：

- 格式未知：展示 metadata、下载和系统打开；不得空白或无限加载。
- 权限不足：展示可请求的权限和资源安全名称，不泄露真实路径。
- Adapter 崩溃：记录 route receipt，尝试安全 Viewer；不能循环重启。
- 文件过大：先显示 metadata/首屏，后台渐进加载；达到端侧上限时转 Desktop/下载。

验收：同一 `resourceRef` 从消息卡片、内联引用和最近资源进入同一决策；模型不能强制指定未经授权的引擎。

### S2：产品经理修改需求，开发与测试收到影响

**参与者**：产品经理、开发、测试、发布负责人；**入口**：Word/Markdown Surface；**端**：Desktop/Web 编辑，任意端审阅。

主流程：

1. 产品经理把“失败重试两次”改为“最多三次，指数退避”并保存。
2. Resource Provider 成功提交 revision 后发出变化事件；自动保存先去抖，不直接通知全部人员。
3. 系统读取最小 before/after Anchor，提出 `Requirement.acceptanceCriteria` 的语义变化。
4. 结构规则可直接验证的变化进入确认步骤；不确定变化进入 Claims Review。
5. 确认后生成 ImpactSet：实现代码、测试用例、开发设计、Release readiness，并解释每条关系路径。
6. Agent 针对每个责任域生成独立 Action proposal 和原生 diff。
7. 开发、测试负责人分别批准、要求修改或豁免；未批准项不执行。
8. 执行器在 apply 时重新检查权限和 revision；成功后显示 receipt，冲突项重新生成并复核。

关键产品约束：

- 相似度只能产生“可能相关”，不能自动成为确认影响。
- 文档保存、语义确认和下游修改是三个不同动作。
- 部分成功必须显示已完成项、失败项和恢复责任人。

验收：用户能在一个 Impact Bundle 内看清“改了什么、为什么与我有关、Agent 想改什么、谁批准、最终是否生效”。

### S3：开发从代码追溯业务上下文

**参与者**：开发工程师；**入口**：Helix/代码 Surface 的符号选区；**端**：Desktop。

主流程：

1. 用户在 `retryPayment()` 上打开 Knowledge Lens。
2. Lens 展示 `implements Requirement R42`、相关 Decision、TestCase 和最新 ChangeSet。
3. 用户点击 Requirement 证据，Muse 在已打开 Word Surface 中复用实例并定位段落；若未打开则创建新 Surface。
4. 用户询问 Agent“此函数为什么改为三次重试”，回答必须引用 Decision 和 PRD Anchor。
5. 用户可发起“标记实现已更新”Action；提交条件检查代码 revision、测试结果和权限。

异常：Anchor 漂移时显示 relocated/ambiguous/orphaned；不能把相似段落静默当成原证据。

验收：从代码符号到需求原文不超过 3 次用户动作；所有解释至少包含一个可点击证据或明确说明没有证据。

### S4：测试负责人在手机上审批用例变更

**参与者**：测试负责人；**入口**：Mobile Impact Inbox 推送；**端**：Mobile。

主流程：

1. 推送只显示工作区、变更类型和风险，不在锁屏暴露敏感正文。
2. 打开后完成身份/策略检查，展示需求语义 diff、影响路径和测试用例 proposal。
3. 用户查看移动端优化后的单元格/代码 diff，可展开原文证据的只读预览。
4. 用户批准、拒绝或要求修改；批准只创建有效 approval，不假装已执行。
5. 后端执行完成后 card 更新为 succeeded/conflict/partial；若需 Desktop 处理，用户一键 handoff。

离线分支：允许缓存后查看非敏感摘要和填写意见；联网后必须重新鉴权/revision 校验，离线操作不能直接执行写入。

验收：在目标网络下完成一次标准审批的中位交互步骤不超过 5 步；状态语言区分“已批准”和“已生效”。

### S5：从文档选区创建并治理知识对象

**参与者**：产品经理/知识工作者；**入口**：Word、Markdown、表格或代码选区；**端**：Desktop/Web，Mobile 仅轻量捕获。

主流程：

1. 用户选择内容并点击“关联知识”。
2. 系统先搜索重复对象，Agent 建议具体 ObjectType、标题、属性、关系和来源。
3. 用户选择已有对象或创建新对象；必填项、可见性和 owner 实时校验。
4. Provider 创建 Anchor；Ontology 创建 proposed Claim 或确认关系。
5. Knowledge Lens 立即展示对象，但待确认内容带明确状态。

验收：创建结果可反向定位原内容；移动内容后 Anchor 状态可检测；拒绝 Claim 不会删除原文或对象。

### S6：团队负责人查看交付一致性

**参与者**：团队负责人；**入口**：Object View / Impact Center；**端**：Web/Desktop/Mobile 摘要。

主流程：

1. 打开 Release 对象，查看纳入的 Feature、关键 Requirement、测试覆盖、未处置 Impact 和失败 Action。
2. 按责任团队、严重度、截止时间和对象类型筛选。
3. 对缺少测试证据、存在冲突 Claim 或 Anchor 失效的项目分派处理人。
4. 运行 readiness Action，结果引用计算规则和输入 revision。

验收：负责人看到的是对象级阻塞，不需要逐一打开所有文档；权限不可见项目不会通过关系计数泄露。

### S7：管理员安装新 CAD/3D Adapter

**参与者**：管理员、插件开发者；**入口**：Plugin & Capability Center；**端**：Desktop/Web 管理面。

主流程：

1. 管理员查看发布者、签名、digest、支持格式、端、网络/文件权限和 TCK 结果。
2. 在测试 workspace 启用 Adapter，使用样例资源执行 probe/open/anchor/crash/fallback 测试。
3. 灰度到指定用户；路由看板显示选择率、失败率和 fallback。
4. 达标后设为 STEP 格式首选；卸载后资源安全降级，既有 CAD Anchor 保持 provider-unavailable。

验收：接入不修改核心扩展名表；权限扩大或包 digest 变化需要重新审核。

### S8：权限撤销和跨系统部分失败

**参与者**：任意用户、管理员；**端**：全部。

- 审阅过程中权限撤销：Surface 隐藏敏感内容，搜索/Lens/Agent Context 同步过滤，旧 approval 在 apply 时失败。
- Word、Git、测试系统三步动作只有前两步成功：UI 显示“部分完成”，保留 receipt，停止依赖步骤并创建恢复任务。
- 用户不得只看到通用“出错”；必须知道哪些事实已改变、哪些没有、下一责任人是谁。

## 7. 功能需求清单

优先级：`P0` 为首个业务闭环必须交付，`P1` 为规模化前必须交付，`P2` 为生态扩展。

### 7.1 资源发现与打开

| ID | 优先级 | 功能需求 | 验收标准 |
|---|---|---|---|
| RES-01 | P0 | 所有入口使用 resourceRef 打开 | DSH 卡片、mention、最近资源、证据链接产生相同 route input |
| RES-02 | P0 | 展示安全 Descriptor | 名称、格式、大小、revision、Provider、可用模式；不暴露 Host locator |
| RES-03 | P0 | 可解释默认路由 | route receipt 包含选中 Adapter、实际 mode、理由与合法备选 |
| RES-04 | P0 | “打开方式” | 只列当前端 probe 成功且策略允许的 Adapter，可保存本设备/工作区偏好 |
| RES-05 | P0 | 安全降级 | edit → annotate → view → metadata → download/system-open，每次明确实际模式 |
| RES-06 | P0 | 生命周期 | open/ready/focus/background/close/dispose 有唯一 receipt，复用兼容 Surface |
| RES-07 | P1 | 大文件渐进加载 | 首屏、页/range/stream 按需读取；取消后停止 I/O 和 materialization |
| RES-08 | P1 | 跨端继续 | 从 Web/Mobile 发送短期 handoff ref 到已授权 Desktop，Desktop 重新鉴权 |
| RES-09 | P2 | 第三方格式生态 | 仅通过 Manifest + Provider + Adapter + TCK 加入新格式 |

### 7.2 Surface 编辑和上下文

| ID | 优先级 | 功能需求 | 验收标准 |
|---|---|---|---|
| SUR-01 | P0 | Markdown 完整编辑 Surface | 打开、定位、编辑、保存、冲突和恢复可用 |
| SUR-02 | P0 | Word Surface | macOS arm64 使用当前 ioffice 能力；其他端按能力预览/系统打开，不虚报编辑 |
| SUR-03 | P1 | 安全 Viewer | 常见 PDF/图片/Office 等只读预览运行于隔离 Web Surface |
| SUR-04 | P1 | Helix Surface | Desktop PTY、resize/focus/save/close、受控 materialization 和 revision commit |
| SUR-05 | P0 | Context contribution | 只发布当前页/选区/对象等有界上下文，带 TTL/revision/大小限制 |
| SUR-06 | P1 | Anchor 定位 | Markdown、Word、Code、PDF 支持定位；漂移返回 relocated/ambiguous/orphaned |
| SUR-07 | P1 | 外部变化 | 已打开资源有新 revision 时提供比较、重载、rebase，不静默覆盖 |

### 7.3 Ontology 与知识浏览

| ID | 优先级 | 功能需求 | 验收标准 |
|---|---|---|---|
| ONT-01 | P0 | 首批对象 | Requirement、Decision、CodeSymbol、TestCase、Artifact、Anchor 可创建/查询 |
| ONT-02 | P0 | 有类型关系 | implements、verifiedBy、representedBy、evidencedBy 等校验源/目标类型 |
| ONT-03 | P0 | Knowledge Lens | 从当前选区显示对象、证据、关系、待处理影响和可用动作 |
| ONT-04 | P0 | Object View | 摘要、属性、证据、关系、时间线、动作和权限状态完整 |
| ONT-05 | P0 | Claim Review | Agent 提议与人工确认事实可区分、可修改接受、拒绝、取代 |
| ONT-06 | P1 | 搜索与关系路径 | 按类型/状态/owner 查找，可解释限定深度路径，执行权限过滤 |
| ONT-07 | P1 | Anchor 修复 | 模糊/失效证据进入处理队列，不参与要求“当前证据”的自动化 |
| ONT-08 | P2 | 自定义领域类型 | 经 schema proposal、兼容检查和治理后扩展 Object/Link/Action |

### 7.4 影响、动作与 Agent

| ID | 优先级 | 功能需求 | 验收标准 |
|---|---|---|---|
| IMP-01 | P0 | 语义 ChangeSet | 记录 before/after revision、对象属性变化、证据和确认状态 |
| IMP-02 | P0 | ImpactSet | 显式关系与规则计算影响，显示路径、严重度、owner、建议动作 |
| IMP-03 | P0 | Impact Inbox | 按责任人聚合、去抖、筛选、委派、豁免和处置 |
| ACT-01 | P0 | Action proposal | 固定 ActionType、输入、writeSet、base revision、风险、diff、幂等键 |
| ACT-02 | P0 | 审批 | 支持批准/拒绝/要求修改；proposal 内容变化使旧审批失效 |
| ACT-03 | P0 | 执行与 receipt | apply 时重新鉴权/验 revision；显示 succeeded/conflict/partial/failed |
| AGT-01 | P0 | 有证据回答 | 业务解释引用 Object/Anchor；无证据时明确说明，不伪造引用 |
| AGT-02 | P0 | 窄工具面 | Agent 只能使用 discover 得到的引用和 Action，不接受任意路径/命令 |
| AGT-03 | P1 | 变更草案 | 为文档、代码、测试生成原生 diff；默认 propose-first |
| WFL-01 | P1 | Workflow Runtime | trigger/condition/debounce/approval/wait/retry/budget/dead-letter |
| WFL-02 | P1 | 模拟与影子模式 | 上线前展示会触发的对象、Action 和权限，不执行副作用 |

### 7.5 管理、安全和审计

| ID | 优先级 | 功能需求 | 验收标准 |
|---|---|---|---|
| GOV-01 | P0 | 统一权限 | Resource/Object/Field/Anchor/Action 权限在 UI、搜索、Agent、执行一致 |
| GOV-02 | P0 | 审计链 | 每次写入具备来源、actor、grant、approval、revision、receipt 和结果 |
| GOV-03 | P0 | 权限撤销 | 撤销后已开 Surface、缓存、搜索、Lens、Agent Context 和旧 approval 失效 |
| GOV-04 | P1 | 插件治理 | 展示签名、digest、权限、版本、TCK；支持灰度、回滚和禁用 |
| GOV-05 | P1 | 数据生命周期 | 删除、保留、导出、索引清理和 orphan Anchor 行为可配置/审计 |
| GOV-06 | P1 | 运行看板 | 按端/格式/Adapter/Provider 查看 SLO、失败、fallback、Action 和 Workflow |

## 8. 各端产品定位

### 8.1 Desktop：重创作与专业工具编排

Desktop 是完整工作台，承担本地文件、原生编辑器、PTY、大文件、多 Surface 和高级 Workflow 设计。

特有能力：

- 本地 workspace file 与系统文件选择器；Host 内部路径映射和安全 system-open。
- ioffice Word 原生编辑；当前首发限定 macOS arm64。Windows/Linux 在引擎通过 TCK 前只提供 Viewer/系统打开。
- Helix PTY Surface、多窗格/多 Surface、拖放、键盘命令和跨资源 diff。
- 大文件 range/stream、本地缓存、离线 proposal queue。
- 完整 Knowledge Lens、Object View、Impact Center、Automation Studio 和插件开发诊断。
- 接收 Mobile/Web handoff 并重新鉴权、恢复对象和 Anchor 上下文。

Desktop 不应把“本机能力”泄露给远端 Host；即使资源来自云端，也由当前 Desktop Host 决定能否物化和打开。

### 8.2 Web：协作、审阅与零安装访问

Web 是完整协作面，但不假设访问用户本地路径或原生进程。

特有能力与约束：

- AppFlowy/浏览器支持的编辑，安全 Web Viewer，上传后注册为 Resource。
- Object View、关系浏览、Impact Inbox、ChangeSet 审阅、审批和管理员控制台为完整能力。
- Automation Studio 支持查看、模拟和管理；高风险本地动作需转交 Desktop Host。
- 不显示“Finder/Explorer 打开服务器文件”；system-open 仅针对浏览器已获得的下载/URL 能力。
- Viewer 使用 sandbox/CSP/受控 loopback 或授权远端 stream；无任意 Host API。
- 断线后保留只读状态和草稿意见，恢复时重新取 revision。

### 8.3 Mobile：通知、捕获、阅读、审批和接续

Mobile 不复制 Desktop，而要把移动中最重要的闭环做到可靠：

- Impact/Approval push、对象摘要、关系路径、证据只读预览。
- 批准、拒绝、评论、委派、豁免；明确区分 approval 与 apply result。
- 轻量 Markdown/AppFlowy 编辑、拍照/扫描/语音/分享入口可注册 Artifact 或 proposed Claim。
- 只读 Office/PDF/图片预览；大文件按页/range；不提供 Helix、本地原生 Office 或 Automation authoring。
- 一键“在桌面继续”，传对象、resourceRef、Anchor 和任务上下文，不传本地路径。
- 离线缓存受数据分类约束；离线只能形成待同步 proposal/意见。

### 8.4 当前能力到目标能力

| 端 | 当前代码基线 | Release A | Release B | Release C |
|---|---|---|---|---|
| Desktop | DSH local sidecar + WebView；Flutter Surface Runtime 已有；ioffice Word 仅 macOS arm64；缺通用 Intent 下行 | 打通 DSH → Host → Flutter；Markdown/Word 统一打开、合法降级和 route receipt | 完整 Lens/Object/Impact/Approval；Word/Markdown 变更闭环 | Viewer、Helix、多 Surface、Workflow、接收 handoff |
| Web | parent bridge 承载部分 workspace/context/presentation 业务消息 | parent bridge 适配为中立 Host Bridge envelope；统一打开和浏览器降级 | 完整 Object/Impact/ChangeSet/Approval 协作 | Viewer 隔离完善、Automation Studio、发送 handoff、管理控制台 |
| Mobile | `AppFlowyDshControlHost` 可发布目录与 Markdown/Word snapshot，主要能导航 AppFlowy Document | 统一资源卡片、对象摘要、合法只读降级 | 移动 Impact Inbox、证据预览、审批/委派、结果状态 | push、离线 proposal、Desktop handoff、Workflow 状态/暂停 |

每个 Release 都必须保持旧路径可回退。新功能只有在目标端实际 probe 成功时才进入 UI；Manifest 或设计稿中的能力声明不能单独决定按钮是否出现。

## 9. 逐端功能矩阵

图例：`完整`＝本端可闭环；`受限`＝能力或格式受限；`只读`＝查看/审阅；`转交`＝交给其他 Host；`—`＝不提供。

| 功能 | Desktop macOS | Desktop Win/Linux | Web | Mobile | 首发阶段 |
|---|---:|---:|---:|---:|---|
| 统一 resourceRef 打开 | 完整 | 完整 | 完整 | 完整 | P0 |
| Markdown/AppFlowy 编辑 | 完整 | 完整 | 完整 | 受限 | P0 |
| ioffice Word 编辑 | 完整（arm64 首发） | 受限/待引擎 | 只读/转交 | 只读/转交 | P0 macOS |
| Excel/Slides/PDF 原生引擎 | 待 Provider | 待 Provider | — | — | P2 |
| 安全多格式 Viewer | 完整 | 完整 | 完整 | 受限 | P1 |
| Helix 编辑 | 完整 | 完整（打包后） | 转交/远端 PTY 可选 | 转交 | P1 |
| 多 Surface/分屏 | 完整 | 完整 | 受限 | — | P1 |
| Knowledge Lens | 完整 | 完整 | 完整 | 摘要/抽屉 | P0 |
| Object View/关系路径 | 完整 | 完整 | 完整 | 完整优化版 | P0 |
| 从选区创建 Anchor | 完整 | 完整 | 完整（支持格式） | 受限 | P0/P1 |
| Impact Inbox | 完整 | 完整 | 完整 | 完整优化版 | P0 |
| ChangeSet 原生 diff | 完整 | 完整 | 完整/按格式 | 摘要 + 关键 diff | P0 |
| 审批/拒绝/委派 | 完整 | 完整 | 完整 | 完整 | P0 |
| Action 执行 | 完整 | 完整 | 远端 Host | 远端 Host | P0 |
| Workflow 查看/运行 | 完整 | 完整 | 完整 | 状态/暂停 | P1 |
| Workflow 设计/模拟 | 完整 | 完整 | 完整 | — | P1 |
| 插件开发诊断 | 完整 | 完整 | 管理/观测 | — | P1 |
| 本地离线 proposal | 完整 | 完整 | 受限 | 受限 | P1 |
| 跨端 handoff | 接收/发送 | 接收/发送 | 发送 | 发送 | P1 |

“完整”仍受 Resource Provider、用户权限和 workspace policy 限制，不代表所有格式必然可编辑。

## 10. 分端关键流程

### 10.1 Desktop 打开与编辑

```text
click → Host authorize → describe/probe → route → materialize
      → create/focus Surface → ready → edit → propose/commit → receipt
```

必须显示的中间状态：正在校验、正在准备内容、正在启动引擎、只读降级、保存中、冲突、恢复中。

### 10.2 Web 打开与审阅

```text
click → remote Host authorize → descriptor/range URL
      → sandbox Viewer/Object Surface → evidence/impact review
      → approval → remote ActionRun status
```

浏览器刷新后应恢复 task/object/Anchor；短期内容 URL 不写入永久历史或可分享链接。

### 10.3 Mobile 通知到审批

```text
redacted push → unlock/auth → fetch current Impact Bundle
      → inspect semantic diff/evidence → approve/comment/delegate
      → wait/leave → receive final ActionRun result
```

若 revision 在审阅期间变化，审批按钮失效并要求刷新差异。

## 11. 性能、容量与可靠性指标

以下是产品验收目标，不代表当前已经达到。Phase 0 必须采集现状；若需调整，只能通过评审修改适用数据集、端或阶段，不能通过排除失败请求美化指标。

### 11.1 测试档位

| 档位 | 资源大小 | 参考内容 |
|---|---:|---|
| Small | ≤ 1 MiB | Markdown、短代码、普通图片 |
| Medium | > 1 MiB 且 ≤ 25 MiB | 常规 Word/PDF、代码包 |
| Large | > 25 MiB 且 ≤ 250 MiB | 大型 PDF、演示、模型 |
| Oversize | > 250 MiB | 必须由 Provider/Adapter 声明能力，允许转 Desktop/下载 |

Web 基准网络：50 Mbps、RTT 100 ms、丢包 1%；Mobile 基准网络：20 Mbps、RTT 120 ms、丢包 1%，同时记录 Wi-Fi/弱网分层。端侧性能报告按设备等级和冷/暖启动分开。

### 11.2 通用协议与路由 SLO

| 指标 | 目标 | 口径 |
|---|---:|---|
| 用户手势到可见加载反馈 | p95 ≤ 100 ms | 全端；不含系统动画 |
| Host-local route decision | p95 ≤ 50 ms | Descriptor 已缓存，不读完整内容 |
| Remote route decision | p95 ≤ 200 ms | 基准网络，含一次 Host RTT |
| Presentation Request 获得 terminal/ready receipt | 100% | 每个 request 恰好一个最终结果；取消也算明确结果 |
| 有效打开结果率 | ≥ 99.9% | `opened/focused/fallback/explicit-unsupported` / 合法请求；policy denied 单列 |
| 默认路由正确率 | ≥ 99% | 抽样中无需用户立即切换 Adapter 的比例 |
| 取消传播 | p95 ≤ 500 ms | 取消后停止新增读取/启动，允许引擎清理 |
| 权限撤销生效 | p95 ≤ 2 s | 在线 Surface、Context、搜索和新调用全部失效 |

### 11.3 Desktop 指标

| 指标 | Small | Medium | Large/说明 |
|---|---:|---:|---|
| Markdown 首个可交互内容（暖启动） | p95 ≤ 700 ms | p95 ≤ 1.2 s | 流式；首屏 ≤ 2 s |
| Word 首个可交互页面（ioffice 暖启动） | p95 ≤ 1.5 s | p95 ≤ 2.5 s | 首屏 ≤ 4 s，后台继续布局 |
| 安全 Viewer 首屏 | p95 ≤ 1.2 s | p95 ≤ 2.0 s | 首屏 ≤ 4 s，按页/range |
| Helix PTY 可输入 | p95 ≤ 800 ms | p95 ≤ 1.5 s | 大文件需告警/按引擎能力 |
| 本地保存确认 | p95 ≤ 300 ms | p95 ≤ 800 ms | 不含远端同步完成 |
| Anchor 定位 | p95 ≤ 150 ms | p95 ≤ 300 ms | Large p95 ≤ 1 s |
| Surface 崩溃恢复 | ≤ 5 s 出现恢复入口 | 适用于有持久 revision 的资源 |  |
| 稳态资源占用 | Shell + 单一轻量 Surface ≤ 600 MiB | 原生引擎单独分项，禁止用总平均掩盖泄漏 |  |

### 11.4 Web 指标

| 指标 | 目标 |
|---|---:|
| Shell 可交互（暖缓存） | p75 ≤ 1.5 s，p95 ≤ 3 s |
| Object View 首屏 | p95 ≤ 2 s |
| Small/Medium Viewer 首屏 | p95 ≤ 2.5 s / 4 s |
| 关系查询（1 跳、≤200 条可见边） | p95 ≤ 800 ms |
| ChangeSet 审阅页首屏 | p95 ≤ 2.5 s；大型 diff 渐进加载 |
| 断线检测与离线提示 | p95 ≤ 3 s |
| 连接恢复后状态重校验 | p95 ≤ 2 s，不自动复用旧 approval |
| 浏览器内存 | 常规单 Surface ≤ 500 MiB；超限前主动降级/提示 |

### 11.5 Mobile 指标

| 指标 | 目标 |
|---|---:|
| 冷启动到 Impact Inbox 可操作 | p75 ≤ 2.5 s，p95 ≤ 4.5 s |
| 推送点击到变更摘要 | p95 ≤ 2.5 s |
| 已缓存 Object View 首屏 | p95 ≤ 800 ms |
| 在线 Object View 首屏 | p95 ≤ 2 s |
| 审批提交到已接收回执 | p95 ≤ 1.5 s；执行完成异步展示 |
| Small Viewer 首屏 | p95 ≤ 2.5 s |
| Medium 文档第一页 | p95 ≤ 4.5 s；不得等待整文件下载 |
| Handoff 创建 | p95 ≤ 2 s；在线 Desktop 收到 p95 ≤ 5 s |
| 崩溃率 | crash-free sessions ≥ 99.8% |
| 后台流量 | 无用户任务时不预取正文；push payload 不含敏感正文 |

### 11.6 Ontology、Impact 与 Action 指标

| 指标 | 目标 |
|---|---:|
| Object ID 查询 | local p95 ≤ 100 ms；remote p95 ≤ 400 ms |
| 1 跳邻接查询 | p95 ≤ 500 ms（≤200 条可见边） |
| 限定 3 跳路径 | p95 ≤ 2 s；超预算返回部分结果与截断原因 |
| Anchor 当前 revision 定位成功率 | ≥ 99%（支持格式、未删除内容） |
| Anchor 漂移误绑定率 | < 0.1%；不确定必须进入 ambiguous |
| 普通 ImpactSet | p95 ≤ 2 s（≤10 万对象/关系工作集）；超限转后台 |
| Action proposal 校验 | p95 ≤ 500 ms，不含模型生成 |
| Apply revision/permission 保护 | 100%；过期或越权 proposal 不得写入 |
| 审计完整率 | 100% 写操作具有 actor、依据、审批、receipt、最终 revision |
| Workflow 幂等 | 相同 event + workflow version 重放的重复副作用数 = 0 |

## 12. 质量和安全指标

| 领域 | 发布目标 |
|---|---|
| 数据隔离 | 跨 workspace 资源/对象/关系/搜索泄露测试 100% 通过 |
| Agent 可追溯 | 影响与动作建议 100% 具有对象或 Anchor 来源；纯建议需标识推断 |
| Claim 质量 | 按对象类型分别评估；P0 不设置一个虚假的总准确率，首发需人工确认 |
| Impact 质量 | 显式关系场景 recall = 100%；模型候选误报/漏报单独监控 |
| 写入安全 | 未批准高风险动作数 = 0；旧 revision 覆盖数 = 0 |
| Web 隔离 | CSP/sandbox/origin/nonce/路径穿越/恶意 MIME 回归全部通过 |
| 可访问性 | Shell、Lens、Object、Impact、Approval 核心路径达到 WCAG 2.2 AA |
| 恢复性 | Provider 超时、Surface 崩溃、部分失败、事件重放演练通过 |

## 13. 产品指标与埋点

### 13.1 北极星与护栏指标

北极星指标：**每周完成的“可追溯知识行动”数**。一次行动必须从对象/Anchor/ChangeSet 发起，获得明确 ActionRun 结果；普通打开和 Agent 对话不计入。

| 类别 | 指标 | 定义 |
|---|---|---|
| 激活 | 首次知识闭环时间 | 新 workspace 从首次打开到首个 Object + Anchor + Action/Impact 完成 |
| 使用 | 跨载体追溯任务完成率 | 用户成功从对象到证据或从证据到对象 |
| 传播 | 关键影响按 SLA 处置率 | 已确认高/中风险 Impact 在期限内完成/豁免/拒绝 |
| 质量 | Agent proposal 采纳率 | 原样或修改后批准；按 ActionType/风险分层 |
| 质量 | proposal 大幅返工率 | 用户改动超过阈值或因证据错误拒绝 |
| 信任 | 可解释路径查看率 | 审批前查看 evidence/path/diff 的比例 |
| 护栏 | 自动化误动作率 | 执行后被回滚、补偿或标记错误的比例 |
| 护栏 | 通知噪声 | 每个有效处置对应的通知数、mute/ignore 比例 |
| 护栏 | 跨端失败率 | handoff、移动审批、Web Viewer 的 terminal failure |

### 13.2 核心事件

事件只记录安全 ID、类型和低基数属性，不记录原文、路径和密钥：

- `resource_open_requested / routed / ready / fallback / failed / cancelled`
- `surface_focused / closed / crashed / recovered`
- `anchor_created / resolved / relocated / ambiguous / orphaned`
- `object_viewed / relation_traversed / claim_reviewed`
- `changeset_confirmed / impact_generated / impact_dispositioned`
- `action_proposed / approval_submitted / action_started / action_terminal`
- `workflow_simulated / enabled / suspended / terminal`
- `handoff_created / received / expired / failed`

所有耗时事件关联 trace ID、端、placement、格式类别、Adapter/Provider 版本、资源档位和结果码。不得把 policy denied 排除后宣称 100% 成功；应分别报告合法能力请求、策略拒绝和系统错误。

## 14. 通知、权限和异常产品规则

### 14.1 通知

- 连续保存按对象和 quiet period 合并为一个 Impact Bundle。
- 只通知 owner、审批者、订阅者或 SLA escalation 对象，不广播全部成员。
- Mobile push 默认脱敏；高敏 workspace 只提示“有一项待处理事项”。
- 同一 root cause 的 Workflow 不重复通知；状态未变化不再次推送。

### 14.2 权限

- 能打开载体不自动拥有对象全部字段；能查看对象不自动拥有所有证据资源。
- 禁用按钮应解释 submission criteria，而不是只显示“无权限”。
- approval 是有范围、有期限的授权，不等于角色永久升级。
- 权限撤销后，缓存和当前 Surface 的敏感内容必须同步失效。

### 14.3 通用错误行为

| 错误 | 用户文案目标 | 下一步 |
|---|---|---|
| Revision conflict | 内容在审阅后已更新，旧批准已失效 | 查看最新 diff、重新生成 proposal |
| Capability unavailable | 当前设备/端只能预览或不支持此格式 | Viewer、下载、系统打开、Desktop handoff |
| Anchor ambiguous | 找到多个可能的新位置 | 展示候选，人工选择 |
| Policy denied | 说明可执行范围与所需角色 | 请求权限、交给 owner |
| Partial apply | 明确已完成和未完成步骤 | 重试幂等步骤、补偿、恢复任务 |
| Provider offline | 展示最后安全缓存的 revision/时间 | 重连、转其他 Host、稍后处理 |

## 15. 版本范围和发布门

### Release A：统一打开与资源基础

范围：RES-01～06、SUR-01/02/05，Desktop + Web/Mobile 合法降级。

发布门：

- Word、Markdown、普通 workspace file 从三个 DSH 入口走统一 route。
- macOS Word/Markdown golden path 达到分端 SLO；Windows/Linux/Web/Mobile 不虚报编辑能力。
- permission denied、未知格式、Adapter crash、超时和取消具有唯一 receipt。
- 跨 workspace 和任意 path 注入安全测试通过。

### Release B：知识对象与影响闭环

范围：ONT-01～05、IMP-01～03、ACT-01～03、AGT-01/02。

发布门：

- Requirement → CodeSymbol → TestCase → Anchor 可追溯。
- 产品需求变化场景可生成有证据 ImpactSet；各责任人可独立审批。
- Agent Claim 与人工确认事实在 UI/API/搜索中严格区分。
- 过期权限、revision conflict 和部分失败端到端演练通过。

### Release C：多引擎、多端接续与 Workflow

范围：安全 Viewer、Helix、Handoff、Workflow、插件治理和运行看板。

发布门：

- 新示例格式无需核心代码修改即可接入并通过 Adapter TCK。
- Mobile 审批和 Desktop handoff 达到 SLO；敏感 push/离线缓存评审通过。
- Workflow 影子模式、幂等、去抖、fan-out、dead-letter 和恢复测试通过。
- 自动写入仍按 ActionType/风险逐项开放，不随 Release C 全局启用。

## 16. 依赖与风险

| 依赖/风险 | 产品影响 | 处理 |
|---|---|---|
| DSH 缺正式 OpenResource seam | 默认点击仍绕过 Muse | Release A 前完成 consumer/provider spike，旧 Sidebar 作 fallback |
| Desktop 缺 Host → Flutter 通道 | 本地 DSH 无法创建 Surface | 冻结中立 Host Bridge transport，禁止临时业务 channel 扩散 |
| ioffice 跨平台能力不足 | 各端 Word 编辑不一致 | 诚实能力矩阵；macOS 首发，其他端 Viewer/系统打开 |
| Anchor 稳定性不足 | 证据误绑定，破坏信任 | 格式 TCK、revision/fingerprint、ambiguous 人审 |
| Ontology 过度建模 | 上手慢、数据质量差 | 首批具体对象；按场景扩展；禁止 God Object/relatedTo 兜底 |
| Impact 通知噪声 | 用户关闭系统 | 去抖、责任路由、风险阈值、按 root cause 聚合 |
| Agent 误改下游 | 安全与信任风险 | propose-first、证据、原生 diff、审批、apply-time 重校验 |
| 大文件/移动端资源限制 | 卡顿或崩溃 | range/stream、端侧上限、首屏优先、Desktop handoff |

## 17. 验收样例

### 17.1 Release A 样例

```gherkin
Given 用户在 macOS Desktop 有权访问一个 Word resourceRef
And ioffice Word Adapter probe 返回 edit 可用
When 用户从 DSH 内联引用点击该资源
Then 100 ms 内显示加载反馈
And Host 选择 Word Surface 并产生 route receipt
And Surface 定位请求的 Anchor
And 不向 DSH 暴露本地路径
```

```gherkin
Given 用户在 Mobile 点击同一 Word resourceRef
And Mobile 没有原生 Word 编辑能力
When Host 完成路由
Then 使用只读 Viewer 或 metadata fallback
And 明确显示“在桌面继续”
And 不显示不可执行的“编辑”按钮
```

### 17.2 Release B 样例

```gherkin
Given Requirement R42 由 Word Anchor A1 表达
And CodeSymbol C7 implements R42
And R42 verifiedBy TestCase T9
When 产品经理保存已确认的验收标准变化
Then 系统生成包含 C7 与 T9 的 ImpactSet
And 每项显示关系路径与证据 revision
And Agent 只能生成 proposal，不能直接写代码或测试
And 旧 revision 的批准无法 apply
```

## 18. 未决产品问题

以下问题不阻塞统一资源基础，但必须在相应 Release 前完成用户研究或 spike：

1. Knowledge Lens 默认展开哪些信息，才能兼顾理解速度与编辑空间？
2. 什么风险/确定性组合允许 workspace 开启自动批准？默认仍为人工批准。
3. Mobile 的轻量 Anchor 创建首发支持哪些来源：文本、拍照、语音或分享扩展？
4. Desktop 多 Surface 的默认布局、资源复用与“在新窗口打开”规则。
5. Impact SLA 应由 ObjectType、risk 还是 Release 阶段共同决定？
6. Windows/Linux Word 编辑是接入新引擎、远端编辑，还是长期只读/系统打开？
7. 对外分享的是 Artifact、Object View 还是受权限裁剪的 Perspective？

默认决策和技术待验证项见 [11-decisions-and-open-questions.md](11-decisions-and-open-questions.md)，完整协议流程见 [10-end-to-end-scenarios.md](10-end-to-end-scenarios.md)。
