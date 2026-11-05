# Local Workspace P0 实现与联合验收

## 1. 本轮目标

建立一个可以真实验收的最小闭环，而不是继续使用内存文件列表：

```text
打开本地目录
  → Project Workspace Mount
  → Explorer 展开/折叠/文件操作
  → 文件在 Host 原生 Tab 打开
  → 保存纯文本版本
  → 修改文件
  → Semantic Diff
  → Host Diff Viewer
  → Version/Audit 重启恢复
  → DSH 使用相同目录作为 cwd
```

AppFlowy 原有页面树没有删除，以 `Team Knowledge` 区域继续承载 Public/Private/Space、协作、分享、锁定、权限和历史。Local P0 没有把本地文件伪装成 `ViewPB`。

## 2. 已实现能力

### 2.1 Workspace Domain

实现对象：

- `MuseWorkspaceMount`
- `MuseWorkspaceEntry`
- `MuseWorkspaceCapability`
- `MuseWorkspaceSnapshot`
- `MuseWorkspaceProvider`
- `MuseWorkspaceProviderRegistry`

身份分离：

- `mountRef` 表示目录与 Project Workspace 的挂载关系。
- `entryRef` 表示树中的位置。
- `resourceRef` 表示可打开和版本化的资源。
- 本地绝对路径只存在于设备侧 Provider/persistence/DSH hint，不进入 AppFlowy 协作页面模型。

### 2.2 Local Provider

已实现：

- bind 目录并 canonicalize root；
- stat/list，目录优先且名称排序；
- create file / create directory；
- rename；
- import 多文件，名称冲突自动分配副本名；
- delete，禁止删除 Mount root；
- recursive watch，250 ms debounce 后刷新已展开目录；
- capability 按 entry kind 和 mount read-only 状态生成；
- 拒绝 `../`、空名称和路径分隔符逃逸；
- symlink 不作为可递归目录展开。

当前文件操作在 Dart Local Provider 内执行，这是 P0 纵向切片。产品化阶段仍按架构迁移到 Rust Provider，以获得跨平台原子写、watch overflow、deadline/cancellation 和统一 TCK。

### 2.3 Workspace 生命周期

- 每个 AppFlowy Account Space 保存独立 Project Workspace definition。
- 通过系统目录选择器挂载真实目录。
- Mount、顺序与展开状态写入 Application Support。
- 切换 Account Space 时取消旧 watcher，并以 generation 隔离 late result。
- 重启/重建 Controller 后恢复 Mount 与根目录展开状态。
- “Remove Folder from Workspace”只解除 Mount，不删除物理目录。

### 2.4 Explorer UI

侧栏现在由两部分组成：

```text
PROJECT WORKSPACE
  project-a/
    src/
    README.md

TEAM KNOWLEDGE
  Public / Private / Space / AppFlowy pages
```

Explorer 支持：

- 根目录与任意子目录展开/折叠；
- 单击文件在 Host Tab 打开；
- 右键新建任意扩展名文件、新建目录、导入、刷新；
- 重命名、删除确认、Finder 定位、解除 Mount；
- 代码、Markdown、Office、图片等基础图标；
- 选中态、加载态、错误摘要；
- 多根目录 Mount。

### 2.5 Plugin Menu 复用

Workspace 文件节点不复制版本或引擎逻辑。右键菜单把文件解析为 `MuseResourceTabTarget`，然后读取同一个 `MuseResourceTabActionRegistry`：

- 使用 iOffice 打开；
- 使用 Helix 打开；
- 使用 Open File Viewer 打开；
- 保存当前版本；
- 与上一保存版本比较；
- 版本历史与审计；
- 复制路径。

外部插件向 Tab 注册的文件动作也会出现在 Workspace 文件菜单。

### 2.6 DSH 绑定

Flutter 写入的 Host hint 新增：

```json
{
  "appflowyWorkspaceId": "...",
  "title": "...",
  "projectRoot": "/canonical/local/root",
  "mounts": []
}
```

DSH `appflowy-workspace` 插件：

- 只接受绝对 `projectRoot`；
- 对本地 Project Workspace 直接注册真实目录为 DSH cwd；
- 不在用户目录写入管理用 README；
- 继续 pin 当前 Host Workspace，防止 DSH UI 删除绑定；
- 没有 Project Mount 时保持原 AppFlowy managed cwd 行为；
- 当前多根 Workspace 以第一个 Local Mount 作为 DSH primary cwd，其余 Mount 保留在 hint metadata，等待 Provider Broker v2。

因此 DSH 产生的文件路径与 Explorer 文件位于同一个 canonical root，点击对话资源路径时继续经过既有 containment 检查和 Resource Presentation seam。

## 3. 纯文本版本、审计与 Diff 联合链路

Workspace 文件节点的版本动作进入已实现的 Universal Version Plane：

1. `保存当前版本` 创建 committed Version 和 SHA-256 Blob。
2. 文件修改后，`与上一保存版本比较` 创建临时 working Version。
3. Text DiffProvider 生成稳定 Change/Hunk ID、增删统计和语义标签。
4. Host 打开 `filename · Diff` Tab。
5. Viewer 支持 Unified/Split、字符级变化、Hunk 折叠和导航。
6. Audit Ledger 追加 `version.capture` 和 `comparison.create`。
7. `版本历史与审计` 可选择历史版本与当前文件比较。

普通 Comparison 仍然只读。Proposal/Decision/三方 Mutation 未混入本轮验收。

## 4. 自动化验收

### 4.1 Workspace Provider

覆盖：

- 目录优先排序；
- capability；
- create/import/rename/delete；
- traversal 名称拒绝；
- Mount root 删除拒绝。

### 4.2 联合 E2E

`workspace_version_e2e_test.dart` 执行：

1. 创建真实临时 project directory；
2. 打开 Account Space；
3. Mount 目录；
4. 通过 Workspace Controller 创建 `plan.md`；
5. 验证 DSH publisher 收到 canonical root；
6. 销毁并重建 Controller；
7. 验证 Mount、expanded root 和文件树恢复；
8. 保存 `Workspace baseline` Version；
9. 修改 Markdown；
10. 生成 Comparison 和 semantic Hunk；
11. 验证 heading label、additions 和 audit events。

### 4.3 DSH Contract

TypeScript 插件新增测试验证：

- hint 的相对 project root 被拒绝；
- Host 选择的绝对 Project Workspace 成为 DSH cwd；
- DSH 不覆盖用户现有 `main.ts`；
- 不向真实 Project Workspace 注入 README。

## 5. 本轮验收矩阵

| 能力 | 自动化 | 构建 | 手工 UI |
|---|---:|---:|---:|
| Mount 本地目录 | 通过 | 通过 | 待发布前逐项执行 |
| 展开/折叠/持久化 | 通过 | 通过 | 待发布前逐项执行 |
| 新建/导入/重命名/删除 | 通过 | 通过 | 待发布前逐项执行 |
| Host Tab 打开格式路由 | 通过 | 通过 | 既有三引擎链路 |
| Workspace 共用插件菜单 | 静态分析/构建 | 通过 | 待发布前逐项执行 |
| 纯文本 Version/Audit/Diff | 通过 | 通过 | 待发布前逐项执行 |
| Unified/Split Viewer | Widget 通过 | 通过 | 待发布前逐项执行 |
| DSH primary cwd | 25 项 DSH suite 通过 | TS build 通过 | 待发布前逐项执行 |

## 6. 尚未完成的产品化能力

- Rust Workspace Provider/TCK、分页、deadline/cancellation、watch overflow。
- 正式 `AppFlowyCollabProvider`；当前 Team Knowledge 保留原实现并完成视觉/导航并置。
- SSH/SFTP/Cloud Provider。
- 多根 Mount 的 DSH Provider Broker；当前 DSH 使用 primary Local Mount。
- Workspace 通用 Command Registry；当前文件动作已经共用 Tab Action Registry，目录动作仍为 Host builtin。
- Preview Tab、拖放移动、全文搜索、100k 节点虚拟树。
- Proposal 接受/拒绝与三方合并。
- Office/媒体/CAD Semantic DiffProvider。
- Ontology Runtime；只保留稳定 refs 和事件扩展边界。

这些能力不得作为本轮 P0 验收结果对外宣称。

## 7. 代码入口

- `lib/workspace_platform/domain/workspace_models.dart`
- `lib/workspace_platform/infrastructure/workspace_provider.dart`
- `lib/workspace_platform/infrastructure/local_workspace_provider.dart`
- `lib/workspace_platform/infrastructure/workspace_persistence.dart`
- `lib/workspace_platform/application/workspace_controller.dart`
- `lib/workspace_platform/presentation/workspace_explorer.dart`
- `lib/plugins/dsh_agent/dsh_workspace_bridge.dart`
- `middlewares/dsh/plugins/appflowy-workspace/src/identity.ts`
- `test/workspace_platform/workspace_version_e2e_test.dart`

