# 账号设备目录、Presence 与 Desktop 配对

> 状态：Implemented / Android real-device E2E verified
>
> 日期：2026-09-30

## 1. Review 结论

旧流程是 `固定 Desktop origin + 6 位码`。它验证了同账号授权和 DSH transport，
但不是完整的多端产品流程：用户无法确认账号下有哪些设备、目标设备是否在线，也无法
从多台 Desktop 中选择目标。

新流程调整为：

```text
GoTrue 登录
   ↓
账号设备登记 + presence heartbeat
   ↓
同账号设备目录（在线 / 离线）
   ↓ 仅在线且声明 paired-desktop.transport 的 Desktop 可选
选择 Desktop
   ↓
Desktop 一次性码确认当前 Workspace grant
   ↓
Paired Desktop transport → 同一个 local DSH / DSH_HOME
```

账号相同只解决“设备属于谁”，不自动授予本地文件和 Agent 执行权限；一次性码解决
“这一次由人明确选择哪台 Desktop、哪个 Workspace”。两层都必须通过。

## 2. 参考产品与取舍

- WorkBuddy 的官方多端流程要求 Desktop 先允许移动端连接，Mobile 登录同账号后从
  设备列表选择电脑；设备关系和在线/离线状态独立于会话状态。
  <https://www.workbuddy.cn/docs/workbuddyapp/features/Multidevice>
- TeamViewer 把账号设备集中在 Devices 入口，并把 online/offline 作为是否可连接的
  前置状态。<https://www.teamviewer.com/en/global/support/knowledge-base/teamviewer-remote/devices/device-groups-explained/>
- ToDesk 同样从设备列表发起连接，并显式呈现设备上下线。
  <https://www.todesk.com/helpcenter/solo-181.html>

OpenMuse 不复制远程桌面产品的“无人值守密码”语义。因为目标能力是 Workspace/DSH，
不是整机桌面控制，所以采用账号设备发现 + Desktop 当次一次性码 + 精确 Workspace
grant。V1 不提供远程唤醒；离线设备只展示，不允许配对。

## 3. 领域与 Plugin 边界

| 领域 | 所有者 | 职责 | 明确不负责 |
|---|---|---|---|
| Authentication | `com.openmuse.auth.gotrue` | 登录 UI、session 恢复/刷新/退出、token capability | 设备 presence、配对、Workspace |
| Account Device Control Plane | `workspace-paired` | 设备登记、心跳、同账号目录、online 判定、撤销 | DSH 会话模型、文件读写 |
| Pairing / Grant | `workspace-paired` | 目标设备校验、一次性码、account/device/workspace/TTL grant | GoTrue secret、DSH 生命周期 |
| DSH Runtime | `dsh-agent` | local sidecar、会话/history/running/消息 | 设备目录与登录 |
| Host | Desktop/Mobile composition root | 安装插件、注入 ports、承载贡献 UI | 跨领域复制状态 |

`Everything is Plugin` 在这里的含义不是把所有代码塞进一个插件：Auth 插件发布最小
身份能力，设备插件消费短期 access token，DSH 插件继续只暴露 runtime connector。

## 4. 控制面合同

服务端新增同账号隔离的 `/api/muse/devices`：

| Method | Path | 语义 |
|---|---|---|
| `POST` | `/api/muse/devices` | 注册或更新当前设备，同时刷新 presence |
| `GET` | `/api/muse/devices` | 列出当前账号未撤销设备，包括在线与离线 |
| `POST` | `/api/muse/devices/heartbeat` | 刷新当前设备 `lastSeenAt` |
| `POST` | `/api/muse/devices/{deviceId}/revoke` | 撤销当前账号的一台设备 |

所有入口使用 GoTrue JWT 推导 account UUID，body 中不接受 account id。数据库主键为
`(account_uuid, device_id)`。在线状态由服务端按 `last_seen_at >= now - 60s` 计算；客户端
每 20 秒心跳，因此正常网络下允许两次心跳丢失。退出登录立即停止心跳并清空本地目录。
身份切换使用 generation fencing，旧账号的迟到响应不得进入新账号 UI。

设备声明：

```json
{
  "deviceId": "desktop.<stable-id>",
  "displayName": "MacBook-Pro",
  "platform": "macos",
  "deviceKind": "desktop",
  "capabilities": ["workspace.local", "dsh.local", "paired-desktop.transport"],
  "transportOrigin": "https://relay-or-device-origin"
}
```

Mobile 的稳定 device id 保存在平台 secure storage；Desktop V1 使用本地 Application
Support Workspace 身份派生的稳定 id。生产 `transportOrigin` 必须是 HTTPS relay/device
endpoint；本地工程验收仅允许 loopback HTTP，并依靠 ADB reverse。

## 5. 产品状态与错误语义

设备状态和 DSH 会话状态分离：

- `online`：设备 heartbeat 新鲜，可以尝试发起配对；
- `offline`：保留在账号设备列表，但连接入口 disabled；
- `paired`：一次性码成功并获得短期 Workspace grant；
- `connected`：grant transport 已连接真实 Desktop DSH；
- `running / waiting-input / completed / failed`：来自该 DSH 的会话状态，不由设备目录推断。

关键错误保持可区分：`SIGNED_OUT`、`DEVICE_OFFLINE`、
`PAIRING_CODE_DENIED`、`ACCOUNT_MISMATCH`、`WORKSPACE_GRANT_DENIED`、
`PAIR_TIMEOUT`、`GRANT_REQUIRED`。不允许把 Desktop 离线降级为同名 Cloud Workspace。

## 6. 两端 UI

Desktop：

1. 未登录时显示迁移后的 GoTrue 登录页；
2. 登录后在“设置 → 账号与设备”看到当前账号、退出入口、自身 online 状态和同账号设备；
3. 只有自身已成功登记在线，才允许生成/显示配对码。

Mobile：

1. 未登录时显示同一 GoTrue 登录插件；
2. 登录后 AppBar 的“账号设备”入口展示同账号设备及 online/offline；
3. 进入 Local Desktop Workspace 时先选在线 Desktop，再输入该机配对码；
4. 离线 Desktop、Mobile/Web 设备或未声明 transport capability 的设备不可点击；
5. 退出登录后回到登录页并停止 presence。

## 7. 验收矩阵

| ID | 场景 | 期望 | 2026-09-30 验收 |
|---|---|---|---|
| A1 | Desktop/Mobile 未登录启动 | 只能看到登录页，不能看到设备或 Workspace | PASS：两端 widget test；Mobile 真机完成邮箱+密码重新登录 |
| A2 | 两端登录同一账号 | 两端设备目录均出现双方设备 | PASS：Desktop 设置和 Android 真机均显示 Mac + Mobile |
| A3 | 两端 heartbeat 正常 | 两端均显示 online | PASS：真实 API + 两端 UI |
| A4 | 停止 Desktop 超过 60 秒 | Mobile 仍显示该设备，但为 offline，连接按钮 disabled | PASS：服务端 TTL 实测 + Mobile 真机 disabled UI |
| A5 | 不同账号登录 | 设备目录互不可见，直接配对由 Desktop 拒绝 `ACCOUNT_MISMATCH` | PASS：两个真实 GoTrue 账号 API 隔离 + gateway test |
| A6 | 在线 Desktop + 错误/过期码 | 配对失败，不签发 grant | PASS：Android 真机错码 + gateway test |
| A7 | 在线 Desktop + 正确码 | 获得绑定 account/device/workspace/TTL 的 grant | PASS：Android 真机配对，UI 显示 account/grant/DSH 三项已连接 |
| A8 | 请求中的 target device 不等于当前 Desktop | 拒绝 `WORKSPACE_GRANT_DENIED` | PASS：gateway 自动化测试 |
| A9 | 登录账号在请求途中切换 | generation fencing 丢弃旧响应 | PASS：directory controller 自动化测试 |
| A10 | Mobile 打开配对 Workspace | 会话列表、running/等待输入、history 来自同一 Desktop DSH | PASS：Android WebView 显示真实 DSH 历史与运行会话 |
| A11 | Mobile 向既有/运行中会话发消息 | Desktop 与 Mobile 同步显示消息与响应 | PASS：真机发送并收到 `OPENMUSE_MOBILE_OK` |
| A12 | Mobile Cloud Workspace | 不受配对插件影响，仍可发送消息并收到响应 | PASS：Android 真机发现 running session、打开历史、新建会话并发送消息，收到 `OPENMUSE-DETERMINISTIC-RESPONSE` |

## 8. 发布边界

本实现完成账号设备控制面、两端登录入口、presence、设备选择与在线门禁。本地真机可用
ADB reverse 验收。跨公网生产发布仍依赖 opaque relay、设备密钥证明、push/wakeup、
速率限制和审计；不能把 loopback 调试 transport 宣称为生产远程通道。

真机验收还修复了两个边界故障：Workspace catalog Future 现在按登录/配对
generation 稳定缓存，失败可见并可重试；Desktop HTTP proxy 不再把已由
Dart 解压的 body 伪装成 gzip，避免 Android WebView 黑屏。
