# Mobile DSH 原生对话流：方案与实现

状态：按 2026-10-10 仓库实现整理。官方 DSH 对话样式的目标是完全兼容；当前代码是可在产品链路使用的纵向切片，不是 Desktop 对话面的完整复刻。  
目标读者：OpenMuse Mobile、DSH Native Gateway、Flutter 对话渲染。

本文描述 `app/openmuse_mobile` 里已经落地的 Flutter 对话流：它如何消费 DSH Session 事件、官方样式做到哪一层、插件定制 UI 为什么只能有限兼容。更早的开发计划与验收矩阵见 [`DSH-NATIVE-FLUTTER-CONVERSATION-ARCHITECTURE-PLAN.zh-CN.md`](../../../docs/DSH-NATIVE-FLUTTER-CONVERSATION-ARCHITECTURE-PLAN.zh-CN.md)。对话以外的业务插件预览与控制见 [`MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md`](../../../docs/MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md)；那是另一条协议，不负责渲染 DSH 对话事件。

## 1. 决策

Mobile 对话的主路径是原生 Flutter，不是把 DSH Web 整页嵌进 WebView。

1. **官方对话事件尽量用内置 Widget 表达。** 用户消息、助手正文、思考过程、工具调用、失败、用户提问、工作区产物、模型与权限，都由 Flutter 按事件类型投影。目标是与 DSH `0.1.7-rc.1` 官方 UI 的语义、交互和样式一致。像素级跨引擎一致不作为承诺；一致的定义沿用架构计划里的语义、交互、布局 token 和视觉阈值四层。
2. **插件不能把 React、CSS 或脚本下发到 Flutter 执行。** 定制对话 UI 只能描述一棵由 Mobile 预置组件组成的数据树。这是类 Composer 的做法：Flutter 拥有组件表，插件只提供节点、绑定和 command 引用。
3. **未知元素不抢走整段会话。** 协议主版本或 Native Gateway 合同不匹配时，才整面退回 WebView。单个未知事件或未适配插件保持原生时间线，在对应元素上提示，并由用户显式打开 Web 兼容模式。
4. **DSH 仍是唯一对话权威源。** Mobile 不保存第二份聊天记录。Desktop Web 与 Flutter 连接同一个 Desktop DSH runtime。

```text
Desktop DSH runtime
        │  Session journal / follow / command
        ▼
openmuse-dsh-bridge  Native Gateway
        │  /openmuse-native/v1/*
        ▼
muse_dsh_conversation_protocol
        │
        ▼
muse_dsh_conversation_core        纯 Dart：journal → 行投影
        │
        ▼
muse_dsh_conversation_flutter     内置官方 Widget + 组件表
        │
        ▼
WorkBuddy 壳（产品路径）或独立 NativeDshPage
```

## 2. 和远程插件 Surface 协议的关系

两条协议都是“宿主定义组件、对端只提交数据”，但解决的问题不同，组件表也不能互换。

| | DSH 对话流（本文） | 远程插件 Surface |
| --- | --- | --- |
| 协议 | `openmuse.dsh-conversation` 传输 + `openmuse.nativeConversation@1` 声明式 UI | `openmuse.remote-surface/v1`、`openmuse.remote-control/v1` |
| 渲染什么 | 同一个 DSH Session 的对话事件 | Desktop 业务插件贡献的预览、控制和状态 |
| 谁定义组件 | `DshNativeComponentRegistry` 与内置对话 Widget | Mobile 预装的 remote-surface 组件 |
| 插件代码 | 不下载 Dart / JS / CSS | 同样不下载，业务插件没有 Mobile artifact |
| 当前产品入口 | WorkBuddy 任务页里的 `NativeDshPage` | WorkBuddy 抽屉里的远程工作台入口，不是对话时间线 |

远程 Surface 的 V1 组件是 `text`、`markdown`、`image`、`gallery`、`video-player`、`document`、`link`、`list`、`grid`、`form`、`stepper`、`progress`、`status`、`diff`、`compare`、`timeline-basic`、`button`、`confirmation`。对话组件表见第 6 节。一个插件不能把 remote-surface 节点塞进对话 slot，也不能把 `nativeConversation` contribution 当成工作台 surface。

类 Composer 在对话流里的具体含义：

```text
插件 package.json
  openmuse.nativeConversation.contributions[]
        │  Gateway 校验 schema、绑定、深度和 command 形态
        ▼
POST /openmuse-native/v1/negotiate
        │  对照 Flutter 实际上报的 slot / component
        ▼
DshNativeContribution（数据，不是 Widget 类名）
        ▼
DshNativeComponentRegistry.buildNode
        ▼
Text / Chip / Progress / Button / ExpansionTile / Row / Column
```

插件不能提交 Flutter 类名、Widget 路径、CSS、绝对定位、任意字体或系统 API 名称。command 只是字符串引用，manifest 本身不授予执行权。

## 3. 运行时链路

产品路径在 `WorkBuddyShell`。配对 Desktop 的历史会话放进 `IndexedStack`，每一页是嵌入的 `NativeDshPage`：`embedded: true`、`showComposer: false`。时间线来自 `DshConversationSurface`，底部输入框来自 WorkBuddy 自己的 Composer。独立打开 `NativeDshPage` 时才会显示对话库自带的 Header 和 Composer。

打开一个会话的顺序：

1. `DshNativeGatewayClient.initialize()` 请求 `GET /openmuse-native/v1/hello`。协议必须是 `1`，DSH 版本必须是 `0.1.7-rc.1`。合同不匹配则整面进入 `RemoteDshPage`。
2. `POST /openmuse-native/v1/negotiate`，body 为 `DshNativeCapabilities.standard()`。
3. `GET /openmuse-native/v1/sessions`，按请求的 session id、正在运行的会话或非空会话选择同一个 Desktop Session。
4. `DshConversationController` 同时做两件事：SSE `session/follow` 作为低延迟路径；每 2 秒按 `projections.asOfSeq` 调用 `session/page` 补洞。迟到且 cursor 更小的 snapshot 会被丢弃。
5. 发送调用 `session/prompt`。本地先插入 optimistic 用户行，权威 `user/message.source.rpcId` 到达后按同一个 request id 替换。助手 `text-delta` / `reasoning-delta` 进入 transient 行，`outcome.kind=committed` 后撤掉，避免流式文本和最终消息重复。

Gateway 当前公布的能力：

`workspaces.list`、`sessions.list`、`session.create`、`session.follow`、`session.page`、`session.prompt`、`session.cancel`、`session.options`、`session.model.select`、`session.permission.select`、`session.question.answer`、`session.changes.summary`、`session.changes.preview`、`session.changes.open-desktop`。

Follow 与 page 都带 `maxMessages: 500` 和 `turnWindow: { minMessages: 50, minTurns: 2 }`。这是窗口，不是无限历史。

Paired Desktop 转发 `/openmuse-native/` 时由 Host 注入 bridge token。Flutter Widget 不持有该 token。

## 4. 代码落点

| 位置 | 职责 |
| --- | --- |
| `packages/muse_dsh_conversation_protocol` | hello、negotiation、follow frame、prompt、模型、权限、提问、产物预览的 wire 类型 |
| `packages/muse_dsh_conversation_core` | `DshConversationStore` 投影、`DshConversationController` 的 follow 与 page 补洞 |
| `packages/muse_dsh_conversation_flutter` | `DshConversationSurface`、官方卡片、`DshNativeComponentRegistry` |
| `contracts/openmuse-native-conversation/v1` | 插件 manifest 与 client capability 的 JSON Schema |
| `third_party/dsh/plugins/openmuse-dsh-bridge` `0.1.15` | Native Gateway、官方插件覆盖表、提问 claim、产物 diff |
| `app/openmuse_mobile/lib/native_dsh_page.dart` | 协商、选会话、把 store 接到表面；失败时 WebView |
| `app/openmuse_mobile/lib/workbuddy/workbuddy_shell.dart` | 产品壳、Composer、模型、权限、语音；对话页嵌入且隐藏库内 Composer |
| `app/openmuse_mobile/lib/remote_dsh_page.dart` | 合同不匹配或会话缺失时的整面 Web 兼容 |

Widget 不解析原始 DSH 事件，也不直接发网络命令。路径是：

```text
SSE / page frame
  → DshConversationStore.apply
  → DshConversationSnapshot.rows
  → _ConversationRow 按 row.kind 选择内置 Widget
  → 仅 tool.call.toolview 且 key 命中时，改走 Component Registry
```

行种类只有八种：`user`、`assistant`、`tool`、`error`、`optimisticUser`、`liveAssistant`、`artifact`、`incompatible`。

## 5. 官方事件如何变成界面

`_knownEvents` 里的类型不会变成“不兼容”卡。其中只有一部分会画出可见行，其余被当作控制事件隐藏。

### 5.1 会画出可见行

| 事件 | 行 | 界面 |
| --- | --- | --- |
| `user/message`，且 `source.kind=user` | `user` | 右对齐纯文本气泡、时间、复制 |
| `assistant/message` | `assistant` | 只读 Markdown；`content.type=reasoning` 折成“思考过程” |
| assistant stream `start` / `text-delta` / `reasoning-delta` | `liveAssistant` | 同一气泡的流式正文和思考，完成后撤销 |
| `tool/call` | `tool` | 默认折叠的单行；进行中显示转圈 |
| `tool/result` | `tool` 或 `error` | 按 `callId` 合并掉对应 call；失败走失败段 |
| `tool` 名为 `ask_user_question` | `tool` | 专用提问卡，不走通用工具行 |
| `llm/retry` | `error` | “模型重试” |
| `turn/end` 且 `reason.kind=error` | `error` | “本轮运行失败”，带 message 与 code |
| `workspace/changes` 且 `turn>=1` | `artifact` | 文件数、增删行、列表、文本 diff、在 Desktop 打开 |
| `deliverables/presented` | 不单独成行 | 文件卡挂在同一回合最后一条助手消息下。见 `lib/src/official/deliverables/` |
| `todo/write` | `artifact` | 官方「任务」面板，默认收起 |
| `goal/change` | `artifact` | 目标条。`clear` 和 `phase=complete` 不显示 |
| 本地 `beginPrompt` | `optimisticUser` | “发送中”或“发送失败” |
| 非 ignorable 且不在已知集合内 | `incompatible` | 元素级提示，可打开 Web |

`source.kind` 不是 `user` 的 `user/message`（例如 runtime context）不进入对话面。已有 result 的 `tool/call` 不再单独占一行。`system/message`、`developer/message`、`user/message`、`assistant/message`、`tool/result` 参与 surface `append` / `replace`，被 replace 覆盖的 seq 不再投影。

工具短标题只覆盖这些 wire name：`read`、`read_image`、`bash`、`pwsh`、`terminal_send`、`write`、`edit`、`str_replace_editor`、`glob`、`grep`、`web_search`、`web_fetch`、`ask_user_question`。其它名字直接显示工具名。图标同样只区分读、写、搜索、网页和默认终端，没有各自的结果版式。

正文抽取只保留 `content[].type` 为 `text` 或 `reasoning` 的块，其它块被丢掉。

### 5.2 已知但故意不画成对话行

这些事件更新快照字段，或只作为传输记录存在：

`permission/preset`、`sandbox/mode`、`approval/policy`、`agent/inbox/spliced`、`session/start`、`session/end-seed`、`session/title`、`session/title-llm-request`、`turn/start`、成功的 `turn/end`、`step/start`、`step/end`、`developer/message`、`system/message`、`assistant/attempt`、`request/header`、`request/context`、`command/run`、`command/done`、`model/selection`。

`session/title`、`model/selection`、`permission/preset` 以及 page 里的 projections 会更新标题、模型、思考强度和权限预设。WorkBuddy Composer 读这些字段。

### 5.3 Gateway 对官方插件的覆盖标签

`OFFICIAL_CONVERSATION_COVERAGE` 只影响 negotiate 返回的 `coverage`，不加载 React。标签不等于 Flutter 已经画出同名官方界面。

| 官方包 | 覆盖标签 | Flutter 实际界面 |
| --- | --- | --- |
| `dsh-client-ui-conversation` | `native-shell` | 单列时间线。产品路径用 WorkBuddy 壳，不用 DSH Header |
| `dsh-client-ui-chat` | `native` | 用户气泡、助手 Markdown、复制、流式行。没有轨迹密度和 Turn 折叠 |
| `dsh-client-ui-tool` | `native-generic-tool` | 通用折叠行加输入/输出文本。没有按工具拆开的官方卡片 |
| `dsh-client-ui-user-questions` | `native-question` | 提问卡：单选、多选、自定义文本、跳过、提交 |
| `dsh-client-ui-model-selection` | `native-control` | WorkBuddy 模型与思考强度 bottom sheet，走 `selectModel` |
| `dsh-client-ui-permission-presets` | `native-control` | WorkBuddy 权限 chip，Gateway 执行官方 `/permission` |
| `dsh-client-ui-trajectory` | `native-projection` | 思考文本可展开。没有轨迹页，也没有 compact / standard / detailed / verbose |
| `dsh-client-ui-deliverables` | `native-deliverables` | `deliverables/presented` 挂在回合末尾助手消息下，按官方文件卡显示；`present` 工具行使用「交付文件」状态。版本与交互见 `packages/muse_dsh_conversation_flutter/lib/src/official/README.md` |
| `dsh-client-ui-approval` | `element-fallback` | 无审批接管 |
| `dsh-client-ui-attachment` | `element-fallback` | 无附件 |
| `dsh-client-ui-goal` | `element-fallback` | 无 Goal |
| `dsh-client-ui-jobs` | `element-fallback` | 无 Jobs |
| `dsh-client-ui-plan` | `element-fallback` | 无 Plan |
| `dsh-client-ui-schedule` | `element-fallback` | 无 Schedule |
| `dsh-client-ui-subagent` | `element-fallback` | 无 Subagent |
| `dsh-client-ui-workflow-run` | `element-fallback` | 无 Workflow Run |

`element-fallback` 的意思是：这些包被标成 `native-covered`，negotiate 不会因为“安装了该官方插件”就把整段会话判为不兼容。它们若发出 `_knownEvents` 之外、且非 ignorable 的事件，store 会插入一条带事件名的不兼容卡，其余行继续显示。若它们只通过官方 slot 改写同一条 `tool/call` 的样子，Mobile 仍显示通用工具行，不会出现不兼容卡，也不会出现官方定制样式。

## 6. 插件定制 UI：有限兼容

Schema、Gateway 校验器和 Flutter 实际上报的能力不是同一份清单。谈判以 Flutter 上报为准，避免把没画出来的组件说成已支持。

### 6.1 Flutter 上报并会参与谈判的集合

`DshNativeCapabilities.standard()`：

- API：`nativeUiApi = 1`
- 组件：`text@1`、`code@1`、`badge@1`、`status@1`、`progress@1`、`keyValue@1`、`disclosure@1`、`toolCard@1`、`column@1`、`row@1`、`section@1`
- Slot：只有 `tool.call.toolview`

contribution 的 `requires.components` 必须全部落在这个集合里，且 slot 必须是 `tool.call.toolview`，谈判才会接受它。`fallback=genericToolCard` 时退回通用工具卡；`fallback=omit` 时不画该 contribution；`fallback=web` 或未声明客户端副作用时，插件记为 `incompatible`，会话模式降为 `generic`，时间线保持原生。

### 6.2 Registry 真正会构造的节点

`DshNativeComponentRegistry.buildNode` 处理：`text`、`markdown`、`code`、`badge`、`status`、`progress`、`keyValue`、`button`、`disclosure`、`row`、`column`、`section`。

外层固定是一张 `Card.outlined`，不读取 contribution 的 `template`。`tone` 只影响 badge 和 status。`density`、`emphasis` 被忽略。`visibleWhen` 只支持 `field == equals` 以及 `all`。绑定路径只能从 `session`、`turn`、`message`、`tool`、`plugin.state` 起头；当前行上下文只填了 `tool` 或 `workspaceChanges`，所以 `session.*`、`turn.*`、`message.*`、`plugin.state.*` 会落到 fallback。

`NativeDshPage` 没有把 `onNativeCommand` 传给表面。contribution 上的按钮即使被画出来，`onPressed` 也是空的，不会打到 Gateway。

未知 `component` 现在渲染成 `SizedBox.shrink()`。若一个已接受的 contribution 在 body 里夹了未实现节点，该节点会空白，而不会变成不兼容卡。

### 6.3 结构限制

Gateway 与 schema 共同限制：每插件最多 64 个 contribution，每个 contribution 最多 256 个节点，深度 12，每组最多 64 个子节点，actions 最多 16 个。不接受未知字段、任意 URL、文件路径和可执行表达式。

## 7. 产品路径上已经可用的交互

WorkBuddy Composer（`showComposer: false` 的嵌入页使用这一套，而不是对话库自带输入框）：

- 多行文本发送，空白不可发；失败时把草稿写回输入框。
- 按住说话，上滑取消。识别结果写入同一输入框。
- 权限预设选择，调用 Gateway 的官方 `/permission`。
- 模型与思考强度选择，调用 `sessionController.selectModel`。
- 会话 `running` 时发送按钮换成停止，调用 `session.cancel`。
- 发送模式固定为 `queue`。界面没有 Queue / Steer 开关。

提问卡在 `phase=call` 且提供了 `onQuestionAnswer` 时可操作。Gateway 只在该 Session 已有 Mobile follow 时 claim 问题；没有 follower 时继续交给 Desktop Web，避免双端各答一次。提交前会检查问题 id、单选、多选和自定义答案。

产物卡按逻辑宽度选择 diff：小于 600dp 默认单栏，否则默认双栏。可以切换布局，并在同一 `workspace/changes` 的文件之间前后导航。二进制和超大预览只显示不可用说明。打开文件的动作发给 Desktop，Mobile 不接收主机路径。

连接失败时 follow 退避重试，page 补洞继续。连续失败或 401/403 会通知 WorkBuddy 重连 Paired Desktop。合同不匹配或请求的 session 不在 Desktop 目录中时，才替换成 `RemoteDshPage`。

## 8. 尚未在 Mobile 支持的对话样式

下面按 Desktop 官方对话面和插件扩展面列举。每一项都是当前 Flutter 路径没有的样式或交互，不是“已经用通用卡等价替代所以算完成”。

### 8.1 会话壳与时间线

| 样式 | Desktop 官方行为 | Mobile 现状 |
| --- | --- | --- |
| 会话 Header 操作区 | `conversation.session.header.actions`、utilities、corner、lineage：Jobs、Schedule、Subagent、Open In App | 产品路径隐藏 `DshConversationSurface` Header，只用 WorkBuddy 顶栏。库内 Header 仅有标题、agent preset 和“完整 DSH 前端”按钮 |
| 对话 / 轨迹切换 | 轨迹页展示过程投影 | 库内 Header 的“轨迹”是不可点的静态标签。WorkBuddy 嵌入模式下这个 Header 不出现 |
| 过程密度 | compact、standard、detailed、verbose 只改变过程展示，不隐藏最终答案 | 没有这四个模式。工具行一律默认折叠，思考一律放进 ExpansionTile |
| Turn 折叠与 Turn footer | 已完成 Turn 可折叠；footer 放 latest action、deliverables、Plan tail | 没有 Turn 容器。`turn/end` 成功时不产生行 |
| Step 过程条 | step start/end 组织工具和思考 | `step/start`、`step/end` 只计入 Header 统计，嵌入模式下统计行也不显示 |
| 历史分页 | 按 Turn `loadOlder`，未加载目标用 `loadThrough` | `hasMore` 会记在 snapshot 里，界面没有“加载更早”入口。客户端 page 调用也不传 `beforeSeq`。单次最多 500 条消息 |
| 滚动跟随 | 停在底部才跟随；上滑阅读时只更新未读，不抢位置 | 每次 store 变化都 `animateTo` 到底部 |
| 未读与跳转到 Turn | 离开底部后的新消息提示，以及跳到未加载 Turn | 没有 |
| Fork / 继承历史 | fork 后的历史归属、closing turn、链接到另一 Session | 没有 |
| 设计 token | 冻结的字号、行高、间距、圆角、亮暗色 | 使用 Material `ColorScheme` 和少量硬编码色值，没有 `contracts/dsh-ui/0.1.7-rc.1` token |

### 8.2 消息正文

| 样式 | 说明 |
| --- | --- |
| 用户消息里的 Markdown | 用户行用纯文本，不解析 Markdown |
| 图片内容块 | `content.type` 不是 `text` / `reasoning` 的块被 `_contentText` 丢弃，包括图片 |
| 文件与资源附件块 | 消息内的文件 chip、本地附件预览、lightbox 都没有。`conversation.message.images` 未实现 |
| Markdown 图片 | `MarkdownBody` 没有自定义 image builder，不能按 Resource Authority 显示工作区图片 |
| 表格、数学公式、Mermaid、任务列表 | 没有专用 renderer，也不按 DSH 样式表排版 |
| 代码块工具条 | 没有语言标签、复制代码块、折行或语法高亮主题 |
| 工作区路径链接 | Markdown 链接不能在 Mobile 打开对应资源，也不能跳到 Desktop 编辑器 |
| 助手消息操作 | `conversation.chat.assistant-actions` 未上报。没有赞、踩或其它官方 action。复制作用在整段纯文本上 |
| 系统行与开发者行 | `system/message`、`developer/message` 在已知集合内且被隐藏，没有官方那套系统提示样式 |
| Compaction / token usage | 没有压缩说明行，也没有 token 用量条 |
| 中间失败与终态失败的官方版式 | `llm/retry` 和 `turn/end` error 是简单文本段，不是官方失败卡的步骤、原因和重试按钮 |

### 8.3 工具卡片

通用 `_ToolCard` 把参数和结果格式化成等宽文本。下列官方工具样式都还没有：

| 工具或视图 | 缺少的样式 |
| --- | --- |
| `bash` / `pwsh` / `terminal_send` | 终端配色、逐行输出、退出码、工作目录、实时追加。现在只是“运行命令”单行，展开后是 JSON 或纯文本 |
| `read` | 路径摘要、行号范围、语法预览。`read_image` 不显示图片 |
| `write` | 新文件正文预览。写入 diff 只存在于单独的 `workspace/changes` 卡，不在工具行内 |
| `edit` / `str_replace_editor` | 工具行内的 unified / split diff。行内改动不会自动变成产物预览 |
| `glob` / `grep` | 命中列表、路径、行号，而不是一整段文本 |
| `web_search` / `web_fetch` | 标题、URL、摘要列表 |
| Todo 工具 | 勾选列表和完成态 |
| 工具树 | PTC root 与 `subCalls` 的缩进、状态和原位更新。现在一次 call 与一次 result 合并成一行，没有子调用树 |
| 工具阶段 | preparing、interrupted 等阶段没有单独样式。进行中只看 `phase=call` 的转圈 |
| Skill、Cordis、Deliverables 等按工具名注册的 `tool.call.toolview` | 只有谈判接受的声明式 contribution 才会替换通用卡。官方这些视图没有对应 contribution，因此仍是通用行 |
| 产物以外的 Deliverables | `deliverables/presented` 已按官方文件卡挂在助手消息下。`workspace/changes` 仍是另一张 diff 卡。Mobile 没有侧边栏预览，主按钮改为在 Desktop 用默认应用打开 |
| 二进制 / 超大 diff | 预览页只说明不可用，没有十六进制、截断策略或图片 diff |
| 在 Mobile 打开产物 | 只有“在 Desktop 打开”。没有经 Resource Authority 的 Mobile 阅读器 |

### 8.4 Composer、队列和会接管输入框的官方流程

产品 Composer 已有文本、语音、模型、权限和停止。下面这些官方 Composer 样式没有：

| 样式 | 现状 |
| --- | --- |
| Queue 与 Steer | `prompt` 协议接受 `queue` 和 `steer`，WorkBuddy 与 `NativeDshSessionHandle.send` 都固定传 `queue`。没有 QueueDock，也没有“运行中改走 Steer”的设置。Chat 与队列不会互相去重，因为队列 UI 不存在 |
| Slash command | 对话库 Composer 的 hint 写了“/ 调用指令”，按键没有命令面板，`+` 按钮是空操作。产品 Composer 的 hint 只有“发消息” |
| `@` 引用 chip | 没有文件、会话或符号 chip，也不能从最终字符串还原引用 |
| 附件与上传队列 | `+` 打开的列表里，“从 Desktop Workspace 引用文件”是禁用项，文案为“附件协议将在下一阶段开放”。没有相册、相机、进度、取消和重试 |
| Approval 接管 | `dsh-client-ui-approval` 是 `element-fallback`。Composer 不会被允许 / 拒绝条替换，危险权限也没有二次确认卡。`approval/policy` 被隐藏 |
| 用户提问接管 Composer | 提问是时间线里的卡片，不是替换底部输入框的 `conversation.composer` |
| Plan mode 输入条 | 没有 plan 开关、计划草稿和确认条 |
| Subagent 接管 | 没有子代理输入、血缘返回或嵌套会话头 |
| 插件输入槽 | `conversation.input.dock`、`conversation.input.left`、`conversation.input.right`、`conversation.composer` 都在 schema 里，Flutter 不上报，因此谈判不会接受 |
| 草稿恢复的官方语义 | 发送失败会把本次文本写回输入框。没有跨页面、并发失败顺序和 takeover 取消后的草稿恢复 |
| 对话库自带 Composer 的权限与模型 | 非嵌入页面仍显示写死的“工作区内修改”，模型只是快照文本，不能在那条 Composer 里切换。产品路径不使用这条 Composer |

### 8.5 官方业务块

这些块在 Desktop 由对应 `dsh-client-ui-*` 绘制。Mobile 没有专用 Widget。若它们产生未知且非 ignorable 的事件，用户看到的是“Mobile 暂不支持此 DSH 元素”加上原始事件名，而不是该块的官方样式。

| 块 | 官方表面 | Mobile |
| --- | --- | --- |
| Approval | 批准、拒绝、理由、一次性提交 | 不兼容卡或被隐藏的 `approval/policy`，没有决定按钮 |
| Plan | 计划步骤、接受 / 修改、Turn tail | 无 |
| Goal | GoalBar 的阶段和目标正文 | 已显示。暂停、编辑、清除仍只在 Desktop，因为 Native Gateway 没有 goal revision command |
| Todo | 会话级 Todo dock | 已按官方「任务」面板显示整份 `todo/write` 快照，默认收起 |
| Attachment | 选择、预览、发送后的 durable 图片和文件 | 无；入口禁用 |
| Jobs | Header 里的后台任务 | 无 |
| Schedule | 定时任务入口和状态 | 无 |
| Subagent | 子会话卡片、跳转、Composer 接管 | 无 |
| Workflow Run | `conversation.chat.node` 上的运行节点 | 无 |
| Message Feedback | 助手消息上的反馈动作 | 无 |

### 8.6 声明式组件里写了、但对话渲染器没有的样式

Schema 与 Gateway 的 `NATIVE_COMPONENTS` 接受这些名字。Flutter 谈判集合或 `buildNode` 没有把它们画成对应控件。插件若把它们列为 `requires`，contribution 不会被接受；若只是夹在已接受 body 里，节点会被缩成空白。

| 组件或模板 | 预期样式 | 现状 |
| --- | --- | --- |
| `markdown` | 插件正文的安全 Markdown | `buildNode` 能画，但 `standard()` 不上报 `markdown@1`。要求该组件的 contribution 不会通过谈判 |
| `button` | 行内按钮 | 同样能画但不上报 `button@1`。产品页又没有 command handler，按钮不可用 |
| `toolCard` | 作为节点的工具卡模板 | 上报了 `toolCard@1`，`buildNode` 没有这个 case。contribution 外层卡片是写死的，不是该节点 |
| `table` | 受限表格 | 不渲染 |
| `image` / `gallery` | 授权图片和画廊 | 不渲染 |
| `fileLink` / `resourceLink` | 资源链接 | 不渲染 |
| `iconButton` / `menu` | 图标按钮和菜单 | 不渲染 |
| `segmentedControl` / `select` / `checkbox` / `textField` | 选择和输入 | 不渲染 |
| `messageAddon` | 挂在消息上的附加区 | 模板名被校验，渲染器忽略 `template` |
| `turnTail` | Turn 尾部 | 同上 |
| `approvalForm` / `questionForm` / `selectionForm` | 官方表单模板 | 同上。提问卡是内置 `ask_user_question`，不是这个模板 |
| `density` / `emphasis` | 紧凑度和强调 | 字段可出现在节点上，渲染器不读 |

未上报、因此不能被谈判接受的 slot：

`conversation.chat.node`、`conversation.chat.assistant-actions`、`conversation.chat.turnTail`、`conversation.input.dock`、`conversation.input.left`、`conversation.input.right`、`conversation.session.header.actions`、`conversation.composer`。

未实现的绑定与动作语义：

- 条件没有 `not`、`or`、比较和存在性判断。
- 行上下文没有 `session`、`turn`、`message`、`plugin.state`，这些绑定只会显示 fallback。
- 没有通用 command registry。按钮不能按插件声明的 schema、权限和幂等键执行。
- 插件图变化后没有 negotiation revision。已接受的 contribution 不会随安装或卸载自动失效。
- 未声明 `openmuse.nativeConversation` 的第三方 React / CSS 不会被翻译。negotiate 给出插件级 `incompatible`，对话顶部可出现兼容提示；具体节点仍取决于它是否发出未知事件。

### 8.7 远程 Surface 组件也不在对话里

即使 remote-surface 的实现继续前进，下面这些控件也不会自动出现在 DSH 对话流中。对话渲染器没有引用 `muse_remote_surface_contract`：

`video-player`、`document`、`link`、`list`、`grid`、`form`、`stepper`、`diff`、`compare`、`timeline-basic`、`confirmation`，以及 remote-surface 自己的 `image`、`gallery`、`markdown`、`button`。

`web-snapshot` 和 `web-interactive` 同样属于工作台协议。对话里的 Web 兼容是整面 `RemoteDshPage`，不是插件页面快照，也不是按元素嵌入的 WebView。

### 8.8 同步与会话行为里还看不见的界面

这些缺口不一定多出一个新卡片，但官方对话在这些状态下的样子 Mobile 还没有：

- 物理断线后的只读快照与 generation 切换提示。当前是失败文案加自动重试。
- Desktop 重启、冷 Session 不意外启动 Agent、三端同时发送时的 Queue 顺序。
- 长会话虚拟化、高速流式合并、动态字号和 reduced motion。时间线是普通 `ListView.builder`，Markdown 在行构建时解析。
- iOS 上这条原生对话链路还没有与 Android 同级的产品验收。

## 9. 兼容时用户实际会看到什么

| 情况 | 界面 |
| --- | --- |
| 协议或 DSH 版本不是 `1` / `0.1.7-rc.1` | 整面 `RemoteDshPage` |
| 请求的 session 不在 Desktop 目录 | 整面 Web |
| 官方基础事件 | 第 5 节的原生行 |
| 工具名有声明式 contribution，且 slot 与 requires 都被接受 | 通用组件卡替换该工具行 |
| contribution 要求了未上报组件 | 不替换，继续通用工具行或按 fallback 省略 |
| 未声明客户端副作用的插件 | 原生时间线保留；若 negotiate 标了 `incompatible`，顶部有一条插件提示和“兼容模式” |
| 未知且非 ignorable 事件 | 该 seq 一张不兼容卡，其余消息仍在；卡上的按钮才进入 Web |
| `OPENMUSE_DSH_NATIVE_FORCE_WEBVIEW=1` | Gateway 把 hello 标成必须 fallback。这是诊断开关，不是单个元素的降级 |

元素级 Web 入口打开的是完整 DSH Web 会话，不是把那一张卡片换成 WebView。
