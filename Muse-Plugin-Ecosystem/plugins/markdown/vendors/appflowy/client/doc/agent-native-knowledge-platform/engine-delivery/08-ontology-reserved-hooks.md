# Ontology 延后实现：预埋接口与边界

## 1. 决策

本轮只交付格式引擎、Resource、Presentation、EngineSession 与 DSH 协同。Ontology Runtime 完全延后。

目标不是删掉未来能力，而是在不实现业务语义的前提下保存最小兼容点，使后续 Ontology 接入不需要修改 vendor 或推翻引擎 session。

## 2. 本轮允许实现

### 2.1 稳定身份

- opaque `resourceRef`；
- opaque `surfaceInstanceRef` / `engineSessionRef`；
- opaque revision token；
- Host-private locator；
- adapter/provider/version/digest。

### 2.2 事实事件

```text
resource.changed
resource.moved
resource.deleted
surface.ready
surface.focus.changed
surface.selection.changed
engine.dirty.changed
engine.commit.terminal
```

这些事件只描述资源和会话事实，不推导业务含义。

### 2.3 Selector/Anchor hint

允许 envelope：

```json
{
  "provider": "muse.word-selection/v1",
  "resourceRef": "resource.opaque",
  "revision": "sha256:...",
  "value": {
    "pageIndex": 3,
    "startCp": 120,
    "endCp": 148
  },
  "quote": "失败后最多自动重试三次"
}
```

它只用于：

- 重新打开时定位；
- DSH 有界 Context；
- 将来创建 Ontology Anchor 的输入。

本轮不赋予 `anchorId`，不写入图谱，不承诺跨 revision 自动重定位。

### 2.4 有界 snapshot

Resource Provider 可以提供：

- text range/line/page；
- Word plain text/selection；
- Viewer page/selection summary；
- Helix 当前文件/line selection（只有可靠 IPC 时）。

Snapshot 带 revision、maxBytes、truncated 和 policy，未来 Context Broker 可复用。

### 2.5 Trace/Causal

open、commit、resource.changed 使用 trace/rootCause/origin，供未来 ChangeSet 关联；当前不生成 ChangeSet。

## 3. 本轮禁止实现

- `Requirement`、`Decision`、`CodeSymbol`、`TestCase` 等 ObjectType。
- `implements`、`verifiedBy`、`representedBy` 等关系实例。
- Claim 抽取、置信度、人工确认队列。
- Anchor 持久化/重定位服务。
- resource.changed → 语义 diff。
- Impact Engine、Impact Inbox。
- ActionType/ActionRun、Workflow/Automation。
- 为了未来 Ontology 将全文复制到新数据库/向量库。
- 在 vendor API 中出现 Ontology、Object、Claim、Action 名称。

## 4. 引擎预埋点

| 引擎 | 本轮 hook | 后续可能用途 |
|---|---|---|
| Helix | resource/revision、可靠时的 file/line/selection、commit event | CodeSymbol Anchor、实现关系 |
| ioffice Word | page、CP selection、plainText snapshot、bytes revision | Requirement/Decision Anchor |
| Viewer PDF | page、bounds/selection（插件支持时）、revision | Evidence Anchor |
| Viewer CAD/3D | provider-namespaced entity selection | Component/Asset Anchor |

“可能用途”不是本轮验收项。Hook 失败只能影响 Context，不影响打开/编辑。

## 5. 扩展兼容规则

- JSON schema 允许明确的 `extensions` namespace，但核心未知字段不影响现有语义。
- major version 才能破坏 selector/value 语义。
- selector provider ID 和 version 固定；引擎内部坐标不能冒充跨格式标准。
- revision 是 opaque；未来 Ontology 只比较引用，不解析排序。
- ResourceEvent 的大 diff 使用 snapshotRef，不在 event 中预留任意业务 JSON。
- Context Provider 是可选 capability；Orchestrator 不因其缺失拒绝打开。

## 6. 后续接入点

Ontology 项目启动后新增 Consumer：

```text
ResourceEvent / Context / Snapshot
        │
        ▼
Ontology Ingestion Consumer
  → proposed Anchor/Claim/Object
  → human/rule confirmation
  → ChangeSet/Impact/Action
```

已有 Provider/Adapter 不改变：

- Resource identity；
- open/materialize/commit；
- EngineSession 生命周期；
- Context/selector 生成。

只新增独立的 ingestion、store、query 和 workflow packages。

## 7. 本轮兼容测试

| ID | 场景 | 预期 |
|---|---|---|
| ON-H1 | Ontology packages 不安装 | 全部引擎 open/edit/view 正常 |
| ON-H2 | Context Provider 缺失 | Surface 正常，只无 selection Context |
| ON-H3 | 无 Context Consumer | event 发布不阻塞、不无限缓存 |
| ON-H4 | 未知 selector provider | Host 透传/明确 unsupported，不崩溃 |
| ON-H5 | selector 超大小 | 截断/引用 snapshot，不塞入 event |
| ON-H6 | revision 变化 | 旧 selector 标记 stale，不自动语义重绑 |
| ON-H7 | future extension field | 旧 reader 按版本规则处理 |
| ON-H8 | vendor dependency scan | vendor 不引用 Muse/Ontology package |

## 8. 启动 Ontology 的前置 Gate

后续只有在以下基础稳定后才启动 Runtime：

- resourceRef 生命周期、revision 和权限冻结；
- Word/Helix/Viewer 至少两种 selector 通过真实漂移 corpus；
- ResourceEvent 可靠、可去重、可追溯；
- snapshot 有大小/权限/新鲜度保证；
- 不同端的 Context 语义一致；
- 事实事件与用户 dirty/自动保存噪声已分离。

这能避免 Ontology 建在不稳定的路径、viewId、屏幕坐标或编辑器内部对象上。

