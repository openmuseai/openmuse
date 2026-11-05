# IntelliJ 级 Universal Diff Workbench 开发计划与测试矩阵

## 落地状态（代码闭环后）

- **VD-R2**：`muse-diff-text` 经 `muse_diff_text_compare_json` 进入 Host 比较链路。Rust 只序列化 Change Block / SimilarBoundary 坐标；Flutter 在后台 isolate 调 FFI，符号缺失回退 Dart Myers。Dart 端用本地 Version 快照组装 hunk，禁止把全文当 Diff 结果回传。
- **VD-R3**：双 Text Surface 已支持查找、软换行、`SelectionArea` 跨行选择/复制。
- **VD-R4**：`MuseTextDiffSync` 用 SimilarBoundary 做 1/3 viewport 同步；Align 关闭时两侧独立行高，连接面按左右 prefix 绘制。
- **VD-R5**：折叠内搜索自动展开、Overview 可点、Search lane、filler/`SelectionContainer.disabled` 不可选。
- **VD-R6**：Host Tab 经 `TabsBloc.openExternalPlugin` 打开 Diff Workbench；integration_test 覆盖保存 → 编辑 → 比较 → 审计。
- **VD-R7 样板**：`muse.diff.image-overlay.v1` 用 page-region anchor，不复用文本 hunk。
- 其余 VD-R8 项仍按下列阶段推进。

## 1. 交付策略

本计划把“看起来像双栏”与“具有双编辑器语义”分开验收。任何阶段不得用固定高度、截图背景或两个独立滚动列表伪装同步 Diff。

阶段顺序：先稳定通用协议和 Text Runtime，再交付双 Surface，随后加入 Connector/Sync/Fold/Align，最后接入 Audit/DSH 和领域插件。Ontology Runtime 不在本计划内。

## 2. 里程碑总览

| 阶段 | 目标 | 退出条件 |
|---|---|---|
| VD-R0 | 设计冻结与基准语料 | 协议评审通过、基准可重复 |
| VD-R1 | 协议 1.0 与迁移 Adapter | 新旧 Provider 可并存，审计不再写明文预览 |
| VD-R2 | Rust Text Runtime | 算法、索引、取消、窗口 API 达标 |
| VD-R3 | 双 Text Surface | 连续全文、选择/复制/搜索/行号可用 |
| VD-R4 | Connector + Sync Scroll | 曲面连接、滚动映射和导航达标 |
| VD-R5 | Fold + Align + Overview | IntelliJ 级核心视觉交互完整 |
| VD-R6 | Workbench/Audit/DSH E2E | Host 当前 Tab 内端到端闭环 |
| VD-R7 | 插件 SDK 与非文本样板 | PDF/Image 或 DOCX 样板证明协议通用性 |
| VD-R8 | 性能、安全、发布 | 门禁全绿，Legacy 默认关闭 |

## 3. 各阶段详细方案

### VD-R0：设计冻结与基准

开发：

- 建立 `version-diff` ADR，冻结六个业务概念和协议/运行时对象边界。
- 建立文本语料：中文、英文、emoji、CRLF/LF、超长行、生成文件、混合编码、10k/100k/1m 行。
- 建立视觉 Goldens：浅/深色、Insert/Delete/Modify/Move、空范围、折叠、软换行。
- 记录当前 Hunk Viewer 的功能和性能基线。

测试：

- Schema review checklist；
- corpus digest 固定；
- benchmark 环境记录 CPU/RAM/Flutter/Rust 版本；
- IntelliJ 参考行为手工对照，不做像素复制。

退出：所有后续阶段都有可执行 fixture 和目标，不以主观“像不像”验收。

### VD-R1：协议 1.0 与兼容层

开发：

- 新增 Request/Input/Options、SemanticChangeSet、Anchor、Quality、Renderer Manifest。
- Provider/Renderer Registry 支持 schema/capability/policy 选择。
- Legacy Text payload Adapter。
- Viewer Session 和 revision/cancellation 契约。
- 新审计 schema，只写 change digest/ID/summary；内容预览改为 read-time API。

测试：

- JSON/Protobuf round-trip 与 unknown field；
- provider 优先级和 fallback；
- capability 权限拒绝；
- legacy adapter 结果与旧算法等价；
- audit 无明文、无绝对路径；
- 不支持/超预算不等于 zero changes。

退出：现有 Host 功能不回归，新旧 Provider 可同时注册且路由确定。

### VD-R2：Rust Text Runtime

开发：

- 新 crate `muse-diff-text`（最终名称以 workspace 规范为准）。
- immutable text snapshot、line/offset index、encoding/newline detection。
- Myers + anchor prefilter + inline diff + budget/fallback。
- SimilarBoundary、TextChangeSet、稳定 Change ID。
- viewport slice、search/copy API、cache lease、cancellation。
- dart-ffi event/request 接口，禁止每帧全文传输。

测试：

- property test：应用 edit script 可从 before 得到 after；
- fuzz：任意 bytes/encoding 不 panic、不越界；
- Unicode：grapheme、组合字符、CJK、RTL；
- 确定性：相同输入、选项和 provider version 结果相同；
- cancellation、timeout、OOM budget；
- 100k/1m 行 benchmark；
- FFI 包大小与调用频率 telemetry。

退出：20k 行 P95 ≤ 1 s；取消 ≤ 100 ms；Dart 端没有全文 payload。

### VD-R3：双 Text Surface

开发：

- `MuseTextDiffSurface` RenderObject/Widget 组合；
- logical/visual/surface/workbench 坐标转换；
- gutter、行号、soft wrap、horizontal scroll；
- 选择、复制、查找、键盘导航和只读 caret；
- layered decorations 与 Host Theme Token；
- 双 Surface responsive layout，父约束驱动宽高。

测试：

- selection/copy 与原文 byte/Unicode 对齐；
- soft wrap、tab、不同字体、缩放；
- 窗口 resize、左右 pane 拖动、sidebar 开关；
- Retina/非 Retina golden；
- accessibility semantics；
- 旧截图中的左侧遮挡和右侧大空白不得复现。

退出：两侧能独立完整浏览，宽度始终填满 Workbench，不存在卡片式断裂。

### VD-R4：Connector、同步滚动与导航

开发：

- visible-range connector geometry/cache/painter；
- hover/select 与 Changes Tree 联动；
- SimilarBoundary line transfer；
- 1/3 viewport anchor + pixel phase 同步；
- sync transaction 防循环；
- previous/next、overview click、DSH change deep link。

测试：

- insert/delete/replace/empty-range connector；
- connector viewport clipping；
- scroll wheel、trackpad、scrollbar、PageUp/Down、programmatic jump；
- 非交叉映射 property test；
- 连续 10 分钟滚动无抖动/反馈循环；
- deep link 定位误差；
- RTL/长行与 horizontal scroll。

退出：稳定后左右 anchor 漂移 ≤ 1 visual px；滚动 P95 ≤ 16.7 ms。

### VD-R5：折叠、对齐与概览条

开发：

- unchanged range folding、波浪边界、上下文扩展；
- AlignmentFiller 与 visual-line measure；
- 300 ms debounce、revision commit、anchor restore；
- overview lanes：diff/search/selection/diagnostics；
- toolbar ignore/highlight/align/sync/collapse commands。

测试：

- fold 内搜索/深链自动展开；
- 单侧/双侧展开映射；
- resize/soft-wrap/font 触发 align；
- filler 不可选择、复制、不改变行号；
- dense overview aggregation；
- theme/color-blind/contrast；
- 计算旧 revision 返回后被丢弃。

退出：参考图中的连接、折叠、对齐、概览核心体验均能由自动化和手工场景验证。

### VD-R6：Workbench、Audit 与 DSH 端到端

开发：

- 当前 Host Tab 打开/复用/关闭 Diff Workbench；
- Changes Tree、Audit Inspector、status/progress/error；
- Tab/Workspace/Renderer 插件菜单贡献；
- DSH `comparison://` 与 `change://` 安全深链；
- User/Agent/System 归因和一致颜色图例；
- Session 保存恢复与 Tab dispose。

测试：

- Workspace 文件 → 保存版本 → 编辑 → Compare → Diff Tab；
- DSH Agent 修改 → 对话点击 → Host 当前窗口定位 Change；
- 用户/Agent insert/delete/modify 在 Viewer、Tree、Audit 颜色和计数一致；
- 无权限、Version 缺失、Provider crash、远端断线；
- 多窗口和相同 Comparison Tab 去重；
- 关闭 Tab 后无 listener/handle 泄漏。

退出：macOS 打包应用完成真实文件 E2E；Windows/Linux 完成契约测试，后续分别做 UI 验收。

### VD-R7：插件 SDK 与非文本样板

开发：

- 发布 Provider/Renderer SDK、manifest、capability 和 sample。
- 选择两个不同视觉范式的样板：建议 DOCX 结构 Diff + Image Overlay Diff；若 Office 引擎接口未稳定，则改为 PDF + Image。
- Reopen With…、fallback 和错误隔离。

测试：

- 插件安装/卸载/升级与 schema negotiation；
- 恶意/超大 payload 限制；
- Renderer crash 不关闭 Host；
- 同一 ChangeSet 切换两个 Renderer；
- 无领域插件时 binary summary 可用。

退出：不修改通用协议即可接入两种非文本 Renderer，证明“任意资源”不是文本接口换名。

### VD-R8：发布加固

开发：

- profile、cache budget、内存压力处理、telemetry；
- content capability、临时缓存、审计签名/保留策略接入；
- accessibility、localization、crash recovery；
- Legacy Hunk Viewer 改为显式 fallback，桌面默认关闭。

测试：

- 24h soak、多 Tab、内存压力；
- 断网/SSH 中断/Cloud token 过期；
- 权限撤销与租户切换；
- corrupted Office/media/CAD fixture；
- release mode macOS notarized build smoke；
- telemetry 不含内容/路径。

退出：发布门禁全部达标，无 P0/P1 缺陷。

## 4. Text 自动化测试矩阵

| ID | 类型 | 场景 | 关键断言 |
|---|---|---|---|
| T-DIFF-001 | Unit | 相同文本 | exact + 0 changes |
| T-DIFF-002 | Unit | 纯插入/纯删除 | 空侧 anchor 正确 |
| T-DIFF-003 | Unit | 行替换 + inline | 行和字符证据一致 |
| T-DIFF-004 | Property | 随机 edit script | replay 后等于 target |
| T-DIFF-005 | Unit | 移动函数 | symbol provider 标 move；fallback 仍有文本证据 |
| T-DIFF-006 | Unit | whitespace policy | 策略改变 presentation/quality，不改变 Version |
| T-DIFF-007 | Fuzz | 任意 bytes | 不 panic、不越界、明确退化 |
| T-VIEW-001 | Widget | 双栏 resize | 无 overflow/遮挡/固定空白 |
| T-VIEW-002 | Widget | soft wrap | connector 端点随 visual range |
| T-VIEW-003 | Widget | fold | 两侧 fold 和映射正确 |
| T-VIEW-004 | Widget | selection/copy | 复制内容与对应 Version 一致 |
| T-VIEW-005 | Golden | light/dark | 多层高亮和对比度 |
| T-SYNC-001 | Unit | boundary transfer | 单调、首尾、无 crossing |
| T-SYNC-002 | Integration | trackpad 快速滚动 | 无反馈循环、漂移达标 |
| T-ALIGN-001 | Integration | resize + debounce | 只提交最新 revision |
| T-CONN-001 | Unit | geometry | 曲面无自交、端点误差 ≤1 px |
| T-CONN-002 | Perf | 10k changes | 只构造可见 connector |
| T-AUDIT-001 | Security | comparison event | 无明文/绝对路径 |
| T-AUDIT-002 | Integration | preview query | 重新授权、可脱敏 |
| T-GRAPH-001 | Unit | branch/merge DAG | parent edge、lane 投影稳定且无环假设错误 |
| T-GRAPH-002 | Integration | bounded graph query | cursor、around、权限裁剪正确 |
| T-PROV-001 | Integration | mixed user/agent changes | evidence 等级和 Unknown fallback 正确 |
| T-DSH-001 | E2E | change deep link | 当前 Host Tab 精确定位 |

## 5. 任意资源契约测试矩阵

每个 Provider 必须通过同一 Contract Suite：

| 类别 | 必须验证 |
|---|---|
| Identity | manifest 唯一、schema/version 合法 |
| Support | media magic/schema 优先于扩展名 |
| Determinism | 同输入同选项结果和 Change ID 稳定 |
| Anchors | 全部 anchor 在对应 Version 范围内 |
| Hierarchy | child 无环、父子路径一致 |
| Quality | 启发式/退化明确，不伪造 exact |
| Cancellation | 在预算内响应，无孤儿进程 |
| Security | 不越权读取、不泄漏 path/content |
| Size limits | 节点数、字符串、嵌套、帧数受限 |
| Renderer handoff | 支持的 Renderer 能投影全部可导航 Change |
| Accessibility | 每个 Change 有可读 label |
| Audit | evidence digest 可验证，Actor 由 Host 提供 |

## 6. 手工验收脚本（macOS）

1. 在 Project Workspace 导入一个包含 Rust、TS、Markdown 的目录。
2. 打开 2k 行代码，保存基线，分别由用户和 DSH Agent 产生插入、删除、修改。
3. 从资源 Tab 菜单打开比较，确认仍在 Host 当前窗口且 Tab 可关闭/复用。
4. 检查两侧完整文档、行号、语法高亮、选择/复制、查找。
5. 使用 trackpad 快速滚动左右两侧，观察同步、连接面、概览条和变化树。
6. 调整窗口、侧栏宽度和 pane divider，确认无左侧遮挡或大面积固定空白。
7. 开关 soft wrap、Align、Collapse unchanged、Ignore whitespace。
8. 从 DSH 点击第三处 change，确认复用 Diff Tab 并自动展开/定位。
9. 打开 Audit Inspector，核对 User/Agent 归因、颜色、版本父链和工具来源。
10. 断开远端或模拟 Provider 失败，确认明确退化且 Host 不崩溃。

## 7. 发布门禁

- 协议、算法、Widget、Integration、E2E、Golden、Accessibility、Security 全部通过。
- macOS Release 构建和 notarization smoke 通过。
- 性能指标在基准设备 P95 达标；未达标项必须显式 feature flag，不得静默降低正确性。
- Audit 新事件零明文；telemetry 零内容、零绝对路径。
- Provider crash、无权限、超预算和未知格式都有用户可理解的状态。
- 文档、插件 SDK、迁移指南和回滚方案齐全。

## 8. 风险与回滚

| 风险 | 缓解 | 回滚 |
|---|---|---|
| 自研 Text Surface 复杂 | 先只读、固定语言能力，再加编辑动作 | Legacy Hunk Renderer feature flag |
| Rust FFI 延迟 | viewport batch、revision、prefetch | 本地小文件 Dart adapter |
| Connector/Align 抖动 | visual measure cache、debounce、anchor restore | 关闭 Align，保留 Sync |
| 非文本 schema 过度抽象 | 两个差异化样板验证 | schema capability versioning |
| Audit 旧明文 | 停止新增、权限隐藏、合规迁移 | 旧库只读隔离 |
| 插件崩溃/恶意文件 | broker、预算、payload validation | binary fallback renderer |
