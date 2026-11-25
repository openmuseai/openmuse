# 独立的 AppFlowy 实时传输与在线状态包（预览）

本包复用固定版本 AppFlowy Cloud WS v2 protobuf 和 Yrs Awareness，提供 Native/WASM 编解码、认证连接与短生命周期的在线状态。没有段落、表格、单元格、资源存储或编辑器 UI。文档、Dataset、Office 的领域适配器分别决定订阅、相对位置、重连、合并和检查点策略。

- `AppFlowyRealtimeConnection`：原 `/ws/v2/{workspace}` 二进制协议；认证 URI 不可输出到日志、UI 或缓存。连接本身不自动重试、不承诺持久 ACK。
- `AppFlowyRealtimeCodec`：保留原消息类型和字节，校验帧大小及整数窄化；UID、Client ID、Redis RID 使用字符串，避免浏览器数值精度丢失。
- `RealtimePresence`：原 Yrs 时钟/空状态离开协议；每个对象独立房间，注入单调时钟，30 秒投影过期；不写入内容更新或持久缓存。服务端负责将用户身份和客户端槽位绑定到认证会话，客户端提交的展示名不是身份凭证。
- 连接的 `actorUid` / `actorChanges` 来自同一 origin 的原认证 `GET /api/user/profile`，由 Rust 从原响应提取完整 i64 字符串，其他 profile 字段丢弃。请求限 64KiB / 5 秒，暂时失败最多再尝试两次；不阻塞 WS2 内容协作，未确认自身 UID 时 UI 不显示推测人数。断开/关闭立即清除身份并取消重试，旧 epoch 回包不能恢复旧身份。可信 Host 可注入 `actorIdentityLoader`；其结果同样校验规范 UID。
- Presence 的类型 6 是原协议 Unknown，仅允许用于 awareness 路由，不能解释为正文/行对象类型。

领域侧通过依赖注入组合本包和 `muse_appflowy_transport`。WS 更新用于低延迟交换；HTTP full-sync 用于显式接受和断线修复。Redis 消息 ID 不是写盘完成凭据；当前也没有 durable outbox/ACK 协议。完整协作验收尚未完成。

运行 `flutter test` 校验 Native 适配器；构建浏览器资产使用 Markdown 智能文档的 `scripts/build_realtime_web_core.py`。来源记录、补丁与授权信息位于该插件的 `vendor/appflowy_proto`，构建不修改上游或全局依赖缓存。
