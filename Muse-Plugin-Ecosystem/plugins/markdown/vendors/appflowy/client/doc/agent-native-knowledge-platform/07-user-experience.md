# 用户体验与信息架构

状态：**目标产品设计 v1**；覆盖 Desktop、Web、Mobile，按端能力降级。

## 1. 体验目标

用户不需要先判断“这是什么文件、该用哪个插件”。他应该围绕任务和知识对象工作：打开任何载体、定位到证据、理解上下游影响、让 Agent 提出行动，再由合适的人批准和执行。

五项体验原则：

1. **一次点击表达意图，不表达实现。** “打开”由 Host 选择最合适 Surface；高级用户可用“打开方式”。
2. **对象与载体双向可达。** 从需求能看到 Word 段落、代码和测试；从段落也能打开对应知识对象。
3. **事实、推断、建议分层显示。** 用户随时知道哪些来自原文、哪些由规则推导、哪些是 Agent 候选。
4. **改变先预览影响。** 高风险写入必须展示对象级和载体级 diff、审批人与失败语义。
5. **跨端语义一致，能力允许不同。** 手机可审阅而不能本地启动 Helix；这属于 placement 能力差异，不是协议分叉。

## 2. 全局信息架构

```text
Workspace
├─ Home / Recent
├─ Knowledge
│  ├─ Objects
│  ├─ Relationships
│  ├─ Claims to review
│  └─ Saved perspectives
├─ Work
│  ├─ Tasks
│  ├─ Impact inbox
│  ├─ ChangeSets
│  └─ Approvals
├─ Automations
├─ Resources
└─ Governance
```

“Resources”不是产品首页，而是文件/连接器管理入口；日常工作优先从任务、对象、影响和最近内容进入。

## 3. Muse Shell 布局

Desktop 建议采用三层布局：

- 左侧：Workspace 导航、对象集合、任务与最近资源。
- 中央：当前 Surface，可承载 Markdown、ioffice、Helix、open-file-viewer、Ontology Object View 或未来 CAD/3D。
- 右侧 Knowledge Lens：当前上下文的对象、关系、证据、影响、Agent 活动和可执行动作。

右栏不是聊天框的永久占位。它按任务切换为：

- `Context`：当前选区对应对象与证据；
- `Relations`：有类型的上下游关系；
- `Impact`：未处置影响；
- `Actions`：可执行动作、权限和审批状态；
- `Agent`：围绕上述受限上下文的对话与操作记录。

Web 延续中央 Surface + 抽屉式 Lens。Mobile 默认单层导航，用底部动作打开 Context/Impact/Approval sheet。

## 4. 打开资源

### 4.1 默认点击

用户点击消息交付物、内联引用、搜索结果或对象证据时，只发出：

```text
presentation.request(resourceRef, mode=open, anchor?, placementHint?)
```

Surface Orchestrator 返回明确反馈：

- 立即打开：创建/复用 Surface，定位 Anchor，更新最近记录。
- 需要授权：显示资源名、Provider、请求权限和原因；不显示 Host 私有路径。
- 正在准备：显示大文件 materialization/远端加载进度，可取消。
- 降级打开：明确提示“当前端只读预览；在桌面端可编辑”。
- 不支持：显示元数据、下载/系统打开/安装适配器等合法选项。

### 4.2 “打开方式”

菜单只列出 Orchestrator 评估后可行的 Adapter：

```text
用 Muse Word 编辑器打开     推荐 · 可编辑
用安全预览器打开            只读
在系统默认应用中打开        将离开 Muse
管理此格式的默认打开方式…
```

选择可保存为“本工作区 / 本设备 / 此格式”的用户偏好，但策略、placement 和 capability 始终优先。Agent 不能代表用户永久修改默认引擎。

### 4.3 Surface 生命周期

- 同一 `resourceRef + mode + compatible revision` 优先聚焦已有 Surface。
- 有未保存写入时，关闭显示保存/丢弃/取消；背景 lease 过期不得丢数据。
- 外部变化显示“有新版本”，允许比较、重载或尝试 rebase。
- 从列表回到 Surface 保持 Anchor 和 selection；权限撤销后立即隐藏正文并解释原因。

## 5. Knowledge Lens

当用户在 Word 中选中一段验收标准，Lens 展示：

```text
需求 R-42 · 支付失败重试
状态：已接受    负责人：支付团队

证据
• 当前 Word 段落（rev 7）
• 决策 D-12 §2

关系
• 由 3 个代码符号实现
• 由 5 个测试用例验证

待处理
• 2 个实现可能受本次修改影响

[查看对象] [分析影响] [询问 Agent]
```

每个关系可展开“为什么”：LinkType、创建者/规则、证据、确认状态和时间。相似度推荐单独放在“可能相关”，不得与确认关系混排。

## 6. Object View

Object View 是知识对象的一等 Surface，不只是属性表：

- Header：类型、标题、状态、负责人、可信/新鲜度提示。
- Overview：核心属性和业务摘要。
- Evidence：按 Artifact/Anchor 聚合，可点击回源并定位。
- Relationships：按语义分组的上下游视图，可切图形/列表。
- Timeline：Claim、ChangeSet、Action 与状态变化。
- Available Actions：只显示当前用户可提交的动作；禁用项解释 submission criteria。
- Activity/Agent：围绕该对象的受限任务记录。

对象名称和状态修改走 Action，而非随手编辑底层数据库字段。

## 7. 创建和确认知识

### 7.1 从选区创建

用户选择文本/单元格/代码/CAD 实体后执行“创建知识对象”或“关联到现有对象”：

1. 系统创建 Anchor 并显示格式特定范围。
2. Agent 建议 ObjectType、属性和可能关系。
3. 用户校正并确认；必填字段、权限和重复对象实时校验。
4. 提交后在 Lens 和 Object View 可见；原资源是否插入可见标记由格式能力决定。

### 7.2 Claims Review

Review Queue 按影响和风险排序，卡片必须显示：原文证据、Agent 提议、置信原因、将新增/修改的对象关系、接受后的自动化后果。支持接受、修改后接受、拒绝、合并重复项和批量处理同规则低风险项。

## 8. Impact Inbox

影响中心面向责任人，而不是给所有人发送文件变更噪声。每个 Impact Bundle 包含：

- 发生了什么：业务语义 diff，而非仅红绿文本；
- 为什么找我：责任/订阅/审批关系；
- 哪些内容受影响：按代码、测试、文档、发布等分组；
- 证据路径：`Requirement → verifiedBy → TestCase`；
- Agent 建议：每项独立选择，标记风险、可逆性和 base revision；
- 截止/SLA 与忽略、委派、豁免理由。

合并规则：同一 ChangeSet 默认一个 bundle；短时间内对同一对象的连续保存经去抖后合并；已取代 revision 的通知自动关闭。

## 9. ChangeSet 审阅

审阅界面分三栏或三段：

1. **意图**：用户/Agent 为什么要改，依据哪些对象和证据。
2. **变更**：按目标 Resource/Object 展示语义 diff 与原生 diff；Word 使用修订预览，代码使用 patch，表格使用单元格 diff。
3. **影响与权限**：预计 Link/状态变化、审批人、不可逆步骤、失败/补偿方式。

操作为“批准所选”“请求修改”“拒绝并说明”。一次 bundle 中可独立批准不同责任域。若 base revision 已过期，批准按钮变为“重新生成并复核”，绝不悄悄 rebase 后沿用旧审批。

## 10. Agent 交互

Agent 的回答引用 Object/Anchor；点击引用回到原 Surface。会产生副作用的表达使用显式 action card：

```text
更新测试用例 TC-17
原因：需求 R-42 将最大尝试次数从 2 改为 3
将修改：test/payment_retry.spec.ts @ commit abc…
风险：中 · 需要测试负责人批准
[查看差异] [提交审批] [不处理]
```

“提交审批”不是“已经修改”。执行后 card 更新为 applied/failed/conflict，并可打开 receipt。Agent 不应只用自然语言宣称成功。

## 11. 搜索与浏览

统一搜索返回混合结果，但按结果种类和可执行性清楚区分：

- Objects：业务对象及状态。
- Resources：文件/视图及支持的打开模式。
- Anchors：命中的具体片段。
- Actions：当前上下文可执行动作。
- Workflows：可查看/运行的流程。

过滤包括 ObjectType/Interface、workspace、owner、状态、时间、数据分类和来源。搜索 snippet 仍需权限过滤；无权对象不得通过标题、计数、向量相似结果或关系边泄露。

## 12. 跨端与离线

| 能力 | Desktop | Web | Mobile |
|---|---|---|---|
| Markdown/AppFlowy 编辑 | 完整 | 依部署能力 | 基础编辑 |
| ioffice Word | 原生编辑 | 预览/远端能力 | 预览/审批 |
| Helix | PTY Surface | 远端 PTY 可选 | 不支持，提供“在桌面打开” |
| open-file-viewer | Web Surface | Web Surface | 受格式/内存限制 |
| Object/Impact/Approval | 完整 | 完整 | 优化后的完整审阅 |

离线时可读缓存必须标注 revision/缓存时间和敏感等级。写操作进入本地 proposal queue，联网后重新做权限、revision 与 submission criteria 检查；离线批准不等于离线执行。

## 13. 错误文案原则

错误要说明用户下一步，不泄露内部实现：

- `revision-conflict`：“该内容在你审阅后已更新。请查看最新差异后重新批准。”
- `capability-unavailable`：“当前设备只能预览此格式。可在桌面端编辑或下载。”
- `anchor-ambiguous`：“原位置已变化，找到 2 个可能段落。请选择正确位置。”
- `policy-denied`：“你可以查看，但无权修改测试用例。可向测试负责人请求审批。”
- `partial-apply`：“3 项中 2 项完成；代码仓库写入失败，尚未更新发布状态。”

## 14. 可访问性与国际化

- 键盘可完成 Surface 切换、Anchor 跳转、Impact 审阅和审批。
- 状态不只用颜色表达；diff、置信度与风险包含文本/图标标签。
- 原生/Web Surface 均暴露可访问名称、焦点边界和焦点返回目标。
- 对象类型和 Action 显示名可本地化，稳定 ID 不本地化。
- 日期、数字、单位和文件大小按 locale 显示，审计保存标准化值。

## 15. UX 验收任务

新用户应能在无培训条件下完成：

1. 点击 Word 交付物，在当前端最合适的 Surface 打开并回到原消息。
2. 从一个需求跳到其代码实现与测试证据，理解关系由何而来。
3. 修改需求后找到 Impact Bundle，查看 Agent 建议并只批准测试文档更新。
4. 遇到 revision 冲突时理解旧审批失效并安全重新审阅。
5. 在手机上发现 Helix 不可用，并把资源发送到自己的桌面会话继续。

