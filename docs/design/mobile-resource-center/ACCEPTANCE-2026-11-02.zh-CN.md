# 资源中心首批 P0 实施与验收报告

最新 iOS（9）、真实美区 VPS 四状态联动与授权切换修复见 [iOS/VPS 验收](ACCEPTANCE-IOS-VPS-2026-11-02.zh-CN.md)。以下保留本批历史证据与当时边界。

后续修复、Mobile（8）产物和美区 Linux 实际部署见 [补充验收记录](ACCEPTANCE-BUILD8-2026-11-02.zh-CN.md)。以下保留首批（7）的范围、结果与原始边界。

2026-11-02。已完成本批共享框架、Mobile Home、Desktop 入口与对话引用链路。Android、iOS 真机和 macOS 原生 Flutter 界面验收通过；设备上的主应用更新至 1.0.0（7）。产物仍为 **internal-test-only**，不等于全部商店发布门禁已通过。

## 结果与证据

| 环境 | 已执行 | 结果 |
| --- | --- | --- |
| Android：PKM110 / Android 16 | 五栏 Home、动态新建、真实编辑/改名、原生剪贴板、引用草稿、真实 PC 对话、保留副本、逐层目录、浅深色 | [原生结果](../../../target/mobile-acceptance/2026-11-02/resource-center/android-r9/result.json) |
| iOS：iPhone Air / iOS 27.2 | 同一共享 UI/协议，USB 私有 IPv6 配对，真实 PC / 模型回复 | [最终联动结果](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-r5/result.json)；[首次独立通过](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-r3/result.json) |
| macOS 26.5 / 原生 Flutter Host | 点击真实对话中的资源卡片；临时目录预先清理；V1/V2；在线/离线四种情况；断开与退出清理 | [结果](../../../target/mobile-acceptance/2026-11-02/resource-center/pc-r2/result.json)、[版本/清理证明](../../../target/mobile-acceptance/2026-11-02/resource-center/pc-r2/proof.json) |
| 实际 DSH 网页前端 / 独立 headless Chrome | 当前 journal 两个 canonical 卡片、保留指针、44px 点击区、隐藏执行上下文、Host 消息 | [结果](../../../target/mobile-acceptance/2026-11-02/resource-center/desktop-web/result.json) |
| macOS 原生 WKWebView / 鼠标操作 | 重启 Host/DSH 后点击历史会话：离线有副本只读打开固定 V1；无副本明确报错；返回会话 | [结果](../../../target/mobile-acceptance/2026-11-02/resource-center/wkwebview-ui/result.json) |
| Linux / Debian 12 amd64 容器 | 全量分析、112 测试通过/4 跳过、真实 DSH 运行时、Helix/36 个语法模块、完整 Release 打包 | [构建记录](../../../target/mobile-acceptance/2026-11-02/resource-center/linux/result.json) |
| macOS 主应用 | 实际插件依赖、Swift WebView/剪贴板与 Flutter 主入口完整构建 | `app/openmuse_host/build/macos/Build/Products/Debug/OpenMuse.app` |

以上真机流程使用隔离验收账号空间、真实本机 DSH 0.2.0-rc.2 与获授权的 OpenCode `deepseek-v4.1-flash`；没有用固定模型回复替代调用。PC 的资源 UI 与配对/执行代码直接复用生产共享包，验收 Host 只负责隔离账号、端口和目录。不是本批重新验收生产 GoTrue 登录/账号删除，也不是美区 VPS 升级记录。

iOS Profile 模式使用已注册的 integration text-input client 输入文字，避免 Flutter 的 debug-only `-1` client 被系统忽略；文件、剪贴板、网络与运行时均为原生实际链路。系统键盘布局另有组件覆盖。macOS 的在线/离线四分支由原生 Flutter 集成测试覆盖；另已使用桌面鼠标操作真实 WKWebView，补验重启后的两个离线分支与返回会话。独立 headless Chrome 检查仍只代表网页呈现和消息桥；WebView2 / WebKitGTK 的 OS 级点击验收尚未执行。

## 用户提出的跨端场景

先在手机创建并保存 V1；两次真实发送分别选择“仅引用”和“保留对话版本”。PC 确认生成过执行投影，模型返回文件中的校验短语。之后手机原件改为 V2，PC 删除执行目录，才进行以下 UI 点击：

| 手机状态 / 副本 | 实际结果 | 截图 |
| --- | --- | --- |
| 在线 / 无副本 | 经手机桥重读固定 V1，没有误读 V2 | [在线原件](../../../target/mobile-acceptance/2026-11-02/resource-center/pc-r2/02-online-origin-pinned-v1.png) |
| 在线 / 有副本 | 打开验证后的对话 V1 | [在线副本](../../../target/mobile-acceptance/2026-11-02/resource-center/pc-r2/03-online-retained-reference-pinned-v1.png) |
| 离线 / 无副本 | 明确不可用，未访问旧临时路径 | [不可用提示](../../../target/mobile-acceptance/2026-11-02/resource-center/pc-r2/04-offline-origin-unavailable.png) |
| 离线 / 有副本 | 显示“对话版本 · 原件离线”，只读打开 V1 | [离线副本](../../../target/mobile-acceptance/2026-11-02/resource-center/pc-r2/05-offline-retained-readonly-v1.png) |

手机完成自己的目录检查后关闭实际前台桥，PC 等待真实心跳到期；没有在测试里切换假的 online 标志。

## 自动化与修复

| 检查 | 通过数 / 状态 |
| --- | --- |
| Mobile 全量测试与分析 | 79；分析无问题 |
| 纯 Dart 资源核心 | 5 |
| IO / RPC / 版本 / 执行投影 | 29 |
| 共享 Flutter 资源 UI | 3 |
| 配对网关回归 | 9，含消费者断开 SSE |
| 原生对话 UI | 14 |
| i18n | 16；866 条 typed messages 生成检查通过 |
| IPv4 / IPv6 私有传输策略 | 1 |
| SDK 新接口、资源 UI/Host、DSH 分析 | 无问题 |
| Linux 全量源码分析及测试 | 分析无问题；112 通过、4 跳过；本项在 macOS 上执行，不代表 Linux 原生 UI |

可核对的摘要在 [logs](../../../target/mobile-acceptance/2026-11-02/resource-center/logs)，源码摘要在 [source-sha256.json](../../../target/mobile-acceptance/2026-11-02/resource-center/source-sha256.json)。已修复：目录 ink/Material 承载、对话卸载时同步 setState、键盘下引用条溢出、存储初始化失败无恢复入口，以及 SSE 取消后的延迟 socket 异常。PC r1 的完成后异常未作为最终通过证据；最终采用修复并清理后的 r2。

## 安装包与主入口

| 产物 | 说明 |
| --- | --- |
| [Google Play AAB](../../../dist/mobile/1.0.0-7/play/OpenMuse-play-1.0.0-7.aab) | 注册的上传证书；[发布记录](../../../dist/mobile/1.0.0-7/play/release.json) |
| [Android APK](../../../dist/mobile/1.0.0-7/play/OpenMuse-play-1.0.0-7.apk) | 相同注册包名/证书，非 debuggable；[记录](../../../dist/mobile/1.0.0-7/play/apk-release.json) |
| [iOS IPA](../../../dist/mobile/1.0.0-7/ios-us/OpenMuse-ios-us-1.0.0-7.ipa) | App Store Distribution 签名；[发布记录](../../../dist/mobile/1.0.0-7/ios-us/release.json) |

Android 已覆盖升级、保留原应用数据，确认真正主应用的 Home 与五栏正常显示，见 [主应用 smoke](../../../target/mobile-acceptance/2026-11-02/resource-center/android-main7/smoke.json) 和 [截图](../../../target/mobile-acceptance/2026-11-02/resource-center/android-main7/home.png)。iOS 使用该 App Store archive 的开发签名副本覆盖安装，保留数据；发行 IPA 未改签，启动/进程复核见 [iOS smoke](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-main7/smoke.json)。

包内容校验、实际私有输入与验收标记排除检查见 [artifact-secret-scan.json](../../../target/mobile-acceptance/2026-11-02/resource-center/artifact-secret-scan.json)。Android ARM64 ELF 静态对齐及 Mobile Office 依赖闭包通过；连接的 Android 为 4KB 页，不能据此宣称 16KB 设备运行测试完成。

## 边界与后续门禁

本批的完整能力参数见 [实现契约](IMPLEMENTED-CONTRACT.zh-CN.md)。外部物理 Workspace 首期只读，应用管理的 Documents 可写；文本运行时上下文限 32 KiB。MCPB 新建贡献协商、Cloud、跨设备最近同步、任意三设备离线转发、二进制模型附件及 Windows/Linux 原生 UI 不在通过声明内。旧端必须升级资源 RPC 才能提供新目录服务；美区既有 VPS 未在本批升级。

原生 UI 没有穷举系统文件选择器、所有多 handler 组合、每一种文件/错误中断或大文件。中文和窄屏/大字由目录与组件测试覆盖，本轮真机主链路为英文浅深色。只将真实执行过的项目标为通过。

两个商店产物仍记录 dirty source、生产输入不完整、线上签名发布未验证等既有门禁；没有上传商店或自动提交仓库，也没有把内部包提升为可直接提交的最终商店版本。具体剩余项沿用发布记录及上架流水线 G0–G6。

## Linux 补充构建

新版美区 amd64 候选的 [.deb](../../../dist/linux/20261102-resource-center/OpenMuse-linux-amd64-us-1.0.0.deb)、[便携包](../../../dist/linux/20261102-resource-center/OpenMuse-linux-amd64-us-1.0.0.tar.gz)、[SHA256SUMS](../../../dist/linux/20261102-resource-center/SHA256SUMS) 已生成并校验。保留 `--enforce-lockfile`，未跳过测试或语法模块；锁文件摘要与当前 checkout 一致。此项明确区别于前面的 macOS 宿主源码测试。

原生启动检查和美区服务替换仍独立记录；构建通过不代表部署通过。线上服务最后成功查询为 active，HTTPS 应用返回 200；后续 SSH 连接多次超时，尚未上传或切换新版。
