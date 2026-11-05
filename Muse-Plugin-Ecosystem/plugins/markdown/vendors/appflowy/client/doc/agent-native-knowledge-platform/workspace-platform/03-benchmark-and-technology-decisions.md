# 主流实现参考与技术选型

## 1. 参考对象

### 1.1 VS Code / Cursor 系

借鉴：

- Folder 与 Multi-root Workspace 分离；工作区可以由多个根目录组成。
- 文件系统使用 URI/scheme，而不是假设所有资源都是 `file:`。
- Virtual Workspace 由 FileSystem Provider 提供，云端资源也能进入同一 Explorer。
- UI Extension 与 Workspace Extension 分离；需要接近数据和执行环境的能力运行在远端。
- Explorer、View、Editor Title 等菜单是贡献点，使用 context/when 条件控制显示和启用。
- Workspace Trust 决定扩展、任务和 Agent 是否允许运行，而不是只有文件读写权限。

不照搬：

- 不采用 Electron/Node Extension Host；Muse 继续使用 Flutter + Rust Host + DSH Plugin Graph。
- 不暴露任意本地 URI 给 Agent；外部接口继续使用 opaque ref 和授权句柄。

官方参考：

- [Multi-root Workspaces](https://code.visualstudio.com/docs/editing/workspaces/multi-root-workspaces)
- [Virtual Workspaces](https://code.visualstudio.com/api/extension-guides/virtual-workspaces)
- [Contribution Points / Menus](https://code.visualstudio.com/api/references/contribution-points)
- [Extension Host](https://code.visualstudio.com/api/advanced-topics/extension-host)
- [Workspace Trust](https://code.visualstudio.com/docs/editing/workspaces/workspace-trust)
- [Remote Extensions](https://code.visualstudio.com/api/advanced-topics/remote-extensions)

### 1.2 Zed

借鉴：

- 本地保留 UI、交互和未保存状态；远端负责源文件、语言服务、任务和终端。
- Remote Project 不复制整个仓库到本地。
- AI/Agent 在远端 Project 中仍使用同一工作区上下文。
- Project 允许多个 worktree/repository，并把信任绑定到 worktree。

不照搬：

- Muse 的 Workspace 不只面向代码，还包括 Office、媒体、协作页面和云对象。
- Muse 必须支持只有对象存储 API、没有可执行远端主机的 Provider。

官方参考：

- [Zed Remote Development](https://zed.dev/docs/remote-development)
- [Zed Worktree Trust](https://zed.dev/docs/worktree-trust)
- [Zed Git / multiple repositories](https://zed.dev/docs/git)

### 1.3 JetBrains

借鉴：

- Project Tool Window 支持目录结构、Project Files/Open Files 等不同投影视图。
- 菜单围绕当前节点、selection 和 scope 提供动作。
- Remote Development 中 UI 与 backend 分离，并明确上传/下载、连接状态和 backend control。

不照搬：

- 不将每种 Provider 做成独立工具窗口；统一进入 Workspace Explorer。
- 不要求所有远端都部署完整 IDE backend。

官方参考：

- [Project tool window](https://www.jetbrains.com/help/webstorm/project-tool-window.html)
- [Remote development overview](https://www.jetbrains.com/help/idea/remote-development-overview.html)
- [Work inside remote project](https://www.jetbrains.com/help/idea/work-inside-remote-project.html)

## 2. 技术选型

| 主题 | 选择 | 未选择 | 决策理由 |
|---|---|---|---|
| 工作区身份 | opaque `workspaceRef` | 本地路径、View ID | 跨本地/SSH/云，避免路径即权限 |
| 多根模型 | Project Workspace + Mount[] | 单根目录 | 产品、代码、数据和协作知识可组合 |
| Provider 合同 | capability-based async SPI | 中央扩展名/类型枚举 | 支持异构文件系统和渐进能力 |
| Host Provider 实现 | Rust | Flutter 直接 `dart:io` | watcher、路径安全、SSH/缓存和 I/O 更适合 Rust |
| UI 状态 | Flutter BLoC + immutable tree cache | 复用 `ViewBloc` 表示全部文件 | 保持现有栈，同时与 AppFlowy Page 模型解耦 |
| 工作区定义存储 | Account Collab metadata + 本机设备状态 | 全部写 `.muse-workspace` | 团队定义可同步，credential/展开状态不上传 |
| 可移植文件 | 可选 `.muse/workspace.json` | 强制项目文件 | 方便 Git 团队共享，但不污染所有工作区 |
| 本地 Provider | Rust `std::fs` + notify/watch | Dart File API | 跨平台、性能、原子操作和统一授权 |
| SSH | Remote Workspace Agent over SSH | 挂载 FUSE 为主 | 让 DSH/LSP/终端靠近数据；避免网络文件系统语义问题 |
| SSH 降级 | SFTP Provider | 下载整个目录 | 可浏览/预览/传输，但禁用依赖远端执行的能力 |
| 云端 | Provider 原生 API + range/cache | 映射成假本地路径 | 保留版本、etag、分享和权限语义 |
| 缓存 | metadata + bounded content CAS | 无界镜像 | 支持大文件、离线和 revision 校验 |
| 插件菜单 | Command Registry + declarative contributions | 每个 Widget 写 enum/switch | Tab/Explorer/命令面板共用，可卸载可治理 |
| DSH 接入 | Workspace Binding + Provider Broker | 传原始 credential/path | Agent 只持有 scope 和短期能力 |
| 搜索 | Provider search；无则 Host 增量索引 | 全量目录注入 prompt | 控制成本，尊重远端和云端能力 |

## 3. 技术原则

1. **Location transparency, capability explicitness**：位置可以透明，能力必须显式。
2. **Metadata first**：先列元数据，再按需读取内容；Explorer 不触发文件正文读取。
3. **Execution follows data**：需要 shell/LSP/index 的任务尽量在 Mount 所在执行域运行。
4. **UI remains local**：Flutter Workbench、Tab、菜单和交互状态在 Host UI 侧。
5. **No fake parity**：Cloud/SFTP 不支持 rename/watch/atomic write 时，菜单应禁用并解释，而不是模拟成功。
6. **Revision before mutation**：所有写入带 expected revision/etag；冲突不覆盖。
7. **Trust before execution**：浏览和预览可在受限模式工作；任务、插件和 Agent 执行需要 Workspace Trust。

