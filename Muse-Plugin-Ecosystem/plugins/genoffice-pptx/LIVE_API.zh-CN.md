# GenOffice Live Slides v1（0.7.0）

这些工具由用户已打开的 MCP Apps View 通过标准 tools/list 和 tools/call 提供；stdio Server 的 slides_read_snapshot 继续表示磁盘快照，不冒充实时内容。通用 Host 只转发工具定义和 CallToolResult。

| 工具 | 输入 | JSON 结果 |
| --- | --- | --- |
| pptx_get_state | {} | name, revision, dirty, pageCount, slideIndex |
| pptx_read_page | {slideIndex: 非负整数} | name, revision, index, text, notes |
| pptx_go_to_slide | {slideIndex: 非负整数} | slideIndex, revision |

页码从 0 开始。读取当前引擎中的内容，包含已应用但未保存的编辑；导航不修改 dirty/revision、不进入撤销栈。导航使用原始 React setCurrent，由插件构建适配提供回调；不调用 applyDeck，避免伪造编辑。边界越界、尚未打开文档和缺少导航上下文返回 MCP isError。notes 是原版演讲者备注。

旧 pptx_read_deck/edit_text/set_font/set_notes/save/undo 工具不变，见 AGENT_INTEGRATION.zh-CN.md。消费者应发现并检查所需工具，缺少时提示升级，不访问 iframe DOM 或引用内部 slide 节点。

验证：scripts/verify_web.mjs 使用独立标准 MCP Apps Host，遍历 23 页、导航两端、读取第一页、执行编辑/冲突/保存重开。没有提供 reference-png-dir 时，像素对照结果为 null；不能把缺少基准当作零差异。原版引擎与画布未改写，vendor 保持原始源码。
