# Helix 模态与 VS Code 式非模态编辑：引擎级设计

状态：2026-09-26 引擎级原型进入第三阶段，**尚未通过发行门禁，不得称为已交付的 VS Code 模式**。

## 当前实现进度

本仓库 `third_party/helix` 固定在上游提交 `079a789e8cb08ead67f19e1971a1b7438b37354b`（MPL-2.0），不从相邻仓库读取源码。`[editor] input-profile = "standard-nonmodal"` 是新增的引擎配置：新会话第一帧进入 Insert；Normal/Select 不能由 Esc 或命令重映射进入；非模态下空闲输入批次和粘贴会生成撤销检查点。Rust 集成测试覆盖普通 `i`、`:`、Esc 与一次撤销，插件偏好包含版本化 profile 并拒绝不支持的二进制。

第二阶段新增独立于 PTY ANSI 的本地事件通道。引擎向每个会话的 loopback socket 发送版本化 `hello`（随机令牌、PID）、`state`（活动文件、dirty、revision）和 `saved`；插件校验令牌/PID/消息长度，并把活动文件变化交给 Host 的已挂载工作区路径授权后激活 Helix Tab。实验性非模态启动必须在 8 秒内收到首个可信 `state`，否则杀掉 PTY、关闭通道并显示失败，不会静默当作已就绪。

第三阶段 `openmuse-nonmodal.3` 在同一认证通道加入带 request ID、目标绝对路径和预期 revision 的语义命令及结果 ACK。引擎直接执行 `save`/`flush`、`undo`、`redo`、`find`、`select_all`，拒绝重复请求、错误文件和过期 revision；保存复用 Helix 自身的格式化/写入流程，并在异步写入完成后才返回成功。Host 的版本快照和“与当前工作版本比较”先要求插件 flush；无法确认活动 buffer 落盘时终止操作，避免以旧磁盘内容生成误导性差异。Flutter 终端仅在非模态 profile 下把 Cmd/Ctrl+S、F、Z、A 送往语义通道，不通过 PTY 注入 `:write` 等字符。Rust 单元/集成测试、打包二进制的真实 PTY 冒烟测试和 Flutter 协议/工作区测试覆盖了这条链；这些测试不等同于 macOS GUI 输入法和 Windows 实机验收。

当前发行设置中非模态选项仍保持禁用；仅当二进制包含 `openmuse-nonmodal.3` 标记且开发环境显式设置 `OPENMUSE_EXPERIMENTAL_NONMODAL=1` 才能试用。系统剪贴板的复制/剪切/粘贴、中文 IME、真实 GUI 焦点与 Cmd 分发、真实 LSP 跨文件跳转端到端测试、会话切换时脏 buffer 处理及 Windows 实机验收**均未完成**。用户现有 `vscodeKeymap` 只作为旧设置兼容，不会自动升级为非模态。

## 为什么仅重映射快捷键不可行

当前插件启动的是随包 `hx` PTY。Helix 默认以 Normal mode 启动；Insert mode 的撤销检查点主要在退出该模式时提交。现有 `vscodeKeymap` 只在 `[keys.insert]` 添加了少量 Ctrl 绑定，仍能通过 Esc 进入 Normal mode，Ctrl+F 在 Normal mode 还是翻页。这会同时呈现两套互相冲突的编辑语义。Helix 官方文档也明确将其定义为模态编辑器；单靠 `config.toml` 不能从引擎根部删除 Normal/Command 状态。

设置现在显示“Helix（模态）”与禁用的“标准非模态（实验）”；旧 `vscodeKeymap` 只读取历史偏好，不再作为一个模式选项。产品发布前须完成以下引擎门禁。

## 目标契约

插件设置引入版本化 `inputProfile`：`helix-modal` 与 `standard-nonmodal`。Host 只持久化 profile 和调度资源 Tab，不解释单个按键。插件声明 profile 能力；安装包未包含通过门禁的非模态引擎时禁用第二项并给出原因，绝不静默回退成混合模式。

`standard-nonmodal` 从文件第一帧起直接接受文本输入，无 Normal/Select/Command mode 可由普通键入或 Esc 到达。Esc 只关闭补全、搜索、重命名、命令面板等临时 UI；命令面板由显式快捷键打开，不暴露 `:` 模式。选择、输入、退格、鼠标与多光标遵守常见桌面编辑器语义。

| 动作 | macOS | Windows/Linux | 引擎行为 |
|---|---|---|---|
| 保存 | Cmd+S | Ctrl+S | 提交活动 buffer 并回报 revision/脏状态 |
| 查找 | Cmd+F | Ctrl+F | 聚焦搜索框，不触发 Helix 翻页 |
| 撤销/重做 | Cmd+Z / Cmd+Shift+Z | Ctrl+Z / Ctrl+Y | 按输入批次提交检查点，不依赖 Esc |
| 复制/剪切/粘贴 | Cmd+C/X/V | Ctrl+C/X/V | 系统剪贴板与选择区一致 |
| 全选 | Cmd+A | Ctrl+A | 当前 buffer 全选 |
| 定义/引用/重命名 | F12 / Shift+F12 / F2 | 同左 | LSP 结果改变活动资源时通知 Host Tab |
| 关闭临时界面 | Esc | Esc | 不改变文本输入模式 |

## 实现边界与取舍

推荐在本仓库维护可审计的 Helix 引擎源码分支，而非继续以终端按键注入模拟非模态。其输入状态机新增 `standard-nonmodal` profile：启动直接进入常驻文本输入；普通字符不经过 Normal mode dispatcher；撤销检查点按输入批次/暂停/命令边界提交；搜索、LSP、重命名、保存使用显式命令分发；为 Host 输出可信的 `activeResourceChanged(uri)`、`dirtyChanged`、`saveCompleted` 事件。PTY ANSI 输出仅用于绘制，不用于猜测活动文件路径。

短期继续用 PTY 绘制。双向结构化通道已经覆盖保存、撤销/重做、查找、全选及比较前 flush；后续还需为系统剪贴板、LSP 导航、搜索框焦点与重命名增加完整语义和 UI 验收。长期可将 Helix core 通过 FFI/IPC 嵌入插件，分离文本核心与终端 UI。非模态 profile 的核心状态和撤销模型必须由引擎实现，Host 只负责标准平台快捷键转成语义命令。不得通过“启动后发送 `i`、屏蔽 Esc、把鼠标动作改成字符序列”作为发行实现。

若 Helix 源码层的维护成本或协议限制不可接受，替代方案是在同一插件位提供独立 GUI 文本引擎，并共享 Resource/Tab/LSP 契约；此时 UI 应标为“标准编辑器”，不能称为 Helix 非模态。

## 验收门禁

1. 空文档、重开文档、Tab 切换、LSP 跳转、恢复焦点后，普通 `i`、`:`、`Esc` 均不切入模态命令状态；中文输入法组合输入不丢字。
2. 上表快捷键在 macOS 与 Windows 逐项实测，包含焦点在搜索框/补全框/编辑器时的分发与菜单冲突。
3. 逐字输入、粘贴、自动缩进、格式化、LSP edit 后撤销/重做边界正确；保存前 buffer flush 与版本比较一致。
4. LSP 定义跳到另一文件时 Host 创建/激活正确 Tab，回退/前进同步；无可信活动文件事件不得放行。
5. profile 切换先处理脏 buffer，再重启/迁移会话；失败时保持原模式且显示错误，不悄悄回退。
6. 引擎源码、二进制构建输入、许可与补丁可在本仓库独立重现；没有旧工程运行时依赖。

依据：[Helix Keymap](https://docs.helix-editor.com/keymap.html)、[Key remapping](https://docs.helix-editor.com/remapping.html)、[Using Helix](https://docs.helix-editor.com/usage.html)。
