# 安全、治理与可观测性

状态：**目标设计 v1**；延续当前 Host 作为 authority、approval digest 与 Host Bridge policy 的实现原则。

## 1. 安全目标

系统必须在“任意资源 + Agent + 插件 + 自动化”的组合下保证：

- 资源引用不可被伪造为任意本地路径或外部系统 ID；
- 读取遵循最小权限，检索、关系和模型上下文不会旁路泄露；
- 写入具备提议、审批、并发控制、幂等、审计和明确失败语义；
- 插件和渲染器的权限与生命周期有界；
- 人可以追溯每项语义、建议和动作的来源；
- Desktop、Web、Mobile 的传输差异不改变授权语义。

## 2. 信任边界

```text
Untrusted / lower trust
  Document content · external URLs · model output · plugin UI · Web renderer
          │ validated contracts / scoped handles
          ▼
Muse Runtime
  Surface Orchestrator · Context Broker · Ontology · Workflow
          │ authenticated Host Bridge invocation
          ▼
Authority
  Host identity/policy · workspace provider · filesystem · AppFlowy · Git · SaaS
```

Host/业务真源是最终 authority。DSH、Agent、Surface 和插件即使运行在同一进程也不能被视为天然可信。

## 3. 身份、授权与引用

每次 capability bind/invoke 至少绑定：

- actor/agent identity 与会话；
- workspace/tenant；
- Host placement 与设备；
- plugin/provider identity 和版本；
- capability、scope、有效期与调用预算；
- policy revision。

`resourceRef`、`objectRef`、`anchorRef` 均是不透明 typed handle。Host 解引用后重新校验 workspace、actor、资源存在性和操作 scope；不得仅因引用格式合法就授权。引用可设置有效期、撤销和 audience，日志使用摘要或安全别名，避免暴露内部路径。

## 4. 权限模型

权限求值同时考虑：

1. Workspace membership 与角色。
2. Object/Resource ACL 或来源系统权限。
3. 属性/字段/Anchor 数据分类。
4. ActionType 的 submission criteria 和 writeSet。
5. 设备、网络区、时间、数据驻留等上下文策略。
6. Agent/plugin 是否被授予相应 capability。

关系不能放大权限：能查看 Requirement 不代表能看到其 confidential CodeSymbol；邻接查询只返回可见节点和经过安全处理的计数。删除或隐藏节点后，也不能由缓存、向量索引、通知标题和审计 UI 泄露。

## 5. Action 与审批安全

- proposal 不授予执行权；apply 时必须重新鉴权和验证 revision。
- approval token 绑定 proposal digest、actor、workspace、允许 effect、有效期和 policy revision。
- proposal 内容、writeSet 或 base revision 改变会使旧审批失效。
- 自批默认禁止；请求者、审批者和执行主体按风险策略分离。
- 高风险动作支持双人/角色审批、时间窗和 step-up authentication。
- capability receipt 保存调用目标的安全 ID、结果摘要和新 revision，不保存密钥。
- UI/Agent 不能合成 approval token；当前 Rust Host 的服务端重校验模式应推广到全部 Provider。

## 6. Materialization 与文件安全

资源内容只能通过 Provider 颁发的短期 materialization handle 提供给引擎：

- 临时文件使用任务专属随机目录、最小权限和固定生命周期；
- 文件名来自安全展示名，不能参与路径拼接或目录穿越；
- materialization handle 绑定 provider、resource、revision、mode、consumer 和过期时间；
- engine 只得到所需形态：路径、只读 fd、byte-range、stream 或 loopback URL；
- 回写不监视任意目录，而通过明确 commit/proposal 接口；
- 关闭/崩溃/撤销时清理句柄；机密内容按策略不落盘或加密缓存；
- system-open 属于外部副作用，需明确用户手势与审计。

大文件/压缩包先校验类型、大小、解压配额、递归深度和解压后总量，防止 zip bomb。MIME、扩展名、magic bytes 和 Provider 元数据冲突时按最保守模式处理。

## 7. Web Surface 与内容隔离

`open-file-viewer` 及未来 Web renderer 默认运行在隔离 WebView/iframe：

- 严格 CSP；禁止任意远端脚本、顶层导航和未经许可的网络访问；
- sandbox 权限按 adapter 声明最小化，不开放 Node/Host 对象；
- loopback URL 使用随机 bearer、origin 校验、短 TTL、单资源 scope；
- postMessage 使用固定 origin、版本化 schema、消息大小限制和 nonce；
- 文档中的链接、宏、嵌入对象、脚本均视为不可信；
- clipboard、打印、下载、摄像头等需单独 capability/用户手势。

ioffice 宏执行默认关闭；若未来支持，必须作为独立高风险 capability，而不是“可编辑”的隐含权限。

## 8. Prompt injection 与 Agent 数据安全

文档正文和外部页面是数据，不是系统指令。Context Broker 应：

- 以结构化边界标记来源与不可信级别；
- 不把文档中“调用工具/泄露数据”等内容提升为策略；
- 工具调用只接受 schema 校验后的 typed refs，不接受正文拼出的路径/URL/命令；
- 对跨域检索和外发动作执行 data-loss policy；
- 对敏感字段做模型可见性检查、遮蔽或本地推理路由；
- 记录使用了哪些 Anchor 与 model/provider/version，但审计摘要避免复制全部敏感正文。

Agent 输出始终是 untrusted proposal，直到由确定性校验和授权 Runtime 接收。

## 9. 插件供应链

Manifest/Plugin Graph v2 应扩展声明：

- publisher、签名、包 digest、来源与版本；
- contracts/protocols/facets、resource/format/surface contributions；
- 需要的 Host capabilities、数据分类上限、网络域和外部进程；
- supported placements/platforms 与 minimum Host version；
- 生命周期、健康检查、资源预算和卸载行为。

治理要求：签名和 digest 校验、允许列表/禁用开关、版本固定与回滚、依赖/许可证/SBOM、隔离安装、权限差异审阅。插件失效只能使对应能力降级，不能阻断 Workspace 基础导航。

## 10. Ontology 治理

### 10.1 Schema

ObjectType、LinkType、Interface、ActionType 和 WorkflowDefinition 都有 owner、版本、状态、数据分类、保留策略与变更记录。生产 schema 修改需 proposal、兼容性检查、fixture、迁移预览、审批和可回滚发布。

### 10.2 Provenance

Object 属性、Link 和 Claim 记录来源：人工、规则、导入、Agent；具体 Resource/Anchor/revision；生成工具或模型版本；确认者。聚合/推导值还记录 rule/function version 和输入摘要。

### 10.3 数据生命周期

- 删除真实资源时，Anchor 标记 orphaned；是否保留摘要遵守来源系统和 workspace policy。
- 纠正事实用 supersede，不静默改写历史。
- 用户删除、法律保留、导出和 workspace 销毁需覆盖事件日志、投影、索引、缓存与备份策略。
- 训练/评估用途与生产运行数据分开授权，默认不跨 workspace 复用内容。

## 11. 审计事件

统一审计 envelope：

```json
{
  "eventId": "evt_01J...",
  "time": "2026-09-14T12:00:00Z",
  "traceId": "trc_...",
  "actorRef": "actor:opaque",
  "workspaceRef": "ws:opaque",
  "operation": "action.apply",
  "targetRefs": ["ont:requirement/r-42"],
  "decision": "allowed",
  "policyRevision": "pol_17",
  "requestDigest": "sha256:...",
  "result": "revision-conflict",
  "providerReceiptRef": "receipt:opaque"
}
```

必须审计：引用签发/撤销、敏感读取、搜索/导出、Surface 外部打开、Claim 确认、schema 变更、Action 提议/审批/执行、Workflow 启停/运行、权限与插件变更。正常光标/滚动不进入安全审计。

日志应防篡改、可检索、按租户隔离、受访问审计，并有明确保留期。正文和输入只保存必要摘要/引用；调试模式也不能默认记录密钥和完整敏感内容。

## 12. 可观测性

### 12.1 Trace

同一 causal flow 使用 `traceId`，跨越：

```text
user gesture → presentation.request → route decision → materialize → surface ready
resource.changed → extraction → changeset → impact → action → provider receipt
```

Span 属性使用低基数的 protocol/adapter/error code；真实路径、正文、对象标题不进入 metrics labels。

### 12.2 指标

| 领域 | 指标 |
|---|---|
| Resource | resolve/materialize 延迟、bytes、缓存命中、过期/撤销 |
| Surface | 路由命中、fallback、open-ready 延迟、崩溃、Anchor 定位成功率 |
| Ontology | query 延迟、orphan Anchor、stale Claim、schema 校验失败 |
| Agent | Claim 接受率、引用覆盖、工具失败、上下文成本 |
| Impact | 计算延迟、fan-out、误报/漏报反馈、处置 SLA |
| Action | approval 延迟、冲突、部分失败、补偿、幂等命中 |
| Workflow | 触发、去抖、循环截断、积压、dead-letter |

### 12.3 SLO 与告警

SLO 按本地/远端、格式、文件大小和风险分层，避免用单一平均值掩盖问题。安全告警包括异常批量读取/导出、短期大量 materialization、审批重放、跨 workspace 引用、插件 digest 变化和自动化 fan-out 激增。

## 13. 威胁与控制

| 威胁 | 主要控制 |
|---|---|
| Agent 猜测本地路径读取文件 | opaque ref、Host 解引用、workspace/actor 重校验 |
| 恶意文档提示 Agent 外传数据 | 内容不可信标记、窄工具、DLP/egress policy、审批 |
| Viewer 脚本逃逸 | Web sandbox、CSP、origin/nonce、无 Host 对象 |
| 旧审批用于新内容 | proposal digest + base revision + apply-time check |
| Ontology 关系泄露机密对象 | node/edge/field 级过滤与安全计数 |
| 自动化触发风暴 | debounce、idempotency、causal depth、预算、熔断 |
| 插件更新扩大权限 | 签名/digest、权限 diff、管理员审批、版本固定 |
| 临时文件残留 | TTL、lease cleanup、加密/禁落盘策略、启动恢复清理 |
| Anchor 漂移指向错误段落 | revision/fingerprint、多候选 review，不静默绑定 |

## 14. 安全发布门槛

- 跨 workspace、伪造 resourceRef、过期 approval、revision race 的负向测试全部通过。
- Renderer escape、路径穿越、zip bomb、恶意 MIME、prompt injection 有回归样例。
- 权限撤销传播至搜索、缓存、Surface、Ontology 和模型上下文。
- 高风险 Action 在 UI、API、自动化三个入口均不能绕过同一 policy。
- 可从用户操作追溯到 route decision、读取证据、审批、Provider receipt 和最终 revision。
- 完成威胁建模、数据保留评审、插件权限评审和故障演练后才允许扩大自动写入范围。

