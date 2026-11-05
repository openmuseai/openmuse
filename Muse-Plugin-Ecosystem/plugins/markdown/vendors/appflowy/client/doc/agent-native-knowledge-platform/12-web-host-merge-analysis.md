# 是否将 `frontend/web` 合并进 `frontend/client`

状态：**分析结论 v1（2026-09-17）**  
性质：对「两套 Host 能否收成一套 Flutter 代码」的现状、工作量、风险、收益与性能评估。不是实施计划。  
结论先行：**不要把 Web Host 并入 Flutter Client。维持双 Host + 一份合同 / 一份 DSH 插件面。**

---

## 0. 问题与边界

当前需要同时维护：

| 路径 | 产品角色 | 技术栈 |
|------|----------|--------|
| `Muse-Clients/frontend/client` | 桌面 / 移动 Host（OpenMuse.app） | Flutter ≥ 3.27 + Rust `dart-ffi` |
| `Muse-Clients/frontend/web` | 浏览器 Host（openmuseai.com `/app`） | React 18 + TypeScript + Vite 6 |

两者在「文档 / 数据库 / 侧栏 / DSH Agent 面板」上功能大体相近。Client 已把 AppFlowy 当作基座，Markdown 与协作是上层应用，并在其上长出 Workspace Platform、Resource Surface、Version Diff、Helix / iOffice 等 Muse Host 能力。

问题是：能否把 Web 也并进 `frontend/client`，只维护一套代码。

本分析比较的是 **Host 壳**（用户看到的 OpenMuse 客户端），不是：

- DSH 自己的 Web UI（Cordis / Harness，两端都是嵌入，并掉 Host 也消不掉）
- `middlewares/dsh` 插件面（已经共享）
- 官网营销站 `frontend/website`

「一套代码」若只指「业务合同与路由一份」，仓库里已经在做。若指「浏览器里跑同一份 Flutter Host」，这是一次换数据面与引擎面的重写，不是搬目录。

---

## 1. 现状拓扑

仓库约定见 `middlewares/scripts/lib/muse-paths.sh`：

```text
frontend/client     Flutter + rust-lib
frontend/web        AppFlowy-Web clone
middlewares/dsh     Muse TS 包（两端 DSH 共用）
```

`frontend/README.md` 写明：`web/` 是 upstream clone，相对 dsh-office 父仓 gitignore；Client 才是对外打包的桌面/移动产品。

### 1.1 两端对照

| | **Client** `frontend/client` | **Web** `frontend/web` |
|--|--|--|
| 入口 | `appflowy_flutter/lib/main.dart` | `web/src/main.tsx` |
| 清单 | `pubspec.yaml` + `rust-lib/Cargo.toml` | `package.json`（name 仍为 `appflowy_web_app`） |
| UI | Flutter | React 18 + Vite |
| 数据面 | Rust `flowy-core`，`dart-ffi` **仅 `staticlib`** | JS HTTP 服务 + Yjs/Slate + Dexie |
| 协作 | 本地 CRDT（rust-lib）；可选 Cloud | **云原生**：Yjs + AppFlowy Cloud / GoTrue |
| DSH 放置 | 本机 Node sidecar + WKWebView / WebView2 | **云端/租户 DSH** + iframe `postMessage` |
| 账号 | README：**Local mode only**，无登录 | 登录 / OAuth / 邀请 / 计费 |
| 分发 | `.app` / zip / APK / IPA | Nginx + Docker，同域 `/app` |
| 上游 | AppFlowy Flutter | AppFlowy-Web |

两边都不是 Electron / Tauri。桌面是 Flutter 原生窗；浏览器是 Vite SPA。

### 1.2 规模（2026-09-17 工作区计数，排除 `node_modules` / `target`）

| 树 | 文件数 | 源码行数 |
|----|--------|----------|
| `web/src`（`.ts` / `.tsx`） | ~1.6k | **~228k** |
| 其中 `web/src/components/dsh-agent` | — | ~3.3k |
| `appflowy_flutter/lib`（`.dart`） | ~1.8k | **~493k** |
| `rust-lib`（`.rs`） | ~0.9k | **~216k** |
| Muse 增量 Dart（dsh / resource / version-diff / office / word / workspace_platform） | — | ~14k |

Client 的「Muse Host 重构」相对 AppFlowy 基座是薄层；Web 的 Muse 增量主要是 DSH 嵌入，不是第二份 Version Diff / 本地目录树。

---

## 2. 为什么 Web 不是 Flutter

不是历史偶然，而是产品与引擎约束，且已写进设计基线。

### 2.1 上游就是双客户端

AppFlowy 官方就是 Flutter 桌面/移动 + 独立 React Web。Muse 分别 fork。把 Web 改成 Flutter，等于永久偏离上游浏览器策略，后续无法再跟 AppFlowy-Web。

### 2.2 浏览器 GTM 需要零安装云 Host

`web/docs/rebrand-plan-web.md`：

> 2026-08 dsh-office v1 **不含 Web**。官网「Start for free」已指向同域 `/app`，本轮必须做 Web 可见层。

Client README 定位是 **Local mode only**。浏览器用户要的是登录、协作、发布、邀请，而不是装包后的本机 FFI。

### 2.3 DSH 放置模型不同

[已冻结] ADR-D12 / 知识平台 D12：Desktop、Web、Mobile **共用合同，按 placement 裁剪能力**。

| 端 | Host UI | DSH |
|----|---------|-----|
| Desktop | Flutter | 本机 sidecar |
| Web | AppFlowy-Web | 云端实例 |
| Mobile | Flutter | 远程实例 |

嵌入合同可以一份；适配器必须两份（WebView vs iframe）。合并 Host UI 不会合并这两种放置。

### 2.4 引擎交付路径已经按栈拆开

`agent-file-references-and-open-routing.md` **K6**（已接受双实现）：

> Web 端客户端是 TS/React，不是 Flutter。承载器要写两份渲染（React occupant + Flutter occupant）；open-file-viewer 例外（本身是 Web 库）。

`engine-delivery/04-ioffice-implementation-plan.md` §9.2：

| 端 | 路径 |
|----|------|
| Desktop | 原生 dylib / FRB |
| **Web** | **wasm + React/Web Adapter，不走 Flutter FRB** |

Helix：桌面 `flutter_pty`；Web 规划是 **服务端 PTY + xterm.js**（`helix-editor-integration.md`：Web 端收益为零，协议可复用、前端另做）。

Word 在 Flutter 里已显式拒绝 Web：

```67:72:Muse-Clients/frontend/client/frontend/appflowy_flutter/lib/plugins/word/word_page.dart
    if (kIsWeb) {
      setState(() {
        _booting = false;
        _bootError = wordWebUnsupportedMessage;
      });
```

Office catalog 四类均 `desktopOnly: true`。

### 2.5 北极星原则并不要求一个 UI 框架

知识平台原则 6 / ADR-D12：**一套业务合同，多种传输**。Desktop UDS/FFI、Web HTTPS/postMessage、Mobile HTTPS/SSE 只是行程。把这一点理解成「一个 Flutter 工程」会和已冻结决策冲突。

---

## 3. 功能重叠真实有多大

「功能大体相同」成立于 **AppFlowy 知识库壳**（文档、Grid/Board、侧栏、账号空间）。Muse Agent 原生 Host 深度并不对称。

| 能力 | Web | Client | 说明 |
|------|-----|--------|------|
| Markdown / 数据库视图 | 云协作（Yjs/Slate） | 本地 CRDT + 可选云 | 同品类，不同数据面 |
| DSH Agent 面板 | iframe + session/open | WebView + sidecar | 合同相近，放置不同 |
| 本地 Project Workspace 目录树 | 无 | 有 | 浏览器无任意 FS |
| Resource Tab / 多引擎打开 | 弱（DSH intent） | Helix / iOffice / Viewer / Diff | Client 独有 |
| Helix | 未进 Web Host UI | 本机 PTY | Web 需服务端 PTY |
| iOffice Word | wasm MVP，未进 Host | 原生 FFI | 文档规定 Web 走 React wasm |
| Version Diff Workbench | **无** | Rust `muse-diff-text` + Flutter | Client 独有 |
| Finder / 系统打开 | 不可能 | 有 | 能力矩阵必须按端裁剪 |
| 发布 / 邀请 / 计费 | 强 | 弱 / 依赖 Cloud | Web 独有 |
| SEO / 同域 `/app` | 已上线 | 无 | 官网入口 |

重叠成本主要是 **DSH 面板两套各约 3k 行**，以及壳层视觉。Version Diff、Workspace Explorer、PTY、Word FFI 并不是「Web 再写一遍」，而是桌面才有的能力。把 Web 并进 Client，要么丢掉这些能力在浏览器上的未来形态，要么在 Flutter-web 里重做一遍 wasm/PTY/HTTP 数据面——后者比继续维护 React Host 更贵。

---

## 4. Flutter-web 能否承载当前 Client

`appflowy_flutter/web/index.html` 只是 Flutter 默认脚手架（标题仍是 `appflowy_flutter`）。没有 `build-*-web` 脚本，README 只列桌面/移动。`kIsWeb` 多处是 **降级/拒绝**，不是产品路径。

| 阻塞 | 证据 | 若强行上浏览器 |
|------|------|----------------|
| Rust 核心 `dart-ffi` | `dart-ffi/Cargo.toml`：`crate-type = ["staticlib"]` | 不能在浏览器加载；要 wasm 重写或改走 HTTP（等于再造 AppFlowy-Web） |
| 本地 PTY / Helix | `helix_resource_surface.dart` + `flutter_pty` | 浏览器不能起进程；只能服务端 PTY + xterm |
| 本地 FS / chmod / Process | Workspace explorer、Helix 工作副本 | 沙箱；File System Access ≠ 现有代码 |
| 原生 WebView 嵌 DSH | WKWebView / WebView2 | 浏览器 Host 用 iframe，不是套一层 Flutter-web WebView |
| iOffice FFI | `desktopOnly` + `kIsWeb` 早退 | 文档路径是 React wasm，不是 FRB |
| Version Diff FFI | `text_diff_runtime.dart` `dart:ffi` | 需 wasm 或纯 Dart，未产品化 |
| 桌面插件 | `window_manager`、`auto_updater`、`webview_windows`、`flutter_pty`… | web 上失败或空操作 |

**判断：** Flutter 可以在 Chrome 里画 Widget，但 **当前 Host 不是 Flutter-web 应用**。打开 `--web` 开关不会得到 OpenMuse Web，只会得到一个没有 rust-lib、没有 PTY、没有 Word 的空壳，然后再花一次 AppFlowy-Web 量级的工期去补数据面。

---

## 5. 已经共享的部分（不必靠合并 Host 获得）

| 层 | 是否共享 | 说明 |
|----|----------|------|
| `middlewares/dsh` | 是 | 同一 Cordis 插件面；桌面 sidecar / 云池 / 移动远程都吃这套包 |
| 资源协议 / 打开路由 | 设计上共享 | C1–C4 在宿主侧一份；C5 承载按端各算 |
| vendors（helix、ioffice、viewer） | 源码共享，交付不同 | 桌面 dylib/PTY；Web wasm/xterm |
| rust-lib | **仅 Client** | Web 走 JS + Cloud HTTP |
| DSH Web UI | 共享嵌入目标 | 第三张 Web 面，与 `frontend/web` Host 不是同一个工程 |
| Version Diff / Explorer | **仅 Client** | |

真正重复的是 Host 壳和 DSH attachment 适配器，不是整份产品。

---

## 6. 若强制合并：工作量

目标假设：下线 React Host，用 Flutter-web 顶 `openmuseai.com/app`，并尽量带上 Muse 引擎。

人周为量级估计，与 `agent-file-references-and-open-routing.md` §9.6 的 Web 承载器预算对齐后放大（因为那份预算假设 **保留 React Host**，只加 occupant）。

| 阶段 | 范围 | 人周 | 备注 |
|------|------|------|------|
| P0 浏览器壳 | Flutter-web 构建、CI、Cloud 登录、文档只读 | **12–20** | 必须先换掉 FFI 数据面（最大未知数） |
| P1 云 Host 对等 | DSH iframe、发布、DB 视图、AI Chat、路由/SEO | **20–35** | 接近重写 AppFlowy-Web |
| P2 下线 React | 切流、双 CI 收口、回滚窗口 | **4–8** | |
| 引擎迁到 Flutter-web | iOffice wasm 进 Flutter、Helix、Viewer、Diff | **+15–25** | 文档里 React wasm 单独已是 3–5 人周；塞进 Flutter 还要桥 |
| **合计** | | **~50–90+** | 高概率做不完云协作体验 |

对照：设计稿在 **保留 React Host** 的前提下，Web 引擎承载器合计约 **10–14 人周**（§9.6）。这才是「一套合同、两套 occupant」的成本。

不能原样搬迁、必须改写的：

- `staticlib` FFI + RocksDB/SQLite 桌面核 → HTTP/wasm 协作客户端（React 已有）
- 本机 Helix → 服务端 PTY 或放弃
- `system` / `reveal` → 下载或远程桌面
- 桌面 WebView DSH → iframe parent-bridge（React 已有）
- 原生 Word dylib → wasm（文档指定 React Adapter）

---

## 7. 风险

| 风险 | 程度 | 说明 |
|------|------|------|
| 性能与体重 | 高 | Flutter-web（CanvasKit / skwasm）冷启动、内存、包体通常重于 Vite SPA；仓库内无对比数，见 §9 |
| SEO / 官网 `/app` | 高 | 现网同域 SPA + OG；Flutter-web 对可抓取营销页不友好，除非落地页继续拆出去 |
| 上游分叉 | 高 | 浏览器策略与 AppFlowy-Web 永久分家 |
| 插件生态 | 高 | 大量 desktop-only 依赖；web stub 不完整 |
| 能力回退 | 高 | Explorer / PTY / Word / Diff 在浏览器上只能砍掉或远程化 |
| 并不能消灭「第二套前端」 | 中 | DSH UI、wasm 引擎、Cordis 仍是 TS。合并 Host 只去掉 React 壳，留下更重的 Flutter-web + 仍在的 JS 运行时 |
| 嵌套运行时 | 中 | Flutter-web 里再 iframe DSH = 双 VM；今日 React iframe 已在生产工作 |
| CI | 中 | 今日：npm Web CI + Flutter 桌面/移动。合并后仍要 wasm 工具链 |

---

## 8. 收益（以及收益到不了哪里）

并成一套 Flutter 的真实收益：

- Host 视觉语言（侧栏、Tab、Version Diff）理论上可三端一致
- DSH 面板交互不必改两次（现在两套约 3k 行）
- 编制上少一条「Web Host 壳」发布线

到不了的地方：

- **不会**变成真正的单栈：DSH UI 与 wasm 引擎仍是 JS
- **不会**让 rust-lib 在浏览器里跑起来
- **不会**自动获得云协作、发布、计费（这些在 React + Cloud，不在 Client 本地模式）
- 桌面 FFI 能力 **不会**因此出现在 Web；相反，为了迁 Web，往往要在 Client 里加大量 `kIsWeb` 空洞

用 50–90 人周换「壳层统一」，而把已上线的 `/app` 和上游 Web 生态押上去，ROI 为负。

---

## 9. 性能指标

仓库里 **没有** Flutter-web vs React SPA 的 FCP/TTI/包体对比。Version Diff 文档只给桌面目标（例如 20k 行 P95 &lt; 800 ms）。下列是若仍要评估任何浏览器 Host 路径时应打的点，以及定性预期。

| 指标 | 为何要看 | 定性预期 |
|------|----------|----------|
| FCP / LCP / TTI（冷启动，4G） | 官网 `/app` 转化 | Flutter-web 通常差于 code-split React |
| JS/WASM 传输 + 解析（MB） | CanvasKit + 字体 + 可能的引擎 wasm | Flutter-web 基线明显高于当前 Vite 包 |
| 稳态内存（文档打开 + DSH iframe） | 双运行时 | Flutter-web + DSH iframe ≥ 今日 React + DSH |
| 编辑器按键 P95 | 知识库主路径 | Slate/DOM vs Skia 文本；云文档路径 React 更熟 |
| Diff 对比 P95（若迁 wasm） | 桌面 FFI isolate vs worker | 桌面应继续走 FFI；浏览器另计 |
| DSH 贴附到 HybridLive 的时延 | 已有 embedding 设计关注点 | 与 Host 框架无关，与 iframe/WebView 放置有关 |

**预期：** 云文档主路径上，继续 React Host 更轻；FFI 重活（Diff、Word、PTY）留在桌面 Flutter 才是性能正确切分。

---

## 10. 结论与建议

### 10.1 决策

**不将 `Muse-Clients/frontend/web` 合并进 `Muse-Clients/frontend/client`。**

Client 重构（AppFlowy 基座 + Markdown/协作上层 + Muse Host）证明的是 **桌面/移动 Host 的分层**，不能外推为「浏览器也应跑同一份 Flutter」。浏览器缺的是数据面（Cloud + Yjs）和引擎面（wasm / 服务端 PTY），不是少一个 `web/` 目录。

这与已冻结的 D12、K6、ioffice Web 路径一致。

### 10.2 建议的「一套」定义

| 保持一份 | 允许两份 | 不要做 |
|----------|----------|--------|
| 资源协议、打开路由、AgentAttachment、Cordis 插件 | Host UI：Flutter 桌面/移动 vs React 浏览器 | Flutter-web 生产 Host |
| `middlewares/dsh` | Occupant：Flutter 原生 vs React/wasm/xterm | 把 rust-lib 塞进浏览器当「合并」 |
| 能力矩阵按端裁剪 | DSH 放置：sidecar vs 云实例 | 为了单仓牺牲 `/app` 零安装 |

这就是设计稿原句：**路由逻辑一份，承载器按端各算。**

### 10.3 编制含义

- **桌面/移动：** 继续只在 `frontend/client` 做 Workspace、Version Diff、Helix、iOffice。
- **浏览器：** 继续 `frontend/web` 做云协作与 DSH iframe；按 10–14 人周量级加 React occupant（wasm Word / viewer / 远程 Helix），而不是 50–90 人周迁 Flutter-web。
- **重复痛点（DSH 面板拖拽、主题）：** 用合同测试和共享 CSS/token 约束，而不是合并框架。

### 10.4 何时可以推翻本结论

只有在产品战略变成下面之一时才值得重开：

1. **不做浏览器 Host**（下载-only，官网不再进 `/app`）；或
2. 明确立项 **绿场 Cloud 客户端**（非 FFI 数据面），且接受与 AppFlowy-Web 分家、以及 Flutter-web 性能与 SEO 成本。

两者都不在当前路线图里（F0 仍把 Web/Mobile 引擎标为未完成；架构仍点名 AppFlowy-Web 为 Web 客户端）。

### 10.5 一句话

- 要 **一套 Muse Host 交互代码**：做不到，除非放弃浏览器或重写数据面。
- 要 **一套桌面 Agent 原生 Host**：已经是 `frontend/client`。
- 要 **一套后端/插件面**：已经是 `middlewares/dsh` + 资源合同。
- 要 **少用人、少分叉语义**：继续 hybrid——Flutter 跑本地引擎，React 跑浏览器 Host 与 wasm，合同不许分叉。

---

## 11. 证据索引

- 拓扑：`Muse-Clients/frontend/README.md`、`client/README.md`、`web/README.md`、`middlewares/scripts/lib/muse-paths.sh`
- 双栈与 K6：`client/doc/agent-file-references-and-open-routing.md` §9.1、§9.5、§9.6
- 合同按端裁剪：`11-decisions-and-open-questions.md` ADR-D12；本目录 README D12
- iOffice Web ≠ Flutter FRB：`engine-delivery/04-ioffice-implementation-plan.md` §9.2
- Word 拒 Web：`lib/plugins/word/word_page.dart`、`office_catalog.dart` `desktopOnly`
- dart-ffi 仅原生：`rust-lib/dart-ffi/Cargo.toml`
- 为何必须有 Web 可见层：`web/docs/rebrand-plan-web.md`
- Helix 不进本期 Web：`client/doc/helix-editor-integration.md`
- Web 引擎承载器 10–14 人周：`agent-file-references-and-open-routing.md` §9.6
