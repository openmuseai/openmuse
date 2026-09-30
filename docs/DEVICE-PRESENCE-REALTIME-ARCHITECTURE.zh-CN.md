# 设备 Presence 与多端实时同步架构

> ADR：采用 REST Snapshot + WebSocket Event + TTL Reconciliation
> 日期：2026-09-30

## 1. 问题复盘

上一版有三个结构性问题：

1. 同账号连接错误地要求一次性配对码；账号信任与跨账号授权混在一起。
2. Presence 是“注册成功后启动 Timer”的单链路；首次注册、Token 恢复或网络切换失败后可能永久停止。
3. `transportOrigin=127.0.0.1` 只适合同机/ADB 验收，不是跨公网 Desktop 发现和传输方案。

## 2. 协议方案比较

| 方案 | 双向 | 断线感知 | 移动网络 | 服务复杂度 | 结论 |
|---|---:|---:|---:|---:|---|
| REST 轮询 | 否 | 慢，取决于周期 | 稳定但耗电/延迟 | 低 | 仅作为快照和兜底 |
| SSE + REST heartbeat | 下行单向 | 一般 | 自动重连友好 | 中 | 无法统一后续 relay 控制帧 |
| WebSocket | 是 | Ping/Pong + close | 前台优秀 | 中 | **控制面首选** |
| MQTT 5 | 是 | Keep Alive/LWT 完整 | 优秀 | 需 Broker 与 Topic ACL | IoT 规模后再评估 |
| WebTransport/QUIC | 是 | 优秀 | 生态/代理兼容仍不如 WS | 高 | 暂不采用 |

选择独立的 `Device Control WebSocket`，不复用 AppFlowy Collab 二进制 WebSocket，也不把
设备 Presence 塞入 DSH Plugin：三者有不同的版本、权限和故障域。

## 3. 目标分层

```text
Mobile/Desktop Host
  ├─ auth-gotrue plugin          身份与 Token
  ├─ account-device plugin       登记、Presence、设备目录
  ├─ workspace-paired plugin     同账号 Grant、Workspace/DSH transport
  └─ dsh-agent plugin            会话、消息、任务状态

Cloud Control Plane
  ├─ REST /api/muse/devices      权威快照、register、heartbeat、revoke
  ├─ WS   /api/muse/devices/events 账号内变更通知
  ├─ PostgreSQL                  durable last_seen/capabilities
  └─ LISTEN/NOTIFY               多实例事件扇出
```

## 4. Presence 状态机

```text
signed_out
   │ login
   ▼
registering ──失败──► reconnecting ──指数退避+抖动──┐
   │成功                                             │
   ▼                                                │
online ◄── REST heartbeat/list + WS connected ──────┘
   │ WS断开
   ▼
reconnecting（保留 last snapshot，REST heartbeat 继续）
   │ last_seen > 60s
   ▼
offline
```

参数：

- REST heartbeat：20 秒；
- 服务端 online TTL：60 秒；
- WS Ping：15 秒；客户端超时：45 秒；
- 重连：1/2/4/8/16/30 秒封顶，并加入 0–25% jitter；
- 每次重连成功先取 REST 全量快照，修复丢事件和乱序。

WS 只是低延迟通知，不是 online 的唯一事实来源。进程崩溃、网络分区和移动端挂起最终都由
`last_seen_at + TTL` 收敛；优雅下线可以额外发送 offline，但不能依赖它。

## 5. WebSocket V1 合同

客户端以 GoTrue Bearer 建立：

```http
GET /api/muse/devices/events
Authorization: Bearer <access-token>
Upgrade: websocket
```

服务端只推同账号事件：

```json
{"type":"device.snapshot-required","reason":"connected"}
{"type":"device.changed","deviceId":"desktop.xxx","reason":"heartbeat","revision":1760000000000}
```

客户端收到任何 `device.changed` 都节流执行 REST `GET /api/muse/devices`。事件只表达“需要刷新”，
不携带完整设备记录，避免事件缓存成为第二权威源。`revision` 用于诊断与后续 gap 检测。

## 6. 同账号访问合同

Mobile 对在线 Desktop 调用：

```http
POST /v1/account/open
Authorization: Bearer <same-account-token>

{"deviceRef":"mobile.xxx","targetDeviceRef":"desktop.xxx","workspaceRef":"..."}
```

Desktop 验证 Bearer 对应账号等于本机登录账号，再生成短期 Grant。此流程没有配对码。
`/v1/pair/open` 仅保留为未来跨账号关系的兼容槽位，不在同账号 UI 暴露。

## 7. 实时任务同步

- Device Control WS：设备上线/离线、能力变化、relay 信令。
- DSH API Gateway/WebSocket：`session/list`、history、follow、prompt、stop；DSH 是任务状态权威源。
- AppFlowy Realtime：Office/Workspace 文档协同；不承担 Agent Session Presence。

三条协议共享 `accountRef/deviceRef/workspaceRef/sessionRef` 关联键，但不共享内部消息模型。

## 8. 公网数据面演进

V1 本地验收仍可走 direct/ADB transport。生产版本必须由 Desktop 建立出站 `wss` 到 Cloud Relay；
Mobile 只得到 opaque relay route，不得到 Desktop IP。Relay 只转发加密流并执行账号/Grant ACL，
不解释 Workspace 或 DSH 业务。后续加入：

1. Desktop 设备密钥与 proof-of-possession；
2. relay stream multiplexing、流控和 0-RTT 禁用策略；
3. APNs/FCM 唤醒与“上线后重试”；
4. Grant 撤销、审计、速率限制；
5. 多实例 relay affinity 或 Redis route registry。

## 9. 验收矩阵

| ID | 场景 | 期望 | 2026-09-30 结果 |
|---|---|---|---|
| R1 | Desktop/Mobile 同账号登录 | 无验证码，设备互见 | PASS：真机直接进入在线 Mac 工作台 |
| R2 | Desktop 数据面端口被占用后恢复 | Presence 不受影响；端口释放后自动启动 Gateway | PASS：Mac 始终在线，释放 `13180` 后 1 秒内恢复并发布 origin |
| R3 | WS 中断但 REST 可用 | 保持 heartbeat，显示重连而非离线 | PASS：单元测试覆盖 WS 退避与 REST reconcile |
| R4 | 服务重启/事件丢失 | 重连后 REST 快照修复 | PASS：单元测试覆盖快照修复 |
| R5 | 停止 heartbeat 超过 60 秒 | 其他端显示 offline，入口 disabled | PASS：TTL/离线判定测试覆盖 |
| R6 | 不同账号调用 account/open | `ACCOUNT_MISMATCH` | PASS：Gateway 鉴权测试覆盖 |
| R7 | 选择在线 Desktop | 直接获得绑定设备/Workspace 的 Grant | PASS：真机无配对码打开 Project Workspace |
| R8 | Mobile 查看 Desktop DSH | 进行中和已完成会话来自同一 runtime | PASS：完整 macOS DSH，运行态、完成态和重进会话均可见 |
| R9 | Mobile 新建/续聊/停止 | Desktop 同步显示且幂等 | PASS：真机发消息并收到确定性 SSE 响应；停止合同由 Gateway 测试覆盖 |
| R10 | Mobile 进入后台 | 不承诺 WS 常驻；回前台全量刷新，后续 Push 唤醒 | PASS：lifecycle resume reconcile 测试覆盖 |

## 10. 实现复盘：Presence 与数据面解耦

macOS 首轮联调暴露的根因不是账号发现失败，而是 Host 激活顺序把 Presence 错误地耦合到了
本地 DSH Gateway：`13180` 被占用时 Gateway bind 抛错，后续设备登记没有执行，因此 Mobile
看不到 Mac。修复后：

1. `account-device` Presence 先启动并独立维持 register/heartbeat/WS；
2. `workspace-paired` Gateway 失败只影响数据面，不撤销设备在线状态；
3. Gateway 使用 1/2/4/8/16/30 秒退避重试，成功后触发目录 reconcile 并发布
   `transportOrigin`；
4. UI 分别呈现“设备在线”和“工作区服务恢复中”，避免把控制面和数据面合并成一个布尔值。

这验证了 Host Plugin 的领域边界：认证插件提供身份，设备插件负责 Presence，Workspace Plugin
提供 Grant/transport，DSH Plugin 负责会话和执行。任一数据面插件故障不应阻止账号设备上线。

验收使用带完整 DSH closure 的 macOS App；Mobile 通过 ADB reverse 访问 Mac Gateway，发送
`DESKTOP-MOBILE-E2E-20260930` 并得到同 marker 响应。延迟响应用例在离开并重新进入会话后仍显示
“深度求索中”和停止按钮，随后收敛为完成态。该用例证明当前 direct transport 的会话权威源在
Mac DSH，而不是 Mobile 本地伪造状态。

## 11. 参考资料

- RFC 6455 WebSocket：https://www.rfc-editor.org/info/rfc6455/
- MQTT 5.0 Keep Alive / Will：https://docs.oasis-open.org/mqtt/mqtt/v5.0/mqtt-v5.0.html
- Firebase Presence / onDisconnect：https://firebase.google.com/docs/database/web/offline-capabilities
- Android FCM 后台消息：https://firebase.google.com/docs/cloud-messaging/android/receive-messages
- Apple Background Execution：https://developer.apple.com/documentation/Xcode/configuring-background-execution-modes
- WorkBuddy 多端协同：https://www.workbuddy.cn/docs/workbuddyapp/features/Multidevice
