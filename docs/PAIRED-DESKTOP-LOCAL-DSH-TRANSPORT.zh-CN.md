# Mobile ↔ Desktop Local Workspace / DSH Transport

> 状态：Engineering Accepted（本机配对与同 runtime 会话同步已通过 Android 真机验收）
>
> 日期：2026-09-30

## 1. 产品边界

Mobile 默认使用 Cloud Workspace；登录后先从同账号设备目录选择一台在线 Desktop，
再输入该 Desktop 生成的一次性 6 位配对码，将某个本地 Workspace 的 DSH 运行面
授权给 Mobile。完整设备发现与 presence 设计见
`ACCOUNT-DEVICE-PRESENCE-PAIRING.zh-CN.md`。Mobile 不会：

- 直接扫描 Desktop 文件系统；
- 在 Desktop 离线后将同名 Workspace 静默切换成 Cloud Workspace；
- 复制一套 Mobile 会话状态或以 Server 投影代替 DSH 内部会话；
- 将 Desktop DSH bootstrap token 或本地路径暴露给 Flutter UI。

因此，“同步”的语义是 Mobile 和 Desktop 连接同一个 DSH runtime 与同一份
DSH_HOME，会话列表、history、running/approval 状态和新消息都只有一个权威源。

## 2. Plugin 与领域分层

```text
Mobile Host                         Desktop Host
    |                                    |
    | OpenMusePairedDesktopMobilePlugin  | OpenMusePairedDesktopHostPlugin
    |                                    |
    +-- PairedDesktopClient              +-- PairedDesktopGateway
            |                                     |
            | account-bound workspace grant       | DshSidecarSupervisor
            +--------------- transport ------------+-- real local DSH
                                                         |
                                                         +-- Desktop DSH_HOME
```

- Host 只负责插件安装、激活和 UI contribution；
- `workspace-paired` 插件拥有配对、grant 和 transport，不拥有 DSH 会话模型；
- `DshSidecarSupervisor` 仍是 Desktop 本地 DSH 生命周期的唯一权威源；
- Mobile 复用 `DshSessionDescriptor` 和现有受限 WebView，Cloud/paired 仅是 transport
  与授权方式不同；
- GoTrue 插件持有 session，paired 插件只通过 token capability 做当次账号验证。

## 3. 配对与连接协议

1. Desktop/Mobile 登录后向 Cloud 设备控制面登记，并每 20 秒 heartbeat；服务端以
   60 秒 TTL 投影 online/offline。
2. Mobile 从同账号设备目录选择 online 且支持 paired transport 的 Desktop；offline
   设备不可发起配对。
3. Desktop 内建插件仅在 loopback 监听，启动时生成 15 分钟有效的一次性配对码。
4. Mobile `POST /v1/pair/open`，携带 GoTrue bearer、请求设备 ref、目标 Desktop ref、配对码和精确
   `workspaceRef`。
5. Desktop 校验目标 device ref，实时调用 GoTrue `/user` 验证 Mobile token，并与 Desktop 当前账号
   ref 精确比较；账号不同、Desktop 未登录、Workspace 不匹配均 fail closed。
6. 验证通过后签发默认 30 分钟的随机 grant，且配对码立即失效。
7. Mobile 只收到 paired origin + `/u/<grant>`。网关在 Desktop 内部消费真实 DSH
   bootstrap URL，把 DSH auth cookie 和 HttpOnly paired cookie 安装到 Mobile WebView。
8. 后续 HTTP 与 WebSocket 都转发到同一 DSH；网关将 `Origin/Referer` 重写为内部
   DSH origin，既保留 DSH CSRF 同源检查，又不对 Mobile 暴露内部 endpoint。

## 4. 安全不变式

- 默认仅允许 HTTPS；debug 构建可显式允许 ADB reverse loopback。RFC1918 明文 HTTP
  只在 `!kReleaseMode` 且显式配置时可用，不进入 release 信任边界。
- DSH endpoint 必须是 loopback HTTP；跨 origin redirect 被拒绝。
- bearer 仅出现在配对请求，不会透传给 DSH；DSH cookie 不会回到 Dart 业务层。
- grant 绑定 account/device/workspace/upstream/expiry；过期或无 cookie 请求返回 401。
- 配对请求和上游连接有超时，错误文案不回显 token、grant 或本地路径。

## 5. 真机验收证据

本地拓扑使用 Android 真机 + ADB reverse、真实 GoTrue 账号、Desktop 实际 DSH_HOME
与真实 DSH runtime。Node fixture 只替代 ADB reverse 与 Dart `HttpServer` 组合下会停滞的
测试 transport adapter；它复制同一账号/grant/cookie/origin/WebSocket 边界，不替代
GoTrue、DSH 或会话存储。生产 Dart 网关由单元测试覆盖相同的 303 cookie 和
Origin/Referer 重写契约。

| 断言 | 结果 |
|---|---|
| 同账号一次性配对，错误码拒绝 | 通过 |
| Mobile 打开 Desktop 真实 DSH Web UI | 通过 |
| DSH `session/list` 全量结果 | 55 条，其中 9 条非空会话；空白占位按 DSH UI 规则隐藏 |
| Mobile 发消息期间的 running 投影 | 1 条，与 Desktop 独立 API 观测一致 |
| Mobile 消息与回复显示 | 通过 |
| Desktop 权威会话存储出现同一 nonce | 通过 |

## 6. 与 Opaque Relay 的关系

本增量是可运行的本机/ADB 配对 transport，不会把它宣称为生产跨网 Relay。
`PAIRED-DESKTOP-E2E-RELAY.zh-CN.md` 定义的设备注册、SAS、E2E channel 和 opaque relay
仍是非同 LAN/非 ADB 场景的发布门禁。两者共享 Workspace grant 和 DSH transport ABI，
不共享 UI 会话状态。
