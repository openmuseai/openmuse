# F0.3 详细产品与技术设计

## 1. 产品目标

用户无论从任务交付物、文档内 mention、Agent card 还是 Agent 工具发起打开，都应把
Word、PDF、代码、CAD 或未来格式当成同一种 Resource。DSH 只表达“呈现哪个资源、偏好
什么模式、放在哪里”，Host 决定权限，后续 Orchestrator 决定引擎。

本阶段成功标准不是显示真实文件，而是把 DSH 到 Host 的能力接缝做成可组合、可回放、
不泄密、可撤销的基础设施。

## 2. 核心场景

### S1：产品经理点击任务中的 PRD 交付物

1. deliverable 已携带 Host 签发的 opaque `resourceRef`。
2. UI Consumer 生成 cause=`dsh-deliverable` 的 Presentation v2 request。
3. request 先写入当前 DSH Session，再通过 Provider 到 Host。
4. Host 返回 terminal receipt；UI 显示已打开/降级/失败。
5. DSH reload 后从 terminal event 恢复卡片，不重复打开。

验收：不读取 `.docx` 扩展名，不传路径，不指定 ioffice。

### S2：开发人员点击文档中的代码 mention

mention 只把 resourceRef、anchorHint、placement 转为 cause=`dsh-mention` 的相同请求。
未来 Helix 或 Viewer 由 Orchestrator 选择；DSH 不知道 selector 最终如何定位。

验收：deliverable/mention/card 除 cause 外的规范化字段完全一致。

### S3：Agent 请求展示刚搜索到的资源

1. catalog/search/authorized tool 在当前 Session 记录 `muse/resource-discovered`。
2. 模型调用 `present_resource(resourceRef, mode, placement, anchorHint)`。
3. Tool 校验 Agent-owned Session，并在默认 30 分钟、256 events 双窗口内查找证据。
4. 先写 requested event，再调用 Host，最后写 terminal event。
5. Tool 只向模型返回安全投影，不返回 EngineSession、Surface、Adapter、URL 或 path。

验收：未发现、跨 Session、过期、path-shaped 输入都在 Host 调用前拒绝。

### S4：Host Provider reload / binding revoke

Provider 收到 host generation、descriptor changed 或 binding invalidated 事件后清除绑定；
下一请求重新 discover/bind。dispose abort active invoke 并等待收敛，不遗留计时器或绑定。

## 3. 不同端功能集

| 端/角色 | F0.3 已实现 | F0.3 明确不做 | 指标 |
|---|---|---|---|
| DSH Desktop UI | deliverable/mention/card；view/prefer-edit/edit intent；四种 placement；replay | 真实 Flutter Surface、extension route | 三入口 100% 共用 `requestPresentation`; reload 0 次副作用 |
| DSH Web UI | 相同 Consumer 可装配；Host availability 明确失败 | Blob/range/origin、iframe 数据面 | 无 Provider 时明确失败；wire 不含内容 bytes |
| DSH Mobile UI | 相同格式无关 intent 可装配 | low-memory profile、placement 降级 | 本阶段不得加入 mobile 私有消息；F0.5 再冻结能力表 |
| DSH Agent | `present_resource`、session recent allowlist、安全 result | path/URL/engineId、PTY/DOM/Office 内部 API | 30min + 256 event 双界；模型输出 secret scan 0 命中 |
| Host Provider | exact schema digest、scoped bind、request/status/cancel、幂等缓存 | 自报 authority、路由评分、数据面 | requestRef 幂等；失效后下一请求重新 bind；dispose active=0 |
| Resource Host | Fake Provider describe vertical slice | 真实 storage/provider | authority 在 Host 内构造；F0.2 counters 归零 |

## 4. API 与时序

```text
UI entry / Agent Tool
  → requestPresentation(session, request, source)
      → append muse/presentation-requested
      → OpenResourceService.request
          → Host Provider discover(family=muse.resource-presentation)
          → exact input/output schema digest check
          → bind(scope hints)
          → invoke(resource.presentation.request)
      → append muse/presentation-terminal
      → UI receipt / model-safe Tool projection
```

Service Definition：

```ts
request(requestV2, source): Promise<PresentationReceiptV2>
status(requestRef, source): Promise<PresentationStatus>
cancel(requestRef, source): Promise<PresentationCancelResult>
```

Transport `requestId` 与语义 `requestRef` 严格分离。`requestRef` 是 Presentation 幂等键；
每次 Host Bridge unary request 仍由 transport 生成独立 requestId。

## 5. Durable / model-visible 语义

| Event | 是否进入模型历史 | DSH envelope | 目的/缺失时行为 |
|---|---|---|---|
| `muse/resource-discovered` | 否 | `ignorable:true` | recent-ref 授权证据；缺失时 Agent fail-closed |
| `muse/presentation-requested` | 否 | `ignorable:true` | side effect 前的意图审计/恢复；缺失时不恢复 Muse 卡片 |
| `muse/presentation-terminal` | 否 | `ignorable:true` | 终态恢复、UI 重放；缺失时不恢复 Muse 卡片 |
| core `tool/call` | 是 | required | 模型原始调用 |
| core `tool/result` | 是 | required | Tool 的安全投影 |

“model-visible”不通过自定义事件伪造；真实 Agent loop 仍由 DSH Tool Runtime 把返回值写入
标准 `tool/result`。测试用相同 Session surface 规则验证 seed replay 后 messages 相等。

Muse 三类事件均为非 Surface 信息记录，不改变 DSH model history、`request/header` 或
`request/context` fold，因此通过上游 `Session.append(..., { ignorable: true })` 显式写入。
完整 Muse 组合可重放；未安装 Muse 插件的同版本 DSH 读取器可安全跳过，避免 Session reload
因仓库外事件类型而失败。Surface event 携带该标记会在类型层和运行时被拒绝。

## 6. 安全与信任

- UI/Agent DTO 没有 path、URL、engineId、handle、credential、`canRead/canWrite`。
- session/workspace/actor 是 scope hint，不是 capability grant；非 opaque 本地值先哈希投影，不上 wire。
- Host 从 authenticated connection + binding + policy 构造 `ResourceAuthority`。
- Agent resourceRef 必须来自同 Session 的 durable discovery evidence。
- Provider receipt 需通过 Presentation v2 schema 且 requestRef 必须匹配。
- Provider 异常转为 `PRESENTATION_PROVIDER_FAILED` terminal receipt；不把 stack 写入模型或事件。

## 7. 性能与容量指标

| 指标 | F0.3 Gate |
|---|---:|
| warm Provider 请求额外 DSH seam P95（不含 Host/engine） | ≤ 10 ms |
| recent evidence 查找上界 | 256 events（默认） |
| discover 单页 | ≤ Host Bridge `maxDiscoverDescriptors` |
| request/result payload | ≤ negotiated Host Bridge input/output limits |
| duplicate requestRef Host invoke | 1 次 |
| Provider terminal receipt cache | 默认最多 1024 项 |
| reload replay Host invoke | 0 次 |
| dispose 后 active operation | 0 |
| secret/path 静态与结果扫描 | 0 命中 |

微基准和 1000 次 soak 统一在 F0.6 执行；F0.3 先固定可测 seam 与 counters。

## 8. Ontology 预埋但不实现

保留 resourceRef、revision、anchorHint、cause、trace 与事实事件，另有类型级
`ReservedPresentationExtensionPoint`。本阶段没有 Ontology Service、存储、ObjectType、关系、
Action、Impact 或 Workflow；不存在时全部测试必须通过。
