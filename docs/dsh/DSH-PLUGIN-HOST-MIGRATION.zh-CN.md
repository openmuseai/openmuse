# DSH 插件与本地 Host 接缝迁移设计

状态：2026-09-24。范围是桌面 Host、本地 DSH sidecar 与独立 Helix 插件；不移入历史产品的账号、云、数据库或 UI 基座。历史仓库只用于代码审计，不是构建输入。当前固定 DSH `0.1.7-rc.1`，锁文件固定所有 registry integrity。该版本升级了 Session 日志与 profile 设置，旧数据迁移、旧 RPC 与客户端插件必须重新验收。

## 历史插件功能审计

| 历史插件类别 | 实际能力与依赖 | 本地产品决策 |
|---|---|---|
| `dsh-model-capabilities`（MIT） | 扩展 DSH Models 的提供方卡片，写 `llm-pi-ai` 设置；模型模态、思考档位、兼容字段、自定义请求头与会话亲和。Host 半部使用 `settings.mutate`/HTTP，浏览器半部注入 `settings.models.provider-card`。 | 已复制 MIT 源码到本仓库并适配新版 `settings.describe()` 读取，作为默认插件以 `--patch` 挂载。固定包内版本为 `0.5.0-openmuse.3`；无 Key Web 启动、客户端模块图和不存在提供方的路由反例通过。Models 卡片实际写入/重启持久化仍须 UI 门禁。 |
| 工作区绑定浏览器半部 | 在会话头展示绑定 chip、切换 Mount；原实现请求旧产品的绑定/SSE/active HTTP 端点，依赖旧装配层。 | 保留交互语义，重写为 OpenMuse WebView ↔ Host 受控消息和 DSH 官方 `sessions`/`workspaces` 客户端服务；不复制旧端点或设备路径暴露策略。 |
| 资源打开、资源引用、资源呈现 Tool/Host | 对话消息中的文件链接交给 Host；引用 chip 与模型可见块；Tool 呈现资源卡。 | 官方 `@deepseek-ai/dsh-client-ui-chat` 默认调用 `sidebarRight.openResource`，因此截图中落在 DSH 自己的预览区。新第一方 `openmuse-dsh-bridge` 客户端插件仅在嵌入式 Host 桥存在时转发 `dsh-resource://file/session/...` 到 Host；普通浏览器和非文件资源保留 DSH 默认行为。模块图、客户端单元测试已过；真实会话点击仍待 UI 门禁。 |
| 工作区目录、Markdown、View、数据库等旧产品专用插件 | 依赖旧产品 View/数据库类型、账号云路由、只读沙箱和旧装配。 | 不装入 DSH closure。需要目录列表时经 Host capability 重新设计，不直接复用旧产品服务。 |
| 移动端输入/Surface | 面向移动容器而非 NSView/HWND。 | 桌面发行包不安装。 |

## 权属与发行边界

历史 `@muse/*` 私有包没有逐包独立许可证，不能仅凭“自研”直接装入发布包。`dsh-model-capabilities` 有独立 MIT 许可证，可作为第三方组件迁入，但必须保留 LICENSE、版本、来源和 notices。DSH 本身及 npm closure 保持独立进程；Host 不导入其内部状态、数据库或 React UI。

## 接缝协议

1. Host 是本地 Mount 的授权与 active Mount 真源。`workspace.snapshot` 返回 canonical Mount 列表和 active path；`workspace.activateMount` 只接受已挂载的 canonical path，拒绝任意路径。
2. DSH 的第一方 bridge 对 Host 授权路径幂等登记；双向切换以 DSH `workspaceId` 与 Host path 的当前映射为准。Host → DSH 通过独立 client bridge 调用 `uiWorkspace.openWorkspace`；DSH → Host 在 `uiWorkspace.openWorkspace/openSession` 完成后从 Session 快照提取 cwd 并发送 `workspace.activate`。双方比较当前值、序列化登记/去重，避免回环。Bridge 仅允许当前 loopback 主 frame、当前端口，消息体有长度上限。不得从 DOM 文本猜测工作区。
3. 对话中的 `resource.open` 使用同一 WK 消息通道进 Host，Host 对 path/cwd 做 canonical path、授权 Mount 与文件类型校验，再复用普通资源 Tab。必须以真实对话点击和一个越权路径反例验收；静态 JS 合同不足以证明端到端。
4. Helix LS 安装属 Helix 插件，不归 Host。安装清单按语言声明来源、固定版本、校验和、安装根目录；下载需用户显式触发、支持取消/进度，先下载临时文件再校验并原子安装。系统 PATH 与用户自定义可执行文件优先；失败不得修改已生效配置。对未固定来源的 LS 禁止“一键安装”。
5. 版本比较前先调用当前编辑器插件的 `flush(resource)`，收到确认才从磁盘读取工作副本。Helix 经 PTY 发送 `:write` 并等待独立完成标记；超时或写入失败要阻止比较，不能把旧磁盘内容标为“当前”。历史版本对历史版本无需 flush。
6. DSH 对话文字复制：仅在嵌入式 WKWebView 主 frame 且存在非输入框文字选区时，拦截 Cmd/Ctrl+C 并写本机剪贴板。无选区和可编辑区域必须保留原行为。选区右键卡片始终有“复制选中内容”；只有字符串像绝对/相对文件路径或文件名时才出现“复制路径”。这仅复制用户选中的字符串，不据此授予文件访问权限，也不从正文猜测活动 Workspace。需真实对话、输入框、跨工作区历史消息和右键交互验收。

## 迁移顺序和工程门禁

- 先扩充 SDK 的可选 flush 能力和 Host 调用链，测试脏 buffer → 保存 → Diff，以及 flush 失败不打开 Diff。
- 再实现新的 DSH client bridge、Host 授权命令与双向去重；在真实 WebView 中验证 Host/DSH 双向切换。
- 对话文件点击用真实 DSH 事件、当前工作区内文件及符号链接越权反例测试。
- `dsh-model-capabilities` 已随固定 DSH 以本地 patch 默认挂载，不依赖旧产品装配包；还需在 Models 卡片实测设置写入、重启持久化、插件卸载回退。插件的 wire 层请求头改写需要单独审查并发会话安全，不应仅凭启动成功视为验证通过。
- LS 安装清单与校验、跨平台分发是独立门禁；未验证的网络安装器不能随发布包启用。
- 最后执行 Flutter/Node 测试、无外部仓库隔离打包、macOS 签名/ZIP 检验，并单独验证 Windows。

当前实现与未完成项以功能矩阵为准。设计通过不等于相应运行时门禁已通过。

## 复制桥当前实现

本仓库第一方 `openmuse-dsh-bridge` `0.1.5` 在客户端捕获真实选区的复制/右键动作；macOS WKWebView 同时在 document start 注册提前的 Cmd+C 监听，避免依赖 DSH 客户端插件的加载顺序。WKScriptMessageHandler 检查当前 loopback 端口、主 frame、消息长度和 `clipboard.write` 类型后写入 `NSPasteboard`。JS 单测覆盖文字选区、无选区、输入框及文件名菜单；Swift Release 编译通过。尚未在真实对话视图通过 Cmd+C/右键的端到端实机门禁；Windows WebView 剪贴板适配仍缺。

## 版本升级实测差异

旧的 `POST /api/workspace.create` / `insertBefore` 在 `0.1.7-rc.1` 返回 404。新第一方 DSH Host 插件使用进程私有随机 token 保护 `/openmuse-bridge/workspaces`，直接以 DSH `workspaceRegistry` 幂等登记 Host 授权目录，不删除 DSH 自建 Workspace。无 Key 真实 sidecar 集成测试已验证新增目录可登记，缺 token 请求被拒绝。当前选中 Mount 的双向消息桥也已接入：Host 先登记再通知 DSH `uiWorkspace`；DSH 选择 Workspace 时经当前端口主 frame 的 WK 消息回传，Host 只接受已挂载的精确 canonical path，重复值去重。新版对话文件点击默认调用 DSH 自身 `sidebarRight.openResource`；新客户端桥仅转发文件资源到 Host，路径仍由 Host canonical Mount 校验。**当前选中工作区和对话文件点击的真实 WebView UI 端到端门禁尚未通过**，不能仅凭客户端单测宣称完全对齐。旧插件的 `settings.get` 已移除；本仓库副本改用 `settings.describe()`。升级时不得直接用实际用户的旧 DSH_HOME 进行破坏性试验，应先在复制的测试 Home 验证 Session 日志和设置导入。
