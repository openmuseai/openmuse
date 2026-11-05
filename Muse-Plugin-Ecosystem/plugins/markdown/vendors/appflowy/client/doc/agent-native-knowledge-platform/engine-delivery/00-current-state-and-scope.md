# 当前状态、目标范围与能力差距

## 1. 路径与审计口径

代码路径相对于 `openmuse-io/` 工作区根目录。本文描述 2026-09-15 工作区事实；未提交代码也计入“当前工作区”，但不等于已发布版本。

## 2. 公共平台现状

| 领域 | 当前状态 | 本轮处理 |
|---|---|---|
| Host Bridge | 已有 discover/bind/invoke/subscribe/event/policy/cancel/status、schema digest、消息限制、UDS/InProcess | 复用；增加 Resource/Presentation/Engine Session 业务合同 |
| Plugin Graph v2 | 已有 runtime、facet、contract provider、grant、contribution、activation/disposal | 增加 Format Provider、Surface Adapter contribution |
| Flutter Surface Runtime | 已有 lease、focus、Context、Domain Change、Presentation Intent 和 receipt 雏形 | 增加 Orchestrator、route decision、EngineSession 绑定 |
| DSH deliverables | 默认 `openFile(path)` 打开 DSH Sidebar；显式 native open 走安全 `/api/present.open` | 新建正式 OpenResource capability seam；旧 Sidebar 为 fallback |
| Desktop transport | DSH WebView 加载页面，缺通用 Host → Flutter Intent | 增加中立 envelope 下行 |
| Web/Mobile transport | parent bridge HTTPS/SSE，仍理解部分业务消息 | 渐进适配为 Host Bridge envelope transport |
| Document contract | `muse.document@2` 有 query/propose/apply/status/event | 保留为领域合同；不当作所有格式的通用编辑 API |

## 3. 三引擎事实

### 3.1 Helix

代码：

- `vendors/helix/helix-term`：`hx` 可执行程序。
- `vendors/helix/helix-tui`：终端 Backend，默认 `termina/crossterm`。
- `vendors/helix/runtime`：queries、themes、语言配置；grammar 产物需单独构建/打包。
- `vendors/deepseek-harness/packages/terminal` 与 `packages/subprocess`：现成 PTY/进程能力基础。

事实：

- Helix 25.7.1、Rust 1.90；当前 workspace 未提供 Muse 集成和预构建 `hx`。
- 它要求真实 TTY/PTY；不能像普通子进程只读 stdout。
- `HELIX_RUNTIME` 和 `--config` 可隔离运行时资源与配置。
- 终端字节流不适合放入 Host Bridge JSON envelope。
- 当前 `doc/helix-editor-integration.md` 已有 PTY 方案，但仍以 path 和专用 WS 为中心，需要升级到 resourceRef/materialization/EngineSession。

本轮目标：

- Desktop 可通过 resourceRef 打开文本资源到 Helix Surface。
- 控制面接入 Host/DSH，PTY 数据面独立。
- 只在 Provider 允许时编辑；保存经过直接写安全模式或工作副本 commit。
- 打包、签名、runtime、grammar 和崩溃清理可验证。

### 3.2 ioffice

代码：

- `vendors/ioffice/word/word_render`：FRB、布局、绘制、`WordSession`。
- `vendors/ioffice/word/word_editor`：编辑壳、ribbon、分页和 selection。
- `Muse-Clients/frontend/client/frontend/appflowy_flutter/lib/plugins/office`：Manifest、Folder/ViewLayout、blob。
- `Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/muse_word_surface`：Surface、Context 与 selection contribution。

事实：

- Word 已有 macOS arm64 dylib、Flutter 渲染和 heap session 编辑。
- 当前 `WordEditorController.canExportDocx == false`。
- vendor 没有 `WordSession.toDocx()`；打开时 `_docx` 是旧字节，编辑后不得写回。
- 因此当前 Word 可以创建/导入/显示/内存编辑，但 Save 和 Agent apply 是 `BLOCKED_BY_KERNEL`。
- Excel、Slides、PDF 的 vendor 目录只是占位；Muse 有 ViewLayout/Manifest/blob 槽，`engineBound=false`。
- Office Catalog 当前占位项仍可能 `creatable=true`，目标 UI 必须以 runtime effective capability 为准，不能生成看似可用的空壳产品。

本轮目标：

- Word 先通过统一 Resource/Surface 链提供诚实的只读/内存体验。
- `toDocx` 与 round-trip gate 通过后，才能开放 Save/commit/DSH apply。
- macOS arm64 为首个平台；Windows/Linux/Web 分别通过产物与行为认证。
- Excel/Slides/PDF 采用 admission gate，不在无引擎时假装进入开发完成态。

### 3.3 open-file-viewer

代码：

- `vendors/open-file-viewer/packages/core/src/types.ts`：`PreviewPlugin.match/render`、`PreviewInstance.resize/goToPage/command/destroy`。
- `packages/core/src/viewer.ts`：队列、AbortController、loading/error、reload/resize/destroy。
- `packages/core/src/plugins/`：text/image/PDF/Office/archive/email/drawing/xmind/CAD/3D/GIS/fallback 等。

事实：

- 当前版本 0.1.45，browser-first、ESM/CJS，测试使用 Vitest/jsdom。
- 输入支持 File/Blob/string URL/ArrayBuffer。
- `isPreviewSupported()` 按插件顺序 match，fallback 不算原生支持。
- PDF 支持 worker、range/stream 选项和浏览器 iframe fallback。
- 多个复杂插件会解析压缩包、邮件、HTML/SVG、CAD/3D/GIS，具有内存、主动内容和依赖风险。
- 当前没有 Muse Adapter、WebView/iframe sandbox、Host resource broker 或格式认证清单。

本轮目标：

- 作为只读、安全降级的 Web Surface Adapter。
- 首批只认证 text/image/PDF；Office/Archive/Email 和 CAD/3D/GIS 分波次。
- Resource Provider 提供短期 URL、Blob/ArrayBuffer 或 range；Viewer 不获得云凭据、本地路径和 Host API。
- download/print/fullscreen/远程 iframe 均由 Host policy 裁剪。

## 4. 目标能力边界

| 能力 | Helix | ioffice | open-file-viewer |
|---|---|---|---|
| 主要模式 | text edit/view | Office view/edit | view |
| 运行形态 | `hx` 子进程 + PTY | Flutter + native/wasm engine | sandbox Web runtime |
| 控制面 | Host Bridge | Host Bridge/Flutter Runtime | Host Bridge + postMessage |
| 大数据面 | PTY stream | Host-local bytes/FFI | short URL/range/Blob |
| 保存 | direct-safe 或 working-copy commit | `toDocx` 后 commit | 不修改原资源 |
| Context | file/cursor/selection/status | page/caret/selection/status | page/zoom/search/可选 selection |
| 首发端 | Desktop macOS | Desktop macOS arm64 | Desktop/Web，Mobile 子集 |
| 未知格式 fallback | 不接受 | 不接受 | metadata/download |

## 5. 本轮必须交付

- `muse.resource@1` 的最小 describe/materialize/commit/subscribe。
- `muse.presentation@2` 的 request/decision/receipt。
- Engine Adapter Manifest、runtime probe、session lifecycle 和 TCK。
- DSH OpenResource Service Definition、Host Provider、UI/Agent Consumer。
- Desktop Host → Flutter Surface 下行。
- Viewer Wave 1、Word 统一只读链、Helix PTY spike/只读链。
- 每条启用写入的路径都有 revision、冲突、幂等、恢复和数据损坏负例。
- 打包产物、许可、签名、SBOM/依赖和 feature flag。

## 6. 本轮不交付

- Ontology Runtime 及任何具体业务对象。
- Office 实时多人协同。
- Helix 作为 library 或 Flutter 自绘 Helix compositor。
- Viewer 修改原始 Office/PDF/CAD 文件。
- 无真实引擎的 Excel/Slides/PDF 编辑。
- 将任意用户机器路径暴露给 DSH/Agent。
- 跨系统 Workflow 或自动审批。

## 7. 关键风险

| 风险 | 级别 | 硬控制 |
|---|---|---|
| Word 编辑后错误写回旧 `_docx` | P0 | 无 `toDocx` 时 Save/commit/apply 代码路径必须不可达 |
| PTY 可执行任意命令/路径逃逸 | P0 | 固定 argv、workspace/materialization、无 shell 拼接、scope/approval |
| Viewer 主动内容/依赖逃逸 | P0 | sandbox、CSP、禁默认网络、格式 wave 认证 |
| Manifest 与实际产物不一致 | P0 | runtime probe + package verifier；UI 只消费 effective capability |
| 大文件导致内存崩溃 | P1 | size budget、range/stream、worker、端侧上限和降级 |
| 三引擎各造一套 receipt/error | P1 | 公共 TCK 和 closed error taxonomy |
| 提前耦合 Ontology | P1 | 只允许 reserved hooks，禁止具体 Object/Action 依赖 |

