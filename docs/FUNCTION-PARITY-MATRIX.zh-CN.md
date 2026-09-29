# OpenMuse 本地工作台功能对齐矩阵

更新：2026-09-27。范围仅为本仓库与历史客户端中可证明为自研的功能语义；历史工程中的 UI、账号/云/协作代码不迁入。本表中的“旧”路径是历史客户端源码树内的相对路径，仅作来源记录；“新”路径均相对于仓库根。截图仅用于验收样式，不是实现规范或授权证明。发布边界仍以 `CODE-REUSE-PROVENANCE.zh-CN.md` 为准。

## 功能、实现逻辑与落点

| 功能 | 旧代码位置与真实逻辑 | 新代码位置与边界 | 当前结论 / 验收 |
|---|---|---|---|
| Workspace mount、树、折叠、宽度 | `workspace_platform/application/workspace_controller.dart` 持有 Project Workspace/Mount；`_bindMount` 订阅本地 provider `watch(recursive: true)`，`_scheduleWatchRefresh` 250 ms 去抖刷新可见目录；`workspace/presentation/home/home_stack.dart` 及树行处理展开/选择/宽度 | `app/openmuse_host/lib/src/host/workspace_controller.dart` 持有本地 mount 和文件操作，现以同样的递归监听 + 去抖自动刷新外部文件变更；`workbench_shell.dart` 绘制树和可拖拽分栏；`local_settings.dart` 保存宽度 | 根目录与文件夹分别可折叠，文件夹状态在刷新后保留；新增/删除外部 `.sh` 文件自动刷新测试通过。仍需继续逐像素截图对比、键盘/拖拽门禁，网络文件系统不可监听时保留手动刷新 |
| Tab 外观与动作 | `workspace/presentation/home/tabs/` 内的旧 Tab 实现包含自研交互：激活/悬停关闭、固定宽度、右键动作注册；依赖旧 UI 组件 | `workbench_shell.dart` `_TabStrip` / `_WorkbenchTab` 自研绘制，`WorkspaceTab` 由 Host 管理 | 关闭、固定、关闭其他标签已实现；仍需补拖拽排序与精确 hover/Popup 基线 |
| 资源抽象与编辑器注册 | `plugins/resource_surface/engine_registry.dart`、`engines/{register,ioffice,helix,open_file_viewer}.dart`：格式在引擎内声明，priority iOffice 30 / Helix 20 / Viewer 10，Viewer catch-all；`resource_surface_session.dart` 管理活动会话 | `packages/openmuse_plugin_sdk/lib/openmuse_plugin_sdk.dart` 的 Contribution/Registry；`plugins/{helix,open-file-viewer}` 自声明扩展名；`app/.../local_settings.dart` 持久化默认引擎 | 已移除 Host 格式表；默认引擎优先于优先级，Viewer catch-all 给未知类型说明页；iOffice 尚未通过许可证/运行时门禁，因此菜单禁用，不假装可用。长线应适配已迁入的 `muse_resource_contract`/`muse_surface_orchestrator`，避免双状态机 |
| “打开方式”与默认引擎 | `resource_open_with_menu.dart`：二级菜单列出三个引擎，不支持的灰掉、当前引擎打勾；`resource_open_defaults.dart` 按扩展名持久化；`resource_tab_actions.dart` 注册菜单 | `workbench_shell.dart` `_showOpenWithMenu`，`local_settings.dart` 默认映射，`_EditorArea` 选择默认引擎 | 树/Tab 均只有一项“打开方式”，可向右进入引擎及默认设置；当前实现为点击级联，尚未达到旧版鼠标悬停级联，列为视觉/交互门禁 |
| 版本保存与历史 | `plugins/version_diff/application/text_version_repository.dart`、`text_version_diff_service.dart` 保存 committed / working 状态；`presentation/resource_version_pane.dart` 打开前 flush 编辑器、自动保存脏内容，历史版本可对当前版本或另一版本比较；`version_history_dialog.dart` 审计 | `workspace_controller.dart` `LocalVersionStore`/`openVersionComparison`；`workbench_shell.dart` 版本历史与 Diff | 已能比较两个不同历史 blob，拒绝同一版本自比，Diff Tab ID 含两端身份；**未完成** Helix buffer flush/自动保存、审计图和统一/并排算法与旧版同级验证；二进制不应进文本 Diff |
| Helix PTY、启动、导航、语法、LS | `plugins/resource_surface/helix/{helix_commands,helix_settings,helix_language_servers}.dart`，`surfaces/helix_resource_surface.dart`：PTY、快捷键、tree-sitter 安装/状态、各语言 LS 安装/覆盖/检查、`languages.toml` 生成 | `plugins/helix/lib/src/{helix_runtime,helix_preferences,helix_editor_surface,helix_language_servers}.dart`，`openmuse_helix_plugin.dart` | `.sh` 等格式登记已补齐；不同文件使用独立 PTY、已开文件会话复用。新增 Helix 编辑区右键导航菜单、17 种 LS 条目、可执行文件及配置路径；Rust 与部分 npm 来源可一键安装。Windows 包由 `stage-helix-windows.ps1` 现场编译 `hx.exe` 并随包；`flutter_pty` 重复 argv 已在引擎侧丢掉。跨文件导航后 Host Tab 仍不会跟随，属于发布阻断项。首帧闪烁、安装进度/取消/供应链锁定仍需验证 |
| Viewer PNG/PDF/未知格式 | `plugins/resource_surface/engines/open_file_viewer.dart` + `surfaces/open_file_viewer_resource_surface.dart` | `plugins/open-file-viewer/lib/openmuse_file_viewer.dart` + macOS AppKit native view | macOS 仍走 AppKit；Windows 栅格图用 `Image.file`，Markdown/SVG 走文本预览，PDF 只校验并显示页数，没有嵌入页面渲染器 |
| DSH 本地 sidecar 与模型选择 | `plugins/dsh_agent/{dsh_sidecar,dsh_runtime,dsh_embedded_view}.dart`；旧 sidecar 错误地将 DeepSeek Key 作为启动前提，最新 DSH 浏览器 UI 可以启动后再配置 provider/model | `plugins/dsh-agent/lib/src/{dsh_sidecar,dsh_panel,dsh_web_view}.dart`；Host 只提供 Workspace 快照与受控 openResource command；`scripts/build_dsh_closure.py` 从本仓库 `third_party/dsh` 固定 tarballs/lockfile 组装产品中立的 DSH npm closure，`scripts/package_macos.sh` 直接装入本仓库固定 Node 输入构成的 Universal Node + closure | 已去掉 DeepSeek Key 启动门槛、移除最新版不接受的 `--no-open`、将工作区 Home 正确传给 `DSH_HOME`。面板默认可见且仅设置控制。真实 Release App 已显示最新版 DSH 对话、模型选择器与 Workspace 列表，当前还保留 DSH 自身侧栏，和旧单列面板样式不一致；模型配置交互仍需无 Key 点选门禁 |
| Workspace ↔ DSH 同步 | `plugins/dsh_agent/dsh_workspace_bridge.dart` 发布 binding、locator 与 active Mount intent；旧 DSH 侧由产品专用 Workspace adapter 消费，该插件属隔离清单，不能直接带入 | `plugins/dsh-agent/lib/src/{dsh_workspace_binding,dsh_workspace_sync}.dart` 订阅 Host `hostChanges`，通过新第一方 `openmuse-dsh-bridge` 的带 token HTTP 接口登记 Mount；WKWebView 的 `MuseHostWorkspace` 与 DSH 客户端插件处理当前工作区双向切换 | 最新版 DSH 0.1.7-rc.1 不提供旧 `/api/workspace.*` RPC。新版真实 sidecar 登记、缺失 token 拒绝、Flutter 集成及客户端单元测试通过；真实 WKWebView 双向切换仍待手测。不会删除用户独立创建的 DSH workspace |
| DSH 对话中打开文件 | `plugins/dsh_agent/dsh_embedded_view.dart` 的 `resource.open` bridge，Host 端解路径并进入资源 Tab | 官方 `@deepseek-ai/dsh-client-ui-chat` 默认调用 `sidebarRight.openResource`，所以文件在 DSH 自己的预览区打开；新第一方 `openmuse-dsh-bridge` 仅在嵌入 Host 时接管文件地址，走 `MuseHostResource` → Swift/Dart → Host `workspace.openResource` | 客户端模块图、桥单元测试、Host 路径边界测试通过；真实会话中点击文件尚未端到端验证，不能称已完全修复；Windows WebView 尚缺 |
| 设置、主题与 DSH 可见性 | 旧设置页混有旧产品账号/云/AI；独立 Helix/Agent 项可复用其业务语义 | `app/openmuse_host/lib/src/host/{settings_dialog,local_settings,design_system}.dart`，各 Plugin 的 `buildSettings` | 本地设置/主题/插件/Agent 分离，右侧 DSH 默认开启、取消工作台额外收起按钮；需进一步对齐字体、间距、弹层阴影与 dark/light token，并检查无旧品牌资产 |

## 实施顺序与独立门禁

### 关键调用链与未迁移边界

- 旧 Workspace 动态链：`workspace_platform/application/workspace_controller.dart` 的 `mountLocalDirectory` → `_bindMount` → `local_workspace_provider.dart` 的 `watch(recursive: true)` → `_scheduleWatchRefresh` → `_loadChildren(force: true)`。新 Host 对应 `addWorkspace` → `_watchMount` → `_scheduleWatchRefresh` → `refreshDirectory`。新实现只处理本地目录，暂不复制旧 Provider 多后端模型；网络目录不支持通知时仍可右键手动刷新。
- 旧 Tab 动作链：历史客户端 `workspace/presentation/home/tabs/` 中的 Tab 组件 → `TabMenu._pluginEntries` → `PluginTabMenuContributor.tabMenuActions`，动态注册插件动作、关闭/关闭其他/Pin。新 Host `workbench_shell.dart` 的 `_WorkbenchTab` / `_showTabMenu` 通过 SDK Registry 得到引擎列表，关闭/Pin 已落地；旧独立 Tab Contributor 与拖拽排序尚缺。迁移依据是独立引入的业务逻辑，不复制旧 UI 依赖。
- 旧版本动作链：`resource_version_pane.dart` 的 `prepareMenu`、`_stashUncommitted` 会先 `MuseResourceSurfaceSession.flush(file)` 再 `saveIfDirty`；`text_version_diff_service.dart` 的 `compareVersions` 用两个不可变版本内容计算 diff；`version_history_dialog.dart` 同时展示审计事件与图。新 `LocalVersionStore` 已存不可变 blob，`openVersionComparison` 明确接收 before/after 两版；但缺失编辑器 buffer flush、自动脏版本与审计事件，不可将当前简化历史弹窗说成全功能迁移。
- 旧 Helix LS 链：`helix_language_servers.dart` 的 `refresh` / `_statusFor`、`setOverride`、`install`、`buildGrammars`、`writeLanguagesToml` 包括 SDK、GitHub、npm、go 与 C 编译器流程。新插件目前仅 `inspectHelixLanguageServers`、手动绝对路径覆盖与 TOML 生成。自动下载/编译属于独立发布门禁，需版本钉住、校验和、取消/进度与无网错误测试后实现；不能把旧安装器的网络副作用直接搬入 Host。
- 旧 DSH 资源链：`plugins/dsh_agent/dsh_embedded_view.dart` 从对话事件得到资源请求，再进入 Host；新版 `dsh-client-ui-chat` 默认在 DSH 自己的右侧预览。新客户端插件只在嵌入式 Host 桥存在时转发文件地址。Native WKWebView 只接受当前 loopback 端口的主 frame，Dart 解包后由 Host `openHostResource` 检查 canonical 路径在授权 Mount 内；还缺真实会话点击与当前会话工作区切换的端到端测试。

1. 资源路由：插件自登记格式 → SDK 稳定优先级 → 扩展名默认偏好 → 未知类型 Viewer 说明页。测试 `.sh/.md/.png/.pdf/.docx/未知`，插件卸载后偏好要回退。当前菜单操作路径已打通，但级联 hover 和 iOffice 仍未过门禁。
2. 版本：保存历史 A、B，修改工作副本 C；分别比较 A↔B、A↔C；拒绝 A↔A；同一文件重复打开 Diff 不出现旧内容。后续接入 Helix buffer flush/脏状态与真正审计记录。
3. DSH：构建固定版本、带许可证/Notices 的最小 Node closure，随 macOS/Windows 安装包分发；不需要模型 Key 启动，进入 UI 后配置 provider/model；验证 binding receipt、active Mount 切换、对话文件打开 Host Tab。
4. Helix：按旧 `helix_language_servers.dart` 将 status、install、override、语法编译/安装拆为插件服务；对 `hx --health`/PTY 参数/首帧提示做录屏门禁；验证 F12/Shift-F12、`.sh` 与多个文件切换。
5. 视觉：固定窗口尺寸下叠图比对 sidebar、tab、popup card、Diff、Settings；所有字体/间距/颜色从新工程 token 管理，绝不复制旧产品受限 UI/品牌资产。

## 本轮验证记录

本轮运行：Host `flutter analyze` 与 `flutter test` 通过；SDK 与 DSH 插件分析/测试通过；macOS 真实 Helix PTY、Native Viewer 平台视图、无 Key 的最新版 DSH CLI 集成门禁通过；DSH workspace catalog 的真实无 Key RPC 集成测试通过。`scripts/package_macos.sh` 重新完成并输出含 DSH 0.1.0-rc.7 与 Universal Node 22.19.0 的 `dist/OpenMuse-macos.zip`（约 231 MB，SHA256 `4d7b7657e0d96e9b2c692484917f71b5876f12bff41e812ab479f546e918ece6`）；脚本验证了 Flutter 测试、Universal Helix/Node、DSH `resource.open`/Workspace RPC 静态合同、代码签名与归档。包内 DSH 在无 Key、隔离 Home 下返回 HTTP 200；签名深度校验与 ZIP 完整性通过，DSH 生成 JavaScript 中不再含旧仓库路径/品牌串。右侧 DSH 刷新控件已连接到嵌入式 WebView 真正的 `reload()`，经 Swift Release 构建验证。新增的递归 Workspace 文件监听与外部 `.sh` 新增/删除自动刷新测试通过。**此包还不是完整产品发行包**：DSH binding receipt/当前工作区双向选择、对话文件打开、LS 自动安装、Windows、正式签名/公证和许可证/SBOM 全量审核仍未通过。Flutter 测试启动器仍提示 `Failed to foreground app; open returned 1`，但原生测试过程与断言通过，需在签名/公证后的发行包上复验。直接 `pnpm deploy --prod --legacy` 得到的 292 MB 产物启动时缺少 `@deepseek-ai/cosmokit`，因此改用上游发布 tarball 的 npm closure 构建。完成的代码路径不代表全产品对齐；上表明确标为“未完成”的项必须保持独立发布门禁。

实机界面复查（1360 px 窗口）：OpenMuse 左侧 Workspace、中央多 Tab 与右侧真实 DSH 均已出现，DSH 中能看到 Host 登记的 `helix` 工作区。但当前 DSH 前端自身以约 280 px 侧栏 + 对话区 + 可选详情区渲染，嵌入右侧面板时对话区明显过窄；这不是 Host 分栏宽度单独能修复的。后续应做独立 DSH Client 嵌入布局适配并验证设置入口不被隐藏，不应仅通过 CSS 隐藏功能。当前 DSH UI 选中项仍可能是历史 `DSH_HOME` 中保存的工作区（复查时为 `skills`），Host active Mount 与 DSH 当前会话/选择的双向同步仍缺。

### DSH closure 重现

独立仓库直接运行 `python3 scripts/build_dsh_closure.py --out target/dsh-closure`：脚本从本仓库 `third_party/dsh` 的发布 tarballs 与 lockfile 执行 `npm ci`，拒绝旧产品包名或路径穿越，清理生成 CSS region 注释中的旧机器路径，并验证 CLI、Host 文件打开桥接及 Workspace RPC 合同；不再读取旧仓库源码树。`scripts/stage_node_macos.sh` 用本仓库两份官方 Node 压缩包与 `SHASUMS256.txt` 组成 Universal Node，`scripts/stage_helix.sh` 检查本仓库内固定的 Helix 二进制/runtime。直接执行 `scripts/package_macos.sh` 即完成组装，无须任何外部仓库路径环境变量。npm 公共 registry 依赖仍按已提交 lockfile 拉取；这不是完全离线构建。当前 closure 中有 25 个只指向自身 `node_modules` 的 `.bin` 内部相对链接，Windows 包须另做平台特定验证。

### DSH 0.1.7-rc.1 升级记录

2026-09-23 已将固定版本从 `0.1.0-rc.7` 更新到官方 npm `next` 的 `0.1.7-rc.1`，不再用旧版 DSH tarballs 作为构建输入。`third_party/dsh/package-lock.json` 固定 registry integrity；本仓库的 MIT `dsh-model-capabilities` 适配副本（`0.5.0-openmuse.1`）通过本地 tarball 与 `--patch` 默认挂载。新的 `scripts/test_dsh_runtime.py` 在无模型 Key、隔离 Home 下验证 Web HTTP 200、客户端模块图及插件设置路由。重新打包的 `dist/OpenMuse-macos.zip` 为 296,625,585 bytes，SHA256 `9e9ae4e51c87fb2ca3efff825817c93b5b01854d93706fe0801db4a70d80619b`；Host `flutter analyze`、33 项测试、签名深度校验、ZIP 完整性和包内无 Key DSH 测试通过。以上仅证明新版 DSH 基础运行与默认插件加载。旧 Workspace HTTP RPC 在新版返回 404，对话文件打开转为 DSH 自身侧栏；Host 双向工作区同步、对话点击 Host Tab 与模型设置真实写入尚未通过，继续作为发布阻断项，见 `DSH-PLUGIN-HOST-MIGRATION.zh-CN.md`。

### 2026-09-24 增量

- Host 新增 Workspace → DSH catalog：第一方 `openmuse-dsh-bridge` 用私有 token 保护本机 HTTP 路由，在 DSH 0.1.7-rc.1 实际 sidecar 中对授权目录幂等登记；Flutter 集成测试与无 Key/拒绝无 token 的 Node 冒烟通过。当前选中项的 Host↔DSH 消息桥和 Host 精确授权校验已接入，真实 WebView 双向切换仍待验收。
- 对话文件链接：明确由官方 `@deepseek-ai/dsh-client-ui-chat` 调用 `sidebarRight.openResource`，不是模型能力插件。新的客户端桥把文件地址交给 Host 现有 `resource.open` 通道；Node 客户端测试通过，但真实 DSH 对话点击尚未验收。
- Helix：新增编辑区右键导航菜单、LS 安装入口、可执行文件/配置文件路径、常见语言目录，Workspace 文件夹图标改为灰色。跨文件导航后 Host Tab 跟随仍缺编辑器主动文件切换事件，未宣称修复。
- LS 一键安装目前只对有明确来源的 rustup / 精确直接版本 npm 包开放。npm 传递依赖锁定、下载校验、取消/进度、Windows 及 Finder 启动环境下的包管理器可用性还未过发布门禁；目录列出的 Java/Lua/C# 等支持本机路径配置，但不伪装成已验证的一键安装。
- DSH 客户端桥 `0.1.3` 新增点击已有会话时的 active Workspace 通知，并在工作区切换完成后才通知 Host；单元测试通过。最终 macOS 包 `dist/OpenMuse-macos.zip` 为 296,658,842 bytes，SHA256 `d5dea548a01c9bede65094118b6d3d10cbffc4d69bc89dccd47b4159171c2e48`。Host 33 项测试通过、签名深度校验与 ZIP 完整性通过，包内 DSH 0.1.7-rc.1 的无 Key 启动及默认插件 HTTP 测试通过。**上述证据未覆盖真实 DSH 对话点击、WK 工作区双向切换、跨文件 Helix Tab 跟随或一键安装实际下载；不能作为完整发行验收。**

### 2026-09-24 复制、Viewer 与图标增量

- `openmuse-dsh-bridge` `0.1.5` 在嵌入式 WKWebView 中仅对非输入框的真实文字选区接管 Cmd/Ctrl+C，走同源主 frame 的原生剪贴板桥；选区右键出现“复制选中内容”，识别为绝对/相对文件路径或文件名时再出现“复制路径”。无选区和可编辑输入框保留 WebKit/DSH 默认行为。JS 单元测试覆盖这些分支，Swift Release 编译通过；真实对话选区、右键和剪贴板仍需实机 UI 门禁。
- Open File Viewer 原本只登记图片/PDF，`.md` 落到 catch-all 的无预览器提示。现新增 Markdown 只读渲染贡献，限定 16 MB，本地异步读取，可选择复制文字；远程图片与外部链接不自动打开。插件的路由、磁盘读取、渲染测试通过。
- macOS AppIcon 已替换为本仓库自研的 OpenMuse 星形图标，生成源在 `scripts/generate_macos_icon.swift`，从最终 `OpenMuse.app/Contents/Resources/AppIcon.icns` 回读验证不是 Flutter 默认图标。
- Helix LS 设置改为先选择具体语言，再显示该语言的可用“一键安装”按钮；无已核实安装源的语言只保留本机路径配置。设置组件测试通过。
- 现有 `vscodeKeymap` 只是仍处于 Helix 模态状态的兼容快捷键，不再标为“VS Code 模式”。真正非模态必须修改引擎输入状态机和撤销模型，详见 `HELIX-NONMODAL-EDITOR-DESIGN.zh-CN.md`；这是未完成的工程门禁，不把按键模拟当修复。
- 非模态第一阶段已在本仓库维护的 Helix 源码中实现常驻文本输入与空闲撤销检查点，源码、macOS 双架构二进制及测试均可本地重建。发布 UI 仍禁用此实验模式；Cmd/剪贴板、IME、结构化 Host 事件与跨平台端到端测试尚未过门禁。
- 最终 macOS ZIP 为 297,686,440 bytes，SHA256 `a2d93c772456000c59b2ede05819bae57babe296d9c61fe734c4470daf251d88`。Host 33 项测试、Helix 3 项、Viewer 3 项、DSH 5 项及 JS 桥测试通过；签名深度校验、ZIP 完整性、包内无 Key DSH 启动测试通过。WKWebView 文档初始化时即安装复制监听，Swift Release 编译通过。上述并不等于真实 DSH UI 剪贴板或真正非模态编辑通过验收。

### 2026-09-24 Helix 非模态引擎第一阶段

- `third_party/helix` 固定上游 Helix `079a789e` 源码并在引擎内实现版本化 `input-profile`、非模态首次输入、Esc 不进入 Normal/Select、空闲/粘贴撤销检查点。旧 `vscodeKeymap` 不会自动升级。macOS 双架构 `hx` 从本仓库源码重建，版本标记为 `openmuse-nonmodal.2`；其独立本地 socket 已输出活动文件/dirty/save 事件，Host 仅在工作区路径授权成功后跟随活动文件并指定 Helix Tab。真实 LSP 跨文件、Cmd/剪贴板/IME 与双向 buffer flush 门禁未完成，发布 UI 仍禁用实验 profile。
- 13 个测试专用 macOS arm64 tree-sitter 语法库与许可证已收入本仓库，不进入产品包。Helix term/view 库测试 15 + 68 项、完整终端集成测试 178 项、Helix 插件 6 项、Host 33 项均通过；`cargo fmt --check`、macOS Release 构建、深度签名与 ZIP 完整性校验通过。
- 该阶段 macOS ZIP 的 SHA-256 为 `f36bcaa58a27e9f48e806fd6739ac8999da9f2c4db9e604c75d41c9c00e03e7f`；已被后续构建替换。

### 2026-09-27 Windows 宿主、DSH 与 Helix

- 添加工作区走原生文件夹选择器，不再手输路径。
- DSH sidecar 用随包 Node 启动，`--patch` + `OPENMUSE_DSH_BRIDGE_TOKEN` + closure 工作目录一并传入；WebView2 挂在顶层窗口并叠在 Flutter 视图之上。
- Helix：`hx.exe` 与 runtime 打进 Windows 归档；PTY 转发完整 Windows 环境；引擎忽略 `flutter_pty` 重复的可执行文件参数，并把控制台代码页设为 UTF-8。
- Viewer：Windows 栅格图可预览，Markdown/SVG 可读，PDF 尚无页面渲染。
- 出包入口仍是 `scripts/ci/build-windows.ps1`。GitHub 仓库是 `openmuseai/openmuse`，`windows-build.yml` 仅 `workflow_dispatch` 与 `v*` tag。

### 2026-09-26 Helix 非模态引擎第三阶段

- `openmuse-nonmodal.3` 双向控制通道加入请求 ID、目标路径、预期 revision 和明确结果；重复请求/错误目标/过期 revision 均被引擎拒绝。`save`/`flush` 走引擎现有格式化与写入队列并等待落盘 ACK，`undo`/`redo`/`find`/`select_all` 由引擎语义命令直接执行。Host 的版本快照和工作副本比较经插件能力接口先 flush，失败则不创建可能过期的差异。非模态终端的 Cmd/Ctrl+S、F、Z、A 经 Flutter 键盘事件转为上述命令，不注入 PTY 字符。
- Helix term/view 单元测试 17 + 68 项、终端集成测试 178 项、真实 PTY 控制通道冒烟测试、Helix 插件 10 项、Host 34 项（另 1 项跳过）、SDK 2 项均通过；Helix 子工程 `cargo fmt --check`、各 Flutter 工程 `analyze`、`git diff --check` 通过。
- 该阶段 `dist/OpenMuse-macos.zip` SHA-256 为 `5c023ba0198e4bddf10f963473bb0ebb83454a5ad83aae544c9d00c40afc95af`；已被后续构建替换。

### 2026-09-28 输入模式切换第四阶段

- 插件设置现显示「Vim 模式」与「VS Code 模式」，由引擎 `openmuse-nonmodal.4` 能力检测决定第二项是否可选，不再依赖开发环境开关。活动会话切换先走认证 `prepare_switch`，保存并检查进程内所有 buffer，随后重启并恢复原有文件会话；失败时尝试恢复原模式。macOS 原生 PTY 测试覆盖两个脏文件的双向切换与落盘。
- 非模态 Cmd/Ctrl+C/X/V 通过 Flutter 系统剪贴板与引擎语义通道处理；复制只接受真实选区，剪切再核对源文件、revision 与文字，粘贴替换选区。真实 PTY 测试覆盖成功路径和选区不符的拒绝路径。合成中文 IME 测试覆盖“组合期间不输出、提交后只输出一次”。
- macOS 打包流程在 Release App 构建后强制运行 Helix 插件分析、测试及原生 PTY 切换门禁。**尚不能宣布跨平台可发布**：真实 macOS GUI 输入法/快捷键焦点、LSP 跨文件回归、Windows 包与实机输入/剪贴板仍待验收。
- 本轮 macOS ZIP：`dist/OpenMuse-macos.zip`，SHA-256 `38009545aed75198975351e4c2e9a2952e87afa52e8b2a32201f183b39835396`；Host 60 项、Helix 插件 14 项通过（Windows PTY 项在 macOS 跳过），Helix term/view 17 + 68 项与 178 项终端集成测试通过，ad-hoc 签名深度校验和 ZIP 完整性通过。Windows 打包脚本已加入随包原生双向切换测试并通过 PowerShell 语法检查，仍需 Windows runner 的实际结果。macOS 包尚无 Developer ID/公证，`spctl --assess` 拒绝，**不是可公开分发的签名包**。
