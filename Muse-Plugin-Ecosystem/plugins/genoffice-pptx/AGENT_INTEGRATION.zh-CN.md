# GenOffice PPTX 0.5.0：资源工具与 Agent

## 产品变化

删除原版 GenSpark 对话面板和 AI 工具组，保留原版 GenOffice 的幻灯片导航、Ribbon 和 React/Konva 编辑器。工具栏的 `Agent` 按钮发送标准 MCP Apps `ui/message`，由 Host 的 Agent 插件接管对话。没有 Agent 适配器的 Host 会明确提示不可用；普通渲染、手动编辑和保存继续可用。

构建适配在 `scripts/build_web.mjs`、`web/src/agent-panel.tsx`、`web/src/agent-button.tsx` 中完成。`vendor/genoffice` 保持原始提交；不改写 PPTX 解析或画布管线。

## 第三方协议边界

插件仅依赖 `@modelcontextprotocol/ext-apps`，没有 Muse-Client 包依赖，也不知道 DSH 的端口、会话、Workspace ID 或认证字段。

- `App({…}, {tools: {listChanged: false}})` 声明 View 工具。
- `onlisttools` 返回普通 MCP Tool/JSON Schema。
- `oncalltool` 接收标准 `tools/call` 并返回 `CallToolResult`。
- `updateModelContext` 推送当前幻灯片、选中元素、文档版本、dirty 状态。
- `sendMessage` 请求 Host 中的 Agent。
- `downloadFile` 交给 Host 保存文件并等待回执。

Host 的通用资源路由工具不是插件的必选依赖。其他标准 MCP Apps Host 可直接发现并调用 View 工具，无需模拟 OpenMuse 私有字段。独立测试 `scripts/verify_web.mjs` 即证明这一点。

## 文档工具

| 工具 | 输入摘要 | 结果 |
| --- | --- | --- |
| `pptx_read_deck` | 可选 `slideIndex`（从 0 开始） | revision、dirty、当前选择、元素 ID 与文本 |
| `pptx_edit_text` | slideIndex、elementId、text、expectedRevision | 保留原版格式的文本事务，返回新版本 |
| `pptx_set_font` | slideIndex、elementId、font、expectedRevision | 颜色/字号/粗体/斜体事务 |
| `pptx_set_notes` | slideIndex、text、expectedRevision | 演讲者备注事务 |
| `pptx_undo` | expectedRevision | 撤销上一次修改，返回新版本 |
| `pptx_save` | expectedRevision | Host 确认保存后返回 saved: true |

全部业务规则和字段都在 `web/src/runtime.ts` 中。编辑使用原版 `@genoffice/pptx-ops.runTxn`，刷新通过原版 `onDeckChanged`，不重开文档。Agent 与手动编辑共享撤销栈。

### 调用顺序

1. 发现当前资源及其贡献的工具。
2. `pptx_read_deck`，读取真实 elementId 与 revision，不能猜 ID。
3. 按返回的 JSON Schema 进行修改，并携带 expectedRevision。
4. 每次成功修改使用返回的新 revision；发生冲突时重新读取。
5. 通过 `pptx_save` 保存。收到 isError 或 saved 未确认时不能宣称成功。
6. 用 `pptx_read_deck` 检查内容；必要时从磁盘重新打开验证保存结果。

示例：

```json
{"name":"pptx_edit_text","arguments":{"slideIndex":0,"elementId":"sp_5","text":"季度经营汇报","expectedRevision":1}}
```

这里的 ID 仅为示意，实际必须从读工具获取。

## 构建、发布与验证

沿用 [开发指南](DEVELOPMENT.zh-CN.md) 的独立打包、上传、审核流程。每次更新提升版本；不要覆盖已审核版本。

```sh
npm ci
npm test
npm run pack:desktop
python3 scripts/stage_service.py dist/genoffice-pptx-0.5.0.mcpb
MCP_APP_HTML=dist/release-service/dist/web/index.html \
  node scripts/verify_web.mjs dist/sample.pptx dist/verification-reviewed-050 /path/to/original-canvas
```

**服务端 UI 必须来自 `stage_service.py` 提取的已审核 MCPB，不能在再次构建后直接同步 dist/web。** 后者可能产生不同 HTML 字节，移动端会正确拒绝校验不一致的页面。

上传、审核脚本分别为 `scripts/publish.py` 与 `scripts/review.py`；令牌通过环境或交互输入。MCP 服务继续独立部署，Agent 编辑调用在本地 View 完成，不上传 PPTX 到生产市场。

## 平台范围与证据

macOS 实现 Host 的标准 View 工具通道与 DSH Agent MCP 适配。Android 仍使用相同离线原版编辑器；Android Agent 的原生桥接不在本阶段完成范围。没有相应能力的 Host 不应宣告 message/context 支持。

23 页原版画布对照、人工编辑保存重开、Agent 工具修改/保存/撤销与冲突测试记录见 `evidence/0.5.0`。对照使用冻结样本，避免用户原始样本被其他应用修改后产生错误比较。真实 DSH 对话验收另行记录，不用独立测试 Host 的通过冒充。
