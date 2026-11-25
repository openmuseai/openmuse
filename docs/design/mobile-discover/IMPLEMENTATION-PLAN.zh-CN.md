# 插件市场「发现」改版计划与验收矩阵

状态：已完成本地实现与核心验收（2026-10-14）。视觉方向采用浅色编辑式信息流，Mobile 与 Desktop 共用暖白底色、石墨色文字、OpenMuse 蓝色操作色；不同屏幕采用各自合适的布局。

## 交付范围

1. Mobile 底栏第四项由「定时任务」改为「发现」，市场作为独立 tab；保留抽屉市场入口，进入同一页面。
2. 「发现」提供精选、办公、UI、编程、生活分类；精选页采用主题媒体、插件列表、平台徽标和安装状态组成的信息流。无内容的类别显示明确空态。
3. 插件详情展示身份、说明、预览媒体、平台/权限、来源、版本和安装/启停操作。媒体模型区分图片、动图、视频；播放只在用户操作后开始。
4. Desktop 市场改成同源色彩和排版的编辑式列表，保留搜索、安装、启用、禁用与已安装入口；详情承载兼容性和 DSH 重启提示。
5. 市场指标尚无服务端时不显示虚构数字。概念图中的图片/视频画面为设计素材，实际条目按已验证 artifact 和真实媒体展示。

## 实施顺序

| 阶段 | 工作 | 完成条件 |
| --- | --- | --- |
| A | 共用市场视觉 token 与媒体组件；准备已授权的主题海报 | Mobile/Desktop 文字、色彩、间距一致；无外部占位图 |
| B | Mobile tab、分类信息流、详情、安装/预览/启停 | 真机从「发现」安装并重新打开 QuickJS/A2UI 插件 |
| C | Desktop 目录和详情重构 | 现有 Archify 安装、启停及搜索流程保留 |
| D | 功能、视觉、无障碍及响应式验收 | 下表逐项记录证据与限制 |

## 验收矩阵

| 编号 | 类型 | 场景 | 判定标准 | 结果与证据 |
| --- | --- | --- | --- | --- |
| F1 | 功能 | Mobile 底栏 | 第四项是「发现」，点按直接切换市场，不打开新的路由 | 通过：`workbuddy_shell_test.dart`；真机 `android-discover-accepted.png` |
| F2 | 功能 | 分类 | 精选/办公/UI/编程/生活切换；空类别说明清楚 | 通过：Android `mobile_plugin_market_e2e_test.dart` 切换办公/生活并检查空态 |
| F3 | 功能 | 详情 | 点条目进入正确详情；返回保留市场入口 | 通过：Android E2E；`android-detail-accepted.png` |
| F4 | 功能 | 安装 | 摘要验证后安装；按钮变为打开；重启仍可见 | 通过：Android E2E 安装/打开；`mobile_plugin_store_test.dart` 验证摘要与恢复 |
| F5 | 功能 | 预览/启停 | 预览不持久安装；禁用后不能打开；启用恢复 | 通过：Android E2E 预览；Store 单测验证禁用持久化，详情页提供启停操作 |
| F6 | 功能 | Desktop 管理 | 搜索、计划、安装、启用、禁用与已安装视图工作 | 通过：`market_surface_test.dart`、`plugin_market_installer_test.dart`、`plugin_market_ui_e2e_test.dart`；后者实装 Archify 并验证 DSH 技能文件 |
| F7 | 功能 | 媒体 | 图片/动图显示；视频可点击播放/暂停；失败回退海报 | 部分通过：真机展示图片；`mobile_market_media_test.dart` 验证动图标识与不安全视频地址回退。暂无已发布的 HTTPS 视频素材，因此真实视频播放尚未验收 |
| U1 | UI | Mobile 信息层级 | 搜索/分类/主视觉/条目/底栏清晰，文字不截断 | 通过：真机 `android-discover-accepted.png`；390×844 widget 测试无异常 |
| U2 | UI | Mobile 详情 | 身份、媒体、说明、权限、操作可连续阅读 | 通过：真机 `android-detail-accepted.png`；Android E2E 可进入并返回 |
| U3 | UI | Desktop 响应式 | 宽窗信息流；窄窗单列，无横向溢出 | 通过：580px/1100px widget 测试无布局异常；macOS Debug App 1592×760 实窗检查首屏、滚动卡片和详情 |
| U4 | UI | 风格一致 | 暖白、石墨、OpenMuse 蓝和图标/圆角/间距一致 | 通过：共享 `openmuse_market_ui` token、海报与组件；Mobile 真机与 macOS 实窗均已目视审核 |
| U5 | UI | 可用性 | 触控目标 ≥44dp，键盘可聚焦，加载/错误/空态可读 | 部分通过：分类、搜索、安装与空态有自动化覆盖；尚未完成屏幕阅读器与全键盘人工审核 |

## 已知外部依赖

市场服务没有下载量、点赞、评论和媒体上传 API；正式签名发布仍需发布密钥和 HTTPS 地址。当前 Mobile 两个条目是功能参照演示，Desktop 固定目录为 Archify；界面不得暗示它们已完成同一跨平台 artifact 的发布。

## 本次交付证据

- [Mobile 发现页真机截图](android-discover-accepted.png)
- [Mobile 详情真机截图](android-detail-accepted.png)
- [Desktop 浅色设计参考](desktop-market-light.png)：概念图仅用于视觉方向；正式数据以产品目录为准。
- Android 真机 `PKM110 / Android 16`：安装正式 Debug APK 后从底栏进入发现页；`mobile_plugin_market_e2e_test.dart` 全部通过。
- macOS Debug App：匿名模式下打开插件市场，确认浅色弹窗、信息流首屏、滚动卡片和 Archify 详情；`flutter build macos --debug` 成功。
