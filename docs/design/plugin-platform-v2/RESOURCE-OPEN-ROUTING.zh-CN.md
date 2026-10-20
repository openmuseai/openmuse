# Desktop 资源打开与插件能力路由

## 当前实现

1. 插件市场从 MCP Registry 获取版本和 MCPB 包地址。安装时校验 SHA-256，写入本机 `mcpb/<插件名>/receipt.json`。收据表示已安装的版本；在线上架状态不替代本机安装状态。
2. Host 启动时遍历安装收据，通过包内标准 MCP `tools/list` 读取工具的 JSON Schema。若文件输入声明了 `contentEncoding: base64` 和 `contentMediaType`，Host 将支持的媒体类型转换成 `OpenMuseEditorContribution`，安装到内存中的 `OpenMusePluginRegistry`。市场内安装完成时也执行同样的发现步骤。第三方 MCPB 无需包含 OpenMuse 私有字段。
3. Workspace 点击文件时传入 `OpenMuseResource`。Host 读取最多 16 KiB 的文件前缀，识别媒体类型，然后由注册表按媒体类型、扩展名和贡献优先级选择编辑器。用户的默认打开方式存放在本机 `settings-v1.json`；菜单中可临时选择其他编辑器。`Open File Viewer` 的 catch-all 贡献只在没有更高优先级的匹配时生效。
4. Desktop 的 PPTX 和 Markdown 视图使用随 Host 构建的 Flutter 适配器。签名 MCPB 决定该能力是否已安装及工具契约；当前 Flutter AOT 运行时不能从下载包加载任意 Dart 代码。其他文件工具可走受控 MCP 调用。

## 本次故障与修复

PPTX 是 ZIP 容器。原文件检查器把 ZIP 签名统一识别为 `application/octet-stream`。注册表在获得确定的媒体类型后以媒体类型为准，因此 `application/octet-stream` 与插件声明的 PPTX 标准 MIME 不匹配，即使文件名是 `.pptx`，仍落到通用预览器。

现对 ZIP 签名且 `.pptx` 扩展名的文件返回 `application/vnd.openxmlformats-officedocument.presentationml.presentation`。其他 ZIP 和伪装成 `.pptx` 的非 ZIP 文件仍不会被识别成 PPTX。线上 MCPB 安装后的能力发现、Workspace 文件检查、注册表选择和 Flutter 渲染分别有回归测试。

## 后续扩展约束

- 能力索引以已安装收据和 MCP 工具 schema 为来源，启动时重建；用户偏好单独持久化。无需把插件可声明的能力复制到可漂移的全局配置文件。
- 当前适配器仅选择单个文件输入工具，且 `extensionForMediaType` 仅覆盖少数媒体类型。若一个包提供多个格式或 `preview`、`edit` 等多个操作，需要将每个工具分别映射为贡献，并在 Host 的操作选择模型里保留相应操作；不得把 OpenMuse 私有字段强加给第三方 MCPB。
- 新增原生 Flutter 编辑器仍需发布兼容的 Host 构建；安装 MCPB 本身不提供可动态链接的 Flutter AOT 代码。
