# E0 Web Attachment 状态机 — 设计 / 实现 / 测试

> 阶段：E0。合同：[HOST-EMBEDDING-DESIGN.zh-CN.md](../HOST-EMBEDDING-DESIGN.zh-CN.md)。计划：[HOST-EMBEDDING-PLAN.zh-CN.md](../HOST-EMBEDDING-PLAN.zh-CN.md)。
> 平台：**Web only**。Desktop / Android 在 E2。不改 systemd、不拉起 Docker `/dsh/`。

---

## 1. 设计

### 1.1 问题

现网面板 `pending/ok/down` 无法表达「缺登录 / 缺工作区 / 正在 open / 排队 / 冷启动 / 可 mount 的 iframe」。配置 URL 是 `/dsh/`（已 502），`shouldFallbackToConfiguredDsh` 为 false 时：**不 POST、不 iframe、随后空渲染**。

### 1.2 决策

引入 `AgentAttachment` 的 Web 实现（本阶段只做到 `Presenting`：iframe 挂上且 `onLoad`；bind 仍可 fire-and-forget，**不**宣称 HybridLive——那是 E1）。

打开链：

```text
open && (workspaceId | NEED_WORKSPACE) && (accessToken | NEED_AUTH)
  → syncIngressCookie
  → POST session/open（A5）
  → Ready: mount webUrl（A1）
  → Queued: 展示 QUEUED，retryAfterMs 后再 POST
  → 其他: Failed(code) + Retry
```

`APPFLOWY_DSH_AGENT_URL` 仅当 loopback 才允许当 iframe src。生产 CSP 用 origin `https://openmuseai.com`，不把 `/dsh/` 当 src。

### 1.3 不变量（本阶段验收）

DESIGN A1、A4、A5、A6、A7。A2/A3 在 E1。

UI：`data-testid` 保持 `dsh-agent-panel` / `dsh-agent-iframe` / `dsh-agent-error` / `dsh-agent-pending`；新增 `data-attachment-stage`、`data-error-code`。禁止第三种「什么都不渲染」的内容区。

### 1.4 不做

- 不改 parent-bridge 注入脚本（E1）。
- 不接 Android。
- 不把 probe no-cors 当成功条件；远程生产 **跳过 probe**，以 `session/open` + iframe `onLoad` 为准。
- 不在 503 以外的错误上盲等 2s。

---

## 2. 实现

| 路径 | 改动 |
|---|---|
| `frontend/web/src/components/dsh-agent/dsh-attachment.ts` | **新建**：`AttachmentStage`、`AttachmentErrorCode`、reducer、`generation` |
| `frontend/web/src/components/dsh-agent/dsh-session.ts` | `openDshSession` 返回 `Ready \| Queued \| Denied`，禁止把 503 压成 `null`；超时/网络 → `POOL_UNAVAILABLE` |
| `frontend/web/src/components/dsh-agent/DshAgentPanel.tsx` | 单协调 effect 驱动打开链；内容区 stage 分支；cookie 在 mount **之前** |
| `frontend/web/src/components/dsh-agent/dsh-origin.ts` | `probeDshOrigin` 不再作为远程 mount 门槛；可删远程 `setStatus('ok')` 捷径 |
| `frontend/web/src/components/dsh-agent/dsh-app-state.ts` | workspaceId = Auth 上下文 \|\| 路由 `params.workspaceId` |
| `local/scripts/deploy_common.py` `inject_web_config` | CSP `frame-src 'self' {origin}`；`APPFLOWY_DSH_AGENT_URL` 若 path 为 `/dsh` 仍写入供 allowlist，但面板不把它当 `frameUrl` |
| `local/scripts/test_inject_web_config.py` | 断言 apex origin CSP；**不再**要求 iframe 默认等于 `/dsh/` 可 mount |
| `contracts/dsh-attachment-error-codes.v1.json` | 新建，E0 先放 Identity/Placing/Presenting 码 |

`openDshSession` 建议形状：

```ts
type SessionOpenResult =
  | { kind: 'ready'; session: DshSessionOpen }
  | { kind: 'queued'; session: DshSessionOpen }
  | { kind: 'denied'; status: number; code: string };
```

Queued：`queuePosition != null && !webUrl`，或 HTTP 503 且 body 含 `retryAfterMs`。

面板协调（伪代码）：

```text
on (open, workspaceId, token, retryNonce):
  gen++
  if !open → close session; stage=Closed; return
  if !token → Failed NEED_AUTH
  if !workspaceId → Failed NEED_WORKSPACE
  cookie sync
  stage=Placing
  result = openDshSession(...)
  if gen mismatch return
  switch result.kind
```

Retry 按钮只 `retryNonce++`，不卸整个 Provider。

---

## 3. 测试方案

- Jest：reducer 纯函数；`openDshSession` 用 mock fetch（200 / 503 queued / 401 / 无 token）。
- 组件：`DshAgentPanel.test.tsx` 覆盖「必 POST / 禁空底 / 禁 /dsh/ iframe / 排队文案」。
- `dsh-origin`：`canMount` 回归。
- `test_inject_web_config.py`。
- 现网：硬刷新 + Network Fetch/XHR（见 §5）。不在 CI 打生产。

---

## 4. 测试矩阵

| ID | 场景 | 期望 | 类型 |
|---|---|---|---|
| E0-T1 | 有 workspace + JWT，open=true | `openDshSession` 被调用恰好 ≥1；最终 iframe src 为返回的 `/u/…` | 组件 |
| E0-T2 | 同上但 `APPFLOWY_DSH_AGENT_URL=/dsh/` | iframe **不是** `/dsh/`；无 `GET` 该配置 url 作为 src | 组件 |
| E0-T3 | `canMountDshIframe('/dsh/')` | false | 单测 |
| E0-T4 | 无 JWT | 0 次 fetch session/open；`data-error-code=NEED_AUTH`；有 Retry | 组件 |
| E0-T5 | 无 workspaceId | 0 次 open；`NEED_WORKSPACE`；非空 pending/error | 组件 |
| E0-T6 | open 返回 webUrl | 内容区有 iframe；无「空 panel 无 testid」 | 组件 |
| E0-T7 | 503 + `queuePosition:1` + `retryAfterMs` | stage 展示排队；计时后第二次 POST；不立刻 `down` | 组件（假时钟） |
| E0-T8 | 200 但 `webUrl` 为 `/dsh/` | 不 mount；`SESSION_DENIED` 或明确 code | 组件 |
| E0-T9 | 快速切 workspace | 只采用最后 generation 的 webUrl | 组件 |
| E0-T10 | 关面板 | `closeDshSession` 调用；再开新 generation | 组件 |
| E0-T11 | `resolveDshAccessToken` 驼峰 | 能读 `accessToken` | 单测 |
| E0-T12 | inject `/dsh/` | CSP 为 `frame-src 'self' https://openmuseai.com`；无模型 key | py |
| E0-T13 | 内容区在 pending/error/iframe 外 | 不存在；扫描 render 分支 | 组件 + code review |

---

## 5. 生产验收（E0 出口）

环境：`https://openmuseai.com/app/<workspaceId>`，已登录，硬刷新。

| ID | 操作 | 期望 |
|---|---|---|
| E0-P1 | 打开 Agent，Network 选 Fetch/XHR | 出现 `POST /api/muse/dsh/session/open` |
| E0-P2 | 该 POST 200 | body 含 `/u/`；随后 `GET /u/<hash>/` 200（Cookie 带上） |
| E0-P3 | 无 `GET` 文档/iframe 到 `/dsh/` | 或仅调试导航，iframe src 不含 `/dsh/` |
| E0-P4 | 未登录打开 | 面板文案登录，无 open POST |
| E0-P5 | 首次当日冷启动 | 保持「正在启动」类文案至返回，不变黑；允许 >60s |
| E0-P6 | 面板 `data-attachment-stage` | 非空；失败时 `data-error-code` 非空 |

未过 E0-P1 不得宣称「DSH 挂了」——先查 Host。

---

## 6. 实现记录（2026-09-10）

代码已按 §2 落地，矩阵自动化如下。

| 路径 | 状态 |
|---|---|
| `frontend/web/.../dsh-attachment.ts` | 已加 stage/code/generation/`queuedCopy` |
| `dsh-session.ts` | `SessionOpenResult` = ready \| queued \| denied；503 不再盲等 2s |
| `DshAgentPanel.tsx` | 单协调 effect；`data-attachment-stage` / `data-error-code` / `data-attachment-id`；内容区三分支 |
| `dsh-app-state.ts` | Auth 上下文 \|\| 路由 `workspaceId` |
| `contracts/dsh-attachment-error-codes.v1.json` | 已建 |
| `test_inject_web_config.py` | E0-T12 已覆盖 CSP apex（W1-05） |

Jest：`dsh-attachment` / `dsh-session` / `dsh-origin` / `DshAgentPanel` **31+ 通过**（E0-T1–T11、T13；T3 在 origin；T12 在 py）。

生产（2026-09-10）：已 `DEPLOY_ENV=production` 打 Web dist。公开 `/app/` 的 `__APP_CONFIG__` 仍为 CSP 用 `https://openmuseai.com/dsh/`；`frame-src 'self' https://openmuseai.com`。懒加载 chunk 含 `data-attachment-stage`、`X-Muse-Attachment-Id`、`/api/muse/dsh/session/open`、`canMountDshIframe`。未启动 Docker `muse-dsh`。E0-P1–P6 仍需已登录工作区硬刷新：Network 选 Fetch/XHR 看 `session/open`，随后 `GET /u/…`，iframe src 不得为 `/dsh/`。

