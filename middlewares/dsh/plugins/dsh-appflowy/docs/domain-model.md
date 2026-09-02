# @muse/dsh-appflowy 领域泳道图（AppFlowy-Web × Muse Plugins × AppFlowy Host）

> 全平台命名、两个 Host 的区分、数据权威与分端泳道：[`docs/platforms/DOMAIN_MAP.zh-CN.md`](../../../docs/platforms/DOMAIN_MAP.zh-CN.md)。  
> 下文是 **Web 入站包** 的细化泳道（6 条实现泳道），不是第二套领域划分。

> 自上而下的泳道图：6 条泳道（参与方），节点按时间阶段从上往下排，箭头标注接口交互。
> 起点：应用启动 —— A 泳道「AppFlowy-Web 应用启动」与 D/E 泳道「DSH sidecar 装配」并行发生。
> 关联文档：[web-workspace-ingress.md](./web-workspace-ingress.md)（Web 入站链路 §13–§16）、
> [cordis-patch-topology.md](./cordis-patch-topology.md)（插桩拓扑）。最后更新 2026-08-28。

## 1. 泳道图

```mermaid
flowchart TD
    %% ============ 泳道 A：AppFlowy-Web（父窗口） ============
    subgraph W["A · AppFlowy-Web（父窗口，React）"]
        direction TB
        A1["① 应用启动<br/>DshAgentProvider 挂载 DshAgentPanel<br/>localStorage 记忆 open/width"]
        A7["① 状态源<br/>AuthInternalContext · AppNavigationContext · AppOutlineContext"]
        A2["① 面板打开<br/>?dsh=1 / 用户点击 → probeDshOrigin(url)"]
        A3["① 设备身份<br/>fetchDshDeviceToken（首次打开触发）"]
        A4["② iframe 装载<br/>监听 frame-ready / intent.dispatch / selectionchange / storage"]
        A5["②③ 消息构造与上行<br/>parent-hello · workspace.bind<br/>context.contribute · surface.closed · intent.receipt"]
        A6["④ 意图执行<br/>applyPresentationIntent → toView(viewId, blockId)"]
    end

    %% ============ 泳道 B：DSH Web UI（iframe） ============
    subgraph I["B · DSH Web UI（iframe，注入脚本）"]
        direction TB
        B1["② 桥接脚本<br/>过滤 source=muse.appflowy-web → POST /muse/v1/parent-bridge"]
        B2["④ 下行通道<br/>EventSource /muse/v1/parent-bridge/events<br/>→ parentWin.postMessage(parsed)"]
        B3["② 页面装载完成 → postMessage frame-ready"]
    end

    %% ============ 泳道 C：DSH 运行时 · Web 会话面 ============
    subgraph R1["C · DSH 运行时 · Web 会话面（parent-bridge / workspace / context-broker）"]
        direction TB
        C1["② 入站端点<br/>POST /muse/v1/parent-bridge<br/>校验：≤32KB · JSON · 禁字段 · source"]
        C2["② 消息分发<br/>parent-hello → rememberDeviceAuth<br/>workspace.bind → bindWorkspace<br/>context.contribute → broker<br/>surface.closed → removeSurface<br/>intent.receipt → ok"]
        C3["② 工作区绑定<br/>applyWorkspaceHint<br/>mkdir → README → resolveByPath<br/>→ setTitle / create → 置顶 → pin"]
        C4["② DSH workspace 注册表<br/>agent 会话 cwd · 防删除（AppFlowyWorkspacePinnedError）"]
        C5["③ 上下文代理<br/>ingestContribution → 投影 render<br/>（focus 110 / tree 40 / markdown 100/90/50）<br/>→ system prompt"]
        C6["④ 意图入队<br/>enqueuePresentationIntent<br/>30s 过期 · SCOPE 校验 · SSE 广播"]
        C8["⑤ 桌面 hint 监听<br/>watchAppFlowyWorkspaceHint（50ms debounce）"]
        C7["③ 列表查询<br/>GET /muse/v1/workspace/views<br/>NO_WORKSPACE/SCOPE 校验 → 转 Cloud"]
    end

    %% ============ 泳道 D：DSH 运行时 · Muse 会话面 ============
    subgraph R2["D · DSH 运行时 · Muse 会话面（connector / host-bridge / plugin-kit）"]
        direction TB
        D1["① host-bridge 装配<br/>autoStart → connect() → hello 握手<br/>→ subscribe 事件泵（1s 重连）"]
        D2["① connector.open()<br/>Cloud URL 已设 → 进程内<br/>否则读 launch descriptor（0600）→ UDS"]
        D3["① 契约发现<br/>discover.request（families+scopeHint）<br/>→ adapter.accepts（sha256 digest 比对）<br/>→ bind.request → 工具 publish（300s 到期整代刷新）"]
        D4["③ 工具调用链<br/>参数 schema 校验 → input 校验<br/>→ policy?（apply 类）→ invoke.request<br/>（traceId / grantId? / idempotencyKey?）"]
        D5["③ 云转发补全<br/>cloudDocumentPayload（补 lastDocumentFocus/bound）<br/>→ assertWorkspaceScope → 鉴权（getLastDeviceAuth）"]
    end

    %% ============ 泳道 E：AppFlowy Core（桌面 Host） ============
    subgraph CORE["E · AppFlowy Core（桌面 Host，Rust）"]
        direction TB
        E1["① MuseRuntime 启动<br/>UDS socket + launch/approval 0600 文件<br/>hostGeneration=appflowy.local.1"]
        E2["① 能力注册<br/>appflowy.document.local（rev3）<br/>appflowy.view-reference.local<br/>appflowy.view-rename.local"]
        E3["② 权威解析<br/>resolve/revalidate<br/>UserManager · FolderManager · ServerProvider<br/>→ actor / scope / epoch / evidence"]
        E4["③ 策略与审批<br/>read→allow；write→approval_required<br/>finalize 校验 HMAC proof → grant（60s 一次性）"]
        E5["③ Provider 落库<br/>document（collab/markdown）· view-reference · view-rename"]
        E6["③ 桌面上下文发布<br/>MuseUiContextPublisher → HostEventHub<br/>→ provider.event（context.updated / surface.closed）"]
        E7["⑤ Flutter 壳在切换工作区时<br/>写 hint 文件"]
    end

    %% ============ 泳道 F：AppFlowy-Cloud ============
    subgraph CLOUD["F · AppFlowy-Cloud（Rust，HTTP）"]
        direction TB
        F1["① 设备身份<br/>POST /api/muse/dsh/device-token<br/>（verify / revoke · HMAC 两段式）"]
        F2["③ 文档操作<br/>POST /api/muse/document/{query|propose|apply|status}<br/>成员校验 · apply 门控后写 collab"]
        F3["③ 工作区操作<br/>POST /api/muse/workspace/{current|tree}<br/>pg 成员校验 · folder collab 有界投影"]
        F4["③ 存储服务<br/>pg（workspace 成员）· collab（文档）<br/>redis（proposal / token 吊销 / TTL）"]
    end

    %% ============ 接口交互（箭头 = 接口调用） ============
    A1 --> A2
    A2 --> A3
    A3 -->|"POST /api/muse/dsh/device-token（账号 Bearer）"| F1
    F1 -->|"签发 {token, expiresAt, deviceId, kid}"| A3
    A1 --> A7
    A7 -->|"派生 workspaceId / title / viewId / outline"| A5
    A2 -->|"iframe src = DSH URL"| B3
    B3 -->|"postMessage frame-ready"| A4
    A4 -->|"收到 frame-ready / deviceToken 就绪 → 回 hello"| A5
    A5 -->|"postMessage（source=muse.appflowy-web）"| B1
    B1 -->|"POST /muse/v1/parent-bridge（原样 JSON，≤32KB）"| C1
    C1 -->|"handleParentInbound 校验通过"| C2
    C2 -->|"workspace.bind / parent-hello"| C3
    C3 -->|"applyWorkspaceHint：mkdir→README→resolveByPath→setTitle/置顶→pin"| C4
    C2 -->|"context.contribute → ingestContribution + pinSurface + rememberDocumentFocus"| C5
    C2 -->|"surface.closed → removeSurface"| C5
    C7 -->|"invokeCloudWorkspace → POST /api/muse/workspace/tree"| F3
    D1 -->|"museHostConnector.open() → transport + proof"| D2
    D2 -->|"读 launch descriptor（0600）→ UDS connect → hello（nonce/proof）"| E1
    E1 -->|"provider 注册（descriptor/revision/schemas）"| E2
    E2 -->|"discover / bind / invoke 前 resolve·revalidate"| E3
    E2 -->|"policy.evaluate → finalize(proof) → grant"| E4
    E4 -->|"auth（grant 一次性消耗）→ provider.invoke 落库"| E5
    D2 -->|"Cloud 模式：InProcess transport + AppFlowyCompositionHandler"| D3
    D1 -->|"WireEnvelope：discover.request / bind.request / subscribe"| D3
    D3 -->|"bind.response → tools 发布（muse_document_* 等）"| D4
    D4 -->|"invoke.request / policy.*（经 MuseHostService）"| D1
    D3 -->|"Cloud 模式：invoke 走进程内 handler 补全"| D5
    D5 -->|"POST /api/muse/document/{query|propose|apply|status}（Bearer deviceToken + X-Muse-Device-Id）"| F2
    D5 -->|"POST /api/muse/workspace/{current|tree}"| F3
    F2 -->|"collab 快照 / redis 提案 / 门控后写 collab"| F4
    F3 -->|"pg 成员 + folder collab"| F4
    D4 -->|"muse_appflowy_present 工具 → buildPresentationIntent"| C6
    C6 -->|"SSE data: {source:muse.dsh-web, intent.dispatch}"| B2
    B2 -->|"postMessage intent.dispatch"| A4
    A4 -->|"parseIntentDispatch → applyPresentationIntent"| A6
    A6 -->|"构造 receipt（applied/rejected/...）"| A5
    A5 -->|"postMessage intent.receipt"| B1
    E6 -->|"bridge.event（provider.event / context.updated）"| D1
    D1 -->|"museHost/event → broker.ingest"| C5
    E7 -->|"写入 $DSH_HOME/bindings/current-appflowy-workspace.json"| C8
    C8 -->|"applyHintFile → applyWorkspaceHint"| C3

    %% ============ 泳道着色 ============
    classDef web fill:#e3f2fd,stroke:#1565c0,stroke-width:1px
    classDef iframe fill:#fff3e0,stroke:#e65100,stroke-width:1px
    classDef dsh fill:#e8f5e9,stroke:#2e7d32,stroke-width:1px
    classDef muse fill:#f3e5f5,stroke:#6a1b9a,stroke-width:1px
    classDef core fill:#ffebee,stroke:#b71c1c,stroke-width:1px
    classDef cloud fill:#ede7f6,stroke:#4527a0,stroke-width:1px
    class A1,A2,A3,A4,A5,A6,A7 web
    class B1,B2,B3 iframe
    class C1,C2,C3,C4,C5,C6,C7,C8 dsh
    class D1,D2,D3,D4,D5 muse
    class E1,E2,E3,E4,E5,E6,E7 core
    class F1,F2,F3,F4 cloud
```

## 2. 泳道说明

| 泳道 | 参与方 | 扮演角色 |
|---|---|---|
| A | AppFlowy-Web 父窗口 | 状态源 + 消息构造 + 意图执行（**不直接触碰 Host**） |
| B | DSH Web UI iframe | 无业务逻辑的透明桥：postMessage ↔ HTTP、SSE ↔ postMessage |
| C | DSH 运行时·Web 会话面 | 入站校验 / 绑定落盘 / 上下文投影 / 意图入队 |
| D | DSH 运行时·Muse 会话面 | 契约发现与工具发布、Bridge 协议客户端、云转发补全 |
| E | AppFlowy Core（桌面 Host） | 权威、策略、provider 落库、UI 上下文发布 |
| F | AppFlowy-Cloud | 设备 token、文档/工作区 Cloud 操作、存储 |

## 3. 时间阶段

- **① 启动（并行）**：A 泳道 Web 应用挂载面板并换取设备身份（直接调 Cloud，绕过 DSH）；D/E 泳道 DSH sidecar 装配，host-bridge 握手，connector 决定 UDS/Cloud 双模式，plugin-kit 完成 discover→bind→工具发布。桌面侧 Core 同时建 UDS 服务并注册 3 个 provider。
- **② Web⇄DSH 握手与绑定**：`frame-ready` → `parent-hello`（带设备 token 与 workspace）→ 注入脚本转 HTTP → `rememberDeviceAuth` + `bindWorkspace` → workspace 注册表 pin 落盘。
- **③ 稳态上行与工具调用**：4 类上下文 envelope 周期性上行进 broker 投影；agent 工具经 plugin-kit → host-bridge →（桌面 UDS / 云 HTTP）→ Host；写操作夹带审批链（evaluate → HMAC proof → finalize → grant）。
- **④ 意图下行**：`muse_appflowy_present` → SSE → iframe → `postMessage` → `applyPresentationIntent` → `toView()` → receipt 回传。
- **⑤ 运行中切换**：Web 换工作区重发 `workspace.bind`（先 `surface.closed`）；桌面换工作区由 Flutter 壳写 hint 文件驱动重绑。

## 4. 交互要点

1. **设备 token 是 Web 直连 Cloud 获取**（`POST /api/muse/dsh/device-token`），不经 DSH；DSH 侧只在 `parent-hello` 的 `rememberDeviceAuth` 处记忆，供 Cloud 转发鉴权。
2. **Cloud 路径的审批链是过渡形态**：`AppFlowyCompositionHandler` 对 `policy.evaluate/finalize` 返回固定 e2e 响应（`policy.e2e`/`grant.e2e`，不做真实 proof 校验），真实审批语义只在桌面 Core Host；Cloud 写操作依赖两端门控（DSH `MUSE_DOCUMENT_CLOUD_APPLY_ENABLED` + Cloud `muse_document_cloud_apply_enabled`）兜底。
3. **同一份上下文 envelope 双路汇合**：Web 走 `ingestContribution`（伪造 `subscription.parent-bridge` 的 provider.event），桌面走真实 `provider.event`，在 broker 汇合后按同一套 digest/priority/TTL/去重规则渲染进 system prompt。