# 国内环境真机连接验收与目录负载修复

日期：2026-11-11。设备：Android PLJ110 / Android 16，与本机 macOS OpenMuse；两端使用同一国内账号。凭证不写入本文或测试源码。

## 实际执行

- 构建 Android 和 macOS，均使用 `OPENMUSE_RELEASE_MARKET=china`、`OPENMUSE_CONNECTION_TRACE=true`；登录、设备目录及 relay 均走 `cn.openmuseai.com`。
- Android 使用既有发布证书覆盖安装；正常应用构建入口是 `lib/main.dart`。macOS 已安装修复后的 native bridge 0.1.19。
- 发现旧 `dist/OpenMuse.app` 进程占用 13180 端口，关闭该旧实例后，新安装 Desktop 网关恢复监听，relay attach 与设备目录事件连接成功。没有批量终止其它 DSH 进程。
- 真实 Mobile → 国内 relay → Desktop → DSH 流量已采样；观察到消息持久化、assistant 数据和 Desktop 重启后恢复。不能据此声称所有交互场景均已验收。

## 发现及修复

原生 `/openmuse-native/v1/sessions` 直接返回 DSH `controller.list` 的全部 projection。它并非完整聊天历史，却包含 Mobile 列表不需要的大量插件状态。当前用户目录响应约 564KB，每两秒校准继续重复传输；Desktop 生成约 100ms，Mobile 网络接收和解析明显更慢。

bridge 0.1.19 将原生目录限制为以下字段：

- sessionId、updatedAt、running、blank；
- projections.asOfSeq（保持权威校准游标）；
- title、agentPreset；
- modelSelection 的 provider/model/reasoningEffort；
- permissions.currentValue。

消息、工具、完整投影继续由 follow/page 提供；Desktop 原本的 DSH Web 协议不变。新增回归验证 600KB 的无关 projection 被省略，游标、模型、权限保留，本地路径不进入原生目录。

## 真机采样结果

| 成功的会话列表请求 | 修复前 | 修复后 |
|---|---:|---:|
| 样本数 | 44 | 472 |
| 响应体代表值 | 564,371 B | 106,046 B |
| P50（请求开始到解析完成） | 1,290.4ms | 423.5ms |
| P95 | 2,109.1ms | 612.5ms |

以上是前后顺序采样，不是随机 A/B；会话和网络状态可能变化。体积约降低 81%，P95 约降低 71%；这些数字只描述该设备、该账号和该次国内 relay 的列表请求，不代表所有首屏或所有网络。

证据：`evidence/2026-11-11/real-device-cn-session-catalog.json` 及同目录的 `*-cn-*-trace.jsonl`。导出的文件只保留 OMTRACE 事件，原始日志保存在本机私有临时目录，没有导出消息内容和凭证。

## 测试状态及限制

- bridge gateway：14 项自动化测试通过。
- 真机列表负载与耗时：已验证并安装 Desktop 修复。
- 首轮 profile 集成测试因账号恢复阶段等待一次性登录输入超时，未通过；不能记为功能通过。
- 测试入口原先屏蔽物理触摸；测试源码现已设置 `shouldPropagateDevicePointerEvents=true`，允许真人操作协议页。正常应用入口已单独构建，测试结束必须覆盖恢复，不能将测试 APK 当作交付版本。
- profile signing 默认不会继承 Android 发布签名。一次安装器签名不匹配导致卸载旧包；后续 profile APK 已在构建后重新签名，并校验与原应用一致的证书。今后运行 `flutter drive --use-application-binary` 前必须先检查证书，避免 Flutter 自动卸载旧包。
- 实际 Home/Recents 切换期间未观察到新增 pair.connect，但只观察到 recents 的阶段不能算作已完成前台恢复验收。
- 完整新会话/多会话热切换/历史分页/前后台自动化验收仍以实际测试报告为准；当前不声称全部通过。

运行可复用的 `app/openmuse_mobile/integration_test/paired_latency_acceptance_test.dart` 前，先在正常国内应用完成用户本人协议确认和账号登录。测试可从应用专属外部 cache 读取一次性运行时认证输入，读后立即删除；缺少输入应明确失败，不把密码放到 dart-define 或源代码中。运行结束删除剩余认证输入并恢复正常 APK。

## 本次收尾状态

用户已明确要求停止自动化，只保留正常版。已停止 flutter drive，清理手机的一次性认证输入与本机测试 APK 副本。正常国内 APK 已从 `lib/main.dart` 构建，版本码 111105，发布证书已核对；用户已在手机完成 OPPO 安装确认；安装命令返回 Success，包管理器核对正常国内版版本码 111105。Desktop 国内版与 bridge 0.1.19 已安装。完整自动化用例未通过，不再自动重试。
