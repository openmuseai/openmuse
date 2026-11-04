# 资源中心 iOS / 美区 VPS 实际联动验收

2026-11-02。共享资源框架与 Home 的本批 P0 联动验收通过，iPhone 主应用已恢复至 **1.0.0（9）**。Android 按用户要求留给其他测试，本轮没有继续操作 Android。商店产物仍为 **internal-test-only**，未上传商店。

## 实际执行范围

- iPhone Air / iOS 27.2：启动生产 Mobile 入口，复用既有美区账号登录态，通过真实设备目录、配对授权和 HTTPS relay 连接美区 Linux。没有替换账号服务、设备目录、RPC 或模型回复。
- Home：Remote → Workspace → 两级目录 → 文档只读打开；已安装 Notes 插件贡献新建类型；本机创建、编辑、保存；原生复制引用并粘贴到 Tasks，显示资源卡片。
- 真实 VPS DSH / OpenCode `deepseek-v4.1-flash`：普通引用与保留副本引用各发送一次，分别检查 **assistant** 回复确实包含手机文档中的验证短语；另查询服务端会话配置确认模型。
- 手机原件提交为 V2，并从实际持久化 ledger 和 blob 独立验证摘要、V2 字节与仍保留的 V1。随后只按本次文档 V1 摘要匹配，清理 VPS 上两个执行租约目录；其他资源未清理。
- PC 点击验收在实际 VPS 的 GTK + WebKitGTK 主应用中完成，经现有私有 noVNC 和本机回环 SSH 隧道操作，没有使用 headless 页面代替原生 WebView。

## 四状态 UI 矩阵

| 手机 / 保留副本 | 实际 PC 行为 | 截图 |
| --- | --- | --- |
| 在线 / 无副本 | 重新向手机读取固定 V1；不使用已更新的 V2 或旧执行路径 | [在线原件](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/pc-ui/02-online-origin-v1.png) |
| 在线 / 有副本 | 只读打开对话 V1 | [在线副本](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/pc-ui/03-online-retained-v1.png) |
| 离线 / 无副本 | 保留引用，明确提示源离线且没有授权副本 | [不可用提示](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/pc-ui/04-offline-origin-unavailable.png) |
| 离线 / 有副本 | 显示 `Conversation version · Original offline`，只读打开固定 V1 | [离线副本](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/pc-ui/05-offline-retained-readonly-v1.png) |

离线通过停止实际手机资源桥、取消配对连接并等待 15 秒实现，超过服务端 10 秒心跳有效期；没有伪造 online 标志。检查结束后恢复美区 Desktop 选择，再覆盖安装主应用，保留数据。

最终 [iOS 原生结果](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/result.json) 为通过、错误 0；[PC UI 结果](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/pc-ui/result.json)、[手机版本证明](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/phone-original-version-proof.json)、[执行目录清理证明](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/projection-cleanup.json)、[手机截图](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-vps-live-r6/device) 均已保存。

## 授权切换修复与回归

前几轮实际远程读取曾出现通用错误。检查发现一个可确定复现的竞态：配对授权刷新会关闭旧 HTTP client，正在读取目录、元数据或内容的请求可能因旧连接关闭而失败。

修复位于共享资源 UI 控制器与 Host 连接适配中，各端使用同一逻辑：只等待已经开始的连接切换；仅当同一个源身份已换成另一个可用 Provider 时重试一次读取，并重新检查权限与版本。不会主动重连离线源、扩大授权，或反复重试拒绝访问。延迟旧 HTTP 响应、关闭旧连接、建立新授权的回归覆盖 list / describe / read；新授权拒绝读取的负向检查也通过。

最终真机流程在浏览目录和打开文档时分别主动刷新真实 VPS 配对授权，随后完整走通上述链路。此前错误的所有原始触发原因未逐项取得错误码，因此不宣称这个竞态是每次通用错误的唯一根因，也不将一轮通过称为长时间网络稳定性测试。早期失败与上一轮成功记录保留在 `ios-vps-live*`，最终结论以 `r6` 为准。

| 检查 | 结果 |
| --- | --- |
| 共享 Host / UI / 连接恢复 | 10 项通过，分析无问题 |
| Mobile 全量回归 | 79 项通过，分析无问题 |
| iOS 原生完整联动 | 通过，错误 0；包含真实授权刷新和两次模型调用 |
| VPS 原生 PC 四状态点击 | 四项通过 |
| iOS 主应用 | Release archive 的开发签名副本安装、启动、进程存活通过 |

测试先在隔离 APFS 源码副本中构建，避免占用 Android 测试输出。首次全量回归缺少副本中的 PPTX fixture 与 HTML 测试工具，补齐原仓库文件后 79 项全部通过；没有跳过这些测试。原生 E2E 使用 Profile 集成测试入口，Release 主入口独立归档并做启动检查，不宣称原样 Distribution IPA 已跑完整自动点击。VPS 仍运行此前已验证的 Linux R4；新的共享客户端恢复代码尚未重新打包到 Linux/macOS/Windows 主应用。

## iOS 产物与边界

- [App Store 签名 IPA：1.0.0（9）](../../../dist/mobile/1.0.0-9/ios-us/OpenMuse-ios-us-1.0.0-9.ipa)、[SHA256SUMS](../../../dist/mobile/1.0.0-9/ios-us/SHA256SUMS)、[发布记录](../../../dist/mobile/1.0.0-9/ios-us/release.json)。包名 `com.openmuseai.mobile`，既有 Apple Distribution 身份与 App Store profile 校验通过。
- [包扫描](../../../target/mobile-acceptance/2026-11-02/resource-center/artifact9-scan.json)：全包 CRC、依赖、签名、验收 fixture 标记排除均通过。旧私有输入文件已不在本机，没有声称重复扫描其凭据值。
- [源码摘要](../../../target/mobile-acceptance/2026-11-02/resource-center/source9-sha256.json)、[主应用恢复记录](../../../target/mobile-acceptance/2026-11-02/resource-center/ios-main9/smoke.json)、[测试与构建日志](../../../target/mobile-acceptance/2026-11-02/resource-center/logs)。未提交或清理用户工作树。

本轮不覆盖 Android 新补验、Windows 原生 UI、任意三设备转发、Cloud 浏览、外部 Workspace 写入或大文件 relay 的完整矩阵。生产 discovery URL、配置验签公钥、允许的资源 origins 与 G0–G6 门禁仍未全部就绪；这份 IPA 是内部验收候选，不等于可以立即提交审核。
