# 当前 Workspace 实现审计

## 1. 审计结论

当前 Host 存在三种名为 Workspace、但语义不同的对象：

1. `UserWorkspacePB`：账号/团队空间，包含成员、角色、类型、图标等。
2. `WorkspacePB`：AppFlowy Folder Collab 根，包含 `ViewPB` 页面树。
3. DSH `workspaceRoot/cwd`：Agent 文件工具和 sandbox 的目录边界。

三者目前靠当前用户、当前 workspace 和路径约定松散连接。它们不能继续共用一个“Workspace”概念，否则 SSH、云文件、多根目录和协作页面会持续互相污染。

## 2. 当前数据模型

### 2.1 用户空间

`flowy-user` 的 `UserWorkspacePB` 已有：

- `workspace_id/name/icon/member_count`；
- `role`；
- `LocalW/ServerW`；
- 创建、打开、重命名、删除、退出和订阅信息。

这部分与 Markdown 无关，应升级为目标模型中的 **Account Space**，继续作为身份、团队、策略和协作边界。

### 2.2 页面树

`flowy-folder` 的 `WorkspacePB` 只有 `id/name/views/create_time`。树节点 `ViewPB` 包含：

- `id/parent_view_id/name/child_views`；
- `layout`；
- 收藏、图标、创建/编辑人、锁定；
- `extra` 作为非结构化补充。

`ViewLayoutPB` 是封闭枚举：Document、Grid、Board、Calendar、Chat，以及后加的 Word、Excel、Slides、PDF。它把“资源是什么”与“用什么布局渲染”绑定，并要求每个新格式修改 Rust、protobuf、Flutter、同步和创建处理器。

结论：`ViewPB` 适合协作页面树，不适合作为通用目录项或文件身份。

### 2.3 侧栏实现

Desktop 侧栏由下列层级组成：

```text
HomeSideBar
  UserWorkspaceBloc       账号空间切换
  SidebarSectionsBloc     Public/Private 根 View
  SpaceBloc               AppFlowy Space 兼容层
  FolderBloc              仅持久化 section 展开状态
  ViewItem/ViewBloc       递归加载 ViewPB、拖拽和页面菜单
  TabsBloc                ViewPB.plugin() → Editor Tab
```

已有能力：

- 根 section 与嵌套页面展开/折叠；
- 新建子页面、重命名、复制、移动、收藏、删除、锁定；
- 拖拽排序；
- Public/Private/Space、Recent、Favorite、Trash；
- 页面更新监听和多端 Collab 同步；
- 点击、辅助键和中键打开 Tab。

不满足目标的问题：

- 节点类型固定为 `ViewPB`，没有 scheme、provider、capability、revision、size、etag 或 connection state；
- 展开状态按 `ViewPB.id`/section 写 KV，不能处理远端分页、断线和 mount 重连；
- “导入”会转换成 AppFlowy 页面，而不是挂载/保留文件原貌；
- 菜单由 `ViewMoreActionType` 静态枚举构造，第三方不能按资源能力注入；
- 文件树没有 watcher generation、分页 cursor、符号链接策略、ignore、冲突或离线状态；
- Tab 的最近项仍偏向 `ViewPB`，外部资源只能绕过 `latest view`。

### 2.4 DSH 现状

已有两条不同链路：

- AppFlowy `muse.workspace@1`：Rust Provider 把 View 页面树投影为最多 64 项、4 层的 `workspace.catalog`；Office 节点目前被省略。
- DSH `workspaceRoot/cwd`：文件工具、LSP、终端和 sandbox 使用真实目录；`workspace-files` 已能分页读文本、读 byte range、列目录和观察变化。

已经完成的 DSH 文件点击桥把 `resource.open` 消息交给 Host Resource Router，并在 Host 文件 Tab 中打开。缺口是：

- Host Explorer 与 DSH 文件系统不是同一个 Provider 实例；
- DSH session 的 cwd 没有稳定映射到 Project Workspace/Mount；
- 远端 credential、连接状态、执行位置没有进入统一能力模型；
- `workspace.catalog` 仍是页面目录，不是通用资源树。

## 3. 可复用基座

| 能力 | 处理方式 | 原因 |
|---|---|---|
| 用户、登录、Account Space、成员、角色 | 保留 | 与内容格式无关，是协作和授权基座 |
| AppFlowy Collab/Folder/Document/Database | 保留并 Provider 化 | 继续承载原生协作页面，不再冒充物理文件树 |
| Public/Private、分享、锁定、Trash | 保留语义，作用于支持该能力的 Provider | 不是所有文件系统都支持相同操作 |
| Host Bridge、authority revalidation | 保留 | 统一 DSH 与插件能力边界 |
| ResourceRef、Surface、Engine Adapter | 保留 | 是任意文件进入 Tab/编辑器的下游 |
| TabsBloc/PageManager | 演进 | 主体可复用，但身份和 session 恢复应从 ViewPB 解耦 |
| Flutter 主题、Popover、Tree 行样式 | 保留 | 保持产品视觉一致 |
| DSH FileSystem/WorkspaceFiles | 适配复用 | 已具有 provider-like 文件操作与分页读取语义 |

## 4. 必须替换或停止扩展

- 停止为每个外部格式增加 `ViewLayoutPB`。
- 停止把本地路径写入 `ViewPB.extra` 或用空 `ViewPB` 伪装外部资源。
- 停止由 Sidebar 维护第二套临时 `LOCAL RESOURCES` 列表。
- 停止把远端目录完整复制到 Collab Folder。
- 停止让 UI、DSH、引擎各自解析扩展名并决定权限。
- 停止用静态 Dart enum 定义所有 Explorer 菜单。

## 5. 兼容迁移

现有 AppFlowy 页面作为 `muse-collab://<account-space>/<section>/<view>` Mount 呈现；`ViewPB.id` 由该 Provider 私有保存，并映射成 opaque `entryRef/resourceRef`。原有页面分享、锁定和协作继续生效。

旧“Personal/Public/Private/Space”不是物理根目录，而是 `muse-collab` Provider 的虚拟目录。新 Explorer 可以同时展示：

```text
PROJECT: Product Launch
  ▾ Local · source-code
  ▾ SSH · staging
  ▾ Team Knowledge · Product
```

如此用户仍能访问原有知识页面，但不会误以为它们与本地文件具有相同保存、删除或执行语义。

