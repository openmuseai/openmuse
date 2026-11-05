# Windows 打开代码文件「秒开」失败分析

> 分析对象：`HelixResourceSurface`（`hx.exe` + ConPTY）与 `OpenFileViewerResourceSurface`（WebView2）
> 基准文档：`WINDOWS-HELIX-WARM-POOL-DESIGN.md`（热进程池设计）
> 结论一句话：**热进程池当前是负优化。**同一台机器、同一份配置、判定标准为「文件内容真的画到屏幕上」：
> 冷启动 `hx <file>` **0.69 s**，而预热池接管 `:open "<绝对路径>"` **2.60 s**（慢 3.8 倍）、`:open "<相对路径>"` **1.58 s**（慢 2.3 倍）。
> 根因是 Helix 的命令行提示符对**每一个按键**都重算一次路径补全（扫文件系统），因此 `:open` 的耗时 ∝ 命令字符数 × 途经目录的规模；而 Helix 的 0.36 s 进程创建又和自己 ~0.3 s 的初始化重叠，所以「敲一串绝对路径」必然比「重新起一个进程」更慢。

---

## 目录

- [1. 结论速览](#1-结论速览)
- [2. 打开一个代码文件的完整流程（代码级）](#2-打开一个代码文件的完整流程代码级)
- [3. 实测数据与测量方法](#3-实测数据与测量方法)
- [4. 根因分析（按影响排序）](#4-根因分析按影响排序)
- [5. 修复建议（含预期收益与代价）](#5-修复建议含预期收益与代价)
- [6. 验收标准与复现命令](#6-验收标准与复现命令)
- [7. 附录](#7-附录)

---

## 1. 结论速览

### 1.1 端到端实测（中位数，4 次；判定 = 文件内容出现在屏幕，非文件名）

| 场景 | 命令 | 点击 → 内容可见 | 相对冷启动 |
| --- | --- | --- | --- |
| **B1 冷启动（真实代码文件）** | `hx --config … --working-dir <项目根> <绝对路径>` | **0.69 s** | 基准 |
| **B2 预热池（当前实现）** | `:open "<绝对路径>"`（125 字符） | **2.60 s** | **慢 3.8×** |
| B3 预热池（改用相对路径） | `:open "<相对路径>"`（65 字符） | 1.58 s | 慢 2.3× |
| A1 冷启动（小目录小文件） | `hx marker.md` | 0.42 s | — |
| A2 预热池（最佳形态：裸文件名 17 字符） | `:open "marker.md"` | **0.29 s** | 快 1.5× |
| A3 预热池（小目录但绝对路径 46 字符） | `:open "<绝对路径>"` | **5.97 s** | 慢 14× |

设计文档预期第 2 个及以后的文件 ~0.15–0.35 s（第 291 行），实测 **1.6–2.6 s**，差 5–10 倍。

### 1.2 三条必须知道的结论

1. **`:open` 不是「几乎免费」**，它是最贵的一步：`open` 提示符每敲 1 个字符就要重算一次补全，补全会 `read_dir` 当前路径前缀所在目录并做模糊匹配。实测本仓库真实路径 **约 17 ms/字符**（152 字符命令 ≈ 1.4–1.9 s 纯打字），而在小目录里裸文件名只要 ~2.5 ms/字符。
2. **判定「文件已打开」的方式不可靠**：`_terminalShowsOpenFile()` 只看「最后三行是否包含文件名」，而 Helix 的**命令行回显**和**路径补全弹窗**都包含文件名 → 提前揭开遮罩；反过来状态栏在 ≤80 列时会把文件名截断成 `helix_resource_surface.`，`contains(basename)` 失败 → 1000 ms 超时 → `_reboot()` 冷重启（再花 ~1.5 s+）。
3. **`Pty.start` 是同步 FFI，跑在 UI isolate 上**，本机实测 **0.36–0.5 s**，冷路径在第 177 行、热路径补池子在第 257 行都调用它；这期间 Dart 事件循环完全停摆，`_watchWarmOpen` 的定时器和终端解析都被冻结。

---

## 2. 打开一个代码文件的完整流程（代码级）

### 2.1 入口与引擎路由

| 步骤 | 位置 | 说明 |
| --- | --- | --- |
| 资源树点击 | `lib/workspace_platform/presentation/workspace_explorer.dart:242` | `MuseResourceSurfaceOpener.open(context, MuseResourceOpenRequest(path: entry.locator, origin: hostPicker))` |
| 路径规范化与引擎选择 | `lib/plugins/resource_surface/resource_open_request.dart:123-156` | `resolveSymbolicLinks()` + `stat()`；`.docx` → ioffice，`viewerExtensions` → openFileViewer，`helixExtensions` → helix，其它 → openFileViewer |
| 插件 id | `lib/plugins/resource_surface/resource_file_plugin.dart:66` | `'resource:<绝对路径>'`（每个文件一个 tab/插件实例） |
| 引擎 → Widget | `resource_file_plugin.dart:125-140` | `MuseLocalEngine.helix → HelixResourceSurface`，`openFileViewer → OpenFileViewerResourceSurface` |

### 2.2 Helix 冷启动路径（当前实际路径的一部分）

`lib/plugins/resource_surface/surfaces/helix_resource_surface.dart` `_boot()`：

| 行 | 动作 | 实测成本 |
| --- | --- | --- |
| 80 | `_settingsController.ensureLoaded()` | KV 读取，一次 |
| 84-88 | `HelixInstall.resolve()` / `installer.ensureRoot()` / `grammarRuntimes()` | 设计文档：5 ms / 启动一次 |
| 89-92 | `copyHelixGrammars()`（已存在时早退） | 0.53 ms（`existsSync`+`listSync` 23 个文件） |
| 93-118 | `helixHasGrammar()` + 语法状态 setState | <1 ms |
| 120-122 | `Directory.systemTemp.createTemp('muse-helix-config-')` | 均摊 3.2 ms（含两次写盘） |
| 123-131 | 写 `runtime/themes/openmuse_host.toml`（89 B） | ↑ |
| 132-138 | 写 `<tmp>/helix-host.toml`（1158 B） | ↑ |
| 139 | `installer.writeLanguagesToml()`（签名缓存命中） | 0.23 ms |
| 140 | `helixProjectRoot(widget.file)`：≤16 层 × 7 个 marker 的**同步** `existsSync` | 3.17 ms |
| 149-155 | `helixGrammarProcessEnvironment()` + `pathPrefix()`（3 次 `existsSync`） | 0.82 ms |
| 161-169 | `buildHelixWarmSpec()`：再写一次 overlay、`existsSync` config/scratch、`writeLanguagesToml()`（**第三次**）、复制 `Platform.environment` | 2.63 ms |
| 171-176 | `HelixWarmPool.instance.take(warmSpec)` | 命中 ≈ 0；未命中即返回 null |
| **177-184** | **`Pty.start(...)`（同步 FFI：`CreatePseudoConsole` + `CreateProcessW`）** | **0.36–0.5 s，UI isolate 冻结** |
| 186-195 | `listen(_onPtyData)`、`_revealRequested = true`、`_armPaintTimeout()`、postFrame `_syncPtySize()` | — |
| 194 | `_enterInsertMode(pty)`：280 ms 后写 `i` | — |
| **196** | **`HelixWarmPool.instance.schedule(warmSpec)`** | **又 0.36 s 冻结**（`_spawn()` 内同步 `Pty.start`） |

Dart 侧准备工作的合计实测：**9.43 ms（中位数，max 10.26 ms）**——不是瓶颈（`openmuse/tmp/helix_open_cost.dart`）。

真正的耗时集中在 `Pty.start` 与 Helix 自己的首帧：

| 组成 | 实测 |
| --- | --- |
| `Pty.start`（ConPTY + 进程创建） | 0.36 s（设计文档 C 冒烟：336 ms） |
| ConPTY 首批字节（终端初始化序列） | 再 ~0.10 s |
| Helix 内部启动（自身 debug 日志首尾差，小文件） | ~0.30 s，**与进程创建重叠** |
| 文件内容上屏（小目录/项目目录，含 app 配置） | 0.42 s / 0.69 s |

`_onPtyData`（202-213）在**第一个字节**（ConPTY 自己的初始化序列）就把遮罩揭开（203-211），此时 Helix 还没画任何东西——用户先看到 ~0.3–0.4 s 空白终端，再看到内容。

### 2.3 预热池接管路径

`helix_warm_pool.dart`：

| 行 | 动作 |
| --- | --- |
| 47-115 | `buildHelixWarmSpec()`：`warmRoot.create` → 写 overlay → `Object.hash(...)` 生成 digest → `helix-host-<digest>.toml` → `writeLanguagesToml()` → `scratch.md` → 复制环境 → `HelixWarmSpec(key: '<digest>|<binary>|<workingDirectory>', args: [--config, --working-dir, scratch.md])` |
| 247-261 | `take()`：`_idle.remove(key)` → `waitUntilReady(1200 ms)`（**只等第一个字节**，不是首帧）→ `releaseToSurface()` |
| 276-304 | `_spawn()`：函数体第一行就是 `Pty.start(...)`（279），**在任何 `await` 之前**，因此在调用者 isolate 上同步阻塞 |
| 264-274 | `schedule()`：同 key 去重；`_idle.length >= 2` 时淘汰 `_idle.keys.first`（因为 `take()` 会移除、补池会重新插入，这个顺序大致等价于「最近一次被领取/启动」的时间）；`unawaited(_spawn(spec))` |
| 321-341 | `warmUpForWorkspace()`：用**挂载点根目录**当 `workingDirectory` 建 spec |
| 352-360 | `helixOpenCommand()`：`:open "<绝对路径>[:行]"` + `\r`，字节流直接 `pty.write()` |

`helix_resource_surface.dart` 接管：

| 行 | 动作 |
| --- | --- |
| 232-237 | `takeBufferedOutput()` 回放（遮罩仍盖着） |
| 243-252 | 换 `_pty`、`listen`、`setState`、`_revealRequested = false`（**不靠 PTY 输出揭开遮罩**）、postFrame `_syncPtySize()` |
| **253-255** | `pty.write(helixOpenCommandBytes(path))` ← 152 字节的 `:open "…"` |
| **257** | `schedule(spec)` ← 补池子，同步阻塞 ~0.36 s |
| 258 | `_enterInsertMode()`：280 ms 后写 `i` |
| 265-289 | `_watchWarmOpen()`：每 **60 ms** 检查一次，**1000 ms** 超时；超时 → `_reboot()` |
| 293-302 | `_terminalShowsOpenFile()`：`_terminal.buffer.lines` 的**最后 3 行**里找 `basename` |
| 324-335 | `_reboot()`：`_flushToOriginal()` → `kill` → `_boot()`（完整冷路径，含全部准备 + 0.36 s spawn） |

`_watchWarmOpen` 的计时语义：`elapsed += 60 ms` 按**定时器 tick** 累加，UI isolate 被 `Pty.start` 冻结时 tick 顺延，所以「1000 ms 超时」在墙钟上 ≥ 1000 ms + 冻结时长。

### 2.4 open-file-viewer（WebView2）路径

`lib/plugins/resource_surface/surfaces/open_file_viewer_resource_surface.dart`：

| 行 | 动作 | 问题 |
| --- | --- | --- |
| 101-104 | **每个 tab 新建 `WebviewController()` + `initialize()`** | 每次都要建 WebView2 环境/控制器（微软建议全应用共享一个 `CoreWebView2Environment`），冷启动数百 ms～1 s+ |
| 109-117 | `addVirtualHostNameMapping('openmuse.viewer', <assets/engines/open-file-viewer>)` | 便宜 |
| 119-133 | 监听 `loadingState` → `loadUrl('https://openmuse.viewer/index.html')` | **整页加载**（pdf.js 等） |
| 152-159 | `_payload()`：`base64Encode(await file.readAsBytes())` | 内存放大 4/3 并复制 |
| 176-189 | `postWebMessage(payload)` 把 base64 推给 JS | 跨进程字符串编组，大文件昂贵（限 64 MiB） |
| 144-146 | `_loaded = true` 只在页面 `viewer.ready` 之后 | 遮罩一直盖到 JS 渲染完成 |
| 192-201 | `dispose()` 释放 controller | tab 切换/重建即整条链路重来 |

没有为查看器做任何池化；每个文件 = 一次 WebView2 初始化 + 一次整页加载 + 一次全量 base64 传输。

---

## 3. 实测数据与测量方法

### 3.1 方法（两处关键修正，否则数据会错）

1. **必须回应 ConPTY 的终端查询。** ConPTY 在子进程输出前会先发 `ESC[1t ESC[c ESC[?1004h ESC[?9001h` 并等客户端应答；不应答时它会**压住子进程输出约 3 s**。不修这一点，`hx --version` 会被测成 3.3 s、`hx <file>` 首帧也会被测成 3.3 s，从而误判「Helix 启动慢」。修正后同一命令是 0.36–0.51 s（`hx_open_latency_probe.py` 的 `_answer_queries()`）。
2. **判定「打开完成」必须用文件内容，不能用文件名。** 状态栏、命令行回显、补全弹窗三者都含文件名。用文件名做判据会把补全弹窗命中的 16 ms 当成「打开完成」。

### 3.2 Helix 自身启动构成（`hx_paint_baseline.py`）

| 场景 | spawn | 首批字节 | 目标上状态栏 |
| --- | --- | --- | --- |
| 小目录、无 `--config` | 368 ms | 52 ms | **365 ms** |
| 项目目录、无 `--config` | 364 ms | 54 ms | 524 ms |
| 小目录 + app 配置 | 364 ms | 54 ms | 371 ms |
| 项目目录 + app 配置 | 362 ms | 54 ms | **508 ms** |
| `hx --version` | 365 ms | 54 ms | 54 ms（无 TUI） |

→ 设计文档的「Helix 自身启动 ≈ 0.38 s」实际上约等于**进程创建**成本；文件真正上屏还要再 ~0.1–0.15 s。工作目录大小只影响 ~0.15 s。

### 3.3 `:open` 的打字成本（`hx_completion_probe.py`，各 120 字符）

| 输入 | 回显完成 | 每字符 |
| --- | --- | --- |
| `:theme <120 字符>`（补全候选在内存里） | 83 ms | 0.7 ms |
| `:echo <120 字符>`（无补全） | 84 ms | 0.7 ms |
| 插入模式 120 字符（无提示符） | 85 ms | 0.7 ms |
| **`:open "<120 字符>`（补全扫文件系统）** | **998 ms** | **7.9 ms** |

这是**因果对照**：同一个客户端、同一个终端尺寸、同样的输出通道，只要参数是路径，成本就高 12 倍。对真实项目路径（途经 `D:\`、仓库根、`client\`、`frontend\`、`appflowy_flutter\`、`lib\`…）斜率约 **17 ms/字符**（由 B2/B3 两点回归：`(2600-1584)/(125-65)`），截距 ≈ 0.49 s（文件加载 + 首帧绘制）。

补充：把整条命令用**括号粘贴**（`ESC[200~…ESC[201~`）发送没有帮助（164 B 变体仍是 1.9 s）——ConPTY 不把它变成 `Event::Paste`，Helix 仍然逐字符处理（`prompt.rs:608` 的 `Event::Paste` 分支才是「一次插入 + 一次补全」）。

### 3.4 Dart 侧每次打开的成本（`helix_open_cost.dart`，20 次迭代）

| 项目 | 中位数 | max |
| --- | --- | --- |
| `helixProjectRoot()`（同步 marker 上溯） | 3.17 ms | 4.13 ms |
| 语法库计数（`existsSync` + `listSync`） | 0.53 ms | 0.70 ms |
| `Map.from(Platform.environment)` + `pathPrefix` | 0.82 ms | 1.05 ms |
| `writeLanguagesToml()` 缓存命中路径 | 0.23 ms | 0.29 ms |
| `createTemp` + 写 overlay + 写 config | 3.22 ms | 3.88 ms |
| `buildHelixWarmSpec` 的 IO（不含 temp 目录） | 2.63 ms | 5.83 ms |
| **每次打开合计** | **9.43 ms** | 10.26 ms |

设计文档「去掉 343 ms 的 `writeLanguagesToml`」的修复是有效的，Dart 侧已经不是瓶颈。

### 3.5 `Object.hash` 跨进程不稳定

`buildHelixWarmSpec()` 用 `Object.hash(configText, overlayThemeToml, revision, runtime) & 0x7fffffff` 当配置文件名（`helix_warm_pool.dart:69-77`）。同一输入在 4 个独立 Dart 进程里得到 4 个不同 digest（`1c00e806` / `c6c87e6` / `14ccf41f` / `a7f5c2`）——Dart 的 `Object.hash` 有每进程随机种子。

后果：`%APPDATA%\openmuseai\OpenMuse\helix-warm\` 里累积 **37 个 `helix-host-*.toml`**（每次启动新增一个，内容完全相同），每次启动多写一个 1158 B 文件，且文件名无法跨启动复用。

### 3.6 状态栏在窄终端下截断文件名

| 列数 | 状态栏 h-2 | 含 basename？ |
| --- | --- | --- |
| 120 / 100 | `NOR   lib\plugins\resource_surface\surfaces\helix_resource_surface.dart   1 sel  1:1` | ✅ |
| **80** | `NOR   lib\plugins\resource_surface\surfaces\helix_resource_surface.` | ❌ |
| 60 | `NOR   lib\plugins\resource_surface\surfaces\hel` | ❌ |
| 40 | `NOR   lib\plugins\resource_` | ❌ |

`_syncPtySize()`（343-347）会把 PTY 调整到实际控件尺寸，因此窗口/分栏窄于 ~80 列时，`_terminalShowsOpenFile()` 必然失败。

---

## 4. 根因分析（按影响排序）

### R1（决定性）`:open` 的路径补全逐键扫盘，让「预热」比「重启」更贵

- 证据：§3.3（`:open` 7.9–17 ms/字符 vs 其它输入 0.7 ms/字符）；§1.1 的 B1/B2/B3。
- 机理：`helix-term/src/ui/prompt.rs` 每次 `insert_char`/`insert_str` 都调 `recalculate_completion`；`open` 的补全函数会 `read_dir` 路径前缀所在目录。125 字符的绝对路径 ≈ 120 次目录列举，其中 `D:\`、仓库根、`appflowy_flutter\` 这类大目录单次 10–130 ms。
- 影响：热路径的净收益（省掉 0.36 s 进程创建）被 1.4–2.0 s 的打字成本完全吃掉，还倒亏 0.9–1.9 s。
- 附带：`_enterInsertMode()` 在 280 ms 后写 `i`（`helix_resource_surface.dart:258`、`337-341`）。命令的字节序保证了 `\r` 先于 `i` 被处理，语义上安全，但 280 ms 这个「常量等待」现在明显小于真实的命令送达时间（803–1900 ms），已经没有任何意义。

### R2 预热完成的判据既会误报也会漏报

误报（提前揭开遮罩）：
- `_terminalShowsOpenFile()` 扫最后 3 行（293-302）。xterm 的 alt buffer `lines.length == viewHeight`，所以 `lines.length-3` 确实是屏幕最后 3 行，**包含命令行提示行**（h-1）。
- 命令行回显（h-1）和补全弹窗（画在 h-2/h-3）都包含文件名。实测：小目录下「名字出现在 h-2」只需 **16 ms**（那是 `note.md  scratch.md  command` 补全列表，不是文件）；真实项目文件下，最后三行判据在 **1.09 s** 命中，而文件真正画出来在 **1.81 s** —— 提前 0.72 s 揭开遮罩，用户看到的是命令行 + 补全弹窗或残留的 scratch 缓冲。
- 若 Helix 不在 alt screen（例如 `\x1b[?1049h` 之前），主缓冲 `lines.length` 可达 10000，`lines.length-3` 就指向**回滚区末尾**而不是屏幕，判据静默失效。

漏报（超时后冷重启）：
- 状态栏 ≤80 列截断文件名（§3.6）→ `contains(basename)` 永远为 false。
- 真实命令送达 + 加载完成需要 1.6–2.6 s，而 `deadline = 1000 ms`（267 行）。
- 超时后 `_reboot()`（286 行 → 324-335）走完整冷路径：再次 Dart 准备 + 0.36 s spawn + 首帧 ≈ 再加 1.5 s 以上，且丢掉了刚预热好的进程。

### R3 `Pty.start` 同步阻塞 UI isolate，且补池子正好落在打开过程中

- `Pty.start` 是同步生成构造函数（`vendors/flutter_pty/lib/flutter_pty.dart:48-127`），最后同步调用 `_bindings.pty_create(options)`（118）→ `CreatePseudoConsole` + `CreateProcessW`。
- 实测 0.36–0.51 s（本机 ~30 次采样，中位数 0.37 s；在 hx 重绘风暴期间 0.44–0.51 s）；设计文档 C 冒烟 336 ms。
- 冷路径 177 行、热路径 257 行（`_adoptWarmProcess` 内、`:open` 之后立即）都会冻结 UI：这期间终端字节不能解析、`_watchWarmOpen` 的 60 ms 定时器不推进、`setState` 不执行。
- 注意：我在探针里曾测到过 3.39 s 的离群值，事后定位为**探针自身**的产物（把 `Session` 对象丢弃时触发了 PTY 句柄的销毁路径），不是 `Pty.start` 的成本；`Pty.start` 本身的成本在 `CreateProcessW` 量级（~0.36 s）。但它是**同步、无超时、跑在 UI isolate 上**的调用，这才是缺陷所在。

### R4 工作区预热用的 key 和真实打开用的 key 不匹配

- `workspace_explorer.dart:82-95`：`warmUpForWorkspace(controller, locator)`，`locator` 是**挂载点根目录**。
- `helix_resource_surface.dart:140`：真实打开用 `helixProjectRoot(widget.file)`，从文件向上找到第一个含 `.git`/`pubspec.yaml`/`Cargo.toml`/`package.json`/`go.mod`/`pyproject.toml`/`.helix` 的目录。
- `HelixWarmSpec.key = '<digest>|<binary>|<workingDirectory>'`（102 行）。只要文件不在挂载点根目录（代码文件几乎都在子工程里，例如 `.../appflowy_flutter/lib/...` → root = `.../appflowy_flutter`），key 必然不同 → **工作区预热出来的进程永远拿不到**，白占 2 个空闲槽之一，90 s 后才被 TTL 回收。

### R5 Dart 准备虽便宜，但存在重复写入与配置膨胀

- 每次打开会写 **两次** `runtime/themes/openmuse_host.toml`（`helix_resource_surface.dart:123-131` 与 `helix_warm_pool.dart:57-63`），`writeLanguagesToml()` 被调用 **三次**（`helix_resource_surface.dart:139`、`helix_warm_pool.dart:82`、`helix_language_servers.dart` 内部），配置文本被算两遍。
- `Object.hash` 不稳定导致每次启动新建一个配置文件名（§3.5，已积累 37 个等价文件）。
- 这些合计在 ~9 ms 级别，属于「顺手清理」，不是卡顿主因。

### R6 冷路径在「首批字节」而非「首帧」揭开遮罩

`_onPtyData`（202-213）在第一个数据块就 `_painted = true`。第一个数据块是 ConPTY 的 23 字节初始化序列（`ESC[1t ESC[c ESC[?1004h ESC[?9001h`），此时 Helix 一行字都还没画。用户看到的是「遮罩消失 + 空白终端 0.3–0.4 s + 内容突现」。8 s 的 `_armPaintTimeout()`（220-227）只是兜底。

### R7 查看器（open-file-viewer）逐 tab 重建 WebView2

见 §2.4：每个文件一次 `WebviewController()` + `initialize()` + 整页加载 + `base64` 全量推送；没有任何共享环境、页面复用或预初始化。这是与 Helix 完全同构的问题（都缺「预热 + 廉价切换」），但当前连「廉价切换」的机制都不存在。

---

## 5. 修复建议（含预期收益与代价）

### P0-A（推荐，需改 vendored Helix 并重编 `hx.exe`）给热进程加一条「打开文件」通道

现状是「用终端键盘把命令敲进去」，最贵的一步可以整段去掉。

- 做法：在 vendored Helix（当前是干净的 `079a789e8`，`vendors/helix/` 无任何 openmuse 定制）里加一个极小的命令通道，例如
  - `--command-file <path>`：启动后监听/轮询一个小文件（或命名管道 `\\.\pipe\openmuse-helix-<pid>`），读到一行就按 ex-command 执行（走 `commands::execute`，与命令行回车同一条路径）；
  - 再加一个 ack（例如把「已切换到 <path>」写回同一管道/文件），让 app 的「打开完成」判定变成**显式信号**，彻底摆脱 R2。
- 预期：真实代码文件 **2.60 s → 0.35–0.50 s**（只剩文件加载 + 首帧，即 B2 回归的截距 ≈ 0.49 s），且不再有 R2 的误报/漏报。
- 代价：维护一个 Rust 补丁 + 重编 `hx.exe`（仓库已有 vendored C 插件打补丁并重编的先例：`flutter_pty_win.c` 去掉 `Sleep(1000)`）。这是唯一能真正达到「秒开」的路线。

### P0-B（不改 Helix，中等收益）把池子按「文件所在目录」预热，命令只发裸文件名

- 实测 A2：小目录 + `:open "marker.md"` = **0.29 s**，比同场景冷启动（0.42 s）快 1.5 倍。
- 做法：`HelixWarmSpec.workingDirectory` 改为「文件所在目录的 `helixProjectRoot`」，`:open` 只发相对该目录的短路径；预热按「最近打开过的目录」进行（例如首次打开某目录后立刻补一个该目录的池子）。
- 代价：命中率下降（目录维度比工程维度多），`maxIdleTotal = 2` 需要重新评估；收益上限仍然只是「比冷启动快 ~30%」，因为文件加载 + 首帧（~0.5 s）无法消除。

### P0-C（最省事，立刻止损）先关掉预热池，把力气放回冷路径

- 依据：当前热路径 2.60 s vs 冷路径 0.69 s，**关掉立省 ~1.9 s**，同时消除 R2/R3/R4 引入的全部风险。
- 代价：放弃「省下 0.36 s 进程创建」的收益；需要同时做 P1/P2/P5 才有正向体验。

### P1（必做）修掉虚假的「已打开」判定

- 只看**状态栏那一行**：`lines[lines.length - 2]`（`_terminalShowsOpenFile` 293-302），并且要求提示行 `lines[lines.length-1]` 不含 `:open`；
- 更稳的是要求**文件内容**上屏（例如文件首行文本），或改用 P0-A 的显式 ack / 读 `hx --log` 的 "Loaded 1 file"；
- 超时从 1000 ms 提到 **4000 ms**（真实送达 1.6–2.6 s），并把「超时」与「失败」区分开：超时不要立刻 `_reboot()`，至少先保持遮罩并继续观察。
- 顺带修窄终端漏报：判定文件用**相对路径的尾段**或结合 `initialLine`；或把 PTY 宽度下限钳到 80 列以上（视觉上可接受）。

### P2（必做）把补池子移出打开路径

- `helix_resource_surface.dart:257` 的 `schedule(spec)` 改为：揭开遮罩后（`_painted == true`）再补，或 `Future.delayed(1–2 s)`，或放到独立 isolate；
- 冷路径 196 行的 `schedule()` 同理。
- 预期：每次打开省掉 0.36–0.5 s 的 UI 冻结。

### P3 修 key 不匹配

- `warmUpForWorkspace` 与 `schedule` 使用**同一个** root 语义：改成「首次成功打开某 root 之后再预热该 root」，并删除挂载点预热（或把挂载点预热改成对 `helixProjectRoot` 的无操作探测）。

### P4 稳定 digest，配置只写一次

- 用确定性摘要（FNV-1a/SHA-1 文本）替代 `Object.hash`；
- 合并 `_boot()` 与 `buildHelixWarmSpec()` 的重复写盘（overlay/config/`writeLanguagesToml` 各一次）；
- 清理历史 37 个 `helix-host-*.toml`（并在启动时做一次 GC）。

### P5 冷路径改为「首帧」揭开遮罩

- `_onPtyData` 里不要用「收到任意字节」当首帧判据：至少等到收到 alt-screen 切换 + 一次完整重绘（例如等到 h-2 出现 `NOR`/`INS` 状态栏），或直接用一个最短可见延迟兜底。

### P6 查看器复用 WebView2

- 全应用共享一个 WebView2 环境与一个常驻查看器页面：文件切换只做 `postWebMessage`（并把大文件走 virtual host 映射，避免 base64）；
- 预期：第 2 个及以后的 PDF/图片从 ~1–2 s 降到 ~0.2–0.4 s。

---

## 6. 验收标准与复现命令

**标准**：从点击到「文件内容可见」的中位数 < 0.35 s（当前冷路径 0.69 s、热路径 2.60 s）。

复现（`dart` 与 `python` 均可直接用绝对路径调用）：

```powershell
# 1) Helix 基线：spawn / 首批字节 / 目标上屏
python openmuse\tmp\hx_paint_baseline.py 3

# 2) 端到端对照：冷启动 vs 预热池（绝对/相对路径），判定用文件内容
python openmuse\tmp\hx_truth_compare.py

# 3) 打字成本因果对照（prompt 补全 vs 其它输入）
python openmuse\tmp\hx_completion_probe.py

# 4) Dart 侧每次打开的成本 + digest 稳定性
D:\muse\flutter-3.27.4\bin\cache\dart-sdk\bin\dart.exe openmuse\tmp\helix_open_cost.dart 20
```

改完后应有：P0-A → 热路径 ≤0.5 s；P0-C + P2 + P5 → 冷路径 ~0.5 s 且无空白闪烁；P1 → 遮罩揭开时机与内容一致（不再出现命令行/补全弹窗/scratch 缓冲）。

---

## 7. 附录

### A. 本次新增/保留的探针

| 文件 | 用途 |
| --- | --- |
| `openmuse/tmp/hx_open_latency_probe.py` | 公共探针框架：ConPTY 启动、**应答终端查询**、select 驱动的主线程泵、屏幕 VT 模拟、`:open`/冷启动场景、app 判据 vs 状态栏判据对照 |
| `openmuse/tmp/hx_truth_compare.py` | **结论表**：A/B 两组、冷/热/相对路径，判定用文件内容 |
| `openmuse/tmp/hx_completion_probe.py` | `:open` 补全成本的因果对照 |
| `openmuse/tmp/hx_paint_baseline.py` | Helix 启动构成矩阵（配置/工作目录隔离） |
| `openmuse/tmp/helix_open_cost.dart` | Dart 侧每次打开的 9.4 ms 分解 + `Object.hash` 稳定性 |

（`openmuse/tmp/` 中既有的 `pty_hx_warm_probe*.py`、`pty_win_smoke.*` 为设计文档那一轮的探针，未改动。）

### B. 与设计文档数据的差异

| 设计文档 | 本次实测 | 差异原因 |
| --- | --- | --- |
| `hx --version` 377/507 ms；`hx README.md` 380/377 ms，「Helix 自身启动 ≈ 0.38 s」（第 41-47 行） | spawn 0.36 s、首批字节 +0.05 s、目标上屏 0.37–0.52 s | 0.38 s 基本等于**进程创建**；文件真正上屏还要 ~0.1–0.15 s。结论方向正确，绝对量略偏小 |
| 「关键路径 ≈ resolve + copyGrammars + 写 theme/config + `take()` + `:open` + 状态栏确认 ≈ 100–250 ms」（第 80-82 行） | 热路径 **1.58–2.60 s** | `:open` 的打字成本被完全忽略（当时未做端到端验证，只测了「进程是否就绪」） |
| 「命中 → 揭开遮罩（正常情况 120–360 ms）」（第 191 行） | app 判据 1.09 s 命中（**且是误报**），真实内容 1.81 s | 判据把命令行回显/补全弹窗当成了状态栏 |
| 「会话内第 2 个及以后~0.15–0.35 s」（第 291 行） | 1.58–2.60 s | 同上 |
| 「所有 PTY 结论都必须在主线程用阻塞 `read()` 得出」（第 304 行） | 正确，但**不充分** | 还必须应答 ConPTY 的 `ESC[c` 等查询，否则数据虚高 ~3 s；另外主线程应改用 `select + read`，否则容易挂死 |

### C. 备注：为什么「补全扫盘」这么贵

`:open` 的参数补全在 Helix 里是路径补全：每次按键都会对「当前输入前缀所在目录」调用一次 `read_dir` 并做模糊匹配。输入绝对路径 `D:\agentic\src\openmuse-io\openmuse\frontend\client\frontend\appflowy_flutter\lib\plugins\resource_surface\surfaces\helix_resource_surface.dart` 时，前缀每跨越一个 `\` 才换一个目录，因此在 `D:\`、仓库根、`client\`、`frontend\`、`appflowy_flutter\`、`lib\`、`plugins\`、`resource_surface\`、`surfaces\` 这几个目录里各自会重复扫描数十次；而这些目录中 `D:\` 和仓库根都很大，单次枚举 10–130 ms（Windows Defender 实时扫描会进一步放大）。这解释了为什么 A3（46 字符但位于 `%TEMP%` 大树下）反而要 5.97 s。
