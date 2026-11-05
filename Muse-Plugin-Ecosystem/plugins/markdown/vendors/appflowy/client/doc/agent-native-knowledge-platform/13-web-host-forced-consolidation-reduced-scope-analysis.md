# Web Host 强制收口到 Client：精简功能方案分析

状态：**条件可行性分析 v1.1（2026-09-17，含 Web 构建与数据面复核）**  
前提：决策层仍要求收口；不要求 `Muse-Clients/frontend/web` 的功能在 Flutter Client 中 1:1 还原；Web 版只需可运行、Host 框架一致、核心闭环成立，允许延期或永久缺失非核心功能。  
关联文档：[12-web-host-merge-analysis.md](12-web-host-merge-analysis.md) 评估的是接近现有 Web 对等替代；本文改用“精简 Web”前提重估。

## 0. 结论先行

在新的约束下，**可以有条件合并**，但“合并”的正确含义必须是：

> 以 `frontend/client` 为唯一 Host UI 源码，在其中增加一个 Web entrypoint、Web capability profile 和 Web remote backend；逐步下线 React Host。不能把原生 FFI、PTY、文件系统硬搬到浏览器，也不应逐行翻译现有 React 页面。

推荐目标不是“AppFlowy Flutter 全功能 Web 化”，而是一个 **OpenMuse Web Core**：

1. 与桌面一致的 Host Shell：品牌、侧栏、Workspace、Tab、命令入口、主题；
2. 登录与云 Workspace；
3. 页面树和 Markdown/块文档的创建、打开、编辑、保存；
4. DSH Agent 远程实例及 iframe 贴附；
5. 基础同步、冲突保护和可观测性；
6. 不可用能力按 Web capability profile 隐藏，不出现“点了必失败”的菜单。

复核后必须区分“壳层样机”和“连接真实 Cloud 的可运行 Web”。当前没有 Web Remote Backend，现有 Flutter Cloud 测试仍通过 Rust FFI，不能直接复用为浏览器数据面：

- **壳层技术样机（fixture/假数据）：6–10 人周**
- **连接真实 Cloud 的纵向 MVP：乐观 23、最可能 40、悲观 65 人周**
- **可替代当前生产 Web 核心流：乐观 38、最可能 68、悲观 110 人周**
- 首发若要求现有质量的实时多人协作，应按生产替代区间上半部规划，不再使用 14–22 人周口径

合理团队是 2 名 Flutter、2 名 Web/Cloud/协议、0.5–1 名 QA/DevOps。连接真实 Cloud 的 MVP 最可能需要 2–3 个日历月；生产切换最可能需要 4–8 个月。单人实施不建议作为交付方案。

精简范围删除了大量功能工作，但 Web 数据面、FFI 解耦和生产浏览器质量属于固定成本，所以生产替代的 likely 估算仍与全量迁移处在相近数量级；它仍不是“把 `web/` 移进 `client/`”或执行一次 `flutter build web`。

## 1. 新旧问题定义的差异

### 1.1 上一份分析的假设

文档 12 假设新 Flutter Web 基本替代现有 React Web，包括：

- 云协作完整体验；
- Database/Grid/Board/Calendar；
- 发布、邀请、计费；
- 多种 Web Engine occupant；
- 当前 Web 路由和产品能力基本对等。

在这个假设下，不建议合并，且估算是 50–90+ 人周。

### 1.2 本文的假设

本文允许：

- 删除或延期非核心功能；
- Web 不具备本机目录、Finder、原生 PTY、原生 Office；
- 首发只支持文档型知识工作；
- 高级协作、发布、计费、Database 可以延期；
- Web UI 以 Client Host 架构为准，而不是复刻 React Web；
- 只要核心用户可以登录、找到 Workspace、编辑文档、使用 DSH 并可靠保存，即可进入受控发布。

因此问题从“迁移 228k 行 React 产品”变成：

> 能否从 493k 行 Flutter Client 中切出一个不依赖本机 FFI 的 Host Shell，并为核心文档闭环提供 Web 数据适配器？

答案是：**可以，但数据 seam 是主工程，不是 UI seam。**

## 2. 当前状态：离 Web 可运行还有多远

### 2.1 两端规模

2026-09-17 工作区计数：

| 树 | 文件数 | 源码行数 |
|---|---:|---:|
| `frontend/web/src`（TS/TSX） | ~1,583 | ~228,321 |
| 其中 `components/dsh-agent`（含测试） | — | ~3,268 |
| `appflowy_flutter/lib`（Dart） | ~1,759 | ~492,694 |
| `client/frontend/rust-lib`（Rust） | ~857 | ~216,486 |
| Muse 增量 Dart（DSH/Resource/Diff/Office/Word/Workspace） | — | ~13,928 |

精简合并不需要翻译 228k 行 React，但需要让 Flutter Host 的启动、数据、平台服务具备可替换边界。

### 2.2 已进行真实 Web 构建探针

执行：

```bash
flutter build web --release --no-pub
```

当前结果：**编译失败**。首个根因是：

```text
Error: Dart library 'dart:ffi' is not available on this platform.
main.dart => appflowy_backend => dart:ffi
```

编译器共报告 287 处 `dart:ffi` 不可用，随后产生大量 `Pointer`、`Struct`、`Allocator` 等级联错误。数字不表示有 287 个独立设计问题，但证明 FFI 已进入主 entrypoint 的依赖闭包。

静态扫描（口径不同需分别看）：

| 项目 | 数量 |
|---|---:|
| `lib` 中直接 import `dart:io` 或 `dart:ffi` 的文件 | **116** |
| 整个 `appflowy_flutter` 树中直接 import `dart:io` 的文件 | **176** |
| 整个树中引用 `dart:ffi` 的文件 | 至少 **9** |
| `lib` 中使用 `kIsWeb` 的文件 | **4** |
| 整个树中 `kIsWeb` 引用 | **14 处 / 8 个文件** |
| `lib` 中已使用 platform conditional import 的文件 | **1** |

这说明当前工程是“少数地方知道 Web”，不是“原生/Web 双实现”。

### 2.3 启动路径是硬阻塞

`lib/main.dart` 直接调用 `runAppFlowy()`；`startup/startup.dart` 无条件加入：

```text
InitRustSDKTask
PluginLoadTask
FileStorageTask
...
```

`InitRustSDKTask`：

- import `dart:io`；
- 构建本机数据目录；
- 读取 `Platform.operatingSystem`；
- 调用 `FlowySDK.init()`。

`packages/appflowy_backend/lib/appflowy_backend.dart`：

- import `dart:ffi`；
- 直接使用 `NativeApi`、`Pointer` 和 `ffi.init_sdk()`。

`packages/appflowy_backend/pubspec.yaml` 只声明 Android/iOS/macOS/Windows，没有 Web plugin 实现。Rust `dart-ffi/Cargo.toml` 只有：

```toml
crate-type = ["staticlib"]
```

所以当前 Client 不能靠 tree shaking 绕过 FFI；必须在 entrypoint 和 backend 接口处切断依赖闭包。构建日志还出现 `win32` 等桌面传递依赖在 dart2js 下失败，Web plugin 白名单也是 G1 的必要工作。

### 2.4 有利条件

并非所有代码都必须推倒：

- AppFlowy Editor UI 大量为纯 Dart/Flutter，可复用呈现和编辑交互；
- Host Shell、Tab、主题、侧栏、BLoC 组织方式可复用；
- 已出现 `WorkspaceRepository` 等抽象，注释明确支持 REST/gRPC/local；
- Cloud protobuf 模型在 Dart 侧已有；
- Web 当前 REST/WS API 是可参考的已工作实现；
- DSH 业务合同已经按 placement 分离；
- `web/index.html` 脚手架存在；
- Resource/Surface 架构已有 capability 概念，适合在 Web 隐藏本机能力。

不利条件是这些 seam 尚未贯穿 Document、Folder、Sync、Auth 全链；很多 BLoC 仍直接调用生成的 Rust Backend Service。代码库中没有 `WebRemoteBackend`、没有与 React `js-services/http` 等价的 Dart HTTP dispatch。现有桌面 Cloud 集成测试证明 Rust Client 能连 Cloud，**不证明浏览器数据面存在**，因为启动仍无条件执行 `InitRustSDKTask`。

## 3. 精简 Web 的核心功能定义

如果不冻结核心范围，“允许缺功能”会在实现中不断反弹，最终回到 50–90 人周。因此必须先冻结 Web Core。

### 3.1 首发必须有

| 域 | 最小能力 | 验收 |
|---|---|---|
| Host Shell | 与 Client 同品牌、侧栏、顶部 Tab、主题、基础设置 | 同一 Flutter Widget 和 token，不另写 Web 壳 |
| Auth | 邮箱/已有 Cloud 登录，token 刷新，退出 | 刷新页面后会话恢复；失效 token 安全退出 |
| Workspace | 列表、切换、页面树 | 能打开既有 Workspace，树状态可恢复 |
| Page | 创建、重命名、移动、删除（可先无回收站） | 服务端持久化，刷新不丢 |
| Document | Markdown/基础块打开、编辑、自动保存 | 1,000 块文档可用；刷新无数据丢失 |
| Sync | 单用户同步、revision/ETag 冲突保护 | 旧 revision 不能静默覆盖新内容 |
| DSH | 云端实例、iframe、附件状态、当前文档上下文 | 能到 `HybridLive`，只接受允许 origin 的消息 |
| Navigation | 可复制 URL、深链到 workspace/view | 刷新深链可恢复相同页面 |
| Capability | 不支持功能被隐藏或明确只读 | 不出现 Finder/本机 Helix/原生 Word 等假入口 |
| Observability | 启动、打开、保存、DSH、错误指标 | 能区分 UI、API、同步、iframe 故障 |

### 3.2 建议首发明确不做

- Grid / Board / Calendar 高级数据库；
- Publish / Sites；
- Billing / Subscription；
- 邀请、成员管理、全局评论；
- Quick Note、模板市场；
- 全量 Import / Export；
- 本地 Project Workspace；
- `system-open`、Finder/Explorer reveal；
- 本机 Helix PTY；
- iOffice 原生编辑；
- 完整 Version Diff Workbench；
- 完整离线优先和多设备冲突合并；
- Safari/Firefox 的所有高级编辑兼容（可以分级支持）。

### 3.3 可在 Core 后增量恢复

优先级建议：

1. 实时多人协作和 presence；
2. 图片/附件；
3. 基础 Version Diff（纯 Dart 或服务器 diff）；
4. 只读 Web Viewer；
5. Grid 只读；
6. 远程 Helix；
7. wasm iOffice；
8. 发布、计费等 Web 商业功能。

## 4. 推荐目标架构

### 4.1 不是一个 entrypoint，而是一个 Host 源码加两个 composition root

```text
frontend/client/frontend/appflowy_flutter/
├── lib/
│   ├── main.dart                  # native composition root
│   ├── main_web.dart              # web composition root
│   ├── host/                      # 共享 Shell / Tab / Theme / Navigation
│   ├── domain/                    # 共享 Resource / Workspace / Document contracts
│   ├── backend_native/            # rust-lib / FFI / filesystem
│   ├── backend_web/               # HTTP / WS / IndexedDB / browser auth
│   └── platform/
│       ├── native/                # PTY / WebView / Process
│       └── web/                   # iframe / postMessage / URL
└── web/                           # Flutter bootstrap、CSP、service worker
```

最终是一套 Flutter Host 源码，但不是“所有代码在所有平台编译”。Native 和 Web 必须在 composition root 通过 conditional import 选择。

### 4.2 推荐：Web Remote Backend

Web 端不初始化 `FlowySDK`，而是注入：

- `WebAuthRepository`
- `WebWorkspaceRepository`
- `WebPageRepository`
- `WebDocumentRepository`
- `WebSyncRepository`
- `WebDshAttachment`
- `WebBrowserStorage`

它们直接调用 AppFlowy Cloud/Muse API（HTTP + WebSocket），并使用 IndexedDB 做轻量缓存。

重点是按 Core 用例实现 repository，不要试图在 Web 端模拟完整 `appflowy_backend` FFI API。后者包含整个 AppFlowy 产品面，会把精简方案重新放大。

### 4.3 文档数据是最大风险

现有 Web 使用 Yjs/Slate，Cloud API 上传 `doc_state`、state vector 和 protobuf sync。Client 使用 Rust/Yrs 和生成的 FFI service。Flutter 编辑器本身能画内容，但浏览器中没有现成的 Rust document manager。

有三种选择：

#### 方案 A：服务端规范块文档 API（推荐 Core MVP）

增加面向 Host 的文档快照/patch API：

```text
GET  /muse/workspaces/{wid}/documents/{id}
PUT  /muse/workspaces/{wid}/documents/{id}
If-Match: revision
```

服务端负责规范块模型与既有 Collab 的转换。Flutter Web 只处理 Dart block model 和 revision。

优点：

- 不把 Yjs 引进 Dart；
- 不把完整 rust-lib 编译 wasm；
- 单用户编辑和冲突保护能较快交付；
- 以后实时协作仍可在此 seam 下扩展。

代价：

- 需要 Cloud/Backend 配合；
- 首发不是实时逐操作协同；
- 转换必须保证未知 block 不丢失。

#### 方案 B：Rust/Yrs 编译 wasm，再由 Dart JS interop 调用

理论上模型更接近 Client，但当前 rust-lib 含 RocksDB/SQLite/Tokio/本机 FFI，不是直接 `wasm32` 可编译。必须裁出 document-only wasm crate、JS wrapper 和 Web Worker。

这适合长期实时协作，不适合最短 Core MVP。预计单独增加 8–14 人周且工具链风险高。

#### 方案 C：Flutter Shell 嵌现有 React 编辑器

最快能“看起来统一”，可用 `HtmlElementView/iframe` 把旧编辑器嵌进 Flutter Shell。

但这不满足“只维护一套前端代码”：React editor、Yjs、主题和桥仍要维护。只能作为 1–2 个版本的迁移桥，不能作为目标架构。

### 4.4 DSH Web 适配

Desktop 是 WebView + 本机 sidecar；Web 必须是 iframe + 云端实例：

- Flutter Web 创建受控 iframe platform view；
- 使用 `postMessage`；
- 固定 allowlist origin；
- attachment/session/token 仍使用共享业务状态机；
- 浏览器端只传 opaque `resourceRef`，不能传服务端绝对路径；
- CSP `frame-src`、sandbox、token 生命周期必须和现有 React 实现对齐。

这部分可以参考 `frontend/web/src/components/dsh-agent` 的约 3.3k 行实现和测试，但应把状态机合同搬到跨端 fixture，而不是翻译 React 组件。

### 4.5 Capability Profile

Web 启动时固定声明：

```text
document.edit.basic       = true
workspace.cloud           = true
dsh.remote                = true
filesystem.local          = false
process.local             = false
engine.helix.local        = false
engine.ioffice.native     = false
system.open               = false
system.reveal             = false
versionDiff.nativeFfi     = false
```

菜单、路由、Agent capability 广告都读同一 profile。禁止在几十个 Widget 中散落 `if (kIsWeb)`。

## 5. 实施工作量

### 5.1 阶段拆分

| 阶段 | 工作 | Optimistic | Likely | Pessimistic | 退出条件 |
|---|---|---:|---:|---:|---|
| G0 范围/合同冻结 | Core 功能、能力矩阵、API、路由、SLO | 1 | 2 | 3 | ADR + contract fixtures 冻结 |
| G1 Web 可编译骨架 | `main_web.dart`、条件导入、移除 FFI closure、Web DI、桌面插件裁剪 | 6 | 10 | 16 | `flutter build web` 通过，Shell 可打开 |
| G2 Host Shell | Sidebar/Tab/Theme/Router/响应式、Web capability | 2 | 4 | 6 | 无数据模式可交互，桌面无回归 |
| G3 Web Auth/Workspace/Page | Cloud HTTP、token、树 CRUD、深链 | 5 | 8 | 13 | 登录后能创建并恢复页面 |
| G4 Document Core | WebRemoteBackend、块模型、读取、编辑、保存、基础同步 | 7 | 14 | 24 | 刷新不丢；并发覆盖被拒绝 |
| G5 DSH iframe | session、attachment、postMessage、CSP | 2 | 4 | 7 | 当前文档上下文能到 DSH |
| G6 内测质量 | IndexedDB、CI、Chrome/Safari smoke、基本监控 | 3 | 6 | 10 | 纵向 MVP 可持续内测 |

上表存在可并行项，不能机械相加成日历时间；三点总估算已考虑一定复用与并行：连接真实 Cloud 的纵向 MVP为 **23 / 40 / 65 人周**。生产替代还需增加完整浏览器回归、性能/SEO、协作稳定性、灰度和运维工作，增量约 **15 / 28 / 45 人周**。

### 5.2 三档估算

| 档位 | 目标 | 人周（O/L/P） | 典型日历 |
|---|---|---:|---:|
| 壳层样机 | Fixture/假数据、Shell、一个本地文档、DSH 外观 | 6–10（范围估算） | 3–5 周 |
| 真实 Cloud 纵向 MVP | 登录、Workspace、一个文档保存、DSH、基本部署 | **23 / 40 / 65** | 2–3 个月（Likely） |
| 精简生产替代（推荐承诺口径） | 纵向 MVP + 协作稳定性、浏览器回归、性能、监控、回滚 | **38 / 68 / 110** | 4–8 个月（Likely） |
| 接近现 Web 对等 | Database/Publish/Billing/高级引擎等 | 50–90+，并可能更高 | 不建议 |

O/L/P 分别为 optimistic / likely / pessimistic。旧的 14–22 人周只足以覆盖理想化 Core 开发，未充分计入 176 个 `dart:io` 文件的依赖裁剪、无 WebRemoteBackend、生产浏览器质量和 Cloud 文档协议，因此不再作为立项口径。

### 5.3 团队配置

推荐：

- Flutter/架构 1：composition root、DI、Host Shell、平台 seam；
- Flutter/编辑器 1：Document Core、状态、性能；
- Web/Cloud/协议 2：Auth、REST/WS、Collab、CSP、DSH iframe、服务端转换；
- QA/DevOps 0.5–1：Chrome/Safari、Playwright、RUM、灰度。

如果没有 Cloud/Collab 经验，G4 风险会显著上升。只增加 Flutter 人手不能消除文档数据格式问题。

## 6. 主要风险与控制

### 6.1 技术风险

| 风险 | 程度 | 控制 |
|---|---|---|
| FFI 耦合比扫描更深 | 高 | G1 做纵向编译 spike；只编译 Core plugin，不给全产品打 Web 补丁 |
| Document block 与 Collab 转换丢数据 | 高 | 未知 block round-trip fixture；服务端转换；原始 doc_state 保留 |
| 实时协作延期引起产品预期错位 | 高 | 首发明确“单用户保存 + 冲突提示”，不宣传实时协作 |
| Flutter Web 包体/冷启动 | 高 | 独立 Web entrypoint、插件白名单、延迟加载；设硬包体门 |
| 浏览器文本/IME/选择 | 中高 | 中文/日文/韩文 IME TCK；Safari 单独预算 |
| iframe 与 Flutter pointer/keyboard | 中 | 全屏 drag shield、焦点回收、postMessage origin TCK |
| 上游升级冲突 | 中高 | Muse Host 保持薄层；不要修改 AppFlowy 基础 Widget 大面积支持 Web |
| 无障碍与 DOM 集成 | 中高 | Semantics/键盘导航测试；关键登录/导航保留 DOM 语义 |

### 6.2 产品风险

- 当前 Web 用户可能依赖 Publish、Database、邀请；必须用实际遥测确认，而不是主观认定“非核心”。
- “框架一致”不等于桌面能力全部显示。隐藏能力比显示灰色失败入口更重要。
- 如果协作本身是 Web 付费/团队用户的核心，不能把它归类为非核心；此时应采用 **38 / 68 / 110 人周生产替代口径**，并优先投入 G4 的 Collab seam。
- Web 用户通常比桌面用户更在意首次加载、URL 分享、浏览器后退和刷新恢复。

### 6.3 安全风险

- token 不得写入可被任意脚本读取的长期 localStorage；优先 HttpOnly cookie 或短期 token；
- DSH iframe 必须严格校验 `origin`、`source`、消息 schema 和大小；
- CSP 必须限制 `frame-src`、`connect-src`、`script-src`；
- Web 不接收本机路径，跨边界只传 `resourceRef`；
- 多租户 Workspace/Document API 必须服务端授权，不能依赖 Flutter 隐藏按钮；
- IndexedDB 缓存需按用户和 workspace 隔离，退出时清理密钥/敏感索引。

## 7. 收益

在精简目标下，收益比“全量对等合并”更合理：

1. **Host 交互一份。** Sidebar、Tab、菜单、主题、DSH 面板壳和未来 Agent 原生交互只在 Flutter 修改。
2. **功能优先级收敛。** 不再被 AppFlowy-Web 全功能面牵着走，Web 只交付 Muse Core。
3. **测试复用。** 纯 Widget/Domain 测试可跨 Native/Web；能力矩阵和合同 fixture 一份。
4. **品牌一致。** 不再分别维护 React token 和 Flutter token。
5. **组织简化。** 前端主团队以 Flutter 为主，Web/Cloud 人力集中在 adapter，不维护完整 React 产品。
6. **长期 Host 架构统一。** Resource/Surface/AgentAttachment 能在同一 UI 架构演进。

但不能把收益夸大为“维护成本减半”：

- DSH 自身仍是 Web/TS；
- Cloud API、浏览器适配、CSP、IndexedDB 仍需 Web 专长；
- Native/Web backend 是两套实现；
- Web 和 Native 仍是两条构建、测试、发布流水线。

合理预期是：

- Host UI 重复维护减少 **60–80%**；
- 整体前端维护成本减少约 **20–35%**；
- Web 特有故障不会消失，只从 React UI 转移到 Flutter Web adapter。

以上是规划假设，需在两个版本后用变更工时、缺陷和发布次数验证。

## 8. 性能现状与验收指标

### 8.1 当前 React Web 可测基线

当前 `frontend/web/dist`：

| 指标 | 当前值 |
|---|---:|
| 完整 dist（未压缩，含懒加载资源/cover） | ~30 MB |
| 初始 `index.js` | 1,529,759 B raw / 349,525 B gzip |
| 初始 `common.js` | 850,230 B raw / 224,229 B gzip |
| 初始 CSS | 142,595 B raw / 25,263 B gzip |
| 主要初始静态传输合计 | **约 599 KB gzip** |

这不是完整网络瀑布，也不含 API、字体与后续 lazy chunks，但可作为 Flutter Web 包体比较基线。

### 8.2 Flutter Web 预算

Flutter Web 不太可能达到 599 KB 初始传输。为了避免“统一代码但 Web 体验不可用”，建议设两级门：

| 指标 | 目标 | 硬门 |
|---|---:|---:|
| 首屏压缩静态传输（不含业务 API） | ≤ 4 MB | ≤ 6 MB |
| FCP p75（桌面宽带） | ≤ 1.8 s | ≤ 2.5 s |
| LCP p75（桌面宽带） | ≤ 2.5 s | ≤ 3.5 s |
| 可交互 TTI p75（桌面宽带） | ≤ 3.5 s | ≤ 5 s |
| 4G 中端设备 LCP p75 | ≤ 4 s | ≤ 5.5 s |
| warm reload 到可操作 | ≤ 1.2 s | ≤ 2 s |

若首屏超过 6 MB gzip 或桌面 LCP p75 超过 3.5 秒，不应切 100% 流量，应继续拆 plugin 和 deferred loading。

### 8.3 交互与业务 SLO

| 指标 | 目标 |
|---|---:|
| 编辑器按键到画面 p95 | < 50 ms |
| 滚动（1,000 块文档） | 桌面 ≥ 55 FPS；中端设备 ≥ 45 FPS |
| 打开 Workspace（warm）p95 | < 1.5 s |
| 打开 1,000 块文档（warm）p95 | < 2 s |
| 自动保存 ACK（同区域）p95 | < 800 ms |
| revision 冲突发现 | < 2 s，且不静默覆盖 |
| DSH attach 到 `HybridLive`（warm）p95 | < 3 s |
| 页面刷新后的已确认数据丢失 | 0 |
| 崩溃/致命白屏会话占比 | < 0.5% |
| API 失败率（排除用户 4xx） | < 1% |

### 8.4 内存预算

建议在 Chrome/Edge 桌面测：

- 1,000 块文档、DSH 关闭：JS/WASM heap + Flutter runtime **≤ 250 MB**；
- DSH iframe 打开后的增量 **≤ 150 MB**；
- 连续切换 30 个文档后稳定内存增幅 **≤ 20%**；
- 关闭 DSH 后 iframe 相关资源在 30 秒内可回收。

### 8.5 测量方法

- CI：bundle size、Lighthouse lab、Chrome integration tests；
- 预发布：Web Vitals RUM（p50/p75/p95，按浏览器和网络分桶）；
- 编辑器：Performance Timeline 标记 open/edit/save；
- DSH：沿用 attachment stage 时间戳；
- 每个指标同时记录当前 React Web 和 Flutter Web，不能只看新版本绝对值。

## 9. 测试与发布策略

### 9.1 测试金字塔

1. Domain/Repository contract tests：Native 与 Web backend 跑同一用例；
2. Widget tests：Host Shell、菜单 capability、文档编辑；
3. Golden：桌面与 Web viewport 各一套，但共享 token；
4. Chrome integration test：登录 → Workspace → 编辑 → 保存 → 刷新；
5. DSH E2E：iframe attach、origin 拒绝、重连；
6. Playwright 外层验证：URL、刷新、浏览器前进后退、CSP；
7. Safari 手工 + 自动 smoke；中文 IME 专项。

### 9.2 灰度

建议：

```text
内部账号
  → 1% 新会话
  → 10%
  → 50%
  → 100%
  → 保留 React 回滚入口 2 个稳定版本
```

切换期间：

- 用户数据继续留在同一 Cloud，不做内容搬迁；
- 旧 React IndexedDB 缓存不迁移，登录后从服务端重建；
- 保持 URL 兼容或做 301/客户端映射；
- 任一硬门失败可将 `/app` 路由切回 React；
- React 进入 feature freeze，而不是第一天删除。

## 10. 推荐实施顺序与决策门

### Gate 0：产品范围（1 周）

必须由产品确认：

- 实时多人协作是否首发核心；
- 现网 Database/Publish/Billing 使用率；
- Safari 是否硬支持；
- 匿名本地模式还是 Cloud 登录模式。

没有这四个答案，不应对真实 Cloud Web 承诺低于 **40 人周 likely**。

### Gate 1：Web 编译纵切（2–3 周）

目标：

- 新 `main_web.dart`；
- Web DI 不 import `appflowy_backend`；
- 只注册 Shell + 一个 fixture document plugin；
- `flutter build web` 通过；
- 首屏压缩包小于 6 MB。

失败条件：必须修改数百个基础 Widget 才能切断 FFI。若失败，应停止，而不是继续给全仓散布 `kIsWeb`。

### Gate 2：真实文档持久化（3–5 周）

打通登录、Workspace、一个文档、revision save。此 Gate 决定项目真正可行性。

失败条件：

- Cloud 无法提供安全快照/patch seam；
- block round-trip 丢未知内容；
- 只能把整个 React editor 嵌回来。

### Gate 3：DSH + 生产质量（4–6 周）

补 iframe、指标、浏览器测试、灰度。通过后才进入替代阶段。

### Gate 4：React 退休

只有满足：

- 核心任务成功率不低于 React 基线；
- 连续两周无 SLO 硬门失败；
- 数据丢失为 0；
- 回滚演练成功；
- 被延期功能已有明确产品告知；

才停止 React Web 的日常维护。

## 11. 最终建议

在“必须合并、允许 Web 缺失非核心功能”的前提下，建议从上一份的“不合并”调整为：

> **有条件支持合并，但只支持 Core-first 的重新组合，不支持全仓 Web 化，也不支持逐页翻译 React。**

项目应按 **38 / 68 / 110 人周（O/L/P）** 的“精简生产核心替代”立项，而不是按 6–10 人周壳层样机对外承诺。连接真实 Cloud、但尚未达到生产替代质量的纵向 MVP，采用 **23 / 40 / 65 人周**。

最关键的架构纪律：

1. 共享 Host UI 和 Domain Contract；
2. Native 用 FFI backend，Web 用 remote backend；
3. entrypoint 层条件组合，禁止到处 `kIsWeb`；
4. Web capability 明确裁剪本机功能；
5. 文档转换由受测 seam 负责，未知 block 必须 round-trip；
6. React 保留为可回滚版本，直到 Flutter Web 达到生产 SLO。

这条路线确实能最终只维护一个 **Host UI 代码库**，但不会变成一个运行时或一个平台适配器。其收益来自统一产品框架，而不是消灭 Web 技术。

## 12. 证据索引

- Client/Web 定位：`Muse-Clients/frontend/{README.md,client/README.md,web/README.md}`
- 当前 Flutter 启动：`appflowy_flutter/lib/{main.dart,startup/startup.dart}`
- FFI 初始化：`lib/startup/tasks/rust_sdk.dart`
- Backend 无 Web：`packages/appflowy_backend/{lib/appflowy_backend.dart,pubspec.yaml}`
- Rust 仅静态库：`client/frontend/rust-lib/dart-ffi/Cargo.toml`
- Workspace repository seam：`lib/features/workspace/data/repositories/workspace_repository.dart`
- 当前 React Cloud API：`web/src/application/services/js-services/http/`
- 当前 React Collab：`web/src/application/services/js-services/http/collab-api.ts`、`web/src/components/ws/`
- DSH Web 实现：`web/src/components/dsh-agent/`
- 多端 capability 与 K6：`client/doc/agent-file-references-and-open-routing.md` §9
- D12：`agent-native-knowledge-platform/11-decisions-and-open-questions.md`
- iOffice Web 路径：`engine-delivery/04-ioffice-implementation-plan.md`
- Helix Web 路径：`client/doc/helix-editor-integration.md`
