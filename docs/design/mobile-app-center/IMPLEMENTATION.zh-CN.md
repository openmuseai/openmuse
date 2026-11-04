# 四 Tab 应用中心实施与验收

2026-11-02。用户批准按 [产品稿](PRODUCT.zh-CN.md) 实施；本轮交付真实 Flutter 页面、iPhone UI 验收及 1.0.0（10）iOS 候选包。视觉沿用仓库 [taste v1 skill](../../../.agents/skills/design-taste-frontend-v1/SKILL.md)，实现继续使用现有 Flutter / registry / 插件贡献体系。

## 已落地

- 底部固定 **Home / Tasks / Apps / Account**；中文为首页 / 任务 / 应用 / 我的，图标统一线性风格。
- Apps 首次进入 **Overview / 概览**；顶部 **Overview / Discover** 单行、横向可滚动；整理时切换为取消 / 完成操作栏。
- 概览保留真实组件、拖动与上下排序、隐藏、置顶和尺寸偏好。布局按账号、设备及 Workspace scope 保存，兼容原有布局；保存失败保留编辑状态。离开脏布局时明确选择保存、放弃或继续编辑。
- Overview / Discover 分别保留页面状态；跨底部 Tab 返回保留搜索与滚动，再次点已选中的 Apps 回到概览顶部。修复旧任务页面挂载结构导致应用中心被销毁、搜索词丢失的问题；Tasks 草稿和当前任务保留。
- **Account → My Plugins** 独立管理页，隐藏底部导航，显示本机安装范围、搜索和全部 / 更新 / 已停用筛选。下载插件与内置贡献去重，下载插件也采用紧凑管理行；PC 安装清单不混入本机清单。
- 管理页“发现更多应用”显示同一个根 Apps / Discover 页面，保留原管理页筛选；顶栏返回、系统 Back 和 iOS 左侧边缘滑动可回到管理页。透明路由移除遮挡根页面的 modal barrier，避免“页面可见但无法点击”。
- 内置可管理插件通过 registry 生命周期启停，偏好原子写入本机配置，失败时回滚；内置插件没有卸载按钮。可下载插件沿用原有验证、启停和卸载流程，更新采用语义版本比较。
- 目录慢、失败或插件下架时仍可管理已有本机安装。Discover 增加与布局一致的加载占位、内联错误与重试；安装详情同步安装中、失败和完成状态，避免重复安装；可贡献概览组件的已安装 WebView 插件提供查看概览入口。
- 对齐中性浅 / 深底色、单一蓝色强调、顶部字体层级、概览卡片分隔和管理行密度。切换语言时重新加载组件预览，修复“标题已切换，预览仍是旧语言”。第三方插件自带文案保持其自身翻译能力。

## 验收结果

| 项目 | 结果与边界 |
| --- | --- |
| Mobile 全量自动化 | **95 项通过**；包含 Home / 资源路由、Tasks、设置、应用中心与编辑回归 |
| 插件市场测试 | **6 项通过，1 项既有跳过**；包含真实 Registry 原生包、离线发现与下架记录 |
| 静态分析 | Mobile 本次生产文件、原生验收入口、应用中心测试及市场代码无问题 |
| 多尺寸与语言 | 320 / 390 / 430pt，中英与 1.5 倍大字的顶栏、整理操作无布局异常 |
| iPhone 原生 E2E | **通过，错误 0**，见 [原生 binding 结果](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/acceptance-results.json) 与 [本次运行标记](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/proof.json) |
| Release 主入口 | **1.0.0（10）已保留数据安装、启动，进程持续存活**，见下方主应用记录 |
| Android | 遵循此前“mobile 先用 iOS”的顺序，本轮未运行 Android 真机验收 |

原生 E2E 使用 `mobile.main()` 的正式启动流程、真实本机 registry/store 和已安装插件，不以设计 fixture 替代应用。设备本次处于未登录状态，验证的是本机应用中心及导航；未把本轮结果描述为重新登录、实际 VPS 对话或账号删除验收。这些链路的此前证据仍见 [资源中心 iOS / VPS 记录](../mobile-resource-center/ACCEPTANCE-IOS-VPS-2026-11-02.zh-CN.md)。

原生流程实际执行四 Tab、默认概览、Discover 搜索跨 Tab 保留与重复点击回概览、编辑继续 / 放弃 / 持久保存、管理筛选的跨入口返回、iOS 边缘返回、内置插件停用 / 重启用及落盘读取、中英深浅色切换。临时修改的布局、主题、语言在结束时恢复；未卸载用户插件或删除用户文档。

安装进度 / 失败 / 真实 store 完成、目录错误重试、下架与重复条目在自动化中单独验证。原生这次不重新下载第三方应用，不把已有用户插件内容改成原型演示数据。首次完整 UI 通过后，截图核对发现了重复清单和预览语言缓存，修复后以 `ios-r3` 再跑完整流程通过。

## 真机截图

[打开实际 iPhone 页面图库](implementation.html)，或在本机预览 <http://127.0.0.1:16085/implementation.html>。

以下来自最终源码的 iPhone Profile 原生验收包，1260×2736；截图只含 Flutter surface，系统状态栏可能不出现在图中。原型的示例应用、推荐内容和文件不作为生产截图的数据要求。

| 场景 | iPhone 截图 |
| --- | --- |
| 英文浅色概览 / 四 Tab | [Overview](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/01-overview-en-light.png) |
| 英文 Discover | [Discover](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/02-discover-en-light.png) |
| 离开整理的确认 | [编辑保护](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/03-edit-leave-guard.png) |
| Account / 应用与服务 | [Account](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/04-account-en-light.png) |
| 去重后的插件管理 | [My Plugins](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/05-my-plugins-en-light.png) |
| 插件启停详情 | [插件详情](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/06-plugin-management.png) |
| 中文深色 Account | [我的](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/07-account-zh-dark.png) |
| 中文深色概览，预览语言已切换 | [概览](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/08-overview-zh-dark.png) |
| 中文深色真实目录 | [发现](../../../target/mobile-acceptance/2026-11-02/app-center/ios-r3/screenshots/09-discover-zh-dark.png) |

[implementation-screenshots](implementation-screenshots/) 中的 `widget-*` 是测试环境中的真实 Flutter 组件渲染，用于快速复核布局，不冒充真机或真实账号截图。可用 `flutter test test/mobile_apps_center_test.dart --dart-define=APPS_CENTER_CAPTURE_DIR=<绝对输出目录>` 重新生成；完整设备入口为 `app/openmuse_mobile/integration_test/apps_center_acceptance_test.dart`。

## iOS 产物与构建记录

- [1.0.0（10）App Store 签名 IPA](../../../dist/mobile/1.0.0-10/ios-us/OpenMuse-ios-us-1.0.0-10.ipa)、[SHA256SUMS](../../../dist/mobile/1.0.0-10/ios-us/SHA256SUMS)、[发布记录](../../../dist/mobile/1.0.0-10/ios-us/release.json)。
- [产物验证](../../../dist/mobile/1.0.0-10/ios-us/artifact-validation.json)：ZIP CRC、深层签名、App Store profile、既有 Distribution 证书、bundle ID 与版本；Office 依赖闭包和资源内可执行文件扫描通过。
- [主应用安装 / 启动记录](../../../target/mobile-acceptance/2026-11-02/app-center/ios-main10/smoke.json)、[构建与测试日志](../../../target/mobile-acceptance/2026-11-02/app-center/logs/)、[关键源码摘要](../../../target/mobile-acceptance/2026-11-02/app-center/source-sha256.json)。

为避免影响 Android 的并行测试，iOS 在隔离源码副本中构建。修复副本外部依赖不同步和原生库链接缓存后，使用 Xcode 原生构建完成 Profile 验收；最终 Release 从 `lib/main.dart` 单独归档。设备安装的是该 Release archive 的开发签名副本，保留原 App Store 签名 IPA；完整自动点击运行于同源 Profile 测试入口，Release 主入口另做启动检查。

现有生产 discovery 配置、公钥、资源 origins 与商店发布门禁仍按原计划处理，候选状态为 `internal-test-only`，不是已可提交商店的声明。没有上传、提交或合并代码；保留其他任务的工作树改动。

构建快照与并行工作树的边界：本轮应用中心相关文件一致；并行任务在快照之后追加到 `main.dart` 的首启 Slides 安装 / 欢迎资源，以及 `workbuddy_shell.dart` 的桌面连接指南，未纳入本次候选。源码摘要同时记录构建快照与工作树哈希，便于后续整合，未覆盖这些并行改动。
