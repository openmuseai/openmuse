# 统一资源协议：任意格式的一等公民

## 1. 目标

统一资源协议不是“支持更多文件扩展名”的工具类，而是 Muse 内部所有内容与应用能力的共同入口。它必须让以下对象走相同的生命周期：

- 工作区中的 Markdown、代码和二进制文件；
- AppFlowy Markdown、Word、表格等云/本地协作文档；
- Office blob、附件、远程 URL、数据集；
- 应用视图、Workflow 定义、Agent 产物；
- 后续 CAD 零件、三维场景、媒体、GIS 等资源。

共同生命周期为：

```text
发现 → 签发引用 → 描述 → 授权 → 选择能力 → 物化 → 承载/调用
     → 提议修改 → 审批 → 提交 → 事件 → 订阅者更新
```

“一等公民”意味着生命周期、引用、权限、审计、降级和扩展方式一致；不意味着每个资源都支持编辑或拥有相同 UI。

## 2. 三个必须分开的概念

### 2.1 ResourceRef：跨边界身份

`ResourceRef` 是 branded、opaque 的字符串。Agent、DSH Web、Flutter 和插件可以保存与回传它，但不能从中解析路径、workspace、数据库主键或凭据。

```ts
type ResourceRef = Branded<string, "ResourceRef">;
// 示例仅表示不透明性，不承诺编码：resource.01J...
```

采用不透明字符串而不是 `{scheme, id}` 的原因：

- 与当前 TS/Rust/Dart 的 `resourceRef: string` 可渐进兼容；
- 防止调用方把 `workspace`、`clouddoc` 等 scheme 当成权限；
- Host 可在不变更 wire 的情况下迁移存储位置；
- 不暴露本地路径、AppFlowy viewId 或外部 URL。

`ResourceRef` 是身份，不是授权。调用还需要有效 Binding、scope 与可选 `ResourceGrantRef`。

### 2.2 ResourceLocator：Host 私有定位

`ResourceLocator` 永不跨出拥有真源的 Host，仅由 Resource Provider 解释：

```ts
type ResourceLocator =
  | { kind: "workspace-path"; rootRef: string; relativePath: string }
  | { kind: "appflowy-view"; workspaceId: string; viewId: string }
  | { kind: "office-blob"; blobRootRef: string; blobKey: string }
  | { kind: "https-origin"; originRef: string; objectKey: string }
  | { kind: string; payload: HostPrivateValue };
```

它可以包含真实路径或数据库 ID，因为只存在于 Host 进程和受控持久层。移动、重命名、云迁移只更新 locator，不改变稳定对象关系。

### 2.3 Artifact 与 Ontology Object：语义身份

`ResourceRef` 解决“如何访问某份内容”；`Artifact` 解决“这是什么工作产物”；Ontology Object 解决“它表达或影响哪个现实/业务概念”。三者不能合并：

```text
ResourceRef ──定位──► 当前可读取内容
Artifact    ──版本──► ArtifactRevision ──使用──► ResourceRef
Feature     ──representedBy──► ArtifactAnchor ──位于──► ArtifactRevision
```

## 3. Resource Descriptor

调用方必须先 `resource.describe`，不能用文件名自行决定能力。

```json
{
  "protocol": "muse.resource/descriptor/v1",
  "resourceRef": "resource.01J...",
  "revision": "etag.sha256.7e1...",
  "display": {
    "name": "Export requirements.docx",
    "iconHint": "document",
    "description": "Product requirement"
  },
  "format": {
    "formatId": "ooxml.word",
    "mediaType": "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "confidence": 1.0,
    "detectedBy": "container-signature"
  },
  "size": 184320,
  "capabilities": [
    "resource.read",
    "resource.view",
    "resource.edit",
    "resource.diff",
    "resource.anchor"
  ],
  "allowedMaterializations": ["host-path", "byte-range", "stream"],
  "classification": "workspace-internal",
  "modifiedAt": 1789372800000
}
```

约束：

- `revision` 是不可解释的并发 token；可以是 hash、CRDT state vector 的摘要或单调版本，但调用方不得比较大小。
- `formatId` 由 Format Provider 注册。协议核心不包含任何 format 常量。
- `mediaType` 仅是互操作提示，不能代替内容嗅探和 Provider 证明。
- `capabilities` 是当前 actor、scope、placement 下的有效交集，不是资源的理论能力全集。
- Descriptor 不包含真实路径、签名 URL、token、正文、数据库 ID。

## 4. 合同族

统一协议作为 Host Bridge 上的领域合同族提供，不增加新的 wire 动词。

### 4.1 `muse.resource@1`

| Operation | Effect | 输入 | 输出 | 说明 |
|---|---|---|---|---|
| `resource.describe` | read | `resourceRef` | Descriptor | 任何交互的第一步 |
| `resource.read` | read | ref、revision?、range/selector | bounded chunk/snapshot | 小内容或结构化读取 |
| `resource.materialize` | read | ref、revision、mode、budget | Materialized Handle | 为引擎准备路径/流/range/URL |
| `resource.diff` | read | fromRevision、toRevision、profile | typed diff | 供用户、Agent、Impact Engine 消费 |
| `resource.patch.propose` | local/sync write | expectedRevision、patch | Proposal | 计算预览与风险，不落真源 |
| `resource.patch.apply` | local/sync write | proposalRef、grant、idempotency | Commit Receipt | 提交并发布事件 |
| `resource.anchor.resolve` | read | anchorRef、targetRevision | Anchor Resolution | revision 变化后重定位 |
| `resource.export` | local/external effect | ref、formatId、destination | Export Receipt | 显式转换/下载 |

订阅使用 Host Bridge 自带 `subscribe/event`，事件类型见 §8。

### 4.2 `muse.presentation@2`

| Operation / Event | 方向 | 说明 |
|---|---|---|
| `surface.open.requested` | DSH/Host → Client Adapter | 请求查看/编辑一个 `resourceRef` |
| `surface.focus.requested` | DSH/Host → Client Adapter | 聚焦既有 Surface |
| `surface.reveal.requested` | DSH/Host → Client Adapter | 解析 Anchor 并定位 |
| `surface.close.requested` | Host → Client Adapter | 关闭或释放 Surface |
| `surface.intent.receipt` | Client Adapter → Host | 最终状态、承载器、surfaceRef、原因 |

当前 `muse.presentation-intent/v1` 可作为 v2 的载荷 envelope 继续使用；v2 的变化重点是 payload 只携带 `resourceRef/anchorRef/disposition`，不再内嵌格式专用 `viewId/blockId`。

### 4.3 领域合同不被资源合同取代

`muse.document@2`、`muse.table@1`、未来 `muse.cad@1` 继续拥有领域语义。例如“在 Word 段落后插入文本”“更新表格单元格公式”“给 CAD 面加公差”不是通用 byte patch。

Resource Provider 在 `resource.patch.propose` 中根据 `patchFormat` 分派到对应领域 Provider：

```json
{
  "resourceRef": "resource.01J...",
  "expectedRevision": "etag...",
  "patch": {
    "patchFormat": "muse.document-mutation/v2",
    "value": { "kind": "replace", "find": "old", "text": "new" }
  }
}
```

通用层只理解 `patchFormat + value + expectedRevision`，不理解 document block 或 spreadsheet cell。

## 5. Materialization：统一路径交付与内容交付

### 5.1 Handle 类型

| Mode | 使用者 | 内容 | 安全要求 |
|---|---|---|---|
| `host-path` | 与资源同一 Host 的 Helix/ioffice | Host-local opaque handle，最后一跳才解析为路径 | same-host、工作区包含、普通文件、lease |
| `byte-range` | PDF/CAD/3D/大型媒体 Viewer | 可 seek 的分段 reader | 每次 range 与总量配额、deadline |
| `stream` | Viewer、解析器、导出器 | 带 backpressure 的字节流 | frame 上限、取消、hash/revision 校验 |
| `loopback-url` | 受控 WebView/open-file-viewer | 一次性 localhost URL | 短 TTL、单资源、origin/CSP、禁止目录遍历 |
| `semantic-snapshot` | Agent、Ontology extractor | 有界结构化投影 | schema digest、字段脱敏、token/byte budget |
| `external-url` | 明确允许的远程资源 | 受策略约束的 URL handle | allowlist、无凭据、默认只读 |

`MaterializedHandle` 不是永久资源地址。它必须绑定：

- `resourceRef + revision`；
- actor/binding/scope；
- engine/surface 或 operation；
- `expiresAt`、budget、允许的读法；
- 可撤销 lease。

### 5.2 为什么不能用 Bridge 直接传整文件

现有 Host Bridge 设计上只允许有界 JSON，最大 output 1 MiB。Office、媒体、CAD 和 3D 文件通常远超上限。Bridge 负责控制面，字节数据走由 Bridge 签发的受限数据面；这保持协议治理而不让控制面变成文件传输隧道。

## 6. Format Descriptor 与注册

格式知识由 Provider/Adapter 自报，不由 Resource Core 集中维护：

```ts
interface FormatDescriptorV1 {
  formatId: string;
  mediaTypes: readonly string[];
  extensions?: readonly string[];       // 低可信提示
  signatures?: readonly ContentSignature[];
  containerProbe?: string;              // 对应可发现的 probe capability
  semanticProfiles: readonly string[];
  anchorKinds: readonly string[];
}
```

识别顺序：

1. 真源 Provider 的已知类型；
2. 容器结构与 magic/signature；
3. 可信服务器 Content-Type；
4. 扩展名；
5. `unknown.binary` / `unknown.text`。

如果结果冲突，Descriptor 记录候选和置信度；涉及执行或写入时要求高置信或用户确认。中央核心不得出现 `.docx → ioffice` 之类分支。

## 7. 写入事务

### 7.1 标准闭环

```text
describe(revision R7)
  → patch.propose(expected R7, patch, idempotency K)
  → Proposal{before/after/risk/requiredApproval}
  → policy.evaluate(actor, action, resource)
  → user/automation approve → Grant{single-use, TTL}
  → patch.apply(proposalRef, grant, K)
  → CommitReceipt{R7 → R8, changed}
  → resource.changed event(cursor C19)
```

不变量：

- expected revision 不匹配必须返回 `REVISION_CONFLICT`，禁止 last-write-wins 静默覆盖；
- apply 必须幂等；相同 idempotency key 只能得到同一结果；
- Proposal 有 TTL，绑定 actor、resource、revision 和 patch digest；
- side effect 与真源提交的原子性必须显式声明。不能原子时采用 outbox + 可重试状态；
- 外部系统动作必须有补偿策略或清楚标记不可逆。

### 7.2 人类原生编辑器直接保存

Helix 或 ioffice 可能直接写 Host-local 文件，无法先走 `patch.propose`。这类 Surface 必须持有单写者 lease，并由 Resource Provider 观察保存：

1. 打开时固定 baseline revision；
2. 保存前/后验证文件仍在允许根目录且类型未变；
3. 生成新 revision 与 typed diff；
4. 发布 `resource.changed`，origin=`human-surface`；
5. 检测到外部并发变化时进入显式冲突/重新加载流程。

Agent 与 Workflow 的写入不得使用这条旁路。

## 8. 资源事件

```json
{
  "protocol": "muse.resource/event/v1",
  "eventId": "event.01J...",
  "cursor": "18291",
  "eventType": "resource.changed",
  "resourceRef": "resource.01J...",
  "previousRevision": "etag.r7",
  "revision": "etag.r8",
  "changeKind": "content",
  "origin": "human-surface",
  "commandRef": "command.01J...",
  "occurredAt": 1789372800000,
  "summaryRef": "diff.01J..."
}
```

事件类型至少包括：

- `resource.created / changed / renamed / moved / deleted / restored`；
- `resource.capabilities-changed`；
- `resource.anchor-orphaned / anchor-relocated`；
- `resource.lease-acquired / lease-released / lease-conflicted`；
- `resource.materialization-expired`。

正文与完整 diff 不进入事件；消费者按 `summaryRef` 有界读取。Model-visible 事件必须可从 durable log 重建。

## 9. Resource Authority

### 9.1 引用签发

允许签发 `resourceRef` 的入口：

- Host Registry 的 scoped discovery；
- 已验证的 DSH session deliverable 坐标；
- 用户显式选择/拖入；
- Ontology Object 上已有、且当前 actor 可读的 Artifact 关系；
- 受信系统连接器的查询结果。

禁止入口：模型自由拼写路径、URL、viewId 或数据库 ID。

模型可以**回传**刚由受权工具返回的 `resourceRef`。这与“模型不能选择 ID”并不矛盾：它不能创造身份，但可以使用 Host 已签发的能力引用。

### 9.2 Path compatibility adapter

现有 DSH `present(files[].path)` 通过坐标 `(sessionId, seq, index)` 回读 durable event 并校验文件。兼容适配器在校验后签发 `resourceRef`，而不是直接把 path 交给 Surface：

```text
present coordinates
 → read session event
 → workspaceFiles.stat + host/process roundtrip
 → ResourceAuthority.issue(locator=workspace-path, scope=session)
 → resourceRef
```

该安全链完整保留。路径只在 Host 内部出现。

## 10. 错误模型

| Code | 含义 | 可重试 |
|---|---|---|
| `RESOURCE_NOT_FOUND` | 引用存在但真源不存在/已删除 | 否，除非恢复 |
| `RESOURCE_REF_INVALID` | 引用不是本 Host 签发或格式错误 | 否 |
| `RESOURCE_SCOPE_DENIED` | actor/binding/scope 不匹配 | 否 |
| `CAPABILITY_UNAVAILABLE` | 当前平台/资源/actor 无该能力 | 可能，切换承载器 |
| `MATERIALIZATION_UNAVAILABLE` | 引擎要求的 mode 不可用 | 可能，选择另一引擎 |
| `BUDGET_EXCEEDED` | range、stream、snapshot 超限 | 是，缩小请求 |
| `REVISION_CONFLICT` | expected revision 过期 | 是，重读并重做提议 |
| `LEASE_CONFLICT` | 已有写者 | 是，聚焦/只读/等待 |
| `ANCHOR_ORPHANED` | 无法安全重定位 | 否，人工重绑 |
| `FORMAT_AMBIGUOUS` | 内容识别不足以写入/执行 | 否，用户确认 |
| `HANDLE_EXPIRED` | 临时数据面句柄失效 | 是，重新 materialize |

所有失败必须映射到一个最终 Intent/Operation Receipt，不允许“点击无反应”。

## 11. 版本与兼容

- Host Bridge wire major 仍是 1；Resource 是新的可发现 contract family。
- 新字段只在 schema minor 兼容范围内增加；语义破坏发布新 major。
- TS/Rust/Dart 从同一 JSON Schema/golden fixtures 生成或校验。
- Host discovery 返回 Provider 支持的 contract version、operation、effect 和 schema digest。
- 旧客户端不知道 Resource Surface 时，DSH path deliverable 继续 Sidebar/system fallback。
- `muse.document@2` 和 `muse.presentation-intent/v1` 在迁移期双栈运行，由 Adapter 转换，不做“大爆炸”切换。

## 12. 协议准入测试

任何新格式/Provider 必须通过：

1. 只新增 Manifest、Format Descriptor、Provider/Adapter，不修改 Resource Core 格式列表；
2. 伪造路径、过期 grant、跨 scope ref 被拒绝；
3. 大小、range、流量、TTL、并发和取消有硬上限；
4. stale revision 不会覆盖；
5. materialized handle 过期或插件卸载后不可使用；
6. 同一请求只产生一个终态回执；
7. Provider unload 后 descriptor、route、handle、listener 全部释放；
8. 未知格式能降级到 metadata/download/system，不崩溃；
9. 资源移动/重命名后 ref 身份与 Ontology 关系保持；
10. 假格式 `application/x-muse-tck` 仅靠外部 Adapter 即可完成 discover→view→patch 测试。

