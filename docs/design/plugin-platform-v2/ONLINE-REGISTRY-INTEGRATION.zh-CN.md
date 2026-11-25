# 在线插件市场接入与资源能力

Desktop 的 `com.openmuse.plugin-market` 与 Mobile 的 `com.openmuse.mobile-plugin-market` 都是 Host 注册表中的 `OpenMusePlugin`，通过 `plugin.market` 和 `mobile.plugin.market` panel 贡献市场内容。入口先查询已注册插件，再显示其页面。内置目录继续供应第一方插件；在线目录来自用户当前选择的 MCP Registry 提供商，默认 `https://plugins.openmuseai.com`。选择保存在各端本机配置中，可替换为其他符合 MCP Registry v0.1 的 HTTPS 服务。

## 第三方边界

第三方仓库只提交标准 MCP Registry `server.json`、MCPB、MCP stdio/Streamable HTTP 与 MCP Apps 的 `resource_link`。客户端不读取第三方自定义市场字段，也不要求第三方导入 Host SDK。Host 的 `OpenMusePlugin` 适配器是客户端内部实现；它通过 MCP 工具输入 JSON Schema 的 `contentEncoding: base64`、`contentMediaType` 推断文件能力，再通过统一资源注册表选择编辑器。具体插件 ID 与客户端版本之间无代码依赖。

`io.openmuse/appflowy-editor` 0.1.4 的 `open_markdown` 声明 `text/markdown`。macOS 下载并校验 MCPB 后运行 stdio 服务，点击 Workspace `.md` 时由资源注册表选择该插件；它返回的标准 HTML `resource_link` 在 OpenMuse 的受控 WKWebView 中展示。Android 安装时验证 Registry SHA-256，并从同一 MCPB 提取 `web/` 静态资产到应用私有目录；插件市场直接在 App 内打开离线 Flutter Web/Wasm 页面。Android Workspace 文件入口通过 Desktop 授权的资源引用读取小型 Markdown 内容，取得用户确认后调用已安装插件的 MCP 工具，并在 App 内 WebView 打开结果。不会把 Desktop 文件路径或配对凭据交给插件。

市场列表约每 20 秒重新读取提供商目录，用户也可手动刷新。审核后台批准的版本出现在公开目录；下架后新用户无法安装，已有本机安装记录仍显示“已下架”并可使用；目录网络不可达时仅显示本机安装记录，不把离线误判为下架。

## 当前限制

- 第三方上传与审核服务仍使用试运行令牌；生产反向代理只公开读取接口，写入和管理后台仅能通过 SSH 隧道访问。公开开发者自助上传需要独立 OIDC、命名空间归属和审核安全扫描。
- Android 从 Workspace 打开 Markdown 当前需在线调用插件的 MCP 工具，且正文限制 32 KiB。插件市场里打开空文档可使用离线资产，不依赖 `127.0.0.1`。
- 编辑器的自动保存发生在应用内 WebView 存储。Workspace 文件被复制进编辑器，但修改不会写回原文件；原资源写入需单独设计带授权和冲突处理的 Host 能力。
