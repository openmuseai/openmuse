# 资源中心 P0：多端共享实现契约

2026-11-02。身份、版本、能力和引用路由属于共享框架；Mobile Home、Desktop 入口和对话卡片都是调用方。既有 Resource v1 descriptor、manifest v2 的严格 wire 未添加隐式字段。

## 模块

| 模块 | 已实现职责 |
| --- | --- |
| `muse_resource_center` | 纯 Dart 身份/版本/引用、Provider、访问约束、最近记录、版本解析与默认打开方式 |
| `muse_resource_center_io` | 管理型文档/历史 blob、只读 Workspace 文件树、配对 RPC、手机反向读取、执行投影 |
| SDK `resource_capabilities.dart` | 可选 handler / creator / context contributor；受限编辑会话和共享资源路由 |
| `openmuse_resource_center` | 共用控制器、目录/菜单、新建、打开方式、版本状态、对话草稿 |
| `openmuse_resource_center_host` | 原生 Host 装配、作用域私有存储、配对连接、导入和生命周期 |
| `openmuse_resource_clipboard` | Android / iOS / macOS 类型化剪贴板和显式读取时的 URI 降级 |
| `plugins/resource-notes` | 通过普通 SDK 贡献 Markdown、纯文本、URL 创建/打开；文本编辑及上下文提取 |

Mobile 与 macOS/Linux 主应用接入相同 NativeResourceCenter。macOS 主应用完成本机构建。Windows/Linux 的 DSH WebView 消息接收使用同一 ResourceLinkRouting，但本轮未在这两种 OS 上运行原生界面。

## 身份、权限与传输

ResourceIdentity 为 `scope/source/workspace/mount/resource`，全部是不透明身份；ResourceVersion 为 `revision + SHA-256 digest`。改名不换身份；同名跨源不合并。

引用序列化为 `openmuse.resource-reference/v1`，文本路由是 `openmuse-resource://v1/<base64url-json>`，上限 8192 字符，可带固定版本与 retained 指针。URI 不含物理路径、cookie、API key 或访问 grant，也不因知道 URI 而获得权限。

Host 为操作产生 ResourceAccess，约束账号、操作、generation、TTL、audience、资源集合和副本读取。既有配对认证将账号、设备、Workspace 与 grant TTL 转成可信 ResourcePeer；请求不能自行声明这些身份。

`openmuse.resource-center/rpc/v1` 提供目录、describe/read、创建/改名/CAS 编辑，以及显式 share、retain、prepare 和手机前台 poll/reply。保留副本的内部源只供精确读取，不出现在普通目录或新建位置。未知旧端点明确报错。

## 插件

插件可实现 OpenMuseResourceHandlerContributor、OpenMuseResourceCreatorContributor、OpenMuseResourceContextContributor；按平台、启用状态、probe、格式和 Provider 权限筛选。停用会立即移除创建项并使默认 handler 失效。

- handler 收到 ResourceEditorSession 的验证字节，通过受限 commit 回调进行 CAS 保存；不接收 Host grant/路径。有效打开后报告 reportOpened 才记最近。
- creator 返回 ResourceSeed，由 Host 在用户选择的位置原子提交；重试使用幂等键，不按 Office/图片写死入口。
- context contributor 提取运行时支持的上下文；没有提取器时保留草稿并明确不支持，不静默上传文件。

本轮有真实 Notes 插件及已有安装插件的旧打开适配。可下载 MCPB 的 manifest v2 没有新增未经协商的 creator 字段；其新建贡献需后续版本化能力协商，不能宣称所有市场插件均已支持新建。图片、视频、Office 等仍由本端实际可用的 handler 打开，格式引擎不是本批新增范围。

## 对话与可用性

添加引用只保存元数据，不读取全文或发送。显式发送时固定版本，并只授权给选定 PC；默认仅引用，“保留对话版本”需用户选择。

PC 校验确切版本，在自己的私有执行目录生成租约文件，再由插件提取上下文。当前 DSH 为文本兼容模式：提取总量 32 KiB，序列化 prompt 上限 56 KiB。长期消息保留 canonical 引用和提取文本，不保留临时路径；尚未实现任意二进制模型附件。

| 状态 | 点击对话引用 |
| --- | --- |
| 在线、无副本 | 按当前授权重新读取固定版本，不用 V2 替代 V1 |
| 在线、有合法副本 | 可打开同摘要的对话版本，区分只读副本与原件 |
| 离线、无副本 | 保留引用，明确提示源不可用 |
| 离线、有合法副本 | 验证后只读打开保留版本；可显式尝试当前原件 |

手机通过当前配对 PC 的前台轮询响应已显式共享的资源，没有额外的 HTTP 文件服务器。PC → 手机使用同一 RPC/Provider 语义。metadata 持久化不代表在线；关闭桥后不能用旧临时路径兜底。

## 边界

- 最近记录按 Host/账号私有保存，最多 200 项、90 天；不新增跨设备最近同步。
- Documents 支持目录、创建、改名、CAS 编辑及历史 blob；导入为应用私有副本，上限 64 MiB。
- 外部物理 Workspace 首期只读、分页、懒加载身份/摘要，拒绝符号链接越界。浏览不下载所有文件，也不改变 Tasks 执行位置。
- 配对单次内容传输上限 16 MiB；超限读取明确失败。
- 执行投影默认 15 分钟；Host 每分钟清理过期租约，重启只清理自己有 marker 的过期目录，不删除源或保留 blob。
- prepare 重试缓存保留最近 100 个完成结果；仅当全部为并发未完成请求时限制新增。关闭期间拒绝迟到结果并清理投影。
- 生产仍要求 HTTPS；验收明确开启的私有网络例外包含 RFC1918 IPv4、ULA IPv6，拒绝公网地址/任意 DNS/重定向扩权。

Cloud 浏览/托管、任意三设备离线转发、外部项目目录写入、批量移动/永久删除以及 Windows/Linux 原生 UI 均不在本批验收完成声明内。
