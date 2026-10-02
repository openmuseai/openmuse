# Desktop 出站附着与按设备转发

> 状态：实施合同
>
> 日期：2026-10-01
>
> 控制面沿用《设备 Presence 与多端实时同步架构》。本文只定义公网数据面。
> 它不是《Paired Desktop E2E Channel》里的 opaque relay：服务端能看到转发的 HTTP，
> 传输保护来自 Desktop 到公网入口的 TLS，而不是端到端 AEAD。

## 1. 问题

同一账号的 Mobile 已经能发现并打开 Desktop 上的 DSH，但数据面是 Desktop 本机
`127.0.0.1:13180`。公网手机访问不到这个地址。SSH 反向隧道能把端口带出内网，
却要求 Desktop 持有服务器 root 凭证，并且一台服务器端口只能挂一台电脑。

公网方案改为：Desktop 用当前 GoTrue access token 主动连出；服务端只按
`(account, deviceId)` 转发；Mobile 只访问一个独立的 HTTPS 源，不拿到 Desktop 的 IP。

## 2. 为什么不能挂在现有站点路径下

`resolveRemoteDshUri` 只接受 path 为空的 origin，会话路径必须以 `/u/` 或 `/session/`
开头。DSH 页面里的脚本和接口使用站点根路径。若把转发挂在
`https://openmuseai.com/api/muse/relay/...` 下，WebView 会回到
`https://openmuseai.com/u/<grant>`，随后的 `/api` 会打进 Cloud 或 BFF，而不是这台 Desktop。

因此公网数据面占用整台主机名。目标名字是：

```text
https://link.openmuseai.com
```

只有 `OPENMUSE_RELAY_PUBLIC_ORIGIN` 被显式设置时 Desktop 才向外附着。不要从 Cloud
主机名猜测 `link.` 前缀：当前证书只包含 `openmuseai.com` / `www.openmuseai.com`，
`link.openmuseai.com` 还没有解析。在这张证书上，本次部署使用

```text
https://openmuseai.com:8443
```

该端口上的全部路径都属于转发，不和 `openmuseai.com:443` 的 `/api/muse`（BFF）、
`/gotrue`、Cloud `:8000` 混用。证书补上 `link` 之后，把这个环境变量改成 443 上的
独立主机名即可，协议不用改。

## 3. 分层

```text
Mobile
  设备目录  REST/WS  https://openmuseai.com/api/muse/devices
  打开 DSH  HTTPS    https://openmuseai.com:8443

Desktop
  设备目录  同上，heartbeat 20s，与数据面无关
  本地网关  http://127.0.0.1:13180     只接受本机
  出站附着  wss://openmuseai.com:8443/attach

生产机 openmuseai.com
  systemd: openmuse-device-relay
    进程只绑定该机器的 127.0.0.1:8096，避免明文端口直接暴露
    路由表 (account, deviceId) → 当前附着连接
    grant 索引只活在这个生产进程里
    /api/muse/devices 也由该进程提供，直到 Cloud 镜像包含这条路由
  nginx（同一台生产机）
    :443  /api/muse/devices  → 127.0.0.1:8096
    :443  /api/muse          → BFF :8010，不改道
    :8443 全部路径            → 127.0.0.1:8096，这是对外的 DSH 转发源

8096 不是开发机端口，也不是 Desktop 的 13180。开发机不参与转发。
Desktop 只向生产机的公网地址出站附着；Mobile 只访问生产机的公网地址。
```

Presence 继续以 PostgreSQL `last_seen_at` 加 60 秒 TTL 为准。附着断开不会把设备
立刻标成离线，心跳成功则目录里仍然在线。本地 `13180` 绑不上也只影响转发，不影响登记。

## 4. 附着

Desktop 登录后维持一条 WebSocket：

```http
GET /attach
Authorization: Bearer <GoTrue access token>
Upgrade: websocket
```

第一条文本帧必须是：

```json
{"type":"hello","protocol":1,"deviceId":"desktop.<stable-id>"}
```

`deviceId` 只允许字母、数字、点、下划线和连字符，最长 160。服务端用 token 向
GoTrue `/user` 取 `id` 作为 account，路由键是 `(account, deviceId)`。同一键的新附着
替换旧连接；旧连接上未完成的流转为 `RELAY_REPLACED`。token 不能把附着登记到别的账号。

服务端回答：

```json
{"type":"hello-ok","pingIntervalMs":15000,"pongTimeoutMs":45000}
```

之后服务端每 15 秒发送 `{"type":"ping","id":N}`。45 秒没有对应 `pong` 就关闭连接，
路由立即删除。附着期间每 60 秒复查 token，失效则关闭。

协议帧全部是 WebSocket 文本 JSON。HTTP 头用二元组列表，保留重复的 `Set-Cookie`。
body 为 base64。单请求 body 上限 8 MiB，每台 Desktop 同时最多 32 条流。

| type | 方向 | 作用 |
|---|---|---|
| `hello` / `hello-ok` | Desktop → 服务端 / 反向 | 绑定设备 |
| `ping` / `pong` | 双向 | 应用层存活，不依赖 WebSocket 协议 ping |
| `http.request` | 服务端 → Desktop | `id, method, path, headers, body` |
| `http.response.start` | Desktop → 服务端 | `id, status, headers` |
| `http.response.chunk` | Desktop → 服务端 | `id, data` |
| `http.response.end` | Desktop → 服务端 | 结束该 HTTP 流 |
| `ws.open` / `ws.ready` | 服务端 → Desktop / 反向 | 打开到本地网关的 WebSocket |
| `ws.data` | 双向 | `encoding` 为 `text` 或 `binary` |
| `ws.close` | 双向 | 关闭该流 |
| `error` | Desktop → 服务端 | `code=UPSTREAM_FAILED` 或 `UPSTREAM_UNAVAILABLE` |

## 5. Mobile 如何进入正确的 Desktop

`transportOrigin` 对启用了公网转发的 Desktop 固定为该 HTTPS 源，不随重连变化。
同一源上可以挂多台电脑，打开时靠 body 里的 `targetDeviceRef` 选择附着。

1. `POST /v1/account/open` 必须带 Bearer。服务端确认调用者 account 与附着 account 相同，
   再把请求转给对应 Desktop。本地网关继续做同账号校验。
2. 成功响应里的 `grantRef` 写入内存索引，直到 `expiresAtMs`。
3. WebView 打开 `https://link.openmuseai.com/u/<grant>`。没有 Bearer 时，用路径中的
   grant 或 `OpenMuse-Paired` cookie 找到设备。grant 对不上则 `401 GRANT_REQUIRED`。
4. 随后 DSH 的页面、接口和 WebSocket 都落在这个源上，由 cookie 继续路由。

服务端进程重启后 grant 索引消失。Mobile 需要重新 `account/open`。Desktop 本地网关里的
grant 可以还在，但不能绕过索引。

没有附着时返回：

```json
{"code":"RELAY_DETACHED","message":"Desktop 公网通道暂时断开，正在重连。"}
```

流满时为 `RELAY_BUSY`。等待 Desktop 超过 30 秒为 `RELAY_TIMEOUT`。

## 6. 本地网关的 origin

附着进程在 Desktop 上访问 `127.0.0.1:13180`，并在每个请求加上只有本机才会发送的头：

```http
x-openmuse-paired-public-origin: https://openmuseai.com:8443
```

网关仅当 TCP 对端是 loopback，且该值是无 userinfo、无 query、path 为空的 HTTPS URI 时采信。
采信后：

- `session.origin` 写成这个 HTTPS 源，`allowInsecureLoopback` 为 false；
- `OpenMuse-Paired` cookie 加 `Secure`，`Path=/`，`SameSite=Strict`。

不采信 `Host`。公网 Host 不能把传输源从 loopback 扩大出去。转到本机 DSH 时仍然改写
`Origin` 和 `Referer` 为 DSH 自己的 loopback，并丢掉上述跳转头。未配置公网源时行为与
现在的 loopback 验收一致。

## 7. 断网之后如何恢复

Desktop 进程还在，网络中途断开时，两条回路互不取消：

| 回路 | 行为 |
|---|---|
| 设备目录 | 注册失败或心跳失败后按 1/2/4/8/16/30 秒加 0–25% 抖动重试。成功后 20 秒心跳，并 `GET /api/muse/devices`。 |
| 出站附着 | 同样的退避。连接持续超过 30 秒后再断开，下一次从 1 秒开始。每次重连读取当前 access token，登录刷新后的 token 会用在新连接上。 |
| 退出登录 | 立刻停止心跳和附着，并关闭 socket。 |

服务端在 socket 关闭、pong 超时或 token 失效时立刻删路由，避免把新请求送进死连接。

Mobile 侧：

- 发现设备靠目录，不靠附着。Desktop 心跳恢复后，Mobile 最迟在自己的下一次 20 秒列表刷新里看到 online。
- `connectSameAccount` 对 `RELAY_DETACHED`、`RELAY_BUSY`、`RELAY_TIMEOUT`、
  `PAIRED_DESKTOP_UNAVAILABLE`、`PAIR_TIMEOUT` 以及连接失败，在 45 秒内按 1/2/4/8 秒重试。
  `ACCOUNT_MISMATCH`、`SIGNED_OUT`、`WORKSPACE_GRANT_DENIED` 不重试。
- 45 秒覆盖 Desktop 一次封顶退避加上重新 hello。用户停留在连接动作上时，网络恢复后不必重启 Desktop 或重装 Mobile。

在线只表示心跳新鲜。附着尚未完成时，连接会在上面的窗口里等到 `hello-ok`。窗口结束后才对用户显示失败。

## 8. 部署

操作步骤、一键脚本和「改了哪段逻辑要重部署哪些服务」写在
[DEPLOY.zh-CN.md](DEPLOY.zh-CN.md)。本文只保留协议。生产机上的转发进程和 nginx
是唯一对外的服务，开发机不参与。

## 9. 验收

| ID | 场景 | 期望 |
|---|---|---|
| N1 | 匿名 `GET /api/muse/devices` | 401，不是 404 |
| N2 | Desktop 登录公网账号并保持进程 | 目录中 online，且 `transportOrigin` 为配置的公网源（当前 `https://openmuseai.com:8443`） |
| N3 | 拔掉 Desktop 网络后恢复，进程不重启 | 附着与心跳自行恢复；Mobile 重新看到 online 并能 `account/open` |
| N4 | 附着未建立时 Mobile 发起连接 | 45 秒内附着恢复则成功，否则 `RELAY_DETACHED` |
| N5 | 另一账号对同一 deviceId 调用 `account/open` | 不进入该 Desktop，`ACCOUNT_MISMATCH` 或 `RELAY_DETACHED` |
| N6 | Mobile 打开会话 | WebView 的 origin 是配置的公网源，DSH 来自这台 Desktop 的本地网关 |
| N7 | 第二台同账号 Desktop 附着 | 替换自己的旧 socket，不影响另一台 deviceId |
| N8 | 退出登录 | 路由删除，Mobile 不能再转发到这台电脑 |

N3 的通过标准是进程不重启。只在局域网或 ADB reverse 下打开 `127.0.0.1:13180` 不算通过。
