# Universal Version, Audit and Semantic Diff Plane

状态：P0 文本/代码纵向切片已实现。VD-R2 Rust Text Runtime 已进入 Host 比较链路（后台 isolate + FFI，只回传 Change Block 坐标；符号不可用时回退 Dart Myers）。双 Text Surface 已支持查找、软换行和跨行选择。Image Overlay 是第一个非文本领域样板。

本目录定义覆盖 AppFlowy 协作页面、本地文件、SSH 文件与云端对象的统一版本、审计、比较与提案平面。它不是另一个 Git，也不是把所有格式强行转换成文本；它为不同领域提供相同生命周期和可审计边界，并把“变化意味着什么”交还给领域 Provider。

## 六个核心概念

```text
Repository
  └── Resource
        ├── Version DAG
        ├── Comparison(A, B)
        │     └── Domain ResourceDiff
        └── Proposal(base, proposed)
              └── Action / Decision
```

1. `Repository`：版本和内容的逻辑归属，不等同于 Git 仓库。
2. `Resource`：被版本化的知识载体；可以是页面、文件、目录内对象或远端对象。
3. `Version`：不可变状态，通过 `parents` 形成 DAG。
4. `Comparison`：两个 Version 的关系，可临时生成，不要求持久化 Diff 结果。
5. `Proposal`：带意图、来源、基线与候选版本的可审阅修改。
6. `Action`：用户或 Agent 对 Proposal 的接受、拒绝、合并、回滚等决定。

`AuditEvent` 是横切日志，不增加第七个业务聚合。Ontology Runtime 本期不实现；协议仅保留稳定 Resource/Version/Actor 标识，未来可投影为 Ontology 对象和关系。

## 三条不可破坏的边界

- Version 与 Diff 解耦：Version 保存不可变事实，Diff 是 `relation(A, B)`。
- Diff 与 Viewer 解耦：通用层只声明 `rendererType`；PPT、Excel、视频、CAD 自己实现 Viewer。
- Diff 与 Action 解耦：Diff 回答“改了什么”，Action 回答“如何处理”。

## 本期实现

- 本地文本、Markdown、常见代码格式的内容寻址快照。
- committed / working Version 类型。
- Myers 行序列比较由 `muse-diff-text` 在后台 isolate 中计算；Rust FFI 只返回紧凑 Change Block 坐标，Dart 再用本地快照组装 hunk/高亮。符号不可用时回退 Dart Myers。
- 单行替换的字符级高亮；双表面支持查找、软换行、`SelectionArea` 跨行复制。
- Host 原生 Tab 内默认左右并排（基线/当前）视图，可切换统一视图，不另开窗口。
- 上一/下一 Hunk、Hunk 折叠、增删统计、无变化状态。
- 资源 Tab 插件菜单：保存当前版本、与上一保存版本比较、版本历史与审计；PNG/JPEG 走 Image Overlay 样板，不再伪装成文本 hunk。
- Project Workspace 文件节点复用同一插件菜单，目录树文件可直接进入上述版本链路。
- Actor 归因支持 User / Agent / System；调用方可把 DSH Agent 身份写入 Version 和 Comparison。
- append-only 审计事件：`version.capture`、`comparison.create`，比较事件携带通用增删改摘要和文本域变更预览。
- 审计面板按新增（绿）、删除（红）、修改（蓝）展示，保留逐行 `+/-` 证据。
- 持久化重启恢复与端到端自动化测试。

## 当前未实现

- Proposal 的 UI 生产入口、逐 Change 接受/拒绝和三方合并；协议模型已定义。
- AppFlowy 协作 Markdown/Database/Word 页面的版本后端适配。
- SSH、对象存储、企业云 Version Store。
- DOCX/PPTX/XLSX/PDF/音视频/CAD 的 Semantic IR、DiffProvider 与 Viewer（Image Overlay 仅为像素域样板）。
- 签名审计、WORM 保留、法律保全和组织级审计导出。
- Ontology Runtime、影响传播与自动 Workflow。

## 1.0 重设计决策

现有 `MuseTextDiffViewer` 的 Hunk Card 结构保留为 Legacy/移动端降级视图，不再作为桌面默认目标。桌面 1.0 使用两个持续存在的资源表面、中央 Connector、视觉行同步滚动、未变化折叠、Alignment Filler、Overview Stripe 和统一 Workbench Shell。

通用协议不把所有资源转换成文本 Hunk。Provider 输出稳定的领域 `SemanticChangeSet`，Renderer 再把领域 Anchor 投影为文本行、页面区域、单元格、时间轴或三维对象。审计事件只记录 Version/Comparison/Change 引用和摘要，不再复制 before/after 内容。

## 文档导航

- [01-product-prd.md](01-product-prd.md)：用户场景、功能、权限与指标。
- [02-protocol-architecture.md](02-protocol-architecture.md)：通用协议、分层、Provider 与存储架构。
- [03-text-code-vertical-slice.md](03-text-code-vertical-slice.md)：文本/代码实现和 UI 交互。
- [04-delivery-test-plan.md](04-delivery-test-plan.md)：开发阶段、测试矩阵、验收门禁。
- [05-intellij-grade-redesign-prd.md](05-intellij-grade-redesign-prd.md)：IntelliJ 级 Workbench 产品形态、场景、端侧功能和指标。
- [06-universal-diff-protocol-and-architecture.md](06-universal-diff-protocol-and-architecture.md)：1.0 协议、Provider/Renderer SPI、运行时、审计和安全边界。
- [07-text-renderer-and-domain-provider-design.md](07-text-renderer-and-domain-provider-design.md)：双 Text Surface、Connector、Sync/Align/Fold 与任意资源矩阵。
- [08-redesign-delivery-plan-and-test-matrix.md](08-redesign-delivery-plan-and-test-matrix.md)：R0–R8 开发阶段、自动化/手工测试矩阵和发布门禁。

## 代码入口

- 通用协议：`lib/plugins/version_diff/domain/version_diff_contract.dart`
- 文本 Provider：`lib/plugins/version_diff/text/text_diff_provider.dart`
- Rust Text Runtime：`frontend/rust-lib/muse-diff-text`，FFI：`frontend/rust-lib/dart-ffi/src/muse_diff_text_ffi.rs`
- Dart 运行时边界：`lib/plugins/version_diff/text/text_diff_runtime.dart`
- Image Overlay 样板：`lib/plugins/version_diff/image/image_overlay_diff_provider.dart`
- 本地版本库：`lib/plugins/version_diff/application/text_version_repository.dart`
- 应用服务：`lib/plugins/version_diff/application/text_version_diff_service.dart`
- Diff Viewer：`lib/plugins/version_diff/presentation/text_diff_viewer.dart`
- Host Tab：`lib/plugins/version_diff/presentation/text_diff_plugin.dart`
- 菜单接入：`lib/plugins/resource_surface/resource_tab_actions.dart`
