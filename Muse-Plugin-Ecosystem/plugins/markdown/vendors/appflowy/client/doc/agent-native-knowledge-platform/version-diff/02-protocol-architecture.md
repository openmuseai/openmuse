# 协议与技术架构

## 1. 分层

```text
┌──────────────────────────────────────────────────────────────┐
│ Host / Workspace / Tab / DSH                                │
│ version selection, permission, routing, audit presentation   │
├──────────────────────────────────────────────────────────────┤
│ Universal Version & Change Protocol                          │
│ Repository Resource Version Comparison Proposal Action       │
├──────────────────────────────────────────────────────────────┤
│ Domain Provider                                              │
│ Text | Office | Spreadsheet | Slides | Media | CAD | ...     │
│ Semantic IR → matching → ResourceDiff payload → renderer     │
├──────────────────────────────────────────────────────────────┤
│ Content & Metadata Backend                                   │
│ AppFlowy Collab | Local CAS | Git | SSH | S3/Cloud | ...     │
└──────────────────────────────────────────────────────────────┘
```

上层不能向下绕过 Provider 解释领域对象；下层不能决定 Host 的 Tab、权限和审计体验。

## 2. 通用协议

### 2.1 Repository

```json
{
  "id": "repo:team-knowledge/acme",
  "providerId": "appflowy-collab-v2"
}
```

Repository 是逻辑命名空间。一个 Workspace 可以 Mount 多个 Repository；同一 Provider 也可提供多个 Repository。

### 2.2 Resource

```json
{
  "id": "resource:sha256/...",
  "repository": "repo:...",
  "locator": "file:///workspace/spec.md",
  "mediaType": "text/markdown",
  "displayName": "spec.md"
}
```

`locator` 是 Provider 可解释的地址，不保证是本地路径。`resource.id` 必须在 Repository 内稳定，不以 Tab 生命周期为边界。

### 2.3 Version

```json
{
  "id": "version:...",
  "resourceId": "resource:...",
  "kind": "committed | working | proposal",
  "contentRef": "sha256:...",
  "contentDigest": "...",
  "byteLength": 1024,
  "parents": ["version:..."],
  "actor": {"id": "...", "displayName": "..."},
  "createdAt": "RFC3339",
  "message": "Review baseline"
}
```

不变式：

- Version 创建后不可修改。
- `contentDigest` 必须验证物化内容。
- 父 Version 可为 0..N，允许 root、线性历史和 merge。
- working Version 也必须在一次 Comparison 生命周期内保持不可变。

### 2.4 Comparison

```json
{
  "id": "comparison:...",
  "resourceId": "resource:...",
  "base": "version:A",
  "target": "version:B",
  "rendererType": "muse.diff-viewer.text.v1",
  "proposalId": null
}
```

Comparison 是关系，不是 Version 的属性。普通查看可以只保留审计元数据；若关联 Proposal，必须能够复现当时的 base 与 proposed Version。

### 2.5 ResourceDiff

通用外壳只包含：

```text
comparison
payload        // opaque domain-owned data
changeCount
summary
```

`payload` 的 Schema、版本和 Renderer 完全属于领域插件。通用层不能出现 line、slide、cell、frame、shape 或 geometry。

### 2.6 Proposal 与 Action

Proposal 持久化“为什么改、谁提出、基于什么、候选状态是什么”。Action 持久化用户决定，并调用领域 Mutation Provider。

```text
Proposal(base=A, proposed=B)
   ├── Decision(change-1, accept)
   ├── Decision(change-2, reject)
   └── Apply(current=C)
          ├── success → Version D
          └── conflict → ConflictSet
```

应用必须是三方过程：base / proposed / current。Viewer 只能发出 Action Intent，不能直接写文件。

## 3. Provider SPI

P0 Dart 接口：

```dart
abstract interface class MuseDiffProvider<TPayload> {
  String get id;
  String get rendererType;
  bool supports(MuseResourceRef resource);
  Future<MuseResourceDiff<TPayload>> compare(...);
}
```

完整目标 SPI：

```text
ResourceProvider
  stat / read / write / watch / capability / lease

VersionStore
  capture / materialize / list / graph / tag / retain

DiffProvider<TIR, TDiff>
  parse / normalize / match / compare / summarize

DiffRenderer<TDiff>
  buildViewModel / render / revealChange

MutationProvider<TDiff>
  validate / apply / merge3 / rollback
```

Registry 按显式 capability、media type、schema version 与优先级选择 Provider。扩展名只可作为弱提示。

## 4. Semantic IR

每个领域自己的 IR 至少满足：

- 节点有稳定或可重建 ID。
- 节点有类型、属性和父子/引用关系。
- 能区分内容、格式、位置和结构变化。
- 能把低层变化提升为用户能理解的摘要。
- 能从 Diff 反向定位原始资源中的区域。

领域例子：

| Domain | IR 节点 | 主要匹配键 | Viewer |
|---|---|---|---|
| Text/Code | symbol, hunk, line span | syntax path + content fingerprint | unified/split |
| DOCX | paragraph, run, table, comment | OOXML id + structural fingerprint | paged redline |
| PPTX | slide, shape, animation | slide/shape id + geometry/content | slide overlay |
| XLSX | sheet, range, formula, chart | sheet id + address + formula graph | grid/formula graph |
| MP4 | shot, time range, track, caption | timecode + perceptual fingerprint | dual player/timeline |
| CAD | assembly, part, feature, constraint | native object id + topology signature | 3D overlay/tree |

IR 是 Diff 计算的内部/领域协议，不进入 universal core。

## 5. Version Graph 与 Provenance Graph

必须分开两张图：

- Version Graph：内容状态的 parent DAG。
- Provenance Graph：Agent task、DSH message、tool call、Proposal、Decision 和 Action 的因果关系。

不能把 Agent 对话伪装成 commit parent，也不能把 merge commit 当作“Agent 已批准”的证据。二者通过 ID 引用连接。

## 6. 存储架构

### 6.1 P0 Local CAS

```text
Application Support/OpenMuse/version-diff-v1/
  blobs/<sha256>
  versions/<resource-id>/<version-id>.json
  audit/<resource-id>.jsonl
```

- Blob 内容寻址和去重。
- Version sidecar 不可变。
- Audit JSONL append-only。
- Working Version 写入 Blob 但不进入历史元数据。

P0 是可替换后端，不是最终 Workspace Local Source。后续 Workspace Provider 应通过统一 VersionStore 接口提供本地、SSH 和云端实现。

### 6.2 AppFlowy Collab Adapter

保留现有协作、分享、锁定、权限与历史服务：

- AppFlowy View ID 映射 Resource ID。
- Collab snapshot/update boundary 映射 Version。
- 原权限服务映射 capability。
- 原历史数据不迁移为本地 Blob；由 adapter 物化。
- Markdown/Database/Word 领域分别提供 DiffProvider，不以 AppFlowy 页面类型污染通用层。

### 6.3 Remote Backend

SSH/S3/企业云实现需要：

- ETag/digest 条件读取。
- 断点/流式物化。
- 本地加密缓存与 TTL。
- 离线 Version 的来源标记。
- 写入前 lease 或 compare-and-swap。

## 7. 审计与安全

审计事件包含 actor、time、repository/resource、subject、动作类型与最小 metadata。禁止记录内容正文和秘密。

企业目标：

- Event ID 幂等。
- 事件链 hash/signature。
- 保留策略与 Legal Hold。
- Actor 区分 human、agent、service。
- 每个 Action 记录 policy decision、base/current/result。
- Provider 运行在最小权限边界；远端凭证由 Host secret store 管理。

## 8. 并发与冲突

普通 Comparison 没有写冲突。Action 应用时：

1. 检查 current digest 是否等于 Proposal base。
2. 相等则按领域 patch 应用。
3. 不相等则请求 MutationProvider.merge3。
4. 无法自动合并时返回结构化 ConflictSet。
5. 用户解决后产生新 Version 和审计事件。

任何失败不得覆盖 current，也不得删除 proposed Version。

## 9. 参考实现取舍

- 采用 Git 的不可变对象、内容寻址和 parent graph 思想，但不要求 Git 工作区。
- 采用 VS Code 的 Provider/URI/Custom Editor 生命周期思想，但 Host 保持统一 Tab 与权限模型。
- 文本 Viewer 的 unified/split 交互参考 Monaco Diff Editor；P0 使用 Flutter 原生实现以保持 Host 主题和布局一致。
- Office/OpenXML 后续 Provider 应解析原生对象模型，不能只 unzip 后对 XML 文本做用户界面 Diff。

参考：

- https://git-scm.com/docs/gitdatamodel
- https://code.visualstudio.com/api/references/vscode-api
- https://code.visualstudio.com/api/extension-guides/virtual-documents
- https://microsoft.github.io/monaco-editor/typedoc/functions/editor_editor_api.editor.createDiffEditor.html

