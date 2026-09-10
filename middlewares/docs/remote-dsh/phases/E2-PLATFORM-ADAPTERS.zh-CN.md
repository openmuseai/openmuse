# E2 平台适配器 — 设计 / 实现 / 测试

> 阶段：E2。依赖 E0 的 stage/code 合同与 E1 的 bind ACK 语义。合同：[HOST-EMBEDDING-DESIGN.zh-CN.md](../HOST-EMBEDDING-DESIGN.zh-CN.md) §4。
> 平台：**Desktop**（映射，不改信任）+ **Android**（补齐 Placement）。Web 本阶段只抽共享类型若尚未抽到包。

---

## 1. 设计

### 1.1 问题

三端错误和打开链不一致：

- Desktop 已有 `launching/ready/lastError`，但 code 是英文 `StateError` 字符串；QA 用桌面「能开」验收 Web。
- Android `DshSessionApi` 已实现 `isQueued`，Shell **不调用**；WebView 加载编译期 origin。现网 `/dsh/` 502 后 Android 远程演示必挂。
- `MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST` + 进程内 `HOST_IN_USE`：池已一租户一进程，整机锁是错误层。

### 1.2 决策

共享合同：`dsh-attachment-error-codes.v1.json` + stage 名。各端自己的 UI 文案必须引用同一 `code`。

**Desktop**

```text
NEED_API_KEY → 现有缺 DEEPSEEK_API_KEY
Placing      → ensureStarted
Presenting   → WebView 加载 dsh web: URL（必须带 launch token 若日志里有）
Binding      → DshWorkspaceBridge.publish 成功即 ACK
HybridLive   → ready==true 且 hint 已写
SIDECAR_EXIT → 进程非 0 退出
```

不启用 `MUSE_DOCUMENT_CLOUD_URL`、不发 `session/open`。文档与测试标题写明 **Desktop 不验收 session/open / postMessage**。

**Android**

```text
DshMobileShellPage 增加 sessionApi + accessToken 来源（ControlHost）
connect():
  ResolvingIdentity → JWT
  Placing → DshSessionApi.open（Queued 则等待 retryAfterMs）
  Presenting → DshRemoteConfig.fromWebUrl(webUrl)
  Binding → 现有 parent-hello HTTP（E1 语义：token 在 hello 里，Android 已由服务端注入）
```

编译期 `MUSE_DSH_PUBLIC_URL`：**只**作 origin allowlist（`fromWebUrl` 的 host 必须匹配）。禁止 `loadRequest(endpoint.publicUri)` 当生产路径。

Mobile lease：默认允许；`HOST_IN_USE` 仅当 **同一实例** 被另一 `deviceId` 独占且产品仍要单设备。跨租户不得 409（它们根本不在同一进程）。

后台：保持现有 degrade（控制面关、WebView 留）= `APP_BACKGROUNDED`，stage 回到 `Presenting` 而非 Closed。

### 1.3 不变量

| ID | 陈述 |
|---|---|
| C1 | Android `Presenting` 的 URI path 匹配 `/u/<hex>/` 或显式允许的测试 origin，不含生产 `/dsh/` |
| C2 | Android 在有 Cloud 登录时 `session/open` 调用次数 ≥1（同 E0-T1） |
| C3 | Desktop 测试套件零请求 ` /api/muse/dsh/session/` |
| C4 | 三端 Failed UI 的 `code` 属于合同 JSON |

### 1.4 不做

- 不把 Desktop WebView 改成远程池（除非单独立项）。
- 不在本阶段做 iOS。
- 不放开 Android 跳过 TLS。

---

## 2. 实现

| 路径 | 改动 |
|---|---|
| `muse-dsh-mobile/lib/src/dsh_mobile_shell_page.dart` | 接收 `DshSessionApi?`、`sessionWebUrl` 改由 coordinator open |
| `muse-dsh-mobile/lib/src/dsh_mobile_coordinator.dart` | Placement 先行；Queued 循环；origin allowlist |
| `appflowy_flutter/.../dsh_mobile_agent_page.dart` | 构造 `DshSessionApi`（Cloud origin + JWT） |
| `dsh_remote_config.dart` | 文档注释：compile-time ≠ page URL |
| `parent-bridge.ts` mobile 分支 | 去掉「无 exclusive flag 则 403」作为生产默认；或实例 env 生产设为与「实例内单 lease」一致 |
| `dsh_agent_controller.dart` | `stage` + `errorCode`；保留 `lastError` 文案 |
| `dsh_sidecar.dart` | 映射 NEED_API_KEY / SIDECAR_EXIT / FRAME_TIMEOUT |
| `dsh_agent_panel.dart` | 展示 code；黑窗回归（open=false 直到 restore） |
| `muse-dsh-mobile/test/dsh_session_api_test.dart` | 保持；补 coordinator 接线测 |
| `dsh_mobile_agent_page_test.dart` | 断言构造 sessionApi（mock） |

共享 JSON 可放 `middlewares/dsh/plugins/dsh-appflowy/contracts/`，Dart 侧继续手写同步（已有 mobile 合同模式），CI 用小脚本 diff key 集合。

---

## 3. 测试方案

- Dart：`DshSessionApi` 已有；新增 coordinator 测试：无 sessionApi → `NEED_AUTH` 或 configInvalid；有 API queued → 不立刻 load origin。
- Flutter Desktop：sidecar 缺 key → `NEED_API_KEY`；不 mock 网络 session。
- parent-bridge mobile：无 exclusive 时同一实例第二设备策略写进矩阵（选定后测 200 或 409）。
- 禁止真机打生产作为唯一证据。

---

## 4. 测试矩阵

| ID | 场景 | 期望 | 类型 |
|---|---|---|---|
| E2-T1 | Android open 200 + webUrl `/u/ab.. /?token=` | WebView load 该 URL，不是 compile-time origin | Dart |
| E2-T2 | Android open queued | 不 load WebView；展示 QUEUED；到时再 open | Dart 假时钟 |
| E2-T3 | Android 无 accessToken | 不调用 post session；`NEED_AUTH` | Dart |
| E2-T4 | Android webUrl host ≠ allowlist | 不 load；`DSH_CONFIG_INVALID` | Dart |
| E2-T5 | Android webUrl path `/dsh/` | 拒绝（生产合同） | Dart |
| E2-T6 | 关页面 | `session/close` 被调用 | Dart |
| E2-T7 | Desktop 缺 API key | `NEED_API_KEY`；无 WebView 或有输入框 | Flutter |
| E2-T8 | Desktop 正常 spawn | `dsh web:` URL 被 setUrl；ready true | 现有 sidecar 测扩展 |
| E2-T9 | Desktop 套件 | 无 `/api/muse/dsh/session` 的 http mock 调用 | 静态/测 |
| E2-T10 | 错误码 JSON key ⊆ 三端枚举 | CI diff | 脚本 |
| E2-T11 | 后台 degrade | WebView 仍在；code=`APP_BACKGROUNDED`；stage≠HybridLive | Dart |
| E2-T12 | 两租户两实例（旗标策略） | 不得因 A 占用而 B 的实例 409 | DSH 单测 |

---

## 5. 验收

| ID | 端 | 期望 |
|---|---|---|
| E2-P1 | Android 试点包 + 登录 | Network（代理）可见 `session/open` 与 `/u/` |
| E2-P2 | Desktop 本地 | 缺 key 有 code；填 key 后 WebView 非黑；无 session/open |
| E2-P3 | QA 清单 | 分列 Web/Desktop/Android，禁止串列验收 |

Android 无试点包时 E2-P1 可在模拟器打 staging BFF；不得用 compile-time `https://openmuseai.com/dsh/` 当通过条件。

---

## 6. 实现记录（2026-09-10）

| 项 | 状态 |
|---|---|
| `dsh_placement.dart` | tenant `/u/<32hex>/`；拒绝 `/dsh/`；host allowlist |
| `DshMobileCoordinator.connect` | 有 `sessionApi` 则 Placement 先行；Queued 按 `retryAfterMs` |
| `DshMobileAgentPage` | Cloud JWT + `requireRemoteSession`；禁止编译期 `/dsh/` 当页 |
| Desktop `DshDesktopError` | `NEED_API_KEY` / `SIDECAR_EXIT` / `FRAME_TIMEOUT`；面板展示 code |
| E2-T12 跨租户 409 | 两份 `ExclusiveMobileLease`（两实例）互不 409；旗标默认关 |

自动化：`dsh_placement_test` T1–T5；`dsh_coordinator_session_test` T1/T2/T3/T6/T11；Desktop T7–T9；Web T10 JSON keys；`ExclusiveMobileLease` T12。

