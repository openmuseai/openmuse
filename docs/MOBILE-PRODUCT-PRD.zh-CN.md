# OpenMuse Mobile 产品设计

状态：提案 v2（2026-09-29）
范围：Android / iOS；先定义产品与验收，不把 Desktop UI 等比例缩小到手机。

## 1. 决策摘要

OpenMuse Mobile 定位为 **Agent-first 的随身工作台**：用户在离开桌面时查看 Workspace、阅读资源、与 DSH 协作、审批动作并把重任务交回 Desktop。Desktop 则是 **local-first 的完整创作与执行工作台**，拥有本地文件、进程、PTY、Helix 和可自由切分的多窗布局。

首发产品决策如下：

1. Mobile 不内嵌 Node、DSH sidecar、Helix 或任意本地进程运行时；DSH 必须运行在远端执行 Host。
2. DSH Agent 在产品上仍是同一个插件 `com.openmuse.dsh-agent`。Local/Remote 是它选择的 execution connector，不是两套业务插件、两套消息或两套 UI。
3. Mobile V1 使用 OpenMuse 账号发现用户自己的 Workspace 与设备，因此 Mobile 需要登录。Desktop 本地模式仍可免登录；只有“从 Mobile 连接 Desktop”时，Desktop 才需要登录同一账号并显式授权。
4. Mobile V1 同时支持两类 Workspace placement：运行在 Cloud 的 Workspace，以及位于用户某台 Desktop 的 Local Workspace。相同账号只是发现和身份前提，不自动授予文件权限；首次连接仍需设备配对和逐 Workspace grant。
5. 普通手机一次只显示一个 Window；大屏、平板和折叠屏根据可用尺寸显示两个或三个 Window。尺寸变化只改变投影，不销毁逻辑 Window 或会话。
6. V1 不做系统画中画或浮动窗，但布局协议保留 overlay placement，避免未来再次更换 Surface 身份模型。
7. Mobile 只编入明确支持 Mobile 的插件。Helix 和 Native Text Gate 不进入 APK/IPA，不能仅在运行时隐藏。
8. 用户可以把 Local Workspace 保持为 local-only，也可以通过 Workspace Sync Plugin 上传为只读快照、云端镜像或迁移成 Cloud Workspace。同步策略必须显式选择，不能因登录自动上传本地文件。
9. S3 是 Cloud Workspace 的可替换 Storage ABI。用户可用 OpenMuse Managed Storage、自己的 S3-compatible Provider，或 Desktop-mediated BYOS；Office、DSH 和 UI 不绑定 RustFS/MinIO。

## 2. 依据与现状

[旧版 Android 构建说明](../../Muse-Clients-Deprecated/frontend/client/doc/android/README.md)与[旧版 host-agnostic Mobile DSH 包](../../Muse-Clients-Deprecated/middlewares/dsh/mobile/muse-dsh-mobile/README.md)已证明以下路径可行：

- Flutter APK 可以在不携带 Node/DSH sidecar 的情况下工作；
- Remote DSH 可通过 `session/open` 获得运行时 URL，在 WebView 中展示；
- HTTPS 上行与 SSE 下行可承载 Host/DSH 协作；
- 编译期公开 URL 只能作为 origin allowlist，不能当作带 token 的页面地址；
- 页面加载、DSH session ready、Host bridge ready 是三个不同状态，不能用一次 WebView `onPageFinished` 代替。

新架构又提供了两个可复用基础：

- Host 已把内容抽象为稳定 Surface/Window binding，插件视图不必等同于 OS 窗口；
- DSH、Helix、Viewer 已经是独立插件，产品装配层可以选择发行版的插件闭包。

但当前实现仍是 Desktop-only：默认布局会在窄宽度继续压缩三个 Pane；DSH 插件只会启动本地 sidecar；资源打开消息仍传本地 `path/cwd`；应用依赖闭包会同时引入 Helix、桌面 DSH WebView 和 Native Text Gate。这些都不是 Mobile 可发布状态。

## 3. 产品定位：Mobile 不是什么

### 3.1 Desktop 与 Mobile 的产品分工

| 维度 | Desktop | Mobile |
|---|---|---|
| 核心心智 | 本地创作、执行、深度编辑 | 随时查看、询问、审批、接力 |
| Workspace 权威 | 本地 Project Workspace / Mount；可选同步到 Cloud | Cloud Workspace Authority，或已配对 Desktop 的代理 Authority |
| DSH 位置 | 本机 Node sidecar | Cloud Workspace 用 managed Remote DSH；Desktop Workspace 使用该 Desktop 上的 local DSH |
| 代码编辑 | Helix + PTY + LSP | V1 只读查看；轻量编辑后续独立插件 |
| 多任务 | 任意 split/move/swap/resize | 1/2/3 槽响应式投影，快速切窗 |
| 连接要求 | 核心本地能力可离线 | Agent 和远端资源依赖网络 |
| 用户决策 | 可直接执行本地命令 | 高风险动作以审批/拒绝/稍后处理为主 |
| 插件生态 | Built-in + 未来 External Runtime | V1 仅签名内置插件 |

### 3.2 Workspace placement

Mobile 的 Workspace 选择器把数据位置作为一等信息，不把所有 Workspace 混成一个“云端列表”：

| Placement | 数据真源 | DSH 执行位置 | 可用条件 | 典型体验 |
|---|---|---|---|---|
| Cloud | Cloud Resource Authority + 用户选择的 S3 Provider | OpenMuse managed Remote DSH | 登录、网络、Workspace 成员权限 | Desktop 不在线也可查看、询问和审批 |
| Desktop | 用户指定 Desktop 的 Local Workspace | 同一台 Desktop 的 local DSH | 同账号、已配对、Desktop 在线、Workspace grant 有效 | 访问真实本地文件与本机工具，不先上传整个 Workspace |
| Cloud Mirror | Local Workspace 与 Cloud Workspace 各有 revision，经同步策略协调 | 用户选择 Cloud 或 Desktop placement | Sync 健康、冲突已处理 | 本地编辑、移动端持续访问 |
| Cloud Snapshot | Desktop 发布的不可变/只读快照 | Cloud Remote DSH 只读或无 Agent 写权限 | 最近一次上传成功 | Desktop 离线时阅读和审阅 |

产品必须始终显示：当前 Workspace 名称、数据真源、DSH 执行设备/区域、最近同步时间和只读/可写状态。用户从 Cloud 切到 Desktop 时不是静默换 endpoint，而是切换一个带独立 generation 的 Workspace session。

### 3.3 数据自主不是“可填 S3 URL”

用户应拥有以下能力：

- 新建 Cloud Workspace 时选择 OpenMuse Managed Storage 或 BYOS；
- 连接测试明确显示 Provider、bucket/prefix、可用能力和不兼容项；
- 随时导出 Workspace manifest、Office 原件、revision/digest 清单；
- 将 Workspace 从一个通过认证的 S3 Provider 迁移到另一个 Provider；
- 撤销凭据、停止同步、删除云端副本并获得 receipt；
- 明确知道 OpenMuse/DSH 是否可以读取明文；BYOS 不等于端到端加密。

官方托管实现优先演进到 RustFS，但产品文案只承诺 OpenMuse Managed Storage SLA，不向用户泄漏或绑定底层实现。详细选型见 [S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md](S3-STORAGE-TECHNICAL-SELECTION.zh-CN.md)。

### 3.4 Mobile V1 不承诺

- 不在 iOS/Android 启动 Node、任意 shell、PTY、Helix 或 Language Server；
- 不把远端文件路径伪装成本机路径，也不提供 Finder/Explorer 的 reveal 语义；
- 不提供 Desktop 等价的自由分割、拖拽 resize、外部插件动态装载；
- 不在离线时伪装 DSH 可用；离线只展示有界缓存与待同步状态；
- 不在 V1 支持系统画中画、悬浮球或多 OS window；
- 不承诺任意大文件完整下载到内存；
- 不承诺 Desktop 离线时仍能访问一个从未上传/发布过的 Local Workspace；
- 不因用户登录同一账号而自动扫描、上传或开放 Desktop 文件；
- 不把 S3 当作文件树、协作数据库或跨文件事务系统。

## 4. 目标用户与核心任务

### 4.1 目标用户

1. 已有 Cloud Workspace，或希望从手机安全连接自己 Desktop Workspace 的用户。
2. 需要快速阅读代码、Markdown、图片、PDF、Diff 或任务结果，但不打算进行长时间键盘编辑的用户。
3. 需要审核 Agent 计划、工具调用、文件修改建议和权限请求的负责人。

### 4.2 Jobs to be done

- “我想立即知道 Agent 做到哪里、为什么停住、是否需要我决定。”
- “我想从 Workspace 找到一个资源并交给 Agent 解释或处理。”
- “我想打开 Agent 引用的文件/位置，在手机上阅读上下文。”
- “我想批准、拒绝或修改一项高风险动作。”
- “这项工作需要完整终端/编辑器时，我想一键发送到 Desktop 继续。”
- “我的 Office 文件要放在我选择的 S3，并且未来能完整迁出。”
- “我想决定哪些本地 Workspace 永不上传，哪些只做快照，哪些保持云端镜像。”

## 5. 信息架构

Mobile 保留逻辑 Window，而不是为每个屏幕尺寸制作不同业务页面。V1 有三个一级 Window：

| Window | 职责 | V1 默认能力 |
|---|---|---|
| Workspace | 选择 Cloud/Desktop placement，浏览、搜索、查看同步与设备状态 | Cloud tree、Desktop proxy tree、刷新、打开资源 |
| Content | 查看当前资源、Diff、附件和只读预览 | Markdown、文本、常见图片；PDF 取决于平台门禁 |
| Agent | DSH 会话、计划、工具结果、审批和输入 | Remote DSH Web surface + 原生能力桥 |

设置、连接管理和账户属于导航目的地，不长期占用一个 Workbench Window。

默认进入策略：

- 有未处理审批或 Agent 需要输入：进入 Agent；
- 从通知深链进入：打开目标 Window 和资源；
- 普通冷启动：恢复上次 Workspace 与 active Window；
- 首次启动：登录，选择 Cloud Workspace 或配对 Desktop，随后进入对应 Workspace。

### 5.1 Workspace 与 Storage 设置

Workspace 详情页分开呈现三个概念：

- **位置：**Cloud、某台 Desktop、Cloud Mirror 或 Cloud Snapshot；
- **存储：**OpenMuse Managed、用户 BYOS Provider，或 Desktop local-only；
- **执行：**DSH 在 OpenMuse Cloud 还是某台 Desktop。

三者不能合并成一个“服务器”下拉框。例如，Cloud Workspace 可使用用户自己的 S3；Desktop Workspace 可把 DSH 留在 Desktop，同时向 Cloud 发布只读快照。

BYOS 设置不直接暴露底层永久 secret 给 Mobile。用户在受控流程中创建 scoped role/key，服务端或 Desktop Credential Store 保存凭据；Mobile 只看到 provider 状态、capability 和最近验证时间。

## 6. 响应式 Window 体验

### 6.1 不是设备名单，而是容量求解

以安全区扣除后的可用逻辑宽度、显示分段和每个 Surface 的最小尺寸计算容量。以下断点是初始产品基线，不是硬编码设备型号：

| Size class | 初始宽度 | 可见槽位 | 默认投影 |
|---|---:|---:|---|
| Compact | `< 600dp` | 1 | 当前 active Window 全屏 |
| Medium | `600–959dp` | 2 | Workspace + Content，或 Content + Agent |
| Expanded | `>= 960dp` | 3 | Workspace + Content + Agent |

若铰链/折痕把屏幕分成两个 display feature segment，求解器先按 segment 放置，再在单个 segment 内判断能否继续拆分。即使总宽度足够，也不得让文字、输入框或主要操作跨越不可用铰链区域。

高度不足时优先保持单窗完整体验，而不是强行展示两个不可操作的窄窗。横屏手机只有在两个目标 Window 都满足最小尺寸时才进入双窗。

### 6.2 Compact：单窗 Window Deck

- 顶部显示 Workspace/连接状态；底部显示最多三个一级 Window 入口。
- 左右边缘滑动切换相邻 Window；顶部标题旁也提供明确的上一窗/下一窗按钮。
- 从屏幕边缘起始的手势才触发切窗，正文内部横向滚动、图片缩放、代码选择和 DSH WebView 手势优先。
- 切换 Window 不等于关闭 Surface。相邻 Window 可保持 warm；其余 Window 进入 suspended，恢复时使用插件自己的 session/snapshot。
- 返回键顺序：先交给当前 Surface 历史，再关闭临时 sheet，再返回上一个 Window，最后退出当前 Workspace。

### 6.3 Medium：主从双窗

- 默认组合由当前任务决定：从 Workspace 选资源后显示 `Workspace | Content`；在 Content 中询问 Agent 后显示 `Content | Agent`。
- 用户可以用 Window switcher 替换左/右槽内容，但 V1 不提供任意 split tree 编辑。
- 折叠屏跨铰链时，一个 Window 对应一个 segment；铰链位置变化只重新投影。
- 焦点 Window 拥有系统键盘、语义返回和主要工具栏；另一 Window 仍可接收点击激活。

### 6.4 Expanded：三窗工作台

- 展示 `Workspace | Content | Agent`，比例以各 Surface 的 min/ideal width 求解；
- 可折叠 Workspace 或 Agent，但折叠只是可见性偏好，不注销插件；
- 平板外接键盘时启用 Window 切换快捷键，但不暴露 Desktop 专属命令；
- V1 仍不提供任意递归切分。自由布局是 Desktop 特性，不是所有大屏都必须复制。

### 6.5 旋转、折叠与状态保持

`activeWindowRef`、每个 Surface 的业务 session、滚动位置恢复凭据与草稿独立保存。以下变化不得改写用户的逻辑布局或重复创建 DSH session：

- 竖屏/横屏旋转；
- 折叠屏展开、半折、合拢；
- 分屏/Stage Manager 改变应用宽度；
- 系统字体缩放和安全区变化。

V1 不做 PIP，但 Window placement 枚举预留 `overlay`；任何插件都不得自行创建悬浮层绕开 Host 的焦点、权限和生命周期管理。

## 7. 关键用户流程

### 7.1 首次启动

1. 用户登录 OpenMuse Cloud。
2. 客户端列出 Cloud Workspace 与同账号在线 Desktop，但不自动显示未授权的本地目录。
3. 用户选择 Cloud Workspace，或选择 Desktop 并完成配对/Workspace grant；Host 创建对应的 Mobile Workspace session。
4. 用户首次打开 Agent 时，DSH 插件根据 placement 连接 managed Remote DSH 或该 Desktop 的 local DSH。
5. 页面可见且 Host bridge 完成绑定后，Agent 才显示“工作区已连接”。

不能把“WebView 页面已加载”显示成“Agent 已连接”。等待、排队、鉴权、页面加载和 bridge 绑定必须分别可见。

### 7.2 连接自己的 Desktop Workspace

1. Desktop 与 Mobile 登录同一账号，Desktop 主动登记设备公钥和在线 presence；
2. Mobile 发起配对，Desktop 展示设备名、账号和一次性 challenge，由用户确认；
3. Desktop 用户从本地 Workspace 列表中逐项授权 read、propose、apply 等能力及有效期；
4. Desktop 建立 outbound relay/tunnel，Mobile 不要求家庭网络开放入站端口；
5. Mobile 收到 `WorkspaceDescriptor` 与 capability snapshot，资源仍由 Desktop Resource Authority 管理；
6. DSH 在 Desktop 本地运行，Mobile 只承载远程 presentation/control/data handles；
7. Desktop 离线、休眠或 grant 撤销时，Mobile 明确进入 `DesktopOffline`/`GrantRevoked`，不回退到同名 Cloud Workspace。

同账号不能替代显式配对，配对也不能一次授权该电脑上的所有目录。

### 7.3 从 Workspace 到 Agent

1. 用户在 Workspace 选择资源。
2. Content Window 使用 `ResourceRef` 打开只读视图。
3. 用户点击“询问 Agent”；Host 创建带 revision 与 anchor 的最小 ContextProjection。
4. DSH 只看到获授权的资源引用和片段，不得到任意本地路径。
5. Agent 产生打开/提案/审批请求时，通过同一 Host Broker 回到对应 Window。

### 7.4 Agent 请求打开资源

1. DSH 发出 `resource.open(ResourceRef, anchor)`；
2. Mobile Host 验证 workspace、revision、权限和可用 renderer；
3. 有本地 Mobile renderer 时在 Content Window 打开；
4. 无 renderer 时展示“在 Desktop 打开”或安全下载，不伪装成功；
5. Host 返回 receipt，过期 generation 的响应被丢弃。

### 7.5 Local Workspace 上传到 Cloud

用户在 Desktop 的 Workspace 设置中选择：

- `local-only`：不上传；Mobile 只能在 Desktop 在线时访问；
- `snapshot`：手动/定时发布不可变只读快照；
- `mirror`：持续同步可编辑 revision；冲突按 Resource 类型处理；
- `migrate`：完成全量校验后把 Cloud 设为新真源，本地变为缓存/checkout。

Workspace Sync Plugin 先扫描变化、计算 digest、上传 immutable blobs，再提交 Cloud metadata revision。上传成功但 metadata commit 失败不能显示同步成功。首次启用必须显示预计对象数、字节量、Storage Provider 和明文可见性。

### 7.6 BYOS 与 Provider 迁移

1. 用户选择 Server-mediated 或 Desktop-mediated BYOS；
2. 系统验证 endpoint、TLS、bucket/prefix、最小权限和 S3 Profile；
3. 连接测试完成 put/head/range/get/delete/multipart，不只测试登录；
4. 新 Workspace 开始使用该 Provider，已有 Workspace 进入可暂停/续传的迁移任务；
5. 每个对象按 SHA-256 校验，切换 provider generation 后进入观察期；
6. 用户确认后才删除旧 Provider 数据，并收到两侧 receipt。

### 7.7 弱网、后台与恢复

- 网络断开：保留最后可见的只读内容并明确标记“离线副本”；Agent 输入不可发送时保留本地草稿。
- 应用进入后台：暂停 WebView 非必要工作，停止高频 context 推送，保留远端 session heartbeat 的平台允许策略。
- 回到前台：先检查 token/session generation，再恢复 bridge；不得短暂显示上一个 Workspace 的内容。
- session 过期：尝试无损重连；只有重新鉴权或 scope 变化才回到 Workspace 选择。

## 8. DSH 产品状态

Agent Window 至少展示以下状态，错误不能全部塌成黑屏或“正在启动”：

| 状态 | 用户信息 | 主要动作 |
|---|---|---|
| NeedAuth | 需要登录 | 去登录 |
| NeedWorkspace | 尚未选择 Workspace | 选择 Workspace |
| Pairing | 正在与 Desktop 建立可信设备关系 | 在 Desktop 确认/取消 |
| DesktopOffline | 目标 Desktop 不在线或休眠 | 等待上线/选择 Cloud Workspace |
| Placing | 正在分配 Cloud 运行时或连接 Desktop DSH | 等待/取消 |
| Queued | 排队位置与预计重试 | 后台等待/取消 |
| LoadingView | 正在加载 Agent 页面 | 重试 |
| Binding | 正在连接 Workspace | 查看连接详情 |
| Ready | Agent 与 Workspace 已连接 | 正常使用 |
| Degraded | 页面可用但 Workspace 联动受限 | 重连联动/继续纯聊天 |
| Offline | 无网络 | 查看缓存/稍后重试 |
| StorageUnavailable | Workspace metadata 可用但对象存储不可用 | 查看缓存/重试/检查 Provider |
| SyncConflict | Local 与 Cloud revision 冲突 | 比较、保留一方或另存副本 |
| GrantRevoked | Desktop 或 Workspace 授权已撤销 | 重新申请/离开 Workspace |
| Failed | 明确错误码和可恢复动作 | 重试/反馈 |

用户可在连接详情看到执行位置（OpenMuse Cloud 或具体 Desktop）、数据真源、Storage Provider 类别、Workspace 名称、最近心跳/同步时间和隐私说明，但看不到 token、S3 secret 或内部 URL。

## 9. Mobile 原生能力

V1 可通过 Host capability bridge 向 DSH 提供：

- 相机/照片选择；
- 系统文件选择器返回的受限 URI；
- 分享 sheet；
- 语音输入（平台与权限允许时）；
- 前后台、网络、主题、locale 与安全区状态。

文件 bytes、Cloud JWT、设备 token、本机绝对路径和 `content://` URI 不进入普通 JS 消息。文件上传通过 Host 管理的短期 handle/stream 完成。每项能力单独授权、可撤销、带 request id、deadline 与审计事件。

## 10. 安全与隐私

1. Remote DSH Web URL 必须来自已鉴权的 `session/open`，且 origin/path 通过 allowlist；编译期 URL 不能携带 token。
2. Cloud access token 只用于 Cloud API；DSH collaboration 使用短期 device token，模型 Key 不返回 Mobile。
3. launch token、JWT、device token、授权 URL query 不写日志、埋点、Crash report 或剪贴板。
4. WebView 禁止任意导航、任意新窗口和非 allowlist 下载；TLS 错误 fail closed。
5. Workspace 切换必须递增 generation、撤销旧 handle 并清理旧 WebView 可见内容。
6. 高风险 Agent action 必须显示目标、影响范围、执行位置和一次性确认；Mobile 的确认不能自动扩大 Desktop 权限。
7. 同账号只用于设备发现；真正的数据访问还要求设备密钥证明、显式 Workspace grant、短期 capability 和可撤销审计记录。
8. Office/DSH 插件只能通过 Resource Authority 取得 bytes handle；不得接收 S3 endpoint、bucket 或 credential。

## 11. 插件可见体验

- 只显示当前发行版中实际存在且 capability 条件满足的插件贡献；
- 不显示“Helix”然后在点击后报“不支持移动端”；
- Viewer 不支持某格式时给出明确 fallback：下载、系统打开或发往 Desktop；
- Mobile 插件设置只展示 Mobile 有效项，不能出现 Node 路径、PTY、LSP 本地路径等 Desktop 设置；
- 插件崩溃只替换对应 Window 的错误面，不结束整个 App；
- Workspace Sync、Cloud Workspace Provider、Desktop Bridge、S3 Storage Provider 和 iOffice Engine 分别作为独立插件/服务贡献；其中任何一个都不能 import 另一个的实现。

## 12. 成功指标与发布验收

### 12.1 产品指标

- Agent Window warm start p95 ≤ 1.5s，冷分配时间单独统计；
- 已分配实例的 `Placing → Ready` p95 ≤ 8s；排队不计入该指标但必须可观测；
- Window 切换视觉响应 p95 ≤ 150ms，不因切窗新建 DSH session；
- Remote session 重连成功率 ≥ 99%；
- Mobile crash-free sessions ≥ 99.7%；
- 100% 高风险动作有显式 receipt；
- APK/IPA 中 Node、`hx`、Desktop native plugin 和 Desktop DSH closure 数量为 0；
- 配对 Desktop 的未授权 Workspace 暴露数量为 0；grant 撤销后新请求拒绝率为 100%；
- Cloud 保存返回成功的 Office revision，其 blob digest 验证成功率为 100%；
- 任何通过认证的 S3 Provider 都可导出并在另一 Provider 恢复，digest mismatch 为 0。

### 12.2 设备验收矩阵

至少覆盖：

- 360dp 与 412dp 宽 Android 手机；
- 小屏与 Max 尺寸 iPhone；
- 600–840dp Android 平板/折叠屏模拟配置；
- 展开/合拢、横竖屏、系统分屏；
- iPad 11 英寸级窗口宽度；
- 1.0x/1.3x/最大辅助字体；
- 触控、软键盘、硬件键盘；
- Wi-Fi/蜂窝切换、断网、后台 5 分钟、token 过期。

## 13. 版本路线

| 版本 | 产品范围 |
|---|---|
| M0 / Internal | Android；登录、Cloud Workspace、单窗、Remote DSH、假数据/测试环境 |
| M1 / Alpha | Android；1/2/3 槽投影、真实 bridge、配对 Desktop Workspace、Markdown/图片、审批 |
| M2 / Beta | Android + iOS；BYOS Preview、local snapshot/mirror、PDF/Office view 门禁、弱网恢复 |
| M3 | Provider 迁移与可验证导出、RustFS managed canary、Office edit 按引擎导出门禁逐项开启 |

## 14. 已确定与暂缓的产品问题

已确定：

- V1 支持 Cloud Workspace 与显式配对的 Desktop Workspace；
- 同账号不等于自动访问，Desktop 配对与 Workspace grant 是硬门；
- Local Workspace 默认 local-only，上传/快照/镜像/迁移均需用户显式选择；
- S3 是 Storage ABI，RustFS/MinIO 是 Provider，不进入 Office/DSH 领域合同；
- V1 Mobile 为只读/Agent/审批产品，不承诺 Desktop 等价编辑；
- Window 通过响应式投影适配 1/2/3 槽，不维护三套页面；
- Mobile 插件为编译期白名单闭包。

暂缓：

- `remote-desktop` 是否在后续增加局域网直连优化；V1 先用 outbound relay 保证网络可达性，端到端通道语义不变；
- PDF 使用系统 renderer 还是统一跨端 renderer；
- iPad 是否在 M2 开放有限的手动双窗组合；
- 是否需要独立轻量文本编辑插件，以及它与 Agent proposal/commit 的冲突策略；
- Client E2EE Workspace 如何向远端 DSH 临时授权解密能力。
