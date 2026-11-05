# 产品 PRD：任意资源的版本、审计与语义 Diff

## 1. 产品目标

把 AppFlowy 原有“协作页面能力”升级为 Agent Native Knowledge Platform 的通用协作平面：任何任务资源都能被版本化、追溯、比较和审阅，同时仍由最理解该格式的领域引擎解释变化。

目标不是让二进制文件拥有假的文本 Diff，而是让用户看到领域语义：

- DOCX：段落、表格、批注、样式和修订变化。
- PPTX：幻灯片、形状、布局、动画和讲稿变化。
- XLSX：工作表、区域、公式、命名范围和图表变化。
- MP4：镜头、时间段、字幕、音轨和元数据变化。
- CAD：装配、零件、参数、约束、几何与材料变化。
- 代码/文本：文件、符号、Hunk、行与字符变化。

## 2. 用户与核心任务

### 2.1 产品经理

场景：Agent 修改了 PRD 的 Markdown 文件。

1. 用户从 DSH 对话中的文件引用打开 PRD，文件进入 Host Tab。
2. 用户在 Tab 菜单保存“评审前基线”。
3. Agent 或用户修改文件。
4. 用户选择“与上一保存版本比较”。
5. Host 打开同一窗口内的 Diff Tab，显示章节语义、增删统计和字符级变化。
6. 审计面记录谁在何时创建版本和比较。

成功条件：不离开当前 Host，不依赖 Git，不要求文件位于工程仓库。

### 2.2 开发者

场景：Agent 同时修改 Rust、TypeScript 与配置文件。

- 每个文件共享相同的 Version/Comparison/Audit 协议。
- Viewer 可按文件语言显示语义标签；P0 使用最近的 class/function/heading。
- 后续 Proposal 模式允许逐 Hunk 接受或拒绝，Action 通过三方应用服务执行，而不是由 Viewer 直接写文件。

成功条件：稳定 Change ID 可被 DSH、审计、评论和接受/拒绝动作共同引用。

### 2.3 审计员或团队管理员

场景：定位某一交付物由谁、何时、以什么意图修改。

- 查看 Version DAG 和 Proposal 来源图。
- 查看 Actor、时间、来源会话、工具、模型、基线与结果。
- 选择任意两个 Version 打开领域 Diff Viewer。
- 导出组织策略允许范围内的审计记录。

成功条件：展示事实链而不是只展示最终文件；Comparison 可重算，原 Version 不可变。

### 2.4 非文本专业用户

场景：设计师比较两个 PPTX，工程师比较 CAD，视频编辑比较 MP4。

- 通用平面负责版本选择、权限、审计和 Viewer 路由。
- 领域插件负责 IR、匹配算法、摘要、可视化与可选 Action Handler。
- 未安装 Provider 时仍可展示二进制摘要、哈希、大小与下载/打开操作，但不得伪造“无变化”。

## 3. 信息架构

### 3.1 Workspace

Workspace Explorer 显示资源当前状态：未版本化、已保存、有工作区变化、有 Proposal、冲突。右键菜单由通用能力与 Provider capability 合成。

### 3.2 Tab

文件 Tab 继续显示文件名而不是编辑内核。菜单项按能力动态出现：

- 所有可版本化资源：保存版本、历史、审计。
- 可比较资源：与上一版本比较、选择两个版本比较。
- 有提案资源：查看提案、接受/拒绝、应用全部。
- 本地资源：Reveal in Finder。
- 协作资源：Share、权限、锁定。

### 3.3 DSH

DSH 既是入口，也是变化来源：

- 点击资源引用：发送现有 `OpenResourceRequest`，打开 Host Tab。
- Agent 生成修改：创建 `Proposal`，不能绕过 Action 直接声称修改已接受。
- 对话卡片引用 `resourceId/versionId/proposalId/changeId`，点击后定位到 Diff Tab 对应 Hunk。
- DSH 只读 Comparison 无需持久化完整 Diff；Proposal 的基线、候选 Version 和决定必须持久化。

## 4. 功能需求

### 4.1 版本捕获

| 编号 | 需求 | P0 状态 |
|---|---|---|
| VER-01 | 手动保存不可变版本 | 已实现：文本/代码本地文件 |
| VER-02 | 内容寻址去重 | 已实现：SHA-256 Blob |
| VER-03 | Parent 支持 DAG | 协议已实现，当前后端线性写入 |
| VER-04 | committed/working/proposal 类型 | 协议已实现；P0 使用前两种 |
| VER-05 | 任意 Version 两两比较 | 已实现“历史版本 vs 当前”；双历史选择后续 |
| VER-06 | 远端 Provider 拉取内容 | 待实现 |

### 4.2 语义比较

| 编号 | 需求 | P0 状态 |
|---|---|---|
| DIF-01 | DiffProvider 按资源 capability 解析 | 已实现通用 Registry 与文本 Provider |
| DIF-02 | Stable Change ID | 已实现 |
| DIF-03 | 插入、删除、替换 | 已实现 |
| DIF-04 | 移动、格式、结构变化 | 文本 P0 不单独分类；其他域待 Provider |
| DIF-05 | 语义上下文 | 已实现 Markdown heading / 代码符号近似标签 |
| DIF-06 | Comparison 结果可重算 | 已实现；持久化版本与审计，不持久化完整 Diff |

### 4.3 Diff Viewer

| 编号 | 需求 | P0 状态 |
|---|---|---|
| VIEW-01 | Host 内原生 Tab | 已实现 |
| VIEW-02 | Unified / Split | 已实现 |
| VIEW-03 | 行号、增删色、字符级变化 | 已实现 |
| VIEW-04 | Hunk 折叠与跳转 | 已实现 |
| VIEW-05 | 语法高亮、Minimap、评论 | 后续 |
| VIEW-06 | 键盘导航与屏幕阅读器增强 | 后续；基础 Flutter semantics |
| VIEW-07 | 逐 Change 接受/拒绝 | Proposal 阶段实现，不属于普通 Comparison |

### 4.4 历史与审计

- 历史面显示时间、Actor、说明、内容摘要。
- 审计面显示 append-only 事件及其 subject。
- P0 事件为 `version.capture` 与 `comparison.create`。
- P1 增加 `proposal.create`、`decision.record`、`action.apply`、`merge.conflict`、`version.restore`。
- 审计记录不可包含文件正文、访问令牌或 SSH 凭证。

## 5. 权限与 Capability

权限判定必须同时满足三层：

```text
Host policy ∩ Repository Provider capability ∩ Resource ACL
```

示例：本地文件支持 Reveal，但没有 Share；Team Knowledge 页面支持 Share，但未必支持 Reveal；只读 SSH Mount 可以比较但不能保存或应用 Action。

建议 capability：

- `version.read`, `version.capture`, `version.restore`
- `comparison.create`, `comparison.comment`
- `proposal.create`, `proposal.decide`, `action.apply`
- `audit.read`, `audit.export`
- Provider 自有 `domain.*`

UI 不以资源扩展名猜权限，只在 P0 本地文本路由阶段以扩展名选择 Provider。

## 6. 产品指标与 SLO

### 6.1 正确性

- 已保存 Version 的 digest 校验错误率：0。
- 同一输入重算稳定 Change ID 一致率：100%。
- 未安装 Provider 时错误地报告“无变化”：0。
- 审计事件对已完成写操作的覆盖率：100%。

### 6.2 性能目标

- 1 MB / 20k 行文本的 P95 比较时间：桌面端小于 800 ms。
- 10 MB 文本：后台计算，可取消，P95 小于 3 s。
- Diff Tab 首屏：有缓存小于 200 ms；无缓存小于 1 s。
- Hunk 列表滚动：目标 60 fps，低端机器不低于 45 fps。
- 内容相同的重复快照：Blob 额外空间约 0，仅增加元数据。

P0 尚未针对 20k/10MB 建立基准测试，交付门禁见测试计划。

### 6.3 可用性

- 从文件 Tab 到比较结果不超过 2 个菜单动作。
- 关键状态不得只靠红/绿色表达，必须同时有 `+/-` 与文字。
- Viewer 在 Host 内容区宽度变化时自适应，最小支持 720 px；窄于此宽度自动建议 Unified。

## 7. 非目标

- 不重新实现 Office 编辑器。
- 不把 Git 作为所有 Repository 的强制后端。
- 不在通用层定义 paragraph/slide/cell/frame/shape/line。
- 不在本期实现 Ontology Runtime 或自动影响传播。
- 不把普通 Comparison 误建模为待审批 Proposal。

