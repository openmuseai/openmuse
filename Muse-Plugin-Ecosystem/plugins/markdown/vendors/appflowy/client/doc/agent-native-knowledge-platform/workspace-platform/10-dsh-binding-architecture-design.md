# Host Project Workspace ↔ DSH Workspace 绑定与资源联动方案设计

状态：Design v1（2026-09-20）  
范围：Host（Dart Local Provider + DSH 中间层插件）、DSH（workspace registry、client UI、会话与工具）、资源打开与引用链路、多端协议  
前置：[09-dsh-binding-product-prd.md](./09-dsh-binding-product-prd.md)（需求 WBD/RLO/RCX/PBU/MPT）、[04-domain-model-and-provider-contract.md](./04-domain-model-and-provider-contract.md)、[05-system-architecture-and-dsh.md](./05-system-architecture-and-dsh.md) §3、[08-local-p0-implementation-and-acceptance.md](./08-local-p0-implementation-and-acceptance.md) §2.6、[../03-universal-resource-protocol.md](../03-universal-resource-protocol.md)  
结论：不在 UI 与本机路径之间新开"传 path"的通道；把 Local P0 的 hint 升级为 `muse.workspace/binding/v1` 绑定文档，由 DSH 侧 Host 插件按 Mount 逐个走"挂载 → materialize → 注册 DSH workspace"流程，`cwd` 仅作兼容投影。

分工：DSH→Host 打开路由与统一资源协议的客户端部分（`ResourceRef`/`describe`/`materialize`/`patch`、`SurfaceIntent`/`IntentReceipt`、`TargetRouter`、能力通道 op `target.open`、三个引擎的承载器注册）由 [../../agent-file-references-and-open-routing.md](../../agent-file-references-and-open-routing.md) 拥有，本文不重复设计。本文负责**绑定侧**：Mount ↔ DSH Workspace 映射、binding 文档与 materialization、`mountScopes` 授权与解析，以及上述链路在本轮必须补齐的前置条件（桌面下行、Windows 面板通道、回执）。

## 1. 目标与范围

本轮要实现 PRD 的 WBD（绑定）、RLO（DSH→Host 打开）、RCX（Host→DSH 引用与元信息）、PBU（panel 呈现）、MPT（多端）五组需求，分为三个阶段（见 §10）。设计必须同时满足：

1. **一一对应**：Host 的每个可用 Mount ↔ 一个 DSH Workspace，可双向追溯（PRD M-2）。
2. **不传 path**：Host 不把设备绝对路径当身份或授权交给 DSH；DSH 侧自行完成与 Host 等价的挂载流程（PRD §1.3）。
3. **多端同构**：Desktop 只是 Local Provider 恰好能 materialize 成本机路径；Web/Mobile 走同一合同（MPT-01/02）。
4. **不破坏既有验收**：08 §4/§5 的 Local P0 验收与 25 项 DSH suite 不得回退。

## 2. 现状架构分析

### 2.1 三层链路

```text
Flutter / Host（Dart）                          DSH 中间层（TS 插件）                      DSH 内核与 panel
──────────────────────────────                  ────────────────────────────               ─────────────────────────
MuseWorkspaceController                         @muse/plugin-appflowy-workspace             @deepseek-ai/dsh-workspace
  open() / mountLocalDirectory() / unmount()      identity.ts:                              workspaceRegistry
  _publishDshBinding()                              bindHostWorkspace(registry)             storages/workspace.json
DshWorkspaceBridge                                  watchAppFlowyWorkspaceHint()            @deepseek-ai/dsh-client-ui-workspace
  hint JSON（best-effort, tmp+rename）            dsh.ts:                                   sidebar.workspaces
                                                     muse_workspace_list_views               conversation.hero.workspace
LocalWorkspaceProvider (Dart)                      systemPrompt.context(order 42)
  providerId muse.workspace.local.v1             @muse/plugin-dsh-appflowy
  bind/stat/list/…                                  parent-bridge.ts:
                                                      parent-hello / workspace.bind
                                                      context.contribute → museContextBroker
                                                      intent.dispatch (muse.presentation-intent/v1)
```

### 2.2 实测现状（本机 Windows，2026-09-20）

| 事实 | 证据 |
|---|---|
| Host 定义含 2 个本地 Mount | `<Application Support>/OpenMuse/workspace-platform-v1/definitions/917dc134-….json`：`frontend`(order 0)、`helix`(order 1) |
| 绑定载体是 hint 文件，无版本/无 ack | `%APPDATA%\OpenMuse\dsh\bindings\current-appflowy-workspace.json`（177 B）：`{appflowyWorkspaceId,title,updatedAt,projectRoot}`，**无 `mounts`** |
| DSH 侧只注册了 primary | `%APPDATA%\OpenMuse\dsh\storages\workspace.json`：`ba80ad2f…` path=`…\openmuse\frontend`（title `My Workspace`，3 个 session） |
| panel 会话确实落在 primary | `storages/session_projcache/sessions/session-73c83945….json`：`cwd = D:\agentic\src\openmuse-io\openmuse\frontend` |
| 第二个 Mount 对 DSH 完全不可见 | `identity.ts` 的 hint 类型无 `mounts`；`helix` 未出现在任何 DSH 记录中 |
| 遗留 workspace 未清理 | 同文件含 `d7b4e252…`（改名前的 `…\DSH Office\Muse\dsh\appflowy-workspaces\917dc134-…`，0 会话）与 `8821cbb9…`（`C:\Users\wps\Desktop\self\work`，1 个旧会话） |
| hint 目录有残留 | `bindings/` 下 15 个 `*.tmp`（含带 `mounts` 的旧 414 B 版本） |
| prompt 与真实 cwd 矛盾 | `appflowy-workspace/src/dsh.ts:16-24` 恒为"cwd 里只有 README.md，glob/ls 不会列出页面"，但绑定后 cwd 是真实项目目录 |
| 远端池另有一套目录约定 | `middlewares/dsh/core/dsh-pool/src/pool.ts:315-317`：`MUSE_APPFLOWY_DSH_WORKSPACE=<home>/appflowy-workspaces/<workspaceRef>` |
| 打开消息**没有生产端** | 全仓 grep `resource.open` / `resource.diff.open` / `MuseHostResource`：只有 Dart 消费端与解析测试，无任何发送方；DSH 上游点击走 RPC `ctx.remote.session.openWorkspacePath`（`vendors/deepseek-harness/packages/client/ui-chat/src/client/apply.ts:120-126`、`ui-deliverables/src/client/ProducedFiles.tsx:124`） |
| Windows 面板**没有 JS 通道** | `dsh_embedded_view.dart:120-125` 只为 WKWebView/Android 注册 `MuseHostResource`；`_WindowsWebView`（`:180-247`）只有 `onLoadError`/`loadUrl`，无 `webMessageReceived` |
| 桌面**没有 host→client 下行** | `parent-bridge.ts:589-594` `parentBridgeEnabled()` 仅在设置 `MUSE_DOCUMENT_CLOUD_URL` 时为真 → SSE `intent.dispatch` 在桌面不可达 |
| 能力通道**按构造禁 path** | `muse-dsh-mobile/lib/src/capabilities/dsh_native_capability_codec.dart:32-40`：`forbiddenKeys = {token,path,contentUri,base64,arbitraryMethod,fetchUrl,intent}` |
| `resourceRef` 已是 opaque branded string | `middlewares/dsh/core/contract-resource/src/types.ts:3`、`ResourceDescriptorV1`（`:15-33`）；`present` 工具 schema 只接受 `resourceRef|mode|placement|anchorHint` 且拒绝 path（`dsh-tool-resource-present/src/index.ts:55-70,85,94-96`） |
| 打开解析只认 `sessionCwd`，不查 Mount | `resource_open_request.dart:123-163`：相对路径基于 `sessionCwd`、containment 仅 `dshConversation` 且跳过 `hostPicker`；`MuseLocalResourceRouter` 不依赖 `MuseWorkspaceController` |
| 打开动作无回执 | `dsh_agent_panel.dart:177`、`resource_surface_dialog.dart:25-39` 为 `unawaited` fire-and-forget |

### 2.3 断点表

| # | 断点 | 位置 | 后果 |
|---|---|---|---|
| B1 | hint 只投影 `mounts.first` | `dsh_workspace_bridge.dart`（`projectRoot = localMounts.first.rootLocator`） | 多根丢失 |
| B2 | DSH 侧不解析 `mounts` | `appflowy-workspace/src/identity.ts` | 即使 hint 带 `mounts` 也不生效 |
| B3 | 绑定载体无协议版本、无 receipt、无 schema | hint 文件本身 | 无法校验、无法做多端收敛（08 与 05 §3.1 之间缺一代） |
| B4 | Host 无 active Mount 概念 | `workspace_controller.dart`（`selectedEntryRef` 不持久化、不发布） | 选中不联动（P-2） |
| B5 | panel 无绑定可见性 | DSH client 侧无该插件；`ui-workspace` 只在 hero/sidebar 呈现 DSH 自己的 workspace | 用户无法知道/切换落点（P-3） |
| B6 | prompt 未区分"绑定真实目录"与"scratch 目录" | `dsh.ts:WORKSPACE_LIST_PROMPT` | 模型被误导不去看真实文件（G-5 反面） |
| B7 | 资源打开以 `path + cwd` 表达 | `dsh_embedded_view.dart:DshResourceOpenMessage{path,cwd,line,…}` → `MuseResourceSurfaceOpener` | 多根时无法判定归属，且路径跨边界（05 §7.3 要求先解析为 resourceRef） |
| B8 | Host→DSH 上下文贡献无资源元信息契约 | `parent-bridge.ts:369` `context.contribute` 仅透传给 `museContextBroker` | 引用只有裸文本，模型不知道来源文件（RCX 缺口） |
| B9 | 打开消息无生产端 | 见 §2.2；设计归属见 `../../agent-file-references-and-open-routing.md` §3.4 通路 A（`internals.openPath` 覆盖 + `target.open` op） | 点产出文件行/行内链接只到 DSH Sidebar 文本预览，落不到 Host 内置应用 |
| B10 | Windows 面板无 JS 通道 | `dsh_embedded_view.dart:180-247` | 即使补上生产端，Windows 上仍不可达（本产品当前主力端之一） |
| B11 | 桌面无 host→client 下行 | `parent-bridge.ts:589-594` | 云文档/资源打开意图无法下发到桌面 panel |
| B12 | 打开解析不查 Mount，容器校验基于消息自报的 `cwd` | `resource_open_request.dart:137-147` | 安全性质变成"DSH 页面声称的 cwd"，第二 Mount 既不是相对基准也过不了 containment |
| B13 | 打开动作无回执 | `dsh_agent_panel.dart:177` | 失败静默；与 `IntentReceipt` 设计不符 |

### 2.4 可复用资产（不要重造）

| 资产 | 位置 | 复用方式 |
|---|---|---|
| DSH workspace registry | `packages/workspace/workspace/src/{entity,spec,paths}.ts`；`packages/api/workspace-controller/src/commands.ts` | 直接调用 `workspaceRegistry`（Host 插件已有）与 `workspace/create|rename|delete|insertBefore`（client 侧） |
| 绑定/归一化与 watcher | `appflowy-workspace/src/identity.ts`（`applyWorkspaceHint`、`watchAppFlowyWorkspaceHint`、`bindHostWorkspace`、`inject=["workspaceRegistry"]`） | 演进而非重写 |
| 资源打开意图 | `dsh-appflowy/src/parent-bridge.ts`：`muse.presentation-intent/v1`、`surface.open` / `surface.revealRange`、`intent.receipt` | 扩展 payload（增 `resourceRef`/anchor），保留 `viewId` 兼容分支 |
| 上下文贡献 | `parent-bridge.ts:369` `context.contribute` → `museContextBroker.contribute` | 定义 typed contribution（引用块） |
| Host 侧打开入口 | `dsh_agent_panel.dart:176-196` → `MuseResourceSurfaceOpener.open(MuseResourceOpenRequest{sessionCwd,…})` | 改为按 `resourceRef` 优先、`path+cwd` 兜底 |
| Local Provider | `local_workspace_provider.dart`（`muse.workspace.local.v1`） | materialize 的 `host-path` 来源 |
| DSH 侧工作目录 env | `dsh-pool/src/pool.ts:315-317` | 多租户/远端场景复用同一绑定语义 |
| panel 内同源 HTTP | `appflowy-workspace/src/views.ts`（`/muse/v1/...`）、`parent-bridge.ts`（`/muse/v1/parent-bridge/intents`、SSE frames） | 新增 binding 查询/事件端点 |
| 插件物化 | `dsh_sidecar.dart`（`profiles/web/node_modules/@muse` 物化、`.muse-seeded` 标记） | 插件变更必须经该流程才在运行中的 app 生效 |

## 3. 目标架构

### 3.1 分层与组件

```text
┌──────────────────────────── Host（Flutter + Dart/Rust Provider）────────────────────────────┐
│ MuseWorkspaceController             DshBindingPublisher（新，替代 DshWorkspaceBridge）        │
│  mounts / activeMountRef / revision   → 写 muse.workspace/binding/v1 文档 + 读 receipt        │
│ LocalWorkspaceProvider                MuseResourceSurfaceOpener（按 resourceRef 打开）         │
│  bind/stat/list/watch → host-path     引用构造器（resourceRef + anchor + provenance）          │
└───────────────────────────────┬──────────────────────────────────────────────────────────────┘
                                │ ① binding 文档（原子写）+ receipt；② context contribution；③ intent.receipt
┌───────────────────────────────▼──────── DSH 中间层（@muse 插件）──────────────────────────────┐
│ @muse/plugin-workspace-binding（由 appflowy-workspace 演进）                                   │
│   BindingReader（校验 schema/version） → MountMaterializer → DshWorkspaceRegistrar             │
│   ActiveMountController（primary 置顶/新会话） · BindingReceiptWriter · LegacyPruner            │
│   HTTPSurface：GET /muse/v1/workspace/binding · GET .../events（SSE）                          │
│ @muse/plugin-dsh-appflowy（parent-bridge / host-channel / mobile-http / webview index patch）   │
│   workspace.bind / context.contribute / intent.dispatch 的 transport adapter（不含业务语义）     │
└───────────────────────────────┬──────────────────────────────────────────────────────────────┘
                                │ ④ workspaceRegistry 注册/置顶/pin；⑤ client 服务与状态
┌───────────────────────────────▼──────── DSH 内核 + Web panel ─────────────────────────────────┐
│ @deepseek-ai/dsh-workspace（registry, storages/workspace.json）                                │
│ @deepseek-ai/dsh-api-workspace-controller（workspace/create|…、快照）                          │
│ @muse/dsh-client-ui-workspace-binding（新）：chip + 切换器 + activate → uiWorkspace.startSession│
│ @deepseek-ai/dsh-client-ui-workspace（hero picker / sidebar.workspaces，保持不动）              │
└───────────────────────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 真源边界

| 问题 | 真源 | 说明 |
|---|---|---|
| 有哪些 Mount、顺序、只读、active | Host `ProjectWorkspaceV1` definition | 唯一真源；DSH 不回写 |
| 某 Mount 在当前设备解析到什么 | Host 设备绑定（`bindingKey → locator`）+ Provider | 不进 DSH 文档 |
| DSH 侧哪个 workspace 对应哪个 Mount | binding 文档 + 确定性 materialization（§4.5） | 可推导，不另存真源 |
| 会话权限与 scope | `DshWorkspaceBindingV1`（session 级） | `cwd` 只是兼容投影 |
| 引用与打开的元信息 | Host 资源身份（`resourceRef`/`entryRef`/anchor） | DSH 只持有 opaque ref |

### 3.3 三类数据流

1. **绑定面**：Host 写 binding → DSH 插件注册/更新 workspace（无 UI 参与）。
2. **资源面**：DSH 意图（`surface.open`）→ Host 校验 → WorkspaceManager → Surface Orchestrator → Tab。
3. **引用面**：Host 选择 → context contribution → DSH 会话事件 + composer chip → 模型按 ref 读取。

## 4. 协议设计

### 4.1 不变量

| ID | 不变量 |
|---|---|
| INV-1 | 绑定文档与引用消息中**不得出现设备绝对路径**；需要路径时使用 `resourceRef` + materialization 结果（后者只存在于 DSH 本机执行域） |
| INV-2 | 所有跨边界身份是 opaque 字符串：`workspaceRef/mountRef/entryRef/resourceRef/requestRef` |
| INV-3 | `cwd` 只作兼容投影；工具调用的授权由 binding + session policy 决定并在调用时重验 |
| INV-4 | 每个 DSH workspace 由 (mountRef, materializationMode) 确定性推导，重复发布幂等 |
| INV-5 | 任何写操作在 Provider 与 policy 两层拒绝只读 Mount，不依赖 UI 隐藏 |
| INV-6 | 模型可见的引用必须可从 session log 重建（模型可见 ⟺ 可记录） |

### 4.2 `muse.workspace/binding/v1`（Host → DSH 绑定文档）

沿用 04 的命名风格与 03 "不新增 wire 动词、按合同族发布" 的做法：

```json
{
  "protocol": "muse.workspace/binding/v1",
  "bindingRevision": "rev.7f3c…",
  "accountSpaceRef": "space.01J…",
  "workspaceRef": "ws.01J…",
  "title": "My Workspace",
  "activeMountRef": "mount.01J…b",
  "mounts": [
    {
      "mountRef": "mount.01J…a",
      "providerId": "muse.workspace.local.v1",
      "providerKind": "local",
      "displayName": "frontend",
      "bindingKey": "device-bound:frontend",
      "order": 0,
      "requestedMode": "read-write",
      "capabilities": ["metadata.read", "children.list", "content.read", "content.write", "changes.watch"],
      "materialization": { "mode": "host-path", "locatorRef": "loc.01J…a" }
    },
    {
      "mountRef": "mount.01J…b",
      "providerId": "muse.workspace.local.v1",
      "providerKind": "local",
      "displayName": "helix",
      "bindingKey": "device-bound:helix",
      "order": 1,
      "requestedMode": "read-write",
      "capabilities": ["metadata.read", "children.list", "content.read", "content.write", "changes.watch"],
      "materialization": { "mode": "host-path", "locatorRef": "loc.01J…b" }
    }
  ],
  "issuedAt": 1789372800000
}
```

要点：

- `materialization.locatorRef` 是 Host 侧 locator 的 opaque 引用；**Desktop 上 DSH 通过本机 loopback 端点换取真实路径**（见 §4.4 step 3），文档里不放路径。为兼容 Local P0 与减少一次往返，v1 允许 Host 在**同一设备**上把路径写入 DSH 独占的 `$DSH_HOME` 内部文件（`bindings/materialized/<mountRef>.path`，权限 0600，永不进入模型上下文与会话日志），文档本体保持无路径。这一点是显式例外的**唯一**位置，必须由测试防回归（§9.3）。
- `activeMountRef` 必须在 `mounts` 内；缺失视为 `mounts[0]`。
- 缺失/未知 `providerKind`、相对路径、重复 `mountRef`、越界 `order` 均视为非法文档 → 整体拒绝并上报（WBD-07），不做部分应用。
- `bindingRevision` 单调递增，用于幂等与乱序丢弃（旧 revision 到达不覆盖新状态）。

### 4.3 `DshWorkspaceBindingV1`（session 级，05 §3.1 落地）

DSH 侧的每个 session 持久化：

```ts
interface DshWorkspaceBindingV1 {
  protocol: "muse.dsh/workspace-binding/v1"
  bindingRef: OpaqueRef
  sessionRef: OpaqueRef
  workspaceRef: OpaqueRef            // Project Workspace
  mountScopes: Array<{ mountRef: OpaqueRef; rootResourceRef: ResourceRef; grants: WorkspaceCapability[] }>
  primaryMountRef?: OpaqueRef
  createdAt: number
  policyRevision: RevisionToken
}
```

与 DSH workspace 的关系：DSH workspace 提供**执行域与 session 归属**；binding 提供**授权**。一个 session 的 `cwd` 落在某个 DSH workspace（= 某个 Mount 的 materialization），binding 的 `mountScopes` 允许它在同一 Project Workspace 的**其他** Mount 上做工具调用（P0 仅实现只读跨 Mount 读取，写操作仍在 primary 内，逐步放开）。

### 4.4 Mount materialization 矩阵与流程

| providerKind | 支持 exec | materialization | DSH workspace | UI 能力声明 |
|---|---|---|---|---|
| `local`（Desktop） | 是 | `host-path`（本机目录，只读时以 policy 限制） | 绑定 | 完整（终端/LSP/watch） |
| `local`（Web 授权目录/虚拟 Provider） | 否 | `semantic-snapshot` 或 `loopback-url` | 绑定（只读投影） | 读取/浏览 |
| `ssh-agent` | 是（远端） | `remote-workspace-agent`（与远端 Agent 共址） | 绑定（远端执行） | 完整（远端） |
| `cloud` / `sftp` | 否 | `read-only-projection`（`semantic-snapshot`/`loopback-url` 实现） | **绑定（只读投影）** | 读取/浏览；写操作显式拒绝（No fake parity） |
| `muse-collab` | 否 | 不 materialize 为目录 | **不绑定**（页面走 catalog/`muse_*` 工具） | 页面目录 |

流程（每个 Mount 一次，幂等）：

```text
读取 binding 文档（校验 protocol/版本/不变量）
  → 对每个 mount：解析 materialization
       host-path  : 取本机路径（内部握手/内部文件），校验存在、canonical、非 symlink 逃逸
       loopback   : 建立短期句柄（TTL），DSH 侧挂载只读投影目录
       projection : 生成只读投影目录（Q-1 决策：不可 materialize 的 Mount 也绑定，形式为只读投影）
                    $DSH_HOME/workspace-projections/<mountRef>/，写能力显式拒绝、UI 标注"只读投影"
  → 生成 materializationPath（确定性，§4.5）
  → workspaceRegistry.resolveByPath(path) 命中即复用，否则 create
  → 设置标题（去重规则）、按 order 排序、primary 置顶、pin（禁止 DSH UI 删除）
  → 写 receipt {bindingRevision, applied:[{mountRef, workspaceId, state}], errors[]}
```

### 4.5 映射表与归属（Host 为真源，DSH 侧确定性兜底）

```text
local / host-path        : DSH workspace path = canonicalization(mount root)
snapshot / loopback      : DSH workspace path = <DSH_HOME>/workspace-projections/<hash(mountRef)>
```

映射表位置（Q-2 决策：**放 Host 侧**）：`<Application Support>/OpenMuse/workspace-platform-v1/dsh-bindings/<workspaceRef>.json`，内容 `{workspaceRef, bindingRevision, mounts:[{mountRef, dshWorkspaceId, materializationPath, state}], updatedAt}`，数据来源是 DSH 写回的 receipt；随后每次发布的 binding 文档在每个 mount 上回传 `dshWorkspaceId` 作为**复用提示**，DSH 优先复用该 id，只有拿不到提示（首装、离线、清空）时才按上面的确定性规则推导。

两种落盘位置的取舍（回答"有何区别"）：

| 维度 | 放 Host 侧（选定） | 放 DSH_HOME |
|---|---|---|
| 与 definition 同源 | 是：随 Project Workspace 生命周期，可审计、可随定义参与团队同步 | 否：只存在于本机 DSH 目录 |
| 多端可用 | 是：Web/Mobile 的 DSH 实例读同一映射 | 否 |
| DSH 冷启动/离线 | 需先收到 binding 文档（文档内携带 id 提示） | 可离线自持 |
| Host 可见性与治理 | 完全可见：清理、诊断、迁移都在 Host | Host 不可见 |
| 备份/迁移 | 随 Host 备份 | 随 DSH_HOME，易丢 |

结论：Host 为真源；DSH 侧只把收到的 id 当缓存，并保留按 path 推导的兜底与可重建的诊断索引 `bindings/workspace-mount-index.json`（非真源）。

归属（Q-6）：该映射表同时记录每个 DSH workspace 的 `workspaceRef` 与 `mountRef`，panel 与审计据此按 Project 归组（"My Workspace · frontend"）。DSH 自身的 workspace 记录只有 `path/title/sessionIds`，不承载业务元数据。

标题规则（PRD §3.4）：`displayName` → 冲突时 `<ProjectWorkspace title> · <displayName>` → 仍冲突时追加 `-2/-3`。改 Mount 名只改标题，不移动 workspace（移动 path 会让 session 归属漂移）。

### 4.6 变更事件与幂等

| 事件 | 触发 | DSH 侧动作 |
|---|---|---|
| `binding.published`（文档 revision 变化） | Host 新增/移除/改名/只读变化/切换 active | 增量 diff：新增→注册；移除→标记 unbound（不删）；改名→rename；active→置顶 + 触发 panel 切换 |
| `binding.revoked` | Project Workspace 关闭/账号切换 | 保持记录，标记不可用 |
| `mount.degraded` | Provider offline/auth/trust | 标记状态，工具调用给可解释失败 |
| `receipt.written` | DSH 应用完成 | Host 侧可见（用于 WBD-07 上报） |

幂等：同一 `bindingRevision` 重复应用为 no-op；乱序 revision 丢弃；单次应用全程在 registry 操作串行队列内（复用 Controller 的 enqueue 模式）。

### 4.7 兼容适配器

| 旧通道 | 处理 |
|---|---|
| hint 文件 `current-appflowy-workspace.json`（Desktop） | **继续读**：作为 binding v0.5 解析（`projectRoot`→单 Mount 的 v1 文档），写 deprecation 日志；升级后 Host 改为写 binding 文档 |
| `workspace.bind`（Web/Mobile） | transport adapter 只做字段搬运，转换为同一 binding 文档；业务语义全部在插件层 |
| `path + cwd`（资源打开） | 在 session binding 内解析为 `resourceRef`；解析失败返回可读错误 + 审计（05 §7.3） |
| `workspace.catalog`（页面目录） | 保持现状（05 §3.3 的 `catalog/v2` 属 P2） |

## 5. 模块设计

### 5.1 Host 侧（Dart，`lib/workspace_platform` + `lib/plugins/dsh_agent`）

| 文件 | 现状 | 变更 |
|---|---|---|
| `application/workspace_controller.dart` | `open/mountLocalDirectory/unmount` + `_publishDshBinding()`；`selectedEntryRef` 仅内存 | 增加 `activeMountRef`（由 `selectedEntryRef` 推导并**持久化**到 device state）、`revision` 递增；selection 变化触发发布（节流 200 ms）；新增 `setActiveMount(mountRef)` 供 panel 切换器回调 |
| `plugins/dsh_agent/dsh_workspace_bridge.dart` | 写 hint（`projectRoot`、`mounts`，best-effort） | 演进为 `DshBindingPublisher`：生成 `muse.workspace/binding/v1`、原子写 + 清 `.tmp`、读 receipt、失败上报；保留 hint 兼容写开关（灰度期） |
| `infrastructure/local_workspace_provider.dart` | `muse.workspace.local.v1`：bind/stat/list/create/rename/import/delete/watch | 暴露 `materialize(mountRef) → {mode:"host-path", path, readOnly, generation}`，供 publisher 使用；写权限判定集中在此（INV-5） |
| `infrastructure/workspace_persistence.dart` | definitions + expanded state | 增加 `activeMountRef`；记录 `lastPublishedRevision` |
| `presentation/workspace_explorer.dart` | Mount/Entry 树、右键菜单 | Mount 行显示绑定/只读/离线状态；选中即更新 active Mount；新增 `Ask Agent about this file/folder`、`Start Agent in this folder`（对齐 05 §3.5） |
| `plugins/dsh_agent/dsh_agent_panel.dart` | `DshResourceOpenMessage` → `MuseResourceSurfaceOpener` | 优先用 `resourceRef`（新增字段），`path+cwd` 兜底；转发 panel 的 `setActiveMount` 意图 |
| `plugins/dsh_agent/dsh_embedded_view.dart` | WKWebView/WebView2 + JS channel | 保持通道名不变；新增消息类型 `muse.workspace.activate`（Host→panel）与 `resourceRef` 字段解析（含长度/字符校验） |

### 5.2 DSH 中间层插件（`middlewares/dsh/plugins/appflowy-workspace` → `workspace-binding`）

保留包名与现有导出以避免破坏 25 项 suite；新增/重构文件：

| 文件 | 内容 |
|---|---|
| `src/binding.ts`（新） | `MuseWorkspaceBindingV1` 类型、schema 校验（手写 + digest）、v0.5 hint 兼容解析、`bindingRevision` 幂等判定 |
| `src/identity.ts`（改） | 保留 `applyWorkspaceHint` 作为兼容入口；新增 `applyBinding(registry, binding)`：逐 Mount materialize → 注册/复用 → 标题去重 → order → primary 置顶 → pin |
| `src/materialize.ts`（新） | `host-path`（校验存在/canonical/非逃逸，读内部路径文件）、`snapshot`/`loopback`（P1）、不支持类型返回 `UNSUPPORTED` |
| `src/receipt.ts`（新） | 写 `$DSH_HOME/bindings/workspace-binding-receipt.json`（`bindingRevision`、`applied[]`、`errors[]`） |
| `src/prune.ts`（新） | 遗留清理：仅清理 (a) 路径落在旧 DSH home 的 `appflowy-workspaces/*`，(b) 非当前 binding 且 0 会话且路径不存在的记录；**绝不清理**当前 Mount 或他人目录；先写审计再删 |
| `src/http.ts`（新） | `GET /muse/v1/workspace/binding`（当前绑定 + 状态）、`GET /muse/v1/workspace/binding/events`（SSE，panel 内订阅） |
| `src/dsh.ts`（改） | `WORKSPACE_LIST_PROMPT` 分叉：绑定真实目录时说明"cwd 是挂载根，可直接 glob/ls/编辑；AppFlowy 页面仍用 `muse_workspace_list_views`"；scratch 目录时保留现有文案。`order` 不变（42） |
| `src/parent-bridge.ts`（改） | 只做 transport：`workspace.bind`/`context.contribute` 转 binding/contribution 结构；新增 `workspace.activate`（Host→DSH）与 `binding.receipt`（DSH→Host） |

### 5.3 panel 客户端插件（新 `@muse/dsh-client-ui-workspace-binding`）

职责（PBU-01/02）：显示当前绑定 chip、列出 Mount 并切换、切换时调用 DSH 原生导航。

| 关注点 | 设计 |
|---|---|
| 数据来源 | 订阅 `GET /muse/v1/workspace/binding/events`（同源、面板内已有 token 会话）；降级为轮询 `GET /muse/v1/workspace/binding` |
| 状态 | **实现偏差**：未声明 store（展示偏好未持久化），组件内局部 `useState` 保存快照/展开态；业务数据全部来自 HTTP 源 |
| 切换动作 | 切到目标 Mount 后取快照里该 Mount 的 `dshWorkspaceId` 调 `uiWorkspace.startSession(workspaceId)`（复用或新建该工作区的会话）；**不传 path** —— 面板与客户端插件之间只传统一资源标识（`mountRef`/`dshWorkspaceId`），路径只存在于 Host 与 sidecar 之间。若该 workspace 尚未出现在 `workspaces.list`（发布竞态），提示可读错误并等待下一次 SSE 快照，不猜测 id |
| 切换回传 | 调 `POST /muse/v1/workspace/active`（新端点，转 `setActiveMount`），保证 Host 与 DSH 双向一致 |
| 接入点（实现前审计） | 需按 `docs/subsystems/slots.md` 与 `packages/client/ui-slots` 的 `SlotMap` 选定 chip 与列表的 slot（候选：会话头部/composer 附带区）；hero 的 `conversation.hero.workspace` 与 `sidebar.workspaces` 归 `ui-workspace` 所有，不重复声明 |
| 构建与生效 | 客户端插件是 `dsh.client` 行 + `lib/client.js`；需加入 web-app bundle 的 `cordis.patch.yml` 与依赖，并经 sidecar 物化流程生效（§2.4） |

#### 5.3.1 已实现：面板数据面（P1-A，2026-09-20）

上表的数据源、切换回传与客户端 chip 插件均已落地（chip/切换器实测见 11 §7.6）。

| 端点 | 方法 | 语义 | 状态码 |
|---|---|---|---|
| `/muse/v1/workspace/binding` | GET | 已应用绑定快照：`protocol=muse.dsh/workspace-binding-panel/v1`、`bindingRevision`、`workspaceRef`、`title`、`activeMountRef`、`appliedAt`、`mounts[{mountRef,displayName,dshWorkspaceId,state,readOnly,mode,code?,message?}]`、`prunedWorkspaceIds`。**不含任何设备路径**（测试守护） | 200；未绑定 409 `NO_BINDING` |
| `/muse/v1/workspace/binding/events` | GET | SSE：连上即发 `event: snapshot`，随后转发 `binding.applied` / `binding.rejected` / `mount.unavailable` / `mount.activated`，15s 一次 `event: ping`；订阅上限 32 | 200 `text/event-stream` |
| `/muse/v1/workspace/active` | POST | body `{mountRef}`：重排 DSH workspace（目标置顶）、更新内存 active Mount、写 Host 意图文件 | 200 `{ok,activeMountRef,idempotent}`；400 `MOUNT_REF_REQUIRED`；404 `MOUNT_NOT_IN_BINDING`；409 `NO_BINDING`/`MOUNT_UNAVAILABLE`；503 `REGISTRY_UNAVAILABLE` |
| `/muse/v1/target.open` | POST | 打开生产端（RLO 通路 A，见 §5.4）：body `{resourceRef?, mountRef?, path?, cwd?, mode?, anchorHint?, disposition?, requestRef?}` + 可选 `sessionId` → 经 §5.4 解析后调用 `openResource.request(...)`，**恰好返回一个终态回执** `muse.presentation-intent-result/v2`（`status∈opened/focused/fallback/unsupported/rejected/timed-out/failed`，含 `reasonCode`） | 200（含终态回执）；400 `PATH_REQUIRED`/`RESOURCE_REF_INVALID`/`INVALID_JSON`；403 `MOUNT_READ_ONLY`/`OPEN_DENIED`/`NO_SESSION_SCOPE`；404 `MOUNT_NOT_IN_BINDING`；409 `NO_BINDING` |

入口归属（实测结论）：`@muse/dsh-appflowy/parent-bridge` 的 `apply()` 在 Desktop 直接 `return`（`parentBridgeEnabled()` 只看 `MUSE_DOCUMENT_CLOUD_URL`），所以**桌面面板**由 `@muse/dsh-appflowy/webview` 注册这三个路由（它本来就 `inject: [webServer]` 且每平台都加载，workspaceRegistry 走可选服务读取）；路由实现是共享模块 `panel-routes.ts`，Web/Mobile 若要暴露同一面板面也复用同一函数，避免两条实现漂移。

切换回传的真源处理：面板不能写 Host state，因此 `POST /active` 写 `$DSH_HOME/bindings/workspace-active-intent.json`（`muse.workspace/active-intent/v1`，含 `workspaceRef/mountRef/requestedAt`）。Host 下次发布 binding 时，若该意图属于同一 `workspaceRef`、`mountRef` 仍在发布集合内、且 `requestedAt > 映射表 state.updatedAt`，则采纳为 `activeMountRef` 并把 `requestedAt` 记为新的 `updatedAt`；否则忽略。这样 Host 仍是真源，但"用户刚在面板切过的 Mount"不会被下一次发布覆盖回去。

### 5.4 DSH → Host 资源打开（RLO，绑定侧增量）

归属：完整设计见 [../../agent-file-references-and-open-routing.md](../../agent-file-references-and-open-routing.md)（§3.3 `TargetRouter`、§3.4 执行通路、§4 统一资源协议、§9 多端）。本文只固定与本绑定相关的前置条件与解析规则，不重复设计承载器与引擎选择。

现状（实测）：打开消息**没有生产端**；Windows 面板没有 JS 通道；桌面没有 host→client 下行；`MuseLocalResourceRouter.resolve` 只看消息里的 `cwd`，从不查询 Host Mount（B9–B13）。因此"绑定接通"之后仍需三件事：

| # | 前置条件 | 落点 |
|---|---|---|
| P-1 | **生产端**：宿主 opener 覆盖（`internals.openPath`/`canOpenPath`，默认实现是 `@deepseek-ai/dsh-native-command`）→ 能力通道 op `target.open` → Flutter `TargetRouter` | 归属文档 §3.4 通路 A；OpenMuse 已有 host 半部 `@muse/plugin-dsh-client-ui-resource-open`（`ResourceOpenClient` → `PresentationRequestV2` → `ctx.openResource`）可作分流点 |
| P-2 | **通道**：桌面 panel 的 host→client 下发（不复活 parent-bridge HTTP，与 `target.open` 共用通道，见归属文档 §3.5）；Windows WebView2 补 JS 通道与 `webMessageReceived`（`dsh_embedded_view.dart:180-247`） | 本文 §5.1、§5.3 |
| P-3 | **回执**：`IntentReceipt{requestId,status,engineId,surfaceRef,materializeMode}`，替换 `unawaited` fire-and-forget（`dsh_agent_panel.dart:177`） | 本文 §5.1 |

绑定侧解析规则（本文新增，替换"仅 `sessionCwd` 容器校验"）：

```text
resolveWithinBinding(target, sessionBinding):
  1. target.resourceRef 命中某个 mountScope → 归属该 Mount（授权来自 mountScope.grants）
  2. target 为 legacy {path, cwd}：
       a. 在 sessionBinding 全部 Mount 根内做 containment 匹配，多根取最长前缀
       b. 命中 → 生成 resourceRef（Entry→Resource 映射由 Host Provider 提供）并审计 resolve.legacy
       c. 未命中 → OPEN_DENIED(MOUNT_NOT_IN_BINDING)，不回落到"任意绝对路径"
  3. 第二个 Mount 内的文件：允许打开（只读语义按 mount.requestedMode），Tab 标题带 Mount 显示名
```

即：`cwd` 从"页面自报的授权依据"降级为"兼容提示"，授权依据改为 session binding 的 `mountScopes`（INV-3）。

### 5.5 Host → DSH 引用与元信息（RCX）

该功能目前**零实现**（无消息类型、无生产端、无消费端，见 §2.2）。承载通道的选择是本节的关键决定：

| 候选通道 | 结论 |
|---|---|
| `muse.native-capability/v1`（现有 Flutter→DSH 通道） | **不可用**：`forbiddenKeys` 明确禁止 `path/contentUri/base64/intent`（`dsh_native_capability_codec.dart:32-40`），且它面向能力调用而非上下文 |
| `context.contribute` + `MuseContextContributionV1` | **采用**：Dart 侧已有模型（`packages/muse_plugin_facets/lib/src/models.dart:36-107`，含 `contextType/contextSchemaDigest/payload/…`）与生产点（`lib/plugins/dsh_agent/dsh_host_plugin_plane.dart:15-111`）、运输（`packages/muse_remote_session/lib/src/parent_bridge_adapter.dart:101-107`）、DSH 侧消费（`parent-bridge.ts:369-392` → `museContextBroker.ingestContribution`）。只需新增一个 `contextType` 及其 payload schema，不新增通道 |
| 新造 envelope | 不采用：与 05 §3.5 的五个 Explorer 动作、03 "不新增 wire 动词"冲突 |

payload（字段集对齐 `ResourceDescriptorV1` 与 `present` 工具 schema；只带 `resourceRef`，不带 path）：

```ts
interface MuseResourceReferencePayloadV1 {
  protocol: "muse.context/resource-reference/v1"
  contextType: "resource.reference"
  resourceRef: ResourceRef            // opaque；解释权在 Host
  mountRef?: OpaqueRef
  entryRef?: OpaqueRef
  displayName: string                 // "src/main.dart"
  anchor?: { kind: "line" | "range" | "section" | "page"; line?: number; start?: number; end?: number; sectionId?: string }
  revision?: string
  capabilities: WorkspaceCapability[]
  source: "host.selection" | "host.explorer" | "host.tab" | "host.mention"
}
```

DSH 侧：

1. `museContextBroker.ingestContribution` 校验后写入会话；因"模型可见 ⟺ 可记录"（INV-6），在 `@muse/...` 插件内用 `SessionEventMap` declaration merging 注册 `muse/resource-reference` 事件，`ignorable: true`（仅结构性格式变更才 bump `SESSION_FORMAT_VERSION`）。
2. composer 显示引用 chip：由 §5.3 的客户端插件通过新 slot 渲染；数据来自会话事件/contribution 快照；删除 chip 只影响本地草稿，不撤回已发送消息。
3. 发送时以结构化块呈现 `displayName + anchor + resourceRef + capabilities`，**不注入全文**（RCX-03）；模型需要内容时按 ref 调用 `muse.resource` 系列工具（`describe/read/materialize`，03 §4 合同族）。
4. 内联小片段（Q-4 决策：**允许**）：`excerpt` 默认开启，上限 8 KiB，由 Host 侧截断并附 `truncated` 标记；超限只发 descriptor，并提示模型用 `muse.resource` 工具读取全文。片段按"资源内容"对待（不进审计日志、不写 hint、不落 device state）。
5. 与 `present` 的关系：`resourceRef` 已是 `present` 的既有载荷（`presentResourceInputSchema` 只接受 `resourceRef|mode|placement|anchorHint` 且拒绝 path），因此"引用 → 模型 → 再 present"的闭环不需要新协议。

## 6. 关键时序

```text
6.1 启动/首次绑定
Host: open(workspaceRef) → 解析 definition + 设备绑定 → PUBLISH(binding v1)
DSH : BindingReader 校验 → 逐 Mount materialize → registry create/reuse → 标题/顺序/primary/pin
      → receipt 落盘 → 事件推送 → panel chip 显示 "My Workspace · frontend"

6.2 新增 Mount（不重启）
Host: mountLocalDirectory() → revision++ → PUBLISH
DSH : diff 出新 mount → materialize → create workspace → 保持现有 session 不动 → chip 列表新增 "helix"

6.3 切换 active Mount
Host: 用户点选 helix → setActiveMount(mountRef) → 持久化 → PUBLISH(activeMountRef=helix, revision++)
DSH : 置顶 helix；client 插件收到事件 → uiWorkspace.startSession(helixWorkspaceId)
      → panel 打开 helix 的空白会话（原 frontend 会话保留在历史）

6.4 DSH 链接 → Host 打开
agent 回答含 resourceRef 链接 → 用户点击 → client 发 intent.dispatch(surface.open{resourceRef,anchor})
Host: authority check → binding 内校验 → Surface Orchestrator 选引擎 → 打开 Tab → intent.receipt
DSH : receipt 落到会话（失败可见）

6.5 Host 引用 → DSH 输入框
Host: 用户选中文本 → 构造 contribution（resourceRef+anchor+displayName）→ context.contribute
DSH : broker 校验 → session event + chip → 用户补充输入 → 发送（结构化引用块随消息）
```

## 7. 数据与持久化

| 数据 | 位置 | 同步 | 生命周期 |
|---|---|---|---|
| Project Workspace definition（含 mounts/order/activeMountRef） | Host `workspace-platform-v1/definitions/<workspaceRef>.json` | 可选团队同步 | 随工作区 |
| 设备绑定（bindingKey→locator） | Host 本机 DB | 否 | 随设备 |
| binding 文档（v1） | `$DSH_HOME/bindings/workspace-binding.json`（原子写） | 否 | 每次发布覆盖，保留上一版 |
| materialized 路径（唯一允许写路径处） | `$DSH_HOME/bindings/materialized/<mountRef>.path`（0600） | 否 | 随 binding |
| DSH workspace 映射表（`mountRef ↔ dshWorkspaceId`） | Host `<Application Support>/OpenMuse/workspace-platform-v1/dsh-bindings/<workspaceRef>.json` | 否 | 随 Project Workspace（§4.5） |
| 只读投影目录（不可 materialize 的 Mount） | `$DSH_HOME/workspace-projections/<mountRef>/` | 否 | 随 binding，Host 可重建 |
| receipt | `$DSH_HOME/bindings/workspace-binding-receipt.json` | 否 | 每次应用覆盖 |
| DSH workspace 记录 | `$DSH_HOME/storages/workspace.json`（既有 unit v2） | 否 | 由 DSH 管理与 pin |
| 引用与打开的会话事实 | DSH session log（`muse/resource-reference` 事件） | 随会话 | 随会话 |
| 展示偏好（chip 展开等） | 客户端 store | 否 | 本机 |

迁移：旧 hint 保留可读（v0.5）；升级首启时按 v1 发布并记录 receipt；遗留 workspace 只按 `prune.ts` 规则处理（PRD Q-5 默认为"不再默认选中 + 可选一次性清理"）。

## 8. 权限、安全与审计

- 授权真源 = `DshWorkspaceBindingV1`；工具调用时按目标 Mount 对应 Provider 重验，权限决定不缓存。
- 只读 Mount：Provider 拒绝 + policy 拒绝，UI 只做提示（INV-5）。
- 新增 Mount 单独评估信任；`restricted` 工作区不得静默并入未信任根（05 §7.1）。
- 禁止进入模型上下文/会话日志/审计日志：绝对路径、正文全文（除显式小 `excerpt`）、credential、短期 bearer handle。
- 审计事件：`workspace.bind.applied`、`workspace.bind.failed`、`workspace.active.changed`、`resource.open.requested/allowed/denied`、`resource.reference.attached`，字段含 `workspaceRef/mountRef/providerId/requestRef/sessionRef/operation/effect/resultCode/generation/latencyMs`。
- 失败关闭：binding 文档非法、mountRef 越界、path 逃逸、过期 binding → 拒绝并给可读原因（不降级为"静默跳过"）。

## 9. 测试与验收设计

### 9.1 单元与契约

| 层 | 测试 | 覆盖 |
|---|---|---|
| DSH 插件（TS） | `tests/binding.test.ts`（新） | schema 校验、v0.5 hint 兼容、revision 幂等/乱序、非法文档整体拒绝、标题去重、order/primary 置顶 |
| DSH 插件（TS） | `tests/identity.test.ts`（改） | 多 Mount 注册/复用（fake registry 模式已有）、pin、只读授予、UNSUPPORTED 分支 |
| DSH 插件（TS） | `tests/prune.test.ts`（新） | 只清理符合规则的记录；当前 Mount/有会话/他人目录必须保留 |
| DSH 插件（TS） | `tests/prompt.test.ts`（新） | 绑定真实目录与 scratch 目录两种文案；不出现路径 |
| 客户端插件（TS） | 组件/逻辑测试 | chip 状态、切换调用 `startSession(workspaceId)`、事件订阅失败降级轮询 |
| Host（Dart） | `test/workspace_platform/*`（扩展既有 E2E 第 5 步） | binding 文档 golden（mounts/activeMountRef/revision）、selection 持久化、切换回调 |
| 契约 fixtures | 共享 golden JSON | TS/Dart 双向校验同一 fixture（03 §11 要求） |

### 9.2 端到端（本地可复现）

1. 挂 2 个目录 → 断言 `workspace-binding.json` + `storages/workspace.json` 出现两条对应记录 + panel chip 正确。
2. 切换 active Mount → 断言置顶、`uiWorkspace.startSession` 生效（新会话 `cwd` = 第二个 Mount 根，可查 `session_projcache`）。
3. 移除 Mount → 记录保留且标记 unbound；重新挂载同目录 → 复用同一 workspace（不新建）。
4. DSH 点击第二个 Mount 的文件链接 → Host 打开正确 Tab 与行号。
5. Host 选中文本/文件 → DSH 输入框 chip + 发送后模型可读到结构化引用。
6. Web 端同一脚本（虚拟 Provider）→ 绑定行为一致、能力显式降级。

### 9.3 失败注入

相对路径、Mount 根不存在、重复 mountRef、同一目录挂两次、只读写尝试、DSH UI 尝试删除已 pin workspace、sidecar 重启期间发布、旧 revision 乱序、`.tmp` 残留、旧 DSH home 遗留记录、symlink 逃逸、超长 displayName。

### 9.4 回归保护

- 08 §4 的 DSH Contract 四项（相对 root 被拒、绝对 root 成为 cwd、不覆盖 `main.ts`、不注入 README）必须继续通过。
- `WORKSPACE_LIST_PROMPT` 的 scratch 分支文案保持（有 snapshot 时同步更新）。
- 绑定文档中"允许出现路径的唯一位置"必须有测试断言其**未**出现在文档与模型可见文本中（INV-1）。

## 10. 分期实施

| 阶段 | 内容 | 交付与验收 |
|---|---|---|
| P0（绑定收敛） | `binding.ts`/`materialize.ts`/`receipt.ts`/`prune.ts` + `identity.ts` 多 Mount + Host `DshBindingPublisher` + Host 侧映射表 + activeMountRef 持久化 + prompt 分叉 + **升级时一次性遗留清理**（Q-5）+ 测试迁入 25 项 suite | WBD-01..07 全绿；SC-1..4、SC-8/9 通过；F5"scope 差异为 0"有记录；旧 hint 兼容；清理有审计与回滚演练 |
| P1（联动） | 绑定侧：per-session `DshWorkspaceBindingV1` 与 `mountScopes` 解析（§5.4 规则）+ 打开回执 + Windows panel 通道/桌面下发（P-2/P-3）；引用侧：`context.contribute` 的 `resource.reference` payload + `muse/resource-reference` 会话事件 + composer chip；panel：`/muse/v1/workspace/binding[/events]` + chip/切换器 | 本文负责的绑定侧条目全绿；RLO/RCX/PBU-01/02 按 PRD 验收；`target.open` 生产端按归属文档 §3.4 通路 A 落地 |
| P2（多端与收尾） | Web/Mobile transport 收敛、`catalog/v2` 切换、只读投影 materialization（Q-1）、灰度与回滚演练、PBU-03/04、RCX-05/06 | MPT-01..03 通过；SC-10 通过 |

每阶段的 PR 切分：插件协议与测试 → Host publisher → panel 客户端 → 文档与验收记录（每步可独立回滚，灰度开关保留 hint 写入）。

## 11. 风险、回滚与决策记录

| # | 风险 | 缓解 |
|---|---|---|
| R-1 | 多注册 DSH workspace 让老会话"看起来丢了" | 不移动 path、只新增/标记；迁移说明 + panel 提示 |
| R-2 | session log 新事件破坏旧构建 | `ignorable: true`；仅结构性变更才 bump `SESSION_FORMAT_VERSION` |
| R-3 | 插件变更未生效（profile 物化） | 文档明确 sidecar 物化流程；验收步骤包含"重启后生效"检查 |
| R-4 | 绑定文档/会话中出现绝对路径 | INV-1 测试 + 审计日志字段白名单 |
| R-5 | 清理遗留 workspace 误删用户数据 | `prune.ts` 三重条件（旧 home / 0 会话 / 路径不存在）+ 审计 + 单测 |
| R-6 | 与 Web `workspace.bind` 语义漂移 | 同一 schema、同一 fixture、同一验收脚本 |
| R-7 | 打开链路的生产端与 Windows 通道都不存在（B9/B10），只做绑定会让"接通"仍不可用 | P1 把 P-1/P-2/P-3 列为同阶段交付；验收必须含"Windows 上点第二个 Mount 的文件链接" |
| R-8 | 与 `agent-file-references-and-open-routing.md` 设计重复/冲突 | 本文只做绑定侧增量；契约以 03 的不透明 `ResourceRef` 为准（该文档 §4.2 的 `{scheme,id}` 表述已被 03 §2.1 取代），落地前对两处表述做一次收敛 |
| R-9 | 统一资源协议文档之间存在表述差异（`ResourceRef` 不透明字符串 vs `{scheme,id}`；capability 23 项 vs 实现 11 项） | 在 F1 的 ADR 中一次性收敛，本文按 03 与 04 为准 |

决策记录（2026-09-20 确认）：

| # | 决策 | 落点 |
|---|---|---|
| Q-1 | 不可 materialize 的 Mount（纯 Cloud 等）**绑定只读投影**（不采用"不绑定"） | §4.4、§7、§10 P2 |
| Q-2 | `mountRef ↔ dshWorkspaceId` 映射表放 **Host 侧**，DSH 侧只做缓存与兜底 | §4.5、§7 |
| Q-3 | Host 删除 Project Workspace 时，DSH 侧 workspace 与会话**保留**（只标记解绑） | §4.6、§5.2 `prune.ts` |
| Q-4 | 引用**允许小文本内联**（`excerpt` ≤ 8 KiB，Host 侧截断） | §5.5 |
| Q-5 | 遗留 workspace 在升级时**一次性清理**（白名单 + 审计 + 可回滚） | §5.2 `prune.ts`、§10 P0 |
| Q-6 | 映射基数采用**每 Mount ↔ 一个 DSH Workspace**；Project Workspace 负责归属与分组（不采用 Project 级 1:1：那需要投影根，会让 agent cwd 与真实目录脱钩并推翻 08 §2.6 的 cwd 验收） | §4.2、§4.5、§5.3 |

无待确认项。
