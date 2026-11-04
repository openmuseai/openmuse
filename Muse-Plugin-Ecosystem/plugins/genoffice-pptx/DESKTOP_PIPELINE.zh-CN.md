# 原版 GenOffice 管线（0.4.0 实现）

本文件替代 9322dd5 的方案阶段描述。0.3.1 的 Flutter OOXML 预览器不再参与本插件的运行；旧代码仅留作历史回归。

## 实际运行链路

```text
MCP Registry → 审核后的 MCPB → MCP tools/list + resources/read
                                      ↓
Host：资源 MIME / 默认打开方式 → 受控 MCP Apps View
                                      ↓ 标准 tool-input（文件 base64 / name）
插件：原版 openPptx → 原版 session-state.buildAllRenderSlides
                    → 原版 React App / Konva / TextEditOverlay
                    → 原版 runTxn / savePptx
                                      ↓ 标准 ui/download-file
Host：仅写入用户授权的文件或导出位置
```

上游仓库作为 git submodule 固定在 `1e643ac74416dbff8a3e75e283bd88b0c1cba5f9`，没有修改上游源文件。`web/src/runtime.ts` 是插件内部的 preload 适配器；原版 `window.slidesApi` 合同不穿过 Host。React App、工具栏、缩略图、画布、文本编辑层和媒体解码均复用原版代码。原版 session builder 的 TIFF、主题 SVG、JPEG EXIF 处理也直接复用。

Electron 相关窗口、磁盘扫描、系统字体目录在浏览器中由插件自己的环境绑定替换。原版的 Electron-free `pptx-ops` 负责编辑，不由 Host 解释节点或 OOXML。当前已接通文本、字体、段落、几何、填充、删除、备注、撤销/恢复与 PPTX 保存；原版菜单中依赖 Electron、系统剪贴板、AI 服务的功能不等于已经完整移植。未经验证的功能不应据此宣传为全功能 GenOffice。

## Host 边界

- MCPB / MCP Registry v0.1、JSON Schema `contentEncoding` / `contentMediaType`。
- MCP Apps：`ui://` 资源、`text/html;profile=mcp-app`、`_meta.ui.resourceUri`、标准 JSON-RPC 初始化、tool-input、download-file。
- 客户端只负责已安装能力发现、资源授权、sandbox、预算和写回；PPTX 路径没有 GenOffice 名称判断、渲染器或业务专属字段。
- macOS：WKWebView 中的沙箱 iframe。外层文档使用不可访问的 `https://mcp-app.invalid/` 作为安全来源，提供 Web Crypto；不是 HTTP 服务。仅外层 frame 可以调用原生文件桥。
- Android：审核包中的离线 HTML 由 WebViewAssetLoader 提供。合成 HTTPS 来源只在 App 内解析；不连接 `127.0.0.1`，不启动 Node。工具输入以临时流式 JSON 提供，避免 20+ MiB HTML/文件在 Java 堆反复复制。
- Mobile 安装时从标准 MCP 发现 UI 和文件 MIME，并核对远端 HTML 与摘要校验后的包内 HTML 完全相同，随后离线使用。查看文档不向市场发送文件。
- Host 给离线 View 加上独立 CSP，禁用外部连接；插件自身 CSP 无法放宽它。

标准参考：[MCP Apps overview](https://apps.extensions.modelcontextprotocol.io/api/documents/overview.html)。

## 验收方法与边界

`Sample_ppt.pptx` 共 23 页。逐页比对必须使用原版 **屏幕 Konva 管线**，不能将 GenOffice CLI 的 PDF→PNG 导出当作完全相同的基准：样本第 4 页在 PDF 导出中透明度不同，原版 GUI 与本插件保持浅色背景。

`scripts/build_reference.mjs` / `web/src/reference.ts` 调用未改动的原版 Desktop session builder（原版系统字体度量）；`reference-ui.tsx` 用原版屏幕画布产生基准 PNG。`scripts/verify_web.mjs` 在独立 MCP Apps 测试 Host 中逐页绘制、比较像素，并真实操作原版文本编辑框、保存、再解析。它不导入 Muse-Client。

“复用同一管线”不代表所有字体跨系统逐像素相同。插件随包提供 OFL Carlito GO、Arimo，其他字体按原版启发式度量和系统回退。样本的几何、图片、裁剪、透明度已恢复；未获授权的商业中文字体不随包分发。应保留逐页差异比例和实机截图，不宣称任意 PPTX 完美或所有菜单功能一致。
