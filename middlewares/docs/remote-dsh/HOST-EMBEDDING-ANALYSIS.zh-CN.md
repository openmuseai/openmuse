# Host 嵌入 DSH：脆弱性领域分析

> 状态：诊断文档（2026-09-10）。配套 [MULTITENANCY.zh-CN.md](MULTITENANCY.zh-CN.md)、[P0-HOST-CHANNEL](phases/P0-HOST-CHANNEL.zh-CN.md)、[GATEWAY.zh-CN.md](GATEWAY.zh-CN.md)、协议细节 [web-workspace-ingress.md](../../dsh/plugins/dsh-appflowy/docs/web-workspace-ingress.md)。
> **目标设计与鲁棒性论证**：[HOST-EMBEDDING-DESIGN.zh-CN.md](HOST-EMBEDDING-DESIGN.zh-CN.md)。
> **开发计划（E0–E4）**：[HOST-EMBEDDING-PLAN.zh-CN.md](HOST-EMBEDDING-PLAN.zh-CN.md)。Document 面：[E4-DOCUMENT-PLANE](phases/E4-DOCUMENT-PLANE.zh-CN.md)。
> 范围：Web iframe / Desktop sidecar WebView / Android WebView 三端如何把 DSH 嵌进 AppFlowy Host，以及为什么现网表现为「打不开、黑屏、没有 `session/open`、Host 与 DSH 不联动」。
> 不范围：池调度算法本身、Nomad、模型 key 分发。那些见 MULTITENANCY / RUNTIME-COMPARISON。

---

## 0. 结论先读

**主因是方案，不是单纯部署事故。** 部署切流转（停 Docker 共享实例、改 nginx、上 systemd-run 池）把方案里本来就没闭合的宿主状态机暴露出来，所以看起来像「一改生产就碎」。

把桌面级 **独占 harness**（单 profile、单 workspace、进程级单例、`scopedMultiTenant: false`）嵌进多端 Host，却没有给 Host 一条**可观察的会话状态机**。三端只共用 Facet **词汇**（`parent-hello` / `workspace.bind` / `context.contribute` / `intent.dispatch`），**载体完全分叉**。远程还叠了三层正交凭据（Cloud JWT cookie、DSH launch token、device token），任一层失败的 UI 都塌成「Starting agent…」或黑屏。

| 判断 | 含义 |
|---|---|
| 方案问题 | 嵌入模型把「本地 sidecar 信任」直接外推到「远程 iframe + 多租户进程」；Host 没有会话 SM；联动 fire-and-forget |
| 部署问题 | P0 入口 `/dsh/` 已停（502），P1 `/u/<hash>/` 的宿主侧未闭合；官方 Cloud 0.16.5 无 `/api/muse/*`，BFF 是旁路；仓库 nginx 与现网不一致 |
| 不是 | DSH 官方 Web Client 本身「不适合 iframe」——它能嵌；碎的是 **Host 怎么打开它、怎么证明身份、怎么确认绑定** |

现网 Web「Network 里没有 `POST /api/muse/dsh/session/open`，面板 Starting agent… 然后黑屏」是 **宿主状态机 + 切流转** 的典型交汇，不是池执行器单独坏了。池侧 `session/open` 在 JWT 下已能 200 并返回 `/u/<hash>/`（见 P0 生产验证表）。浏览器没发出 POST，问题在 Web Host，不在 systemd。

---

## 1. 领域模型

### 1.1 限界上下文（不要混成一个「DSH 服务」）

```text
┌──────────── AppFlowy Host ────────────┐     ┌──────── DSH Runtime ────────┐
│ 文档 / 工作区 / 成员 / GoTrue JWT      │     │ harness + 插件 + 模型调用     │
│ Web SPA · Flutter Desktop · Android   │     │ 单进程 = 单租户实例          │
└───────────────┬───────────────────────┘     └─────────────┬──────────────┘
                │                                           │
                │  ① 控制面（谁可以有一台实例）                │
                │  ② 入口面（谁可以看见 Web UI）               │
                │  ③ 联动面（工作区/上下文/意图）               │
                ▼                                           ▼
         Cloud BFF / muse-bff / nginx              parent-bridge / hint / UDS
```

五个不该合并的上下文：

| 上下文 | 职责 | 现网实现 |
|---|---|---|
| **Identity** | 人是谁、是不是该 workspace 成员 | GoTrue JWT；device token；`POST /api/muse/workspace/current` |
| **Placement** | 给这个 `(account, workspace)` 一台实例 | `@muse/dsh-pool` + systemd-run；Desktop 则是本机 sidecar |
| **Presentation** | 把 DSH Web UI 嵌进 Host | iframe / WKWebView / Android WebView |
| **Collaboration** | Host 选区/焦点 ↔ DSH agent 状态 | Facet：bind / context / intent |
| **Document** | AppFlowy 页面目录与正文 | Cloud `workspace/tree` + `document/query`；生产 BFF **未实现**，落入 `{message:NOT_FOUND}` |

当前代码把 ③ 的失败表现成 ② 的黑屏，把 ② 的 401 表现成 ① 的「没登录」，把 ① 的 cookie 没写表现成 ④ 的「DSH 挂了」。**把 DSH cwd（README.md）当成 AppFlowy 页面树，把 BFF 404 当成「工作区为空」**，是把 Document 面误读成 Collaboration / Placement。领域边界没有在 UI 与工具错误码上显影，所以排障只能靠 Network 碰运气。

### 1.2 实体

| 实体 | 标识 | 生命周期 | 注意 |
|---|---|---|---|
| Actor | GoTrue `sub` | 登录会话 | JWT **禁止**进入 DSH 进程 |
| Workspace | AppFlowy `workspaceId` | Host 文档域 | **不是** DSH cwd。cwd 是实例盘上的 scratch（现网默认只有 README.md） |
| Folder view | AppFlowy `viewId` | Cloud folder collab | 页面清单来自 `workspace/tree`，不是 `ls` |
| Current document | Host 焦点 `viewId` | Collaboration 投影 | `muse_document_read_current` 的「当前」只来自 contribute，模型不能自选 |
| Device | `deviceId`（Web: `web.*` localStorage） | 浏览器/设备 | 与 tab 不同；心跳按 device |
| DeviceToken | 两段式 HMAC，非 `sk-` | TTL（现网约 900s） | 只用于 L2 bind / 部分 ingress |
| Tenant | `(accountRef, workspaceRef)` → `tenantKey` / `tenantHash` | 池内去重 | URL 只暴露 `tenantHash` |
| Instance | 一个 Node harness，`DSH_HOME` + loopback port + cgroup | READY / idle / 销毁进程保留盘 | 进程内仍是单租户单例 |
| Session | `sessionRef` | `session/open` → heartbeat → close | 控制面记录，不是 DSH 内部 session |
| LaunchToken | `?token=` | 该实例 Web UI 一次性/短会话 | 打开页面用，不能替代 device token |
| Attachment | 一个 iframe / WebView | 随面板 | 同 session 理论上可多设备，Web 未做 |

### 1.3 三层凭据（正交，必须全部成功）

```text
L0  Ingress     Cloud JWT Cookie `access_token`     nginx auth_request → ingress-auth
L1  UI session  Launch token `?token=`              DSH 自己的 Web 登录
L2  Host bind   Device token in parent-hello        verifyHostWorkspace → 才能 pin workspace
```

| 只成功 | 用户看到 |
|---|---|
| 无 L0 | iframe/`/u/` 401，或根本不发 `session/open`（Host 认为没登录） |
| L0 无 L1 | HTML 进得去但 DSH 停在自己的登录/空白（黑屏） |
| L0+L1 无 L2 | DSH UI 能聊，但不知道当前 AppFlowy 工作区；工具/上下文是空的 |
| L0–L2 无 Document 面 | 工作区已 pin、cwd 有 README，问「有哪些页面 / 读当前文档」得到 `NOT_FOUND` |
| 三层都有但无 ACK | 看起来「开了」，联动其实 401 被丢掉 |

Desktop sidecar **故意不做 L0/L2**：本机进程 + hint 文件，信任模型不同。把 Desktop 的手感当成 Web 的验收标准，会误判远程「应该也能直接打开」。

### 1.4 不变量（方案已写、宿主未完全执行）

摘自 MULTITENANCY，对照宿主代码：

| ID | 不变量 | 宿主现状 |
|---|---|---|
| I0 | bind 必须经 Cloud 成员校验，fail-closed | DSH 侧 `hostAuthRequired()` 已做；Web 注入脚本 **不读** 401；`workspace.bind` **不带** deviceToken，依赖 `rememberDeviceAuth` 竞态 |
| I1 | 同一 tenant 最多 1 实例 | 池已做；Mobile 仍用编译期 origin，绕过池 |
| I5 | JWT 不进实例 | nginx 剥 Cookie 是对的；Web 却把同一 JWT 镜像成非 HttpOnly cookie（XSS 面 = localStorage） |
| I6 | 冷启动 < 10s | 现网首次 tsx 编译可达 ~3 min；Host 无等待室、无超时文案分级 |
| — | 配置 URL ≠ 会话 URL | Web 仍注入 `APPFLOWY_DSH_AGENT_URL=https://openmuseai.com/dsh/`，而 `/dsh/` 已 502 |

---

## 2. 协议与交互拆解

### 2.1 分层（从上到下）

```text
Host UI 状态机（缺失）
    │
    ├─ HTTP 控制面     POST /api/muse/dsh/session/{open,close,heartbeat}
    │                  POST /api/muse/dsh/device-token
    │                  GET  /api/muse/dsh/ingress-auth   (nginx 内部)
    │
    ├─ HTTP 入口面     GET  /u/<tenantHash>/?token=…     (P1)
    │                  GET  /dsh/                        (P0，现网 502)
    │
    └─ 联动面（三载体）
         Web:     iframe postMessage ↔ 注入脚本 XHR/SSE ↔ /muse/v1/parent-bridge
         Desktop: $DSH_HOME/bindings/current-appflowy-workspace.json + host-bridge UDS
         Android: HTTPS POST/SSE parent-bridge，source=muse.appflowy-mobile → 改写成 web

    └─ 文档面（与联动面正交）
         DSH 工具 → InProcess Cloud provider → POST {MUSE_DOCUMENT_CLOUD_URL}/api/muse/
              workspace/tree     列 AppFlowy folder views
              document/query     读当前 view 的 markdown 快照
         鉴权：Bearer deviceToken（禁止 JWT 进实例）
         生产：上述路径打到 muse-dsh-bff；BFF 只实现 session/device-token/workspace.current，
              其余一律 `{code:1, message:NOT_FOUND}`
```

`source` 常量：

| 值 | 谁 |
|---|---|
| `muse.appflowy-web` | Web 父页；Android 入站后被改写成这个再进 inbox |
| `muse.appflowy-mobile` | Android 原始 HTTP body |
| `muse.dsh-web` | DSH → Host（`frame-ready` / `intent.dispatch`） |

Facet 消息（Web/Android 词汇相同）：

| type | 方向 | 作用 |
|---|---|---|
| `frame-ready` | DSH → Host | 注入脚本就绪（仅 Web iframe） |
| `parent-hello` | Host → DSH | 带 deviceToken；可附 workspaceRef |
| `workspace.bind` | Host → DSH | pin 工作区（Web **不带** token） |
| `context.contribute` | Host → DSH | workspace.focus / tree.ui / markdown.* |
| `surface.closed` | Host → DSH | 切文档时关旧 surface |
| `intent.dispatch` | DSH → Host | 让 Host 打开 view / reveal block |
| `intent.receipt` | Host → DSH | ACK |
| `peer.ping` / `peer.close` / `bridge.ready` | Android 附加 | lease |

### 2.2 Web 打开序列（设计 vs 代码）

设计（MULTITENANCY §4 / P0）：

```text
登录 JWT
  → device-token
  → session/open {workspaceId, deviceId}
  → iframe src = webUrl (/u/<hash>/?token=)
  → frame-ready
  → parent-hello(deviceToken) + workspace.bind
  → context / intent
```

代码（`DshAgentPanel.tsx`）实际是 **至少 6 条互不汇合的 effect**：

1. 写 ingress cookie（`syncDshIngressCookie`）
2. 拉 device-token（独立，失败只 `setDeviceToken(null)`）
3. `session/open`（依赖 `open && workspaceId && token`）
4. probe origin（把 path 剥掉打到 apex）
5. 听到 `frame-ready` 再 hello/bind
6. `status==='ok'` 再 hello/bind 一次（iframe 可能还没挂上）

任何一条的 cleanup（React Strict Mode、`workspaceId` 闪烁、关面板）都会丢结果。`session/open` 与 device-token **并行**，hello 经常在 token 到达前发出。

注入脚本（`PARENT_BRIDGE_SCRIPT`）是：

```text
postMessage in → XHR POST /muse/v1/parent-bridge   // 不读 status
SSE EventSource → postMessage out                  // 无 parentOrigin 则丢掉 intent
frame-ready → parent.postMessage(..., "*")
```

这是 **单向尽力而为**，不是 RPC。

### 2.3 Desktop 打开序列（另一套信任）

```text
面板打开 → DshSidecar.ensureStarted
  → 解析 stdout `dsh web: <url>?token=`
  → Embedded WebView 加载 loopback
  → DshWorkspaceBridge 写 hint JSON
  → 插件 watchAppFlowyWorkspaceHint → applyWorkspaceHint
```

`parentBridgeEnabled()` 在无 `MUSE_DOCUMENT_CLOUD_URL` 时为 false：**没有** iframe 注入脚本，**没有** `session/open`。联动是文件，不是 Facet 载波。这是有意分叉（`parent-bridge.ts` 注释：Desktop = hint + UDS）。

### 2.4 Android 打开序列（设计有、接线没有）

`DshSessionApi.open` 存在，但 `DshMobileShellPage` / `DshMobileAgentPage` **不传** `sessionWebUrl` / `sessionApi`。Coordinator 回退到编译期 `MUSE_DSH_PUBLIC_URL`（只保留 origin，去掉 `/u/` 与 query token）。

控制面另走 HTTP parent-bridge，且要求 `MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST=1`，否则 403 `MOBILE_DISABLED`。失败时 `_degrade`：**WebView 仍在，联动丢掉**。后台会 degrade。

### 2.5 门控（两端同名，注入面不同）

| Flag | Web Host | DSH 进程 | 默认 |
|---|---|---|---|
| `MUSE_WEB_WORKSPACE_BIND` | `window.__APP_CONFIG__` / Vite；**生产 inject 不写** | `envFlagEnabled`；远程被 `hostAuthRequired` 强制 | on |
| `MUSE_WEB_CONTEXT_UPLINK` | 同上 | 同上 | on |
| `MUSE_WEB_INTENT_DOWNLINK` | 同上 | 同上 | on |
| `MUSE_PARENT_BRIDGE` | — | 有 Cloud URL 则 on | 跟 `MUSE_DOCUMENT_CLOUD_URL` |
| `MUSE_REQUIRE_HOST_AUTH` | — | `start-instance.sh` 默认 `1` | on（远程） |
| `MUSE_MOBILE_BRIDGE_EXCLUSIVE_TEST` | — | 必须 `1` | **off** |

生产 Web inject 只写 `APPFLOWY_*` 四个键（`deploy_common.py` `inject_web_config`）。Host 侧门控靠默认 on，一般不是现网杀手；**DSH 实例若没带上 `MUSE_DOCUMENT_CLOUD_URL`，parent-bridge 整段不注册**，注入脚本没有，联动为零。

---

## 3. 方案问题 vs 部署问题

### 3.1 方案（改部署好不了）

1. **独占进程当嵌入运行时。** DSH 模块级 `lastWorkspaceSurface` / `lastDeviceAuth` / `ExclusiveMobileLease`。池用「一租户一进程」绕过进程内多租户，但 Host 仍按「一个全局 Agent 面板」编程。
2. **没有 Host 会话状态机。** pending / ok / down 三态撑不住「排队 / 冷启动 3 分钟 / 缺 workspace / 缺 token / bind 401 / iframe onLoad」。ok + 不能 mount → 曾经直接 `return null`（深色空面板 = 黑屏）。
3. **配置 URL 与会话 URL 混用。** `APPFLOWY_DSH_AGENT_URL=/dsh/` 同时承担 CSP `frame-src`、失败回退、probe 目标。P1 之后 `/dsh/` 不是合法 iframe src。
4. **联动无确认。** bind 失败 Host 不知道；context 超 32KB 或含 `access_token` 字样直接丢。
5. **三载体分叉未当成产品事实。** 文档写「与 Desktop sidecar 同构」，远程其实多了 BFF + 池 + cookie + device token。验收如果拿 Desktop 比 Web，永远觉得 Web「莫名其妙脆」。
6. **Cloud 合同与制品分叉。** 设计是 AppFlowy-Cloud 提供 `/api/muse/*`；现网官方镜像 0.16.5 没有这些路由，用 `muse-dsh-bff` 填洞。这是架构决策，不是一次配错。

### 3.2 部署（方案对了也会炸）

1. **切流转顺序反了：** 先停 `muse-dsh`（`/dsh/` → 502），后改「禁止 iframe `/dsh/`」。中间版本会 502 或黑屏。
2. **systemd-run 细节**（已修，属于执行器而非嵌入协议）：`PidsMax` vs `TasksMax`；非 TTY `--pipe` 堵死 stdout；`EnvironmentFile` 里的 `PORT=3080` 覆盖实例端口。
3. **冷启动：** 池拉起 `node --import tsx/esm`，首次编译可达数分钟；Host 无 `retryAfterMs` / `queuePosition` UI。
4. **nginx 漂移：** 仓库 `cloud-same-origin-dsh.conf` 的 `auth_request` 指向 `127.0.0.1:8000`（官方 Cloud）；现网必须打到 BFF `:8010`。片段未与生产合一。
5. **ingress cookie 时序：** cookie 只在面板 `open` 时写。先打开 `/u/` 再写 cookie，或第三方 iframe 不带 cookie，L0 失败。
6. **probe 打错地方：** `probeDshOrigin` 用 `normalizeDshOrigin` 剥 path，对 `https://openmuseai.com/u/<hash>/` 实际 GET 的是营销站 `/`。远程分支还把失败也当成 `ok`，iframe 挂上去再黑。

### 3.3 一张对照表

| 现象 | 层 | 主因 |
|---|---|---|
| Network 无 `session/open` | 方案 + 现网 Web | Host 没发：无 workspaceId / 无 JWT / 面板 effect 被取消；不是池挂了 |
| Starting agent… → 黑屏 | 方案 | 三态不够；`ok && !canMount` 画空底；或 iframe 401/空白 |
| `GET /dsh/` 502 | 部署（有意） | 共享实例已停；Host 仍把它当默认 src 就是方案未切干净 |
| iframe `/u/` 401 | 部署时序 | cookie 未写或 auth_request 打到没有 `/api/muse` 的 Cloud |
| DSH 能开但不能跟文档联动 | 方案 | hello 无 token；bind 无 ACK；parent-bridge 未启用 |
| Desktop 一直还能用 | 方案分叉 | 根本不走远程这套 |
| Android 打不开远程实例 | 方案未接线 | `session/open` API 有，Shell 没用 |

---

## 4. 关键功能问题

### 4.1 无法启动（含「没有 POST session/open」）

**现象**

- 打开 DeepSeek Agent 面板，Starting agent… 停住或变黑。
- DevTools Network 过滤不到 `POST /api/muse/dsh/session/open`。
- 或有 POST 但一直 pending（冷启动）。

**原因（Web，按概率）**

1. **请求根本没发出**
   - `openDshSession` 在 `!workspaceId` 时直接 return。生产配置是 `/dsh/`，`shouldFallbackToConfiguredDsh` 为 false，于是既不 iframe 也不 POST，一直 pending。
   - `getTokenParsed()?.access_token` 为空则 **fetch 之前** return null（token 若是 `accessToken` 驼峰也会漏，已在代码里补双键，**尚未当作已发布事实**）。
   - Network 面板若只看 Doc/JS，XHR 会被滤掉——验收应固定看 Fetch/XHR。
2. **发出了但被 Host 丢掉**
   - effect cleanup 把 in-flight 的 200 丢弃；用户只看到第一次的空窗。
3. **发出了但 60s 内看不到 200**
   - 池冷启动 + tsx 编译；nginx `proxy_read_timeout` 已放到 200s 才够。Host 没有「仍在启动」进度。
4. **503 排队**
   - 池 `READY=1` 时返回 `queuePosition` **没有** `webUrl`。Host 把非 200 / 无 webUrl 当失败。

**Desktop：** 缺 `DEEPSEEK_API_KEY`、harness 路径、端口占用。与 `session/open` 无关。

**Android：** Shell 从不调用 `DshSessionApi.open`。编译期 URL 若仍是 `/dsh/` 或裸 origin，现网就是 502/错页。

**解决**

| 优先级 | 动作 |
|---|---|
| P0 | Host 显式状态：`need-workspace` / `need-auth` / `opening` / `queued` / `mounting` / `bound` / `failed`，每态有文案与 Retry |
| P0 | `session/open` 的触发条件写成断言：有 `workspaceId`（URL params **或** Auth 上下文）且有 access token 就必须 POST；禁止用「等 iframe」代替 |
| P0 | 处理 503：展示排队，按 `retryAfterMs` 重试，不要变 down |
| P1 | 预热或预编译 runtime，把 I6 拉回十秒级 |
| P1 | Android Shell 接入同一套 `session/open` → `webUrl` |
| 验收 | Network 必须出现 `session/open` 再出现 `GET /u/<hash>/`；禁止再出现文档导航 `GET /dsh/` |

### 4.2 黑屏

**现象：** 标题栏还在，「Starting agent…」消失后中间一块 Host 背景色（AppFlowy `bg-background-primary` 深色）或 iframe 内全黑。

**原因分层**

| 黑的是谁 | 机制 |
|---|---|
| Host 面板 | `status==='ok'` 但 `canMountDshIframe(frameUrl)` 为 false 时曾经 `return null`；共享 `/dsh/` 被禁 mount 后最容易踩中 |
| iframe 文档 | L0 401 空白；L1 无 launch token；DSH SPA 白/黑底但 JS 挂了 |
| 探活误导 | 远程 probe 失败仍 `setStatus('ok')`，iframe 挂到不可用 origin |
| DSH 自己的 UI | 已加载但未 bind、无会话，官方 Client 深色空壳 |

**解决**

- 永远不要画「无 iframe、无错误、无 pending」的空容器。
- iframe `onLoad` 前保留遮罩；`onLoad` 后若仍无 `frame-ready`（例如 2s）显示「UI loaded but bridge silent」。
- probe 必须打 `webUrl` 的 path（`/u/…`），禁止 `normalizeDshOrigin` 打到营销站。
- 生产 **禁止** 把 `/dsh/` 当作 iframe src；该 URL 只留作「人类调试 / 明确 401」。

### 4.3 Host 无法与 DSH 联动（绑定失败）

**现象：** 面板里能看见 DSH 聊天壳，问「当前文档」或工具要打开页面时，DSH 不知道 workspace / 打开了错误工作区。

**原因**

1. **hello / bind 竞态**
   - `parent-hello` 的 `deviceToken` 来自独立 effect；`frame-ready` 可能更早。
   - 紧随其后的 `workspace.bind` **不含** deviceToken（`MuseDshWorkspaceBind` 类型如此）。
   - `bindWorkspace` 在 `hostAuthRequired()` 时：消息本身没 token 就用 `rememberDeviceAuth`。hello 若被丢掉或尚未 `remember`，bind → `NO_DEVICE_TOKEN`。
2. **注入脚本丢弃 HTTP 错误**
   - `xhr.send` 无 `onload`。401/403/503 只存在于 DSH 日志。Host 继续发 context，像「联动坏了」。
3. **parent-bridge 没加载**
   - 实例 env 缺 `MUSE_DOCUMENT_CLOUD_URL` → `parentBridgeEnabled()===false` → 不注入脚本 → 没有 `frame-ready`。Host 只看到黑/空 iframe。
4. **origin 校验**
   - Host `event.origin` 必须等于 iframe src 的 origin。同源 `/u/` 与 `/app` 同为 `https://openmuseai.com`，这条是通的。若误把 iframe 指到 `dsh.openmuseai.com`，postMessage 会被丢（也是当初废弃裸子域的原因）。
5. **Desktop 对照失效**
   - 桌面靠 hint 文件，不走这条。用桌面「已经能跟工作区走」验收 Web，会误判协议已通。

**解决**

- `workspace.bind` 每次携带当前 deviceToken（或 bind 失败则抑制后续 contribute）。
- 注入脚本改为有 requestId 的 RPC：Host 等到 `{ok:true, bound}` 再标 `bound`。
- hello 必须在 deviceToken 就绪后发；在此之前 iframe 可以 mount，但 Host 显示「Authenticating with agent…」。
- 实例启动契约：`MUSE_DOCUMENT_CLOUD_URL` + `MUSE_REQUIRE_HOST_AUTH=1` 写入 `instance.env` 并做探针（`GET /muse/v1/parent-bridge/capabilities`）。

### 4.4 联动功能「开着但无效」（context / intent）

**现象：** 绑定看似成功，但 Agent 看不到当前页、选区；或 Agent 说「打开某页」Host 没反应。

**原因**

1. **表面挂了、控制面没挂。** Android `_degrade` 明确走这条；Web 没有 degrade 文案，效果相同。
2. **门控两端不一致。** 默认 on，但若有人在实例 env 关了 `MUSE_WEB_CONTEXT_UPLINK` / `INTENT_DOWNLINK`，Host 仍默认发送。DSH 对 contribute 返回 `{ok:true}` 的 **静默 no-op**（`parent-bridge.ts` 关旗仍 200）。
3. **intent 下行条件：** EventSource 先于 `parentOrigin` 赋值。`frame-ready` 与第一条 parent 消息之前的 SSE 事件被丢。Host 从未绑定 `intent.dispatch` 监听以外的重放。
4. **context 被校验扔掉：** >32KB 或 JSON 里碰巧出现 `access_token` / `api_key` 子串 → `FORBIDDEN_FIELD`。选区/文档碎片可能误伤。
5. **iframe `contentWindow` 为 null 时 postMessage。** 注释已写；`status==='ok'` effect 在 iframe 提交前就会跑。消息进虚空。
6. **切工作区。** 进程内 `removeSurface(previous)` 会顶掉旧 surface；若 Host 仍往旧 origin 发 contribute，DSH 已 pin 新租户实例（P1 后应换 iframe src）。Web 换 `workspaceId` 会重开 session，但旧 iframe 可能仍挂着直到 `webUrl` 更新。
7. **Desktop 没有这条 Facet 载波。** 工具走 UDS；不要在桌面验收「postMessage intent」。

**解决**

- Host 在 `bound` 之前不发 contribute。
- DSH 对关旗的 contribute 返回明确 `error: "FLAG_OFF"`，不要 200 no-op。
- SSE 在 `parentOrigin` 未设置时 **缓冲** intent。
- capabilities 响应让 Host 显示「context on / intent on / mobile exclusive」。
- 换 workspace 必须换 iframe `key` + 新 `webUrl`，并 `session/close` 旧 ref。

### 4.5 工作区已挂载，但 Host 页面不可见（2026-09-10）

**现象**

- DSH 面板能开，默认工作区目录在；`ls` 只有 `README.md`。
- Agent 说「页面索引/绑定当前不可用」，`muse_document_read_current` 与 Cloud Muse 桥返回 `NOT_FOUND`。
- 附带：bash 被沙箱拒绝（缺 bubblewrap/Landlock）。这是 **DSH 主机工具** 的执行器限制，与 AppFlowy 页面树无关。

**这不是「工作区为空」。** 两套工作区被方案和工具提示混成了一个词：

```text
DSH cwd / MUSE_APPFLOWY_DSH_WORKSPACE
  = 实例盘 scratch（bindHostWorkspace 保证 Web Client 不空）
  = 预期只有 README.md 一类本地文件

AppFlowy Cloud workspace (workspaceId)
  = folder collab + 若干 viewId
  = 用户在 Host 里看到的页面树
  = 必须走 Document 面，禁止用 bash/ls 代替
```

`workspace.bind` / `workspace.current` 只证明 **成员 + pin**。它不提供 view 列表，也不提供正文。`muse_document_read_current` 的契约是 `document.current.query`：模型不能传 viewId，只能读 Host 已经贡献的焦点。

**原因（按链路）**

1. **Document 面从未成为嵌入合同的一等公民。** E0–E3 把 HybridLive（bind ACK + contribute/intent）当成「能跟文档走」。现网 HybridLive 最多把 `workspace.focus` / `markdown.surface` 写进 DSH 进程内存（`setLastDocumentFocus`）。真正读页面仍要 DSH 出站打 Cloud。
2. **生产 `/api/muse` 的长期形态是 BFF，但 BFF 只覆盖控制面。** `muse-bff.ts` 实现：`ingress-auth`、`device-token`、`workspace/current`、`session/{open,close,heartbeat}`。未匹配路径：

   ```text
   json(res, 404, { code: 1, message: "NOT_FOUND" });
   ```

   仓库里的完整实现在 Muse 补丁 Cloud：`AppFlowy-Cloud/src/api/muse.rs` 的 `workspace_tree_handler` / `document_query_handler`（读 collab）。官方镜像 0.16.5 没有这些路由；nginx 把 `/api/muse` 指到 BFF 之后，**树和正文在现网不存在**。Agent 看到的 `NOT_FOUND` 就是这条 catch-all，不是空 folder。
3. **「当前文档」依赖 Host 焦点，不是 pin。** `createCloudMarkdownProvider.fillSelection` 从 `ctx.documentFocus.viewId` 填空。没有 contribute 的 `viewId` 时，正确错误是 `NO_CURRENT_SELECTION`。现网先撞上 BFF 404，把「没选页」和「桥没接」糊成同一个词。
4. **Host 上行的 tree 不是目录。** `buildTreeUiEnvelope` 只带 `expandedViewIds`（侧栏展开态），不是 `{viewId,title,layout}[]`。即便 HybridLive 正常，DSH 也不能从 contribute 重建页面清单；清单合同写在 `GET /muse/v1/workspace/views` → `invokeCloudWorkspace(workspace.tree.query)` → 同样打到 BFF 404。
5. **出站鉴权与 I5 不冲突，但被 404 挡住了。** 实例用 device token 作 Bearer（`composition-host`：`accessToken = MUSE_DOCUMENT_CLOUD_TOKEN ?? lastDeviceAuth`）。Muse Cloud 的 `MuseActor` 与 BFF `resolveActor` 都认 device token。缺的是 **路由与 collab 适配器**，不是再塞 JWT。
6. **bash 沙箱失败是第五个问题。** `host.*` / bash 作用在 DSH cwd 与宿主机策略上。缺 sandbox 后端时应返回明确 `SANDBOX_UNAVAILABLE`，并禁止模型把「命令失败」解释成「工作区没有页面」。

**解决（方案，不是再切执行器）**

把 Document 收成独立端口，失败码不许再用 `NOT_FOUND` 当万能词。分两层，P0 就能让 Agent 看见页面，P1 才接 canonical collab：

| 层 | 能力 | 真相来源 | 现网缺口 |
|---|---|---|---|
| Selection | 「当前是哪一页」 | Host `context.contribute`（focus / markdown.surface） | HybridLive 已发 envelope；无 viewId 必须 `NO_CURRENT_SELECTION` |
| Catalog | 「这个工作区有哪些页」 | P0：Host 投递有界 `{viewId,title,layout}[]`；P1：BFF/Cloud `workspace/tree` | Host 只发了 expanded ids；BFF 无 tree |
| Snapshot | 「读当前正文」 | P0：Host 投递有界 markdown（≤32KB）；P1：`document/query` | 工具只走 Cloud query → BFF 404 |
| Mutation | propose / apply | **只能** Cloud collab | 保持 `CLOUD_COLLAB_ADAPTER_NOT_WIRED`，不要假装 apply |

立即合同（E4，见计划）：

1. BFF 对已知但未接线的 `/api/muse/workspace/tree`、`/api/muse/document/*` 返回 **501** `CLOUD_COLLAB_ADAPTER_NOT_WIRED`（code 1067），禁止再 404 `NOT_FOUND`。
2. Host 在 HybridLive 增加 **catalog contribute**（页面元数据，不含正文、不含 token）。DSH `workspace.tree.query` 在 Cloud 未接线时读这份投影。
3. `muse_document_read_current`：无焦点 → `NO_CURRENT_SELECTION`；有焦点无 Cloud → 读 Host snapshot，否则 `CLOUD_COLLAB_ADAPTER_NOT_WIRED`。
4. 系统提示写死：DSH cwd ≠ AppFlowy folder；禁止用 bash 列 Host 页面。
5. P1 再把 `workspace/tree` + `document/query` 做进 BFF（device token → actor → Cloud postgres/collab），或 nginx 把这两条指到 Muse 补丁 Cloud。**不要**为了列目录去部署官方 0.16.5「补路由」。

验收：Agent 问「有哪些页面」得到标题列表或明确 `CLOUD_COLLAB_ADAPTER_NOT_WIRED` / `NO_CURRENT_SELECTION`；**禁止**再出现裸 `NOT_FOUND`。`ls` 仍只有 README.md，且提示里承认这是 scratch。

### 4.6 现网切流转特有的组合拳（2026-09-09/10）

时间线压缩：

1. 生产停 Docker `muse-dsh`，`GET /dsh/` → 502（有意）。
2. 池 + BFF + `/u/` + `session/open` 在 **curl + 铸造 JWT** 下是通的。
3. Web 仍注入 `APPFLOWY_DSH_AGENT_URL=/dsh/`（CSP 需要 apex origin；错误地把同一字符串当 iframe 默认 src）。
4. 第一轮修复：禁止 mount `/dsh/`，等待 `webUrl`。未部署完整状态机时：无 workspace → 不 POST → pending → 随后空渲染变黑。
5. 用户看不到 POST：要么没发出，要么看错 Network 类型，要么冷启动未等完。

这不是「systemd 不能嵌 iframe」，而是 **P0 入口已拆、P1 宿主未成为唯一打开路径**。

---

## 5. 分平台

### 5.1 Web（AppFlowy-Web，现网主战场）

| 项 | 实现 |
|---|---|
| 进程 | 远程池；BFF `session/open` |
| UI | iframe，`src` 必须是 `/u/<hash>/?token=` |
| L0 | 非 HttpOnly `access_token` cookie + nginx `auth_request` |
| L1 | `webUrl` 上的 launch token |
| L2 | `POST device-token` + postMessage hello |
| 联动 | 注入脚本 ↔ parent-bridge（HybridLive 后 contribute） |
| 文档 | 工具打 `MUSE_DOCUMENT_CLOUD_URL`；生产 BFF 无 tree/query |
| 与方案差距 | E0–E3 未把 Document 面单独验收；cwd README 被当成页面树 |

**要做的（只 Web）**

- 配置拆两字段：`DSH_FRAME_ORIGIN`（CSP）与「禁止作为 src 的 shared path」。
- 打开协调器单飞轮，替代平行 effect。
- bind RPC + 明确错误码上屏。
- E4：catalog contribute + BFF 对未接线 muse 文档路由返回 501，而不是 404 `NOT_FOUND`。

### 5.2 Desktop（Flutter sidecar）

| 项 | 实现 |
|---|---|
| 进程 | 本机 Node，`127.0.0.1:3080` |
| UI | WKWebView / WebView2，URL 来自 `dsh web:` 日志 |
| L0/L2 | 不做（无 Cloud URL 则 parent-bridge 关） |
| 联动 | hint JSON + UDS；**不是** iframe Facet |

**要做的（只 Desktop）**

- 保持分叉，文档与 QA 矩阵写死：「Desktop 不验收 session/open / postMessage」。
- 启动失败原因（缺 key、端口、harness）直接显示，避免空 WebView。
- 不要为了对齐 Web 强行打开 parent-bridge（会把 JWT/Cloud URL 带进本机进程，违反 I5 精神）。

### 5.3 Android（muse-dsh-mobile）

| 项 | 实现 |
|---|---|
| 进程 | **应当**远程池；**实际**编译期 origin |
| UI | WebView `loadRequest(publicUri)` |
| L0 | **无** cookie 同步；依赖 URL 本身可匿名或另有头 |
| L1 | 只有 `fromWebUrl` 才会带 query token；Shell 没给 |
| L2 | ControlHost 签发 device token；HTTP hello |
| 联动 | 独立于 WebView 的 HTTP/SSE；`exclusive-test` 门；失败 degrade |

**要做的（只 Android）**

- `DshMobileShellPage` 注入 `DshSessionApi`：先 open，再用 `webUrl` 加载。
- 编译期 `MUSE_DSH_PUBLIC_URL` 只作 **origin allowlist**，禁止当页面 URL。
- 现网 `/dsh/` 502 后，不接线就无法做远程演示。
- 生产若要多用户，`exclusive-test` / `HOST_IN_USE` 与「一用户一实例」冲突——lease 应改成 per-instance（实例已是租户），不要整机一把锁。

### 5.4 对照（验收请按列，不要串列）

| 能力 | Web | Desktop | Android |
|---|---|---|---|
| 打开远程租户实例 | 设计有 / 宿主脆 | 不适用 | API 有 / **未接线** |
| 打开本机 sidecar | 仅 dev 回退 loopback | 主路径 | 不适用 |
| 工作区绑定 | Facet + Cloud 校验 | hint 文件 | Facet HTTP，旗标关则无 |
| 文档上下文 | postMessage contribute | 非 Facet | HTTP contribute |
| 打开 Host 页面 | intent SSE | UDS/工具 | SSE；degrade 则无 |
| 列出 / 读 AppFlowy 页 | Cloud tree+query 或 Host catalog | UDS collab | 同 Web；现网 BFF 404 |
| 多租户隔离 | 进程池 | 单用户本机 | 未用池则共享死实例 |

---

## 6. 推荐修复顺序

不把「再切执行器」当修复。嵌入问题在 Host 与协议 ACK。分阶段合同见 [计划](HOST-EMBEDDING-PLAN.zh-CN.md) 与 E0–E4；目标态见 [设计](HOST-EMBEDDING-DESIGN.zh-CN.md)。Document 面见 [E4](phases/E4-DOCUMENT-PLANE.zh-CN.md)。

### 6.1 立刻（Web 现网可验收）

1. 宿主状态机 + 永远有文案（禁空黑底）。
2. 有 workspace + JWT 必须 POST `session/open`；缺一则说明缺的是哪一个。
3. iframe 只接受 `canMount` 的 `webUrl`；CSP 继续 `frame-src 'self' https://openmuseai.com`。
4. `workspace.bind` 带 deviceToken；注入脚本上报 HTTP 状态。
5. 503 / 无 webUrl 走排队重试。
6. 硬刷新验收清单：Fetch `session/open` 200 → Document `GET /u/…` 200 → `frame-ready` → bind 200。

### 6.2 随后（协议硬化 + Document 面）

1. parent-bridge 升为带 `requestId` 的 RPC；关旗返回错误码。
2. capabilities 给 Host 做功能矩阵。
3. 仓库 nginx 片段与现网 BFF `:8010` 合一，避免下次 infra 脚本把 auth 打回官方 Cloud。
4. Android 接 `session/open`；去掉 exclusive 整机锁或改为实例内锁。
5. **E4：** BFF 对 tree/query 返回 501 而不是 `NOT_FOUND`；Host catalog contribute；`muse_document_read_current` 区分 `NO_CURRENT_SELECTION`。P1 再接 collab。

### 6.3 不要做

- 不要为了 iframe「好看」再把共享 Docker `/dsh/` 拉起来当生产入口。
- 不要用官方 Cloud 镜像「补」`/api/muse`（0.16.5 合同如此）；BFF 旁路要文档化为长期形态，或排期真正进 Cloud。
- 不要让 Desktop 改走远程 Facet 来「统一代码」——信任模型不同。
- 不要在 Host 用 no-cors probe 当健康证明。
- 不要让 Agent 用 bash/`ls` 验收 AppFlowy 页面；不要把 BFF 404 当成空工作区。
- 不要指望「在 AppFlowy 里刷新工作区」恢复页面列表——现网树接口根本没挂上。

---

## 7. 领域上「正确」的嵌入模型（目标态）

把打开过程收成一条**单一聚合** `AgentAttachment`：

```text
AgentAttachment
  identity: { accessToken, deviceId, deviceToken }
  placement: { sessionRef, webUrl, queuePosition? }
  presentation: { frameStatus: loading | ready | failed }
  collaboration: { boundWorkspaceId?, flags, lastBindError? }
```

状态转移：

```text
Closed
  → ResolvingIdentity     (JWT / cookie / device-token)
  → Placing               (session/open；queued 循环)
  → Presenting            (iframe/WebView load；等 frame-ready 或 document ready)
  → Binding               (hello+bind RPC；失败可重试，不进 Hybrid)
  → HybridLive            (context + intent)
  → Failed(stage, code)   (任何一跳)
```

三端只替换 **Placement** 与 **Presentation** 适配器：

- Web Placement = BFF session；Presentation = iframe
- Desktop Placement = sidecar spawn；Presentation = 本地 WebView；Binding = hint 文件（Collaboration 适配器不同）
- Android Placement = 同一 BFF session；Presentation = WebView；Binding = HTTP parent-bridge

Facet 词汇保持一套。载体允许分叉，但 **Host 聚合与错误码不许分叉**。

---

## 8. 代码锚点

| 主题 | 路径 |
|---|---|
| Web 面板 / 打开 | `frontend/web/src/components/dsh-agent/DshAgentPanel.tsx` |
| 禁 `/dsh/` iframe | `…/dsh-origin.ts` `canMountDshIframe` / `isSharedDshFrameSrc` |
| session HTTP | `…/dsh-session.ts` |
| cookie / device-token | `…/dsh-device-token.ts` |
| 生产 inject | `local/scripts/deploy_common.py` `inject_web_config` |
| DSH inbox | `middlewares/dsh/plugins/dsh-appflowy/src/parent-bridge.ts` |
| 注入脚本 | 同文件 `PARENT_BRIDGE_SCRIPT` |
| 池 webUrl | `middlewares/dsh/core/dsh-pool/src/tenant.ts` `webUrlOf` |
| nginx | `middlewares/dsh/deploy/nginx/cloud-same-origin-dsh.conf` |
| Desktop sidecar | `frontend/client/…/dsh_sidecar.dart` / `dsh_workspace_bridge.dart` |
| Android 未接线 | `middlewares/dsh/mobile/…/dsh_mobile_shell_page.dart` |
| Android session API | `…/dsh_session_api.dart`（有实现，Shell 未用） |

---

## 9. 一句话

Host 嵌 DSH 脆，是因为 **把一个桌面独占进程，用三种未确认的载波，塞进一个只有三态的面板，还要同时通过三层凭据**；生产又在 P0 入口拆掉之后，让 Web 继续把 `/dsh/` 当默认世界。E0–E3 修好打开与 bind 之后，现网下一刀是 **Document 面缺席**：pin 工作区 ≠ 列出 AppFlowy 页面 ≠ 读当前正文。BFF 对 tree/query 回 `NOT_FOUND`，cwd 的 README 又像「空工作区」。修执行器解决不了这件事。打开链用 Attachment SM；文档链用 **DocumentPort（Host catalog/snapshot 为 P0，Cloud collab 为 P1）**，错误码必须把「没选页 / 桥没接 / scratch 目录」分开。
