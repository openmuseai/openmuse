# Host、DSH、Surface 与格式引擎协同设计

## 1. 领域职责

| 领域 | 唯一职责 | 禁止承担 |
|---|---|---|
| Resource Authority Host | resourceRef 解析、权限、revision、物化、提交、事件 | UI 路由偏好、格式内部编辑语义 |
| Surface Orchestrator | 候选 Adapter、probe、评分、Surface 去重、生命周期、receipt | 解引用本地路径、解析 Office/PTY/WebGL |
| Engine Adapter | 把公共 session 意图翻译为具体引擎调用 | 自行扩大权限、创建全局资源身份 |
| Engine | 渲染/编辑格式本身 | 认识 DSH、Ontology、workspace policy |
| DSH Host Provider | 在 DSH capability seam 上暴露 Host 可执行能力 | 替 Host 做最终授权 |
| DSH UI Consumer | 将 deliverable/mention/card 转为 OpenResource request | 解析扩展名并直接选择引擎 |
| DSH Agent Consumer | 为模型提供窄、已记录的 open/read/propose 工具 | 接受任意 path、engineId 或原始凭据 |
| Flutter/Web Surface | 呈现、输入、焦点、状态和错误 | 持有长期 access token、静默写入 |

## 2. Capability seam

DSH 侧必须完整提供三种角色：

```text
Service Definition
  @muse/dsh-resource-presentation
    OpenResourceService.request()
    OpenResourceService.status()

Provider
  @muse/dsh-resource-presentation-host
    Host Bridge client
    session/workspace/actor binding

Consumers
  @muse/dsh-client-ui-resource-open
  @muse/dsh-tool-resource-present
  ui-deliverables / inline mentions / presented cards adapters
```

注册与订阅使用 Cordis effect/disposer；缺 Provider 在组合加载阶段或首次可判断点明确失败。模型可见请求与结果写入 session event，确保重放时可解释，不能只在 React 本地状态中存在。

## 3. 控制面与数据面

### 3.1 控制面

所有引擎共用 Host Bridge：

- discover/bind：发现 Resource、Presentation、Engine Session capabilities；
- invoke：describe、requestOpen、close、commit、resolveAnchor；
- subscribe/event：resource change、session state、selection/context；
- cancel/status：取消打开或长操作，查询最终 receipt；
- policy：权限/能力撤销和端能力变化。

控制消息遵守 Host Bridge 现有大小限制。Descriptor、状态、错误、短上下文和 handle 可以进入 envelope，完整 docx、PDF、模型和 PTY 帧不进入。

### 3.2 数据面

| 引擎 | 数据面 | 授权方式 | 生命周期 |
|---|---|---|---|
| ioffice | Host-local bytes、临时只读文件或 FFI memory | materialization handle 绑定 resource/revision/consumer | 随 EngineSession/lease 释放 |
| Helix | PTY 双向 byte stream；编辑文件为 direct-safe path 或 working copy | stream handle + single attach + device/session token | 进程退出、lease revoke、Host shutdown |
| Viewer | Blob/ArrayBuffer、短期 loopback/HTTPS URL、HTTP range | URL bearer + origin/audience/TTL/range policy | Viewer destroy/TTL/revoke |

数据面句柄是短期 capability，不等于稳定 resourceRef。

## 4. 打开事务

### 4.1 请求

```json
{
  "protocol": "muse.presentation/request/v2",
  "requestRef": "preq_01J...",
  "resourceRef": "resource.opaque",
  "disposition": "open",
  "requestedMode": "prefer-edit",
  "anchorHint": {
    "provider": "muse.text-anchor/v1",
    "value": { "line": 42, "column": 7 }
  },
  "placementHint": "current-window",
  "cause": {
    "kind": "dsh-deliverable",
    "sessionRef": "session.opaque",
    "eventRef": "event.opaque"
  }
}
```

`requestedMode` 是偏好，不是授权。请求不接受强制 engine ID；用户“打开方式”只能携带 Host 刚签发的 adapterChoiceRef。

### 4.2 时序

```text
DSH Consumer
  → Host Provider: requestOpen(resourceRef, intent)
  → Resource Host: authorize + describe(revision)
  → Orchestrator: enumerate adapters
  → Adapters: probe(descriptor, placement, policy)
  → Orchestrator: select + create route decision
  → Resource Host: materialize(selected mode/audience)
  → Adapter: create EngineSession
  → Surface Runtime: open lease + focus
  → Adapter: ready / failed
  → Orchestrator: final receipt
  → DSH Consumer: persist/display result
```

### 4.3 原子性

打开不是数据库事务，但必须满足：

- 一个 requestRef 只有一个 terminal receipt；
- 每个已创建的 materialization、EngineSession、Surface lease 都有 owner 和 disposer；
- 任一步失败会逆序释放已完成资源；
- fallback 是新的 route attempt，沿用 root requestRef、生成独立 attemptRef；
- 用户取消后不再创建新 Surface；已启动引擎进入有界清理；
- late ready/failed 事件因 generation 不匹配被忽略。

## 5. 路由

候选必须同时满足：

1. Descriptor 的 format/mime/capability 与 Adapter 支持相交；
2. 当前 OS/ABI/runtime/engine artifact probe 成功；
3. policy 允许 requested/effective mode；
4. 数据面存在可用 materialization；
5. Adapter 健康，未熔断；
6. 文件大小、加密、宏、GPU/内存等限制可接受。

推荐优先级：

```text
security/compliance
  > effective capability
  > placement/device
  > workspace policy
  > one-shot user choice
  > user default
  > fidelity/editability
  > startup cost
```

示例：

- docx + macOS arm64 + ioffice ready + edit allowed → ioffice。
- docx + Web 或 ioffice unavailable → Viewer office plugin，实际 mode=view。
- text/code + Desktop + edit allowed → Helix；只读或 Mobile → Viewer text。
- PDF → Viewer；ioffice PDF 只有未来真实 engine probe 成功后才成为候选。

## 6. EngineSession 状态机

```text
allocating → materializing → starting → ready ↔ focused/background
      └──────────────→ failed       │
                                    ├→ dirty → committing → ready
                                    │              └→ conflict/failed
                                    └→ closing → closed
                                          └→ forced-terminated
```

状态要求：

- `dirty` 只能由可编辑 Adapter 报告。
- `committing` 必须引用 base revision；成功返回新 revision。
- `conflict` 保留工作副本/内存 session，不丢用户修改。
- read-only EngineSession 永远不能进入 dirty/committing。
- Surface close 与 EngineSession terminate 分开；可支持背景 session，但 owner/TTL 明确。
- Host policy revoke 可从任何状态进入 closing。

## 7. 人类编辑与提交

### 7.1 ioffice

```text
resource bytes@R1 → WordSession → user edits heap
  → toDocx() [硬门]
  → Host resource.commit(expected=R1, bytesHandle)
  → Provider atomically stores bytes@R2
  → resource.changed(R1,R2)
```

没有 `toDocx()` 时只能 view/ephemeral-edit；Save disabled，关闭 dirty 时明确丢弃，不允许提交打开时旧字节。

### 7.2 Helix

两种 materialization：

- `direct-safe`：仅 Host-local workspace file Provider 明确允许，Helix 直接写真实路径；Provider 监控/核验并发出 revision event。
- `working-copy`：默认。Host 生成单文件工作副本和 base revision；Helix `:w` 写副本，Adapter 检测稳定 hash 后调用 commit。

冲突时禁止覆盖原资源；保留工作副本，提供 compare/retry/save-as/放弃。PTY 退出不等于 commit 成功。

### 7.3 Viewer

永远不提交原始资源。下载、打印、外链、全屏是独立 policy capability；未来批注保存为单独 Annotation Resource，不修改原文件。

## 8. DSH 协同

### 8.1 用户点击

`ui-deliverables`、ProducedFiles、inline mentions、cards 统一调用 UI Consumer。旧 `openFile(path)` 只作为 feature-flag fallback，且必须先用 session coordinates 回读 durable event 和 Host 验证路径，再换成 resourceRef。

### 8.2 Agent 打开

Agent 工具输入：

```json
{
  "resourceRef": "resource.opaque",
  "anchorHint": { "provider": "muse.text-anchor/v1", "value": { "line": 10 } },
  "mode": "view"
}
```

工具只能回传最近由 catalog、search、deliverable 或其他受权工具发现的 resourceRef。工具结果包含 requestRef、effective mode、placement、receipt/status ref；不返回 path、URL bearer 或 EngineSession secret。

### 8.3 Agent 读取与编辑

本轮 Agent 只支持：

- 读取 Provider 提供的有界 snapshot；
- 请求呈现或聚焦资源；
- 在已有领域合同允许时生成 proposal。

不允许：

- 通过 PTY 任意发送 Helix 命令修改文件；
- 操作 Viewer DOM；
- 在 ioffice 缺 `toDocx` 时绕过 Save；
- 创建 Ontology 对象或触发影响传播。

如未来允许 Agent 与用户共享 Helix 会话，必须单独设计 terminal ownership/approval 和 transcript；不作为当前引擎接入隐含能力。

## 9. 上下文事件

通用事实事件：

| Event | 最小字段 | 用途 |
|---|---|---|
| `surface.ready` | sessionRef、resourceRef、revision、mode、adapterRef | DSH/UI 显示完成 |
| `surface.focus.changed` | sessionRef、focused | Context 生效/失效 |
| `surface.selection.changed` | resourceRef、revision、selector、summary、TTL | 当前选区上下文 |
| `engine.dirty.changed` | sessionRef、dirty、workingCopyRevision | 保存状态 |
| `engine.commit.terminal` | sessionRef、result、newRevision/error | 保存结果 |
| `resource.changed` | resourceRef、before/after revision、origin | 已提交真实变化 |

事件 payload 服从 Context/Event 大小限制，正文用 snapshotRef/selector 获取。只有 `resource.changed` 表示真源变化；光标和 dirty 不是业务变化。

## 10. Transport

| Placement | 控制面 | 数据面 |
|---|---|---|
| Desktop local | Host Bridge UDS/InProcess + Rust/FFI → Flutter | FFI bytes、loopback URL、PTY WS/Unix stream |
| Web remote | HTTPS/WebSocket transport of Host Bridge envelope | HTTPS range/stream、sandbox iframe worker |
| Mobile remote | HTTPS/SSE transport of Host Bridge envelope | 分页/range/缩略图；不承载 Helix PTY |

parent bridge 迁移期可以双栈，但业务类型只在 Host Bridge contract 层解释；transport adapter 不再增加 `word-open`、`viewer-open` 等私有消息。

## 11. 故障与回退

| 故障 | 行为 |
|---|---|
| Host 不可达 | 请求失败；显示重试/其他 Host，不创建本地假 session |
| Adapter probe 失败 | 选下一候选；记录 unavailable reason |
| materialization 超时 | 取消读取并清理 handle；可降级 metadata |
| engine start 崩溃 | 熔断该 adapter/version，Viewer 或 system-open fallback |
| Surface 消失 | terminate/后台保留按策略；DSH receipt 不重复 |
| permission revoke | 关闭数据面、清缓存、终止可写 session |
| revision conflict | 保留用户修改，禁止自动覆盖 |
| DSH reload | 从持久 request/status ref 恢复结果，不重复打开副作用 |

## 12. 协同验收

- 三种 DSH UI 入口调用同一 Consumer，并通过 session snapshot 测试。
- Agent open 工具的 model-visible 请求/结果都存在于 durable session event。
- Host Bridge 不出现完整 docx/PDF/PTY payload。
- Route/Materialize/Engine/Surface 的每个资源都有 disposer，故障注入后计数归零。
- permission revoke、cancel、late event、duplicate request、Host restart 有确定行为。
- Ontology 包不存在时所有引擎功能仍完整工作。
