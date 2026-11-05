# 思源客户端插件

面向 Agent、多人协作与 Mobile/Web/Desktop 的后续智能文档设计，见 [架构、产品稿、可点击视觉稿与开发计划 v1.1](docs/smart-document/v1/README.zh-CN.md)（2026-11-05；含协作 package/plugin 解耦设计，尚未实施运行产品）。[前期架构讨论](docs/smart-document/README.zh-CN.md)与 [思源 + AppFlowy 融合评估](docs/collaboration/README.zh-CN.md)保留为来源记录；以下描述仍为当前已实现的思源插件。

资源能力插件：使用固定版本的 Protyle 编辑器和 Go 内核。无需部署服务端，不连接远端思源实例。原始源码位于 `vendors/siyuan`，插件构建在独立目录进行，不覆盖上游产品产物。

领域归属：笔记本、块、Protyle、属性视图、搜索、引用、关系图、附件、闪卡、历史、加密及工作空间数据均归思源插件。客户端 Host 只消费公开插件贡献接口，不增加思源 API、数据库、文件格式或原生桥接分支。

`flutter_runtime` 是随发行版编译的原生插件，包含本机内核及 WebView。Android/iOS 不下载可执行内核，不运行 Node。思源原有 HTTP/WebSocket 是插件内部本机通信，监听回环地址和随机端口；入口使用每次进程启动生成的随机凭据和 HttpOnly Cookie。用户数据保存在应用支持目录下独立的插件数据目录，构建和卸载资源不覆盖笔记。原生 Go 单例随应用进程存活，停用时撤销本机入口并落盘，重新启用保留工作空间。

OpenMuse 嵌入版不启动原厂云同步、版本/公告/账号轮询及云端速记定时任务，不启动另一套内核插件管理器或自动嵌入索引器。同步与 AI 授权由 OpenMuse 对应入口管理；本地 SQL 队列、搜索、历史与 OCR 任务保留。该边界由 `scripts/build.py` 对独立暂存源码应用，不改写用户现有服务配置、密钥或固定的上游源码。它不表示用户主动打开的外部链接或远程资源都在本机处理。

第一批构建目标是 ARM64 的 macOS、Android 和 iOS。Host 下限为 macOS 12、Android API 24、iOS 15.1。随包分发的 macOS OCR 原生库要求 macOS 14；更早系统的可用性由原版 OCR provider 探测。Android 的 OCR 原生库使用独立的 `libsiyuan_ort.so` 文件名和 ELF SONAME，避免与其他插件的 ONNX Runtime 冲突。两个端都不借用语音插件的运行库。

构建入口位于 `scripts/build.py`。需要 Go 1.26.10、Node 24、pnpm、Flutter、Xcode，以及 Android NDK。所有内核产物必须启用 `fts5 sqlcipher`，不能用关闭加密支持的构建绕过编译失败。构建先校验固定上游提交，并将源码复制到 `.build`，再应用插件嵌入适配。

```sh
npm run prepare:source
npm run build:web
npm run build:desktop
npm run build:android
npm run build:ios-simulator
npm run build:ios
```

## 资源框架适配（0.2）

[开发计划](docs/resource-adaptation/IMPLEMENTATION-PLAN.zh-CN.md) · [本轮验收](docs/resource-adaptation/ACCEPTANCE.zh-CN.md) · [人工客户端](docs/resource-adaptation/MANUAL-REVIEW.zh-CN.md)

方案、功能/逻辑/UI 分区和开发计划见 [资源适配设计](docs/resource-adaptation/DESIGN.zh-CN.md)。正式入口改为资源中心动态贡献的「块笔记」，Desktop 在 OpenMuse 工作区标签内打开，Mobile 从 Home 打开统一资源页。编辑器不启动原 SiYuan 应用壳。

`.sydoc` 使用原生 `.sy.zip` 归档内容，保留块文档、同包关联、附件与属性视图；保存通过 SDK 受限会话做版本提交。`.sy` 原始正文仅提供只读打开。旧 userdata 保留，不自动拆分知识库。

## 早期完整客户端验收（0.1，历史基线）

以下是 0.1 的历史完整应用壳验收；其实际 Host UI 验收和普通客户端见 [人工验收说明](docs/MANUAL-REVIEW.zh-CN.md)。Desktop 可从“我的插件”直接打开；Mobile 从工作台卡片进入。原生插件随客户端预装。

见 [验收记录](docs/ACCEPTANCE.zh-CN.md)。Desktop、Android 真机与 iOS 模拟器各完成 14 项实际功能检查；iOS 真机安装和商店发行不包含在模拟器结果中。

```sh
cd flutter_runtime/example
flutter pub get
flutter test integration_test/client_acceptance_test.dart -d macos
flutter test integration_test/client_acceptance_test.dart -d <ios-simulator-id>
cd ../..
python3 scripts/accept_android.py --device <android-device-id>
python3 scripts/collect_acceptance.py
```

收集脚本需要将两个 Apple 端命令的输出分别保存在 `.build/acceptance-macos.log`、`.build/acceptance-ios.log`。它只接受原生测试输出中 14 项全部成功的结果。Android 使用独立验收包名，不覆盖正式应用；沿用现有移动验收的原生启动方式，关闭该设备不兼容的 Impeller 调试路径。

文件导入使用插件自己的系统选择器；导出经原版导出 API 和插件保存桥完成，不把用户选中的文件路径交给 Host 的思源专用分支。当前单次桥接导出预算为 64 MiB。`.sy` 是工作空间内部正文；0.2 只读查看它，新建/编辑使用带依赖的 `.sydoc`。

## Host 接入

`Muse-Client/plugins/siyuan-client` 实现公开资源 handler、creator 和 context 贡献；`distribution/openmuse_client_plugins` 负责装配。Host 只增加通用资源会话与工作区标签之间的承载桥，没有思源 API、数据结构或插件 ID 分支。原有下载式 HTML 的 CSP 和网络隔离保持不变。

许可证：上游 AGPL-3.0-or-later 及第三方许可随客户端资源打包，发行时需同步提供完整对应源码及构建适配。思源商业云服务、支付、平台商店授权不由本插件或 Muse Host 代为实现。
