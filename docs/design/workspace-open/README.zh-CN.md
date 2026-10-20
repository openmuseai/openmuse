# Mobile Workspace 打开进度与商店入口（2026-10-20）

## 结论与实测

已在连接的 OPPO Android 真机，以原有登录帐号、原有已安装 GenOffice PPTX 0.5.0，从 Workspace 打开 `Sample_ppt.pptx`（5,541,885 字节）。真实下载进度可见；在应用内显示正文及 23 页缩略图；返回后可重新打开。最终完整 APK 使用 `lib/main.dart`，通过 `adb install -r` 更新，保留帐号和安装记录。

| Android 最终包阶段 | 耗时 |
| --- | ---: |
| 建立 HTTP 连接 | 126 ms |
| 等待 Desktop 响应头（包含读取、往返） | 159 ms |
| 接收文件 | 15,137 ms |
| 匹配本机已安装插件 | 25 ms |
| 准备本地编辑器 | 167 ms |
| 点击至交给编辑器 | 15,614 ms |

这次主要耗时在网络接收。渲染成功单独通过真机截图确认；没有把协议初始化或页面加载完成当作 PPTX 渲染完成，也没有记录精确的解析／绘制时间。本次没有修改 PPTX 引擎、发布或覆盖 0.5.0 插件。

修改前后同一开发机经生产中继下载的对比：响应头 5.818 s → 0.288 s；总传输 20.683 s → 21.060 s。流式传输改善首字节和进度可见性，网络带宽限制仍存在。这组数字与真机数字为不同终端的独立测量。

## 已修复的链路问题

1. 原 Mobile 请求在连接、响应头和读取空闲期均只有 12 秒，且异常会被通用详情页面改写为“检查 Desktop 连接”。现在 Workspace 读取器按阶段生成安全错误；授权失效、资源引用失效、等待超时和传输中断分别显示，可重试。
2. 原生产中继缓存整个非 SSE 响应，最大仅 8 MiB，而资源接口支持 64 MiB。资源下载现以标准 HTTP 流式转发，限制统一为 64 MiB；超时或上游失败不会发送成功结束标记。SSE 和配对接口保持原有路径。
3. Desktop 转发原来丢弃 `Content-Length`。现在未解压的响应保留标准长度；解压后的响应不沿用压缩长度。旧 Desktop 没有长度时使用 chunked，进度条保持不确定状态并显示真实接收量。
4. 原文件点击等待启动时的远程插件能力补全。现在补全在后台运行，打开只查询本机安装记录、能力快照和已审核 UI，不因市场服务慢而阻塞。
5. Mobile 读取改为 `BytesBuilder(copy:false)`，避免大型 `List<int>` 扩容和最后复制。连接／空闲限制为 45 秒，总上限为 5 分钟；超限会关闭请求，不无限等待。

截图中那次历史失败没有底层异常记录，无法证明它只由某一个缺陷触发；上述问题均由代码确认，修复后的打开、重试已在真机完成。

## 责任边界

- Workspace 文件插件负责进度、计时、失败文案、重试和并发打开限制。页面以 250 ms 间隔读取插件实例的 `openingProgress`；仅传递步骤、字节数和耗时。
- Host 的资源打开回调报告通用的匹配／准备／启动步骤，按资源 MIME 和已安装插件能力分发；没有新增 PPTX 插件 ID、业务字段或渲染逻辑。
- 文件下载完成 100% 不表示绘制完成。进入编辑器后使用编辑器自身的“正在打开”状态；内容解析与绘制仍在资源插件内部。
- 诊断日志只记录阶段和相对时间，不记录文件内容、授权凭据、用户路径或资源引用。

## MCP 的角色与商店展示

MCP 不是产品分类，也不是所有插件统一必需的运行时。

| 层次 | 当前用途 | 是否等于在线渲染依赖 |
| --- | --- | --- |
| MCP Registry | 标准商店发现／安装元数据适配器，可换提供商 | 否 |
| MCP tools/resources | 能力发现及 Agent 工具调用；可有本地或远程服务 | 取决于具体工具 |
| MCP Apps | 沙箱视图与 Host 的标准消息接口 | 否，视图通道使用本机 postMessage |
| Mobile 本机编辑器 | 已审核安装包 UI + 已下载资源 | 当前 PPTX／文档打开不调用远程 MCP 渲染 |
| Desktop MCPB | 由客户端管理的本地 Node/MCP 服务、工具和视图 | 依赖本机服务，不能描述成完全不依赖 MCP |

官方说明：[Registry](https://modelcontextprotocol.io/registry/about)、[MCP Apps 架构](https://apps.extensions.modelcontextprotocol.io/api/documents/overview.html)。当前离线安装、MIME 能力快照属于 OpenMuse 的分发／缓存实现，不能把它们说成 MCP 标准保证的离线执行能力。

发现页面现在直接展示线上插件卡片，支持同页搜索和安装／打开；“我的”展示本机安装记录。发现隐藏已经下架的版本，“我的”保留下架安装并标明状态。提供商配置与刷新保留。Registry 未给出产品分类时展示在精选和搜索，不按 MCP、运行时或传输方式推断为办公／编程等分类。

## 服务部署与回滚

生产 `openmuse-device-relay` 已部署流式修复并确认 active，Desktop 自动重连。服务代码仍位于 `Muse-Server/device-relay/relay_server.py`；本目录保存可审查的增量补丁和流式测试副本，未另建一个重复的服务。

- 增量补丁：`DEVICE-RELAY-STREAM.patch`
- 流式测试：`test_resource_stream.py`，在 device-relay 目录下运行 `python3 -m unittest test_resource_stream`。
- 生产备份：`/opt/openmuse-device-relay/relay_server.py.before-workspace-stream`；恢复该文件后重启原 systemd 服务可回滚。
- 重启会重建内存中的短期配对状态；客户端需重新连接 Desktop。

## 验证与交付

- Mobile 传输／分发测试 4 项通过，覆盖首块提前到达、准确字节数、授权失效、阶段超时和大文件交给 Host。
- 统一线上列表测试 1 项通过，覆盖同页展示及“我的”保留下架安装。
- 已安装插件断网重开测试 1 项通过。
- 中继新增 3 项通过：9 MiB 文件、首块提前到达、旧 Desktop chunked、失败不伪装成功；原中继 3 项通过。
- Mobile、插件市场、paired workspace 分析均无问题；完整 macOS／Android 构建的 analyze、签名／打包门禁通过。
- macOS `dist/OpenMuse.app` 和 ZIP 均为新版，已运行并登录；Android 真机安装最终 APK。
- 构建包含工作区原有未提交的 UI 工作。本轮提交按补丁拆分，没有把这些已有改动混入提交。

证据与详细数字见 `evidence/acceptance.json`、`evidence/android-opening-stages.txt`、真机截图。安装包：`dist/OpenMuse-macos.zip`、`dist/OpenMuse-Android-arm64.apk`；哈希：`dist/SHA256SUMS-workspace-open.txt`。
