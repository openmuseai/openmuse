# OpenMuse 登录、GoTrue 鉴权插件迁移设计

> 状态：实施基线  
> 适用端：Android / iOS / macOS / Windows / Linux  
> 服务端策略：优先复用当前 AppFlowy Cloud、GoTrue 与 DSH Pool 契约；为“可发现正在运行的会话”补充最小、向后兼容的会话投影 API，并重新构建本地服务

## 1. 目标与结论

本文梳理 `Muse-Clients-Deprecated/frontend/client/frontend` 中已经跑通的登录和 GoTrue 鉴权链路，并给出向 `Muse-Client` 迁移的边界、接口、视觉规范、实施顺序和验收矩阵。

结论如下：

1. 登录适合实现为 Host 启动时加载的内建插件 `com.openmuse.auth.gotrue`。
2. 插件只负责认证领域：登录 UI、GoTrue 协议、会话恢复/刷新/退出和安全存储；它不拥有 Workspace、DSH 或窗口布局。
3. Host 需要增加一个很小的可选扩展点 `AuthenticationContributor`，由认证插件贡献响应式会话和登录门面。现有 Editor/Panel 插件 ABI 不需要改变语义。
4. Mobile 与 Desktop 显式安装同一个认证插件；Mobile 默认以登录门面作为应用入口，Desktop 在 Workbench 外增加相同门面。
5. 登录后的 Workspace 与 DSH 仍由独立 Cloud Workspace/DSH 适配器加载。访问令牌通过只读 token provider 注入，密码和 refresh token 不进入 Host、Workspace、DSH 或 Agent 领域。
6. 当前服务端没有“列出当前账号 DSH 会话”的接口。仅靠确定性 `open` 可以证明复用，却不能满足连接前“看到正在运行会话”的产品语义。因此增加 `GET /api/muse/dsh/sessions`，由 AppFlowy Cloud 完成账号鉴权，DSH Pool 只返回该账号的安全投影。
7. 跨端连续性使用两层验收：列表中能发现 Desktop 正在运行的 workspace session；Mobile attach/open 后得到相同 `sessionRef` 并附着到已有实例。
8. 迁移保留旧版的核心行为和视觉，但不会复制明文 token/passcode 日志、URL fragment token 传递等安全缺陷。

## 2. 范围

### 2.1 本次包含

- 邮箱 + 密码登录。
- GoTrue session 的内存态、安全持久化、启动恢复、提前刷新和退出清理。
- 旧版登录主页面、密码页面、错误提示、loading/disabled 状态和品牌视觉的一比一迁移。
- 第三方登录区域的布局与扩展位；仅对服务端确实启用且客户端具备安全回调能力的 provider 展示入口。
- Mobile 和 Desktop 的认证门面与账号态绑定。
- 复用既有 Cloud API 获取账号 Workspace。
- 同账号跨端复用既有 DSH session。
- 单元、Widget、契约、Desktop、Android 真机和端到端测试。

### 2.2 本次不包含

- 修改或迁移 GoTrue 身份数据与账号模型。
- 对既有 DSH open/close/heartbeat 接口做破坏性变更。
- 重置数据库中的账号密码。
- 让客户端绕过 Cloud/BFF 直接访问 DSH Pool 的内部会话状态。
- 将 refresh token 交给 DSH、Agent 或普通业务插件。
- 匿名账号合并、注册、找回密码和 magic link 的完整产品化；保留协议与 UI 扩展位，后续独立验收。

## 3. 旧版实现梳理

### 3.1 代码地图

旧版 Flutter 根目录：

`Muse-Clients-Deprecated/frontend/client/frontend/appflowy_flutter`

关键文件：

| 领域 | 旧版文件 | 职责 |
| --- | --- | --- |
| 认证端口 | `lib/user/application/auth/auth_service.dart` | 定义密码、passcode、OAuth、guest、sign-out、current user |
| 后端桥接 | `lib/user/application/auth/backend_auth_service.dart` | 将 Dart payload 转发给 Rust event |
| Cloud 实现 | `lib/user/application/auth/af_cloud_auth_service.dart` | Flutter 侧 Cloud auth service 组合 |
| 页面状态 | `lib/user/application/sign_in_bloc.dart` | 登录方式切换、提交、错误和完成事件 |
| 页面入口 | `lib/user/presentation/screens/sign_in_screen/sign_in_screen.dart` | Desktop/Mobile 页面选择 |
| Mobile 页面 | `.../mobile_sign_in_screen.dart` | 手机布局与设置入口 |
| Desktop 页面 | `.../desktop_sign_in_screen.dart` | 桌面布局与匿名入口 |
| 邮箱入口 | `.../continue_with_email.dart` | 邮箱输入和下一步 |
| 密码入口 | `.../continue_with_password.dart` | “Continue with password”入口 |
| 密码页面 | `.../continue_with_password_page.dart` | 邮箱 + 密码登录表单 |
| 品牌标题 | `.../title_logo.dart`、`.../logo/logo.dart` | Logo 和 Welcome 标题 |
| 协议提示 | `.../sign_in_agreement.dart` | Terms / Privacy 文案 |
| Rust 用户事件 | `rust-lib/flowy-user/src/event_handler/user_event.rs` | 接收登录事件并调用 UserManager |
| Rust Cloud auth | `rust-lib/flowy-user/src/services/auth/appflowy_cloud.rs` | GoTrue 登录、Cloud token 验证和用户生命周期 |
| GoTrue client | `rust-lib/flowy-cloud/src/client.rs` | `/token`、刷新、用户、登出等 HTTP 调用 |

### 3.2 旧版运行链路

```text
SignInScreen
  -> SignInBloc
  -> AuthService / BackendAuthService
  -> Flutter-Rust event: UserEventSignInWithEmailPassword
  -> UserManager.sign_in_with_password
  -> AFCloudUserAuthServiceImpl.sign_in_with_password
  -> GoTrue POST /token?grant_type=password
  -> AppFlowy Cloud GET /api/user/verify/{access_token}
  -> token state + login callback
  -> profile/workspace 初始化
  -> authenticated application shell
```

旧版没有把 GoTrue token 响应直接当成“应用已登录”。`SignInBloc` 将响应交给 deep-link/login-callback 流程，Rust 用户生命周期在 Cloud 校验成功后初始化用户、profile 和 workspace。这一点必须保留其语义：

- GoTrue 成功代表身份凭证成立；
- Cloud profile/workspace bootstrap 成功代表业务会话可用；
- 两阶段错误必须分别呈现，不能把 Cloud 不可用误报为密码错误。

### 3.3 GoTrue 协议

既有 GoTrue 使用下列标准端点：

| 操作 | HTTP | 请求/认证 | 结果 |
| --- | --- | --- | --- |
| 密码登录 | `POST /token?grant_type=password` | JSON `{email,password}` | access/refresh token、过期时间、user |
| 刷新 | `POST /token?grant_type=refresh_token` | JSON `{refresh_token}` | 轮换后的完整 session |
| 当前用户 | `GET /user` | `Authorization: Bearer <access>` | GoTrue user |
| 登出 | `POST /logout` | Bearer access token | 服务端撤销（若实现支持） |
| 服务设置 | `GET /settings` | 无 | 已启用登录/provider 能力 |

Token 响应的核心字段为：

```text
access_token, token_type, expires_in, expires_at,
refresh_token, user,
provider_access_token?, provider_refresh_token?
```

迁移实现必须容忍 `expires_at` 缺省并由 `expires_in` 推导；不得持久化 provider access token，除非对应 provider 的独立需求明确要求。

### 3.4 页面状态机

```text
boot
  -> restoring session
      -> authenticated
      -> signed out
      -> recoverable error -> signed out + message

signed out
  -> email entry
  -> password page
      -> submitting
          -> GoTrue rejected -> password page + field/form error
          -> GoTrue accepted, Cloud rejected -> service error + retry
          -> bootstrap accepted -> authenticated

authenticated
  -> access token near expiry -> refreshing -> authenticated
  -> refresh rejected -> clear secrets -> signed out
  -> sign out -> best-effort remote logout -> clear secrets -> signed out
```

并发刷新必须 single-flight。所有等待者共享同一次 refresh，防止轮换 refresh token 被并发消费。

### 3.5 旧版视觉基线

迁移以旧版页面为视觉基线，不把 Desktop Workbench 的复杂布局带入 Mobile 登录页。

| 元素 | 基线 |
| --- | --- |
| 表单最大宽度 | 320 dp |
| 主页面 Mobile padding | vertical 38 / horizontal 40 |
| Logo | Mobile 40×40；Desktop 36×36 |
| 主标题 | `Welcome to OpenMuse` |
| 间距 token | 4 / 6 / 8 / 12 / 16 / 20 |
| L 按钮 | horizontal 16 / vertical 10 / radius 10 |
| L 输入框 | horizontal 8 / vertical 10 / radius 10 |
| 主文字 | `#21232A` |
| 次文字 | `#6F748C` |
| 弱文字 | `#989EB7` |
| 边框 | `#E4E8F5` |
| 品牌/焦点 | `#00B5FF` |
| primary hover | `#0092D6` |
| error | `#E71D32` |
| 背景 | `#FFFFFF` |

主页面顺序：Logo + 标题、邮箱输入、`Continue with email`、描边 `Continue with password`、可用的第三方登录区域、Terms/Privacy。Desktop 保留其底部设置/匿名扩展区域，Mobile 保留设置/切换 Cloud 扩展区域，但未实现动作必须禁用或隐藏，不能提供假入口。

密码页顺序：Logo + 标题、当前邮箱、密码输入（可见性切换）、忘记密码扩展位、primary Continue、Back。

### 3.6 不应复制的旧版问题

- 日志输出 passcode、access token、refresh token 或完整 session JSON。
- 通过 URL fragment 传递 access/refresh token。
- JSON 解析失败时把原始 token 内容写入日志。
- 仅清理 UI 状态却保留本地 refresh token。
- 把 Cloud bootstrap 失败归类为 invalid credentials。

“一比一保留”适用于用户可见逻辑与样式，不适用于这些安全缺陷。

## 4. 当前 Muse-Client 差距

1. `openmuse_plugin_sdk` 只有 editor/panel contribution，没有认证或 application gate。
2. `OpenMuseSessionPort` 是静态 getter，不能通知 token 刷新、登出和账号切换。
3. Mobile composition 默认是 signed-out 占位态，未提供实际登录入口。
4. Desktop 直接启动本地 Workbench，没有账号 gate。
5. 当前 `HttpCloudWorkspaceService` 使用 `/v1/workspaces`、`/v1/dsh/sessions` 等目标架构接口，与现有 AppFlowy Cloud 的真实接口不一致。
6. 当前 AppFlowy Cloud 提供 `/api/workspace` 和 `/api/muse/dsh/session/open`，但没有 account-scoped session list，需要补齐安全投影。

## 5. 目标领域与依赖方向

```text
App Composition (Mobile / Desktop)
        |
        +--> Host Plugin Registry
        |       |
        |       +--> Auth GoTrue Plugin
        |               |
        |               +--> Auth Domain
        |               +--> GoTrue Provider
        |               +--> Secure Session Store
        |               +--> Login UI
        |
        +--> Account-scoped Cloud Workspace Adapter
        |       |
        |       +--> accessToken() only
        |
        +--> DSH Session Adapter
                |
                +--> Cloud-issued device/session API
```

依赖规则：

- Auth domain 不依赖 Flutter Widget、Workspace、DSH 或具体 secure-storage 包。
- GoTrue provider 实现 Auth domain 的 provider port。
- 登录 UI 只调用 Auth controller，不直接发 HTTP。
- Host 只观察公开的认证快照，不读取 refresh token。
- Workspace 只接收短期 access token provider。
- DSH 只接收 Cloud 返回的 DSH session/device 凭证；不能接收 GoTrue refresh token。
- 普通插件只能申请 `auth.identity.read` 等最小能力，不允许读取秘密。

## 6. Plugin 设计

### 6.1 描述符

```text
id: com.openmuse.auth.gotrue
kind: builtIn
activation: startup
platforms: android, ios, macos, windows, linux
permissions:
  - network.auth
  - secrets.session
contributions:
  - authenticationProvider: gotrue
  - applicationGate: login
```

这是发行版内建插件，而不是可以下载任意代码的第三方插件。原因是它持有长期会话秘密，必须由官方签名、固定依赖和平台安全存储策略保护。

### 6.2 SDK 最小扩展

```dart
abstract interface class OpenMuseAuthenticationContributor {
  OpenMuseAuthenticationController get authentication;
  Widget buildAuthenticationGate({
    required BuildContext context,
    required Widget authenticatedChild,
  });
}

abstract interface class OpenMuseAuthenticationController
    implements Listenable {
  OpenMuseAuthenticationSnapshot get snapshot;
  Future<void> restore();
  Future<void> signInWithPassword(String email, String password);
  Future<String?> accessToken({bool forceRefresh = false});
  Future<void> signOut();
}
```

该扩展是可选 contribution：不安装认证插件的本地/离线发行版仍可启动；安装多个认证插件时 composition root 必须显式选择一个，Registry 不做隐式优先级猜测。

### 6.3 Auth domain 模型

- `AuthUser`: id、email、必要 metadata；不携带密码。
- `AuthSession`: access token、refresh token、expiresAt、user。
- `AuthState`: restoring / signedOut / submitting / bootstrapping / authenticated / refreshing / failure。
- `AuthFailure`: invalidCredentials、network、server、invalidResponse、sessionExpired、bootstrap、cancelled。
- `AuthProvider`: password login、refresh、current user、logout、settings。
- `AuthSessionStore`: read/write/delete；实现必须原子替换轮换后的 session。
- `CloudAccountBootstrap`: 验证 Cloud 账号并加载 profile/workspace，作为认证完成的第二阶段。

### 6.4 秘密所有权

| 数据 | 所有者 | 可见范围 |
| --- | --- | --- |
| 密码 | 登录表单瞬时内存 | Auth UI -> provider 单次调用 |
| access token | Auth controller | 只通过异步 provider 给 Cloud adapter |
| refresh token | Auth plugin + secure store | 不公开 |
| GoTrue user id/email | Auth domain | Host 可读身份快照 |
| Workspace id | Workspace domain | Workspace/DSH |
| DSH device/session token | DSH adapter | DSH transport |

日志必须使用结构化错误分类，只记录 request id、状态码和安全的 endpoint 名称，不记录 Authorization、密码或响应 body 中的 token。

## 7. 平台装配

### 7.1 Mobile

启动顺序：

1. 加载发行版内建插件清单，认证插件默认启用。
2. 初始化平台 secure store。
3. `restore()`，期间显示与旧版一致的轻量 splash/loading。
4. 未登录显示 Mobile 登录页；成功后创建 account-scoped composition。
5. 使用 access token 调用既有 Cloud `/api/workspace`。
6. 选中 Desktop 已使用的 workspace，调用 `/api/muse/dsh/session/open`。
7. 先从会话列表标记 Desktop 正在运行的 workspace；用户进入后 attach/open，服务端返回同一 `sessionRef` 时显示“已连接正在运行的会话”，并进入现有 Mobile window/surface。

Android 本地联调使用 `adb reverse` 暴露宿主机端口；debug 构建可以使用 loopback HTTP，release 必须是 HTTPS 且拒绝明文 endpoint。

### 7.2 Desktop

Desktop 保留现有 Workbench 产品形态：

1. 在 Workbench 外增加认证 gate，不重写 Workbench 内部窗口/插件布局。
2. 登录成功后由 account scope 注入 Cloud workspace catalog。
3. 本地 workspace 和 cloud workspace 仍是不同 provider；账号只决定可访问的 cloud catalog。
4. Desktop 为目标 workspace 打开 DSH session，记录非秘密的 `sessionRef` 到 session model。
5. Mobile 同账号、同 workspace open 时由服务端复用该 session。

### 7.3 Endpoint 配置

Auth plugin 接收显式配置：

- `gotrueOrigin`
- `cloudOrigin`
- `allowInsecureLoopback`（仅 debug/test）

校验规则：

- 生产环境只允许 HTTPS。
- debug 的 HTTP 仅允许 loopback 或 emulator 映射地址。
- 禁止 endpoint 携带 user-info、query、fragment。
- origin 规范化后再拼接固定 path，避免 token 被发往任意 URL。

## 8. 服务端契约映射与最小扩展

### 8.1 Cloud envelope

既有接口返回：

```json
{
  "data": {},
  "code": 0,
  "message": ""
}
```

`code == 0` 才是业务成功。HTTP 2xx 但业务 code 非 0 必须转换为 typed failure。

### 8.2 Workspace

使用 `GET /api/workspace`，Bearer GoTrue access token。关键字段：

- `workspace_id`
- `database_storage_id`
- `owner_uid`、`owner_name`、`owner_email`
- `workspace_type`、`workspace_name`
- `created_at`、`icon`
- `member_count?`、`role?`

客户端适配器负责将其转换为 `WorkspaceDescriptor`，不要求服务端提供当前客户端 `/v1/workspaces` 形状。

### 8.3 既有 DSH 契约

使用现有 `/api/muse`：

- `POST /api/muse/workspace/current`
- `POST /api/muse/workspace/tree`
- `POST /api/muse/dsh/device-token`
- `POST /api/muse/dsh/session/open`
- `POST /api/muse/dsh/session/close`
- `POST /api/muse/dsh/session/heartbeat`

`session/open` 的服务端身份键是 `(accountRef, workspaceRef)`。当已有 session 处于 starting/ready/idle 时，新设备附着到相同 session/process。响应可包含：

- `sessionRef`
- `instanceRef?`
- `webUrl?`
- `expiresAt?`
- `queuePosition?`
- `retryAfterMs?`
- `nodeId`

确定性复用仍是跨端 attach 的最终证据：

1. Desktop open workspace A，取得 session S；
2. 保持 Desktop session alive；
3. Mobile 用同账号 open workspace A；
4. Mobile 取得同一 session S（ready 时 instanceRef 也一致）；
5. Mobile 显示 resumed/attached 状态并连接 S。

不同账号或不同 workspace 不得复用同一 session。

### 8.4 新增账号级会话投影

AppFlowy Cloud 新增：

```http
GET /api/muse/dsh/sessions
Authorization: Bearer <gotrue-access-token>
```

Cloud 从已验证 identity 取得 `accountRef`，调用只在内网开放的 DSH Pool：

```http
POST /internal/session/list
Content-Type: application/json

{ "accountRef": "<verified-user-uuid>" }
```

客户端响应继续使用 Cloud envelope，`data` 为投影数组：

```json
[
  {
    "sessionRef": "session.…",
    "workspaceRef": "workspace-uuid",
    "state": "ready",
    "instanceRef": "instance.…",
    "nodeId": "local",
    "createdAt": "2026-09-29T10:00:00Z",
    "lastActiveAt": "2026-09-29T10:03:00Z",
    "attachedDeviceCount": 1
  }
]
```

安全与兼容约束：

- `accountRef` 只能由 Cloud 的认证上下文生成，客户端 query/body 不能指定账号。
- DSH Pool 必须按原始 account identity 或不可逆 tenant/account 索引过滤；禁止先返回全量再由客户端过滤。
- 响应不包含 homeDir、容器名、宿主端口、内部 IP、device id、token 或环境变量。
- 默认只返回 `starting|ready|idle|queued`；已过期/failed 仅在短暂诊断窗口内可选返回。
- 列表是 discovery/read model，不改变 `open` 的幂等和确定性复用语义。
- 原有 open/close/heartbeat 路由和响应保持不变。
- AppFlowy Cloud 和 DSH Pool 分别增加鉴权、租户隔离、字段脱敏与回归测试，再重建本地服务镜像/进程。

## 9. 错误与恢复策略

| 场景 | UI | 状态处理 |
| --- | --- | --- |
| 邮箱/密码错误 | 表单内错误，停留密码页 | 不持久化 session |
| GoTrue 网络错误 | 可重试 service error | 保留邮箱，不保留密码 |
| GoTrue 成功、Cloud bootstrap 失败 | “登录成功但工作区服务不可用” | session 可暂存于内存；持久化策略必须等待 bootstrap 成功 |
| access token 过期 | 静默 single-flight refresh | 原请求等待一次并重试 |
| refresh 失效 | 会话过期提示并回登录页 | 删除 secure store |
| secure store 数据损坏 | 回登录页 | 删除损坏条目，不输出原文 |
| DSH 排队 | 显示 queuePosition/retryAfter | 保持 workspace 上下文 |
| DSH 不可用 | Workspace 仍可浏览 | DSH 独立重试，不登出账号 |

## 10. 分支与实施序列

每个阶段从上一个已验收提交 checkout 新分支，独立设计、实现和验收。

### A1 — 旧版梳理与迁移设计

- 分支：`feature/auth-plugin-migration-design`
- 产物：本文档。
- 验收：旧链路、真实服务端 API、安全差异、插件边界、跨端 session 语义和测试矩阵完整。

### A2 — Auth SDK 扩展与 GoTrue 插件核心

- 分支：`feature/auth-plugin-core-gotrue`
- 产物：可独立测试的 auth domain、GoTrue HTTP provider、session store port、controller、plugin contribution。
- 验收：密码登录、恢复、提前刷新、并发刷新、登出、错误映射和敏感日志测试通过。

### A3 — 旧版 UI/样式迁移

- 分支：`feature/auth-plugin-ui-parity`
- 产物：Mobile/Desktop 登录主页面和密码页、品牌资源、视觉 token、Widget/golden 测试。
- 验收：320dp 表单、断点、键盘/滚动、loading/error、浅色视觉与旧版基线一致。

### A4 — Mobile 默认加载

- 分支：`feature/auth-plugin-mobile-binding`
- 产物：Mobile composition、平台 secure store、auth gate、debug endpoint 与 Android 配置。
- 验收：未登录启动进入登录页；重启恢复；登出清理；Android 构建通过。

### A5 — Desktop 加载

- 分支：`feature/auth-plugin-desktop-binding`
- 产物：Workbench 外层 auth gate、账号态和 Cloud catalog 注入。
- 验收：同一账号可登录；Workbench 行为不回退；账号切换清理 account scope。

### A6 — Cloud Workspace / DSH 会话投影与客户端适配

- 分支：`feature/auth-existing-cloud-workspace-dsh`
- Server 分支：`feature/account-dsh-session-projection`（在对应 Server/DSH Pool 仓库）
- 产物：`/api/workspace` envelope adapter、`GET /api/muse/dsh/sessions`、内部 account-scoped list、既有 `/api/muse/dsh/session/open` adapter、跨端 discovery + attach 状态。
- 验收：Desktop/Mobile 同账号看到同一 workspace；Mobile 列表能发现 Desktop 的运行会话；attach 后同 workspace 返回相同 sessionRef；另一账号不可枚举或附着。

### A7 — 集成与真机候选版

- 分支：`feature/auth-mobile-desktop-final-candidate`
- 产物：完整测试报告、Desktop 构建、Android APK、ADB 安装与关键路径证据。
- 验收：下列测试矩阵的 P0 全部通过，P1 无阻塞缺陷。

## 11. 测试矩阵

### 11.1 单元与契约

| ID | 优先级 | 用例 | 预期 |
| --- | --- | --- | --- |
| AUTH-U01 | P0 | password token 成功 | session/user 正确解析 |
| AUTH-U02 | P0 | invalid credentials | typed failure；不写 store |
| AUTH-U03 | P0 | 启动恢复未过期 session | authenticated |
| AUTH-U04 | P0 | access 临期 | 刷新并原子保存轮换 token |
| AUTH-U05 | P0 | 10 个并发 token 请求 | 只有一次 refresh HTTP 请求 |
| AUTH-U06 | P0 | refresh 被拒绝 | 清 store 并 signedOut |
| AUTH-U07 | P0 | logout | best-effort remote + 必定清本地 |
| AUTH-U08 | P0 | 损坏 session JSON | 安全清理；日志无原文 |
| AUTH-U09 | P0 | 非法/明文生产 endpoint | 配置拒绝 |
| AUTH-C01 | P0 | `/api/workspace` code 0 | descriptor 映射正确 |
| AUTH-C02 | P0 | HTTP 200 + 业务错误 code | typed cloud failure |
| DSH-C01 | P0 | session open ready | sessionRef/instanceRef/webUrl 解析 |
| DSH-C02 | P0 | session open queued | queue/retry 字段解析 |

### 11.2 Widget / 视觉

| ID | 优先级 | 尺寸/场景 | 预期 |
| --- | --- | --- | --- |
| AUTH-W01 | P0 | 390×844 手机 | 无横向溢出；主结构与旧版一致 |
| AUTH-W02 | P0 | 320×568 小屏 + 键盘 | 可滚动且提交按钮可达 |
| AUTH-W03 | P1 | 折叠屏单 pane | 表单仍限宽 320 并居中 |
| AUTH-W04 | P1 | Desktop 1440×900 | gate 居中；Workbench 未泄露 |
| AUTH-W05 | P0 | submitting | 输入/按钮禁用且只提交一次 |
| AUTH-W06 | P0 | invalid password | 错误可读、焦点合理、密码不回显 |

### 11.3 既有服务与跨端集成

| ID | 优先级 | 步骤 | 预期 |
| --- | --- | --- | --- |
| AUTH-I01 | P0 | 真实测试账号登录 GoTrue | token 成功且日志无秘密 |
| AUTH-I02 | P0 | token 访问 Cloud workspace | 返回账号 workspace |
| AUTH-I03 | P0 | Desktop 与 Mobile 同账号 | workspace id 集合一致 |
| AUTH-I04 | P0 | Desktop open workspace A | session S ready/running |
| AUTH-I05 | P0 | Mobile 获取会话列表 | 能看到 workspace A 的 session S |
| AUTH-I06 | P0 | Mobile open 同一 workspace A | 返回并连接同一 session S |
| AUTH-I07 | P0 | Mobile open workspace B | 不复用 A 的 session |
| AUTH-I08 | P0 | 另一账号请求会话列表/open A | 看不到 S；无权限或不同 session，绝不越权复用 |
| AUTH-I09 | P1 | access token 刷新后访问 workspace | 无感恢复，不重建 DSH session |

### 11.4 构建与设备

| ID | 优先级 | 用例 | 预期 |
| --- | --- | --- | --- |
| BUILD-01 | P0 | 全仓 analyze/test | 通过 |
| BUILD-02 | P0 | Desktop debug/release build | 通过并可登录 |
| BUILD-03 | P0 | Android APK build | 通过 |
| DEVICE-01 | P0 | `adb install -r` | 安装成功 |
| DEVICE-02 | P0 | cold start signed out | 出现登录页 |
| DEVICE-03 | P0 | 真机登录已有账号 | 进入 workspace UI |
| DEVICE-04 | P0 | kill/restart | secure session 恢复 |
| DEVICE-05 | P0 | Desktop 先启动 DSH，Mobile 后连接 | 同 sessionRef，Mobile 可操作 |
| DEVICE-06 | P0 | logout + restart | 回登录页，旧 token 不可复用 |

真实账号密码只通过运行时环境变量、交互输入或设备输入提供；不得写入 Git、测试快照、命令历史或测试报告。

## 12. 完成定义

本需求完成必须同时满足：

- 认证是独立内建插件，并被 Mobile/Desktop composition 显式加载。
- 旧版密码登录核心行为与视觉被保留。
- session 安全存储、恢复、刷新和退出均有自动化验证。
- 客户端兼容当前 GoTrue/AppFlowy Cloud/DSH API；新增 session list 是向后兼容的可发现性扩展。
- Android 真机使用已有账号登录并显示与 Desktop 相同的 workspace。
- Desktop 正在运行的目标 workspace DSH session 可先被 Mobile 发现，再被复用，并以相同 `sessionRef` 形成证据。
- P0 自动化、构建和关键真机路径通过；报告不含任何密码或 token。
