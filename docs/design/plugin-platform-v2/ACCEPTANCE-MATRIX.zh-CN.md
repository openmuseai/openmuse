# Mobile 与 Desktop 插件平台验收矩阵

更新：2026-10-16。基线见 [设计稿](DESIGN-SPEC.zh-CN.md)、[实施方案](IMPLEMENTATION-PLAN.zh-CN.md) 与 [WebView 包规范](MOBILE-WEBVIEW-PACKAGE.zh-CN.md)。「通过」表示所列验证范围内通过；真实帐号与设备联调另列为发布门槛。

| 编号 | 类型 | 结果 | 证据与边界 |
| --- | --- | --- | --- |
| A1 | Everything is Plugin | 通过 | 预置定时任务、Workspace 文件及下载式 WebView 插件均经 SDK 贡献接口进入同一 registry；Shell 不按插件 ID 决定卡片 UI。 |
| A2 | A2UI 组件安全 | 通过 | `MobileWidgetCatalog` 统一校验版本、消息、组件及尺寸。下载包的卡片经白名单后用 Flutter 渲染；非法组件、权限、entrypoint 和摘要被拒绝。 |
| A3 | WebView 隔离 | 范围内通过 | Android 16 真机此前完成预置插件页面/消息桥/外域导航测试；Android 14 独立模拟器及 iPhone 17 Pro 模拟器均完成下载式 WebView 插件的渲染、JS 交互、主题注入和 Host 调用拒绝。最新版已覆盖安装到 Android 16 真机；下载式 WebView 的真机 JS 桥尚未复验。 |
| A4 | 市场包与生命周期 | 通过 | `openmuse-mobile-web/v1`、签名远端目录与同源 artifact、SHA-256、manifest V2、平台/权限校验、安装、更新、停用、移除、重启恢复均通过测试。任务看板、用量与预算示例已迁移；PPTX 保留 QuickJS 兼容。示例包随 App 内置，未上传到公开市场服务器。 |
| A5 | 配对帐号隔离 | 通过 | 修复旧 grant 在帐号切换后访问代理、镜像和远端页面的路径；网关回归与 Mobile→Desktop 领域端到端测试通过。 |
| F1 | 工作台 | 通过 | Flutter/A2UI 卡片、详情 WebView、置顶、停用占位和管理入口、拖动排序、用户覆盖高度/列宽、取消与重建恢复通过 Widget 测试。 |
| F2 | 定时任务 | 范围内通过 | 真实 Mobile 插件→配对 HTTP 网关→Desktop 领域服务→模拟 DSH HTTP 的端到端测试覆盖创建、到期仅发送一次、结果记录及服务重建恢复。真实 Desktop 与用户帐号联机发送尚未验证；Desktop 关闭时只能启动后补执行。 |
| F3 | Workspace 文件 | 范围内通过 | 同一跨端链路覆盖挂载、子目录、文件名搜索、文本预览和绝对路径不泄漏；Android 真机此前读到已配对 Desktop 的真实挂载。真实设备上的子目录、搜索、图片预览和断连提示尚未复验。 |
| F4 | 登录保持 | 部分通过 | 安全存储及刷新失败保留会话测试通过；Android 16 真机使用相同签名的 APK 执行 `adb install -r --no-streaming`，原浅色偏好保留，切深色后强制关闭重启仍为深色，随后已恢复浅色。当前设备未验证已登录会话持续；此前 Flutter 集成测试工具卸载 App 清除了本地凭据，不能算卸载重装免登录通过。 |
| F5 | 导航与账户 | 通过 | 底栏为任务/工作台/发现/我的；侧栏市场与帐号入口移除；帐号、退出、主题和插件管理位于「我的」。Widget 测试与 Android 16 真机 UI 层级检查通过。 |
| U1 | 主题与发现 | 通过 | 发现页及详情的浅/深主题、搜索、类别、平台标识和无权限提示通过 Widget 测试；Android 14 模拟器与 Android 16 真机从「我的」切深色后「发现」保持深色，真机重启后偏好仍在；下载式 WebView 在 Android 模拟器中随主题切换并拒绝未授权 Host 调用；iOS 下载式页面跟随主题此前通过模拟器测试。 |
| U2 | 布局与可访问性 | 范围内通过 | 390dp 双列、320dp 与 1.5 倍字号、拖动、置顶、滚动与读屏语义通过 Widget 测试。Android 16 真机确认工作台卡片、编辑入口、尺寸和 pin 控件可见且标题完整；读屏与真实拖动手势仍待人工验收。 |
| U3 | Desktop 市场 | 范围内通过 | 当前 Debug 构建已在 macOS 前台应用走通「匿名模式→工作区→插件→市场」，验证分类、搜索、详情、我的插件整页滚动及关闭返回；修复了已安装卡片被截断。宽/窄窗 Widget 测试覆盖布局、⌘K、详情与卡片位置；隔离的 macOS 原生集成测试完成 Archify 安装与禁用/启用，并核对 DSH profile、skill 与组合配置。当前市场目录仅有一个已验证商品，窗口手动缩放尚未复核，差异见 [UI 对照](UI-AUDIT.zh-CN.md)。 |

## 执行记录

- `packages/openmuse_mobile_plugin_runtime`: `flutter analyze --no-pub`、`flutter test --no-pub`，含签名 HTTP 目录到下载式 WebView/A2UI 包安装和篡改拒绝。
- `app/openmuse_mobile`: `flutter analyze --no-pub`；工作台、市场、Shell、配对插件等 Widget 与 HTTP 测试通过。
- `app/openmuse_mobile`: iPhone 17 Pro / iOS 26.5 与 Android 14 / arm64 独立模拟器分别运行 `integration_test/mobile_webview_plugin_e2e_test.dart`，两端各 2/2 通过；包含下载式 WebView 页面。
- `app/openmuse_mobile`: Android 16 / PKM110 真机比较新旧 APK 签名指纹后以 `adb install -r --no-streaming` 覆盖安装成功，未卸载现有 App；核对四项底栏、发现/我的插件分区、工作台编辑控件、主题切换及重启持久化，并恢复原浅色偏好。预置插件 WebView 的 JS 桥测试来自此前执行。
- `tests/plugin-platform-e2e`: `flutter test --no-pub test/mobile_desktop_domain_e2e_test.dart` 通过，使用真实 Mobile、配对网关和 Desktop 领域服务，DSH HTTP 端点为测试替身。
- `plugins/workspace-paired`: `flutter test --no-pub` 全部通过，含帐号切换后旧 grant 的代理/镜像/远端页面回归。
- `app/openmuse_host`: 定时任务和 Workspace 镜像测试通过；`flutter build macos --debug --no-pub` 成功。
- `plugins/plugin-market`: 580px / 1100px 布局及市场操作 Widget 测试通过。
- `app/openmuse_mobile`: Android arm64 Debug APK 构建成功。真机此前的工作台视觉记录：[Android 工作台截图](acceptance-android-home.png)。
- `app/openmuse_mobile`: Android 14 / arm64 隔离模拟器运行 `integration_test/mobile_plugin_market_e2e_test.dart`、`mobile_webview_plugin_e2e_test.dart` 与 `mobile_shell_theme_e2e_test.dart`，覆盖市场安装、WebView 交互与隔离、主题跨页和工作台卡片。
- `app/openmuse_host`: macOS 原生运行 `integration_test/plugin_market_ui_e2e_test.dart`，覆盖全页市场入口、侧栏搜索与分类、精选按钮安装、我的插件禁用/启用、DSH profile 与 skill 配置。使用打包运行时中的 DSH、Node 和 pnpm。渲染证据：[发现](acceptance-desktop-discover-current.png)、[我的插件](acceptance-desktop-discover-current-installed.png)。
- `app/openmuse_host`: `flutter build macos --debug --no-pub` 构建成功；打开该构建的实际 OpenMuse 窗口，通过匿名模式进入工作区，点击左下「插件」进入市场，操作分类、搜索、详情和「我的插件」，确认完整卡片与连续滚动，关闭后回到工作区。实际用户配置中 Archify 已安装且启用，未改动该状态。
- `app/openmuse_mobile`: Android 14 模拟器中的正式应用目视截图：[发现浅色](acceptance-android-discover-current.png)、[发现深色](acceptance-android-discover-dark-current.png)、[我的深色](acceptance-android-my-dark-current.png)、[工作台深色](acceptance-android-workbench-dark-current.png)、[工作台编辑深色](acceptance-android-workbench-edit-dark-current.png)。覆盖安装后深色偏好仍在；当前未将此结论扩展到卸载重装。

## 尚未通过的发布门槛

1. 在真实 Desktop 与 Mobile 同一帐号配对环境中，创建短期计划并核对指定 Workspace 实际收到一次消息；测试断连与重启恢复。
2. 在真机浏览真实 Desktop Workspace 的子目录，执行搜索、文本与图片预览，并核对撤销授权后的反馈。
3. 将签名 WebView 示例包发布到实际市场源，从网络下载安装、更新、停用和移除；当前只是内置目录与本地签名 HTTP 测试源。
4. 最新版 Android APK 已在用户真机安全覆盖安装并完成导航与浅/深色目视检查；读屏、动态字号、真实拖动手势和下载式 WebView JS 桥仍待真机人工复核。不得用 Flutter 集成测试命令直接覆盖当前用户设备：该工具曾先卸载 App 并清除本地登录数据。
5. 第三方下载包目前没有 Host 业务权限。若要让第三方插件调用定时任务或文件能力，还需能力声明、逐项授权、撤销和审计；公开市场发布还需单独审核平台政策。
