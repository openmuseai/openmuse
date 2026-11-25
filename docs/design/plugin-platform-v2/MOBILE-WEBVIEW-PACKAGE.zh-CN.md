# Mobile WebView 插件包 v1

状态：2026-10-15 已实现基础格式。与 [设计基线](DESIGN-SPEC.zh-CN.md)、[开发计划](IMPLEMENTATION-PLAN.zh-CN.md) 配套。

## 领域与运行边界

- **Catalog** 持有身份、版本、编辑媒体与可安装 artifact 的 SHA-256。远端目录采用 Ed25519 签名、过期时间、单调序号和同源 HTTPS；内置目录随 App 发版。
- **Installation** 由 Mobile Host 的 `MobilePluginStore` 负责。安装前检查包摘要、manifest V2 身份/版本、目标 OS、运行时、Web 文档摘要、权限一致性及 A2UI 卡片白名单。安装状态按设备保存，更新可替换版本，停用/移除同步到统一插件 registry。
- **Workbench** 只接收已验证的 A2UI v0.9 消息，使用共享 `MobileWidgetCatalog` 渲染 Flutter 卡片。下载包不能创建 Flutter Widget。
- **Detail** 在 Android WebView / iOS WKWebView 打开完整 HTML。Host 注入 CSP、当前主题和仅属于本插件的消息通道；外域导航、网络连接、表单提交、iframe 和对象嵌入被拒绝。
- **Capability** 下载包 v1 只允许空 `requested_permissions`。`OpenMuseWebViewInstalledPlugin` 的 Host 桥默认拒绝业务方法。定时任务与 Workspace 文件是同一 registry 的预置插件，它们的领域操作经配对 grant 和 Desktop 服务授权。第三方权限授权、撤销和审计是后续独立阶段。
- **Legacy** QuickJS/A2UI 包仍可安装，供旧版 PPTX 插件迁移；新演示插件采用 WebView/A2UI。

## 包格式

`openmuse-mobile-web/v1` 是单个 UTF-8 JSON 文件，最大 1 MiB：

```json
{
  "format": "openmuse-mobile-web/v1",
  "uiProtocol": "a2ui/v0.9",
  "id": "com.openmuse.mobile.task-board",
  "version": "0.2.0",
  "html": "<!doctype html><html><head>...</head><body>...</body></html>",
  "card": {"title": "任务看板", "summary": "本地演示", "columns": 1, "height": "medium"},
  "preview": [
    {"version": "v0.9", "createSurface": {"surfaceId": "main", "catalogId": "https://a2ui.org/specification/v0_9/catalogs/basic/catalog.json"}},
    {"version": "v0.9", "updateComponents": {"surfaceId": "main", "components": [
      {"id": "root", "component": "Column", "children": ["line"]},
      {"id": "line", "component": "Text", "text": "本地任务清单"}
    ]}}
  ],
  "manifest": {"manifest_version": 2, "ui_runtime": {"kind": "web-view", "entrypoint": "index.html"}, "execution_connector": {"kind": "none"}}
}
```

示例仅展示关键字段；完整 manifest 必须通过 SDK V2 模型验证。`artifacts` 中每个受支持 Mobile target 使用 `web-bundle`，其 SHA-256 对应 `html` 的 UTF-8 字节；目录项摘要对应整个包的原始字节。目录的 `runtime` 和 `permissions` 若存在，必须与包一致。卡片只允许共享目录中的 Column、Row、Text、Divider、Card，最多 8 个消息、24 个组件和 16 KiB；详情 HTML 最多 512 KiB。

## 发布与验收

运行 `python3 scripts/build_mobile_market.py` 可重复生成 PPTX 兼容包、两个 WebView 演示包及内置目录；脚本的结果与仓库文件字节一致。远端发布必须使用独立签名流水线和 HTTPS 同源 artifact，不得把示例脚本视为远端签名工具。验收包含：签名目录与篡改拒绝、安装/启停/更新/移除/恢复、A2UI 卡片、平台 WebView 交互、Host 桥拒绝、配对领域服务链路。当前设备与帐号结果见 [验收矩阵](ACCEPTANCE-MATRIX.zh-CN.md)。
