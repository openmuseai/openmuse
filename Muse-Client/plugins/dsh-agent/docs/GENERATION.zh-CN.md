# DSH Generation v1

DSH 插件通过通用 Tool Contributor 暴露标准 MCP Tool/JSON Schema；第三方通过公开 Resource Extensions v1 的 list_plugin_tools / call_plugin_tool 使用，不读取本插件源码、端口、bridge token、Workspace ID 或 DSH session ID。

| 工具 | 参数 | 返回 |
| --- | --- | --- |
| dsh_generate | text（1–32000 字符），可选 conversationRef | 不透明 taskRef、conversationRef、status |
| dsh_generation_status | taskRef | running/completed/failed/cancelled，text，可选 error，phase |
| dsh_generation_cancel | taskRef | 取消后的任务状态；无法确认取消返回 failed + GENERATION_CANCEL_UNCONFIRMED |

结果为 MCP CallToolResult，同时含 structuredContent 与 JSON 文本。generate 接收回执不是模型完成；消费者必须轮询到 completed 且非空输出，才能宣布讲解成功或翻页。实际输入、模型输出和取消状态保存在同一个可见 DSH 会话。首次生成创建会话，后续传 conversationRef 复用；引用只对相同调用者和 Workspace 有效。

每调用者同时最多一项任务，总任务历史最多 128；三分钟超时取消。关闭资源或 Host 调用者通过通用 disposeToolCaller 撤销任务；插件自行处理私有 native session/prompt/follow/cancel 适配。首个 follow snapshot 建立后才提交 prompt，忽略旧快照中的完成事件，只在本轮 turn/start 后接收输出与 turn/end。真实模型失败、流断开、无输出都不能冒充完成。

测试 dsh_generation_test.dart 使用真实 HTTP/SSE 边界覆盖顺序、会话复用、任务归属、并发拒绝和取消。真实模型及原生对话验收见生态仓库 Document Explainer 的证据目录；HTTP 夹具本身不是模型 E2E。
