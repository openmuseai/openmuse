# 当前实现审计与差距

## 1. 审计口径

本文件以 2026-09-14 工作区中的实际源码为准。由于相关改动中存在未提交文件，结论描述“当前工作区能力”，不等同于某个已发布版本。目标设计不得把文档中的意图当作实现事实。

下列代码路径均相对于 `openmuse-io/` 工作区根目录，避免混淆 `Muse-Clients` 子树与同级 `vendors`。

## 2. 已有地基

### 2.1 Host Bridge 与能力注册

**[已实现]** `@muse/host-bridge` 已提供领域中立元协议：`hello / discover / bind / invoke / subscribe / event / policy / cancel / status`，具备协议协商、schema digest、大小限制、deadline、取消、幂等、错误码和 UDS/InProcess 运输。

关键代码：

- `Muse-Clients/middlewares/dsh/core/protocol/host-bridge/src/contract/types.ts`
- `Muse-Clients/middlewares/dsh/core/protocol/host-bridge/src/codec/limits.ts`
- `Muse-Clients/middlewares/dsh/core/protocol/host-bridge/rust/host-registry/`
- `Muse-Clients/frontend/client/frontend/rust-lib/flowy-core/src/muse_host.rs`

当前硬上限包括 2 MiB 消息、256 KiB input、1 MiB output、256 KiB event payload。大文件不能直接塞进 Bridge envelope，这正是资源协议必须使用流、range 或 Host-local handle 的原因。

### 2.2 Plugin Graph 与三类运行时

**[已实现]** `muse.plugin/v2`、`muse.host/v2`、`muse.composition-plan/v2` 已定义 artifact、facet、runtime、platform、contract provider、grant、contribution、激活与释放顺序。

关键代码：

- `Muse-Clients/middlewares/dsh/core/plugin-graph/src/v2.ts`
- `Muse-Clients/middlewares/dsh/core/ecosystem-sdk/src/index.ts`

现有 runtime 是 `dsh-native / remote-dsh / flutter / rust-host`；现有 facet kind 是 `agent / presentation / domain / service`。本方案的新资源、Ontology 和 Workflow 能力应作为合同与 Facet 加入，不修改元协议的业务中立性。

### 2.3 Facet 与 Surface Runtime

**[已实现]** Flutter 侧已有：

- Facet 注册、Surface lease、状态与焦点；
- Context 发布的 TTL、revision、大小限制与 control/state lane；
- Domain Change 按 `pluginId + resourceRef` 路由；
- Presentation Intent 按 `pluginId + scopeRef + targetSurfaceInstanceRef` 路由；
- 超时、歧义、not-found、not-supported 等回执；
- Renderer Registry、贡献冲突、代次释放和审批 proof 雏形。

关键代码：

- `Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/muse_ui_surface_runtime/lib/src/runtime.dart`
- `Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/muse_ui_surface_runtime/lib/src/kernel.dart`
- `Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/muse_plugin_facets/lib/src/models.dart`

这套 Runtime 是统一承载层的正确地基，但目前“打开任意资源并自动创建 Surface”还没有进入它。

### 2.4 文档与表格领域合同

**[部分实现]** `muse.document@2` 已有 query、propose、apply、status、snapshot 与 committed event；Markdown Provider 已连接 AppFlowy Document，Word Provider 已提供读取路径，表格另有 `muse.table/range-snapshot/v1`。

关键代码：

- `Muse-Clients/middlewares/dsh/core/contract-document/src/`
- `Muse-Clients/middlewares/dsh/plugins/appflowy-markdown/`
- `Muse-Clients/frontend/client/frontend/rust-lib/flowy-core/src/muse_markdown.rs`
- `Muse-Clients/frontend/client/frontend/rust-lib/flowy-core/src/muse_word.rs`
- `Muse-Clients/middlewares/dsh/plugins/appflowy-database/src/index.ts`

它们是**领域合同**，不应被删除或强行合并成一个万能文件 API。统一资源协议负责身份、描述、交付和承载；领域合同继续拥有 Markdown edit、table range、Word semantic edit 等业务语义。

### 2.5 Workspace、身份与权限

**[已实现]** Rust Host 在 bind/invoke 时根据当前 user、workspace、view 和 auth type 重新解析 authority；scope hint 只接受 AppFlowy 允许的命名空间。读默认允许，写要求审批；Desktop approval 使用 Host 持有的 HMAC proof。

关键代码：

- `Muse-Clients/frontend/client/frontend/rust-lib/flowy-core/src/muse_host.rs`
- `Muse-Clients/frontend/client/frontend/rust-lib/flowy-core/src/muse_view_reference.rs`
- `Muse-Clients/middlewares/dsh/core/protocol/host-bridge/rust/host-policy/`

这已经落实“Agent 不能把自报 ID 当权限”的核心原则。

### 2.6 多端载波

**[部分实现]** Web/Mobile 通过 parent-bridge 的 HTTPS/SSE 承载 Context 与 Presentation Intent；Mobile 的 `AppFlowyDshControlHost` 会发布 workspace catalog、Markdown/Word snapshot，并能打开 AppFlowy Markdown 页面。Desktop 仍是 local sidecar + WebView，`DshEmbeddedView` 只负责加载页面，未安装 Native Capability Broker，也没有通用的 Host→Flutter Intent 下行。

关键代码：

- `Muse-Clients/middlewares/dsh/plugins/dsh-appflowy/src/parent-bridge.ts`
- `Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/muse_remote_session/lib/parent_bridge_adapter.dart`
- `Muse-Clients/frontend/client/frontend/appflowy_flutter/lib/plugins/dsh_agent/appflowy_dsh_control_host.dart`
- `Muse-Clients/frontend/client/frontend/appflowy_flutter/lib/plugins/dsh_agent/dsh_embedded_view.dart`

## 3. 当前文件交付与打开的真实行为

### 3.1 DSH 交付物

**[已实现]** DSH 的 `ui-deliverables` 从成功写工具和 `present` 会话事件构建结构化文件词表，正文行内代码只在精确路径或唯一 basename 匹配后可点击。

**[已实现]** 默认点击调用 Chat View 注入的 `openFile(path)`，把 `dsh-resource://file/session/...` 交给右侧 Sidebar。它不会进入 Muse Flutter Surface Runtime。

**[已实现]** 只有交付卡片菜单中的“Host 默认应用/在文件管理器中显示”才调用 `/api/present.open`，由 Host 回读 session event、执行 `stat`、路径往返映射后调用 `sessionController.openWorkspacePath`。

关键代码：

- `vendors/deepseek-harness/packages/client/ui-deliverables/src/client/Deliverables.tsx`
- `vendors/deepseek-harness/packages/client/ui-chat/src/client/apply.ts`
- `vendors/deepseek-harness/packages/client/ui-deliverables/src/present-open.ts`
- `vendors/deepseek-harness/packages/api/session-controller/src/index.ts`

### 3.2 初稿必须修正的关键假设

初稿提出通过 `session-controller` 的 `internals.openPath` 注入即可接管全部打开动作。当前源码不支持这个结论：

1. `SessionControllerInternals` 是构造参数，注释明确为直接单元测试替换，并不是 Cordis Service seam 或可由后装插件覆盖的注册表。
2. 它只覆盖显式原生打开，不覆盖默认卡片点击、ProducedFiles 和正文 mention；这些入口直接调用 Chat View 的 `openFile` 打开 Sidebar。
3. 因此即使设法替换 `openPath`，Markdown 默认点击仍具有特权路径，无法满足“所有格式同构”。

目标实现必须新增一个**受支持的 Resource Presentation seam**，并让默认预览和显式打开共同成为它的 Consumer；不能把私有测试注入点当产品扩展机制。

## 4. 当前引擎状态

| 引擎 | 当前事实 | 可复用部分 | 缺口 |
|---|---|---|---|
| ioffice Word | **[部分实现]** macOS arm64 预编译 dylib 已入库；Flutter `word_render/word_editor`、Word View、blob store、Word Surface 已存在 | 文档渲染编辑、AppFlowy ViewLayout、blob 与 snapshot | 统一 Resource Provider/Surface Adapter、其他平台产物、路由 |
| ioffice Excel | **[占位]** Muse 有 ViewLayout、Manifest、blob 目录和空 OOXML 包；vendor 目录只有说明 | 插件槽与存储骨架 | 真引擎、Surface、领域合同 |
| ioffice Slides | **[占位]** 同 Excel | 插件槽与存储骨架 | 真引擎、Surface、领域合同 |
| ioffice PDF | **[占位]** 有 ViewLayout、Manifest、最小 PDF 与 magic 校验；vendor 目录无引擎 | 插件槽与存储骨架 | 真查看/批注引擎、Surface |
| Helix | **[未集成]** vendor 是 TUI 上游源码，当前无 Muse Surface 与打包产物 | Helix 自身编辑能力；DSH 有 terminal/subprocess seam | PTY owner、WS/原生终端承载、路径授权、打包与签名 |
| open-file-viewer | **[未集成]** vendor 是浏览器优先 SDK，已有 `PreviewPlugin.match/render/destroy` 和多格式插件 | Web 容器、按格式匹配、PDF/Office/媒体/CAD/3D 等预览 | 受控资源读取、一次性 URL/range、Flutter WebView Surface、CSP |

Office 当前 Manifest 将 Word 标为 `engineBound: true`，Excel/Slides/PDF 为 `false`；四者都标为 `desktopOnly: true`。任何路线图都必须据此区分“槽已存在”和“引擎已可用”。

## 5. 当前 Ontology 状态

**[拟新增]** 当前仓库没有通用 Ontology Language、Object/Link/Action 类型注册表、对象实例存储、影响图或自动化运行时。现有 `resourceRef`、Facet、Domain Change、Host Capability Registry 和 Document Contract 是可复用基础，但不能被描述成 Ontology 已经实现。

最接近的已有能力是：

- `workspace.catalog`：有界页面目录投影；
- `resourceRef`：领域资源引用；
- `muse.domain-change/v1`：可路由的领域变化；
- `muse.presentation-intent/v1`：Surface 意图；
- Document proposal/apply/event：受控写入闭环；
- Plugin Manifest/Contract discovery：类型和能力的组合基础。

Ontology 应在这些抽象之上建立，不能把 `workspace.catalog` 直接扩成图数据库。

## 6. 协议与实现不一致项

| 编号 | 现状 | 风险 | 目标处理 |
|---|---|---|---|
| G1 | TS/Rust/Dart 均使用裸 `String resourceRef`，部分调用直接承载 viewId | ID 被误当权限；命名冲突 | 引入 branded opaque ref + Host 私有 locator + scoped grant |
| G2 | Document TS 内存实现使用数字 revision，JSON Schema 要求 sha256 revision | 合同测试与真实 Provider 语义漂移 | 冻结统一 Revision token 规则或显式区分 monotonic/etag |
| G3 | `muse_appflowy_present` 直接接受 `viewId`，receipt 当前只返回 `ok` | 模型可传未从发现得到的 ID；动作结果不可追踪 | 只接受已授权 `resourceRef`/坐标；持久化最终 receipt |
| G4 | parent-bridge 仍 switch `parent-hello/workspace.bind/context.contribute` | 运输层理解业务 | 逐步变为 Host Bridge envelope 的 Web/Mobile transport adapter |
| G5 | Desktop DSH WebView 没有通用 Intent 下行 | 本地交付物无法打开 Flutter Surface | 通过 Desktop Host Bridge/FFI 事件进入 Surface Runtime |
| G6 | Renderer Registry 与 Surface Runtime 是两个相邻机制，未形成统一打开事务 | 能注册 renderer，不会自动创建/聚焦 Surface | 新增 Surface Orchestrator，拥有路由、实例去重和回执 |
| G7 | Office Manifest 在客户端维护扩展名，DSH 交付物维护路径，Viewer 也会维护 MIME | 格式规则多处漂移 | Format Descriptor 由插件拥有；核心只消费 capabilities |
| G8 | Context/Domain/Intent 三类 envelope 已有，但缺通用资源事件 | 移动、重命名、revision、anchor 失效无法统一表达 | 新增 `muse.resource@1` 合同与 Resource Change event |

## 7. 可复用与禁止复用

### 7.1 直接复用

- Host Bridge 的 discover/bind/invoke/subscribe/policy/cancel/status。
- Rust Host Registry 的 authority revalidation、schema gate、lease 与 shutdown。
- Plugin Graph v2 的 runtime/platform/grant/provider selection。
- Flutter Surface Runtime 的 lease、focus、context、domain change 与 intent receipt。
- DSH `present.open` 的坐标化、session event 回读、stat 和路径往返校验。
- Document Contract 的 propose/apply/idempotency/event 模式。

### 7.2 作为兼容适配器保留

- DSH `files[].path` 交付声明。
- Chat Sidebar 的 `openFile(path)`。
- Web/Mobile parent-bridge 的现有载波。
- AppFlowy ViewLayout 与 Office blob 存储。

### 7.3 不应扩展为核心

- `SessionControllerInternals` 测试注入点。
- 按扩展名硬编码的中央路由表。
- 把 `viewId`、文件路径或 URL 直接升级为全局资源身份。
- 将 Markdown 的文本 mutation 当成所有格式的统一 patch。
- 让 Agent Loop 理解 AppFlowy、Office、Ontology 或 Surface 业务。

## 8. 结论

当前实现已经拥有强大的“协议、能力、Facet、Surface、审批、事件”底座，真正缺失的是三个组合层：

1. **Resource Fabric**：把 path/view/blob/url 统一成受权资源身份与交付能力；
2. **Surface Orchestrator**：把发现、路由、物化、打开、聚焦、回执连成事务；
3. **Ontology Runtime**：把资源片段投影成语义对象、关系、动作与影响传播。

这三个层应以新增合同和插件实现，不应重写 Host Bridge、Agent Loop 或现有编辑器。
