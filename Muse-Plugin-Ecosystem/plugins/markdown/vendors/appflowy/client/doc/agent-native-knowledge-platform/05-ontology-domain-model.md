# Ontology 领域模型

状态：**目标设计 v1**；当前代码尚无 Ontology Runtime，本文定义首个可实现基线。

## 1. 目标与边界

Ontology 的职责不是保存另一份 Office 文档，也不是给全部数据套一层“知识点”标签。它把真实工作世界中稳定、有业务含义、可被行动的事物建模出来，并把它们连接到文档、代码、数据和应用中的证据。

它回答四类问题：

1. **是什么**：需求、功能、决策、代码符号、测试义务、风险、发布等对象的定义与当前状态。
2. **如何关联**：谁实现谁、谁验证谁、谁依赖谁、证据在哪里。
3. **可以做什么**：哪些 Action 可对对象执行，前置条件、权限和副作用是什么。
4. **发生了什么**：谁依据什么证据提出、批准和执行了改变，影响如何传播。

Ontology 不负责像素渲染、文件字节存储、编辑器选择或实时协同算法。这些分别属于 Resource Fabric、Surface/Engine 与各真源系统。

## 2. 三层结构

```text
Language / Schema
  ObjectType · Interface · PropertyType · LinkType · ActionType · Policy
                         │ 实例化
                         ▼
Operational Ontology
  Requirement · Decision · CodeSymbol · TestCase · Artifact · Claim · Link
                         │ 产生/约束
                         ▼
Kinetic Runtime
  ChangeSet · ImpactSet · ActionRun · AutomationRun · Approval · AuditEvent
```

- **Semantic layer**：对象、属性、关系、接口和证据，表达“世界是什么”。
- **Kinetic layer**：动作、函数、自动化和变更集，表达“世界如何改变”。
- **Governance layer**：身份、权限、策略、保留、审计，贯穿前两层。

这借鉴了 Palantir Ontology 将语义与行动统一的公开理念，但不复制其产品 API、存储方案或命名体系。

## 3. 关键区分

### 3.1 Resource、Artifact、Knowledge Object

| 概念 | 含义 | 示例 |
|---|---|---|
| `Resource` | 可被 Host 发现和访问的外部/本地资源；由不透明 `resourceRef` 指向 | 一个 docx、Git 文件、AppFlowy view、S3 对象 |
| `Artifact` | 有版本、作者、生命周期的工作成果；可由一个或多个 Resource 表现 | “支付重试 PRD”、某版本测试报告 |
| `KnowledgeObject` | 业务世界中的语义对象接口，不作为万能存储类型 | `Requirement`、`Decision`、`CodeSymbol` |
| `Anchor` | Resource 某个稳定片段的引用 | Word 段落、表格单元格、函数、CAD 零件 |
| `Claim` | 对对象或关系的可审核陈述 | “函数 X 实现需求 R”，置信度 0.81 |

同一 Artifact 可更换 Resource 位置；同一 Knowledge Object 可由多个 Artifact/Anchor 共同表达；删除某个文件不等于删除业务对象。

### 3.2 “知识点”不是 God Object

产品界面可继续使用易懂的“知识点”一词，但协议和存储层不得只定义一个带任意 JSON 的 `KnowledgePoint`。首批对象必须是具体业务类型，并通过 Interface 共享行为：

- `WorkItem`：有负责人、状态、截止日期。
- `Traceable`：可关联来源、证据和派生关系。
- `Reviewable`：可提出、批准、拒绝或要求修改。
- `Releasable`：可归属发布版本并执行就绪检查。
- `Executable`：声明可以执行的动作，而不是表示任意代码执行。

Interface 提供多态查询和动作约束，不制造深继承树。

## 4. 首批对象类型

### 4.1 核心工作对象

| ObjectType | 关键属性 | 典型 Action |
|---|---|---|
| `Requirement` | 标题、正文摘要、状态、优先级、验收标准、负责人 | 澄清、接受、变更、废弃 |
| `Feature` | 目标、范围、状态、发布目标 | 规划、拆分、发布 |
| `Decision` | 问题、结论、理由、状态、决定时间 | 提议、批准、取代 |
| `CodeSymbol` | 仓库引用、语言、限定名、符号种类、当前 revision | 打开、请求修改、标记实现 |
| `TestObligation` | 要验证的行为、风险级别、状态 | 分配、满足、豁免 |
| `TestCase` | 步骤/自动化入口、结果、覆盖状态 | 运行、更新、关联证据 |
| `Risk` | 概率、影响、缓解措施、状态 | 接受、缓解、升级 |
| `Release` | 版本、范围、时间、就绪状态 | 加入范围、检查、发布 |
| `WorkflowDefinition` | 触发器、条件、动作、审批、版本 | 启用、停用、演练 |

### 4.2 证据与运行对象

| ObjectType | 作用 |
|---|---|
| `Artifact` / `ArtifactRevision` | 表示工作成果和不可变版本；连接到 `resourceRef` |
| `Anchor` | 记录格式相关 selector、引用时 revision、片段指纹与回退定位 |
| `Claim` | 保存主语、谓语、宾语/值、来源、提议者、置信度和审核状态 |
| `ChangeSet` | 一组原子或协调提交的语义/资源变更 |
| `ImpactSet` | 某变更推导出的直接/间接影响、原因路径、严重度和处置状态 |
| `ActionRun` | 一次动作从 proposed 到 terminal state 的完整记录 |
| `AutomationRun` | 触发、去抖、规则版本、动作运行和失败处理 |

## 5. 关系模型

关系必须使用方向明确、可读的动词，并声明合法的源/目标 Interface。首批关系：

| LinkType | 方向示例 | 含义 |
|---|---|---|
| `representedBy` | Requirement → Anchor | 哪段载体表达该对象 |
| `evidencedBy` | Claim → Anchor | 陈述的证据位置 |
| `implements` | CodeSymbol → Requirement | 代码实现需求 |
| `verifiedBy` | Requirement → TestCase | 需求由测试验证 |
| `dependsOn` | Feature → Feature | 业务/技术依赖 |
| `affects` | ChangeSet → KnowledgeObject | 已计算的影响目标 |
| `supersedes` | Decision → Decision | 新决策取代旧决策 |
| `contradicts` | Claim → Claim | 两项声明存在冲突 |
| `ownedBy` | WorkItem → Actor/Team | 处置责任 |
| `releasedIn` | Feature → Release | 发布范围 |
| `generatedBy` | ArtifactRevision → ActionRun | 产物的行动来源 |

禁止使用无语义的 `relatedTo` 作为默认兜底。确实未知的关系先保存在待分类 Claim 中，不污染已确认图谱。

## 6. Anchor：知识与载体之间的桥

Anchor 是跨格式协议，但 selector 由格式 Provider 解释：

```json
{
  "anchorId": "anc_01J...",
  "resourceRef": "muse-resource:opaque-token",
  "revision": "sha256:...",
  "selector": {
    "kind": "text-range",
    "provider": "muse.word-anchor/v1",
    "value": { "paragraphId": "p-184", "start": 12, "end": 46 }
  },
  "quote": "失败后最多自动重试三次",
  "fingerprint": "sha256:...",
  "state": "resolved"
}
```

首批 selector：

- Markdown：AST node ID + text range + quote。
- Word：稳定段落/控件 ID + run range；无稳定 ID 时用 OOXML part、结构路径和指纹组合。
- Code：repository ref + commit/revision + language symbol ID + range。
- Table：sheet/table ID + row/column identity，避免只依赖易漂移的 `A1`。
- PDF：page + bounding box + normalized quote，仅可注释而非直接编辑。
- CAD/3D：provider namespace + entity/component ID + model revision。

解析状态为 `resolved | relocated | ambiguous | orphaned | inaccessible`。Provider 可重定位 Anchor，但不能静默改变其语义目标；模糊结果必须交由人或 Agent 审核。

## 7. Claim 与真实性

Agent 抽取的内容不能直接成为已确认事实。Claim 生命周期：

```text
proposed → confirmed → superseded
    ├────→ rejected
    └────→ expired / needs-review
```

每个 Claim 至少记录：

- `subject`、有类型的 `predicate`、`object/value`；
- 来源 Anchor 与读取时 revision；
- `assertedBy`（人、Agent、导入器或规则）及时间；
- 置信度、抽取器/模型版本和推理摘要；
- 审核状态、审核者及取代链；
- 可见性/数据分类标签。

UI 必须区分“文档明确写明”“规则推导”“模型推测”和“人工确认”。置信度不是权限，也不能替代审批。

## 8. ActionType 模型

Action 是对真实世界的受治理改变。每个 `ActionType` 定义：

| 字段 | 说明 |
|---|---|
| `inputSchema` | 有类型参数，引用必须来自发现或当前上下文 |
| `targets` | 可作用的 ObjectType/Interface |
| `submissionCriteria` | 提交前动态校验，如状态、角色、字段范围、工作区策略 |
| `readSet` / `writeSet` | 预期读取与写入对象/资源范围 |
| `approvalPolicy` | 自动、单人、多人、角色或风险分级审批 |
| `effectPlan` | Ontology 变更、Resource proposal、外部 capability 调用 |
| `postconditions` | 成功后必须满足的不变量 |
| `idempotencyScope` | 重试的去重范围 |
| `compensation` | 可补偿动作或人工恢复手册；不虚构全局事务 |

动作实例必须引用冻结的 ActionType 版本。Action 只能调用 Host 发现并授权的 capability，不能携带任意 URL、路径或命令。

## 9. 版本、分支与冲突

- Schema 使用不可变 `typeId + version`；兼容新增字段为 minor，语义/约束破坏为 major。
- Object 使用单调 `revision` 或内容摘要作为乐观并发条件；不得混用整数和 schema 要求的 sha256 token。
- Link 与 Claim 均为一等版本实体，支持 `validFrom/validTo` 与取代链。
- ChangeSet 保存 base revision；冲突返回具体对象、属性、当前值和可重放 proposal。
- 初版不提供任意 Ontology 分支。需要沙盒时使用隔离 workspace/preview graph，批准后通过 ChangeSet 合并。

## 10. 真源与投影

| 数据 | 真源 | Ontology 保存 |
|---|---|---|
| Word/Markdown 正文 | AppFlowy Collab 或文件 Provider | Artifact、revision、Anchor、Claim，不复制完整正文 |
| 代码 | Git repository | Repo/commit/symbol 引用、语义关系、必要摘要 |
| 测试运行 | CI/Test provider | TestCase identity、状态与运行证据引用 |
| 人员/组织 | Workspace identity provider | 稳定主体引用、最少展示字段 |
| Action/Audit | Muse Ontology Runtime | 完整事件与投影 |

查询时可以联邦读取，也可以构建受策略约束的搜索/向量派生索引。索引是可重建投影，不成为事实真源。

## 11. 存储与查询建议

MVP 不应因“图”字样立即引入专用图数据库。推荐：

1. 不可变事件日志保存对象、关系、Claim、Action 的事实变化。
2. 关系型投影保存当前对象/关系、租户与权限，利用事务和成熟迁移能力。
3. adjacency/全文/向量索引作为可重建读模型。
4. Local-first 部署可用嵌入式数据库，同一合同连接 Cloud 投影。
5. 只有多跳查询规模与延迟经基准证明关系型方案不足时，再在 Provider 后增加图存储。

首批查询能力：按 ID 取对象、按 Interface/属性筛选、邻接关系、限定深度的路径、Anchor 反查、全文/语义检索、ImpactSet 查询。禁止无界图遍历。

## 12. Schema 治理

新增 ObjectType/LinkType/ActionType 必须提交 schema proposal，包含：业务定义、负责人、真源、例子/反例、权限、保留策略、迁移和质量规则。评审重点：

- 是否在建模真实业务概念，而不是来源系统的表名或部门缩写；
- 是否与现有类型重复，是否应实现 Interface；
- 是否演化为“厨房水槽”或 God Object；
- 关系方向与基数是否清楚；
- 动作是否有最小写集合、审批和失败语义；
- 是否存在可测量的数据质量责任人。

每个 schema 版本提供 fixture、contract test 和迁移演练。废弃先停止新写、迁移读者、验证引用，再进入只读保留期。

## 13. MVP 验收

- 可创建 `Requirement`、`Decision`、`CodeSymbol`、`TestCase`、`Artifact`、`Anchor` 与有类型关系。
- Word/Markdown 段落与代码符号可作为 Anchor，revision 漂移可检测。
- Agent 抽取只产生 proposed Claim；人工确认后才参与高风险自动化。
- 从一个 Requirement 可解释地查询到实现代码和验证用例，返回关系路径与证据。
- Object、Link、Claim、Action 均满足 workspace 隔离、字段级可见性和审计要求。
- schema fixture 能阻止无效关系、无来源 Claim 和绕过 approval 的 Action。

