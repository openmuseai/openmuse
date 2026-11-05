# Workspace 插件、菜单与 UX 设计

## 1. Workbench 信息架构

Desktop 保持当前 App 的三栏骨架：

```text
┌─ Sidebar / Explorer ─┬─ Editor Tabs & Surface ────────────┬─ DSH ───────┐
│ Account Space        │ [prd.docx] [retry.ts] [design.pdf] │ Conversation │
│ Project Workspace ▾  │ breadcrumb / resource toolbar      │ Context      │
│ ▾ Local · client     │                                     │ Changes      │
│ ▾ SSH · service      │ active engine surface               │              │
│ ▾ Team Knowledge     │                                     │              │
└──────────────────────┴─────────────────────────────────────┴──────────────┘
```

Account Space switcher 继续位于顶部用户区域；Project Workspace selector 放在 Explorer header。用户不会再看到临时 `LOCAL RESOURCES` 区域。

## 2. Explorer 结构

### Header

- Workspace title、连接/信任状态；
- `New/Open Workspace`；
- `Add Mount`；
- Refresh、Collapse All、More；
- Filter 输入框在激活后出现，不永久占用窄侧栏。

### Mount row

- Chevron、Provider icon、显示名；
- 本地/SSH/Cloud/Team badge；
- connecting/offline/auth/readonly 状态；
- Hover 显示 `New` 与 `…`；
- 右键使用同一 context menu。

### Entry row

- 20 px chevron 区，16 px 类型图标，单行名称；
- decoration 位于名称右侧：modified、conflict、agent-changed、shared；
- 连接问题和权限问题使用 icon + tooltip，不只使用灰色；
- active resource 使用当前 App 的 selected surface token；hover、focus、drop target 分开；
- 行高 28 px，默认每层缩进 12 px，最大视觉缩进 10 层，之后显示深度指示。

## 3. 基础交互

| 操作 | 行为 |
|---|---|
| 单击目录 | 展开/折叠；首次展开显示 skeleton，支持取消 |
| 单击文件 | Preview Tab；再次选择其他文件时复用未固定 Preview |
| 双击/Enter | 固定 Tab |
| Cmd/Ctrl+Click | 新固定 Tab，不切换现有布局策略 |
| 中键 | 新 Tab，遵循平台习惯 |
| 右键/`…` | 同一 Menu Registry 结果 |
| F2/Return | rename；Provider 不支持时显示原因 |
| Delete | 可恢复 Provider 进入 Trash；否则明确永久删除风险 |
| 拖放同 Mount | Provider move/copy capability 决定 |
| 拖放跨 Mount | copy transaction，成功后可选择删除源 |
| Reveal active file | 展开 ancestor pages，滚动并聚焦，不抢占编辑器焦点 |

## 4. Workspace 生命周期 UI

### Welcome/Empty

空窗口提供：

- Open Local Folder
- Connect over SSH
- Add Cloud Folder
- Open Recent Workspace
- Open Team Knowledge

按钮可由 Provider 插件贡献，但 Host 控制布局和最多 5 个主动作。

### 打开与恢复

- 先恢复工作区框架和 Tab 标题，再连接 Mount；
- 每个 Mount 独立显示 skeleton/state；一个远端失败不阻塞本地根；
- 上次 active file 不可用时保留 Tab，显示可操作错误页；
- 不自动弹出多次认证窗口，统一进入 Mount status center。

### 移除 Mount

移除只删除 Workspace binding，不删除源目录。若有 dirty Tab：

1. Save/Commit；
2. Save a Copy；
3. Keep Mount；
4. Discard（仅明确确认）。

## 5. 公共命令与菜单协议

现有 `PluginTabMenuContributor` 演进为位置无关协议：

```dart
enum WorkbenchMenuLocation {
  tabTitleInline,
  tabTitleContext,
  workspaceTitle,
  workspaceMountInline,
  workspaceMountContext,
  workspaceEntryInline,
  workspaceEntryContext,
  workspaceEmpty,
  commandPalette,
}

abstract interface class WorkbenchCommand {
  String get id;
  String get title;
  CommandEffect get effect; // read, mutate, execute, external
  JsonSchema get inputSchema;
  Future<CommandResult> invoke(CommandContext context, Object? input);
}

final class MenuContribution {
  final String contributionId;
  final String commandId;
  final WorkbenchMenuLocation location;
  final String group;
  final int order;
  final ContextExpression when;
  final ContextExpression? enabledWhen;
  final String? submenuId;
}
```

插件注册 command 和 contribution；菜单本身不持有业务 closure。这样同一命令可出现在 Tab、Explorer 和 Command Palette，并统一经过 policy、telemetry 和 dispose。

## 6. Context Keys

Host 维护类型化 context，不允许插件任意读取用户数据：

```text
workspace.providerKind       local | ssh-agent | sftp | cloud | muse-collab
workspace.trust              restricted | trusted
workspace.online             boolean
workspace.multiRoot          boolean
mount.capability             string[]
resource.scheme              opaque logical scheme
resource.kind                file | directory | virtual | ...
resource.format              canonical format ID
resource.dirty               boolean
resource.readonly            boolean
resource.engineCandidates    string[]
selection.count              number
selection.sameMount          boolean
agent.sessionBound           boolean
agent.hasPendingChanges      boolean
```

`when` 只决定可见性，`enabledWhen` 决定启用状态；真正 invoke 时 Host 必须重新鉴权，不能相信旧 context。

## 7. 默认菜单分组

### Entry context

```text
0_navigation   Open, Open to Side, Open With…
1_agent        Ask Agent, Add to Agent Context, Start Agent Here
2_create       New File, New Folder
3_edit         Rename, Cut, Copy, Paste
4_compare      Compare, Open Changes
5_transfer     Upload, Download, Save a Copy
6_path         Copy Resource Link, Copy Display Path, Reveal
7_manage       Pin, Add to Workspace, Provider actions
9_danger       Delete / Remove
```

### Mount context

```text
0_navigation   Open/Focus Root
1_agent        Start Agent in Mount
2_workspace    New File/Folder, Add Folder to Workspace
3_connection   Reconnect, Authentication, Remote Status
4_search       Find in Mount, Reindex
7_manage       Settings, Trust, Copy Mount Link
9_danger       Remove from Workspace
```

### Tab context

沿用已实现的 Close/Close Others/Pin 和格式引擎贡献；新增 Reveal in Explorer、Copy Resource Link、Move Tab、Split、Agent actions。`Open With…` 只显示 probe 成功且 policy 允许的 Engine Adapter。

## 8. 插件示例

```json
{
  "pluginId": "muse.ioffice",
  "commands": [{
    "id": "muse.ioffice.open",
    "title": "使用 iOffice 打开",
    "effect": "read"
  }],
  "menus": [
    {
      "id": "muse.ioffice.entry.open",
      "location": "workspaceEntryContext",
      "command": "muse.ioffice.open",
      "group": "0_navigation@30",
      "when": "resource.format == 'ooxml.word' && resource.engineCandidates contains 'muse.ioffice'"
    },
    {
      "id": "muse.ioffice.tab.open",
      "location": "tabTitleContext",
      "command": "muse.ioffice.open",
      "group": "3_open@30",
      "when": "resource.format == 'ooxml.word'"
    }
  ]
}
```

Helix、Open File Viewer 和未来 CAD/3D 插件使用相同结构。Explorer 不知道具体引擎名称。

## 9. 可访问性与视觉质量

- 支持键盘完整遍历、ARIA/Flutter Semantics、屏幕阅读器层级与展开状态；
- Focus ring 不被 Hover 覆盖；菜单支持方向键、Home/End、Esc、typeahead；
- 连接、只读、冲突、dirty 除颜色外必须有图标/文本；
- 文字截断保留 tooltip 和可复制安全路径；
- Light/Dark/High Contrast 均使用现有 AppFlowy color scheme token；
- 不直接复制 VS Code 黑色主题，保持当前圆角、字号、popover 和 surface 风格；
- 动画 120–180 ms，并尊重 reduced motion；大规模树滚动使用虚拟化，不做逐节点隐式动画。

## 10. 错误文案

错误必须包含：发生在哪里、保持了什么、下一步是什么。

- `SSH · staging 已断开。已打开文件保持只读缓存；重新连接后才能保存。 [重新连接]`
- `report.xlsx 在云端已更新。你的版本未覆盖。 [比较] [另存副本]`
- `此 Provider 不支持在原位置新建文件。 [选择其他 Mount]`
- `Workspace 处于受限模式；Agent 无法执行任务。 [管理信任]`

