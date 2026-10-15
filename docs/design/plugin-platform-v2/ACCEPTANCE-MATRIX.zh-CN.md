# Mobile 插件工作台首阶段验收矩阵

2026-10-14。基线见 [设计稿](DESIGN-SPEC.zh-CN.md) 与 [实施方案](IMPLEMENTATION-PLAN.zh-CN.md)。

| 编号 | 类型 | 结果 | 证据与边界 |
| --- | --- | --- | --- |
| A1 | 架构 | 通过 | 两个首批插件均实现 SDK 可选贡献接口，经 `OpenMusePluginRegistry` 注册后自动出现于工作台；市场「发现 / 我的」读取同一 registry。 |
| A2 | 组件安全 | 通过 | 集中式 `MobileWidgetCatalog` 限定 A2UI 版本、组件、消息和字节数；越界单元测试通过。 |
| A3 | WebView 隔离 | 通过 | Android 16 真机与 iPhone 17 Pro 模拟器集成测试均验证详情渲染、插件作用域消息桥和外域导航拦截。 |
| F1 | 工作台 | 通过 | 双卡片、置顶持久化和停用后隐藏的 Widget 测试通过；详情页使用平台 WebView。 |
| F2 | 定时任务 | 部分通过 | Desktop 持久化、取消、到期执行、异常中断不自动重发、配对 grant 校验及假 DSH 服务调用测试通过；Mobile 插件到配对 HTTP 网关的创建请求端到端测试通过。真实 Desktop + Mobile 联机到期发送尚待新 Desktop 构建与账户环境联调。Desktop 关闭时不能准时触发，只能在再次启动后补执行。 |
| F3 | Workspace 文件 | 部分通过 | Desktop 镜像的挂载、目录分页、文件名搜索、opaque ref、无绝对路径泄露测试通过；Mobile 插件经配对网关完成目录、搜索、文本预览的端到端测试。Android 真机通过已配对 Desktop 读取到三个真实挂载目录；子目录、搜索及内容预览的真机操作待联调。 |
| F4 | 登录保持 | 部分通过 | Mobile 安全存储写后回读、刷新失败保留已有会话的单元测试通过；Android 真机 `adb install -r` 覆盖安装后观察到已登录账户。卸载重装会清除本地凭据，尚无云端免密恢复方案。 |
| F5 | 导航与账户 | 通过 | 底栏为任务/工作台/发现/我的；侧栏市场与账户入口移除；账户、退出、主题和插件管理入口位于「我的」。相关 Widget 测试通过。 |
| F6 | 市场新格式 | 待实施 | 两个首批插件属于随 App 发布的预置插件，市场可展示并打开。市场下载包仍采用 QuickJS/A2UI v1；WebView 插件签名包格式、动态安装及通用能力授权尚未交付。 |
| U1 | 浅/深主题 | 部分通过 | 真机观察工作台与「我的」随主题切换，市场不再强制浅色；窄屏主题切换 Widget 测试通过。iOS 与大字号/读屏矩阵待测。 |
| U2 | 卡片布局 | 部分通过 | 390dp 双列、320dp 单列、内容滚动、置顶与禁用通过 Widget 测试，Android 真机工作台已目视检查；拖动排序已有实现，尚无真机手势和重启恢复自动测试。 |
| U3 | Desktop 市场 | 通过 | 宽窗类别在左、搜索在右；窄窗自动换行。580px 与 1100px 的 Widget 测试核对布局位置、搜索、分类、详情、安装及启停。 |

## 执行记录

- `app/openmuse_mobile`: `flutter analyze --no-pub`；`flutter test --no-pub test/mobile_workbench_page_test.dart test/workbuddy_shell_test.dart test/mobile_app_test.dart`。
- `app/openmuse_mobile`: `flutter test --no-pub test/mobile_paired_plugins_test.dart`，真实 HTTP 配对握手与两个插件能力路由通过。
- `app/openmuse_mobile`: Android 16 / PKM110，`flutter test --no-pub -d 3B65BB01B4H00000 integration_test/mobile_webview_plugin_e2e_test.dart`。
- `app/openmuse_mobile`: iPhone 17 Pro / iOS 26.5 模拟器，同一 WebView 集成测试通过。
- `app/openmuse_host`: `flutter analyze --no-pub`；`flutter test --no-pub test/scheduled_task_service_test.dart test/workspace_mirror_service_test.dart`。
- `plugins/workspace-paired`: `flutter test --no-pub`；定时任务路由的 grant 与账户切换测试。
- `plugins/auth-gotrue`: `flutter test --no-pub test/auth_controller_test.dart`。
- `packages/openmuse_mobile_plugin_runtime`: `flutter analyze --no-pub`；`flutter test --no-pub test/mobile_widget_catalog_test.dart`。
- `plugins/plugin-market`: `flutter analyze --no-pub`；`flutter test --no-pub`，包括 580px / 1100px 布局与安装启停。
- `app/openmuse_host`: `flutter build macos --debug --no-pub` 构建成功。
- `app/openmuse_mobile`: `flutter build apk --debug --no-pub --target-platform android-arm64` 构建成功，`adb install -r` 覆盖安装到 PKM110 成功，`pm path` 确认安装。
- Android 真机浅色工作台目视记录：[工作台截图](acceptance-android-home.png)。账户页截图包含个人邮箱，因此未保存到仓库。

## 尚需联调的发布门槛

1. 用包含本次 Desktop Host 改动的新构建登录，与已更新的 Mobile 配对；创建一分钟后的定时任务，验证指定 Workspace 收到一次消息、任务状态和重启恢复。
2. 在同一配对下从 Mobile 浏览真实 Desktop Workspace，验证子目录、文件名搜索、文本与图片预览以及断连/撤销权限后的错误状态。
3. iOS WKWebView、深浅主题、文字放大、读屏与拖动排序真机检查。
4. 若要开放第三方动态安装，先定义签名 WebView 包与权限能力清单，再将市场下载安装从 QuickJS v1 迁移；此阶段不得把任意下载的 HTML/JS 当作受信 Host 代码。
