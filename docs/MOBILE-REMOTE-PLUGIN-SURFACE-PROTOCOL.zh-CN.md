# Mobile 远程插件预览与控制协议

状态：建议方案 / 协议设计，尚未实现（2026-10-05）  
目标：Mobile 默认安装一个通用插件；Desktop 后装的业务插件可向它贡献预览、控制和状态，无需为每个业务插件重新构建 APK/IPA。

## 1. 决策

在 Mobile 默认发行包中安装 `com.openmuse.remote-workbench`。它只实现通用的能力发现、协议协商、资源展示、有限组件渲染、用户操作、任务状态和错误恢复。Desktop 的业务插件通过 `openmuse.remote-surface/v1` 服务贡献一个或多个 surface；DSH 可以驱动业务插件，但不拥有 Mobile 展示协议。Desktop 默认组件负责把已授权的业务 surface 接到 Paired Desktop transport。业务插件没有 Mobile Dart/原生 artifact。

```text
Mobile                                            Desktop
┌───────────────────────────────────┐             ┌──────────────────────────┐
│ remote-workbench（默认安装）         │             │ Paired Gateway + Broker   │
│ 发现 / Native 组件 / 图片 / 视频 / Web │◀══transport══▶│ Surface Registry         │
│ 动作确认 / 断线恢复                   │             │ Resource Data Plane      │
└───────────────────────────────────┘             └──────────┬───────────────┘
                                                          │ Service + grants
                                             ┌────────────┴────────────┐
                                             │ Easel Social / 视频编辑 / 其他插件 │
                                             │ 业务状态、媒体加工、命令执行 │
                                             └─────────────────────────┘
```

**可扩展性的精确含义**：新插件只使用 Mobile 已支持的组件、媒体格式和动作语义时，可在 Desktop 安装并立即从 Mobile 操作；若需要新的原生解码器、相机/系统 API、全新手势或新的协议主版本，仍要发布 Mobile 更新。业务流程和控件组合可以动态变化，Mobile 不下载并执行新的 Dart 或原生代码。Flutter 的 deferred components 仅覆盖 Android/Web，并不构成这个跨端插件机制。[Flutter 官方文档](https://docs.flutter.dev/perf/deferred-components)

## 2. 现有基础与缺口

- Manifest v2 的 `presentation.remote_capable` 只表示意图；其 `PresentationSurface` 仍是 Desktop 的 editor/sidebar/panel 枚举。Manifest v2 严格拒绝未知字段。首版不直接添加顶层字段：业务插件在 `contributes.services` 声明 `openmuse.remote-surface`、version `1`，具体 surface 目录由运行中的服务返回；以后确需静态索引时再演进 Manifest schema 与 Rust/Dart 投影。当前 Flutter `OpenMusePluginRegistry` 主要登记已构造插件的 editor/panel，尚未把 v2 service 声明接到运行时调用路由。
- Mobile 已有 Paired Desktop 插件和远程 DSH WebView；Android 真机上验证过配对与同一 DSH 会话。现有网关把获 grant 的请求代理到 DSH，未按业务插件、surface、资源或动作建立隔离路由；跨网生产级 E2E Relay 仍未完成。
- `muse_resource_contract` / `muse_resource_bridge` 已定义 ResourceRef、revision、materialization、remote stream/range 和 Mobile capability profile。这是媒体数据面的基础，不能把图像/视频 bytes 塞进控制消息。现有 Mobile profile 的 `maxResourceBytes=64 MiB` 是**当前实现上限**，不能直接用它传大型原视频；应传代理媒体或短期流，并为媒体预览另定预算。
- `muse.presentation/request/v2` 处理“打开某个 ResourceRef”，不定义远程业务 surface、交互动作、媒体会话或事件同步。新协议应复用其资源身份和路由思想，但拥有独立命名空间。
- Mobile 的 `OpenMusePluginContext.executeHostCommand` 目前是空实现，应用根也没有通用远程插件 surface 容器；默认插件需要真实的 Paired Desktop/Broker connector 与 Mobile 导航入口，不能把现有 DSH WebView 等同于通用控制面。

## 3. 三条协议面

| 面 | 合同 | 内容 | 不包含 |
| --- | --- | --- | --- |
| 发现/展示 | `remote-surface/v1` | 插件/surface 描述、组件树或媒体/Web 预览 handle、能力协商、展示状态 | 本地 path、任意 URL、业务 secret |
| 控制/事件 | `remote-control/v1` | 有 schema 的 action、提交、回执、可恢复事件 | 直接 shell、DOM click 坐标、未校权 RPC |
| 媒体数据 | 复用 Resource Authority，增加 preview rendition 合同 | 缩略图、图片分块、Range/HLS 视频、Web snapshot/受限页面 | 大文件内嵌 JSON、永久外链 |

Paired/Relay 只运送这三类消息；它不决定业务权限。Desktop Host 以 `actor + mobileDevice + desktopDevice + workspace + plugin + surface + action` 计算授权，并将控制请求交给目标插件。DSH 的 Agent 工具和 Mobile 按钮调用同一业务 command，但 caller 与 grant 各自独立，不能互相借权。

### 3.1 能力发现和协商

1. Mobile 连到已配对 Desktop，发送自己支持的协议主/次版本、组件集合、媒体格式/编解码能力、WebView 级别、最大控制消息、Range/HLS 能力和网络/内存预算。
2. Desktop 返回**此 Workspace 中已启用且对该 actor/device 可见**的 surface 列表。一个插件可贡献多个 surface，如“社媒发布”和“素材库”；禁用/崩溃的插件不出现。
3. Mobile 打开 surface 时取得短期 `SurfaceSessionRef`、`generation`、渲染模式与最小权限。服务端基于双方能力选择最高兼容模式；协商失败显示清晰的 unsupported 说明，不降级为任意网页代理。
4. 插件升级/停用、grant 撤销、Workspace 切换或 Desktop 重连使旧 generation 和媒体 handle 失效；Mobile 清空旧界面和缓存，再重新发现。

示意 descriptor（字段为提案，需同时定义 JSON Schema、Rust/Dart 投影与 fixture）：

```json
{
  "protocol": "openmuse.remote-surface/descriptor/v1",
  "pluginId": "com.openmuse.easel-social",
  "surfaceId": "publish-workflow",
  "title": "社媒发布",
  "workspaceRef": "opaque-workspace-ref",
  "modes": ["declarative", "media", "web-snapshot"],
  "requiredCapabilities": ["image", "video-hls", "form", "progress"],
  "readPermissions": ["workspace.resource.read"],
  "actions": [
    {"id": "social.preview", "effect": "read", "requiredPermissions": ["workspace.resource.read"]},
    {"id": "social.publish.commit", "effect": "external-side-effect", "requiredPermissions": ["social.content.publish"]}
  ]
}
```

### 3.2 通用展示语法

V1 只提供预装的控件：`text/markdown`（安全子集）、`image/gallery`、`video-player`、`document/link`、`list/grid`、`form`（文本/选择/开关/日期）、`stepper`、`progress/status`、`diff/compare`、`timeline-basic`、`button/confirmation`。组件树是数据，限定深度、节点数、文本长度和消息体积；未知必需组件拒绝打开，未知可选组件退化为只读信息卡。插件不能下发 Dart 类名、JS bridge 方法、Flutter Widget 路径或任意系统 API 名称。

组件使用稳定 `nodeId`；状态快照有 `stateRevision`。高频手势（拖动时间线/滑杆）在 Mobile 本地即时显示；操作完成后发送一次 `commit` 或限频的 `preview`，不让每个指针事件经过 Broker。键盘草稿留在 Mobile 的 surface session 中，重连可恢复；服务端负责业务 revision 与冲突判定。

### 3.3 Desktop 本地网页

网页有两种安全等级：

1. **默认 `web-snapshot`**：Desktop 插件把本地页面渲染成截图/缩略图或经过处理的只读 HTML 预览；点击业务按钮改走 `remote-control/v1` action。适合发布页预览、分析看板和不适配移动布局的 Desktop 网页。
2. **可选 `web-interactive`**：经 Host 分配的插件专属虚拟 origin、固定路径、导航/子资源 allowlist、CSP 与短期 surface session 在 Mobile WebView 中呈现响应式网页。只有通过该入口的受控 `actionId + payload` 可以改业务状态；不透传任意 `POST/PUT/DELETE` 或 Desktop 浏览器 cookie，不提供任意 `localhost` 转发，也不允许页面直接调用原生 capability。业务插件需专门实现这个适配，旧网页的绝对 URL、service worker、弹窗/OAuth 和 WebSocket 不能保证透明代理。

网页模式不会绕过 Host/Broker：WebView 的通信桥只接受固定协议消息，逐条验证来源、session、action schema 和 grant；网页持有的 token 不具备直接访问 Worker 的权限。Mobile 可随时退回原生只读模式。iOS 发行前需针对 Apple 当前 [App Review Guidelines §2.5.2/§4.7](https://developer.apple.com/app-store/review/guidelines/) 审核远程网页/插件入口，尤其是不得向下载的软件无授权暴露原生 API 和用户隐私权限；协议式原生组件优先。

### 3.4 图片和视频

Desktop 的 `file://`、`http://127.0.0.1` 和原始浏览器 profile URL 永不进入 Mobile descriptor。图片提供按 Mobile 尺寸/DPR 生成的缩略图、必要时分块原图；每个 rendition 绑定原 ResourceRef、Revision、内容 digest、workspace、device、surface session 和 TTL，离线只缓存无敏感性的有限缩略图。

视频 V1 提供 poster、关键帧缩略图、时长和自适应的 HLS 或可 Range 的兼容编码代理文件。Desktop 执行转码和剪辑，Mobile 解码播放。HLS 适合常规回放与弱网自适应；逐帧拖动或实时特效预览需要额外的低时延路径（例如定帧图请求或后续 WebRTC），不能把 HLS 当作所有视频编辑操作的解决方案。iOS/Android 平台均有 HLS 播放支持，可作为预装播放器的基础：[Apple HLS](https://developer.apple.com/documentation/http-live-streaming)、[Android Media3 HLS](https://developer.android.com/media/media3/exoplayer/hls)。

媒体请求走独立流/Range，不受控制消息大小上限限制；网关需要带宽/并发/缓存配额、取消和背压。插件只交 `ResourceRef/Revision` 与转换参数给 Host；Host 发短期媒体 handle，按 grant 流出。每段 HLS/Range 请求都要受控，不能只保护播放列表第一页。

### 3.5 控制动作与状态机

控制请求至少包含：`requestId`、`surfaceSessionRef`、`generation`、`actionId`、经 JSON Schema 验证的 `input`、`expectedStateRevision`、`idempotencyKey`、相对超时 `deadlineMs`（Host 转为本地单调时钟 deadline）。actor/device/workspace/plugin 来自已验证的连接上下文，不能信任 UI 自报。Desktop 返回 `accepted/denied/conflict/unsupported` receipt；耗时工作返 `jobRef`，由递增 `seq` 的事件流报告，断线按 cursor 重放。

示意请求（真实 `input` schema 由插件声明并经 Host 校验）：

```json
{
  "protocol": "openmuse.remote-control/request/v1",
  "requestId": "req-73",
  "surfaceSessionRef": "opaque-surface-session",
  "generation": 4,
  "actionId": "social.publish.commit",
  "input": {"previewRef": "preview-18", "accountRef": "account-2"},
  "expectedStateRevision": "state-12",
  "idempotencyKey": "submission-unique-73",
  "deadlineMs": 10000
}
```

成功 receipt 返回 `jobRef`、新 `stateRevision` 与审计 `decisionRef`；被拒绝时返回稳定错误码。Mobile 遇到超时先按 `requestId/idempotencyKey` 查询 receipt，不自行重发外部副作用动作。事件订阅必须先取 snapshot，再以 `afterSeq` 追增量，并用 generation 拒绝旧连接的迟到事件。

`effect` 分类建议为 `read`、`workspace-propose`、`workspace-commit`、`external-side-effect`。读预览可持短期 session grant；外部发布、删除、费用、导出等动作必须走独立的明确确认与 Broker 再授权。提交时比较 `expectedStateRevision` 和资源 revision，拒绝过期界面操作；用户修改草案后旧确认失效。插件提供 action schema、状态和执行器，Host 拥有鉴权、审计和调用超时。动作名称是业务插件命名空间内的稳定 ID，不是任意 RPC 方法。

## 4. 对视频编辑等业务插件的适用范围

视频编辑插件可贡献 `timeline-basic + video-player + frame-gallery + form + progress`，动作如 `clip.trim`、`clip.reorder`、`caption.set`、`render.preview`、`render.export`。Desktop 持有素材与编辑引擎，Mobile 发送带 revision 的编辑意图、播放代理视频并查看渲染结果。新转场/滤镜可作为表单选项和 Desktop 算法增加，无需改 APK。

可预装 V1 原语无法表达的需求，例如多指逐帧曲线编辑、专业调色面板或移动端本地实时合成，有三种路径：用插件专属响应式 `web-interactive` 实现；增加新的通用组件协议并发布一次 Mobile 更新；或明确该功能仅 Desktop 可用。**“无需更新 APK”适用于协议能力范围内的扩展，不是任意新移动端功能的承诺。**

## 5. 仓库落点

| 增量 | 建议位置 | 说明 |
| --- | --- | --- |
| Wire schema / fixture / TCK | `contracts/openmuse-remote-surface/v1/` + Rust/Dart contract packages | descriptor、snapshot、action、receipt、events、错误码、版本协商 |
| Mobile 默认插件 | `plugins/remote-workbench/` | 安装到 `app/openmuse_mobile` 的默认装配，并接入通用导航/surface 容器；通用组件和播放器，不依赖 Easel/视频编辑插件 |
| Desktop registry/gateway | Host/Broker 与 `plugins/workspace-paired/` 的受限路由 | 业务 service 发现、授权、媒体 handle、独立插件 origin；现有 DSH 全路径代理不能直接复用为通用插件代理 |
| 业务 SDK | `packages/openmuse_plugin_sdk/` 新的 remote-surface provider 接口 | 业务插件提交状态/媒体/动作处理器；Manifest v2 service contribution 表示可发现性 |
| Easel 适配 | Desktop 可选插件 | 首个协议生产者；DSH 仅负责 Agent 编排 |

`remote-workbench` 是 Mobile 安装物；Desktop 业务插件的 Mobile target 仍为 unsupported，因为它没有 Mobile artifact。`remote_capable=true`、已配对、surface service 存在和权限通过是四个不同条件，缺一不可。

## 6. 实施与验收

1. **先做协议闭环**：fake Desktop 插件提供文本、图片、视频 poster、表单和 `preview` action；真实 Android/iOS Mobile 用默认插件发现、渲染、操作、断线恢复；验证未知必需组件/旧 generation 被拒绝。
2. **补媒体面**：实机图片与 HLS/Range 播放；长视频/弱网/seek/取消/背压；验证 Mobile 无本地 path，Media handle 过期/跨设备/跨 Workspace 均拒绝。
3. **补网页面**：Desktop 本地测试页转只读 snapshot；单独验证受限 `web-interactive` 的 origin、导航、CSP、cookie、action 桥和网页断线恢复；不采用通用反向代理。
4. **接 Broker 与配对 Relay**：建立插件级 service routing、read/action/media 分权、持久审计；Android/iOS 真机分别验证局域网与生产 Relay，后者以现有 E2E/设备注册门禁为前提。
5. **两个不同业务验收**：Easel 发布流水线和视频编辑最小切片都使用**同一个已安装的 Mobile 包**；仅在 Desktop 安装/升级/卸载业务插件，Mobile 列表与功能随之变化；一个插件崩溃不影响另一个或 DSH。

关键依据：[`PLUGIN-MANIFEST-V2.zh-CN.md`](PLUGIN-MANIFEST-V2.zh-CN.md)、[`PAIRED-DESKTOP-LOCAL-DSH-TRANSPORT.zh-CN.md`](PAIRED-DESKTOP-LOCAL-DSH-TRANSPORT.zh-CN.md)、[`PAIRED-DESKTOP-E2E-RELAY.zh-CN.md`](PAIRED-DESKTOP-E2E-RELAY.zh-CN.md)、[`packages/muse_resource_contract/lib/src/presentation.dart`](../packages/muse_resource_contract/lib/src/presentation.dart)、[`packages/muse_resource_bridge/lib/src/data_plane.dart`](../packages/muse_resource_bridge/lib/src/data_plane.dart)、[`plugins/workspace-paired/lib/src/paired_desktop_gateway.dart`](../plugins/workspace-paired/lib/src/paired_desktop_gateway.dart)。
