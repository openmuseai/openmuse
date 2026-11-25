# 资源中心补充验收：Mobile 1.0.0（8）与 Linux R4

最新 iOS（9）、真实美区 VPS 四状态联动与授权切换修复见 [iOS/VPS 验收](ACCEPTANCE-IOS-VPS-2026-11-02.zh-CN.md)。以下保留本批历史证据与当时边界。

本记录补充 [首批 P0 验收](ACCEPTANCE-2026-11-02.zh-CN.md)，不覆盖或删除 1.0.0（7）的原始证据。商店产物仍为 **internal-test-only**。

## 修复与自动化

- 桌面 Home 路由声明覆盖原生 WebView，进入资源中心时隐藏 DSH，返回后恢复。macOS/Linux 各有一项真实 Host 路由回归测试；macOS 工作台完整回归为 23 通过、1 跳过。
- 桌面主应用启动时激活声明 `onStartup` 的资源能力贡献插件，恢复 Notes 的打开、新建和运行时上下文能力。筛选依据共享 SDK 接口，不使用文档类型白名单。
- 本机/最近刷新独立于远程目录服务；远程不可达或授权失效不再遮蔽本机资源。
- 连接初始化失败后关闭传输并清除连接状态，同一个连接标识可以重新尝试。
- 共享 Host/UI 回归 5 项通过；Mobile 分析无问题，发行流程全量测试通过；Linux Debian 12/amd64 分析及 113 项测试通过、4 项跳过，DSH 运行时与 36 个语法模块检查通过。

## 实际平台检查

| 平台 | 已执行 | 证据 / 边界 |
| --- | --- | --- |
| iPhone Air / iOS 27.2 | 同设备隔离 HTTP 服务先拒绝连接，再以相同标识重试成功；远程刷新失败后仍能浏览、打开本机文件；原生引用剪贴板读写 | [结果](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-offline8/result.json)、[截图](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-offline8/screenshots)。此补验不是新的手机到 VPS 模型调用。 |
| iPhone 主应用 | 用发行 archive 的开发签名副本覆盖安装 1.0.0（8），保留数据并确认进程存活；发行 IPA 未改签 | [启动记录](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-main8/smoke.json) |
| Android / PKM110 | 独立补充验收 APK 已安装；用户将 Android 留给其他测试，本次暂停 Android 交互 | 尚未将该新测试标通过，也尚未覆盖安装主应用（8）。此前（7）的真机结果仍保留。 |
| 美区 VPS / 实际 GTK + WebKitGTK | R4 已部署；原账号、工作区、DSH 与模型配置保留；Home 完整显示、Workspace 浏览、原有 TXT 只读打开、动态新建/编辑/保存均通过 | [结构化记录](../../../target/mobile-acceptance/2026-11-02/resource-center/linux/result.json)、[截图](../../../target/mobile-acceptance/2026-11-02/resource-center/linux/vps-ui)、[真实持久化复核](../../../target/mobile-acceptance/2026-11-02/resource-center/linux/vps-ui/persistence-proof.json)：25 字节内容和摘要一致、2 个版本。 |
| macOS 主应用 | 修复后的主应用完成构建；保留工作区后续桌面变更，并重新完成 23 项回归（1 跳过）及最终构建 | [测试](../../../target/mobile-acceptance/2026-11-02/resource-center/logs/macos-workbench-final-tests.txt)、[构建](../../../target/mobile-acceptance/2026-11-02/resource-center/logs/macos-main-final-build.txt)。构建时源码清单及后续差异保留。 |

原生 WKWebView 的历史引用补验另见 [结果](../../../target/mobile-acceptance/2026-11-02/resource-center/wkwebview-ui/result.json)：重启后，离线有副本可只读打开固定 V1，无副本显示不可用，返回会话正常。在线/离线四分支及真实模型链路仍以首批真机和 macOS 证据为准，不能视作已在新 VPS 上重新跑过完整矩阵。

## 产物与校验

- [Play AAB（8）](../../../dist/mobile/1.0.0-8/play/OpenMuse-play-1.0.0-8.aab)、[Android APK（8）](../../../dist/mobile/1.0.0-8/play/OpenMuse-play-1.0.0-8.apk)、[iOS IPA（8）](../../../dist/mobile/1.0.0-8/ios-us/OpenMuse-ios-us-1.0.0-8.ipa)。Android 注册包名与上传证书一致，APK 非 debuggable；iOS 使用既有 App Store Distribution 签名。
- [Linux R4 deb](../../../dist/linux/20261102-resource-center-r4/OpenMuse-linux-amd64-us-1.0.0.deb)、[便携包](../../../dist/linux/20261102-resource-center-r4/OpenMuse-linux-amd64-us-1.0.0.tar.gz)、[SHA256SUMS](../../../dist/linux/20261102-resource-center-r4/SHA256SUMS)。已在 VPS 验证上传摘要。
- [Mobile 包内容扫描](../../../target/mobile-acceptance/2026-11-02/resource-center/artifact8-scan.json)、[Linux 扫描](../../../target/mobile-acceptance/2026-11-02/resource-center/linux/artifact-r4-scan.json)、[构建源码清单](../../../target/mobile-acceptance/2026-11-02/resource-center/source8-sha256.json)。本次扫描完整读取压缩包并排除验收 fixture 标记；此前私有输入文件已不在本机，不将此扫描宣称为重复的私有凭据值检查。

## 部署与剩余项

VPS 最终程序为 `/opt/openmuse-us/desktop/candidates/20261102-resource-center-r4/OpenMuse/openmuse_linux`；原启动脚本保存在 `/var/backups/openmuse-us-desktop/20261102-resource-center/launcher`。首版发现原生覆盖问题后已实际回滚，再以修复版本替换；没有覆盖账号目录、工作区或模型设置。无授权访问公开资源 RPC 返回 401 `GRANT_REQUIRED`。

Android 新补验与主应用升级按用户要求暂缓。iOS 正在执行真实美区 VPS 的完整四状态矩阵，须取得实际结果后再更新。Cloud、外部 Workspace 写入、MCPB 新建协商等既有 P0 边界沿用实现契约。商店生产输入、线上签名发布和 G0–G6 门禁仍未全部完成；没有上传商店。
