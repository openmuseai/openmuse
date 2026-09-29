# OpenMuse Mobile 架构与实现计划

状态：提案 v2（2026-09-29）

产品输入：[MOBILE-PRODUCT-PRD.zh-CN.md](MOBILE-PRODUCT-PRD.zh-CN.md)

存储决策：[S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md](S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md)

总体架构输入：[PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md](PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md)、[WORKBENCH-FREE-LAYOUT.zh-CN.md](WORKBENCH-FREE-LAYOUT.zh-CN.md)

## 1. 目标与关键结论

目标是在不复制 Desktop 产品形态的前提下，让 Host/Plugin 合同、资源身份和 DSH 业务语义跨 Desktop/Mobile 保持一致。

关键架构结论：

1. **一个 DSH Agent 插件，多个 placement adapter。** `local-sidecar` 负责 Desktop 本机 spawn/loopback；Mobile 通过 `cloud-remote` 连接 managed DSH，或通过 `paired-desktop` 连接用户 Desktop 上的同一个 local DSH；上层面板、Workspace binding、resource intent 和错误语义共用。
2. **传输不是业务协议。** Local 与 Remote 都实现 `DshRuntimeConnector`、`DshCollaborationChannel` 和 `DshPresentationHandle`；DSH 插件不根据 `Platform.isAndroid` 写业务分支。
3. **资源协议必须先去路径化。** 当前 `path/cwd` 只能代表 Desktop 本地实现，Mobile 必须使用 `ResourceRef + revision + anchor + delivery handle`。
4. **逻辑布局与可见布局分离。** 当前 split tree 保留为逻辑 Window 图；新增纯函数 `AdaptiveWindowProjector`，按 viewport 将它投影到 1/2/3 个 slot。旋转和折叠不修改逻辑图。
5. **发行版依赖闭包决定打包内容。** Mobile app 的 pub dependency graph 中不能出现 Helix、desktop sidecar 或 desktop native gate；运行时 `if (Platform...)` 不能解决体积、注册、许可和商店审核问题。
6. **Mobile V1 只允许编译期内置插件。** 不下载执行 Dart/native code；远端 DSH 内的 server plugin 与 Mobile UI 插件是两个信任域。
7. **Workspace 位置不是 Storage Provider。** Cloud/Desktop placement、S3 Provider、DSH execution placement 是三个正交维度，不能合并成一个 endpoint 字段。
8. **Everything is Plugin，但 Authority 留在 Host。** Local/Cloud/Paired Workspace、Sync、Object Storage、Office Engine 和 DSH 都通过独立 contribution/port 接入；Host 只持有身份、策略、资源 revision、Broker 和审计，不吸收具体实现。
9. **S3 是 Storage ABI。** RustFS 是官方自建方向、MinIO 是兼容/迁移 Provider；Office 和 DSH 只消费 Resource/Blob handle，不感知 S3。

## 2. 当前实现差距

| 领域 | 当前事实 | Mobile 阻塞 |
|---|---|---|
| App runner | [`app/openmuse_host`](../app/openmuse_host/) 只有 macOS/Windows runner | 无 Android/iOS 工程、签名与生命周期接线 |
| 产品装配 | Host 和 `openmuse_builtin_plugins` 都依赖四个插件 | Mobile 构建会把桌面插件拉入依赖闭包 |
| Manifest | v1 只有一个 `runtime`，无 target/capability/artifact | 无法在构建前判定平台兼容性 |
| DSH | [`DshSidecarSupervisor`](../plugins/dsh-agent/lib/src/dsh_sidecar.dart) 启动 Node；WebView 只适配 macOS/Windows | Mobile 无 Node，也无 Android/iOS presentation adapter |
| DSH intent | `DshResourceOpenMessage` 使用 `path/cwd` | 远端路径无法在手机上安全解析 |
| Workspace | `LocalWorkspaceController` 与本地 Mount | 缺 Cloud provider、Paired Desktop proxy、统一 catalog 和 placement 模型 |
| Layout | [当前 solver](../app/openmuse_host/lib/src/host/layout/layout.dart) 始终为 split tree 分配 rect | 窄屏会把三窗压扁，无单窗/双窗投影 |
| Surface lifecycle | 隐藏可能卸载 Surface | Mobile 切窗可能丢 WebView/滚动/输入状态 |
| Viewer | Markdown 跨端；图片依赖本地 file；PDF 非 macOS 仅元数据 | Remote delivery、Android/iOS PDF renderer 未完成 |
| Server | 已有 device-token 与 session open/close/heartbeat 入口 | Mobile connector、错误联合、真实部署合同未接入新 Client |
| Storage | Muse Server 已用 `aws-sdk-s3`，但接口仍泄漏 AWS 类型并存在全对象 collect/签名 URL 日志风险 | 缺 Provider-neutral contract、BYOS、TCK、迁移与用户可验证导出 |
| Office | `muse_ioffice_adapter` 仅声明 macOS arm64 Word view，edit/export 未过门禁 | Mobile artifact、bytes handle、保存/同步/冲突与平台认证未完成 |

[旧版 `muse_dsh_mobile`](../../Muse-Clients-Deprecated/middlewares/dsh/mobile/muse-dsh-mobile/)可以作为已验证行为和测试场景的参考：session placement、URL allowlist、generation、防过期回调、WebView 导航策略、前后台与 control channel。是否复用源码必须先通过当前仓库的 provenance/许可证规则；本计划不默认复制旧包。

## 3. 目标系统

```text
┌──────────────────────── OpenMuse Mobile App ──────────────────────────┐
│ Host Shell: Broker / ResourceRef / Permission / Window projection     │
│ Plugins: DSH UI · Viewer · iOffice* · Cloud/Paired Workspace clients  │
└────────────────────────────┬───────────────────────────────────────────┘
                             │ placement-neutral contracts
              ┌──────────────┴───────────────┐
              │ Identity / Device Directory  │
              │ Relay / Cloud Control Plane  │
              └──────────┬────────────┬──────┘
                         │            │
              Cloud placement        Paired Desktop placement
                         │            │ outbound E2E channel
            ┌────────────▼──────┐  ┌──▼───────────────────────────┐
            │ Cloud Workspace   │  │ OpenMuse Desktop             │
            │ Resource Authority│  │ Local Workspace Authority    │
            │ Remote DSH        │  │ Local DSH + local tools      │
            └────────────┬──────┘  └──┬───────────────────────────┘
                         │            │ optional sync/snapshot
                         └──────┬─────┘
                                ▼
                       BlobStorePort / S3 ABI
                  Managed RustFS* / BYOS / MinIO / AWS ...
```

`*` 表示 capability/admission gate 后才启用，不代表当前已达到发布门槛。

Desktop 使用相同 DSH core，但选择 `LocalSidecarConnector + DesktopWebViewAdapter + LocalWorkspaceProvider`。Mobile 与 Desktop 的产品能力不同，不影响 DSH 插件业务合同相同。

## 4. 领域隔离与 Plugin 拓扑

### 4.1 限界上下文

| 领域 | 拥有的真源 | 不拥有 |
|---|---|---|
| Identity | account、session、成员身份 | 设备文件权限、S3 secret |
| Device/Pairing | device key、presence、pairing、workspace grant | Workspace 内容 |
| Workspace Catalog | Workspace identity、placement、入口与状态 | 资源 bytes |
| Resource Authority | resourceRef、revision、ACL、CAS、lease | Office 内部模型、S3 协议 |
| Workspace Sync | change feed、sync cursor、conflict、provider generation | Workspace 身份、blob 实现 |
| Object Storage | immutable blob、range/multipart、storage receipt | 文件树、协作、当前 revision |
| Office Engine | 格式解析、布局、编辑、导出 | Workspace ACL、S3、设备配对 |
| DSH Agent | session、Agent UI、context/proposal/intent | 资源真源、S3 凭据、最终 commit |
| Presentation | Window/Surface/焦点/生命周期 | 文档与 Agent 业务状态 |

跨域只能通过 Host Broker 的 Command/Service/Event/Capability 与 Data handle。插件不得 import 另一插件实现，也不得通过共享数据库表或绝对路径形成旁路。

### 4.2 Everything is Plugin 的具体含义

建议 contributions：

```text
workspace.authority@1
  providers: local, cloud, paired-desktop

workspace.sync@1
  policies: local-only, snapshot, mirror, migrate

blob.store@1
  providers: local-fs, s3

office.word@1 / office.excel@1 / office.slides@1 / office.pdf@1
  engines: ioffice adapters, viewer fallback

dsh.runtime@1
  connectors: local-sidecar, cloud-remote, paired-desktop
```

Host 可以定义这些接口、provider selection 和 policy，但具体 provider 都位于 plugin/distribution 层。DSH 的 server-side Workspace/Office plugins 同样只能消费这些语义合同；“Everything is Plugin”不是跳过 Resource Authority 的理由。

## 5. DSH：同一插件、不同 placement adapter

### 5.1 拆分边界

建议包结构：

```text
plugins/dsh-agent/                         # 插件 ID、Panel、状态/设置、业务协调
packages/dsh_runtime_contract/             # transport-neutral Dart interfaces/types
packages/dsh_local_sidecar/                 # Desktop Process/Node/loopback adapter
packages/dsh_cloud_remote/                  # Cloud placement/auth/heartbeat/bridge
packages/dsh_paired_desktop/                # Device relay/E2E channel/Desktop attachment
packages/dsh_view_desktop/                  # WKWebView/WebView2
packages/dsh_view_mobile/                   # Android WebView / WKWebView
```

`plugins/dsh-agent` 不 import `dart:io Process`、平台 WebView 实现、Cloud SDK 或 pairing SDK。composition root 根据 Workspace placement 注入 connector、view adapter 和 credential/workspace ports。

### 5.2 核心接口

```dart
abstract interface class DshRuntimeConnector {
  Stream<DshAttachmentState> get states;
  Future<DshAttachment> connect(DshConnectRequest request);
  Future<void> reconnect(DshAttachmentRef attachment);
  Future<void> disconnect(DshAttachmentRef attachment);
}

abstract interface class DshCollaborationChannel {
  Stream<DshIntentEnvelope> get intents;
  Future<DshReceipt> bind(WorkspaceBinding binding);
  Future<DshReceipt> contribute(ContextProjection projection);
  Future<void> close();
}

abstract interface class DshPresentationAdapter {
  Widget build(DshPresentationHandle handle);
  Future<void> setVisibility(SurfaceVisibility visibility);
  Future<void> reload();
}
```

`DshAttachment` 只携带经过验证的 presentation handle、collaboration channel、attachment/session refs 和 capability snapshot。Local connector 可以把 loopback URL 封装进 handle；Remote connector 可以把 `session/open` 返回的 URL 封装进去。插件 core 不读取原始 query token。

`DshConnectRequest` 必须携带 `workspaceRef`、`workspacePlacement`、`requestedCapabilities` 和 generation。connector 选择规则由 composition/registry 完成：

| Workspace placement | Mobile connector | DSH 实际位置 |
|---|---|---|
| Cloud | `CloudRemoteDshConnector` | OpenMuse managed instance |
| Paired Desktop | `PairedDesktopDshConnector` | 目标 Desktop 的 local sidecar |
| Desktop 本机 | Mobile 不适用；Desktop 使用 `LocalSidecarConnector` | 当前 Desktop |

三个 connector 都必须通过同一 TCK。Cloud 与 Paired Desktop 的鉴权、连接和故障状态不同，但 `bind/contribute/intent/receipt` 语义不能分叉。

### 5.3 Attachment 状态机

```text
idle
  → resolvingIdentity
  → resolvingPlacement
       ├─ cloud: placing ─────→ queued ─┐
       └─ desktop: presence → pairing/grant
  → connectingTransport                 │ retryAfter
  → presenting                          └────→ placing
  → binding
  → ready ↔ degraded
  → reconnecting
  → closed

任意阶段 → failed(code, retryable, attachmentRef)
scope/generation 变化 → closingOld → idle
```

规则：

- `session/open` 返回 Ready/Queued/Denied 判别联合，不允许以 `null` 表示所有失败；
- 每条异步响应校验 `attachmentRef + generation`；
- 页面 ready 与 bridge bind receipt 分开；
- heartbeat 失败采用有界容错，连续失败后进入 degraded/reconnecting；
- app background 时降低活动，不在平台冻结后假设定时器可靠；resume 后重新验证 session/token；
- `disconnect` best-effort close 之后必须撤销本地 token/handle 和 JS channel；
- local connector 映射相同状态：spawn=placing，WebView=presenting，workspace RPC=binding。
- Paired Desktop 的离线、休眠、设备 key 不匹配、grant 过期必须映射为独立错误，禁止悄悄改连同名 Cloud Workspace；
- paired relay 只转发加密 envelope/stream，不成为 Workspace 或 DSH 业务参与者。

### 5.4 Cloud 与 Paired Desktop 控制面

Cloud placement 优先复用 [Muse Server 已存在的接口](../../Muse-Server/AppFlowy-Cloud/src/api/muse.rs)：

- `POST /api/muse/dsh/device-token`
- `POST /api/muse/dsh/session/open`
- `POST /api/muse/dsh/session/close`
- `POST /api/muse/dsh/session/heartbeat`

实现前仍需做一次 contract test，确认生产 BFF/DSH pool 的 Ready/Queued/Denied 响应、URL path、TTL、错误码和 bridge capabilities 与客户端 schema 一致。代码存在不等于生产部署已满足合同。

Paired Desktop 需要新增独立控制面，不能复用 Cloud DSH `session/open` 假装完成：

```text
device.register / device.presence
pairing.create / pairing.confirm / pairing.revoke
workspace.grant.create / workspace.grant.revoke
desktop.attachment.open / heartbeat / close
relay.connect
```

Desktop 只建立 outbound relay 连接。配对使用设备长期签名 key；每次 Workspace attachment 使用短期 capability，内部再建立端到端加密 channel。账号服务与 relay 能看见最小路由元数据，但不应看到 Workspace bytes、DSH launch token 或 S3 secret。

### 5.5 安全分层

| 凭据/handle | 用途 | 禁止 |
|---|---|---|
| Cloud access token | 调 Muse Server | 进入 DSH instance/JS/log |
| Device signing key | 证明 Mobile/Desktop 设备 | 离开 OS keychain/keystore、跨设备复制 |
| Pairing/grant capability | 连接一台 Desktop 的一个 Workspace | 扩展到其它 Workspace/长期复用 |
| Device token | workspace bind/collaboration | 当模型 key、持久明文存储 |
| Launch token | 打开特定 DSH Web presentation | 埋点、日志、复制、作为通用 API token |
| Resource delivery handle | 读一个授权资源/范围 | 暴露源存储凭据、跨 workspace 复用 |
| S3 credential/role | Object Storage Provider 内部访问 | 进入 Mobile、Office、DSH、ResourceRef |

WebView 只允许 session URL 的 scheme/host/port 和规定的 tenant path；导航、新窗口、下载、证书异常、外部 scheme 都由 Host policy 处理。

## 6. Workspace、Resource Authority 与 Sync

### 6.1 Provider Registry，实现同一 Host contract

```text
WorkspaceAuthority
├── LocalWorkspaceProvider          # Desktop: mounts, local files, materialization
├── CloudWorkspaceProvider          # Cloud tree, remote revisions, streams
└── PairedDesktopWorkspaceProvider  # Mobile proxy to an authorized Desktop provider
```

三者输出相同 `WorkspaceDescriptor / ResourceRef / ResourceRevision`。Host Resource Authority 仍负责统一授权、lease、revision 和审计；provider 负责实际定位/读取/提交。消费插件不能发现具体 provider 后绕过 Broker。

建议 placement：

```text
WorkspacePlacement =
  local(deviceRef)
  | cloud(regionRef, storageProfileRef)
  | pairedDesktop(deviceRef, grantRef)
```

`Cloud Mirror` 和 `Cloud Snapshot` 是 Sync policy/status，不是第四种 Authority。否则同一 Workspace 会出现两个真源且无法定义冲突。

### 6.2 废除跨边界 path 语义

现有 DSH bridge 的：

```text
resource.open { path, cwd, line }
```

应迁移为：

```text
resource.open {
  resourceRef,
  revision?,
  anchor: { line?, column?, blockRef?, selectionRef? },
  preferredMode: view | edit | diff
}
```

Desktop adapter 可以在 Host 授权后把 `ResourceRef` 解析成本地 materialization；Mobile adapter 取得 remote stream/range/short-lived URL。远端 DSH 永远不能要求 Mobile 打开服务器绝对路径。

### 6.3 Data plane

- 小文本/Markdown：有界 UTF-8 snapshot，带 revision/digest；
- 图片/PDF：短期 scoped URL 或 range stream，不把整个大文件塞进 control envelope；
- Agent context：最小必要片段，带来源与 revision；
- 修改：Agent 返回 proposal，Authority 做 revision 检查与 commit，Mobile 负责审批而非直接信任远端写入；
- URL/handle 过期、撤销和 audience mismatch 必须是可区分错误。

### 6.4 Workspace Sync Plugin

`com.openmuse.workspace-sync` 是独立插件，不属于 Local Workspace、Cloud Workspace、S3 或 Office Engine。它消费 change feed 与 Resource 服务，产生 Cloud revision 和 sync receipt。

```text
Local change feed
  → plan(digest, baseRevision, policy)
  → materialize bounded stream
  → BlobStore.putIfAbsent
  → Cloud ResourceAuthority.commit(expectedRevision)
  → sync cursor/outbox receipt
```

策略：

| Policy | 方向 | Cloud 可写 | Desktop 离线时 Mobile |
|---|---|---:|---|
| local-only | 无 | 否 | 不可访问 |
| snapshot | Local → Cloud 不可变发布 | 否 | 可只读 |
| mirror | 双向 revision sync | 是 | 可读写；冲突显式处理 |
| migrate | 一次性 Local → Cloud，校验后换真源 | 是 | 迁移完成后可用 |

Sync 不使用文件时间戳作为 revision，不原地覆盖 S3 key。文本/Office/binary 冲突策略分别由相应 Resource/Engine provider 提供；无法语义合并时保留两个 revision。

## 7. Object Storage 与数据自主

详细决策见 [S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md](S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md)。本架构只冻结与 Host/Plugin 相关的边界：

```text
Resource Authority ── BlobStorePort ── S3StorageProvider ── S3 ABI
                                    ├─ OpenMuse Managed / RustFS*
                                    ├─ User BYOS / MinIO or RustFS
                                    └─ Other certified S3 provider
```

- Object Storage 只拥有 immutable blob/range/multipart/receipt，不拥有 Workspace tree、ACL 或当前 revision；
- `aws-sdk-s3` 只能出现在 Rust S3 provider crate；
- Flutter、iOffice、DSH 和 Workspace Sync 都不能接收 endpoint/bucket/credential；
- OpenMuse Managed 与 BYOS 使用同一 Provider TCK；
- RustFS 是官方自建方向，但 stable pin、安全修复、恢复/升级演练和 90 天 soak 通过前不得成为唯一生产真源；
- MinIO 保留为兼容/迁移 Provider，不作为新建长期默认；
- Mobile 获得的是短期 Blob/Resource handle，而不是 presigned URL 真源或永久 S3 key。

Cloud 保存顺序必须是 `engine export → validate/digest → immutable blob → HEAD/checksum → metadata CAS → outbox event`。S3 成功但 metadata 失败的孤儿对象由 grace-period GC 回收；metadata commit 未完成前 UI 不显示保存成功。

## 8. Flutter + Rust iOffice 插件边界

当前 [`muse_ioffice_adapter`](../packages/muse_ioffice_adapter/)已经遵循正确方向：接收 `bytesHandle`，不直接读取 S3；但目前只声明 macOS arm64 Word view，`edit/export` 仍被 admission gate 禁止。

目标插件链：

```text
ResourceRef@R1
  → Host materialize(bytes/range handle)
  → muse.ioffice.word adapter
  → Rust Office engine session
  → Flutter Surface
  → engine.toDocx()
  → Host resource.commit(expectedRevision=R1, outputHandle)
  → Resource Authority / BlobStorePort
```

硬规则：

- 每种格式是独立 Engine Adapter，不创建一个跨 Word/Excel/Slides/PDF 的大 switch；
- Office Engine 不知道 Workspace placement、S3 Provider 或 DSH placement；
- 无 `toDocx/toXlsx/toPptx` 就不注册 edit/commit，heap 编辑不能伪装保存；
- Mobile native artifact 分别经过 Android `.so`/ABI 和 iOS static XCFramework/签名门禁；
- 未通过本机引擎门禁时可选择 Viewer fallback 或 Desktop remote handoff；
- DSH 通过 `muse.word`/`muse.table` 等领域插件 query/propose/apply，不能直接调用 FFI 或写 blob；
- Agent apply 必须经过 approval + expected revision + engine validation + Resource commit。

Storage Provider 迁移不改变 Office Engine session contract；Office 文档迁移后只有 BlobRef/provider generation 变化，resourceRef 与应用 revision 保持可追踪。

## 9. 响应式 Window 架构

### 9.1 两层模型

当前 `WorkbenchLayoutSnapshot` 同时承担逻辑结构和屏幕几何。Mobile 增加独立投影层：

```text
WindowGraph / SurfaceBinding        持久、与设备宽度无关
              +
ViewportProfile                    width/height/insets/features/input
              +
SurfacePresentationHints           min/ideal size, priority, canSuspend
              ↓
AdaptiveWindowProjector (pure)
              ↓
WindowPresentationPlan             slots + visibility + lifecycle hints
```

`WindowPresentationPlan` 是派生状态，不写回 `layout-v1.json`。Mobile 可另存 `activeWindowRef`、双窗组合和用户折叠偏好，但不能因旋转把三窗逻辑图永久改成单窗。

### 9.2 建议类型

```dart
enum WindowSizeClass { compact, medium, expanded }
enum SurfaceVisibility { visible, warm, suspended, detached }
enum SurfacePlacementKind { embedded, fullscreen, overlay, floating }

final class ViewportProfile {
  Size logicalSize;
  EdgeInsets safeInsets;
  List<DisplaySegment> segments;
  double textScale;
  Set<InputModality> inputs;
}

final class WindowPresentationPlan {
  WindowSizeClass sizeClass;
  List<WindowSlot> slots;
  Map<String, SurfaceVisibility> visibility;
  String focusedWindowRef;
}
```

`overlay` 只冻结合同，V1 projector 不产出它。`floating` 继续只用于 Desktop/未来系统多窗。

### 9.3 求解顺序

1. 扣除 system safe area、IME 与不可用 display feature；
2. 根据 segment 与 Surface min width 计算最大 1/2/3 容量；
3. 保证 active/focused Window 可见；
4. 按最近任务和组合偏好选择其它 Window；
5. 分配 ideal width，剩余空间按 stretch weight 分配；
6. 产生 visible/warm/suspended 提示；
7. 仅在 plan identity 变化时更新视图几何。

### 9.4 PlatformView 与状态保持

继续沿用当前稳定 `instanceRef + root Stack` 的方向，但补齐 Mobile 生命周期：

- 几何切换不更换 Widget key；
- visible → warm 优先隐藏/暂停，不注销 DSH attachment；
- 超过内存预算时 warm → suspended，插件必须用 snapshot/session 恢复；
- Android/iOS PlatformView 的保活、offstage 和 texture/hybrid composition 必须分别做 spike，不能假定 Desktop `Positioned` 行为等价；
- Window swipe 的动画层与真实 PlatformView 手势隔离。必要时切换期间使用 snapshot proxy，结束后恢复真实 view；
- IME、selection、WebView history 和 scroll 不承诺在 OS 杀进程后原样保留，但业务 session 与未发送草稿必须可恢复。

## 10. Plugin Manifest v2 与平台判定

### 10.1 为什么 v1 不够

当前 `runtime: built-in | native-process | web-view` 把 UI runtime、execution placement 和平台 artifact 混在一个字段中，也没有 target/capability。DSH 同时有 built-in Flutter UI、WebView presentation 和可本地/远端的执行 runtime，单一枚举无法准确表达。

建议新增 v2，v1 通过迁移器读取：

```json
{
  "manifest_version": 2,
  "id": "com.openmuse.dsh-agent",
  "version": "0.2.0",
  "ui_runtime": "built-in-flutter",
  "execution": {
    "connectors": ["local-sidecar", "cloud-remote", "paired-desktop"]
  },
  "compatibility": {
    "targets": ["macos-arm64", "macos-x64", "windows-x64", "android-arm64", "ios-arm64"],
    "requires_any": [
      ["process.spawn", "loopback.webview"],
      ["cloud.dsh.session", "embedded.webview"],
      ["paired.desktop.attachment", "embedded.webview"]
    ]
  },
  "presentation": {
    "min_width": 320,
    "ideal_width": 420,
    "can_suspend": true,
    "placements": ["embedded", "fullscreen"]
  },
  "artifacts": []
}
```

规则：

- manifest compatibility 用于构建前解析和运行时二次防御；
- distribution lock 记录最终选择的插件、adapter、target、版本、哈希和许可证；
- target 不匹配或 capability 表达式不成立时，贡献根本不注册；
- 一个插件可声明多个 connector，但每个发行版只绑定经过允许的 adapter；
- `platforms` 不能只依赖 Flutter pubspec，因为纯 Dart 插件和远端 connector 同样需要产品约束。

## 11. 插件兼容性与改造评估

先评估 `Muse-Client/plugins/` 中现有四个插件，再列出为 Workspace placement、Storage 和 Office 必须补齐的第一方插件。工程量为单人有效工程周的量级估计，完成 spike 后校准。

| 插件 | Desktop | Android/iOS V1 | 打包策略 | 主要改造 | 估算 |
|---|---|---|---|---|---:|
| DSH Agent | macOS/Windows：local sidecar | **纳入**：cloud remote + paired Desktop | 同一 plugin core；按 placement 注入 adapter | core/connector 解耦、Mobile WebView、Cloud/Pairing 状态机、ResourceRef | Cloud 4–6 周；Pairing 3–5 周；iOS额外 1–2 周 |
| Open File Viewer | macOS 完整度较高；Windows 部分 fallback | **部分纳入**：Markdown/文本/图片；PDF 过门禁后启用 | mobile profile 按 renderer feature 裁剪 | remote delivery、缓存/range、图片解码预算、Android/iOS PDF | 2–4 周 |
| Helix | macOS 可用，Windows 在建 | **不打包** | 仅 desktop distribution 依赖 | V1 无改造；未来另做 remote editor/terminal surface，不能把本地 hx 塞进 Mobile | V1 0；未来 4–7 周 |
| Native Text Gate | macOS/Windows 诊断插件 | **不打包** | 仅 desktop test/internal profile | 它不是产品插件；Mobile IME 另建 host integration gate | Mobile gate 0.5–1 周 |

补充判断：

- `Open File Viewer` 当前 Markdown renderer 是最接近跨端的部分；`Image.file` 和本地 path contract 仍需替换。
- 当前非 macOS PDF 只是读取 bytes/估算页数，不应标记为 Mobile PDF 已支持。
- Helix 强依赖进程、PTY、二进制与本地文件 materialization。即使 Android 技术上可交叉编译，也不符合 V1 产品定位、APK 体积和 iOS 对齐要求。
- Native Text Gate 的价值是 Desktop NSView/HWND 生命周期验证，不应出现在用户 Mobile 插件列表。

后续第三方插件必须先过 `target × capability × artifact` 静态校验；未知兼容性默认不打包，不能乐观启用。

新增/升级的第一方插件与 provider：

| 插件/Provider | 运行位置 | 职责 | 明确不做 | 估算 |
|---|---|---|---|---:|
| Local Workspace Provider | Desktop | 本地 mount/change feed/materialization | Cloud、S3、配对 | 1–2 周重构 |
| Cloud Workspace Provider | Server + Client adapter | Cloud catalog/revision/stream/commit | S3 实现、Office 模型 | 3–4 周 |
| Paired Desktop Workspace Provider | Mobile + Desktop | 代理已授权 Desktop Authority | 自动授权、数据落云 | 3–5 周（与 Pairing 共用） |
| Workspace Sync Plugin | Desktop/Server worker | snapshot/mirror/migrate、cursor/conflict | 成为 Resource 真源 | 4–6 周 |
| S3 Storage Provider | Server；可选 Desktop agent | S3 Profile、BYOS、multipart/range/receipt | Workspace tree、S3 UI | 4–6 周，RustFS qualification 另计 |
| iOffice Word Engine | Flutter + Rust artifact | docx view/edit/export，经 bytes handle commit | S3/Workspace/DSH auth | Mobile view 3–5 周；可靠 export 取决于 engine |
| iOffice Excel/Slides/PDF | 各自 adapter | 格式独立 Surface/Context/commit | 共享大 switch、虚假 capability | 每种独立估算，未有 engine 不排期 |

`muse_ioffice_adapter` 当前不是已装配生产插件；只有 artifact、license、ABI、layout、export 和 corpus gate 全部通过后，才进入对应 distribution lock。

## 12. 发行版与打包架构

### 12.1 分离 app root

Flutter 的 dependency graph 在构建前已决定 native plugin 注册和 assets。仅用 flavor 或运行时条件无法真正移除 Helix/Node。因此采用独立 app root：

```text
app/openmuse_desktop/                    # 由现 openmuse_host 演进
app/openmuse_mobile/                     # Android + iOS runner
packages/openmuse_host_shell/            # 共用 Broker/Window/主题/路由
distribution/openmuse_desktop_plugins/   # local workspace/sync/helix/viewer/dsh-local/office gates
distribution/openmuse_mobile_plugins/    # cloud/paired workspace/viewer/dsh-remote/mobile office gates
```

不要让 `openmuse_host_shell` 反向依赖任一具体插件。composition root 创建 `HostCapabilitySnapshot`，装配允许的 plugin/adapters。

### 12.2 Target profiles

| Profile | 包含 | 明确排除 |
|---|---|---|
| desktop-macos | Local Workspace、Sync、Helix、Viewer、DSH local、已认证 iOffice、macOS adapters | Mobile presentation |
| desktop-windows | Local Workspace、Sync、Helix、Viewer、DSH local、已认证 iOffice、Windows adapters | macOS assets |
| mobile-android | Cloud/Paired Workspace、DSH remote、Viewer mobile、已认证 iOffice、Android capabilities | Node、hx、PTY、desktop native code |
| mobile-ios | Cloud/Paired Workspace、DSH remote、Viewer mobile、已认证 iOffice、iOS capabilities | Node、hx、动态下载代码 |
| internal-gates | 对应平台诊断插件 | 不进入 production catalog |

### 12.3 Closure 门禁

每个 release 生成 `distribution-lock.json` 和 SBOM，并在制品层扫描：

- APK/AAB/IPA 内不存在 `node`、DSH npm closure、`hx`、Helix runtime grammar、WebView2、Windows DLL、macOS dylib；
- Desktop 包不意外携带 Mobile-only SDK；
- native plugin registrant 与 distribution lock 一致；
- 未声明当前 target 的插件或 artifact 使构建 fail closed；
- notices 与实际闭包一致；
- iOffice native artifact 的 target/ABI/digest/admission 与 lock 一致；未认证格式不出现创建/编辑入口；
- Storage Server 的 provider lock 单独记录 RustFS/MinIO/SDK 版本，不把对象存储二进制装入 Client。

## 13. 实施阶段

### M0：合同冻结、Provenance 与风险 Spike（2 周）

交付：

- 冻结本 PRD、placement model、DSH connector、Workspace Authority、ResourceRef、BlobStorePort、Sync receipt 和 manifest v2 draft；
- 对 Android/iOS PlatformView 隐藏/保活/滑动做最小 spike；
- 对生产 `session/open`、Queued、device-token、bridge capabilities 做 contract probe；
- 对 Desktop outbound relay、设备 key、E2E channel 和大文件 range stream 做 threat model/prototype；
- 用 AWS S3、RustFS、MinIO 各跑一次 S3 Profile smoke test，形成第一版差异清单；
- 验证 iOffice 各格式现有 artifact、license、ABI、view/export 能力，不把计划中的能力写成已完成；
- 确认旧 `muse_dsh_mobile` 的源码 provenance，决定复用、迁移测试还是 clean-room 重写；
- 选择 Mobile PDF 技术路线，若未通过则明确从首个 alpha 移除。

退出门禁：所有关键响应 schema 有 fixture；旋转/单窗切换不会必然重建 WebView；配对、凭据与 relay threat model 无未处置 P0；RustFS/MinIO 的结论来自黑盒结果而非 feature table。

### M1：App、Distribution 与 Adaptive Host（3–4 周）

交付：

- 抽取 `openmuse_host_shell`；建立 `openmuse_mobile` runner；
- 拆分 desktop/mobile distribution；
- manifest v2 parser、target/capability resolver、distribution lock；
- CI 添加 dependency closure 与 APK/IPA 禁带扫描；
- Mobile Host capability snapshot、登录/配置注入 seam；
- `ViewportProfile / AdaptiveWindowProjector / WindowPresentationPlan`；
- Compact Window Deck、Medium 双窗、Expanded 三窗；
- display feature/hinge、安全区、IME、字体缩放接入；
- visible/warm/suspended lifecycle 与稳定 `instanceRef`；
- edge swipe、显式切窗、返回键和键盘焦点仲裁；
- layout migration：Desktop `layout-v1` 不被 Mobile 投影污染。

退出门禁：空 Mobile shell 可构建 Android APK 和 iOS `--no-codesign`；制品不含 Desktop runtime；尺寸/姿态 golden 通过；200 次旋转/切窗无重复 attachment、无孤立 PlatformView。

### M2：Resource/Workspace Provider 与 S3 基线（3–4 周）

交付：

- 从 Local Workspace 抽出 `WorkspaceAuthority`、`ResourceAuthority`、change feed 和 provider registry；
- 完成 `ResourceRef + revision + materialize/commit handle`，path 只留在 Local adapter；
- 建立 Cloud Workspace metadata/CAS/outbox 基线；
- 实现 provider-neutral `BlobStorePort` 与 Rust `aws-sdk-s3` adapter；
- 建立 S3 Profile TCK，覆盖 put/head/range/delete/multipart、checksum、TLS、addressing style 和故障注入；
- 建立 BYOS 配置、Credential Vault 接口和 endpoint SSRF policy，但此阶段不开放最终用户入口。

退出门禁：Local/Cloud provider 通过共同 Resource TCK；三种 S3 实现通过基础黑盒套件；Office/DSH/Flutter 依赖图中无 S3 SDK 或永久凭据。

### M3：Cloud Workspace + Remote DSH Vertical Slice（3–4 周）

交付：

- 将当前 DSH plugin 拆成 core/local/remote/view adapters；
- 保持 Desktop local sidecar 回归测试绿色；
- Mobile 实现 identity、session placement、Queued、WebView、bind receipt、heartbeat、close/reconnect；
- 实现 Cloud Workspace catalog、snapshot/stream/range delivery 与 Agent proposal/approval/receipt；
- Android/iOS navigation policy、TLS fail-closed、token redaction；
- picker/share/speech/lifecycle capability bridge 的最小集合；
- Markdown/文本/图片 Mobile renderer；PDF 按 M0 结论启用或 fallback；
- Agent Window 完整状态与错误码 UI。

退出门禁：测试环境真实账号可完成 `login → Cloud Workspace → session/open → WebView → bind → ResourceRef/context → intent receipt`；跨 Workspace、过期 revision 和过期 handle 均 fail closed；换 Workspace 无旧内容闪现。

### M4：Paired Desktop Workspace 与 DSH（4–6 周）

交付：

- 设备登记/presence、pairing challenge、device key rotation/revoke；
- Desktop 逐 Workspace grant 与 capability/TTL/revoke；
- Desktop outbound relay、Mobile attachment、E2E envelope/stream；
- `PairedDesktopWorkspaceProvider` 与 `PairedDesktopDshConnector`；
- offline/sleep/reconnect/grant revoked/device replaced 的状态和 UI；
- Desktop 本地 Resource Authority 与 DSH 保持真源，不因 Mobile 连接自动上传。

退出门禁：仅同账号但未配对、已配对但未授权 Workspace、grant 过期三类访问全部失败；relay 无法读取业务 payload；Desktop 离线时不静默切换同名 Cloud Workspace；大文件按 range/backpressure 传输。

### M5：Workspace Sync、BYOS 与迁移（4–6 周）

交付：

- `Workspace Sync Plugin` 的 local-only/snapshot/mirror/migrate 策略；
- durable outbox、sync cursor、幂等重试、冲突对象和用户可见 receipt；
- BYOS server-mediated 与 Desktop-mediated 配置流程；
- Provider TCK 报告、能力快照、凭据撤销和健康状态；
- S3 Provider 间可暂停/续传迁移、digest 验证、generation 切换和旧源观察期；
- RustFS qualification 环境、版本固定、监控、备份/恢复/升级 runbook；MinIO/AWS S3 保持互操作。

退出门禁：任一同步策略不会在未确认时上传；断网/重启可恢复；冲突不静默覆盖；迁移前后 manifest 和 digest 一致；删除旧副本必须再次确认并产生 receipt。

### M6：iOffice 插件化与 Mobile 门禁（4–7 周，按格式拆分）

交付：

- 将 `muse_ioffice_adapter` 装配为独立 format engine contributions；
- Android `.so`/ABI 与 iOS XCFramework/签名 artifact pipeline；
- `ResourceRef → bytes handle → engine session → export handle → expectedRevision commit`；
- Word 优先完成 view；只有可靠原格式 export/corpus gate 通过后才注册 edit；
- Excel/Slides/PDF 各自按 artifact 和语义测试独立开关；
- 无引擎时提供 Viewer fallback 或“在 Desktop 打开”。

退出门禁：Office Engine 不接触 Workspace/S3/DSH 凭据；未通过 export 的格式没有保存入口；崩溃、低内存、字体/布局 corpus 和跨平台 round-trip 达到对应 capability 门禁。

说明：该阶段估算只覆盖接入、artifact 与门禁；若底层 Office engine 本身缺少可靠编辑/导出，实现引擎能力需另行估算，不能由 adapter 排期吸收。

### M7：Android Alpha（2 周）

交付：

- applicationId、release signing、arm64 + Play 要求 ABI；
- deep link/通知入口、网络与后台恢复；
- APK/AAB 打包脚本、安装脚本、artifact verification；
- 真机手机、平板、折叠屏矩阵；
- 隐私、权限文案、SBOM/notices。

退出门禁：产品 PRD M1 范围全部通过；不能继续使用 debug signing 作为发布产物。

### M8：iOS Beta（2–3 周）

交付：

- bundle ID、entitlements、签名/TestFlight；
- WKWebView、picker/share/speech、background/resume 适配；
- iPhone/iPad 多尺寸、动态字体和 VoiceOver；
- App Store 隐私清单与审核说明。

退出门禁：与 Android 使用同一 DSH/Resource fixture；无下载执行代码；TestFlight 真机验收通过。

上述范围约为 **27–38 单人工程周**。Flutter/Rust、Server/Storage、Desktop/Relay 和平台发布可并行时，目标日历时间约 **15–22 周**；M3 可先形成 Cloud-only 内测，M4 后开放 Paired Desktop，M5 后开放 BYOS/Sync。估算不含底层 Office engine 新增编辑/导出能力，也不把 RustFS 90 天生产 soak 压缩进开发工期；M0 后必须按 spike 和真实 Provider TCK 结果重估。

## 14. 测试与发布门禁

### 14.1 合同测试

- Dart/Rust/Server 对 manifest v2、Workspace placement、ResourceRef、sync receipt、DSH state/error fixture 交叉解析；
- Ready/Queued/Denied、token expiry、workspace mismatch、late generation；
- local/cloud/paired DSH connector TCK：相同 bind/context/intent/receipt 语义；
- local/cloud/paired Workspace provider TCK：相同 list/materialize/commit/revision/error 语义；
- S3 Provider TCK：AWS S3、RustFS、MinIO 的必需 Profile 行为与故障映射；
- Office adapter TCK：bytes handle、capability 声明、expected revision、export validation；
- v1 manifest 迁移和不支持 target fail closed。

### 14.2 Window 测试

| 场景 | 断言 |
|---|---|
| 360×800 / 412×915 | 只显示一个完整 Window，可显式/边缘切换 |
| 600–959dp | 只显示两个满足最小宽度的 slot |
| ≥960dp | 默认三窗，焦点与比例稳定 |
| 铰链分段 | 无主要控件跨 hinge；每 segment 几何合法 |
| 旋转/分屏 200 次 | logical graph 不变、instanceRef 不变、无重复连接 |
| 大字体 + 软键盘 | 主要动作可达，无负尺寸/溢出 |
| WebView 内横滑 | 内容手势优先，边缘手势仍可切窗 |

### 14.3 Remote DSH 测试

- 无登录/无 Workspace 不发 session open；
- Queued 严格按 `retryAfterMs`，切 scope 后停止旧重试；
- 页面 ready 但 bind 失败只进入 degraded，不显示 Ready；
- background/resume、网络切换、token expiry、服务端重启；
- URL host/path/query 校验、TLS 失败、外部导航和下载拦截；
- 日志/埋点/崩溃报告 secret 扫描；
- Desktop local connector 无回归，仍不依赖 Cloud session API；
- paired connector 在 Desktop 离线/休眠、grant 撤销、device key 轮换时给出可区分错误；
- 同账号但未配对、已配对但未获当前 Workspace grant 时不得 attachment；
- Cloud 与 Paired Desktop 不允许按名称或路径互相 fallback。

### 14.4 Pairing、Sync 与 Storage 测试

- pairing challenge 重放、设备冒充、key 轮换、撤销、grant scope/TTL 和跨 Workspace 越权；
- relay 仅见路由元数据，E2E envelope 篡改/重放/乱序 fail closed；
- Desktop 断网、休眠、重启和 IP 变化后的恢复，不要求家庭网络入站端口；
- snapshot/mirror/migrate 的断点续传、进程崩溃、幂等重放和 split-brain 冲突；
- 未经用户选择时 local-only Workspace 零上传，含日志、预览和诊断包；
- S3 put 成功/metadata CAS 失败、multipart 中断、checksum mismatch、慢读和权限骤变；
- Provider 迁移双读核验、generation 原子切换、回滚、观察期和删除 receipt；
- RustFS/MinIO 节点或磁盘故障、扩缩容、滚动升级、恢复演练及 digest audit。

### 14.5 Office 能力门禁

- 每种格式分别验证 view/edit/export capability，未实现能力不注册；
- Office corpus 覆盖字体、分页、表格、公式、批注、图片、嵌入对象和损坏输入；
- 原格式 round-trip、导出后重新解析、digest/size、并发 revision 冲突；
- Android/iOS ABI、签名、最低系统、低内存和引擎崩溃隔离；
- Agent apply 必须经过审批、expected revision、engine validation 和 Resource commit。

### 14.6 性能与资源

- 冷/暖启动、Window 切换、WebView first content 与 bridge ready 分别测量；
- 低内存设备只保留当前与一个相邻 warm Surface；
- 图片/PDF 解码像素和 bytes 上限；
- Paired Desktop relay 与 S3 range stream 具备 backpressure，不全量 collect；
- Office 大文档打开、编辑峰值内存、导出时间和取消行为按格式记录；
- 30 分钟 Agent 会话、50 次前后台、100 次 Workspace/Window 切换 soak；
- 电量、网络流量和后台活动纳入 beta gate。

## 15. 任务落点

第一批代码改动建议按以下边界提交，避免一次 PR 同时重写产品、协议和平台 runner：

1. `schemas/` 与 `crates/openmuse-plugin-protocol/`：manifest v2 + compatibility；
2. `packages/openmuse_plugin_sdk/`：target/capability/presentation hints；
3. `distribution/`：desktop/mobile composition 与 lock generator；
4. `packages/openmuse_host_shell/`：从现 Host 抽取 Broker、Window、主题；
5. `packages/openmuse_adaptive_windows/`：纯投影模型与测试；
6. `packages/dsh_runtime_contract/`：connector TCK；
7. `plugins/dsh-agent/` 与 `packages/dsh_*` adapters：先 local 无回归，再 remote；
8. `packages/muse_resource_contract/`：ResourceRef、Authority port、remote delivery 与 provider TCK；
9. `plugins/workspace-local/`、`workspace-cloud/`、`workspace-paired/`：三个 Authority provider；
10. `plugins/workspace-sync/`：change feed、policy、cursor、conflict 与 receipt；
11. `crates/openmuse-storage-contract/`、`openmuse-storage-s3/`、`openmuse-storage-tck/`：S3 ABI 与 Provider gate；
12. Muse Server：Device/Pairing/Workspace Grant、Relay rendezvous、Cloud Resource Authority 与 Vault 接口；
13. `packages/muse_ioffice_adapter/` 与各 format plugin：artifact/capability/commit gate；
14. `app/openmuse_mobile/`：Android 后 iOS；
15. `scripts/`：target build、closure verification、SBOM、Provider matrix 和签名门禁。

## 16. 主要风险与缓解

| 风险 | 影响 | 缓解 |
|---|---|---|
| PlatformView 在滑动/折叠时重建或黑屏 | Agent 状态丢失 | M0 双平台 spike；稳定 identity；必要时 snapshot proxy |
| “同插件”被实现成大量平台 `if` | 长期分叉 | connector/view/resource ports + TCK，core 禁止平台 import |
| Mobile pub graph仍拉入 Desktop 插件 | 体积/审核/构建失败 | 独立 app root + distribution lock + archive scan |
| Remote path 泄漏到 Mobile | 无法打开/越权 | 先迁移 ResourceRef；path 只在 Local adapter 内解析 |
| Server 代码与生产部署漂移 | Client 永远 Binding/黑屏 | M0 真实 contract probe + deploy gate + attachment trace id |
| 弱网和后台使 callback 过期 | 串 Workspace/错误 UI | attachment generation、幂等 close、resume 重验证 |
| Mobile 被需求拉回 Desktop 等价编辑 | 范围失控 | PRD 明确只读/Agent/审批；编辑器单独立项 |
| 旧版代码直接复制破坏 clean-room/provenance | 法律与维护风险 | 先做 provenance；优先复用合同/测试事实，代码逐文件审查 |
| 把同账号误当成 Desktop 文件授权 | 本地数据越权 | device key + 人工配对 + 逐 Workspace grant + TTL/revoke |
| Relay 成为明文中间人或业务真源 | 隐私/可用性边界失控 | outbound-only + E2E channel；relay 只路由，不解析 Resource/DSH payload |
| Mirror 出现双写与 split-brain | 静默覆盖 Office 文档 | 单一 Authority、expected revision、conflict revision，禁止 last-write-wins |
| 将 S3 当目录/数据库 | 重命名、并发与权限不一致 | immutable blob + metadata CAS；ListObjects 不服务产品 UI |
| S3 Provider 方言泄漏 | BYOS 名义兼容、实际锁定 | 自有 Profile/TCK；禁用 Admin API 与 ETag=MD5 假设 |
| RustFS 1.0 成熟度或安全修复不足 | 数据损坏/停机/越权 | stable pin、破坏/恢复/升级测试、90 天 soak，门禁前用成熟认证 Provider |
| 永久 S3 secret 泄漏到 Mobile/插件 | 用户 bucket 失陷 | Vault/OS Credential Store、scoped role、短期 handle、secret scan |
| Office capability 过度声明 | 用户以为已保存但无法导出 | 按格式 admission；无可靠原格式 export 就不注册 edit/commit |
| Provider 迁移删源过早 | 不可恢复的数据丢失 | digest 双读、generation 切换、观察期、二次确认与双侧 receipt |

## 17. 完成定义

Mobile 架构不能以“APK 能启动”作为完成。只有同时满足以下条件才可称为首发闭环：

- Mobile/desktop dependency closure 真实隔离；
- 1/2/3 Window 投影与 Surface 生命周期通过设备矩阵；
- DSH local-sidecar/cloud-remote/paired-desktop 通过同一 connector TCK，Mobile 不含 Node；
- Cloud 与 Paired Desktop 两种 Mobile placement 都通过同一 Workspace/DSH 语义 TCK；
- 同账号发现、设备配对、逐 Workspace grant、撤销和 DesktopOffline 状态形成完整闭环；
- Remote Workspace 全程使用 ResourceRef/handle，无服务器 path 穿透；
- Local Workspace 只有在用户选择 snapshot/mirror/migrate 后才上传，冲突不会静默覆盖；
- OpenMuse Managed、至少一个用户 BYOS 和 Provider 迁移通过 S3 Profile/TCK 与恢复演练；
- RustFS 未通过版本、安全、恢复和 soak 门禁时，不作为唯一生产真源；
- iOffice 各格式只暴露已通过 artifact、view/edit/export 与 corpus 门禁的能力；
- Remote DSH 的身份、placement、presentation、binding、intent receipt、恢复均可观测；
- 当前四个插件有明确 target 决策，未支持插件不注册也不入包；
- Android release signing 与 iOS TestFlight 真机门禁通过；
- SBOM、notices、secret scan、可访问性和崩溃指标达到 PRD 门槛。
