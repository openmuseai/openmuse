# DSH 对话流技术选型

> 状态：方案  
> 日期：2026-10-01  
> 范围：手机原生壳里的 DSH 对话流。外壳仍是现有任务工作台；本文只决定对话正文怎么实现、插件副作用怎么跟上、以及怎么跟 DSH 版本一起迭代。

## 1. 结论

对话流做成一个库，库对外只暴露「打开同一条 DSH 会话、发送、停止、读取快照」。库里面放两个表面：

- **默认真表面是 WebView。** 加载与 Desktop 同一份已钉住的 DSH 客户端闭包，以及同一组声明了 `dsh.client` 的插件。这是唯一能让后装插件增加元素、替换某一行、改自己的样式，并且仍和 Desktop 同时看到同一条会话的做法。
- **Flutter 只渲染核心节点。** 用户消息、助手正文、思考、通用工具行、错误、停止。它用于任务列表上的状态、断线后的最后一帧，以及没有对应 Web 插件时的兜底。未识别的插件节点保持为不透明块，不假装已经一比一画出来。

不采用「完全用 Flutter 重写 `dsh-client-ui-chat`」。那个包本身就不是整条对话流，插件通过 Cordis slot 在运行时把 React 组件插进去。把这些组件逐个翻译成 Flutter，会在每一次 DSH 升级和每一次用户安装插件时失步。

同步的事实来源不变：手机和 Desktop 连同一个 DSH runtime、同一份会话存储。库不保存第二份聊天记录。

## 2. 源码不在 `0.1.7-rc.1` 这个目录

`/Users/mac/src/openmuse-io/0.1.7-rc.1` 不存在。能读到的前端源码和产品实际运行的版本不是同一棵树。

| 位置 | 实际内容 | 版本 |
|---|---|---|
| `vendors/deepseek-harness` | DSH 客户端源码，含对话壳和 Chat target | 仓库根 `package.json` 与 `dsh-client-ui-chat` 都是 `0.1.5-rc.2` |
| `vendors/dsh-desktop` | Desktop 品牌对若干 UI slot 的占用 | `dsh-desktop-client-ui` `0.1.0`，peer 依赖 `^0.1.5-rc.1` |
| `Muse-Client/third_party/dsh/package.json` | 产品打包闭包的依赖声明 | `@deepseek-ai/dsh` 钉在 `0.1.7-rc.1` |
| `Muse-Client/contracts/dsh/0.1.7-rc.1.contract.json` | 已冻结的 Provider 合同 | 覆盖 fs、shell、subprocess，不覆盖客户端 slot |

因此：`vendors/deepseek-harness` 可以用来理解对话流怎么拆、插件从哪里插进来。它不能当作 `0.1.7-rc.1` 的实现，也不能当作手机渲染器的直接复制源。`0.1.7-rc.1` 的客户端以 npm 闭包为准。升级时两边的 slot 名和节点种类必须重新对过，对不上就先停，不能用 `0.1.5-rc.2` 的源码解释 `0.1.7-rc.1` 的界面。

现有文档已经写过同一条边界：vendor 源码是架构证据，产品行为以打包闭包为准。见 `DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md`。

## 3. 前端对话流实际由谁组成

对话流不是一个页面文件，而是两层包加上一组运行时注册的插件。

`@deepseek-ai/dsh-client-ui-conversation` 拥有和具体展示目标无关的壳：

- 消费 Session Controller 的事件，组装 Conversation binding。它不另开一条事件源。
- 会话头、视图切换、输入框、排队、空会话 Hero。
- 对外是 Cordis slot，不是写死的组件树。

`@deepseek-ai/dsh-client-ui-chat` 是 Chat 这个展示目标：

- 把已加载的 Session 窗口投影成一行一行的 Chat Node。
- 本地先显示刚发出的 transcript 和 steering，权威记录到达后原子替换。排队中的提交不进 Chat。
- 紧凑模式下收起已完成轮次的过程行，保留最终答案。
- 滚动锚点、加载更早、轮次导航都在这个包里。
- 它只渲染已经记录的状态，不组装模型请求。

节点种类由已安装的 Chat 模块通过 declaration merge 填进 `ChatNodeDataMap`。源码里的 `ChatNodeKind` 是这个映射的键，不是一份写死的枚举。当前树里工具行由另一个包注册成 `conversation.chat.node` 的键 `tool-call`，再在其下挂 `tool.call.toolview`，按工具名换成 bash、读文件、搜索、网页、待办、提问等不同的行。

和对话流直接相关、会改画面的包还包括：`ui-tool`、`ui-message-feedback`、`ui-user-questions`、`ui-plan`、`ui-jobs`、`ui-trajectory`、`ui-input-trigger`、`ui-model-selection`。`openmuse-dsh-bridge` 的客户端声明则注入 `dsh-api-session-controller`、`dsh-client-ui-sidebar-right` 和 `dsh-client-ui-workspace`，用来把文件打开和工作区切回 Host，它不重画消息行。

## 4. 插件对对话流的副作用

客户端插件在 `package.json` 的 `dsh.client.inject` 里声明依赖，加载后调用 `ctx.slots.register` 或 `ctx.slots.inject`。slot 有四种占位方式，副作用不一样。

| 方式 | 对话流里的例子 | 安装插件之后 |
|---|---|---|
| `keyed` | `conversation.chat.node`、`tool.call.toolview`、`conversation.chat.commandview` | 同一个键再注册一次会换掉原来的渲染器。没有占用的种类不画那一行 |
| `single` | 图片廊 `conversation.message.images`、输入框上的模型选择、计划按钮、空会话品牌 | 后来的注册替换原先那一块。没有注册时，图片直接不显示 |
| `list` | 助手消息操作 `conversation.chat.assistant-actions`、会话头按钮、输入框左右两侧 | 按顺序追加。反馈插件就是这样在助手消息上加赞和踩 |
| `chain` | `conversation.chat.turnTail`、`conversation.composer` | 第一个接受当前状态的占用者画出来，用来临时接管输入框 |

所以「装了一个插件」可以是：多一个按钮、多一种工具行、换掉某种消息的整行、在输入框里加控件，或者临时换成另一套输入区。样式跟着那个插件自己的组件走，一般是它的 CSS module，不是改 Flutter 主题，也不是改一份全局对话皮肤。插件不通过改 Chat 包的源码生效。

服务端插件（工具、模型、fs、shell）只改变会话事件里有什么。它们不直接改前端。前端要出现新样子，还得有对应的 client 插件去注册 slot。只装了服务端插件时，Chat 用已有的通用行把新事件画出来。

`ui-chat` 的说明写明：没有注册图片渲染器时，图片省略；没有对应 command 渲染器时，用通用卡片。这就是插件未安装时的降级，不是渲染器自己猜。

## 5. 三种做法

### 5.1 完全 Flutter

把 `ui-conversation` 和 `ui-chat` 的壳、节点、折叠、滚动用 Flutter 重写，并为每个已知工具行写一个 Widget。

做得到的部分是核心节点和任务列表上的状态。做不到的部分是第 4 节：用户之后安装的 client 插件是 React 组件，运行时才注册，还带着自己的 CSS。Flutter 库无法在不发版的情况下执行这段插件。把已有插件逐个翻译，下一次 DSH 改 slot 名或节点种类就会对不上，而且 `0.1.5-rc.2` 源码和 `0.1.7-rc.1` 闭包还不是同一版。

因此完全 Flutter 不能同时满足「一比一」和「插件副作用」。

### 5.2 完全 WebView

手机对话区直接打开现有 `RemoteDshPage` 那条路：同一 grant 下的 DSH 客户端。Desktop 装了哪些 client 插件，只要手机加载的闭包里也有，画面就跟着变。会话列表、历史和运行中状态已经在真机上对过同一份 DSH。

代价是对话正文不是 Flutter Widget。WorkBuddy 的顶栏、侧栏、底栏仍然是 Flutter，中间一块是 Web。文件点击、复制、安全区域要继续走现在的桥，不能从 Web 里直接摸 Desktop 路径。插件如果依赖桌面才有的窗口能力，手机上会缺那一块，这是闭包差异，不是投影错误。

### 5.3 一个库，两个表面

库名沿用宿主依赖方向，放在 `Muse-Client/packages/openmuse_dsh_transcript`。它不依赖 WorkBuddy 页面。

库拥有：

- 会话快照和事件订阅。权威仍是 DSH，断线保留最后一帧，重连先补快照。
- 发送与停止。停止带上当时的 `sessionRef` 和 `generation`。新消息是 steering 还是排队，由 DSH 决定。
- `TranscriptSurface`：打开一条会话并跟着事件更新。
- `WebConversationSurface`：默认实现，WebView 加载钉住的客户端闭包。
- `NativeCoreTranscript`：只画核心节点。未知种类显示为不透明块，并标明这块来自未映射的插件。

WorkBuddy 只依赖 `TranscriptSurface`。任务列表用快照里的标题和状态。点进任务后，默认把 `WebConversationSurface` 放进正文区域，不再使用现在那份本地样例对话。

## 6. 采用的方案

采用第 5.3 节。默认表面是 WebView，原因是插件副作用和版本伴生关系都在 DSH 客户端里，不在 Flutter 里。

Flutter 核心渲染器同时做，但验收口径分开：

- Web 表面验收「和 Desktop 当前客户端一致」，包括已加载插件带来的行和按钮。
- Flutter 表面验收「核心节点与快照一致」。不把未实现的插件行算进一比一。

在 Web 表面的 S1 到 S5 通过之前，不把 Flutter 核心渲染器标成对话流的正式界面。现有 WebView 页留到那时再从任务入口撤下。

手机闭包要带上 Desktop 产品闭包里声明了 `dsh.client` 的插件，至少包括 `dsh-client-ui-conversation`、`dsh-client-ui-chat`、`dsh-client-ui-tool`，以及 `openmuse-dsh-bridge` 的客户端。少带一个，对应的按钮或工具行就会按第 4 节的规则消失或变成通用卡片。这是预期降级，要在闭包清单里写明，不能等用户看到空白再查。

## 7. 版本兼容和迭代

1. 客户端闭包和 `@deepseek-ai/dsh` 使用同一个钉扎版本。现在是 `0.1.7-rc.1`。不从 `vendors/deepseek-harness` 的 `0.1.5-rc.2` 抽一份私有前端放进手机。
2. 为这个钉扎版本增加一份客户端合同，和现有 Provider 合同分开。合同记录：对话相关 slot 名、`ChatNodeKind`、`tool.call.toolview` 的内置键、以及本产品附带的 client 插件清单。生成方式是加载该闭包的注册表，不是手抄 `0.1.5-rc.2` 的 `slots.ts`。
3. DSH 升级时先跑这份合同。slot 或节点种类变化则构建失败，必须改合同并重跑 Web 表面的对照验收。Flutter 核心渲染器只在合同里它声明支持的种类上运行；新种类自动落到不透明块。
4. 用户额外安装的 client 插件：Web 表面能显示，当且仅当该插件被打进手机加载的那份闭包。不把插件的 React 组件编译进 Flutter。服务端插件不要求 Flutter 发版，事件仍从同一条会话流进来，画面用通用行或已有工具行。
5. 库的 Dart API 按快照和命令版本化，不按 DSH 的 React 组件版本化。DSH 小版本只更新闭包和客户端合同；Dart API 在快照字段不兼容时才升。

## 8. 和验收的关系

上一份验收矩阵仍然有效，但要注明表面：

- S1 到 S5 对 `WebConversationSurface` 执行，对照物是同一台 Desktop 上的 DSH 客户端，而不是 Flutter 自己的核心渲染器。
- 另外增加一条插件验收：Desktop 闭包启用 `dsh-client-ui-message-feedback` 时，手机 Web 表面的助手消息上出现同一组操作；不启用时，这组操作不出现。这用来证明副作用来自闭包，而不是 Flutter 写死的按钮。
- Flutter 核心渲染器单独列一份子集：用户消息、助手正文、通用工具行、停止后的状态。未知节点必须可见地标成未映射，不能画成空白或画错种类。

跨公网 relay、推送唤醒、在手机里跑 DSH 进程，仍不在这个库里。
