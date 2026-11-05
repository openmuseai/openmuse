# Surface 路由与引擎适配方案

## 1. 职责边界

统一打开需要四种角色，不能再用一个 `TargetRouter` 同时承担格式、权限、内容和 UI：

| 角色 | 负责 | 不负责 |
|---|---|---|
| Resource Provider | 签发 ref、describe、read/materialize、revision、patch、event | 选择 UI 或引擎 |
| Format Provider | 识别格式、声明语义 profile 与 anchor 类型 | 打开 Surface |
| Surface Adapter | 声明接受能力、平台、交付模式；创建/聚焦 Surface | 解释 Host 私有 locator |
| Surface Orchestrator | 收集候选、应用策略、物化、打开、回执、fallback | 维护格式白名单、实现具体引擎 |

第三方引擎本体保持 vendor：ioffice 仍是 Flutter/Rust Office 引擎，Helix 仍是 TUI 编辑器，open-file-viewer 仍是 Web SDK。Muse 插件 Facet 才是协议参与者。

## 2. Surface Adapter 合同

```ts
interface SurfaceAdapterV1 {
  readonly adapterId: string;
  readonly engineId: string;
  readonly surfaceKinds: readonly string[];
  readonly intents: readonly ("view" | "edit" | "annotate" | "diff")[];
  readonly deliveryModes: readonly MaterializationMode[];
  readonly platforms: readonly ("desktop" | "web" | "mobile")[];
  readonly placements: readonly HostPlacement[];
  readonly requiresCapabilities: readonly string[];
  readonly providesCommands: readonly string[];
  score(input: SurfaceMatchInput): SurfaceMatchResult;
  open(input: SurfaceOpenInput): Promise<SurfaceReceipt>;
  focus?(surfaceRef: string, anchorRef?: string): Promise<SurfaceReceipt>;
  close(surfaceRef: string): Promise<void>;
}
```

`score()` 是纯函数，只使用 Descriptor、请求 disposition、平台能力、placement、策略与偏好。它不得自行读取文件或产生副作用。

返回：

```json
{
  "supported": true,
  "score": 860,
  "reasons": ["FORMAT_PROFILE", "EDIT_CAPABLE", "LOCAL_ENGINE"],
  "deliveryMode": "host-path",
  "effectiveDisposition": "edit",
  "warnings": []
}
```

## 3. 路由算法

### 3.1 输入

- `resourceRef` 与当前 Descriptor revision；
- 用户意图：view/edit/annotate/diff；
- 手势：默认点击、显式“打开方式”、Agent 建议、Workflow；
- `HostDescriptorV2`：platform、runtime、grants；
- placement：本机 embedded、用户远端桌面、租户云 Host；
- 活跃 Surface inventory；
- 组织策略、用户偏好、大小/网络/离线状态；
- Adapter 注册表。

### 3.2 决策顺序

```text
authorize resourceRef
 → describe effective capabilities
 → 已存在相同 resource+disposition Surface？是：focus/reveal
 → 过滤 platform / placement / grant / disposition / delivery
 → 各 Adapter score
 → 应用强制组织策略
 → 应用用户显式偏好（只在可用候选内）
 → 确定候选和 fallback ladder
 → materialize for selected Adapter
 → open + receipt
 → 失败时按可重试性选择下一候选
```

### 3.3 分数建议

核心只定义维度，不定义格式：

| 维度 | 建议权重 |
|---|---:|
| 满足请求 disposition | +300 |
| Format Provider 的精确 profile 匹配 | +250 |
| 真源原生 Surface（无需转换） | +150 |
| 本地可用且无内容跨设备 | +100 |
| 用户对该 format/profile 的偏好 | +120 |
| 已有 Surface 可聚焦 | +200 |
| 只能降级 view/metadata | -100 / -250 |
| 需要外部转换或上传 | -400，且默认策略可禁止 |
| 资源过大、内存风险、网络质量差 | Adapter 自报惩罚 |

同分时按组织策略 priority、Adapter version、稳定 `adapterId` 排序，确保可重复。

### 3.4 决策记录

每次打开写入：

```json
{
  "decisionRef": "route.01J...",
  "resourceRef": "resource.01J...",
  "revision": "etag...",
  "requestedDisposition": "edit",
  "selectedAdapter": "muse.surface.ioffice-word",
  "effectiveDisposition": "edit",
  "materializationMode": "host-path",
  "candidateReasons": ["..."],
  "preferenceRef": "preference.01J...",
  "placement": "local-embedded",
  "decidedAt": 1789372800000
}
```

UI 的“为什么用此应用打开”直接读取该记录，而不是重新解释规则。

## 4. Surface Intent v2

```json
{
  "protocol": "muse.presentation-intent/v2",
  "intentRef": "intent.01J...",
  "intentType": "surface.open",
  "resourceRef": "resource.01J...",
  "expectedRevision": "etag...",
  "disposition": "edit",
  "anchorRef": "anchor.01J...",
  "targetSurfaceInstanceRef": null,
  "scopeRef": "scope.01J...",
  "routeDecisionRef": "route.01J...",
  "leaseRef": "lease.01J...",
  "epochRef": "epoch.42",
  "requestedAt": 1789372800000,
  "expiresAt": 1789372830000
}
```

回执：

```json
{
  "protocol": "muse.presentation-intent-result/v2",
  "intentRef": "intent.01J...",
  "status": "opened",
  "surfaceInstanceRef": "surface.01J...",
  "adapterId": "muse.surface.ioffice-word",
  "effectiveDisposition": "edit",
  "observedRevision": "etag...",
  "reasonCode": "OPENED_NEW_SURFACE",
  "completedAt": 1789372800123
}
```

终态状态至少包括 `opened / focused / fallback / rejected / stale / not-found / unsupported / conflict / timed-out / failed`。一个 `intentRef` 恰好一个终态；中间 loading phase 走 progress event，不冒充结果。

## 5. DSH 交付物接入的正确路径

### 5.1 当前问题

- ProducedFiles、正文文件 mention、交付卡片默认点击直接调用 Chat `openFile(path)`，只打开 DSH Sidebar。
- `/api/present.open` 只服务显式原生菜单。
- `SessionControllerInternals.openPath` 不是运行时插件 seam，且覆盖不了默认点击。

### 5.2 目标改造

在 DSH Client/Host 引入完整的 Resource Presentation capability seam：

```text
Service Definition
  resourcePresentation.describeCoordinates()
  resourcePresentation.open()
  resourcePresentation.listAdapters()

Providers
  default-sidebar-provider
  default-native-provider
  muse-host-provider

Consumers
  ProducedFiles
  inline file mentions
  PresentedFileCard default action
  Open With menu
  tool cards / future knowledge references
```

要求：

1. Service Definition 不包含 AppFlowy/Flutter/ioffice 字样；
2. 默认 Provider 保持现有 Sidebar 行为，确保上游独立可用；
3. Muse Host Provider 用 `(sessionId, seq, index)` 或已验证 file address 换取 `resourceRef`；
4. Client 默认点击也调用 seam，不再直接 `openFile(path)`；
5. 原生菜单通过同一 seam 指定 disposition/adapter preference，而不是旁路；
6. 注册、事件和 UI contribution 都由 `ctx.effect()` 对称释放；
7. 需要同时更新 DSH Client 与 Host 的真实组合测试和 keyless session snapshot。

如果短期不能修改上游 seam，过渡方案是在 Muse client-half 中替换 Deliverables contribution，并保留默认 Sidebar provider。不得宣称仅替换 opener 已完成统一。

## 6. Desktop / Web / Mobile 传输

| 端 | Resource control plane | Surface intent downlink | 数据面 |
|---|---|---|---|
| Desktop | 现有 DSH↔Rust Host Bridge UDS | Rust Host Event/FFI → Flutter Surface Runtime | host-path、UDS stream、loopback URL |
| Web | Host Bridge Cloud/InProcess + HTTPS Adapter | SSE/postMessage Adapter → Web Surface Registry | range/stream/同源短期 URL |
| Mobile | HTTPS Host Bridge Adapter | SSE → `MuseParentBridgeAdapter` → Surface Runtime | range/stream/短期 URL；严禁整文件驻内存 |

Desktop 不应为了复用 Web parent-bridge 而开启 Cloud URL。目标是合同一致，transport 可以不同。

## 7. 引擎适配

### 7.1 ioffice Word

当前可用基础：macOS arm64 dylib、`word_render`、`word_editor`、AppFlowy `Word` layout、blob store、Word Surface、Word snapshot Provider。

新增 Facet：

- Domain：`muse.resource@1` Provider，把 AppFlowy Word view/blob 映射为 opaque ref；
- Presentation：`muse.surface.ioffice-word`，接受 `ooxml.word` semantic profile；
- Agent：继续使用 Word/Document 领域合同，不让 Resource Core 理解 OOXML；
- Anchor：页、段落、run/selection 的稳定选择器与 fallback fingerprint。

打开模式优先 `host-path` 或 Host 内 byte source；保存走 Word Domain Provider，不能把编辑后的 OOXML 作为无版本裸文件覆盖。

### 7.2 ioffice Excel / Slides / PDF

当前只有 Manifest、ViewLayout、blob 与占位页，不是已完成引擎。接入分两步：

1. 先作为 Resource + open-file-viewer 的只读 Surface，实现统一引用、预览和 Anchor 的最小子集；
2. 真 ioffice 引擎到位后注册更高分的 edit/annotate Adapter，路由自动升级，核心不变。

PDF 默认 disposition 为 view/annotate，不应因 Office 分类而假设可编辑。

### 7.3 Helix

Helix 是需要真 PTY 的 TUI，不是 Flutter library。推荐：

- DSH sidecar 注册 `type: helix` terminal backend；
- Host 校验并物化 same-host path；
- Flutter `code.terminal` Surface 使用终端渲染器连接受认证 WS；
- Surface 获得单写者 lease，保存由文件 Resource Provider 观察并发布 revision；
- Mobile/Web 只能在其连接的 Host 上运行远程 PTY，UI 必须显示 Host 归属和延迟。

Helix v1 无需 LSP/DAP；语法 grammar、runtime、签名和平台二进制必须进入打包验证。详细引擎约束仍参考 [helix-editor-integration.md](../helix-editor-integration.md)，但打开入口以本文件的 Resource/Surface 合同为准。

三个 vendor 的阶段级实施、开发任务与测试矩阵见 [engine-delivery/README.md](engine-delivery/README.md)。该专项方案以本文件为上位约束，并明确将 Ontology Runtime 延后。

### 7.4 open-file-viewer

它已经有 `PreviewPlugin.match/render/destroy`，适合作为一个受控 Web Surface，不适合作为 Resource Authority。

接入要求：

- Source 只能来自 `byte-range / stream / loopback-url / external-url` handle；
- WebView/iframe 使用独立 origin、严格 CSP、隔离 storage namespace；
- 插件按需加载，destroy 时释放 object URL、worker、WebGL、listener；
- `fallbackPlugin` 始终最后，不参与“原生支持”评分；
- Office server conversion 默认关闭；任何上传/转换是独立 external-side-effect Action；
- 大文件按格式设置 range 和内存预算，Mobile 不创建全量 ArrayBuffer；
- Viewer 的格式匹配结果可贡献评分，但最终格式事实归 Format Provider。

### 7.5 Markdown

Markdown 是验证“无格式特权”的第一项 TCK：

- AppFlowy Markdown Surface、DSH Sidebar Text Surface、Helix Surface 都可以声明接受它；
- 路由根据 disposition 和 placement 选择，而不是 `if extension == md`；
- AppFlowy 文档的协同编辑优先原生 Markdown Surface；工作区 `.md` 的代码编辑可优先 Helix；只读 Web 预览可选 Viewer/Sidebar；
- 三个候选都使用同一个 Descriptor、Intent、Receipt 与 audit。

## 8. Anchor 能力

跨格式统一的是 Anchor envelope，不是 locator 内容：

```json
{
  "anchorRef": "anchor.01J...",
  "resourceRef": "resource.01J...",
  "baseRevision": "etag.r7",
  "anchorKind": "document.block-range",
  "selectorSchemaDigest": "sha256:...",
  "selector": { "blockRef": "opaque", "start": 12, "end": 48 },
  "quoteFingerprint": "sha256:..."
}
```

格式插件拥有 selector schema：Markdown block、Word paragraph、Excel range、代码 symbol、PDF page rectangle、CAD entity、3D node 各不相同。核心只负责 revision、ownership、TTL/持久性和 resolve 状态。

## 9. 生命周期与并发

- 同一 `resourceRef + effectiveDisposition + hostRef` 默认复用一个 Surface；重复打开为 focus。
- 写 Surface 需要单写者 lease；第二个请求可聚焦、以只读打开、等待或显式接管。
- Surface close 释放 materialization、写 lease、context contribution 和 renderer resources。
- Host epoch 变化后旧 Intent、Receipt、stream frame 和 Surface snapshot 全部拒绝。
- 跨设备不会假装聚焦另一个设备上的窗口；回执应为 `active-on-other-host`，再由用户选择接管或只读。

## 10. 能力降级

统一 ladder：

```text
edit
  └─不支持→ annotate
       └─不支持→ view
            └─不支持→ metadata
                 └─不支持→ download / system-open（仅合法 placement）
                      └─不支持→ unsupported receipt
```

降级必须对用户可见并记录原因。请求 edit 而实际只读时，Surface 标题区持续显示“只读”；不能悄悄吞掉保存。

## 11. Adapter 准入清单

- Manifest 声明 runtime/platform/grant/contract/contribution。
- score 是纯函数且有负例测试。
- delivery mode 与真实数据访问一致。
- 不使用 extension 作为唯一高置信判断。
- 打开、聚焦、关闭、超时、取消、卸载均有测试。
- 资源/Surface/Host 归属始终可见。
- 内容、路径、token 不进入日志。
- 新 Adapter 不修改 Resource Core。
- 至少一个真实组合测试，不只手工 new fake Context。
- 端到端快照覆盖产品可见卡片、错误与 fallback。
