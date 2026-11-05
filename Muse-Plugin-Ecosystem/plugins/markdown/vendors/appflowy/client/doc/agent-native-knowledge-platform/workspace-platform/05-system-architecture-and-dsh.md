# 系统架构、Host 与 DSH 协同

## 1. 目标架构

```text
┌──────────────────────── Flutter Workbench ────────────────────────┐
│ Account Space Switcher | Workspace Explorer | Tabs | DSH Panel    │
│        Command/Menu Registry | Context Keys | Decorations          │
└──────────────────────────────┬──────────────────────────────────────┘
                               │ typed intents / state
┌──────────────────────── Rust Host Runtime ─────────────────────────┐
│ Workspace Manager                                                  │
│   Definition Store · Mount Manager · Tree Cache · Watch Merger      │
│ Resource Authority                                                  │
│   ResourceRef · policy · revision · data handles · commit           │
│ Provider Registry                                                   │
│   Local · AppFlowy Collab · SSH Agent · SFTP · Cloud                │
│ Surface Orchestrator                                                │
│   iOffice · Helix · Open File Viewer · future adapters              │
│ Host Bridge / Audit / Credential Vault / Trust                      │
└──────────────┬───────────────────────────────┬──────────────────────┘
               │ local IPC/FFI                 │ encrypted transport
┌──────────────▼──────────────┐  ┌─────────────▼─────────────────────┐
│ Local DSH Runtime           │  │ Remote Workspace Agent / Gateway  │
│ session · tools · sandbox   │  │ provider · DSH · shell · LSP      │
│ resource presentation      │  │ watcher · search · materializer   │
└─────────────────────────────┘  └───────────────────────────────────┘
```

## 2. Host 组件

### WorkspaceManager

职责：

- create/open/close/reopen Project Workspace；
- 加载 definition，解析设备 binding；
- 为每个 Mount 创建 lease/generation；
- 聚合 mount health、trust 和 capability；
- 保存 session state，但不拥有文件内容。

### WorkspaceTreeStore

职责：

- 按 `entryRef` 保存 metadata 和 child page；
- 支持 placeholder/loading/partial/error；
- 合并 watch event，维护 generation；
- 执行 LRU 和 node budget；
- 为 Flutter 输出 immutable delta，而不是每次传完整树。

建议默认预算：50,000 metadata nodes、200 MB metadata/cache；超预算优先释放折叠且未选中的 subtree。

### WorkspaceProviderRegistry

职责：

- 按 provider descriptor 和 runtime 选择 Provider；
- bind/probe/health/operation dispatch；
- provider generation 与 disposer；
- capability 和 schema 校验；
- circuit breaker、deadline 和 telemetry。

Provider 不是格式引擎。Provider 回答“资源在哪里、如何安全读取/写入”；Engine Adapter 回答“如何查看/编辑该格式”。

### CommandRegistry / MenuRegistry

Host/Flutter 公共服务，消费插件贡献；详见插件设计。命令 handler 可以位于 Flutter、Rust Host、DSH 或远端 Agent，但调用必须经过统一 effect/policy gate。

## 3. DSH Workspace Binding

### 3.1 绑定对象

每个 DSH session 持久化：

```ts
interface DshWorkspaceBindingV1 {
  protocol: "muse.dsh/workspace-binding/v1"
  bindingRef: OpaqueRef
  sessionRef: OpaqueRef
  workspaceRef: OpaqueRef
  mountScopes: Array<{
    mountRef: OpaqueRef
    rootResourceRef: ResourceRef
    grants: WorkspaceCapability[]
  }>
  primaryMountRef?: OpaqueRef
  createdAt: number
  policyRevision: RevisionToken
}
```

`cwd` 是某个 execution domain 的兼容投影，不是权限真源。工具调用时由 binding + session policy 解析目标 Provider。

### 3.2 接入现有 DSH 能力

现有 DSH 已有抽象 `FileSystem`、`WorkspaceFiles`、sandbox policy、LSP 和 terminal。迁移方式：

1. 为 `FileSystem` 增加/实现 Muse Provider Broker adapter；
2. `FsTarget.targetKey` 映射为 Host 私有 entry/resource identity；
3. Local DSH 使用 Local Provider adapter；
4. Remote DSH 与 Remote Workspace Agent 共址，使用远端 Provider；
5. Cloud Mount 没有 exec 时，仅暴露 read/list/range 等可用工具；
6. `WorkspaceFiles` 的 list/read/watch 继续作为 DSH UI 数据面，但 scope 来自 Workspace Binding；
7. 所有 produced files 通过 resourceRef 发布，旧 path 字段仅兼容显示。

### 3.3 上下文投影

替换当前只投影 AppFlowy View 的 `workspace.catalog`：

```ts
interface WorkspaceCatalogV2 {
  protocol: "muse.workspace/catalog/v2"
  workspaceRef: OpaqueRef
  mounts: Array<{
    mountRef: OpaqueRef
    title: string
    providerKind: string
    state: string
    capabilities: string[]
  }>
  selection?: { entryRefs: OpaqueRef[] }
  openResources?: { resourceRef: ResourceRef; dirty: boolean }[]
  revision: RevisionToken
}
```

Catalog 只提供控制面。文件树通过分页工具查询；文件正文通过 resource snapshot/range 工具读取。不得把 100k 文件名或完整文件内容持续注入 prompt。

### 3.4 DSH → Host 打开

沿用已实现的 Resource Presentation seam：

```text
DSH deliverable/mention/card
  → ResourceOpenClient(resourceRef, anchor, requestedMode)
  → Host Bridge authority check
  → WorkspaceManager validates resource belongs to active grant
  → Surface Orchestrator selects iOffice/Helix/Viewer
  → Host native file Tab
  → terminal receipt persisted in DSH session
```

### 3.5 Host → DSH

Explorer 提供标准动作：

- `Ask Agent about this file/folder`：发送 resourceRef/entryRef，不发送正文；
- `Start Agent in this folder`：创建 scoped binding，folder 成为 primary root；
- `Add to current Agent context`：创建 TTL context lease；
- `Show Agent changes`：按 change receipt 聚合；
- `Reveal active task files`：基于 session durable events 高亮树节点。

## 4. 执行位置

```text
Command/Tool request
  ├─ UI-only / theme / reveal → local Flutter
  ├─ resource metadata/read → owning Provider runtime
  ├─ format render → selected Engine runtime
  ├─ shell/LSP/build/test → Mount execution domain
  └─ cloud-native action → Cloud Provider/Gateway
```

插件 descriptor 声明 `runtimePreference: ui | host | workspace | either`。Workspace runtime 插件需要文件和执行环境；UI 插件不能直接访问远端 locator。该模型对应主流编辑器的 UI/Workspace extension 分离，但使用现有 Plugin Graph v2 实现。

## 5. 本地、SSH 与云端数据流

### 5.1 Local

```text
Flutter expand
 → Rust WorkspaceManager
 → LocalProvider.list
 → Tree delta

DSH edit
 → DSH FileSystem adapter
 → Host/Local Provider policy
 → atomic mutation + revision
 → watch event
 → Explorer/Tab/DSH change feed
```

### 5.2 SSH Agent

```text
Local Flutter/Host
 → SSH tunnel
 → Remote Workspace Agent
    Provider + DSH + shell + LSP + search
 → scoped metadata/data handles
 → Local Surface Engine when appropriate
```

Helix 有两种部署：

- PTY Helix 在远端运行并把终端帧传给 Host；适用于代码编辑和远端工具链。
- 本地 Engine 编辑 remote working copy，保存时 Provider commit；作为 SFTP/无远端二进制降级。

### 5.3 Cloud

```text
Host → CloudProvider API → metadata/range handle
                  └──────→ optional Cloud execution service
```

没有 execution capability 时，DSH 不提供 shell/LSP，只提供 Provider 原生查询与安全 materialization。

## 6. 协作策略

协作分两类：

1. **Workspace definition collaboration**：团队共享 Mount 的逻辑定义、显示顺序、工作区策略和推荐插件。
2. **Content collaboration**：由资源自身 Provider 决定。AppFlowy Page 支持实时 Collab；Git 文件通过 Git；Cloud 文档用其版本/协作 API；普通本地文件默认不自动上传。

Presence 可以显示“某成员正在查看 resourceRef”，但若对方没有该 Mount 的权限，只显示安全名称，不泄露 locator 或内容。

## 7. 安全模型

### 7.1 信任与权限分层

```text
Identity permission
  ∩ Account Space policy
  ∩ Workspace trust
  ∩ Mount grant
  ∩ Provider capability
  ∩ Command effect approval
```

- Restricted：允许 metadata、预览、显式打开；禁用自动任务、workspace 插件、Agent 写入。
- Trusted：按角色和 Provider 能力开放执行；仍不等于 danger-full-access。
- 新增 Mount 单独评估信任；不能把未信任根静默加入已信任多根 Workspace。

### 7.2 Credential

- 保存于 OS Keychain/enterprise vault；
- 插件使用 credentialRef 请求受限操作，不能读取 secret；
- SSH Agent/Cloud token 按 audience、mount、operation、TTL 签发；
- 日志只记录 providerId、requestRef、effect、result code。

### 7.3 路径与链接

- Local canonicalization 和 containment 必须在 Host Rust 侧；
- symlink 默认显示但跨根跟随需 policy；
- SSH/Cloud 的 locator 由 Provider 解析；核心不拼接字符串路径；
- DSH supplied path 必须先在 session binding 内解析成 resourceRef。

## 8. 可观测性

每个打开、展开、搜索、写入、重连产生关联字段：

```text
workspaceRef, mountRef, providerId, requestRef, sessionRef?
operation, effect, generation, latencyMs, bytes, cacheHit
resultCode, retryCount, fallback, policyDecisionRef
```

禁止记录：绝对用户路径、文件正文、query secret、credential、短期 bearer handle。

关键看板：Provider availability、list P50/P95、watch overflow、cache hit、reconnect、DSH scope denial、write conflict、engine route result。

