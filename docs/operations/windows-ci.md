# Windows CI（GitHub Actions）

本仓库是**单仓库**：Host（`app/openmuse_host`）、插件（`plugins/`）、Dart 包（`packages/`）、
Rust crates（`crates/`）和打包脚本（`scripts/`）都在同一个 checkout 里，没有 submodule。
因此 Windows 流水线不需要像旧的三棵树工程那样先校验 submodule，也没有"client 提交没推上来"
这一类前置条件。

## 文件清单

| 文件 | 职责 |
| --- | --- |
| `.github/workflows/windows-build.yml` | 出包流水线：装工具链 → 算指纹 → 搬缓存 → 调 `build-windows.ps1` → 上传产物 |
| `.github/workflows/diagnose-windows.yml` | 只跑工具链探针，几分钟出结果，用来回答"为什么看不到 Visual Studio" |
| `.github/workflows/desktop-gates.yml` | push/PR 门禁；Windows job 调 `build-windows.ps1`，macOS job 调 `build-macos.sh` |
| `scripts/ci/bootstrap-windows.ps1` | 装 rustup target、开 Windows desktop、precache Flutter 产物，并报告工具链身份 |
| `scripts/ci/build-windows.ps1` | **唯一的构建入口**：rust / dart / flutter / pack / verify 五个阶段 |
| `scripts/ci/stage-dsh-windows.ps1` | 组装固定 Node 22.19.0 + DSH npm closure，拷到 `OpenMuse.exe` 旁边 |
| `scripts/ci/stage-helix-windows.ps1` | `cargo build -p helix-term` 产出 `hx.exe`，连同 pinned runtime 打进 Flutter assets |
| `scripts/ci/lib/domains.ps1` | 指纹、工具链身份、域标记的读写 |
| `scripts/ci/lib/domain-plan.ps1` | 域的定义（输入、产物、是否可缓存），指纹脚本和构建脚本共用 |
| `scripts/ci/domain-fingerprints.ps1` | 只算 key，导出给 workflow 的 `actions/cache` |
| `scripts/ci/diagnose-windows-vs.ps1` | vswhere / MSVC / SDK / CMake / Ninja / `flutter doctor`，可选 scratch 构建 |
| `scripts/ci/remote-build-windows.ps1` | 把 GitHub Windows runner 当远程编译机：dispatch + 跟踪 + 拉失败日志 |
| `scripts/package_windows.ps1` | 历史入口，转发到 `scripts/ci/build-windows.ps1` |

设计约束（与参考实现一致）：**构建逻辑全部在 `scripts/ci/*` 里，workflow 只做工具链准备、
缓存搬运和产物上传**。`actions/cache` 的 restore/save 属于"搬运"，不属于构建逻辑。

## 前置条件

- Windows 包在 `pack` 阶段现场编译 Helix：`scripts/ci/stage-helix-windows.ps1` 从
  `third_party/helix` 打出 `hx.exe`，再把 `plugins/helix/assets/engines/helix/runtime`
  拷进 `data/flutter_assets/.../engines/helix/`。仓库里 pin 的 `hx` 仍是 macOS 通用二进制，
  不会打进 Windows 归档。`HELIX_DISABLE_AUTO_GRAMMAR_BUILD=1` 必须打开，否则 `helix-term`
  的 `build.rs` 会在干净 checkout 里去 clone 全部 tree-sitter 语法。
- `pack` 在写 ZIP 前会以随包 `hx.exe` 和 `flutter_pty.dll` 强制运行 Windows PTY 打开文件测试、两个脏文件会话的 Vim/VS Code 双向切换测试；缺失原生库或版本标记 `openmuse-nonmodal.4` 时直接失败。此门禁须在 Windows runner 实际执行，macOS 上的脚本语法检查不能代替实机结果。
- DSH 同样在 `pack` 阶段组装：固定 Node `22.19.0` + `third_party/dsh` 的 npm closure，
  落到 `openmuse/dsh/`。`verify` 会检查 `node.exe`、`bin.js`、`hx.exe` 和
  `runtime/languages.toml`。
- `workflow_dispatch` 要求 workflow 文件已经在该仓库的**默认分支**上。第一次添加
  `windows-build.yml` 后必须先 push 到 `main`，才能用 `remote-build-windows.ps1` 触发。

### 推送权限

`scripts/ci/*` 按 `-Token` → `OPENMUSE_TOKEN`（进程、用户、**机器**环境变量）→ `GH_TOKEN` /
`GITHUB_TOKEN` → `gh auth token` → git 已存凭据的顺序取用，所以推送和触发 workflow 都不需要再登录。
换 token 时：

```powershell
setx OPENMUSE_TOKEN "<新的 classic PAT，需 repo + workflow>"
```

只对新开的进程生效；脚本另外会读用户级环境变量，所以不重开终端也能生效。

> ⚠️ 不要用 `cmdkey /generic:"git:https://github.com" /pass:"<PAT>"` 写凭据：Git Credential
> Manager 存的是自己序列化过的 blob，读不懂这种纯文本条目，实测结果是 push 挂住后报
> `fatal: Cannot prompt because user interactivity has been disabled`。

## 触发方式

```powershell
# GitHub UI: Actions -> "Windows build" -> Run workflow
```

| 输入 | 默认 | 说明 |
| --- | --- | --- |
| `profile` | `release` | `release` / `debug` |
| `rust_targets` | `x86_64-pc-windows-msvc` | 逗号分隔的 rustup target；第一个是被打包的目标 |
| `skip_tests` | `false` | 跳过 Dart / Rust 测试（`cargo test` 降级为 `cargo build`） |
| `force_rebuild` | `false` | 忽略所有域缓存，全量重建 |
| `retention_days` | `14` | 产物保留天数 |

另外 push tag `v*` 也会触发一次出包，并且**强制全量**：正式包不复用缓存。

本地触发（不需要 gh CLI）：

```powershell
cd D:\openmuse-io\openmuse
# origin 指向 github.com/openmuseai/openmuse 时可省略 -Repository
pwsh scripts/ci/remote-build-windows.ps1 -Push
pwsh scripts/ci/remote-build-windows.ps1 -Inputs @{ skip_tests = 'true'; force_rebuild = 'true' }
pwsh scripts/ci/remote-build-windows.ps1 -RunId 123456789 -DownloadLogs
```

## 产物

上传 `dist/` 下的：

- `OpenMuse-windows-x64.zip`（便携目录，解包即为可运行目录）
- `SHA256SUMS.txt`（归档的 SHA-256）
- `build-info.txt`（commit、profile、构建时间、工具链版本、**各域是 cache 还是 build**）

`build-windows.ps1` 的 `verify` 阶段会自己校验归档里存在 `OpenMuse.exe`、`flutter_windows.dll`、
`data/app.so`、随包 Node/DSH CLI 和 `hx.exe` + `runtime/languages.toml`，并核对 `SHA256SUMS.txt`，
所以上传的产物不可能是半成品。

## 本地复现（和 CI 完全一致）

```powershell
cd D:\openmuse-io\openmuse

# 1. 环境：rustup target + Windows desktop + Flutter windows 产物
pwsh scripts\ci\bootstrap-windows.ps1

# 2. 先看要做什么（-DryRun 保证不改任何文件）
pwsh scripts\ci\build-windows.ps1 -DryRun

# 3. 完整构建（含测试、打包和校验）
pwsh scripts\ci\build-windows.ps1

# 4. 按域复用：命中就跳过，没命中就正常构建
pwsh scripts\ci\build-windows.ps1 -DomainCacheDir .muse-domain-cache
```

`build-windows.ps1` 的开关：`-SkipPreflight -SkipRust -SkipDart -SkipFlutterBuild -SkipTests
-SkipZip -SkipVerify -DryRun -ForceRebuild -DomainCacheDir <dir> -Profile debug
-RustTargets a,b`。

历史入口 `scripts\package_windows.ps1` 等价于 `build-windows.ps1`（默认参数）。

> **PowerShell 版本**：CI 用 `shell: pwsh`（PowerShell 7）。脚本同时兼容 Windows 自带的
> Windows PowerShell 5.1，所以本机可以直接 `pwsh` 或 `powershell` 跑；`$IsWindows` 只存在于
> PS 6+，脚本里统一用 `Test-OpenMuseWindowsHost`。

## Runner 要求与时间预算

`windows-2022` 自带 VS 2022 C++ 工具链、Python、Git、`tar`、Node；workflow 再补 Rust
（`dtolnay/rust-toolchain@stable`）和 Flutter 3.44.2（`subosito/flutter-action`）。
`build-windows.ps1` 在本机也可以用同样的步骤跑，只有工具链来源不同。

| 阶段 | 大致耗时（`windows-2022`，实测） |
| --- | --- |
| checkout + Rust + Flutter | ~2.2 min（Flutter `subosito/flutter-action` 约 110 s，Rust 约 7 s，checkout 约 5 s） |
| Bootstrap（`flutter precache --windows`） | 首次 36 s，Flutter 缓存命中后 7 s |
| Domain fingerprints | 5 s |
| rust 域（`cargo test --workspace --locked`） | 冷构建约 1 min；命中后 0 |
| dart 域（Dart 包 + host 的 pub get/analyze/test） | 3-4 min，**每次都会跑** |
| flutter 域（`flutter build windows --release`） | 冷构建约 2-3 min；命中后 0 |
| pack（DSH closure + `hx.exe` + zip + verify） | 首次编译 Helix 约 15 min，增量约 2 min；DSH npm 约 1 min |
| **整个 job** | 含 Helix 冷构建预计 25–40 min（见下表的历史数据是 Helix 入包之前的） |

job `timeout-minutes: 120`；缓存策略见 [`windows-incremental-build.md`](windows-incremental-build.md)。
实测 CI（commit `f39d949`，同一 commit 连续两次 dispatch）：

| 运行 | job 用时 | Build 步骤 | 产物 SHA-256 |
| --- | --- | --- | --- |
| [run 35857369632](https://github.com/openmuseai/muse-clients/actions/runs/35857369632)（冷） | 556 s | 365 s | `bfb30641166f07dd…` |
| [run 35858799593](https://github.com/openmuseai/muse-clients/actions/runs/35858799593)（热） | 417 s | 275 s | `bfb30641166f07dd…` |
| [run 35860044525](https://github.com/openmuseai/muse-clients/actions/runs/35860044525)（`diagnose-windows.yml`） | ~2 min | —（不起构建） | — |

热运行里 `Save rust cache` / `Save flutter cache` 两步是 `skipped`，即缓存确实命中；
两次的产物哈希完全一致，说明"复用"没有改变产物。省下的 90 s 全部来自 rust + flutter 两个域，
`dart` 域（60 多个测试文件）照常执行——这是刻意的，不能把没跑过测试的产物发出去。


## 排障

| 症状 | 原因 / 处理 |
| --- | --- |
| `Visual Studio with the C++ desktop workload was not found` | 跑 `scripts/ci/diagnose-windows-vs.ps1 -CheckBuild`，用 scratch 工程区分"工具链坏了"和"本仓库坏了" |
| `error C2220: the following warning is treated as an error` | Flutter 的 `apply_standard_settings` 给插件加了 `/W4 /WX`，插件里任何 warning 都会变成 error。**在 Windows 上必须真的编过一遍**，无法用 macOS 的编译结果代替 |
| `error C2589: '(': illegal token on right side of '::'` | `<windows.h>` 的 `min`/`max` 宏和 `std::min`/`std::max` 冲突。runner 目标自己定义了 `NOMINMAX`，**插件目标没有**，插件里要么显式比较、要么避免 `std::min/max` |
| Dart 测试只在 Windows 失败 | 检查是不是 `/` 分隔符假设（`path.split('/')`）或者对 Windows 路径做 `contains()`——TOML 里的反斜杠是被转义的 |
| `flutter pub get` 重新解析了依赖 | `pubspec.lock` 里记的是 `https://pub.flutter-io.cn`，runner 上默认是 `pub.dev`，pub 会重新解析并重写 lock。不影响构建正确性，但会让 lock 与仓库里的版本出现 diff；要固定就跑一次然后提交 |
| 产物被判定为"复用"但其实过期 | 域指纹漏了输入。检查 `scripts/ci/lib/domain-plan.ps1` 的 `paths` 和 `toolchain`，并用 `-ForceRebuild` 做对照 |
| `file INSTALL cannot find ".../build/native_assets/windows"` | 手工删了 `build/` 但没删 `.dart_tool/flutter_build/`：Flutter 认为 native assets 目标是最新的而跳过它，MSBuild 却仍按旧的 `cmake_install.cmake` 去装这个目录。要清就 `flutter clean`（它连 `.dart_tool` 一起清），别只删 `build/` |
| 声明的资源目录里有文件没被打进包 | Flutter 的资源**目录**条目只包含该目录的**直接子文件**（`flutter_tools/lib/src/asset.dart` 的 `_parseAssetsFromFolder` 用的是非递归 `listSync()`），嵌套子目录里的文件会被静默丢掉，必须逐条列出来 |
| Windows 包突然大了几十 MB | 同一个原因的另一面：`assets/engines/helix/` 下的 `hx` 是 56 MB 的 macOS 通用二进制，目录条目会把它一起打进 Windows 包（实测归档从 12.15 MB 涨到 31.95 MB）。目录条目也不能带 `platforms` 过滤，所以要过滤就得**逐条列文件**，像 `plugins/helix/pubspec.yaml` 里那样给 `hx` 单独写 `platforms: [macos]` |
| 新增资源文件后没被打进包，但改 pubspec 后又好了 | Flutter 自己的 `flutter_build` 增量判断（`AssetBundle.needsBuild`）可能放过新增的大文件；改了 pubspec 会强制重建清单。怀疑资源清单过期时用 `flutter clean` 而不是只删 `build/` |

拉失败日志：

```powershell
pwsh scripts/ci/remote-build-windows.ps1 -RunId 123456789 -DownloadLogs
# 解压到 tmp/ci-logs/run-<id>/
```

## 已验证 / 未验证

- **已验证（本机，2026-09-27，`openmuseai/openmuse`）**：
  - Flutter 3.44.2 + rustc 1.98.1 + MSVC 14.29：`build-windows.ps1` 产出
    `dist/OpenMuse-windows-x64.zip`，归档 SHA-256
    `33060096a835c10f8d3b715400d1cb1b338985ce7d0264e940271061470ccff7`。
  - 工作区添加走原生文件夹选择器（`#32770`，标题「选择工作区文件夹」）。
  - 随包 Node 拉起 DSH Web UI；WebView2 作为 Flutter 视图的兄弟窗口叠在右侧面板上，
    本机 HTTP 可达。
  - `hx.exe` 随包。`flutter_pty` 在 Windows 上会把可执行文件写进 argv 两次，Helix 必须丢掉
    这个重复项，否则会把 `hx.exe` 当文档打开（状态栏出现 `windows-1252`，编辑区是乱码）。
    启动时把控制台代码页设为 65001。PTY 环境必须带上 `SystemRoot` 和 `Path`。
  - open-file-viewer 在 Windows 上用 `Image.file` 打开栅格图，Markdown/SVG 走文本预览；
    PDF 目前只校验并显示页数，没有嵌入页面渲染器。
- **历史远程数据**（当时仓库是 `openmuseai/muse-clients`，Helix/DSH 尚未随包）：
  `windows-build.yml` 冷
  [35857369632](https://github.com/openmuseai/muse-clients/actions/runs/35857369632)、
  热 [35858799593](https://github.com/openmuseai/muse-clients/actions/runs/35858799593)
  均为 `completed/success`。这些数字不能代表当前 `openmuseai/openmuse` 的产物体积或耗时。
- **未验证**：Windows 安装器（Inno Setup / MSIX）、代码签名、SBOM 与完整第三方 notices；
  Windows 上的 PDF 页面渲染；tree-sitter 语法高亮用的 `.dll`（当前 runtime 仍以 macOS
  `.dylib` 为主，不影响打开文件）；正式 GitHub Windows 出包在本页更新后的下一次
  `windows-build.yml` 运行中验收。


## macOS

macOS 出包是另一条流水线，见 [`macos-ci.md`](macos-ci.md)。`desktop-gates.yml` 的 macOS job
和 `macos-build.yml` 调的是同一个 `scripts/ci/build-macos.sh`。出包需要的 Universal `hx`、
Helix runtime、DSH tarball 和 darwin Node 归档已经在仓库里。
