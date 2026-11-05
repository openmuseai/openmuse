# Helix 打开加速：预热进程池设计

> 适用：OpenMuse / 思构 桌面端（Windows 为主，方案与平台无关）
> 相关代码：
> - `frontend/appflowy_flutter/lib/plugins/resource_surface/surfaces/helix_resource_surface.dart`
> - 新增 `frontend/appflowy_flutter/lib/plugins/resource_surface/helix/helix_warm_pool.dart`
> - `vendors/flutter_pty/src/flutter_pty_win.c`（上一轮已改）

---

## 1. 背景与目标

### 1.1 现象

每打开一个 Helix 文件（`.md .js .json .toml .yaml .dart .rs .py .sh …`）：

1. 点击后先出现一段空白（旧版本）或转圈（当前版本）；
2. 约 0.4–1.0 s 后才出现文件内容与语法高亮。

用户反馈："每次打开一个文件，都很慢，都要加载一下空白页面，再出现内容"。

### 1.2 已经做掉的部分（上一轮）

| 改动 | 效果 |
| --- | --- |
| 删除 `vendors/flutter_pty/src/flutter_pty_win.c` 里 `CreateProcessW` 之前无条件的 `Sleep(1000)`，改为仅失败重试时退避 | 每次打开省掉固定 1 s（冒烟测试：`CreateProcessW` 336 ms，之前 ≥1 s 纯等待） |
| `HelixResourceSurface` 在 PTY 建立到 Helix 画出第一帧之间显示 loading 遮罩 | 不再出现"空终端"的空白页 |

### 1.3 本轮目标

把「点击 → 看到内容」从约 1 s 压到 **0.1–0.2 s**，且**不改变编辑语义**（同样的配置文件、同样的 working dir、同样的插入模式、同样的 dirty/save 行为）。

---

## 2. 现状与实测数据

用 `pywinpty` 在真实 ConPTY 里跑打包产物里的 `hx.exe`（与 app 走同一条 ConPTY 路径）：

| 场景 | 首次输出 |
| --- | --- |
| `hx --version` | 377 / 507 ms |
| `hx README.md`（working dir = 巨型 `frontend/client` 根） | 380 / 377 ms |
| `hx README.md`（working dir = 空的小目录） | 392 / 374 ms |

结论：

1. **Helix 自身启动 ≈ 0.38 s**（Windows 进程创建 + 运行时/主题/grammar 加载），与工作目录大小无关；
2. 已启用的 LSP（`rust-analyzer`、`ruff`）**不阻塞首帧**，Helix 异步拉起；
3 一次打开的耗时构成 ≈ app 侧准备（读设置、解析运行时、写 theme/config/languages.toml，约 0.2–0.3 s）＋ **hx 进程启动 0.38 s** ＋ 终端首帧。

因此：**剩下的地板就是"每个文件新起一个 hx 进程"**。要再快，只能把进程启动从关键路径上挪走 —— 这就是预热。

### 2.1 第一次构建后的实测：真正的瓶颈在 PTY 之前

第一次带预热池的构建仍然"每次都要等、每次都显示正在启动 Helix 编辑器"，于是用一个
Flutter 测试（`test/helix/helix_boot_cost_test.dart`）把 `_boot()` 在 spawn 之前做的每一步都计时：

| 步骤 | 耗时 |
| --- | --- |
| `HelixLanguageServerInstaller.refresh()`（启动时一次） | 379 ms |
| `writeLanguagesToml()`（**每次打开都跑**） | **343 ms** |
| `writeLanguagesToml()` 第二次（冷路径会再跑一次） | **321 ms**（无任何缓存） |
| `HelixInstall.resolve()` | 5 ms |
| `copyHelixGrammars()` | 5 ms |

也就是说：每次打开文件光"给 7 个 LSP 逐个扫 PATH 生成 languages.toml"就要 ~340 ms，
冷路径还要再来一次 ≈ 660 ms —— 全部发生在 loading 遮罩期间，预热省下的 0.38 s 进程启动
完全被它淹没。**这就是"预热做了但还是慢"的真正原因。**

修复：`writeLanguagesToml()` 增加签名缓存（revision + overrides 指纹），
内容没变且文件仍在时直接返回；`refresh()` 时把签名置空，强制下次重写。

修复后实测：

| 步骤 | 修复前 | 修复后 |
| --- | --- | --- |
| `writeLanguagesToml()` 第一次 | 343 ms | 343 ms（首次仍要算） |
| `writeLanguagesToml()` 第二次 | 321 ms | **0 ms** |

于是每次打开 Helix 文件的关键路径 ≈ `resolve` 5 ms + `copyHelixGrammars` 5 ms +
写 theme/config ~5 ms + `take()`（预热命中，≈0）+ `:open` + 状态栏确认（60 ms 轮询）
≈ **100–250 ms**。

### 2.2 实现前用真实 hx.exe 做的契约验证

| 验证 | 结果 |
| --- | --- |
| `hx --working-dir <dir> <file>` 在真实 ConPTY 里首次输出 | **~380 ms**，且与工作目录大小无关 |
| **`hx` 不带文件参数** | 会直接打开 Helix 自己的**文件选择器（picker）**，键盘输入被 picker 吃掉，`:open` 根本到不了命令行 ✗ |
| `:open` 是否支持 `path:line` | 支持：`typed.rs::open_impl` → `crate::args::parse_file(&arg)` |
| `:open` 是否需要引号 | 需要：`:open` 的 `Signature` **没有**设 `raw_after`（全仓库只有 `:shell`/`:set-option`/`:toggle-option` 等设了），所以按普通引号规则切分，带空格的路径必须用 `"..."`，引号会被剥掉 |

结论（已反映到下面设计里）：**预热进程必须带一个占位文件启动**（否则停在 picker 上，`:open` 无效），文件内容用 `:open "绝对路径[:行号]"` 切换。

---

## 3. 为什么现在是"一文件一进程"

不是 Windows 引入的，而是既有架构：

- `HelixResourceSurface` = 一个资源标签一个 surface；`initState` 里 `attach(file)`，`_boot()` 里 `Pty.start(binary, arguments: ['--config', …, '--working-dir', …, file])`，`dispose()` 里 `pty.kill()`；
- Helix 本体是 TUI 程序，一次调用打开一个文件（`hx <file>`），没有"服务端 + 多 buffer"模式；
- macOS 版就是这套，Windows 只是补齐。

### 3.1 备选方案

| 方案 | 做法 | 收益 | 代价 / 风险 |
| --- | --- | --- | --- |
| A（已完成） | 删 `Sleep` + 首帧 loading 遮罩 | 省 1 s，不再空白 | 地板仍是 ~1 s |
| **B（本方案）** | **保留一个"空跑"的 hx 进程待用，打开文件时把它 `:open` 到目标文件** | **关键路径只剩 `:open` + 重绘 ≈ 0.1–0.2 s** | 需要一个池 + 缓冲回放；池子失效时自动降级 |
| C | 常驻单一 hx 宿主，`:open`/`:buffer` 复用多文件 | 打开近乎瞬时 | 多 buffer 状态、dirty 回写、PTY resize、崩溃恢复全要自己管，回归面大 |
| D | 简单文件用原生预览控件替代 Helix | 快 | 放弃 Helix 一致性，不建议 |

**选 B**：收益接近 C，改动只集中在 `HelixResourceSurface` + 一个新文件，且始终保留"冷启动"兜底路径，最坏情况退化为当前行为。

---

## 4. 详细设计

### 4.1 池模型

```
HelixWarmPool (进程级单例)
  _idle: Map<key, HelixWarmProcess>      // 每 key 最多 1 个
  maxIdleTotal = 2                       // 最多同时预热 2 个（不同 key）
  idleTtl     = 90 s                     // 闲置超时后杀掉
  readyWait   = 1.2 s                    // take() 最多等这么久让它"画出画面"

key = f(binary, workingDir, arguments, HELIX_RUNTIME, XDG_CONFIG_HOME, settingsFingerprint)
```

- `schedule(spec)`：后台拉起一个**不带文件参数**的 hx（`hx --config <warm toml> --working-dir <dir>`），把它的输出缓存在内存里，放进 `_idle`，并挂 TTL 定时器。失败只记日志，**绝不抛出**。
- `take(spec)`：命中且健康 → 返回该进程（从 `_idle` 移除，所有权转移给 surface）；未命中/已死/缓冲溢出 → 返回 `null`，surface 走原来的冷启动路径。
- `invalidate()`：设置变更时杀掉所有闲置进程。
- `dispose()`：杀掉所有闲置进程并取消定时器（窗口关闭时调用）。

`HelixWarmProcess` 职责：

| 成员 | 说明 |
| --- | --- |
| `Pty pty` | 已启动的 Helix |
| `BytesBuilder _buffer` | 记录 idle 期间的全部输出（Helix 的 dashboard 等），上限 256 KB |
| `bool ready` | 收到过至少 1 字节输出 |
| `bool dead` | `pty.exitCode` 完成（进程退出）或缓冲溢出 |
| `takeBufferedOutput()` | 取走缓冲（供 surface 回放进自己的 Terminal） |
| `releaseToSurface()` | 池子彻底放手，不再 kill/缓冲 |

### 4.2 预热进程如何"变成"目标文件

**预热进程带占位文件启动**：`hx --config <warm toml> --working-dir <dir> <warmRoot>/scratch.md`
（`scratch.md` 是应用数据目录下的一个固定小文件）。带文件启动可以避开 Helix 的 picker，
命令行才可用；占位文件本身不会被用户看到（有 loading 遮罩），也不会被写坏
（`:w` 只作用于当前 buffer，buffer 没切过去时写的是 scratch 自己）。

Helix 支持在命令行里 `:open`：

- `helix-term/src/commands/typed.rs::open_impl` → `crate::args::parse_file(&arg)`，**支持 `path:line:col` 后缀**；
- `:open` 的 `Signature` 未设置 `raw_after`，因此走普通引号规则：带空格的路径写 `"..."`，
  引号会被剥掉（已核对 vendored 源码）。

因此 surface 采用目标文件后，向 PTY 写入：

```
:open "<绝对路径>"[:<行号>]\r
```

- 行号：`widget.initialLine != null` 时写成 `path:line`（等价于命令行 `hx file:line`）；
- 引号内的 `"` 用 `""` 转义（Windows 路径不允许 `"`，实际不会出现）；
- 发送后等 280 ms，再走既有的 `_enterInsertMode(pty)`（写 `i`）。

**working dir 语义不变**：`--working-dir` 仍取 `helixProjectRoot(file)`，并通过 `key` 保证"只有同一个 working dir 的预热进程才会被复用"，所以文件选择器（space-f）的根目录、相对路径解析与现在完全一致。

### 4.3 终端模型一致性（关键）

surface 里的 `Terminal` 是本地终端模型，**必须收到进程发出的全部字节**，否则转义序列错位、画面花掉。

所以 idle 期间的输出不能丢：`HelixWarmProcess` 把它们按序缓存，surface 接管时先 `_terminal.write(buffered)`，再订阅后续 `pty.output`。

- 缓冲 > 256 KB → 认为该进程不可信，标 `dead` 并杀掉，surface 走冷启动（不会拿到错位画面）。
- 缓冲里的 dashboard 只写进终端模型，**不立刻显示**（见 4.4）。

### 4.4 首帧揭示时机（不闪 dashboard、不闪命令行）

`_painted` 控制 loading 遮罩：

- 冷启动：spawn 之后置 `_revealRequested = true`，第一段输出到达 → `_painted = true`；
- 预热接管：先回放缓冲（遮罩保持），写完 `:open` 后由 `_watchWarmOpen` 每 120 ms 检查一次
  **终端最下面 3 行里是否出现目标文件名**（Helix 的状态栏就在底行；alt-screen 里 `buffer.lines`
  最后几行就是屏幕底部）。

  - 命中 → 揭开遮罩（正常情况 120–360 ms）；
  - **1.5 s 内没命中 → 判定 `:open` 没生效，直接冷启动重来**（kill + 走原来的冷路径）。
    这样即使某个环境里命令行行为异常，用户也只会多等 ~1 s，**绝不会看到占位文件**；
  - 8 s 兜底定时器保留（Helix 完全不输出时也不会一直盖着）。

> 为什么不能用"有输出"当成功信号：Helix 会回显正在输入的命令行，`:open` 一敲就有输出，
> 但此时文件还没打开。所以必须看状态栏内容。

### 4.5 失效、健康与容量

| 情况 | 处理 |
| --- | --- |
| 预热进程启动失败 | 记日志、丢弃；surface 冷启动 |
| 预热进程中途退出 | `pty.exitCode` 完成 → 标 `dead`，`take()` 时丢弃并返回 `null` |
| 设置变更（主题/键位/LSP/背景/字体） | `invalidate()` 杀掉闲置进程；新 key 含 settings 指纹，旧进程不会被误用 |
| 闲置超过 90 s | 定时器杀掉，释放内存与进程 |
| 闲置数超过 2 | 淘汰最旧的 |
| 应用退出 | `onWindowClose()` / `dispose()` 调 `HelixWarmPool.instance.dispose()`；另有 TTL 兜底 |

指纹 = `theme|keymap|enableLsp|backgroundColor|fontFamily|fontSize|lspRevision`，同时决定预热配置文件的路径
（`<installer.root>/warm/helix-host-<hash>.toml`），因此旧配置的进程天然不会与新设置混用。

### 4.6 与 surface 的接口（保持兜底）

`_boot()` 尾部改成：

```
spec = HelixWarmSpec(...)                      // 由 surface 组装（binary/args/env/cwd/指纹）
warm = await HelixWarmPool.instance.take(spec)
if (warm != null)  _adoptWarm(warm)            // 快路径
else               _spawnCold(...)             // 现有逻辑，一字不改
HelixWarmPool.instance.schedule(spec)          // 无论走哪条路，都补一个预热进程给下一个文件
```

`_adoptWarm` 做的事：接管 `Pty` → 回放缓冲 → 订阅输出 → `_booted = true` → `setState` → `_syncPtySize()`
→ 发送 `:open` → `_enterInsertMode`。

`_reboot()`（设置变更触发的重启）沿用同一条 `_boot()`，自然复用/重建预热进程。

### 4.7 预热触发点

1. **每次 Helix surface 启动后**（`schedule`）——保证"打开第 2 个及以后的文件"必然是热的；
2. **工作区挂载完成后**（`MuseWorkspaceExplorer` 拿到 mount 根目录时）——让"会话里第一个 Helix 文件"也尽量吃到预热；该调用是 `unawaited` + try/catch，失败无影响。

### 4.8 失败模式与降级矩阵

| 失败 | 用户可见结果 |
| --- | --- |
| 池 spawn 失败 | 与今天完全一致：冷启动 + loading 遮罩 |
| `take()` 拿到死进程 | 丢弃 → 冷启动 |
| `:open` 失败（文件被删等） | Helix 显示自身错误信息（与命令行 `hx file` 行为一致） |
| 预热进程占内存 | 单个 hx ≈ 40 MB；最多 2 个，90 s 后释放 |
| app 崩溃退出 | 最多残留 2 个 hx 进程，90 s 后由 TTL 自杀（app 正常退出时立即 kill） |

---

## 5. 代码改动清单

| 文件 | 改动 |
| --- | --- |
| `lib/plugins/resource_surface/helix/helix_warm_pool.dart` | **新增**：`HelixWarmSpec`、`HelixWarmProcess`、`HelixWarmPool`、`buildHelixWarmSpec`（含 scratch 占位文件与配置指纹）、`helixOpenCommand[Bytes]` |
| `lib/plugins/resource_surface/surfaces/helix_resource_surface.dart` | `_boot()` 接入 take/schedule；新增 `_adoptWarmProcess`、`_watchWarmOpen`、`_terminalShowsOpenFile`、`_onPtyData`/`_onPtyError`、`_armPaintTimeout`；`_onSettingsChanged` 里 `invalidate()`；`_reboot`/`dispose` 取消 `_openWatch` |
| `lib/workspace_platform/presentation/workspace_explorer.dart` | 挂载就绪后 `unawaited(pool.warmUpForWorkspace(...))`（可选预热点） |
| `lib/startup/tasks/windows.dart` | `onWindowClose()` / `dispose()` 里 `HelixWarmPool.instance.dispose()` |

不改：`helix_install.dart`、`helix_language_servers.dart`、`helix_settings.dart`、`flutter_pty`（上一轮已完成）。

---

## 6. 验证计划

1. `dart analyze` 三个改动文件无告警；
2. 全量构建 + 打包：`python scripts\pack-windows-client.py --skip-tests --skip-core-build --skip-packages`；
3. **人工验证**（用户操作，我读日志/进程佐证）：
   - 打开 1 个 `.md`：应有转圈，然后内容出现（首个文件可能仍 ~0.4 s，因为预热可能还没就绪）；
   - 再连续打开 3–5 个不同 Helix 文件：应"几乎立刻"出内容，无空白等待；
   - 打开 PDF/图片：Open File Viewer 正常（回归检查）；
   - 关掉应用：`hx.exe` 不残留（`Get-Process hx` 为空），等 90 s 确认 TTL 也生效；
   - 改一次 Helix 主题：当前标签会重启（原有行为），再次打开文件依旧快。

---

## 7. 风险与回滚

| 风险 | 缓解 |
| --- | --- |
| 预热进程与目标文件不在同一 working dir | key 里含 working dir，不匹配就不复用（退化冷启动） |
| 缓冲回放导致终端错位 | 缓冲上限 256 KB，超限即弃用该进程；回放严格按字节序；接管后改由单订阅 StreamController 转发，避免"释放到 listen 之间"的丢字节 |
| `:open` 在某些环境没生效（命令行被吞、权限等） | 1.5 s 内状态栏没出现目标文件名 → 自动冷启动重来，用户最多多等 ~1 s，且永远看不到占位文件 |
| 首个文件仍慢 | 工作区挂载后即预热 + 遮罩；且不影响后续文件 |
| 关闭顺序导致残留 hx | `onWindowClose` kill + 90 s TTL + 最多 2 个 |
| 占位文件被误写 | `:w` 只写当前 buffer；未切到目标文件时写的是 `scratch.md` 自己，1.5 s 后即被冷启动替换 |
| 想完全关掉预热 | 回滚只需让 `take()` 恒返回 `null`（或删掉 `schedule` 调用），其余代码即退化为今天的冷启动路径 |

---

## 8. 预期结果

| 场景 | 现在 | 之后 |
| --- | --- | --- |
| 会话内第 2 个及以后 Helix 文件 | ~0.9–1.1 s，先转圈 | **~0.15–0.35 s**（`:open` + 重绘），遮罩基本一闪而过 |
| 会话内第 1 个 Helix 文件 | ~0.9–1.1 s | 工作区挂载后已预热则同上；否则 ~0.9 s（有遮罩） |
| 非 Helix 文件（PDF/图片/docx） | 不受影响 | 不受影响 |
| 预热失效/异常 | — | 自动冷启动，行为与今天一致，只是多 ~1 s |

## 9. 验证记录（本轮）

- `dart analyze`（`helix_warm_pool.dart`、`helix_resource_surface.dart`、`workspace_explorer.dart`、`windows.dart`）：**No issues found**
- PTY 契约验证脚本：`openmuse/tmp/pty_hx_warm_probe.py`、`pty_hx_warm_probe3/4/5.py`
  - 结论 1：`hx <file>` 首帧 ~380 ms（与目录大小无关）
  - 结论 2：`hx` 无文件 → 打开 picker，键盘输入进不去命令行 → **必须带占位文件预热**
  - 结论 3：`:open` 未设 `raw_after` → 使用普通引号规则 → 路径用 `"..."` 包裹
  - 注意：`pywinpty` 的 `read()` 在**非主线程**里收不到数据（脚本里表现为只拿到 23 字节终端初始化序列），
    所有 PTY 结论都必须在主线程用阻塞 `read()` 得出
