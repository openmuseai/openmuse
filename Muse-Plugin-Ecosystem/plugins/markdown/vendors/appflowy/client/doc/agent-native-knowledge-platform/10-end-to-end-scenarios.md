# 端到端场景与协议走查

状态：**目标行为 v1**；用于产品评审、架构走查和 E2E fixture 设计。

## 1. 场景 A：产品需求变化联动开发与测试

### 1.1 前置状态

- `Requirement R42`：“支付失败自动重试两次”。
- Word PRD 段落 `Anchor A1 representedBy R42`。
- `CodeSymbol C7 implements R42`，指向 `retryPayment()`。
- `TestCase T9 verifiedBy` 关系覆盖最大重试次数。
- `Decision D3` 规定只允许指数退避。
- 产品经理可修改 PRD；开发/测试负责人分别审批各自资源写入。

### 1.2 用户动作与系统序列

```text
PM edits Word paragraph and saves
  Word Adapter → Resource Provider: commit(baseRevision=31)
  Resource Provider → Event Bus: resource.changed(31 → 32, A1)
  Change Interpreter → Snapshot: read minimum before/after anchor
  Extractor → Claims Review: proposed semantic diff
  PM/Rule → Ontology: confirm ChangeSet CS8
  Impact Engine → Ontology: ImpactSet IS8
  Agent → Action Runtime: propose code/test/dev-doc actions
  Owners → Approval Runtime: approve selected proposals
  Executor → Providers: apply with current revisions
  Verifier → Ontology/Event Bus: receipts + postconditions
```

### 1.3 ImpactSet 示例

| 目标 | 路径 | 严重度 | 建议 |
|---|---|---|---|
| `C7 retryPayment()` | `C7 implements R42` | 高 | 审查循环次数与 backoff |
| `T9 retry exhaustion` | `R42 verifiedBy T9` | 高 | 更新期望次数并新增间隔断言 |
| 开发设计 §4.2 | `R42 representedBy A6` | 中 | 更新算法说明 |
| `Release 2.4` | `Feature F2 releasedIn 2.4` | 中 | 重新运行 readiness check |

每项展示关系路径、规则版本和证据。向量检索发现的“可能相关日志告警”放在候选区，不混入确认影响。

### 1.4 冲突分支

测试负责人审阅期间，T9 已被另一个提交修改。Apply 返回：

```json
{
  "code": "revision-conflict",
  "expected": "git:abc123",
  "actual": "git:def456",
  "proposalRef": "proposal:p9"
}
```

系统不得套用旧批准。Action 回到 `proposed` 派生一个新版本，保留旧审计，Agent 基于最新内容重新生成 diff，负责人重新批准。

### 1.5 验收断言

- 保存文件不等于批准语义解释；高歧义变化进入 Claims Review。
- 代码、测试、文档可分别批准/拒绝。
- 未审批的代码不被自动修改；部分成功明确显示。
- R42、C7、T9 和各 revision 可形成完整追溯链。

## 2. 场景 B：DSH 交付物无格式特权打开

Agent 在消息中产生 `report.docx`、`trace.log` 和 `model.step` 三个交付物。DSH 只持有三个 resourceRef。

1. 用户点击 Word：Host describe 为 OOXML Word，ioffice Adapter probe 可编辑，路由到 Word Surface。
2. 用户点击 log：Helix 可编辑且是文本偏好；Desktop 有 PTY，因此路由到 Helix。
3. 用户点击 STEP：尚无 CAD Adapter；Viewer 也不支持，降级到 metadata，提供下载/系统打开。

三个入口调用完全相同的 Presentation 合同。差异来自 Provider 能力、Adapter probe、端 placement 和策略，不来自核心扩展名 switch。

路由 receipt 示例：

```json
{
  "requestId": "preq_77",
  "resourceRef": "muse-resource:opaque",
  "selectedAdapter": "muse.ioffice.word/v1",
  "mode": "edit",
  "reasons": ["format-exact", "native-edit", "desktop-available", "user-preference"],
  "alternatives": [
    { "adapter": "muse.open-file-viewer/v1", "mode": "view", "score": 61 }
  ]
}
```

验收断言：ProducedFile、inline mention 和 card preview 点击同一资源得到一致结果；模型不能在 payload 中强制 `selectedAdapter`。

## 3. 场景 C：新增 CAD/3D 格式不修改核心

第三方发布：

- `CadResourceProvider`：describe/snapshot/materialize/subscribe。
- `StepFormatProvider`：识别 STEP，提供 metadata 和 entity anchor schema。
- `CadSurfaceAdapter`：支持 `view | annotate`，Desktop/Web placement。
- `CadOntologyContributor`：可选，把零件提议为 `Component`，关系为 `usedIn`。

安装后流程：

1. Plugin Graph 验证 manifest、签名、合同版本与权限。
2. Registry 注册 contributions；Surface Orchestrator 的候选集自动出现 CAD Adapter。
3. 打开 `resourceRef` 时 Adapter 请求 loopback stream，不取得原始云凭据。
4. 用户选择实体 `part-884`，Provider 创建 `cad-entity` Anchor。
5. Agent 可将该 Anchor 关联到 Requirement/Risk，但没有 Ontology contribution 也不影响查看。

验收断言：Resource、Presentation、Ontology 核心包均无需添加 `.step`、CAD vendor 名或专用 if/else；卸载插件后安全降级，既有 Anchor 标记 provider-unavailable 而非删除。

## 4. 场景 D：手机审阅，桌面继续编辑

用户在 Mobile Impact Inbox 打开受影响代码：

1. Presentation request 指定 `mode=view`，Mobile Host 没有 Helix PTY。
2. Orchestrator 选择只读 code viewer，提示“桌面端可编辑”。
3. 用户点击“在我的桌面继续”；系统选择其已授权在线设备，并发送短期 handoff ref，而非本地路径。
4. Desktop Host 重新 resolve/鉴权，路由到 Helix，定位同一 Code Anchor。
5. 若设备离线，创建有过期时间的待处理 handoff；过期后需重新发起。

验收断言：Mobile 不宣称能本地启动 Helix；Desktop 不盲信 Mobile 的权限决定；Anchor 和 object context 连续。

## 5. 场景 E：权限撤销发生在审阅期间

用户打开机密设计并生成修改 proposal，随后被移出项目：

- Host 发出 policy/resource capability revocation。
- Surface 清空正文和敏感缓存，保留无泄露错误状态。
- 搜索、Knowledge Lens、Ontology 邻接和模型上下文立即过滤对象。
- 旧 approval/proposal 在 apply 时被拒绝；不能因已有 Surface lease 继续写。
- 审计保留 opaque target 与决策，不向该用户显示被撤销内容。

验收断言：权限在 UI、缓存、Context Broker、Action Executor 和索引查询全部一致，没有标题/计数侧信道。

## 6. 场景 F：Anchor 漂移与重定位

Word 中有人把需求段落移动并改写措辞：

1. 原 Anchor 的 paragraph ID 不存在，Provider 使用结构路径、quote 和 fingerprint 搜索。
2. 仅一个高确定性候选时返回 `relocated`，创建新 selector/revision，保留重定位事件。
3. 两个相似段落时返回 `ambiguous`，UI 展示候选上下文，请用户选择。
4. 无候选时为 `orphaned`；知识对象仍存在，但自动化不得引用该证据作为当前事实。

验收断言：系统不会因文本相似就静默把需求绑到错误段落；所有使用陈旧 Anchor 的 Action 在提交前失败或要求复核。

## 7. 场景 G：自动化风暴被阻断

Workflow A 更新开发文档，文档变化又触发 Workflow A：

- 事件携带 `causalChain=[CS8, WF-A/run-1]` 和 depth。
- Runtime 的 `(workflowVersion, objectRef, rootCause)` 幂等键识别重复。
- quiet period 合并连续保存；max depth/fan-out 阻止循环。
- 超限进入 suspended/dead-letter，通知 workflow owner，不继续发送普通用户噪声。

验收断言：停止条件可复现、可观测；恢复时从 receipt/事件重放，不重复已完成 effect。

## 8. 场景 H：外部系统部分失败

一次已批准 Action 要更新 Word、提交 Git patch、更新测试管理系统：Word 和 Git 成功，测试系统超时。

- Runtime 不报告整体成功。
- 已完成步骤的 Provider receipt 与 revision 被保留。
- 如果 Word/Git effect 声明安全补偿，则提出补偿；默认不自动删除真实提交。
- 创建人工恢复任务，包含失败步骤、重试是否幂等、已完成事实和下一步。
- Release readiness 保持 `blocked-by-incomplete-action`。

验收断言：UI 和 Agent 使用“部分完成”而非含糊的“出错”；重试只执行未完成且幂等的步骤。

## 9. 场景到自动化测试映射

| 场景 | Fixture | 核心断言 |
|---|---|---|
| A 需求联动 | Word revisions + ontology graph + Git/Test mocks | evidence、impact、approval、conflict |
| B 统一打开 | 三类 Resource descriptors + adapter registry | route 一致性、fallback、模型无引擎权 |
| C CAD 插件 | 动态 manifest + mock stream/entity | 核心零改动、卸载降级 |
| D 跨端 | Mobile/Desktop Host capability matrices | placement 与重新鉴权 |
| E 撤权 | policy revision event | 全链过滤、旧批准无效 |
| F Anchor | before/after document fixtures | resolved/ambiguous/orphaned |
| G 风暴 | recursive event sequence | debounce、depth、dead-letter |
| H 部分失败 | saga provider receipts | 不伪报、恢复、幂等 |

