# Workspace 领域模型与 Provider 合同

## 1. 聚合边界

### 1.1 AccountSpace

复用 `UserWorkspacePB` 的身份与成员体系，建议产品文案改为“空间/团队空间”。

```ts
interface AccountSpace {
  accountSpaceRef: OpaqueRef
  title: string
  membership: { actorRef: OpaqueRef; role: string }[]
  policyProfileRef: OpaqueRef
  collaborationEndpointRef?: OpaqueRef
}
```

### 1.2 ProjectWorkspace

```ts
interface ProjectWorkspaceV1 {
  protocol: "muse.workspace/definition/v1"
  workspaceRef: OpaqueRef
  accountSpaceRef: OpaqueRef
  title: string
  mounts: WorkspaceMountV1[]
  settings: {
    trust: "restricted" | "trusted"
    defaultOpenMode: "preview" | "pinned"
    followActiveFile: boolean
    ignoreProfileRefs: OpaqueRef[]
  }
  revision: RevisionToken
}
```

定义中不允许出现 secret、bearer URL、SSH private key 或未经抽象的设备绝对路径。共享定义中的本地 Mount 使用 `bindingKey`，由每台设备解析实际 locator。

### 1.3 WorkspaceMount

```ts
interface WorkspaceMountV1 {
  mountRef: OpaqueRef
  providerId: string
  bindingKey: string
  displayName: string
  rootResourceRef: ResourceRef
  requestedMode: "read-only" | "read-write"
  order: number
  visibility: "private" | "account-space"
  settings: Record<string, JsonValue> // provider schema 校验，禁止 secret
}
```

一个 Provider 可以有多个 Mount；同一物理目录可以在不同 Workspace 中以不同 policy 挂载。`mountRef` 是工作区关系身份，不等于资源身份。

### 1.4 Entry 与 Resource

```ts
interface WorkspaceEntryV1 {
  entryRef: OpaqueRef
  mountRef: OpaqueRef
  resourceRef: ResourceRef
  parentEntryRef?: OpaqueRef
  name: string
  kind: "directory" | "file" | "symlink" | "virtual" | "unknown"
  formatHint?: string
  mimeHint?: string
  size?: number
  revision?: RevisionToken
  modifiedAt?: number
  capabilities: WorkspaceCapability[]
  decorations?: EntryDecoration[]
  childState?: "unloaded" | "loading" | "partial" | "complete" | "error"
  extensionData?: Record<string, JsonValue>
}
```

`entryRef` 表示 Provider 树中的可导航位置；`resourceRef` 表示内容身份。同一资源可通过 alias/symlink 出现在多个 Entry。菜单和选择使用 entryRef，打开与编辑使用 resourceRef。

## 2. Provider SPI

Provider 通过 Host Registry 注册，不由 Flutter 直接持有实现对象。

```ts
interface WorkspaceProviderV1 {
  descriptor(): WorkspaceProviderDescriptorV1
  bind(request: BindMountRequestV1): Promise<MountLeaseV1>
  unbind(leaseRef: OpaqueRef): Promise<void>

  stat(request: StatEntryRequestV1): Promise<WorkspaceEntryV1>
  list(request: ListChildrenRequestV1): Promise<ListChildrenPageV1>
  resolve(request: ResolveLocatorRequestV1): Promise<WorkspaceEntryV1>

  create(request: CreateEntryRequestV1): Promise<MutationReceiptV1>
  rename(request: RenameEntryRequestV1): Promise<MutationReceiptV1>
  move(request: MoveEntryRequestV1): Promise<MutationReceiptV1>
  copy(request: CopyEntryRequestV1): Promise<MutationReceiptV1>
  delete(request: DeleteEntryRequestV1): Promise<MutationReceiptV1>

  read(request: ReadResourceRequestV1): Promise<DataHandleV1>
  write(request: WriteResourceRequestV1): Promise<MutationReceiptV1>
  search?(request: SearchWorkspaceRequestV1): AsyncIterable<SearchPageV1>
  watch?(request: WatchWorkspaceRequestV1): AsyncIterable<WorkspaceChangeV1>
}
```

每个 operation 都必须：

- 带 `requestRef/deadline/cancellation`；
- 在 Host 侧重新验证 actor、workspace、mount 和 policy；
- 对 mutation 声明 effect 与幂等键；
- 返回 terminal receipt，不以 UI toast 代替结果；
- 不把 Provider exception、credential 或真实 locator直接传给 DSH/UI。

## 3. Capability 模型

基础 capability：

```text
metadata.read, children.list, content.read, content.range
entry.create-file, entry.create-directory, entry.rename
entry.move, entry.copy, entry.delete, content.write
changes.watch, search.name, search.content
execution.shell, execution.task, language.lsp
share.link, collaboration.realtime, history.read
native.reveal, transfer.upload, transfer.download
```

Capability 是 Provider、Mount policy、actor permission、workspace trust 和当前连接状态的交集：

```text
effective = provider ∩ mountMode ∩ actorGrant ∩ trust ∩ health ∩ device
```

UI 不根据 scheme 猜能力。例如 SSH Provider 断线时可能只保留 cached metadata/read；Cloud Provider 可能支持 rename 但不支持 atomic move。

## 4. 核心请求

### 4.1 分页列目录

```json
{
  "protocol": "muse.workspace/list-children/v1",
  "requestRef": "wreq.01...",
  "workspaceRef": "workspace.opaque",
  "mountRef": "mount.opaque",
  "parentEntryRef": "entry.opaque",
  "cursor": null,
  "limit": 200,
  "sort": [{ "field": "kind", "direction": "asc" }, { "field": "name", "direction": "asc" }],
  "generation": 7
}
```

响应必须回显 generation。用户折叠、刷新或切换 Workspace 后 generation 增加，旧响应不得写入当前树。

### 4.2 写入

```json
{
  "protocol": "muse.resource/write/v1",
  "requestRef": "wreq.02...",
  "resourceRef": "resource.opaque",
  "expectedRevision": "sha256:...",
  "contentHandleRef": "handle.short-lived",
  "intent": "save",
  "idempotencyKey": "example-operation-id"
}
```

如果 Provider 没有 revision/etag，descriptor 必须声明 `writeConsistency: unsafe`；Host 在每次写入前要求显式确认，不允许插件隐藏风险。

### 4.3 变化事件

```ts
interface WorkspaceChangeV1 {
  protocol: "muse.workspace/change/v1"
  eventRef: OpaqueRef
  mountRef: OpaqueRef
  sequence: number
  kind: "created" | "modified" | "renamed" | "moved" | "deleted" | "overflow" | "resync"
  entryRef?: OpaqueRef
  resourceRef?: ResourceRef
  previousEntryRef?: OpaqueRef
  revision?: RevisionToken
  observedAt: number
  source: "external" | "host" | "agent" | "collaboration"
  // Ontology 预留，不在本轮消费
  semanticHints?: { objectRefs?: OpaqueRef[]; changeKind?: string }
}
```

watch overflow 必须产生 `overflow`，由 Tree Store 对受影响目录做有界 resync，不能静默丢事件。

## 5. 状态机

### 5.1 Workspace

```text
defined → opening → ready ↔ suspended
              ├→ degraded
              ├→ blocked(auth/trust)
              └→ failed
ready/degraded/blocked → closing → closed
```

Workspace ready 不要求所有 Mount ready；至少一个可用且 definition 已加载即可。整体状态展示最严重且可操作的 Mount 问题。

### 5.2 Mount

```text
unbound → binding → connecting → ready
                    ├→ auth-required
                    ├→ trust-required
                    ├→ degraded
                    └→ offline
ready ↔ reconnecting/offline
* → unbinding → unbound
```

每次 bind 产生 `mountLeaseRef + generation`。旧连接的事件不得更新新 generation。

### 5.3 Tree node

```text
unloaded → loading → partial → loading-next → complete
              └→ error → retry
expanded ↔ collapsed
```

collapsed 只是视图状态；缓存是否保留由预算和 LRU 决定。删除/rename 的开放 Tab 通过 resourceRef 保持连续，不以树节点 Widget 生命周期管理 Engine Session。

## 6. Provider 类型

### LocalProvider

- root binding 保存于设备 Keychain/secure preferences；
- canonicalization、symlink 和 containment 在 Rust 完成；
- 原子写使用同目录 temp + fsync + rename；
- watcher 归一化成公共事件；
- `native.reveal/execution.shell/language.lsp` 可用取决于 trust。

### SshAgentProvider

- SSH 只负责认证和隧道；远端 Agent 实现同一 Provider/DSH FS 合同；
- list/read/watch/search/exec 在远端执行；
- Data Handle 使用带 audience、TTL、range 的加密通道；
- Host key 变化阻断连接，不自动接受。

### SftpProvider

- 用于不能部署 Remote Agent 的环境；
- 支持 metadata/list/range/read/write/rename（以服务端能力为准）；
- 默认没有 watch、search.content、exec、LSP；
- 不把远端文件伪装成本地路径交给 Helix，需 materialize working copy。

### CloudProvider

- 使用 Provider 原生 item ID、revision/etag 和 delta token；
- cursor 原样由 Provider 私有封装成 opaque token；
- OAuth token 只在 credential vault；
- 浏览器/Viewer 用短期 range handle，不直接得到长期下载 URL。

### AppFlowyCollabProvider

- `ViewPB`、Folder Collab、Document/Database 合同留在 Provider 内部；
- Public/Private/Space 投影为虚拟目录；
- share/lock/history/realtime 能力继续可用；
- 页面创建仍调用现有 Folder operation handler；
- 不向其他 Provider 强加 ViewLayout。

## 7. 持久化分层

| 数据 | 存储 | 是否同步 |
|---|---|---|
| Account Space、成员、角色 | 现有 user/cloud | 是 |
| Project Workspace definition | 新 Workspace Collab/metadata | 可选团队同步 |
| portable definition | `.muse/workspace.json` | 由 Git/Provider 决定 |
| credential/binding | OS Keychain + 本机 DB | 否 |
| 展开、选择、滚动、窗口布局 | 本机 device state | 否 |
| 最近 Workspace/Tab session | 本机 DB，可选加密同步 | 默认否 |
| 文件树 metadata cache | 本机 SQLite/LRU | 否 |
| content cache | bounded CAS | 否；可清理 |
| AppFlowy Page 内容 | 现有 Collab | 是 |

