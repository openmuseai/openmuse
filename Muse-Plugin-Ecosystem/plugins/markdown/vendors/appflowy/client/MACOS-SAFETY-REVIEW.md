# macOS 兼容性复核（Windows 移植改动逐文件审计）

对象：`frontend/client`（分支 `main_rebrand_0908`）当前工作区，即"跑通 Windows"这一轮引入的全部改动。
目的：确认这些改动**没有改坏 macOS 逻辑**。结论：**没有发现破坏 macOS 的改动**，只有一处需要知道的环境前提
（`vendors/AppFlowy-Collab`）和两处需要说明的行为差异，详见文末。

复现审计命令（在 `frontend/client` 下）：

```powershell
git status --porcelain
git diff --stat
git diff -- frontend/rust-lib/Cargo.toml
```

## 1. 改动清单与 macOS 影响

| 文件 | 改动性质 | macOS 影响 |
| --- | --- | --- |
| `vendors/flutter_pty/src/flutter_pty_win.c` | 只改 Windows 实现（argv[0] 重复、引号/UTF-8、去掉 `Sleep(1000)`） | **无**。整份 vendored 包与上游 `flutter_pty-0.4.2` **逐字节比对，唯一差异就是这个文件**；`macos/Classes/flutter_pty.c`、`ios/Classes/flutter_pty.c`、`src/flutter_pty_unix.c`、`src/forkpty.c` 全部原样 |
| `vendors/flutter_pty/pubspec.yaml` | 未改 | 原本就声明 `android/ios/linux/macos/windows` 全部 `ffiPlugin: true`，macOS 正常注册 |
| `helix_install.dart` | 增加 `hx.exe` 双名查找 + Windows bundle 目录 | **向后兼容**：`.app`/Frameworks 路径仍是**第一个**候选（原逻辑一字不改），`binaryNames` 先找 `hx`；新增的 `data/flutter_assets/...` 候选在 macOS 上不存在，直接跳过 |
| `helix_language_servers.dart` | PATH 分隔符、`chmod` 平台分支、纯 Dart `which`、`languages.toml` 签名缓存 | **语义等价**：`Platform.isWindows ? ';' : ':'` 在 macOS 仍是 `:`；`_markExecutable` 的 `chmod` + `xattr` 分支保留，只在 Windows 提前返回；`_which` 在 POSIX 上仍是"扫 PATH 找同名可执行文件"，不再 `Process.run('which')` |
| `helix_resource_surface.dart` | 启动遮罩 + 预热进程池（`take()` 优先，未命中走原冷启动） | **冷启动路径逐行保留**；预热是纯增量（`try/catch` 包裹，失败即 `null`）→ macOS 同样获得提速，并有 1 s 冷启动兜底 |
| `helix_warm_pool.dart`（新增） | 预热池 | 平台无关实现：`Pty.start` / `File` / `Directory` / `p.join`；PATH 前缀在 macOS 用 `:`；`helixGrammarProcessEnvironment` 直接复用冷启动的环境构造 |
| `open_file_viewer_resource_surface.dart` | 新增 Windows/WebView2 分支 | **macOS 分支不变**：`initState`/`build`/`dispose` 全部 `Platform.isWindows` 判断后才走新代码；非 Windows 仍是 `webview_flutter` + `MuseViewerBridge`；消息回调只是被提取成 `_onBridgeMessage`，逻辑完全一致 |
| `pubspec.yaml` | 新增 `flutter_pty` → `vendors/flutter_pty` 覆盖、`assets/engines/helix/runtime/grammars/` | **安全**：覆盖包是全平台的（见上）；`webview_windows` 本来就已经是 `dependencies`（第 184 行，本次未新增） |
| `tool/open_file_viewer/muse_viewer.ts` | `reply()` 增加 WebView2 回退 + 监听宿主 message | **macOS 优先路径不变**：仍先判 `window.MuseViewerBridge`；`chrome.webview` 在 macOS 上是 `undefined`，全部走可选链空操作 |
| `workspace_platform/presentation/workspace_explorer.dart` | `_open()` 后预热第一个挂载点 | 用 `getIt.isRegistered<HelixSettingsController>()` 守卫 + `warmUpForWorkspace` 内部 `try/catch`，失败只打印一行 `[helix-warm] … skipped` |
| `startup/tasks/windows.dart` | 关闭/销毁窗口时 `HelixWarmPool.instance.dispose()` | 该任务在 `startup.dart:134` **无条件注册**（`InitAppWindowTask()`），所以 macOS 关窗时同样会清理预热进程，不会残留 |
| `frontend/rust-lib/Cargo.toml` | collab 系列从 git rev 改为 `path = ../../../../vendors/AppFlowy-Collab/*` | 见第 2 节：这是产品自带脚本 `frontend/scripts/tool/update_collab_source.sh` 的既定做法，macOS 需要一个已存在的 vendor 检出 |
| `scripts/pack-windows-client.py` | 仅 Windows 打包 | macOS 不使用（macOS 仍走 `tool/build_macos_engine_assets.sh`，未改） |
| `pubspec.lock` / `Cargo.lock` | 依赖解析产物 | 平台无关文件，macOS 上 `flutter pub get` / `cargo` 会按需重算，不影响逻辑 |
| `test/helix/helix_boot_cost_test.dart`、`WINDOWS-*.md`、`tmp/` 脚本 | 只读/文档/工具 | 不参与 macOS 构建 |

## 2. `Cargo.toml` 的 collab 改动（唯一需要留意的点）

改动内容：`[patch.crates-io]` 里 8 个 collab crate 由
`{ version = "0.2", git = ".../AppFlowy-Collab", rev = "4dfccef" }`
改为 `{ version = "0.2", path = "../../../../vendors/AppFlowy-Collab/<crate>" }`。

核实结果：

- 该 vendor 检出 `HEAD = 4dfccefb9e4fd35240107b93fb4b5966295b452e`，**就是被钉住的 rev `4dfccef`**（ahead 0）；
  工作区仅有 `collab-folder/src/view.rs`（+16 行）未提交改动 —— 即 Muse 的办公布局补丁
  （`ViewLayout Word/Excel/Slides/Pdf` + `is_office_blob`）。也就是说 `path` 指向的源码 == 原 rev + 产品补丁，
  **不是另一个版本**。
- 产品自带脚本 `frontend/scripts/tool/update_collab_source.sh` 干的就是同一件事，写的也是
  `{crate} = { path = "$repo_path/$crate" }`，并且会在缺目录时自动 `git clone`。Cargo 把 git 依赖视为不可变源，
  打过的 cargo checkout 不会被重新感知，所以本地 path 是本仓库文档化的既定手段（`Cargo.toml` 原注释就写着
  "To switch to the local path, run: scripts/tool/update_collab_source.sh"）。

因此对 macOS 的影响：

1. **代码语义不变**：同一 rev、同一份补丁；
2. **唯一前提**：macOS 那台机器上要存在 `vendors/AppFlowy-Collab`。该目录与 macOS 构建本来就需要的
   `vendors/helix`、`vendors/open-file-viewer`、`vendors/ioffice` 在**同一个 `vendors/` 树**里，
   正常交付的工作区必然带着它；
3. 若要 100% 回到改前的 macOS 行为，一条命令即可：
   `git -C frontend/client checkout -- frontend/rust-lib/Cargo.toml`
   （代价：Windows 会丢掉办公布局补丁，需要重跑 `update_collab_source.sh` 再回来）。
   注意 `[patch]` 表不支持按 target 分平台，所以无法写成"仅 Windows 用 path"。

## 3. 两处已知的行为差异（非破坏，透明说明）

1. `_which()` 不再调用系统 `which`。POSIX 语义等价于"按 PATH 顺序找同名文件"，但不检查可执行位，
   也不识别 shell 函数/别名。极端情况下若 PATH 里有一个与 LSP 同名的**目录**，会返回该路径
   （Helix 启动 LSP 失败并自行报错，不影响编辑器）。如需要，可在下次重建时加
   `FileSystemEntity.typeSync(path) == FileSystemEntityType.file` 收紧。
2. macOS 现在也会常驻一个空闲 Helix 预热进程（最多 2 个、空闲 90 s 回收、关窗 `dispose()` 全部 kill）。
   这是本次设计的预期收益（首帧从 ~0.38 s 进程启动里省掉），不是逻辑改动；若某台机器上不希望常驻，
   把 `helix_resource_surface.dart` 里 `take()` 之前的 `buildHelixWarmSpec(...)` 调用去掉即可回到纯冷启动。

## 4. 结论

- macOS 的三条关键分支（Helix 启动参数与 bundle 解析、Open File Viewer 的 `webview_flutter` + `MuseViewerBridge`、
  `flutter_pty` 的 Unix/macOS 原生实现）**都保持原样**，未引入任何 macOS 专属 API 或平台假设。
- Windows 专属代码全部有 `Platform.isWindows` 或独立文件边界；跨平台文件里的改动都是"加大分支、保留原分支"的形式。
- 唯一实质性的跨平台依赖变更是 collab 的 git→path，已核实指向同一 rev + 产品补丁，并可由该仓库自带脚本复现。
