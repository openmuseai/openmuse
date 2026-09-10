# E1 Bind RPC — 设计 / 实现 / 测试

> 阶段：E1。依赖 E0（iframe 已按 `webUrl` mount）。合同：[HOST-EMBEDDING-DESIGN.zh-CN.md](../HOST-EMBEDDING-DESIGN.zh-CN.md) §4.3 / A2 / A3 / A8。
> 平台：Web Host + DSH `parent-bridge` 注入脚本。Android 载波在 E2 复用同一 inbox 语义。

---

## 1. 设计

### 1.1 问题

联动是 fire-and-forget：

- `parent-hello` 与 `fetchDshDeviceToken` 并行，常在无 token 时发出；
- `workspace.bind` **不含** deviceToken，靠 `rememberDeviceAuth`；
- `PARENT_BRIDGE_SCRIPT` `xhr.send` 不读 401/403；
- 关 `MUSE_WEB_CONTEXT_UPLINK` 仍 `{ok:true}`；
- Host 在 iframe `contentWindow` 为空时 postMessage。
- 无 `MUSE_DOCUMENT_CLOUD_URL` 时整段 bridge 不注册 → 无 `frame-ready`。

用户看到「DSH 壳在，但不跟文档走」。

### 1.2 决策

把注入脚本升级为 **带 `requestId` 的 RPC**：

```text
Host → iframe:  { source: muse.appflowy-web, type, requestId, ... }
脚本 → POST /muse/v1/parent-bridge
脚本 → parent: { source: muse.dsh-web, type: 'bridge.reply', requestId, status, body }
```

Host 只在 `bridge.reply` 且 `body.ok && bound === workspaceId` 后进入 `HybridLive`。

顺序：

```text
Presenting (E0 iframe onLoad)
  → 等 frame-ready（超时 FRAME_TIMEOUT / BRIDGE_SILENT）
  → 若无 deviceToken：Failed NO_DEVICE_TOKEN（可重试 issue，不拆 session）
  → hello(deviceToken, workspaceRef) 等 ACK
  → bind(deviceToken, workspaceRef) 等 ACK
  → HybridLive；此后才 contribute / 听 intent
```

DSH `bindWorkspace`：消息必须自带合法 device token（远程）；**禁止**只靠 remembered 且 remembered 为空时静默。可保留 remembered 作 fallback，但 Host 契约是每次都带。

关旗：`context.contribute` / intent enqueue 返回 `{ok:false, error:'FLAG_OFF'}`，HTTP 403。

SSE：在 `parentOrigin` 未设置前 **缓冲** intent，设置后 flush。

### 1.3 不变量

DESIGN A2、A3、A8。另：

| ID | 陈述 |
|---|---|
| B1 | 每个 outbound Facet 消息有 `requestId`；Host 在 timeout 内要么 ACK 要么 Failed |
| B2 | `HybridLive` 之前 contribute 发送次数 = 0 |
| B3 | capabilities `GET /muse/v1/parent-bridge/capabilities` 失败 ⇒ 不得标 HybridLive |

### 1.4 不做

- 不改 Desktop hint。
- 不在本阶段改 mobile exclusive lease（E2）。
- 不把 JWT 放进 hello。

---

## 2. 实现

| 路径 | 改动 |
|---|---|
| `plugins/dsh-appflowy/src/parent-bridge.ts` `PARENT_BRIDGE_SCRIPT` | XHR onload/onerror → `bridge.reply`；SSE 缓冲 |
| 同文件 `handleParentInbound` | 关旗 `FLAG_OFF`；bind 远程无 token → `NO_DEVICE_TOKEN`（已有则保持） |
| `frontend/web/.../dsh-origin.ts` | `workspaceBindMessage` 增加 `deviceToken`/`deviceId`；`parseBridgeReply` |
| `frontend/web/.../dsh-hybrid.ts` | contribute 仅当 stage=HybridLive（由面板/hook 门闩） |
| `frontend/web/.../DshAgentPanel.tsx` | Binding 阶段；等 reply；capabilities 可选 HEAD/GET 同源 `/u/.../muse/v1/parent-bridge/capabilities`（经 iframe 同源，由脚本转或 Host 不跨源打实例——**只经 postMessage 让脚本 GET capabilities**） |
| `plugins/dsh-appflowy/docs/web-workspace-ingress.md` | 标明 fire-and-forget 作废 |
| `plugins/dsh-appflowy/tests/parent-bridge.test.ts` | reply 合同、FLAG_OFF、无 token bind |
| `contracts/dsh-attachment-error-codes.v1.json` | Binding 码 |

Host 不要直接 fetch 实例 loopback。capabilities 由注入脚本在 frame-ready 时带上，或 `bridge.reply` 一条 `type:'capabilities'`。

超时：hello/bind 默认 8s → `BRIDGE_SILENT`。

---

## 3. 测试方案

- parent-bridge 单测（已有 host-channel / parent-bridge.test）：补 RPC 与 FLAG_OFF。
- 注入脚本：用 jsdom 或把 `postHost` 抽到可测函数（避免只测巨型字符串）。
- Web 组件：mock `postMessage` 循环，断言 contribute 在 ACK 前为 0。
- 禁止：只靠手工「问 DSH 当前工作区」。

---

## 4. 测试矩阵

| ID | 场景 | 期望 | 类型 |
|---|---|---|---|
| E1-T1 | hello 带合法 deviceToken + workspace | HTTP 200，`bound` 为该 id | 单测 DSH |
| E1-T2 | 远程 bind 无 token、无 remembered | `NO_DEVICE_TOKEN`，非 200 ok | 单测 DSH |
| E1-T3 | bind 消息自带 token | 不依赖前序 hello 的 remember | 单测 DSH |
| E1-T4 | 注入脚本 XHR 401 | parent 收到 `bridge.reply` status=401 | 单测脚本 |
| E1-T5 | 关 CONTEXT_UPLINK 后 contribute | `{ok:false, error:FLAG_OFF}` | 单测 DSH |
| E1-T6 | Host：bind 未 ACK | 0 次 contribute；stage≠HybridLive | 组件 |
| E1-T7 | Host：ACK bound 匹配 | 允许 contribute；`data-attachment-stage=HybridLive` | 组件 |
| E1-T8 | hello JSON 含 `access_token` | FORBIDDEN_FIELD；Host 非 HybridLive | 单测 |
| E1-T9 | frame-ready 后 8s 无 reply | `BRIDGE_SILENT`；iframe 可仍在 | 组件假时钟 |
| E1-T10 | 先 SSE intent 后 parentOrigin | 缓冲后仍投递一条 intent.dispatch | 单测脚本 |
| E1-T11 | 伪 device token | 401 `DEVICE_AUTH_REJECTED`；Host `BIND_REJECTED` | 单测 + 组件 |
| E1-T12 | 跨 workspace token | 403 `SCOPE_MISMATCH` | 单测（扩 P0-T4） |

---

## 5. 生产验收

| ID | 操作 | 期望 |
|---|---|---|
| E1-P1 | E0 链成功后 | Network 可见同源 POST `.../muse/v1/parent-bridge`（在 iframe 上下文；可用 iframe initiator 过滤） |
| E1-P2 | 该 POST 200 且 body `bound` | 面板 stage=HybridLive |
| E1-P3 | 故意清 device-token 再 bind | 面板 `NO_DEVICE_TOKEN`，DSH 壳可在 |
| E1-P4 | Agent 打开当前页 | Host 发生导航/高亮（intent）；无则查 FLAG 与 SSE |

E1-P1 在 DevTools 需看 iframe 子帧请求；文档里写明，避免再出现「主文档 Network 没有就当没发」。

---

## 6. 实现记录（2026-09-10）

| 项 | 状态 |
|---|---|
| `PARENT_BRIDGE_SCRIPT` | XHR `onload`/`onerror` → `bridge.reply`；SSE `pending` 缓冲 |
| `handleParentInbound` context 关旗 | `{ok:false, error:FLAG_OFF}` HTTP 403 |
| `workspaceBindMessage` | 携带 `deviceToken` / `deviceId` / `requestId` |
| `parseBridgeReply` | Host 收到 `ok+bound` → `HybridLive` |
| contribute | 仅 `HybridLive`（E1-T6 代码门闩） |

已测：parent-bridge FLAG_OFF + 脚本含 `bridge.reply`；Web parseBridgeReply / bind 带 token。

组件矩阵：E1-T6 / T7 / T9 / T11。脚本：T4 / T10（`ParentOriginBuffer` + `makeBridgeReply`）。DSH：T1–T3 / T5 / T8 / T11 / T12。fire-and-forget 已在 `web-workspace-ingress.md` 作废。

生产（2026-09-10）：已将 `parent-bridge.js` + `parent-bridge-runtime.js` 同步到 `/opt/muse-dsh/runtime/dsh/node_modules/@muse/dsh-appflowy/dist/src/`（含 `flushPending` / `bridge.reply`），并 `systemctl restart` 当时唯一活实例；loopback 随后 200。`MUSE_DOCUMENT_CLOUD_URL` 已在实例环境中。E1-P1–P4 仍需登录后在 iframe initiator 下看 `POST .../muse/v1/parent-bridge`。

