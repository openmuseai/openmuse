# @muse/dsh-appflowy 源码解读

> 工作区与文档 Cloud/E2E 已迁出：`@muse/plugin-appflowy-workspace`、`@muse/plugin-appflowy-markdown`（`host`/`cloud`）。装配职责以 [ROLE.zh-CN.md](ROLE.zh-CN.md) 与 [TECH.zh-CN.md](../TECH.zh-CN.md) 为准。下文仍有旧路径名，仅作历史对照。

## 1. 包定位

```
@muse/dsh-appflowy (private, 0.1.0) — "installable DSH bundle"
```

这是一个 **DSH（DeepSeek Harness）可安装 bundle**：它把 Muse Host Bridge、AppFlowy 能力插件装配进 DSH 运行时，使部署在某台机器上的 **AppFlowy 桌面/Web 客户端** 能与 DSH 里的 AI agent 双向协作：

- **下行**：DSH agent 通过 Muse 协议读写 AppFlowy 文档（Markdown），或请求 Web UI 打开视图/滚动到块。
- **上行**：AppFlowy-Web 以 iframe 嵌入 DSH Web UI，把当前工作区、文档焦点、设备凭据传给 DSH。

装配清单是 [`cordis.patch.yml`](../cordis.patch.yml)，包内声明的 `dsh.bundle.patch` 字段即指向它。生产环境下 sidecar 进程由 Flutter 壳拥有，因此 `dsh-market` 等插件配置了 `allowRestart: false`。

### 外部依赖（package.json）

| 依赖 | 用途 |
|---|---|
| `@muse/host-bridge` (link) | UDS/进程内传输、`MuseHostConnectorService`、`WireEnvelope` 类型 |
| `@muse/plugin-appflowy-markdown` (link) | `APPFLOWY_MARKDOWN_*` operation 常量和 4 组输入/输出 JSON Schema |
| `@muse/plugin-facets` (link) | `ContextContributionEnvelopeV1` / `PresentationIntentEnvelopeV1` 类型 |
| `@muse/context-broker` (link) | 上下文投影（ingest / pin / remove / registerProjection） |
| `@muse/plugin-appflowy-view-reference` / `view-rename` (link) | 其他 AppFlowy 能力插件（视图引用/重命名） |
| `@deepseek-ai/dsh-tools` (link) | `defineTool`（本包注册 `muse_appflowy_present` 工具） |
| peer: `@deepseek-ai/cordis` ^4.0.1 | DSH 容器（Context / plugin / effect 生命周期） |
| peer: `@deepseek-ai/dsh-system-prompt` | vertical-slice 测试装配系统提示词 |

### 对外导出（exports 字段）

| 导出 | 内容 |
|---|---|
| `@muse/dsh-appflowy/connector` | `AppFlowyConnector`（默认）、`InProcessAppFlowyConnector` |
| `@muse/dsh-appflowy/workspace` | 工作区绑定插件 `apply` + hint 工具函数 |
| `@muse/dsh-appflowy/webview` | WKWebView scoped URL 编码插件 |
| `@muse/dsh-appflowy/parent-bridge` | 父桥插件 `apply` + HTTP 路径常量 |

`approval.ts`、`cloud-document.ts`、`scope.ts` 是内部模块，不参与 exports，仅被 connector/parent-bridge 内部引用。

## 2. 模块总览

| 模块 | 行数 | 职责 | 装配插件名 |
|---|---|---|---|
| `connector.ts` | 344 | 连接器：Muse Host 握手/发现/绑定/调用；Cloud 或原生 UDS | `@muse/dsh-appflowy/connector` |
| `parent-bridge.ts` | 572 | 父桥：iframe ⇄ DSH 的 HTTP + SSE 桥、上下文上行、意图下行、投影 | `@muse/dsh-appflowy/parent-bridge` |
| `workspace.ts` | 252 | AppFlowy 工作区 ⇄ DSH workspace 绑定、hint 文件监听、防删除 | `@muse/dsh-appflowy/workspace` |
| `cloud-document.ts` | 136 | 云文档适配器：4 个 operation → Cloud HTTP，fail-closed | 内部 |
| `webview.ts` | 70 | WKWebView `/plugins/@scope` URL 编码补丁 | `@muse/dsh-appflowy/webview` |
| `approval.ts` | 41 | 审批证明：部署方 HMAC，插件不可见 secret | 内部 |
| `scope.ts` | 28 | 调用作用域校验：invoke workspace 必须匹配绑定工作区 | 内部 |

## 3. connector.ts — 连接器与文档操作入口

### 3.1 三种连接模式（按优先级）

| 模式 | 触发条件 | 传输 | 文档数据来源 |
|---|---|---|---|
| **Cloud 模式** | `MUSE_DOCUMENT_CLOUD_URL` 已设置 | `InProcessMuseHostTransport`（进程内假 Host） | `invokeCloudDocument()` HTTP → AppFlowy Cloud `/api/muse/document/*` |
| **原生模式（默认）** | 未设置 Cloud URL | `DesktopMuseHostTransport`（本地 UDS，端点在 launch descriptor 中） | 真正的 Rust Host（AppFlowy Core） |
| **E2E fixture** | `InProcessAppFlowyConnector`（仅测试导入） | `InProcessMuseHostTransport` | 固定 fixture Markdown（`MUSE_APPFLOWY_E2E_MARKDOWN` 可覆盖） |

关键设计：**Cloud 模式不改连接器结构**——`AppFlowyCompositionHandler` 同时承担"进程内假 Host"和"Cloud 请求转发"两种角色，`open()` 时仅按配置选择 transport 与 handler 是否携带 `baseUrl`。

### 3.2 AppFlowyCompositionHandler

实现 `TransportHandler.unary()`，处理 6 种 WireEnvelope 请求：

| kind | 响应要点 |
|---|---|
| `hello.request` | 版本 1.0、`HARD_LIMITS`、`hostSessionId` / `hostGeneration`（`appflowy.e2e.1`）、时钟容差 1000ms |
| `discover.request` | 注册 1 个描述符 `appflowy.document.local`（revision 3，family = `APPFLOWY_MARKDOWN_FAMILY`，contract v2.0） |
| `bind.request` | `binding.appflowy-markdown`，300s 有效期，附 4 个 operation 的 schema digest |
| `policy.evaluate.request` | `approval_required`，approvalId `approval.e2e`，60s |
| `policy.finalize.request` | `approved` + `grant.e2e`，60s |
| `invoke.request` | 见 3.3 |

描述符声明 4 个 operation（`effect` / 幂等性见注释）：

| operationId | effect | idempotency | Cloud 路径 |
|---|---|---|---|
| `APPFLOWY_MARKDOWN_READ_OPERATION` | read | none | `/api/muse/document/query` |
| `APPFLOWY_MARKDOWN_PROPOSE_OPERATION` | read | none | `/api/muse/document/propose` |
| `APPFLOWY_MARKDOWN_APPLY_OPERATION` | local_write | **required** | `/api/muse/document/apply` |
| `APPFLOWY_MARKDOWN_STATUS_OPERATION` | read | none | `/api/muse/document/status` |

### 3.3 invoke.request 处理链（Cloud 模式）

```
invoke.request
  ├─ bindingId ≠ BINDING_ID          → OPERATION_NOT_FOUND
  ├─ cloudDocumentPayload()          补齐 workspaceId / viewId（缺省取最近文档焦点或绑定工作区）
  ├─ assertWorkspaceScope(bound, ...) 不匹配 → SCOPE_MISMATCH
  ├─ auth = getLastDeviceAuth()      取父桥记忆的设备 token（Cloud accessToken 优先）
  ├─ invokeCloudDocument()            POST {operation, ...payload}，附 Authorization + X-Muse-Device-Id
  └─ 成功 → invoke.response { value, receipt{status:"applied_local"} }
      失败 → invoke.response { ok:false, error{kind:"bridge", code, retryable:false} }
```

非 Cloud 分支直接返回 fixture 快照/提案/回执（`document.e2e`），供离线与测试使用。

### 3.4 启动身份（原生模式）

原生模式读取 **AppFlowy Core 写入的私有 launch descriptor**（路径 `$TMPDIR/appflowy-muse-host-<uid>.json`，可被 `MUSE_APPFLOWY_LAUNCH_FILE` 覆盖）：

- 强校验：`isFile()`、**uid == 当前用户**、`mode & 0o077 == 0`（非 0600 拒绝）、字段合法性（endpoint 以 `/` 开头、opaque 格式、nonce ≥ 32 字符）。
- 字段：`endpoint`（UDS）、`nonce`、`hostGeneration`、`runtimeInstanceId`。
- 产出 `RuntimeProof { runtimeInstanceId, nonce }` 供 DSH 验证 Host 真实身份。

## 4. parent-bridge.ts — 父桥（最大模块，572 行）

### 4.1 三个 HTTP 端点

| 端点 | 方法 | 用途 |
|---|---|---|
| `/muse/v1/parent-bridge` | POST | 父 iframe → DSH 入口（接收 postMessage 转发） |
| `/muse/v1/parent-bridge/events` | GET | SSE 下行通道（DSH → 父 iframe 推送 `intent.dispatch`） |
| `/muse/v1/parent-bridge/intents` | POST | 直接提交意图（供外部 HTTP 调用方，等价于工具入队） |

统一限制：`MAX_PARENT_MESSAGE_BYTES = 32KB`。

### 4.2 入站消息处理链 `handleParentInbound`

```
raw
 ├─ byteLength > 32KB                       → PAYLOAD_TOO_LARGE (413)
 ├─ JSON 不可序列化                           → INVALID_JSON (400)
 ├─ jsonLooksForbidden(serialized)           → FORBIDDEN_FIELD (400)
 │     正则 /access_token|refresh_token|api[_-]?key/i  —— 上行禁用凭据字段
 ├─ 非对象 / source ≠ "muse.appflowy-web"    → INVALID_MESSAGE / INVALID_SOURCE
 └─ type 分发：
     parent-hello      → rememberDeviceAuth (deviceToken 两段式 JWT 且非 sk- 前缀) + 绑定
     workspace.bind    → bindWorkspace → applyWorkspaceHint(registry)
     context.contribute→ ingestContribution → broker；rememberDocumentFocus；pinSurface
                         （受 MUSE_WEB_CONTEXT_UPLINK 门控）
     surface.closed    → broker.removeSurface(ref)
     intent.receipt    → 直接 ok（回执确认，无后续处理）
```

**记忆状态**（模块级、供 connector 复用）：

| 状态 | 内容 | 消费方 |
|---|---|---|
| `lastDeviceAuth` | `{ token, deviceId }` | connector 的 Cloud 调用鉴权 |
| `lastDocumentFocus` | `{ workspaceId, viewId }` | connector 补全文档 payload |
| `lastWorkspaceSurface` | `surface.appflowy.workspace.<id>` | 表面切换清理 |

其中文档焦点只从 `workspace.focus` / `markdown.surface` / `markdown.selection` 三类 contextType 且 workspaceId+viewId 齐备时写入。

### 4.3 下行意图 `enqueuePresentationIntent`

```
muse_appflowy_present 工具  ┐
POST /intents 端点          ┴ → buildPresentationIntent()
   ├─ intent 类型: surface.open | surface.revealRange（默认 open）
   ├─ workspaceId 缺省取绑定工作区；与绑定不一致 → SCOPE_MISMATCH（工具路径）
   ├─ 30s 过期窗口；>32KB 拒绝；< 转义为 \u003c（防 XSS）
   └─ SSE data: {"source":"muse.dsh-web","type":"intent.dispatch","intent":{...}}
```

门控 `MUSE_WEB_INTENT_DOWNLINK`（默认开启）。`muse_appflowy_present` 工具说明："Ask the AppFlowy Web UI to open a view or reveal a block… Does not write document text."

### 4.4 index.html 注入脚本 `PARENT_BRIDGE_SCRIPT`

- `window.addEventListener("message")`：仅收 `data.source === "muse.appflowy-web"`，记录 `ev.origin` / `ev.source`，POST 到 `/muse/v1/parent-bridge`。
- `new EventSource("/muse/v1/parent-bridge/events")`：收到事件 → `parentWin.postMessage(parsed, parentOrigin)`。
- 注入位置：`<head>` 之后；无 `<head>` 则前置。脚本体不含裸 `<` 与 `</script>`（避免 HTML 解析破坏）。

### 4.5 上下文投影（Context Projection）

注册 2 个投影到 `museContextBroker`：

| contextType | schemaDigest | priority | maxTokens | 渲染内容 |
|---|---|---|---|---|
| `workspace.focus` | `WORKSPACE_FOCUS_DIGEST` | 110 | 80 | 当前工作区 id/标题/视图 |
| `workspace.tree.ui` | `WORKSPACE_TREE_UI_DIGEST` | 40 | 120 | 侧边栏展开视图 id 列表（≤64 个） |

两个 schemaDigest 常量与 surface open/reveal 的 digest 一起导出（`WORKSPACE_FOCUS_DIGEST`、`WORKSPACE_TREE_UI_DIGEST`、`SURFACE_OPEN_DIGEST`、`SURFACE_REVEAL_DIGEST`），用于与 facet schema 对账。

## 5. workspace.ts — 工作区绑定

### 5.1 概念

AppFlowy 工作区（数据库里的 workspace）映射为 DSH 里的一个 workspace（本地 cwd 目录，默认 `$DSH_HOME/appflowy-workspaces/<sanitized-id>`）。文档内容**不落盘为文件**——统一走 Muse 工具读写，目录里只放一个说明性 `README.md`（`flag:"wx"` 幂等写入）。

### 5.2 hint 文件（单点事实来源）

Flutter 壳在 AppFlowy 当前工作区变化时写入 `$DSH_HOME/bindings/current-appflowy-workspace.json`：

```json
{ "appflowyWorkspaceId": "...", "title": "...", "updatedAt": 1234567890 }
```

`apply` 插件启动时立即 `applyHintFile()`（文件不存在/解析失败则静默跳过），随后 `watchAppFlowyWorkspaceHint()`：监听**目录**（`persistent:false`），文件名匹配则 50ms debounce 后重新绑定。返回的 stop 函数挂到 `ctx.effect` 以便卸载时清理。

### 5.3 防删除（pin）

`pinPath(registry, canonical)` 用 WeakMap 记录"已固定的 canonical 路径"，并 **monkey-patch `registry.delete`**：删除位于固定集合中的 workspace 时抛 `AppFlowyWorkspacePinnedError`（DSH 的 create/rename/delete 对其他 workspace 不受影响）。已存在的 workspace 会 `setTitle` 对齐标题并 `insertBefore` 置顶。

### 5.4 环境变量覆盖

| 变量 | 作用 |
|---|---|
| `DSH_HOME` | DSH 根目录（默认 `~/.dsh`） |
| `MUSE_APPFLOWY_DSH_WORKSPACE` | 单目录锁死（tests / legacy，跳过按 id 分目录） |
| `MUSE_APPFLOWY_DSH_WORKSPACE_ROOT` | 多工作区根目录（默认 `$DSH_HOME/appflowy-workspaces`） |
| `MUSE_APPFLOWY_WORKSPACE_HINT` | hint 文件路径覆盖 |
| `MUSE_APPFLOWY_WORKSPACE_TITLE` | 标题覆盖（默认 `AppFlowy`） |

`sanitizeWorkspaceId`：非 `[A-Za-z0-9._-]` → `_`，去首尾 `_`，空则 `workspace`，最长 128。

## 6. cloud-document.ts — 云文档适配器（fail-closed）

把 markdown 插件 4 个 operation 映射到 Cloud HTTP：

```http
POST {baseUrl}/api/muse/document/{query|propose|apply|status}
Content-Type: application/json
Authorization: Bearer <accessToken>     # 可选
X-Muse-Device-Id: <deviceId>            # 可选
body: { "operation": <operationId>, ...payload }
```

**失败闭合规则**（顺序执行），核心目标：**绝不让 DSH 把"未真正落库"的写操作当作成功**：

1. 响应不是对象 → `UNAVAILABLE`
2. `code !== 0`（含 `CLOUD_FEATURE_NOT_AVAILABLE = 1067`）→ `UNAVAILABLE`（message 取自 `message` 字段或 `CLOUD_COLLAB_ADAPTER_NOT_WIRED`）
3. **apply 特殊**：`data.status === "applied"` 但 `MUSE_DOCUMENT_CLOUD_APPLY_ENABLED !== "1"` → 视为伪造回执，`UNAVAILABLE`（测试 `rejects a forged applied receipt`）
4. `data === undefined` → `UNAVAILABLE`
5. 非 JSON 响应：401/403 → `unauthorized`；404 → `NOT_WIRED`；其他 → `invalid (HTTP n)`
6. 网络异常 → `document adapter unreachable`

未知 operationId → `OPERATION_NOT_FOUND`（不发起网络请求）。

## 7. webview.ts — WKWebView scoped URL 补丁

**问题**：WKWebView 把 `/plugins/@scope/...` 中 `@` 当作 userinfo 分隔符 → URL 畸形 → classic `<script src>` 加载失败（iOS 壳）。Chromium 不受影响（Node `/plugins` 处理器会 `decodeURIComponent`）。

**方案**（三层，注释原文要点）：
1. `encodeScopedPluginUrls(html)`：`replaceAll("/plugins/@", "/plugins/%40")` + 注入内联脚本。
2. 内联脚本 `SCOPED_PLUGIN_URL_PATCH_SCRIPT`：
   - 立即重写 `window.__DSH_BOOT__.entries[].url`；
   - 覆写 `HTMLScriptElement.prototype.src` setter（编码后落盘）；
   - 覆写 `Element.prototype.setAttribute`（`name === "src"` 时编码）。
3. 脚本体不含裸 `<` / `</script>`。

因为 client-modules 的 tap 可能在 `__DSH_BOOT__` 注入**之前**执行，所以保留 `replaceAll` 兜底（适用于 tap 在 boot 之后注册的场景）。插件依赖 `webServer.tapIndex`，缺依赖直接抛错。

## 8. approval.ts — 审批证明（HMAC）

- 描述符：`$TMPDIR/appflowy-muse-approval-<uid>.json`（可被 `MUSE_APPFLOWY_APPROVAL_FILE` 覆盖），强校验：`isFile`、属主、`mode & 0o077 == 0`、`secret` 为字符串且 ≥ 32 字符。
- `expectedHostProofId(secret, approvalId)` = `"proof." + hex(HMAC-SHA256(secret, approvalId))`。
- `createHostHmacApprovalRequester()` 返回 `MuseApprovalProofRequester`：approvalId 校验 opaque 格式（`^[A-Za-z0-9._~-]{1,128}$`）；secret 加载或计算失败一律返回 `{ outcome: "unavailable" }`（**不泄露失败原因给插件**）。
- 在 connector 构造函数注册：`setMuseApprovalProofRequester(ctx, ...)`。注释明确：**插件永远看不到 HMAC secret**。

## 9. scope.ts — 调用作用域校验

`assertWorkspaceBound(boundWorkspaceId, input)`：解析 `input.workspaceId`（trim 后非空)；仅当 **绑定工作区与请求工作区都存在且不等** 时返回 `SCOPE_MISMATCH`。即：未绑定或请求未指定时不拦截；一旦双方明确，必须一致——防止 agent 在 A 工作区上下文里操作 B 工作区文档。

## 10. 三条主数据流

### 10.1 文档读写（DSH agent → Cloud / 原生 Host）

```
DSH agent 工具(plugin-appflowy-markdown)
  → ctx.museHost (host-bridge/dsh)
  → connector.open() transport
      ├─ Cloud:  AppFlowyCompositionHandler → invokeCloudDocument → AppFlowy Cloud /api/muse/document/*
      └─ Native: DesktopMuseHostTransport(UDS) → AppFlowy Core Rust Host
  → 回执(proof, receipt) → 工具结果呈现给 agent
```

### 10.2 上下文上行（AppFlowy-Web iframe → DSH 上下文）

```
AppFlowy-Web postMessage{source:"muse.appflowy-web", type:"context.contribute", ...}
  → 注入脚本 → POST /muse/v1/parent-bridge
  → handleParentInbound（大小/敏感字段/source 校验）
  → broker.ingestContribution(envelope)
  → 投影 render（workspace.focus / workspace.tree.ui）→ DSH 上下文（受 MUSE_WEB_CONTEXT_UPLINK 门控）
```

### 10.3 意图下行（DSH → AppFlowy-Web UI）

```
agent 调用 muse_appflowy_present / 外部 POST /intents
  → buildPresentationIntent（30s 过期，scope 校验）
  → enqueuePresentationIntent（SSE 广播，< 转义）
  → 注入脚本 EventSource 收到 → postMessage → AppFlowy-Web 打开视图/滚动块
```

## 11. 环境变量总表

| 变量 | 默认 | 作用 |
|---|---|---|
| `MUSE_DOCUMENT_CLOUD_URL` | 无 | 设置后启用 Cloud 文档适配器（baseUrl，去尾 `/`） |
| `MUSE_DOCUMENT_CLOUD_TOKEN` | 无 | Cloud access token（优先于父桥记忆的 deviceToken） |
| `MUSE_DOCUMENT_CLOUD_APPLY_ENABLED` | `0`/未设 | `=1` 才认可 `status:"applied"` 回执 |
| `MUSE_APPFLOWY_LAUNCH_FILE` | `$TMPDIR/appflowy-muse-host-<uid>.json` | 原生启动描述符路径 |
| `MUSE_APPFLOWY_APPROVAL_FILE` | `$TMPDIR/appflowy-muse-approval-<uid>.json` | 审批描述符路径 |
| `MUSE_APPFLOWY_E2E_MARKDOWN` | fixture 文本 | E2E 连接器的 fixture 文档内容 |
| `DSH_HOME` | `~/.dsh` | DSH 根目录 |
| `MUSE_APPFLOWY_DSH_WORKSPACE` | 无 | 单工作区目录锁死（tests/legacy） |
| `MUSE_APPFLOWY_DSH_WORKSPACE_ROOT` | `$DSH_HOME/appflowy-workspaces` | 多工作区根目录 |
| `MUSE_APPFLOWY_WORKSPACE_HINT` | `$DSH_HOME/bindings/current-appflowy-workspace.json` | hint 文件路径 |
| `MUSE_APPFLOWY_WORKSPACE_TITLE` | `AppFlowy` | 绑定 workspace 标题 |
| `MUSE_WEB_WORKSPACE_BIND` | on（`0/false/off` 关闭） | 是否响应 `workspace.bind` |
| `MUSE_WEB_CONTEXT_UPLINK` | on | 是否接收 `context.contribute` |
| `MUSE_WEB_INTENT_DOWNLINK` | on | 是否推送 intent（SSE） |

## 12. 测试矩阵（vitest）

| 文件 | 覆盖 |
|---|---|
| `vertical-slice.test.ts` | M05 纵向切片：Cordis 装配 → read/propose/apply → 一个回执；工具卸载后 schema 清空 |
| `cloud-document.test.ts` | W3 fail-closed：1067 → UNAVAILABLE；伪造 applied 被拒；`APPLY_ENABLED=1` 放行；401 区分 unauthorized |
| `parent-bridge.test.ts` | 消息校验（大小/敏感字段/source）、workspace 绑定、surface 清理、intent 入队（187 行） |
| `workspace.test.ts` | hint 解析、目录/标题/sanitize、防删除 pin、readme 幂等 |
| `connector.test.ts` | 握手/发现/绑定响应结构 |
| `webview.test.ts` | `/plugins/@` → `/plugins/%40` 编码与注入位置 |
| `approval.test.ts` | HMAC proof 派生、描述符校验 |
| `scope.test.ts` | 双 null / 单向 / 不匹配三种场景 |

命令：`pnpm test`（vitest）、`pnpm typecheck`、`pnpm build`、`pnpm check`（三者串联）。

## 13. 设计要点与潜在注意点

1. **fail-closed 是主基调**：cloud 适配器的每条错误路径都返回 `UNAVAILABLE`，绝不把未确认的写操作报告成功；审批/启动描述符失败也不向插件泄露原因。
2. **双通道分离**：HTTP（入站，POST）与 SSE（出站）独立端点，iframe 只需 `postMessage` + `EventSource` 即可双向通信，无需 DSH 内部 API 暴露给 Web。
3. **上下文上行禁用凭据字段**：`FORBIDDEN` 正则防止 Web 端把 token/api key 误传上来；下行则对 `<` 做 `\u003c` 转义防注入。
4. **Cloud 模式下 `HOST_SESSION_ID` / `HOST_GENERATION` 仍是 e2e 预留值**（`host-session.appflowy-e2e` / `appflowy.e2e.1`）——若接入生产 Host，需确认这两个标识按环境区分，避免与真实 Host 会话混淆。
5. **`registry.delete` 被 monkey-patch**：`pinPath` 依赖 `registry.get(id)` 返回可决议的 path，如果某个 registry 实现不维护 path 属性，pin 校验会静默失效（当前 DSH 实现满足契约）。
6. **凡是 `process.getuid` 不可用（Windows）**，属主校验自动跳过（`process.getuid !== undefined` 才检查），0600 位检查依然生效。