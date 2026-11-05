# Helix 编辑器融入 Muse Client 设计文档

状态：**设计稿（未实现）**。本文件只描述方案与落地路径，不含代码改动。

范围：把 `vendors/helix`（Helix 编辑器）接入 Muse 桌面客户端（AppFlowy 原生面板），并为后续扩展到 Web 预留通道。

---

## 1. 目标与非目标

### 1.1 已确认的决策

| 项 | 决策 | 含义 |
|---|---|---|
| 内容基底 | **阶段 1：编辑 agent workspace 的真文件** | `cwd` = DSH 工作区，保存即落盘；不碰 CRDT |
| 内容基底（后续） | 阶段 2 可扩展为 AppFlowy 文档 | 走 `ViewLayout::Code` + office blob + host-bridge 回写 |
| 体验形态 | **AppFlowy 原生面板（仅 client）** | Flutter 内渲染，不进 WKWebView；Web 端本期不交付 |
| 语言能力 | **v1 不带 LSP** | 无补全/诊断；但保留语法高亮（Helix 自带 tree-sitter） |

### 1.2 非目标（v1 明确不做）

- 不做 LSP / DAP（无自动补全、无调试器）
- 不做 Web 端编辑器（阶段 2 之后再评估，架构需预留）
- 不做 Helix-as-library 自绘渲染后端（见 §4.4）
- 不做多人协同编辑同一文件（阶段 2 若落到文档才会涉及）
- 不改 Helix 上游源码（保持可跟上游 rebase）

---

## 2. 现状勘察（结论 + 证据）

### 2.1 Helix 自身的形态约束

| 事实 | 证据 | 对方案的影响 |
|---|---|---|
| 上游版本 `25.07-984-g079a789e8`，105M，**未构建**（无 `target/`） | `vendors/helix`、`git describe` | 需自建二进制并纳入打包流水线 |
| 渲染后端只有 `crossterm` / `termina` 两种，**必须真 TTY** | `helix-tui/Cargo.toml:15` `default = ["termina","crossterm"]`；`helix-term/src/application.rs:68` `Terminal<TerminalBackend>`、`:111-119` 建 backend + `Terminal::new` | 必须起 PTY + 客户端终端模拟器 |
| `helix-tui` 有 `Backend` trait（`claim/draw/set_cursor/size/flush`…）+ `test.rs` 参考实现 | `helix-tui/src/backend/mod.rs:26`、`helix-tui/src/backend/{crossterm,termina,test}.rs` | 自绘后端技术上可行（但见 §4.4 的代价） |
| 运行时资源查找顺序含 `HELIX_RUNTIME` 环境变量与 `config_dir()/runtime` | `helix-loader/src/lib.rs:37-56`、`:120` | 打包时显式指定，不依赖用户 home |
| 配置可用 `--config <path>` 覆盖；另有 `config_dir()/config.toml`、`languages.toml` | `helix-term/src/args.rs:72`、`helix-loader/src/lib.rs:278`、`:160` | 出厂预置配置、隔离用户 `~/.config/helix` |
| `runtime/` 现状：`queries` 5.0M + `themes` 1.5M + `tutor` 52K 可直接带；**`grammars` 是空的**（0 sources、0 `.so`） | `du -sh vendors/helix/runtime/*`、`ls runtime/grammars/sources` | 语法高亮需要**按平台预编译 grammars**（需 C 编译器） |

### 2.2 仓库里现成的接缝

| 接缝 | 证据 | 复用方式 |
|---|---|---|
| DSH 已有持久 PTY 会话 seam | `@deepseek-ai/dsh-terminal`（`ctx.terminals`，按 `type` 注册 backend）；`terminal-bash` 后端内部是 `ctx.subprocess.spawnTerminal(spec)`（`packages/terminal/terminal-bash/src/index.ts:111-128`）；`dsh-tool-terminal` 是模型侧消费者 | 新增一个 `type: 'helix'` backend，不重造 PTY |
| 该 seam 明确不含渲染策略 | `packages/terminal/terminal/README.md`：*"contains no node-pty, sandbox, tool-schema, prompt, task, or terminal-rendering policy"*，且会话 *process-local* | 渲染与客户端传输由我们自己补 |
| webServer 提供 HTTP 路由 / **WS upgrade 路由** / HTML tap | `packages/host/webserver/src/index.ts:94 register()`、`:109 registerUpgrade()`、`:125 registerFallback()`、`:139 tapIndex()` | 终端字节流的双向通道 |
| Muse 插件已在使用 webServer | `middlewares/dsh/plugins/dsh-mobile-input/src/index.ts:12` `inject = ["webServer"]`；`dsh-mobile-surface`、`dsh-appflowy/src/webview.ts` 同 | 新插件沿用同一形状 |
| sidecar 生命周期已由客户端管理 | `dsh_sidecar.dart:30 ensureStarted()` | 面板开之前确保 sidecar 已起 |
| 审批与租约模式现成 | `middlewares/dsh/plugins/dsh-appflowy/src/approval.ts`、`src/mobile-lease.ts` | PTY 权限面与"谁在编辑"的归属 |
| 客户端"新视图种类"有成熟模板 | `lib/plugins/office/office_plugin.dart:22-51`（`PluginBuilder`/`PluginConfig`）、`:111-198`（`PluginWidgetBuilder` + `NavigationItem`）、`office_catalog.dart` / `office_manifest.dart` / `office_blob_store.dart` | 阶段 1 的入口与阶段 2 的视图注册都照此形状 |
| 本地文件 blob 存储有先例 | `rust-lib/flowy-core/src/deps_resolve/folder_deps/office_blob.rs`（按 `{userData}/<kind>/<view_id>.<ext>`） | 阶段 2 的落盘方式 |
| 打包/校验钩子 | `scripts/lib/muse_windows.py:31 VENDORED_PLUGIN_DIRS`、`:612 stage_vendored_dsh_plugins()`、`:978 assert_packed_runtime()`；`scripts/verify-macos-app.py` | 二进制与 runtime 纳入打包与断言，避免"静默缺件" |

> 参考：桌面端目前把 DSH Web UI 装在 WKWebView 里（`dsh_embedded_view.dart:8`，Windows 用 WebView2）。本期**不**走这条路，但它是阶段 2 之后 Web 复用的现成基础。

---

## 3. 总体架构

```
┌─ AppFlowy Flutter 客户端（原生面板，本期交付） ────────────────────────┐
│  EditorPane（Flutter Widget）                                          │
│    ├─ 终端渲染：xterm 类组件（VT 解析、光标、选区、IME）                │
│    ├─ 状态栏：路径 / 行列 / 编码 / 保存状态                              │
│    └─ PtyBackend 抽象 ──┬── SidecarWsPtyBackend（本期实现）             │
│                         └── (FFI PtyBackend，备选，见 §4.2)             │
└──────────────────────────┬────────────────────────────────────────────┘
                           │ WS localhost  /muse/editor/attach
                           │  二进制帧 = PTY 字节；JSON 控制帧 = resize/exit/error
┌──────────────────────────┴────────────────────────────────────────────┐
│ DSH sidecar（Node，客户端已负责启动）                                    │
│  @muse/dsh-editor-helix（新宿主插件；inject: terminals, webServer）      │
│    ├─ ctx.terminals 注册 backend  type: 'helix'                        │
│    ├─ webServer.registerUpgrade({ path: '/muse/editor/attach', ... })   │
│    └─ 所有权/审批：复用 dsh-appflowy 的 lease + approval 模式            │
└──────────────────────────┬────────────────────────────────────────────┘
                           │ ctx.subprocess.spawnTerminal(spec)
                           ▼
                     真 PTY:  hx <file>
                              cwd = 工作区
                              env = HELIX_RUNTIME, TERM=xterm-256color, COLORTERM=truecolor
```

组件职责边界：

- **客户端**：渲染、输入翻译、面板生命周期、文件入口。不含 PTY 逻辑（本期）。
- **DSH 插件**：PTY 会话所有权、字节转发、路径校验、审批。
- **Helix**：编辑语义、语法高亮、`:w` 落盘。

---

## 4. 关键决策与被否决方案

### 4.1 D1｜渲染在 Flutter 原生面板（已由需求确定）

代价与收益：省掉 WKWebView 的键盘/URL/兼容坑，键盘与 IME 行为可控；但 **Web 端收益为零**（Web 端需要另做一套 xterm.js 客户端，宿主侧不用改，见 §5.1）。若后续要 Web，宿主插件与协议可原样复用，只多一个前端实现。

### 4.2 D2｜PTY 归属：DSH sidecar（推荐），Dart 侧抽象隔离

| 方案 | 落点 | 优点 | 缺点 |
|---|---|---|---|
| **B1（选）** | 新宿主插件用 `ctx.terminals`；Dart 通过 localhost WS 直连 | 会话在 DSH 内 → agent 将来能共看/驱动同一会话；阶段 2 与 Web 可复用；不新增 Rust 原生依赖 | 依赖 sidecar 已启动（`ensureStarted` 已具备）；多一跳本地 WS |
| B2（备选） | `rust-lib` 加 `portable-pty`，经 `dart-ffi` 暴露 | 无 Node 依赖、启动最快 | 会话对 agent 不可见；Web 需重写；`dart-ffi` 是 `allo_isolate` + `lib_dispatch` 风格（非 flutter_rust_bridge），新增原生依赖面较大 |

**结论**：本期实现 B1；Dart 侧统一写成 `PtyBackend` 接口，B2 将来可作为第二实现插入（例如 sidecar 不可用时降级）。

### 4.3 D3｜传输不走 `@muse/host-bridge`

`@muse/host-bridge` 是 JSON 信封协议，带 `BridgeLimits.maxMessageBytes` 等上限与 `Effect: read | local_write | sync_write` 语义（`middlewares/dsh/core/protocol/host-bridge/src/contract/types.ts:31-49`、`:148`），适合"有 schema 的命令/事件"，不适合终端字节突发流（重绘、大目录 `ls`）。

**结论**：终端流走 sidecar 本地 `webServer.registerUpgrade()` 的 WS 通道；鉴权复用 `dsh_web_auth.dart` 的 device token 机制。

### 4.4 D4｜不做 Helix-as-library 自绘后端

`helix-tui` 的 `Backend` trait 很小，理论上可以自己实现一个"画到 Flutter 纹理/JSON 单元格"的后端。但 `helix-term/src/application.rs` 的 `Application` 同时还持有终端**事件读取**（termina/crossterm 的键鼠/resize）、compositor、jobs、LSP/DAP；自绘等于自己实现"终端单元格渲染 + 合成输入事件"这一整层——**就是 xterm 那层**，还得跟上游分叉。

**结论**：v1 不采用。保留一个窄用法：若将来只需要"Helix 的模态编辑手感"而不要终端观感，可只取 `helix-core`（rope / textobject / motions）或 `languages.toml` + tree-sitter 做高亮，走 ioffice 既有 FFI 管线（`vendors/ioffice/word/word_render` 的 `native/macos/arm64/libword_core_ffi.dylib` 模式），成本约 1–2 人周。

### 4.5 D5｜v1 无 LSP，但必须裁剪 `languages.toml`

未安装语言服务器时，Helix 仍会按 `languages.toml` 尝试拉起 LSP，产生错误/诊断噪音。**结论**：出厂发一份裁剪过的 `languages.toml`（去掉/禁用 LSP 字段，保留 grammar + 高亮查询）。

---

## 5. 接口设计

### 5.1 客户端 ⇄ 宿主 attach 协议（草案）

传输：`webServer.registerUpgrade({ path: '/muse/editor/attach', handler })`，本地回环，二进制帧承载 PTY 字节。

握手（JSON 文本帧，仅首帧）：

```jsonc
// client → host
{ "op": "attach", "sessionId": "<opaque>", "cols": 120, "rows": 32, "token": "<device-token>" }
// host → client
{ "op": "attached", "cols": 120, "rows": 32, "shell": "hx", "file": "src/main.rs", "writable": true }
{ "op": "error", "code": "OWNERSHIP_CONFLICT" | "PATH_OUTSIDE_WORKSPACE" | "SPAWN_FAILED", "message": "..." }
```

运行期：

- 二进制帧：PTY 原始字节，双向。
- JSON 文本帧（低频控制）：
  - `{ "op": "resize", "cols": N, "rows": N }` → 宿主 `TIOCSWINSZ` + 向 `hx` 投递 `SIGWINCH`
  - `{ "op": "signal", "name": "SIGINT" }`
  - `{ "op": "exit", "code": 0 }`（宿主 → 客户端，会话结束）
- 背压：宿主对同一 session 的写操作串行化（对齐 `dsh-terminal` 的"一次一个 send"约束）；客户端渲染队列满时丢弃"仅重绘"帧的策略需在 spike 中实测后定。

所有权：同一 `sessionId` 只允许一个 attach；第二个 attach 返回 `OWNERSHIP_CONFLICT`（复用 `mobile-lease` 的租约思路）。会话在宿主重启后不恢复（与 `dsh-terminal` 的既有语义一致）。

### 5.2 新宿主插件 `@muse/dsh-editor-helix`

落点：`middlewares/dsh/plugins/dsh-editor-helix/`（形状照 `dsh-mobile-input` / `dsh-terminal-bash`）。

```
inject: ["terminals", "webServer"]
apply(ctx):
  ├─ ctx.terminals 注册 backend   type: 'helix'
  │    spawn: 校验 path 在工作区内 → spawnTerminal({
  │             argv: [<muse>/bin/hx, "--config", <userData>/helix/config.toml, <file>],
  │             cwd: <workspace>, env: { HELIX_RUNTIME, TERM, COLORTERM } })
  ├─ webServer.registerUpgrade({ path: '/muse/editor/attach', handler })
  └─ 审批：首开面板或首次写入前走 approval 面（对齐 dsh-appflowy/src/approval.ts）
```

待定（spike 决定）：会话 id 用 `ctx.terminals` 的 opaque id 还是自建；是否需要在 `dsh-appflowy` 的 connector 里暴露"打开编辑器"的 host 能力。

### 5.3 Dart 侧抽象

```dart
abstract class PtyBackend {
  Future<PtySession> open({required String file, required String cwd, required int cols, required int rows});
}

abstract class PtySession {
  Stream<List<int>> get output;          // PTY → 渲染
  Future<void> write(List<int> bytes);   // 输入 → PTY
  Future<void> resize(int cols, int rows);
  Future<void> signal(String name);
  Stream<int> get exit;
  Future<void> close();
}
```

本期实现 `SidecarWsPtyBackend`；`EditorPaneController`（Flutter 侧）持有 backend + 渲染组件 + 状态栏状态，便于将来替换渲染实现（见 §9 兜底）。

### 5.4 Flutter 面板接入点

- **阶段 1**：以工作区文件的入口打开面板（从工作区树 / "在编辑器中打开"菜单），面板作为独立 pane/tab。实现形状参考 `office_plugin.dart:22-51` 的 `PluginBuilder` 与 `:111-198` 的 `PluginWidgetBuilder`（`leftBarItem` / `tabBarItem` / `buildWidget`），但**不必**先引入新的 `ViewLayoutPB`。
- **阶段 2**：新增 `ViewLayoutPB.Code`，按 `OfficePluginRegistry.byLayout()` 的方式注册新的 manifest + `pageBuilder`，内容落到 blob（`office_blob_store.dart` 模式），保存经 host-bridge 回写。

---

## 6. 打包与运行时

### 6.1 二进制与资源落点

```
<muse>/                      # .app/Contents/Resources/muse 或 Windows 的 {exeDir}/muse
├── bin/hx                   # Helix 可执行（Windows: bin/hx.exe）
├── helix-runtime/           # = vendors/helix/runtime 的 queries+themes（+预编译 grammars）
└── (既有) closure/ node/ plugin-tools/ patch.yml
```

- `HELIX_RUNTIME` 显式指向 `<muse>/helix-runtime`（不依赖 exe 相对查找，也不读用户 `~/.config/helix`）。
- `--config <userData>/helix/config.toml`：出厂预置（主题/键位/关闭不需要的特性）；用户可改，但不写用户的 `~/.config`。
- 打包钩子：在 `muse_windows.py` 增加 stage 步骤（与 `stage_vendored_dsh_plugins` 同层），并在 `assert_packed_runtime()`（`:978`）把 `bin/hx` 与 `helix-runtime/queries` 列为**必备项**——即上次 `dsh-model-capabilities` 漏包的教训：缺件必须在打包/验证阶段 fail fast，而不是静默降级。

### 6.2 平台矩阵

| 平台 | v1 | 备注 |
|---|---|---|
| macOS arm64 | ✅ | 本期目标；`hx` 与 grammars 一起进签名/公证流程 |
| macOS x86_64 | 可选 | 同源构建即可 |
| Windows x86_64 | 未验证 | 打包器共用 `muse_windows.py`，需单独构建 `hx.exe` 并冒烟 |
| Linux（Web/云侧） | 阶段 2 | 需要进 `middlewares/scripts/build-dsh-image.sh` 的镜像 |

### 6.3 体积预算与取舍

| 项 | 估算 | 策略 |
|---|---|---|
| `hx` 二进制 | 约 30–60MB | 必须带 |
| runtime queries+themes | 6.5MB | 必须带 |
| 预编译 grammars | 约 30–80MB／平台 | v1 可只带常用语言子集；或首次启动按需下载（需签名与缓存校验设计） |

当前 macOS zip 已 291.5MB；上述全量约 +70–140MB。**建议**：v1 带 10–15 个常用语言 grammar，其余按需下载。

### 6.4 签名与公证

- `hx` 与 grammars（`.dylib`/`.so`）必须纳入 `.app` 的深度签名；Helix 自身在运行时 dlopen grammar 动态库，签名不完整的 grammar 会在加固运行时下加载失败。
- 沿用 `pack-macos-client.py` 的嵌套优先签名顺序（`build-macos-appflowy.sh` / `pack-macos-client.py` 现有实现），并在 `verify-macos-app.py` 增加"hx 可执行 + grammar 可加载"的断言。

---

## 7. 安全与权限

- **PTY = 任意命令执行面**。最小约束：`cwd` 固定工作区；文件路径必须解析后仍位于工作区内（拒绝 `..`、符号链接逃逸）；面板默认只读打开、首次写入需显式确认（复用 `approval.ts` 的审批面）。
- 面板开关做成设置项（默认关闭），并在首次打开时说明"该面板在本地执行命令"。
- attach 必须校验 device token，且只监听回环地址。

---

## 8. 分期计划与验收标准

### P0｜Spike（2–3 天，四个可并行验证）

| # | 验证 | 通过判据（go/no-go） |
|---|---|---|
| 1 | `hx` 构建与资源可用性 | arm64 release 构建成功；`HELIX_RUNTIME` 指向 bundle 内 runtime 后能启动并正确高亮；给出体积与冷启动耗时 |
| 2 | PTY backend + WS attach | 能在 `ctx.terminals` 起 `type:'helix'` 会话，WS 双向字节通，`resize` 生效（`hx` 重排布局），退出事件可达 |
| 3 | 渲染与输入（**最高风险**） | 最小 Flutter 页接 WS 后可交互；**中文 IME 可用**、Esc/Tab/方向键/Ctrl 组合正确、宽字符对齐、复制粘贴可用 |
| 4 | 打包断言 | `hx`+runtime 进入 `.app` 且被 `assert_packed_runtime()` 校验；`verify-macos-app.py` 通过 |

任一项不达标则回到 §4 重新评估（第 3 项不达标的兜底见 §9）。

### P1｜v1（3–4 周）

- 面板：标签/分栏最小集、状态栏、保存（`Cmd+S` → `:w`）、打开文件入口
- 会话生命周期：`ensureStarted`、切工作区/关窗时的归属与清理、崩溃重连
- 边界：路径必须落在工作区内；只读/可写两种模式
- 运行时：`HELIX_RUNTIME` + `--config` 预置、裁剪 `languages.toml`
- 流水线：纳入 pack / verify / 签名；Windows 打包至少不破坏
- 验收：能打开工作区文件 → 编辑 → 保存 → 磁盘内容正确；无 LSP 噪音；IME 与快捷键清单全通过；`.app` 体积增量在预算内

### P2｜扩展（评估后再排）

- AppFlowy 文档基底（`ViewLayout::Code` + blob + host-bridge 回写）
- Web 端（同一宿主插件 + 同一 WS 协议，新增 xterm.js 前端实现）

---

## 9. 风险登记

| 风险 | 影响 | 缓解 / 兜底 |
|---|---|---|
| Flutter 终端组件的中文 IME 不达标 | 高（中文用户不可用） | P0 第 3 项先验；兜底：**只把这一个面板**换成内嵌 WebView + xterm.js，架构不变（`PtyBackend` 接口与宿主插件原样复用） |
| AppFlowy 全局快捷键（Cmd+S/W/P）抢焦点 | 中 | 面板 Focus 拦截 + 快捷键作用域；Esc 的单/双击语义按 xterm 规则处理 |
| grammars 未签名/加载失败 | 高（无高亮） | 纳入深度签名；`verify-macos-app.py` 增加加载断言 |
| 包体积增加 70–140MB | 中 | 语言子集 + 按需下载 |
| 无 LSP 导致"编辑器很弱"的用户预期落差 | 中 | 明确 v1 定位（编辑+高亮）；`languages.toml` 裁剪避免噪音 |
| sidecar 未启动/崩溃导致面板不可用 | 中 | `ensureStarted()` + 面板内错误态 + 后续 B2 实现作为降级 |
| PTY 逃逸工作区 | 高 | 路径解析校验 + 审批 + 默认只读 |
| Web 端将来要重写 | 低 | 宿主插件与协议已为 Web 预留（§5.1） |

---

## 10. 测试计划

- **单元**：路径校验（`..`/符号链接/大小写）、控制帧解析、attach 所有权冲突。
- **集成**：起 sidecar → attach → 写入字节 → 断言 PTY 回显 → resize → 退出码透传。
- **打包断言**：`assert_packed_runtime()` 覆盖 `bin/hx` 与 `helix-runtime`；`verify-macos-app.py` 冒烟。
- **手工清单**：中文 IME（拼音候选窗位置）、CJK 宽字符对齐、Esc/Tab/方向键/Ctrl+c、鼠标拖动选区、复制粘贴（含 xterm 选区）、窗口 resize、`Cmd+S`、打开超长行大文件、只读模式。

---

## 11. 未决问题

1. **pub 包选型与版本未核实**：终端渲染组件（xterm 类）与 sidecar WS 客户端的候选包、版本、维护状态需在 P0 现场核实并 pin（本次编写环境无法联网检索）。
2. 会话 id 与生命周期：复用 `ctx.terminals` 的 opaque id，还是 Muse 自建 session 表（若要支持"agent 与用户共看同一会话"，倾向前者）。
3. grammars 交付方式：全量随包 vs 语言子集随包 + 按需下载（涉及签名与缓存校验）。
4. 阶段 2 的 `ViewLayout::Code` 是否会与 collab 上游冲突（新增 layout 需同步 `scripts/tool/patch_collab_word_layout.sh`，并清理 release target 的陈旧产物——git 依赖按 rev 指纹，不感知脏文件）。
5. Windows 端是否同期交付（打包器共用，但需单独构建与冒烟）。

---

## 12. 预计改动清单（实现阶段，供评审）

**新增**

- `middlewares/dsh/plugins/dsh-editor-helix/`（宿主插件：terminals backend + WS upgrade + 审批）
- `frontend/client/frontend/appflowy_flutter/lib/plugins/editor/`（面板、`PtyBackend`、状态栏）
- 构建产物：`<muse>/bin/hx`、`<muse>/helix-runtime/`

**修改**

- `frontend/client/scripts/lib/muse_windows.py`（stage + `assert_packed_runtime`）
- `frontend/client/scripts/verify-macos-app.py`（断言）
- `middlewares/dsh/plugins/dsh-appflowy/cordis.patch.yml`（挂载新插件行）
- `frontend/client/frontend/appflowy_flutter/pubspec.yaml`（终端组件与 WS 依赖）
- 打包入口（`pack-macos-client.py` 复用既有 staging 路径）

**不改**

- `vendors/helix` 上游源码（保持可 rebase）

---

## 13. 附录：PTY 是什么，以及 Helix 为什么需要它

> 背景知识，供不熟悉终端子系统的评审者阅读。结论已体现在 §3 总体架构与 §4 关键决策（尤其 D2、D4）中。
>
> 本附录引用的 §9.8、§9.9 均指同目录下的配套文档 [`agent-file-references-and-open-routing.md`](./agent-file-references-and-open-routing.md)（本文自身只有 1–13 节）。

### 13.1 PTY（伪终端）是什么

终端不是屏幕，而是内核里的一个抽象。早期终端是硬件（如 VT100 经串口连主机），内核的 TTY 子系统夹在"程序"与"设备"之间替程序做这些事：

- **termios 属性**：canonical（行缓冲、自动回显）还是 **raw**（逐键直达、不回显）
- **信号生成**：`Ctrl+C`→SIGINT、`Ctrl+Z`→SIGTSTP、`Ctrl+\`→SIGQUIT、`Ctrl+S/Q`→流控
- **窗口尺寸**：`TIOCGWINSZ` 读、`TIOCSWINSZ` 写；尺寸变化时内核向前台进程组发 **SIGWINCH**

PTY 就是把这个"设备那一端"用软件伪造出来，它成对存在：

| 一端 | 谁持有 | 职责 |
|---|---|---|
| **slave**（`/dev/pts/N`） | 被启动的程序（shell、vim、`hx`） | 程序把它当 stdin/stdout/stderr 与控制终端；用 `isatty()` 判断、ioctl 问尺寸、termios 切 raw |
| **master**（`/dev/ptmx` 分配） | 终端模拟器（iTerm/xterm/tmux；本方案里是 sidecar 的 `node-pty`） | 从 master 读程序吐出的字节流（VT/ANSI 转义序列），把按键编码成字节写回，并在尺寸变化时设置 slave 的 winsize |

内核在 slave 侧挂着 **line discipline（线路规程）**，上述属性、信号与尺寸都归它管。`openpty()` / `forkpty()` 即"分配 PTY + fork 子进程 + 把 slave 设为子进程控制终端"的一体化封装。

一句话：**PTY 让一个程序能假装自己在与真终端对话，而由另一个程序扮演终端。**

### 13.2 Helix 与 PTY 的关系

Helix 没有 GUI 层，它的原理就是**向终端输出转义序列 + 从终端读取事件**：

| 环节 | 证据 | 为什么必须面对真终端/PTY |
|---|---|---|
| 进入终端模式 | `helix-tui/src/backend/crossterm.rs:162,165,178`（`enable_raw_mode` / `EnterAlternateScreen` / `EnableMouseCapture`），退出时 `:223-225` 复原 | 全是 termios/ioctl/转义序列；非终端下返回 `io::Error`，Helix 用 `?` 上抛——所以 `hx > out.txt` 不成立 |
| 输出即画面 | `helix-term/src/application.rs:113` `CrosstermBackend::new(std::io::stdout(), …)`；`crossterm.rs:228` `draw()` | 画的是字节流，不是像素也不是控件树 |
| 尺寸来自内核 | `crossterm.rs:323-325` `size()` → `terminal::size()`（ioctl TIOCGWINSZ） | winsize 由 master 侧设置；不同步则按旧尺寸重排 |
| 输入来自终端 | `application.rs:1286-1298` `crossterm::event::EventStream` / `termina::EventStream` | 事件源就是那个 tty |
| 交互与否看 isatty | `application.rs:227` `stdin().is_terminal()` 分流（文件参数 / 空 buffer / 从 stdin 读文档） | Helix 自己就用 tty 判定语义 |

因此"把 Helix 嵌进 App"必然是三件套：

```
App 内终端组件（终端模拟器：xterm.js / 终端类 Flutter 组件）
   ▲ 按 VT 序列绘出单元格            │ 按键 → 编码字节
   └─────── PTY master（node-pty，在 DSH sidecar 内）───────┐
                │ 内核 line discipline：raw 模式 / 信号 / winsize
                ▼
        PTY slave = hx 的 stdin/stdout/stderr + 控制终端
                │
            hx：吐 ANSI 序列 / 读事件 / ioctl 问尺寸
```

- 没有 PTY → 连 `enable_raw_mode` 都过不去，`hx` 起不来；
- 没有终端模拟器 → 字节流无人解释成画面；
- 不同步 resize（`TIOCSWINSZ`）→ Helix 不知该重排。

这也是 **D4（不做 Helix-as-library 自绘后端）** 的根据：`helix-tui` 的 `Backend` trait（`helix-tui/src/backend/mod.rs:26`：`claim`/`draw`/`set_cursor`/`size`/`flush`）只覆盖输出那一半，`Application` 仍要从终端读输入事件——自绘等于自己重写一个终端模拟器。

### 13.3 三端的 PTY 落点（与打开路由方案 §9.8 的宿主放置对齐）

| 端 / 放置 | PTY 在哪 | 可行性 | 说明 |
|---|---|---|---|
| Desktop · `local-embedded` | 本机 sidecar（Node + `node-pty`） | ✅ | 我们自己的进程，可 `openpty` + spawn `hx` |
| Web · `remote-cloud-*` | 云端 DSH 实例（Linux） | ✅ | 浏览器无本机进程权限；页面只跑 xterm.js 解析字节。**"本地打开"语义变为"服务端打开"** |
| Web / Mobile · `remote-desktop` | **用户自己的桌面** | ✅ 体验最优 | PTY 在用户本机 → 编辑的就是他自己的本地文件，同时绕开 iOS 的进程限制 |
| Mobile · `remote-cloud-*` | 云端实例 | ⚠️ 仅远程 | iOS 沙箱不允许 fork/exec 任意二进制、也不允许自建 PTY → **本机 PTY 不可能** |
| Mobile · 本地 | —— | ❌ | Android 理论可行（NDK 交叉编译 + `posix_openpt`），但 APK 体积/ABI/许可收益太低，v1 不做 |

**PTY 的创建权归属**：`@deepseek-ai/dsh-terminal` 只声明持久 PTY 会话 seam，真正创建 PTY 的是它的后端实现（`terminal-bash` 经 `ctx.subprocess.spawnTerminal`，其 `sanitize.ts:34` 注释即"Consume one decoded `node-pty` data chunk"）。本方案是**往该 seam 注册一个 `type:'helix'` 的后端**（D2），而不是自建 PTY 层。

> 远程与本地 PTY 的**体验差异**（延迟预算、文件归属、会话生命周期、剪贴板/换行）见打开路由方案 §9.9。
