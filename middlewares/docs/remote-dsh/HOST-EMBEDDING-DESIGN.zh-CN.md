# Host 嵌入 DSH：目标设计与鲁棒性论证

> 状态：设计 v1（2026-09-10）。诊断见 [HOST-EMBEDDING-ANALYSIS.zh-CN.md](HOST-EMBEDDING-ANALYSIS.zh-CN.md)。开发计划见 [HOST-EMBEDDING-PLAN.zh-CN.md](HOST-EMBEDDING-PLAN.zh-CN.md)。阶段：[E0](phases/E0-WEB-ATTACHMENT-SM.zh-CN.md) / [E1](phases/E1-BIND-RPC.zh-CN.md) / [E2](phases/E2-PLATFORM-ADAPTERS.zh-CN.md) / [E3](phases/E3-TRACE-AND-DEPLOY.zh-CN.md) / [E4](phases/E4-DOCUMENT-PLANE.zh-CN.md) / [E5](phases/E5-MOBILE-DESKTOP-PLUGIN-PLANE.zh-CN.md)。
> 不替代 MULTITENANCY 的池/执行器合同。本文只规定 **Host 如何嵌 DSH**：聚合、适配器、错误码、凭据顺序、可观察性。

---

## 1. 设计原则

1. **一个聚合，三种适配器。** Host 只操作 `AgentAttachment`。Placement / Presentation / Collaboration 按平台替换，状态名和错误码不许分叉。
2. **配置 URL ≠ 会话 URL。** `APPFLOWY_DSH_AGENT_URL` / `MUSE_DSH_PUBLIC_URL` 只做 origin allowlist 与 CSP。真正加载的地址必须来自 Placement（`session/open.webUrl` 或 sidecar `dsh web:`）。
3. **先身份，再放置，再呈现，再绑定，最后联动。** 缺上一跳不得进入下一跳。联动（contribute / intent）只在 `HybridLive`。
4. **失败可见、可重试、可引用。** 每个失败带 `stage` + `code` + `attachmentId`。禁止空容器、禁止 200 no-op、禁止丢掉 HTTP 状态。
5. **JWT 永不进 DSH 进程。** L0 在 nginx 终止；L1 是 launch token；L2 是 device token。Desktop 不引入 Cloud URL 来「对齐」远程。
6. **载波可以分叉，合同不能。** Facet 词汇一套；Web postMessage、Android HTTP、Desktop hint 都是适配器内部。

---

## 2. 当前实现（按平台）对照目标

### 2.1 Web（`frontend/web/src/components/dsh-agent/`）

| 目标 | 现状 | 缺口 |
|---|---|---|
| 单一 Attachment SM | `pending / ok / down` + 6 条平行 effect | 无 stage、无 code、无 attachmentId |
| 有 JWT+workspace 必 `session/open` | `openDshSession` 在 `!workspaceId` 或无 token 时静默 return | 生产 `/dsh/` 禁止回退 → 不 POST 也不 iframe |
| 只 mount `webUrl` | `canMountDshIframe` 已禁 `/dsh/` | 配置仍注入 `/dsh/` 当默认 `url`；probe 剥 path 打营销站 |
| 排队 | `pickDshSessionPayload` 能读 `queuePosition` | `openDshSession` 把 503 硬等 2s 一次，失败当 null |
| bind ACK | hello/bind 发出即当成功 | 注入脚本 XHR 不读 status；`workspace.bind` 不带 token |
| 永远有 UI | 曾 `ok && !canMount` → 空底 | 必须 pending / error / iframe 三选一 |
| 追踪 | 无 | 无 requestId 贯穿 BFF |

关键文件：`DshAgentPanel.tsx`、`dsh-session.ts`、`dsh-origin.ts`、`dsh-device-token.ts`、`local/scripts/deploy_common.py`。

### 2.2 Desktop（`frontend/client/.../plugins/dsh_agent/`）

| 目标 | 现状 | 缺口 |
|---|---|---|
| Placement = sidecar | `DshSidecar.ensureStarted` 已做 | 错误字符串，未映射共享 `code` |
| Presentation = 本地 WebView | `DshEmbeddedView`；controller 推迟创建避免黑窗 | `launching / ready / lastError` 未对齐 Attachment stage |
| Collaboration = hint | `DshWorkspaceBridge.publish` | 与 Facet 无关，文档/QA 未写死「不验收 postMessage」 |
| 不走 session/open | 正确 | 不要为对齐 Web 打开 parent-bridge |

关键文件：`dsh_sidecar.dart`、`dsh_agent_controller.dart`、`dsh_agent_panel.dart`、`dsh_workspace_bridge.dart`、`dsh_web_auth.dart`。

### 2.3 Android（`middlewares/dsh/mobile/muse-dsh-mobile/`）

| 目标 | 现状 | 缺口 |
|---|---|---|
| Placement = `session/open` | `DshSessionApi` + `isQueued` **已实现** | `DshMobileShellPage` / `DshMobileAgentPage` **不传** session |
| Presentation = `webUrl` | `fromWebUrl` 允许 query token | 实际 `loadRequest(compile-time origin)` |
| Collaboration = HTTP parent-bridge | Coordinator + ControlHost | `MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST` 默认关 → degrade |
| 错误分层 | `fatal` / `degraded` 合同已有 | 未覆盖 `NEED_AUTH` / `QUEUED` / `NO_DEVICE_TOKEN` |
| generation 防过期 | `session.isLive(generation)` | Web 应抄这套，而不是 cancelled 布尔 |

关键文件：`dsh_mobile_coordinator.dart`、`dsh_mobile_shell_page.dart`、`dsh_session_api.dart`、`dsh_remote_config.dart`、`dsh_mobile_error_codes.dart`。

### 2.4 部署（现网 `openmuseai.com`）

| 组件 | 现状 | 对嵌入的影响 |
|---|---|---|
| `dsh-pool.service` | 宿主机 Node，控制 `:13079` 反代 `:13080` | Placement 已通（curl+JWT） |
| `muse-dsh-bff` | Docker host 网络 `:8010` | 官方 Cloud 0.16.5 无 `/api/muse` |
| nginx `/u/` | `auth_request` + 剥 Cookie | L0 依赖 Host 写入 `access_token` cookie |
| nginx `/dsh/` | 仍指向 `:3080`，Docker 已停 → **502** | 配置 URL 不能当 iframe src |
| 仓库 `cloud-same-origin-dsh.conf` | `auth_request` → `:8000` | 与现网 `:8010` 漂移，下次 infra 会回退 |
| `inject_web_config` | 写入 `APPFLOWY_DSH_AGENT_URL=/dsh/` | CSP 需要 apex；iframe 不应使用该 path |
| 实例 `start-instance.sh` | 默认 `MUSE_REQUIRE_HOST_AUTH=1` | 无 `MUSE_DOCUMENT_CLOUD_URL` 则 parent-bridge 不注入 |
| 冷启动 | 首次 tsx ~170s | Host 必须能停在 `Placing`/`Queued`，不能当失败 |

### 2.5 落地后对照（2026-09-11）

Web Attachment + E4 catalog 已在生产 `/u/<hash>/` 打通。Android Placement（session/open → tenant URL）已在 E2 接线，但 **parent-bridge HTTP 默认关**，Host 也未投递 catalog。Desktop 仍是 sidecar + hint，正确，不要改成远程。下一跳是 [E5](phases/E5-MOBILE-DESKTOP-PLUGIN-PLANE.zh-CN.md)。

---

## 3. 目标聚合：`AgentAttachment`

```text
AgentAttachment {
  attachmentId: string          // uuid，面板打开即分配，重试保持或新开可 bump
  generation: number            // 每次 open/retry +1；过期响应丢弃
  stage: AttachmentStage
  identity: {
    accessTokenPresent: boolean
    deviceId: string
    deviceToken?: string
  }
  placement: {
    sessionRef?: string
    webUrl?: string             // 必须 canMount；禁止 /dsh/
    queuePosition?: number
    retryAfterMs?: number
  }
  presentation: {
    frameStatus: 'idle' | 'loading' | 'ready' | 'failed'
    frameReadyAt?: number
  }
  collaboration: {
    boundWorkspaceId?: string
    flags: { bind, context, intent }
    lastBindError?: string
    document?: {
      focusViewId?: string
      catalogSource: 'host' | 'cloud' | 'none'
      lastReadError?: string
    }
  }
  error?: { stage, code, message, retryable }
}
```

### 3.1 状态机（所有远程 Host 共用）

```text
Closed
  │ open panel
  ▼
ResolvingIdentity     NEED_AUTH / NEED_WORKSPACE ──► Failed
  │ cookie + device-token（device-token 可与放置并行，但 Binding 前必须有）
  ▼
Placing               session/open
  ├─ 200 + webUrl ──► Presenting
  ├─ 503 / queuePosition ──► Queued ──(retryAfterMs)──► Placing
  └─ 401/403/5xx ──► Failed(Placing, code)
  ▼
Presenting            加载 webUrl；等 frame-ready 或 document-ready
  └─ timeout / 401 ──► Failed(Presenting, FRAME_TIMEOUT | INGRESS_DENIED)
  ▼
Binding               hello+bind RPC，必须 ACK {ok:true, bound}
  └─ NO_DEVICE_TOKEN / SCOPE_MISMATCH ──► Failed(Binding, code)  iframe 可留着
  ▼
HybridLive            才允许 context.contribute / intent
  │ close / workspace change
  ▼
Closing               session/close；Desktop 不 close 池
  ▼
Closed
```

**不变量**

| ID | 陈述 | 违反即 bug |
|---|---|---|
| A1 | `stage ∈ {Presenting, Binding, HybridLive}` ⇒ `webUrl` 可 mount 且非 `/dsh/` | 黑屏 / 502 iframe |
| A2 | `stage = HybridLive` ⇒ `boundWorkspaceId === hostWorkspaceId` 且最近 bind ACK 成功 | 假联动 |
| A3 | `stage < Binding` ⇒ 不发送 contribute / intent | 静默丢消息 |
| A4 | 无 access token ⇒ 不 POST `session/open`，stage=`Failed(NEED_AUTH)` 而非 pending | 「没有 POST」却无说明 |
| A5 | 有 token + workspace + 面板 open ⇒ 必须已发出或正在发出 `session/open` | 现网主症状 |
| A6 | 任意 Failed 都有 `code` + 非空 UI（文案 + Retry） | 空黑底 |
| A7 | 响应 `generation` 不匹配则忽略 | Strict Mode / 闪烁 workspace |
| A8 | JWT 不出现在 parent-bridge JSON / 实例日志 | I5 |

Desktop 映射：`ResolvingIdentity` 检查 API key；`Placing` = spawn sidecar；`Presenting` = WebView 加载 `dsh web:` URL；`Binding` = 写 hint（无 RPC）；`HybridLive` = hint 已写且 WebView ready。无 L0/L2。

---

## 4. 适配器

### 4.1 PlacementPort

```text
open(workspaceId, deviceId, accessToken) → SessionOpen
  SessionOpen = Ready{sessionRef, webUrl, expiresAt}
              | Queued{sessionRef, queuePosition, retryAfterMs}
              | Denied{code}
close(sessionRef)
heartbeat(sessionRef)
```

| 平台 | 实现 |
|---|---|
| Web / Android | `POST /api/muse/dsh/session/open`（现 `dsh-session.ts` / `DshSessionApi`） |
| Desktop | `DshSidecar.ensureStarted` → 解析 `dsh web:`；无 sessionRef 则用本地 `sidecar.<pid>` |

`openDshSession` 今日把 Queued/非 200 都变成 `null`，必须改成判别联合，否则 A5/排队不可测。

### 4.2 PresentationPort

```text
mount(webUrl)     // iframe src 或 WebView loadRequest
onFrameReady()    // Web: postMessage frame-ready；Android: onPageFinished + 可选 bridge.ready
onLoadFailed(code)
unmount()
```

`webUrl` 校验：`https:` 或桌面 `http://127.0.0.1`；path 不是 `/dsh`；query `token` 仅 Presentation 使用，不写入日志。

### 4.3 CollaborationPort

```text
hello(deviceToken, workspaceRef) → Ack
bind(deviceToken, workspaceRef, title) → Ack{bound}
contribute(envelope) → Ack | FlagOff | TooLarge
onIntent(handler)
closeSurface(ref)
```

| 平台 | 载波 |
|---|---|
| Web | postMessage → 注入脚本 **带 requestId 的 XHR** → parent-bridge |
| Android | HTTPS parent-bridge（source 改写）；`SharedHostSession` 与 Web 同实例同 workspace 共享 SSE/catalog；`HOST_IN_USE` 仅附件上限 |
| Desktop | 写 hint 文件；`hello/bind` 映射为 `publish()` 成功 |

Web `workspace.bind` 今日不含 token，必须纳入消息。注入脚本今日 `xhr.send` 无回调，必须 `onload`/`onerror` 回 `bridge.reply`。

### 4.4 IdentityPort

```text
readAccessToken() → string | null     // access_token | accessToken
syncIngressCookie(token, expiresAt)   // 仅 Web；Android 不写 cookie
issueDeviceToken() → DeviceToken | null
```

Cookie 必须在 **第一次 GET `/u/` 之前** 写入（`Placing` 成功、iframe mount 前）。这是现网 L0 竞态的根。

### 4.5 工作区 Plugin 的页面面（与 Attachment SM 正交）

列出页面、读正文不是嵌入状态机的职责。工作区是业务 Plugin（`@muse/plugin-appflowy-workspace`）：Identity 钉 cwd，Catalog Facet 接收 Host 侧栏投影，Domain 调 Cloud tree，未接线则回退 catalog。JWT 仍不进 DSH；出站用 device token。Document 正文仍由 markdown Plugin 负责。

```text
catalog(workspaceId) → Views{items[]} | CollabUnwired | ScopeMismatch
readCurrent()        → Snapshot | NoCurrentSelection | CollabUnwired
propose(...)         → Proposal | CollabUnwired
apply(...)           → Receipt | CollabUnwired
```

| 能力 | P0 真相（现网可做） | P1 真相（canonical） |
|---|---|---|
| Catalog | Host contribute 有界 `{viewId,title,layout,parentViewId?}[]` | BFF/Cloud `POST /api/muse/workspace/tree` |
| Current selection | Host `workspace.focus` / `markdown.surface` 的 `viewId` | 同左；工具禁止模型自带 viewId |
| Snapshot | Host 有界 markdown（当前 view，≤32KB） | `POST /api/muse/document/query` |
| Mutation | 返回 `CLOUD_COLLAB_ADAPTER_NOT_WIRED` | Cloud `propose`/`apply` + collab 存储 |

**不变量**

| ID | 陈述 | 违反即 bug |
|---|---|---|
| D1 | DSH cwd / `ls` 不是 AppFlowy folder | Agent 把 README-only 说成「工作区为空」 |
| D2 | 无 `viewId` 焦点时 `readCurrent` = `NO_CURRENT_SELECTION`，不是 `NOT_FOUND` | 与空树、桥未接无法区分 |
| D3 | BFF 对已认知但未实现的 `/api/muse/workspace/tree`、`/api/muse/document/*` 不得 404 `NOT_FOUND` | 现网 Agent 误判 |
| D4 | 出站 Authorization 只许 device token 或显式 `MUSE_DOCUMENT_CLOUD_TOKEN`，不许 GoTrue JWT | I5 |
| D5 | `stage < HybridLive` 不投递 workspace catalog / document snapshot | 假读 |

P0 与 P1 可并存：Cloud 501 时 fallback Host 投影；Cloud 200 时投影只作焦点，目录以 tree 为准。

Desktop：DocumentPort = UDS collab，不走 BFF。不要为对齐 Web 在桌面开 Cloud URL。

---

## 5. 错误码（跨平台合同）

新建 `contracts/dsh-attachment-error-codes.v1.json`。与 mobile 合同并存：fatal 继续给 WebView TLS；Attachment 码给打开链。

| code | stage | 用户语义 | retryable |
|---|---|---|---|
| `NEED_AUTH` | Identity | 请登录后再打开 Agent | no（去登录） |
| `NEED_WORKSPACE` | Identity | 打开一个工作区后再启动 | no |
| `NEED_API_KEY` | Identity | Desktop：填写模型 key | no |
| `SESSION_DENIED` | Placing | 无权限或会话 API 拒绝 | no |
| `SCOPE_MISMATCH` | Placing / Binding | 不是该工作区成员 | no |
| `POOL_UNAVAILABLE` | Placing | 控制面不可用 | yes |
| `QUEUED` | Queued | 前方还有 N 人（展示，不算 Failed） | auto |
| `COLD_START` | Placing | 首次启动较久（展示进度） | wait |
| `INGRESS_DENIED` | Presenting | 页面 401（cookie / auth_request） | yes |
| `FRAME_TIMEOUT` | Presenting | 页面未就绪 | yes |
| `BRIDGE_SILENT` | Presenting | 有 HTML 无 frame-ready | yes |
| `NO_DEVICE_TOKEN` | Binding | 设备凭据缺失 | yes |
| `BIND_REJECTED` | Binding | Cloud 拒绝 pin | no |
| `FLAG_OFF` | HybridLive | 服务端关闭了 context/intent | no |
| `NO_CURRENT_SELECTION` | HybridLive | Host 未贡献当前 viewId | no（先在 AppFlowy 打开一页） |
| `CLOUD_COLLAB_ADAPTER_NOT_WIRED` | HybridLive | 页面树/正文桥未接 | no（产品层；P1 接线后消失） |
| `SIDECAR_EXIT` | Placing | Desktop 进程退出 | yes |

UI：`data-attachment-stage` + `data-error-code` + 可读文案 + `attachmentId`（可复制，默认折叠）。

---

## 6. 鲁棒性 / 稳健性 / 可恢复追踪性

这三项必须能**用测试矩阵证伪**，不能停留在形容词。

### 6.1 鲁棒性（不正确状态进不去）

| 性质 | 设计手段 | 如何证伪 |
|---|---|---|
| 不会 iframe `/dsh/` | A1 + `canMountDshIframe` 在 mount 前硬校验 | E0-T3：配置为 `/dsh/` 时 iframe src 仍只能是 mock 的 webUrl |
| 不会在无 JWT 时假装启动 | A4 | E0-T4：无 token → 无 fetch、错误码 `NEED_AUTH` |
| 不会在无 workspace 时空转 | A4/A5 | E0-T5：文案 `NEED_WORKSPACE`，0 次 open |
| 不会未 bind 就 contribute | A3 | E1-T6：bind 401 时 contribute spy 为 0 |
| 不会把 503 当「DSH 挂了」 | Queued 态 | E0-T7：展示排队并按 `retryAfterMs` 再 POST |
| JWT 不进桥 | A8 + FORBIDDEN_FIELD | E1-T8：消息含 access_token → 拒绝且 Host 显示 |

**稳健性补充（长时间正确）**

- 换 workspace：generation++，close 旧 session，新 webUrl 换 iframe `key`。
- device token 过期：Binding 失败 `NO_DEVICE_TOKEN` → 重新 issue → 只重 bind，不拆实例。
- 心跳：`HybridLive` 每 60s heartbeat；失败不立刻 Failed，连续 N 次再标 `POOL_UNAVAILABLE`（避免一次网络抖动拆掉 iframe）。

### 6.2 可恢复性（失败后能回到已知态）

| 失败 | 恢复动作 | 不丢失的 |
|---|---|---|
| `QUEUED` / `COLD_START` | 自动重试 open | attachmentId、deviceId |
| `FRAME_TIMEOUT` / `BRIDGE_SILENT` | Retry：新 generation，同 sessionRef 可复用 webUrl | session |
| `NO_DEVICE_TOKEN` | 重发 device-token + bind | iframe、session |
| `INGRESS_DENIED` | 重写 cookie 再 reload iframe | session |
| `SIDECAR_EXIT` | 再 spawn | hint 文件仍在 DSH_HOME |
| 用户 Retry | generation++，从当前 Failed.stage 的上一跳开始，不从头登录 | identity |

过期响应：所有 async 出口比较 `generation`（抄 Android `session.isLive`）。Web 今日 `sessionRequestId` 只盖住 open，hello/device-token 仍用 cancelled 布尔，E0 统一。

### 6.3 可追踪性（一次失败能定位到层）

一条附件从打开到失败，日志/UI/Network 必须能对上：

```text
attachmentId = att_...
generation  = 3
stage       = Binding
code        = NO_DEVICE_TOKEN
```

传播：

| 跳 | 载体 |
|---|---|
| Host UI | `data-attachment-id`；console `muse.dsh.attachment` 结构化事件（无 token） |
| `session/open` | Header `X-Muse-Attachment-Id`；BFF 打同一 id 进 pool 请求 |
| 池 / systemd | journal 带 attachmentId / tenantHash（已有 instance unit 名） |
| parent-bridge | 请求体 `requestId`；响应原样返回；Host 把 requestId 记到 attachment |
| 禁止 | 日志打印 JWT、device token、launch token |

排障剧本（E3 写入 runbook）：

1. UI 复制 `attachmentId` + `code`。
2. 浏览器 Network：是否有 `session/open`、状态码、是否有 `GET /u/`。
3. BFF / pool journal：`attachmentId`。
4. 若 open 200 但无 bind ACK：看实例 `parent-bridge` 响应码。

没有 id 的失败视为产品缺陷（E3-T1）。

---

## 7. 部署合同（嵌入相关，不是再切执行器）

1. **BFF 是生产 `/api/muse` 的长期形态**（直到官方 Cloud 合并）。nginx `auth_request` 与 `/api/muse` 必须指向同一 BFF，仓库片段与现网一致（E3）。
2. **`/dsh/` 保持 401/502 均可，但不得进入 iframe。** 人类调试用。
3. **inject 拆字段：** `APPFLOWY_DSH_FRAME_ORIGIN=https://openmuseai.com`（CSP）与可选 `APPFLOWY_DSH_AGENT_URL`（仅 local loopback 才允许当 src）。
4. **实例 env 必含** `MUSE_DOCUMENT_CLOUD_URL`（BFF 或 Cloud 的 workspace.current）+ `MUSE_REQUIRE_HOST_AUTH=1`。缺一则 capabilities 探针失败，Host 进 `BRIDGE_SILENT` 而不是假 HybridLive。
5. **BFF `/api/muse` 合同分两档：** 控制面（session / device-token / workspace.current）必须 200 路径；文档面（workspace/tree、document/*）在接线前必须 501 `CLOUD_COLLAB_ADAPTER_NOT_WIRED`，禁止 catch-all `NOT_FOUND`。
6. **冷启动：** Host 按 `COLD_START` 展示「首次可能超过一分钟」；池侧预热仍属 P2/P3，不阻塞 E0。

切流转顺序（再部署 Web 时）：

```text
1. BFF + pool 健康（已具备）
2. 部署带 Attachment SM 的 Web（E0）—— 此时才禁止依赖 /dsh/ iframe
3. 硬刷新验收 Network 链
4. 再动 Android / bind RPC（E1/E2）
```

历史上「先停 Docker、后改 Host」违反这条，E0 就是补上 Host。

---

## 8. 明确不做

- 不用共享 Docker `/dsh/` 当生产 iframe。
- 不把 Desktop 改成远程 Facet。
- 不在 Host 用 `no-cors` GET 当健康检查。
- 不在 E0 引入 Nomad / 跨机调度。
- 不把模型 key 注入 `__APP_CONFIG__`。
