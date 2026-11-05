# Agent、Action 与 Workflow Runtime

状态：**目标设计 v1**；复用现有 Document proposal、Host policy/event 与 Facet 生命周期，新增 Ontology/Workflow 合同。

## 1. 核心立场

Agent 是 Ontology 上受权限与策略约束的协作者，不是拥有所有数据和工具的超级用户。它的主要价值是把分散载体中的变化转成可解释的语义变化，计算影响，生成可审核的行动方案，并在批准后调用真实系统能力完成回写。

```text
Observe → Ground → Propose → Evaluate → Approve → Apply → Verify → Learn
  观察      取证       提议        评估       审批       执行      验证      反馈
```

任何阶段都要能回答：基于什么 revision、引用了哪些证据、用了哪个规则/模型、谁授权、改了什么、结果是否满足后置条件。

## 2. 运行组件

| 组件 | 职责 | 不负责 |
|---|---|---|
| Context Broker | 按任务和权限选择对象、Anchor、片段与 capability | 无界抓取整个工作区 |
| Semantic Extractor | 从受支持的快照提出 Claim、对象和关系候选 | 直接确认事实 |
| Change Interpreter | 将 Resource event/domain change 解释为语义差异 | 把每次按键当业务事件 |
| Impact Engine | 沿有类型关系和规则计算 ImpactSet | 凭相似度自动改下游 |
| Action Planner | 生成有类型 Action proposal 与依赖图 | 发明未注册工具或 ID |
| Policy/Approval Engine | 执行 submission criteria、风险分级和审批 | 用模型置信度代替权限 |
| Action Executor | 幂等调用 Resource/Ontology/外部 capability | 绕过 Host authority |
| Workflow Runtime | 触发、去抖、编排、等待、重试、补偿 | 承诺跨系统 ACID 事务 |
| Verification/Eval | 验证后置条件、质量与人类反馈 | 静默吞掉部分失败 |

## 3. 从资源变化到业务变化

### 3.1 两级事件

资源层事件只描述事实：`resource.changed`、`resource.moved`、`resource.deleted`、`anchor.invalidated`。语义层事件描述业务：`requirement.acceptanceCriteriaChanged`、`decision.superseded`。

转换流程：

1. Provider 提交带 revision 的 `resource.changed`，仅包含安全元数据和可选 change summary。
2. Change Interpreter 判断是否达到语义解析阈值；自动保存/游标移动只做去抖，不运行 Agent。
3. Context Broker 通过授权 snapshot/anchor 能力读取最小片段。
4. Extractor 生成 proposed Claim 和候选语义 diff，引用 before/after revision 与 Anchor。
5. 确定性规则先验证；不确定项进入人工 review queue。
6. 确认的变化形成 `ChangeSet`；Impact Engine 基于冻结的 schema/rule 版本计算 `ImpactSet`。

### 3.2 ChangeSet

```json
{
  "changeSetId": "chg_01J...",
  "workspaceRef": "ws:opaque",
  "cause": {
    "kind": "resource-change",
    "resourceRef": "muse-resource:opaque-token",
    "beforeRevision": "sha256:old",
    "afterRevision": "sha256:new"
  },
  "changes": [
    {
      "op": "replace-property",
      "objectRef": "ont:requirement/r-42",
      "property": "acceptanceCriteria",
      "before": "重试两次",
      "after": "重试三次且采用指数退避",
      "evidenceAnchorRef": "ont:anchor/a-9"
    }
  ],
  "assertedBy": "actor:user-or-agent",
  "state": "proposed"
}
```

大型正文 diff 不放在事件中，只保存受权限控制的 Resource/Revision/Anchor 引用。

## 4. Impact Engine

Impact 不是“向量相似的都通知”。计算分四层：

1. **确定性直接边**：如 `Requirement verifiedBy TestCase`。
2. **类型规则**：例如验收标准改变会使相关测试义务进入 `needs-review`。
3. **受限路径**：沿允许的 LinkType、方向、最大深度和 workspace 范围遍历。
4. **语义候选**：相似内容只能补充候选，必须注明低可信原因，不能自动执行高风险动作。

Impact item 包含目标、严重度、关系路径、规则版本、证据、建议动作、责任人和处置状态。合并/排序依据是业务风险与依赖路径，不只是模型分数。

```text
Requirement R42 changed
 ├─ implements ← CodeSymbol retryPayment()       [review code]
 ├─ verifiedBy → TestCase TC-17                  [update expected attempts]
 ├─ dependsOn  ← Feature Checkout Reliability    [re-evaluate release]
 └─ representedBy → Dev Design §4.2              [refresh wording]
```

循环关系使用 `(objectRef, ruleId, changeSetId)` 去重；限定最大深度、最大节点数和执行预算。超限返回部分结果与截断原因。

## 5. Action proposal 与执行状态机

```text
draft → proposed → validating → awaiting-approval → approved → applying
  └────────→ rejected       └──────────────→ expired      │
                                                    ┌─────┴─────┐
                                                 succeeded   failed
                                                               │
                                                    compensating/manual
```

状态变化只能由 Runtime 执行，客户端不得直接把状态写成 `approved` 或 `succeeded`。

Action proposal 必须包含：

- 固定的 `actionTypeId@version`、目标对象和输入；
- `baseRevisions` 与预计 `readSet/writeSet`；
- 人可读目的、预计影响和不可逆性；
- 证据 Anchor、Impact item 和发起者；
- `idempotencyKey`、有效期与审批策略；
- 资源修改时的 domain-specific proposal，例如 `muse.document@2` mutation。

提交时重新执行 submission criteria、权限与 revision 校验。审批令牌绑定 actor、workspace、action digest、过期时间和允许效果，沿用当前 Rust Host 的 HMAC/重校验思想，不能只相信 UI。

## 6. 写入编排

一次业务 Action 可能触及多个真源。默认使用 saga，而非伪装成分布式事务：

1. 预检全部 Provider 可用性、revision 和权限。
2. 创建不可变 ActionRun，冻结 effect plan。
3. 按依赖顺序 apply；每一步记录 provider receipt。
4. 验证后置条件并读取最新 revision。
5. 部分失败时执行已声明的补偿；不可补偿则冻结后续步骤并创建人工恢复任务。
6. 成功后提交 Ontology 投影和 result event；不得提前宣告完成。

对于必须原子的一组 Ontology 写入，由 Ontology Store 自身事务处理。对于 Word + Git + CI 等跨系统修改，只能显式呈现部分成功。

## 7. Workflow Definition

```yaml
id: workflow.requirement-change/v1
trigger:
  event: ontology.changeset.confirmed
  where: changedObject implements Requirement
debounce:
  key: "${objectRef}"
  quietPeriod: 5m
steps:
  - computeImpact:
      ruleset: requirement-impact/v3
  - createReviewBundle:
      when: impact.count > 0
  - awaitApproval:
      policy: product-and-tech-owner-if-high-risk
  - executeApprovedActions: {}
  - verifyTraceability: {}
onFailure:
  createRecoveryTask: true
```

YAML 仅用于可读示例，线上定义以有版本 JSON Schema/合同为准。Workflow 由插件贡献，但启用权属于 workspace policy。

### 7.1 自动化护栏

- 默认只读或生成提议；对正文、代码、外部系统写入默认需批准。
- 仅确定性、低风险、可逆、范围有界的动作可设自动批准。
- 触发器具备 quiet period、速率限制、最大 fan-out 与熔断。
- 同一事件和 workflow version 使用幂等键，重放不会重复写入。
- 规则/模型升级不回溯重写历史；显式运行 migration/re-evaluation。
- 禁止 workflow 自触发无限循环；写入事件携带 causal chain 与 depth。

## 8. Agent 可发现工具面

Agent 只看到与会话、Actor、workspace 和当前对象相关的窄工具：

| Capability | 说明 |
|---|---|
| `ontology.searchObjects` | 类型、Interface、属性和权限限定的搜索 |
| `ontology.getNeighborhood` | 有界关系路径与证据 |
| `resource.getSnapshot` | 指定 revision/selector 的最小快照 |
| `resource.resolveAnchor` | 检查/重定位 Anchor，不返回任意 Host 路径 |
| `impact.compute` | 按固定 ruleset 计算并返回解释路径 |
| `action.propose` | 对注册 ActionType 生成 proposal |
| `action.getStatus` | 读取运行/审批状态 |
| `presentation.request` | 请求展示对象/资源，不能指定未经授权引擎 |

工具输出进入现有 model-visible logging：摘要、引用、截断、策略决定和错误对模型可见；密钥、真实路径和不可见字段绝不进入 prompt。

## 9. Context Broker 与预算

Context bundle 由以下部分构成：

- 当前选中对象/Anchor 和用户明确引用；
- 必要的邻接对象与关系路径；
- 指定 revision 的短片段，而非完整二进制；
- 可执行 Action 的 schema、风险和审批提示；
- 每项内容的来源、时间、权限标签和 token/byte 成本。

Broker 必须支持 workspace/actor 过滤、数据分类、去重、优先级、token/byte 限额、截断说明和 TTL。Context Facet 继续用于插件贡献界面上下文，但 Ontology Context Bundle 需要独立、有版本的合同，避免把任意对象塞入 `metadata`。

## 10. 产品需求联动示例

产品经理把 Word PRD 中“失败重试两次”改为“最多三次，指数退避”：

1. Word Provider 在保存成功后发出 `resource.changed(old,new)`。
2. Extractor 发现 Anchor A 对应 `Requirement R42.acceptanceCriteria`，提出语义 diff。
3. 人工确认或已有严格结构规则确认 ChangeSet。
4. Impact Engine 通过 `implements` 找到 `retryPayment()`，通过 `verifiedBy` 找到 `TC-17`，通过 `representedBy` 找到开发设计段落。
5. Agent 生成三项 Action proposal：代码变更建议、测试用例变更草案、开发文档替换；每项展示证据和 base revision。
6. 产品、开发、测试负责人按策略审批各自范围。未审批项不影响其他可独立项。
7. Executor 调用 Git/Document/Test Provider；冲突项回到 rebase/review，而非覆盖新内容。
8. Verification 检查实现、测试与需求之间的 traceability，并将 Release readiness 更新为待复核或通过。

“联动”因此是可解释的影响与受治理动作，不是跨文件的盲目字符串替换。

## 11. 评估与学习

Runtime 至少记录并评估：

- Claim 抽取 precision/recall、人工接受率与纠错原因；
- Impact 漏报率、误报率、平均处置时间和路径解释覆盖率；
- Action proposal 接受/修改/拒绝率、冲突率、回滚率；
- Workflow 重复触发、fan-out、失败与人工恢复率；
- Context 引用命中率、过期引用率、每任务 token/byte 成本；
- 按 object/action/risk 类型切片，不能只看总平均。

人类修改提议形成结构化反馈，但不得未经治理直接训练或跨 workspace 使用。规则与 prompt/model 变更走离线回放、影子运行、灰度和回滚。

## 12. MVP 验收

- Word/Markdown 中一个已确认 Requirement 变化可生成带证据的 ChangeSet。
- ImpactSet 能找到已显式关联的 CodeSymbol/TestCase，并解释每条路径。
- Agent 只能提出注册 Action；修改文档必须走现有 `propose/apply` 或等价 Resource transaction。
- revision 冲突、权限撤销和审批过期均在 apply 时重新校验。
- Workflow 重放不产生重复写入，循环和 fan-out 可被预算终止。
- 部分失败对用户、模型和审计均可见，并给出安全恢复路径。

