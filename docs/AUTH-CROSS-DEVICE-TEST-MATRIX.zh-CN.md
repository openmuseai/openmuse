# 认证、Cloud Workspace 与 DSH 跨端测试矩阵

## 1. 验收边界

本轮不把“会话发现”伪装成任意 Desktop 本地目录发现。跨端验收以服务端身份为边界：

1. GoTrue JWT 只由认证插件持有并按需刷新；Workspace/DSH 插件只消费短期 token capability。
2. Cloud Workspace 只来自当前账号的 `GET /api/workspace`。
3. DSH 会话键由服务端使用 `accountRef + workspaceRef + tenantSalt` 确定性生成；客户端不能声明 `accountRef`。
4. Desktop 与 Mobile 对同账号、同 Workspace 调用 `open` 必须得到同一个 `sessionRef`，并在 `GET /api/muse/dsh/sessions` 中看到运行态投影。
5. 当前真机验收使用 ADB reverse 连接本机 GoTrue、Cloud API 与 DSH proxy；不代表生产网络拓扑。

## 2. 自动化测试

| ID | 层 | 用例 | 期望 | 结果 |
|---|---|---|---|---|
| AUTH-U1 | Auth plugin | 密码登录、恢复、刷新轮换、登出 | session 原子持久化；刷新 single-flight；本地登出权威 | 通过 |
| AUTH-U2 | Auth UI | 邮箱校验、密码页、错误、loading、可见性 | 与旧版布局和视觉 token 一致 | 通过 |
| SDK-U1 | Plugin SDK | 多认证贡献者选择 | 未显式选择时 fail closed | 通过 |
| CLOUD-U1 | Cloud adapter | `/api/workspace` envelope 映射 | 只接受 `code=0`；映射可写状态 | 通过 |
| CLOUD-U2 | Cloud adapter | 401 后强制刷新 | 最多重试一次且不持有凭据 | 通过 |
| CLOUD-U3 | Cloud Workspace plugin | Pool `ready` 会话投影 | `ready` 与兼容态 `running` 均显示为可复用的运行中会话 | 通过 |
| DSH-U1 | DSH Pool | 同账号、同 Workspace 重复 open | 相同 `sessionRef` / `instanceRef` | 通过 |
| DSH-U2 | DSH Pool | 账号会话列表 | 只返回该账号的 running/queued 投影，不泄露路径、端口、token、设备 ID | 通过 |
| DSH-U3 | DSH Pool | 跨账号 close/heartbeat | 不改变目标会话附件与活跃时间 | 通过 |
| HOST-U1 | Shared Host shell | running session 标记 | Workspace 列表显示 `DSH running` | 通过 |
| HOST-U2 | Desktop layout | Local DSH 与 Cloud Workspace | 两个 transport 面板可同时存在 | 通过 |
| MOBILE-U1 | Mobile distribution | 默认插件集合 | 默认加载 GoTrue 与 Cloud Workspace 插件 | 通过 |
| MOBILE-U2 | Remote DSH WebView | URL 与导航约束 | 只接受 HTTPS 或 debug loopback HTTP；只允许 `/u/`、`/session/` 与同源导航 | 通过 |
| DSH-U4 | DSH Pool local executor | 真实 runtime 启动与就绪探测 | 按配置启动 DSH，透传隔离目录/端口/Host auth；失败时回收子进程 | 通过 |

## 3. 本地集成测试

测试拓扑：GoTrue `127.0.0.1:9999`、OSS AppFlowy Cloud `127.0.0.1:8000`、DSH Pool control `127.0.0.1:13079`、DSH proxy `127.0.0.1:13080`。测试使用数据库中已有账号；报告不记录邮箱、密码、JWT 或 sessionRef。

| ID | 场景 | 断言 | 结果 |
|---|---|---|---|
| E2E-1 | 已有账号密码登录 | GoTrue 返回可用 session；Cloud 接受 JWT | 通过 |
| E2E-2 | 列出账号 Workspace | 返回 1 个当前账号 Workspace | 通过 |
| E2E-3 | Desktop open 后 Mobile open | 两次返回同一 `sessionRef` | 通过 |
| E2E-4 | 查询运行中会话 | 状态 `ready`，附件设备数 2 | 通过 |
| E2E-5 | 无 JWT 查询会话 | 请求被拒绝 | 通过 |
| E2E-6 | 非成员 Workspace open | envelope 返回 `SCOPE_MISMATCH` | 通过 |
| E2E-7 | 另一账号猜测 sessionRef 并 close | 原会话附件设备数仍为 2 | 通过 |
| E2E-8 | 真实 DSH runtime 创建会话并 prompt | `session.create`、`session.prompt` 成功，history 中出现 1 条匹配的 assistant 消息 | 通过 |

本地集成使用真实 DSH runtime、Pool、Cloud 鉴权和 proxy。由于外部 DeepSeek 测试账号返回配额不足（HTTP 402），消息闭环使用本地 OpenAI-compatible 确定性模型响应器；替换范围仅限模型供应商，不替换 DSH 的会话创建、prompt、持久化、Web UI、Pool、Cloud 鉴权或 proxy。客户端对返回的 `/u/` 或 `/session/` URL 执行同源且路径受约束的 transport。

## 4. 构建与设备测试

| ID | 场景 | 期望 | 结果 |
|---|---|---|---|
| BUILD-D1 | macOS Desktop debug build | 生成 `OpenMuse.app` | 通过 |
| BUILD-M1 | Android debug APK | 构建成功；SHA-256 `222d1cdaa983295608972e17261262edf331d8a24155d9634317e205fe01afe8`；205,766,002 bytes | 通过 |
| DEVICE-M1 | ADB 安装 | 当前 APK 安装成功 | 通过 |
| DEVICE-M2 | Android 冷启动 | 进入 OpenMuse 登录页，无崩溃 | 通过 |
| DEVICE-M3 | 真机同账号密码登录 | 显示当前账号的 Cloud Workspace | 通过 |
| DEVICE-M4 | 真机运行中会话投影 | 对已由 Desktop transport 打开的 Workspace 显示 `DSH running` | 通过 |
| DEVICE-M5 | 真机 Cloud DSH 消息闭环 | 打开受约束 WebView；发送 `MOBILE-CLOUD-E2E-20260930-0840`；显示用户消息与响应 `CLOUD-E2E-20260930-0840`，会话统计为 1 轮/1 步 | 通过 |
| DEVICE-D1 | Desktop 本机 UI 登录与恢复 | 同账号安全会话写入 Keychain；移除测试播种钩子并重建后仍可自动恢复 | 通过 |
| DEVICE-D2 | Desktop Cloud Workspace/DSH 投影 | 显示 `My Workspace`、`Remote DSH 运行中` 与“复用”入口 | 通过 |

## 5. 回归门槛

- Auth plugin、Cloud Workspace plugin、Mobile core/cloud、Host shell、Desktop Host、Mobile App 的 test 与 analyze 必须全部通过。
- Server `cargo check` 必须通过；DSH Pool test/build 必须通过。
- Android release 不允许明文 HTTP；本地 debug 的 loopback HTTP 必须同时满足显式 debug 配置与 loopback host。
- 任何日志、测试输出和文档不得包含密码、JWT、refresh token、DSH launch token 或用户本地绝对工作区路径。
