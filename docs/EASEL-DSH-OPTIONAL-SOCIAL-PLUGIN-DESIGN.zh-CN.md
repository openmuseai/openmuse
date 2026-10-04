# Easel 能力复用：DSH 驱动的可选社媒插件设计

状态：技术方案建议，待 PoC 验证（2026-10-05，按通用 Mobile 预览/控制协议修订）  
目标：复用 Easel 的素材加工、平台适配、社媒登录和发布能力；OpenClaw 完全退出运行链路；Easel 业务插件不随 OpenMuse 默认发行，在 Desktop 运行，Mobile 通过默认安装的 [`remote-workbench`](MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md) 操控流水线。

## 1. 结论与边界

**有条件可行。** Easel 的 Python 媒体与发布脚本可作为 Desktop 执行体；DSH 负责对话、技能选择、内容适配和工具编排；Mobile 默认安装的通用插件负责发现、预览、交互、确认和状态恢复。Easel 只提供 Desktop 业务插件，不包含 Mobile 代码。当前仓库已经具备 DSH Desktop sidecar、配对 Desktop 的远程 DSH 页面、Manifest v2、资源与 Broker 的协议/参考实现，但尚未具备通用远程 surface 协议、运行时插件装载器、生产级资源物化/授权接线及社媒业务 API。

首版限定：一台在线的 macOS Desktop；小红书图文；已有 Desktop 登录态；Mobile 选素材、看预览、确认发布、看结果。验证通过后再做跨端首次登录、抖音视频和更多平台。Windows 可在平台脚本与浏览器 profile 验证后接入。Desktop 离线时 Mobile 可看已缓存状态，但不能执行发布。

这个限定来自三个实际约束：

- Easel 的脚本使用本地路径、持久化浏览器 profile 和 Playwright，必须在 Desktop 运行；`asset-manager/scripts/assets.py` 也把 `outputs/INDEX.json` 与路径作为索引真源。
- Flutter Plugin SDK 的 `install()` / `uninstall()` 管理的是已构造的 Dart 对象；Desktop 和 Mobile 当前都在应用代码里创建/安装插件，Manifest v2 目前主要是声明、校验和 resolver，不能据此认为任意 Flutter 代码已能运行时下载并执行。通用 `remote-workbench` 必须随 Mobile 包预装一次，之后业务插件只下发协议数据；Flutter 官方延迟组件无法替代这个方案。[Flutter deferred components](https://docs.flutter.dev/perf/deferred-components)
- Mobile 现有 `executeHostCommand` 是空实现，远程 DSH 页面是 WebView；配对网关代理 DSH HTTP/WebSocket，并没有社媒业务命令或执行侧的社媒授权协议。

## 2. 对现有调研的修正

[`third_party/Easel/EASEL-PLUGIN-INTEGRATION.zh-CN.md`](../third_party/Easel/EASEL-PLUGIN-INTEGRATION.zh-CN.md) 适合当资产目录和问题清单，不应直接作为实施依据：

1. **许可证事实需修正。** Easel 根 `LICENSE` 和 README 标注 Apache-2.0，而旧调研称 AGPL-3.0。Easel 中技能又有 `EASEL-META.md` 记录外部来源；实际要复用的每个文件仍需单独核对来源/许可。仓库里的 DSH bridge 有自己的 AGPL-3.0-only 标识，不能把它混同于 Easel 根许可。
2. **`publish_dispatch.py` 只是派发计划。** 它检查标题/正文/标签数量和内容类型并给出 warning；没有实际渲染预览、素材媒体探测、账号状态检查，也没有强制阻断全部 warning。产品预览需要新建结构化 `PreviewReport` 并由目标 publisher 做最后校验。
3. **`ResourceRef` 与 Policy 已有契约，但未全量落到现有 Host/paired 路径。** C2 的 Policy/Audit 当前是内存参考 provider；C3 有参考 Resource Authority；不能把这些协议存在等同于可用的生产级 `materialize → publish` 链路。
4. **业务插件无 Mobile artifact；通用控制插件默认安装。** 如果把 Easel Flutter UI 编译进 Mobile，即使不激活，代码也已随安装包分发。Mobile 应只预装通用组件/媒体/Web 预览器；Easel 下发结构化界面和媒体 handle，可选提供受限 Web 页面。业务 UI 不必依赖网页，可优先由 Mobile 原生组件渲染。

## 3. 建议架构

```text
Mobile OpenMuse（默认 remote-workbench：通用组件 / 图片 / 视频 / 受限 Web）
   │ 发现 + remote-surface/control v1 + 媒体数据面
   │ Paired Desktop / Relay（actor/device/workspace grant）
   ▼
Desktop Host（插件目录、Surface Registry、Broker、资源访问、审计）
   │ 按需安装/启动 com.openmuse.easel-social
   ├─ remote-surface provider（素材、预览、账号、发布状态、可执行动作）
   ├─ DSH Adapter（注册少量结构化 social.* 工具）◀─ DSH Agent
   └─ Easel Worker（独立 Python 进程）
       ├─ 素材加工：按需选择 Easel 脚本 / FFmpeg / Pillow
       ├─ 预检：publish_dispatch + 平台特定校验 + 渲染缩略图
       ├─ 授权：各平台 login/whoami，profile 留 Desktop
       └─ 发布：content_guard + 各平台 publisher + readback
```

### 3.1 一个可选 Desktop 业务包，Mobile 只安装通用插件

建议插件 ID `com.openmuse.easel-social`，Manifest v2 仅把经过验证的 Desktop target 设为 supported，Mobile target 显式 unsupported（原因是无本地执行 artifact）；`presentation.remote_capable` 描述 Desktop 插件可通过受控远程 surface 呈现，**不代表 Mobile 安装了该插件**。另在 `contributes.services` 声明 `openmuse.remote-surface` v1；运行时 service 返回可用 surface、结构化控件、媒体 handle 和 action schema。Mobile 的 `com.openmuse.remote-workbench` 是另一个默认安装插件，支持多个业务插件。Easel 包包含：

| 部分 | 用途 | 生命周期 |
| --- | --- | --- |
| Desktop worker | Python 运行环境、经挑选的 Easel 脚本、浏览器依赖 | 仅安装/激活插件时出现；停用后退出 |
| DSH adapter | 暴露 `social.assets/prepare/preview/accounts/authorize/publish/status` 等结构化工具，并加载改写后的少量技能 | 按插件启用状态向 DSH 注册；停用时撤销；必要时重启 DSH sidecar，不重启 OpenMuse |
| remote-surface provider | 素材网格、发布步骤、预览媒体、账号挑战、进度和动作描述 | Desktop 插件提供协议数据；Mobile 通用插件渲染 |
| 可选 Web bundle | 平台发布网页的只读 snapshot 或专门适配的响应式预览 | 由 Host 提供受限虚拟 origin；控制动作仍走协议 |

Host 的通用装载能力需要增量开发：从**用户主动安装的目录**发现 Manifest v2、核对精确 target 与 artifact digest/signature、建立按插件命名空间的持久数据目录、注册/撤销 surface 和 service、按需 spawn/stop worker。应用的默认 distribution closure 不包含 Easel 运行时、Easel 脚本、社媒 UI 或浏览器 profile。纯 `registry.install()` 无法满足这一点。插件升级与卸载要撤销工具、拒绝新任务、等待/取消运行中任务、终止进程；账号 profile 删除必须单独明确选择，避免误删用户登录态。

### 3.2 DSH 取代 OpenClaw

不运行 `easel gateway`、`easel chat/skill`、`web/app.py`、`openclaw/sync.sh` 或 OpenClaw profile；`easel/gateway_questions.py`、OpenClaw 的 session key、端口与 workspace symlink 都不进入发行包。DSH 是唯一 Agent runtime。

从 Easel 的 SKILL 中挑选 `asset-manager`、媒体加工、跨平台预览和首个平台 publisher 的**业务规则**，改写为 DSH 可加载的技能/工具说明。替换 `skills/openclaw/...` 命令路径、AGENTS.md 假设、输出目录和平台 publisher 的自然语言委派方式。用户画像作为每个 DSH 会话/任务的显式上下文输入，避免全局文件在并发会话中互相覆盖。

DSH 负责“理解需求、生成平台草案、选择工具”；所有有副作用的动作仍由结构化工具和 Worker 决定。DSH adapter 不把任意 shell、Python 路径、cookie 或 `--exec` 暴露给 Agent。Adapter 在 DSH 进程内应尽量薄；Easel 脚本和浏览器运行在独立 Worker，故 Worker 崩溃不会带走 DSH。仍需 PoC 验证 DSH 当前版本的工具动态注册/卸载接口以及插件停用时的行为；若 DSH 需要重启以更新工具清单，只重启其 sidecar。

## 4. 流水线与数据合同

业务协议建议 `openmuse.social/v1`，由通用 [`remote-surface/control/v1`](MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md) 承载，与本地/配对传输解耦。社媒 surface 输出通用组件树：素材 `gallery`、平台选择 `form`、预览 `image/video-player + compare`、授权 `stepper + challenge form`、发布 `confirmation + progress`。业务状态和媒体由 Desktop 权威保存，Mobile 只持 opaque ref。所有控制请求有 session、generation、actionId、input schema、expected revision、idempotency key；事件含递增 `seq`、`jobRef`、时间和明确状态，断线后按 cursor 补齐。

| 阶段 | 输入/输出 | 执行者 |
| --- | --- | --- |
| 素材选择与加工 | `ResourceRef + Revision` → `AssetSetRef` 与新产物 revision；配方记录裁剪/尺寸/封面 | Desktop Worker，经资源数据面取得临时副本 |
| 平台草案 | `AssetSetRef + platform + accountRef? + 文案` → `DraftRef` | DSH 生成建议，Worker 校验与渲染 |
| 预览 | `DraftRef` → `PreviewRef`、逐平台 warning/error、实际媒体缩略图、`fingerprint` | Worker，UI 展示；编辑后重新预览 |
| 授权 | `platform` → `AuthSessionRef`、challenge、`AccountRef`/状态 | Worker 持有 profile，Mobile 只传 challenge 响应 |
| 提交发布 | `PreviewRef + fingerprint + AccountRef + 用户确认令牌` → `JobRef` | Desktop Host 校权，Worker 真发 |
| 回读 | `JobRef` → `published/failed/outcome_unknown`、平台内容 ID/URL（如可得） | Worker + 平台 readback |

资源跨边界只传 Ref/Revision。最后一跳由 Desktop Host 为 Worker 签发限定 audience、TTL、固定 revision 的 materialization handle，并在 Worker 私有临时目录落副本；加工产物经资源 provider commit 后返回新 Ref。Worker 不能接收 Mobile 提供的绝对路径。初版可以只支持 Desktop Workspace 中已能经 Host 授权解析的资源；Cloud/Paired Resource provider 在通过 C3 数据面 TCK 后接入。

`publish_dispatch.py` 的平台约束表可作为初始数据源，但 `PreviewReport` 必须增加媒体实际宽高/时长/大小、平台能力、账号有效性、平台脚本的最终校验及明确的 blocking status。每个平台生成一份不可变草案；用户在 Mobile 改标题或素材会产生新 `DraftRef/PreviewRef`，旧确认令牌立即失效。产品预览是内容与约束的模拟，不能承诺平台真实发布页面 100% 一致。

## 5. 授权、发布与故障处理

### 5.1 社媒授权

`AccountRef` 只指向 Desktop 插件私有 profile/credential；Mobile、DSH 和 Web UI 均拿不到 cookie、AppSecret 或 profile 路径。一个账号一个独立 profile/锁，`whoami` 定期验证有效性；授权、失效、撤销与 profile 清理分别有明确状态。

平台登录方式不能统一假设“手机上显示二维码即可扫码”：用户只有这一部手机时，通常无法用同一台设备扫描自己的二维码。首版使用预先在 Desktop 完成的登录态；后续逐平台选择官方 OAuth/设备授权（若平台提供）、Desktop 上显示 QR 由手机扫、或经人工确认的远程浏览器交互。若某平台只提供扫码且用户无第二显示设备，首次授权在 Mobile 独立完成的体验**不可保证**；UI 应说明所需设备，不进入无解等待。抖音短信挑战可由 Mobile 输入，敏感验证码仅发给 Worker 且不落审计正文。

### 5.2 真发门禁

把 `plan/preview` 和 `publish.commit` 设成不同命令。`publish.commit` 必须校验 actor/device、workspace、插件与 Worker grant、目标账号、当前预览 fingerprint、资源 revision、一次性确认令牌；执行前在 Worker 再跑 `content_guard.py`，失败即中止。拒绝 DSH 通过普通 shell 绕过此入口。现有 C2 Broker 有权限交集、handle 和审计 reference 实现，但生产 Policy provider、持久审计以及 Desktop Host 命令接线仍需实现。

发布幂等只由**持久 Job ledger** 保证“相同提交请求不会在本系统重复执行”。不能仅用内容 revision + 平台 + 账号作 key：用户可能有意发布同一内容两次；应由一次用户确认生成唯一 submission ID。平台发布动作一般不提供可靠幂等语义；Worker 在点击后断线或超时时标记 `outcome_unknown`，先 readback/人工核查，**不得自动重试真发**。多平台任务逐平台记录结果，成功一半不能回滚成全失败。

配对通道的现有网关面向 DSH 代理。通用 remote-surface gateway 必须按插件/surface/action 路由并分别授权；Web 预览经受限虚拟 origin，不能直接反代任意 Desktop `localhost` 网页；图片/视频经短期 media handle、Range/HLS 流出，不把 Worker loopback 端口或社媒 cookie 暴露给 Mobile。Mobile 不因登录 OpenMuse 账号而自动拥有任一社媒账号发布权。

### 5.3 隔离与停用

插件停用或故障时：Host/DSH 仍可运行；入口和工具消失；新任务拒绝；运行任务被安全取消或进入可恢复状态；没有其他插件依赖社媒 Worker。进程有 CPU/内存/磁盘/并发上限，浏览器 profile 与临时目录限权；失败日志默认脱敏。Python 依赖和浏览器由插件包自己的锁文件及版本管理，不调用 Easel 的全量 `setup.sh` 修改用户环境。平台选择器变化只影响对应 publisher，并提供独立健康检查。

## 6. 实施顺序与验收门槛

1. **运行时装载 PoC**：无插件启动；用户安装包后发现/验签/启用；停用和卸载后清除 surface、工具与进程；默认安装包和 Mobile 包中无 Easel artifact。DSH 只加载被选技能，确认可动态注册或界定 sidecar 重启行为。
2. **Desktop 垂直切片**：先选小红书图文，打通 `ResourceRef → 临时副本 → 加工 → 预览 → 已登录账号 → 人工确认 → 真发 → 回读`。Worker 使用 Easel 原脚本的受控 wrapper，记录需修改的路径和输出约定；不接 Easel Web/CLI/OpenClaw。
3. **通用 Mobile 控制面**：先交付并默认安装 `remote-workbench`，用 fake 业务插件验证原生组件、图片、视频、受限网页、动作、事件恢复和断线重连；然后 Easel 只提供 Desktop remote-surface service，验证**不重新安装 APK**即可在手机端出现社媒流水线并完成预览、确认和查结果。此阶段可使用 Desktop 预授权账号。
4. **授权与平台扩展**：逐平台实现 challenge 状态机、账号多开/撤销、抖音视频、其他平台；每个平台分别验收登录、预检、发布、回读和失效恢复。
5. **生产门禁**：持久 Policy/Audit、发行 artifact 与依赖 SBOM、逐文件许可审查、平台发布条件审查、故障演练。无这些门禁时只用于隔离试验环境和测试账号。

最小验收场景：插件不存在时 Host/Mobile/DSH 功能与启动正常；启用后 Desktop 可完成一次受控发布；Mobile 在配对 Desktop 上可完成同一流水线；Mobile 断线重连不重复发；插件停用后命令不可调用且 Worker 退出；`outcome_unknown` 不自动重试；越权 workspace/account/旧 preview 被拒绝并留审计 receipt。

通用协议还要由第二个领域验证：仅在 Desktop 安装一个最小视频编辑插件，Mobile 在**同一安装包**中发现它，播放 Desktop 生成的视频代理、提交 trim/reorder、看到渲染进度。这样才能证明 Mobile 插件确实可复用，而非只为 Easel 定制。

## 7. 需要先做的技术验证

- DSH 0.1.7-rc.1 的工具/技能注册是否支持无进程重启更新；已有 bridge 证明可注册 Web 路由，但不能据此推断动态工具 API 已满足要求。
- 可选插件包在 Desktop 的签名、安装、卸载、版本兼容和 Sidecar 进程管理实现；Manifest v2 只是必要合同。
- Desktop 实际 Workspace controller 与 C3 Resource Authority 的适配，以及配对链路对资源缩略图/事件流的吞吐与访问控制。
- Easel 的小红书脚本在独立 profile、独立输出目录、固定 Python 依赖下是否通过真机登录/发布/回读；平台规则和选择器可能变化。
- 逐文件许可与第三方依赖检查；Easel 根许可为 Apache-2.0，不能据此推定所有 SKILL 和脚本来源均相同。

关键代码依据：[`packages/openmuse_plugin_sdk/lib/openmuse_plugin_sdk.dart`](../packages/openmuse_plugin_sdk/lib/openmuse_plugin_sdk.dart)、[`app/openmuse_host/lib/main.dart`](../app/openmuse_host/lib/main.dart)、[`app/openmuse_mobile/lib/main.dart`](../app/openmuse_mobile/lib/main.dart)、[`plugins/dsh-agent/lib/src/dsh_sidecar.dart`](../plugins/dsh-agent/lib/src/dsh_sidecar.dart)、[`plugins/workspace-paired/lib/src/paired_desktop_gateway.dart`](../plugins/workspace-paired/lib/src/paired_desktop_gateway.dart)、[`third_party/Easel/skills/openclaw/skill-cross-platform-publish/scripts/publish_dispatch.py`](../third_party/Easel/skills/openclaw/skill-cross-platform-publish/scripts/publish_dispatch.py)。
