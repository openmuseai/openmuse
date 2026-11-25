# Mobile / Desktop 会话连接与延迟修复

日期：2026-11-11。本文区分已实现的修复、后续协议演进和仍需真机测量的指标。

## 当前真实架构

```text
Mobile ── WebSocket /api/muse/devices/events ── 设备控制面
       └─ HTTP heartbeat /api/muse/devices/heartbeat
Desktop ─ 同一设备目录控制面（账号、presence）
Desktop ─ 出站 WebSocket（hello protocol=1，HTTP/WS 多路转发）── paired relay
Mobile ─ HTTPS /v1/account/open → grant
       └─ /u/<grant> → cookie → /openmuse-native/v1/* HTTP + follow SSE
          → relay → Desktop PairedDesktopGateway → loopback DSH bridge
          → DSH session controller / 权威 journal
```

已有常驻控制通道，不能把这次问题归因于“完全没有控制通道”：

- `plugins/workspace-paired/lib/src/account_device_directory.dart` 的设备目录 WebSocket 已带 15 秒 ping，注册后每 20 秒 HTTP heartbeat，失败后指数退避与 jitter；它服务设备发现/presence，不承载 DSH 消息。
- `desktop_outbound_relay.dart` 已持有一条 Desktop 出站 WebSocket，15 秒 ping、握手超时、退避重连，并按请求 id 转发 HTTP chunk 与 WebSocket frame。
- Mobile 的 DSH 数据面原来是页面自己的 HttpClient + bootstrap cookie + hello + negotiate + sessions + SSE。设备目录在线不代表某个 DSH follow 已就绪。HTTP keep-alive、账号授权 grant、SSE 订阅是三个不同生命周期。
- 原生协议已有 `session/page`，支持固定 throughSeq 和 beforeSeq。原先服务端每次 follow/page 请求的是最多 500 条 message 的窗口、最少 50 条/2 turn，**不是无限全量历史**；但 Mobile 未实现向前分页。

## 根因与本次改动

| 症状 | 代码根因 | 已实现 |
|---|---|---|
| 新会话发消息后等待 | WorkBuddy 发送前无条件 `connectDevice(force: true)`，更换 grant；又独立 bootstrap、create、刷新 catalog，再初始化会话 | 使用有效 grant 和设备级 HTTP 池；连接到设备时预热 hello/协商；创建期间立即显示待发送文本和进度；仍等 Desktop 权威 create 成功才认定会话存在 |
| 切会话又连接 | WorkBuddy 有 4 个页面的内存保留，但隐藏时立即 pause；超出 4 个就丢消息视图；旧 NativeDesktopShell 每次重建页 | 保留 2 分钟再停 follow；恢复取消 idle timer；两种 shell 复用设备传输；最近 12 个会话各缓存最多 120 条 journal event（不是 120 条聊天消息） |
| 切后台回来全重连 | 超过 30 秒回前台无条件刷新授权；grant 是页面 key，因此全页失效 | 前台恢复复用有效 grant；即将过期继续提前 1 分钟续期；401/403 触发授权更新；普通网络错误由已有 follow/reconcile 重试修复 |
| 历史加载阻塞当前消息 | 尾部窗口较大，前端没有 prepend；快照刷新会清空已分页历史；每次消息变化强制滚到底部 | bridge 尾部/分页窗口改为 maxMessages 80、minMessages 20/minTurns 2；上滑接近顶部请求 beforeSeq；prepend 不回退 live cursor；连续历史在尾部刷新时保留；存在序号缺口时重新以尾部为起点补历史；补页保持视口，阅读旧消息时不自动滚到底部 |
| 半开连接长期挂起 | 原 SSE 没有应用层保活 | bridge 每 15 秒发送 SSE comment；客户端 45 秒无任何 SSE 行触发重试；JSON 请求仍有 20 秒超时；HTTP idle pool 保留 2 分钟 |

共享 HttpClient 的关闭权归 shell，单个 controller dispose 不关闭其他会话的连接。hello 合并并发初始化，失败可重试。协商结果缓存于当前 grant，不跨设备/授权复用。旧原生 shell 的瞬时 catalog 错误不再替换已有消息页。

缓存位于系统 application cache directory；账号/设备隔离，不写 grant、cookie 或 transport token；单页超过 128 KiB 跳过，最多 12 页，总读取限制 2 MiB。500ms 合并更新，串行临时文件 + rename；损坏/不可读缓存按 miss 处理。系统可清除该目录。缓存只包括已确认 journal 与展示 projections，不持久化未确认 prompt 或进行中的 assistant delta，不自动重发不确定是否已接受的 prompt。缓存内容是应用沙箱内的 JSON，**本次未增加文件内容加密**。

本次缓存解决进入已知会话时的本地预览与页面淘汰后恢复；首次启动仍需恢复账号、取得设备授权和会话目录，尚未实现 Desktop 完全离线时的独立历史浏览。预览不授予发送权限，模型分享确认仍读取当前 Desktop recipient。

## 对比与方案选择

| 方案 | 能解决什么 | 对本项目的取舍 |
|---|---|---|
| Tailscale：控制面与数据面分离，先 relay 后升级直连 | relay 立即可用，NAT 探测成功后减少路径 RTT | 保留现有 relay 兜底；未来在后台探测 LAN / UDP 路径，不让 ICE 探测挡首条消息。直连只能减少网络路径延迟，无法修复重复授权/初始化 |
| WebRTC DataChannel + ICE/STUN/TURN | 数据直连、跨 NAT、失败 TURN 兜底 | 可作为后续数据面；需要信令、候选交换、DTLS、网络切换与请求多路复用适配。首屏不应等待 ICE 完成。当前不引入第二套传输栈 |
| Signal：本地消息历史 + 与已关联设备同步 | 历史读取不依赖当前网络，已关联设备接收新消息 | 采用“最近本地视图 + Desktop journal 权威 + 增量/分页”方向；不照搬 Signal 协议，也不声称现有 relay 已具备 Signal 的 E2EE |
| 本次：设备级 HTTP 池 + 复用授权 + 预热 + SSE + 有界缓存 | 消除应用额外串行往返，改善热切换与已知会话首屏 | 可以兼容现有 paired relay 和 DSH controller；已实现 |

官方参考：

- [Tailscale encryption](https://tailscale.com/docs/concepts/tailscale-encryption)：控制面/数据面、DERP 起步再升级直连。
- [Tailscale DERP 协议](https://github.com/tailscale/tailscale/blob/main/derp/README.md)：home relay 常驻与 discovery。
- [WebRTC peer connections](https://webrtc.org/getting-started/peer-connections)：ICE/STUN/TURN 与独立信令。
- [Signal linked-device history](https://signal.org/blog/a-synchronized-start-for-linked-devices/)：设备本地历史与加密历史转移。
- [Apple background execution](https://developer.apple.com/documentation/uikit/extending-your-app-s-background-execution-time)：普通 iOS 应用后台后会被挂起，不能承诺永久 socket 保活。这里的 2 分钟保留是进程有执行机会时的导航/短时切换宽限；长期后台应由 APNs 提示有变化，前台后恢复通道。

后续协议建议：复用设备目录控制面进行 DSH ready/headSeq invalidation 通知；增加带 sessionId/cursor 的多会话订阅、明确 ack 与请求幂等状态查询；在同一数据通道多路复用以减少每会话 SSE；LAN/直连与 relay 竞速，relay 首先可用就展示，直连成功后迁移。它们涉及服务器和运行时契约，属于后续工程，没有在本次宣称实现。

## 验证与上线

本次新增测试覆盖：短时 idle/resume 不重开 follow；分页与实时事件竞争不回退 cursor；尾部刷新保留已加载历史；缓存截断/hasMore；磁盘恢复和账号隔离。另跑原有 core、native UI、mobile shell、bridge gateway 回归与定向静态检查。

bridge 已更新至 0.1.18，重新打包 tgz 并更新 DSH closure 依赖/lock；Desktop 需重新构建/发布，才能启用更小窗口和 SSE heartbeat。仅升级 Mobile 也能获得授权复用/缓存/分页，但旧 bridge 在空闲超过 45 秒时会被新客户端探测并重开 follow。

尚未做真实手机与跨网 Desktop 的性能基准，不能宣称已达到某个毫秒值。真机验收需记录冷启动、前后台 5s/30s/5min、切换 2/12 个会话、长历史、Desktop 休眠/唤醒、网络切换：点击/提交 → 本地文本首帧 → grant → bootstrap/hello → follow 首帧 → Desktop prompt accepted → 首个 assistant token；报告 P50/P95、请求数、grant 次数和 reconnect 次数。目标：有效授权内普通切换 grant 请求为 0，2 分钟内热切换 follow 重建为 0，缓存预览不等待 follow，历史仅在上滑时向前读取。

后续 trace 与直连升级协议设计见 [Relay 优先、后台直连升级与耗时诊断](RELAY-FIRST-DIRECT-UPGRADE-TRACE.zh-CN.md)。该文区分本次已实现的请求优化和待实现的 P2P 协议。
