# Mobile 插件工作台实施方案

状态：首阶段已实施；验收结果见 [验收矩阵](ACCEPTANCE-MATRIX.zh-CN.md)。以 [设计稿](DESIGN-SPEC.zh-CN.md) 为 UI 基准。

## 领域边界

| 领域 | 聚合/接口 | 责任 |
| --- | --- | --- |
| Plugin Catalog | `OpenMusePluginRegistry`、manifest V2 | 插件身份、生命周期、平台兼容性、能力声明与启停。Host 不按插件 ID 写 UI 分支。 |
| Widget Presentation | 共享 Mobile A2UI 组件目录与受限 renderer | 插件声明预览树，Host 验证组件与尺寸并渲染 Flutter 卡片；布局/pin 由用户和 Host 持有。 |
| Detail Presentation | `WebView` 文档和按插件作用域分发的消息桥 | 插件拥有详情界面，Host 掌握导航、网络、权限和资源能力；不能把任意 Host RPC 暴露给网页。 |
| Scheduled Task | Desktop 持久化任务与执行服务 | 计划、运行记录由执行端持有；Mobile 只编辑。Desktop 运行时按时触发，关闭期间积压的任务在下次启动后执行。执行中断时状态变为 `unknown`，不自动重发。 |
| Workspace Mirror | 现有配对授权及 opaque ref | Desktop 限定已授权 workspace、分页列目录、搜索及读取预览；Mobile 永远看不到绝对路径。 |
| Identity/Preferences | GoTrue session、Host ThemeMode | 登录与刷新令牌存安全存储；主题为整个 App 的单一状态，市场不得强制浅色。 |

## 运行模型

`Plugin descriptor → Host registry → workbench contribution → 共享 A2UI profile → Flutter Widget 卡片`。点卡片时打开同一插件的 `WebView` 详情。网页通过 `OpenMusePlugin` JS channel 发带 requestId 的能力请求；Host 将请求分发到当前插件的受限实现，Desktop 操作还须有当前配对 grant。两个首批插件为随 App 发布的预置插件，在市场「发现 / 我的」显示。QuickJS 下载包仅保留迁移期兼容；任意第三方 WebView 插件的签名包安装及通用能力授权尚未实现，不能据此宣称 QuickJS 已完全废弃。

```mermaid
flowchart LR
  Catalog[市场目录 / 预置插件] --> Registry[Mobile Host 插件注册表]
  Registry --> Card[插件 A2UI 预览声明]
  Card --> Shared[共享组件目录与校验]
  Shared --> Flutter[Flutter 工作台卡片]
  Registry --> Detail[平台 WebView 详情]
  Detail --> Bridge[当前插件作用域消息桥]
  Bridge --> Paired[配对授权网关]
  Paired --> Schedule[Desktop 定时任务领域服务]
  Paired --> Mirror[Desktop Workspace 镜像领域服务]
```

Mobile 页面只持有视图状态；定时执行记录与文件系统权限由 Desktop 领域服务持有。工作台排序、置顶按账户、设备、Workspace 分域保存。当前两款插件的详情网页是随应用构建的静态文档；未来的下载包应在签名、能力清单和版本回滚边界完成后接入同一 WebView Host。

Android WebView / iOS WKWebView 作为详情页平台视图，有独立渲染成本，因此不应在瀑布流里逐卡嵌入。Flutter 官方文档也提醒平台视图的合成有性能代价；这正是卡片采用 A2UI/Flutter 的原因。[Flutter Platform Views](https://docs.flutter.dev/platform-integration/android/platform-views)、[webview_flutter](https://pub.dev/packages/webview_flutter)。iOS 发布前另需按 Apple 当前的插件/mini app 条款核对市场下发策略；技术迁移本身不构成上架保证。[Apple App Review Guidelines](https://developer.apple.com/app-store/review/guidelines/)。

## 实施阶段和验收

| 阶段 | 工作 | 验收证据 |
| --- | --- | --- |
| A | SDK 可选贡献接口、共享 A2UI 目录、WebView Host 桥 | 组件白名单、消息作用域和导航限制测试 |
| B | 四项底栏、工作台布局与「我的」、全局主题 | Widget 测试 + 亮暗实机截图 |
| C | 定时任务、Workspace 文件插件 | 持久化/执行、目录分页/搜索、内容预览的端到端测试 |
| D | 移除侧栏市场/账户入口、修复登录保持 | 重启与保留数据升级安装的会话恢复测试 |
| E | Android 与 Desktop 联调 | 配对、插件卡片、详情、计划触发、文件搜索/预览矩阵 |

## 验收矩阵

| 编号 | 类型 | 标准 |
| --- | --- | --- |
| A1 | 架构 | 新插件通过贡献接口出现在工作台；Shell 无按插件 ID 的业务分支。 |
| A2 | 安全 | A2UI 未声明组件/超限树被拒绝；WebView 禁止外域导航，桥仅分发当前插件能力。 |
| F1 | 功能 | 工作台可打开 WebView 详情；插件停用后卡片与详情入口不可执行。 |
| F2 | 功能 | 定时任务记录指定 workspace、时间、消息；重启后仍存在；到期只执行一次并记录结果。 |
| F3 | 功能 | 文件插件能列目录、分页、搜索与预览文本/图片；路径不出现在 Mobile 响应。 |
| F4 | 功能 | 同一安装的 App 重启及保留数据升级后免登录；显式退出后要求重新登录。 |
| F5 | 功能 | 侧栏无市场和账户入口；两者分别在「发现」和「我的」。 |
| U1 | UI | 设计稿的底栏、顶部 tab、搜索位置、浅色/深色主题一致。 |
| U2 | UI | 工作台两列、长内容、编辑与 pin 无溢出；大字号和窄屏可用。 |

“卸载后重新安装”与“覆盖安装”不同：Android 卸载通常删除应用数据，不能靠本地安全存储保证免登录；应以覆盖安装 `adb install -r` 验收持久会话。若产品要求卸载重装也自动恢复，需明确采用操作系统凭据/备份或服务端设备授权方案并另做安全评审。
