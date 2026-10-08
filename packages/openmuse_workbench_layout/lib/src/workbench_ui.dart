import 'package:flutter/material.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';

enum OpenMusePaneAction {
  splitRight,
  splitDown,
  bindNewEditor,
  swapLeft,
  swapRight,
  swapUp,
  swapDown,
  close,
  reset,
}

/// The same menu is used by Desktop and Web; platform adapters handle actions.
final class OpenMusePaneMenu extends StatelessWidget {
  const OpenMusePaneMenu({
    super.key,
    required this.paneId,
    required this.hasBinding,
    required this.onAction,
    this.panelNames = const [],
    this.onBindPanel,
  });

  final String paneId;
  final bool hasBinding;
  final ValueChanged<OpenMusePaneAction> onAction;
  final List<String> panelNames;
  final ValueChanged<int>? onBindPanel;

  @override
  Widget build(BuildContext context) => PopupMenuButton<Object>(
    key: Key('pane-menu-button:$paneId'),
    tooltip: '窗格操作',
    padding: EdgeInsets.zero,
    iconSize: 17,
    splashRadius: 14,
    constraints: const BoxConstraints(minWidth: 26, minHeight: 26),
    style: const ButtonStyle(
      visualDensity: VisualDensity.compact,
      tapTargetSize: MaterialTapTargetSize.shrinkWrap,
      padding: WidgetStatePropertyAll(EdgeInsets.zero),
      minimumSize: WidgetStatePropertyAll(Size(26, 26)),
    ),
    color: Theme.of(context).colorScheme.surface,
    onSelected: (value) {
      if (value is OpenMusePaneAction) onAction(value);
      if (value is int) onBindPanel?.call(value);
    },
    itemBuilder: (_) => [
      const PopupMenuItem(
        value: OpenMusePaneAction.splitRight,
        child: Text('向右切分'),
      ),
      const PopupMenuItem(
        value: OpenMusePaneAction.splitDown,
        child: Text('向下切分'),
      ),
      const PopupMenuDivider(),
      const PopupMenuItem(
        value: OpenMusePaneAction.swapLeft,
        child: Text('与左侧交换'),
      ),
      const PopupMenuItem(
        value: OpenMusePaneAction.swapRight,
        child: Text('与右侧交换'),
      ),
      const PopupMenuItem(
        value: OpenMusePaneAction.swapUp,
        child: Text('与上方交换'),
      ),
      const PopupMenuItem(
        value: OpenMusePaneAction.swapDown,
        child: Text('与下方交换'),
      ),
      const PopupMenuDivider(),
      if (!hasBinding)
        const PopupMenuItem(
          value: OpenMusePaneAction.bindNewEditor,
          child: Text('绑定新编辑组'),
        ),
      for (var index = 0; index < panelNames.length; index++)
        PopupMenuItem(value: index, child: Text('绑定/交换 ${panelNames[index]}')),
      const PopupMenuDivider(),
      const PopupMenuItem(value: OpenMusePaneAction.close, child: Text('关闭窗格')),
      const PopupMenuItem(
        value: OpenMusePaneAction.reset,
        child: Text('重置默认布局'),
      ),
    ],
  );
}

final class OpenMuseBrandMark extends StatelessWidget {
  const OpenMuseBrandMark({super.key});

  @override
  Widget build(BuildContext context) => Container(
    width: 22,
    height: 22,
    decoration: BoxDecoration(
      color: const Color(0xffe8ebff),
      borderRadius: BorderRadius.circular(7),
    ),
    child: const Center(
      child: Icon(Icons.auto_awesome, color: OpenMuseTokens.accent, size: 13),
    ),
  );
}

final class OpenMuseSmallIconButton extends StatelessWidget {
  const OpenMuseSmallIconButton({
    super.key,
    required this.tooltip,
    required this.icon,
    required this.onPressed,
  });

  final String tooltip;
  final IconData icon;
  final VoidCallback? onPressed;

  @override
  Widget build(BuildContext context) => IconButton(
    tooltip: tooltip,
    visualDensity: VisualDensity.compact,
    constraints: const BoxConstraints.tightFor(width: 30, height: 30),
    padding: EdgeInsets.zero,
    style: IconButton.styleFrom(backgroundColor: Colors.transparent),
    onPressed: onPressed,
    icon: Icon(icon, size: 17, color: OpenMuseTokens.textMuted),
  );
}

final class OpenMuseSidebarAction extends StatelessWidget {
  const OpenMuseSidebarAction({
    super.key,
    required this.icon,
    required this.label,
    required this.onTap,
    this.shortcut,
  });

  final IconData icon;
  final String label;
  final String? shortcut;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) => Material(
    color: Colors.transparent,
    child: InkWell(
      borderRadius: BorderRadius.circular(6),
      onTap: onTap,
      child: SizedBox(
        height: OpenMuseTokens.itemHeight,
        child: Row(
          children: [
            const SizedBox(width: 4),
            Icon(icon, size: 17, color: OpenMuseTokens.textMuted),
            const SizedBox(width: 9),
            Text(
              label,
              style: OpenMuseTokens.compactText.copyWith(
                color: Theme.of(context).colorScheme.onSurface,
              ),
            ),
            if (shortcut != null) ...[
              const Spacer(),
              Text(
                shortcut!,
                style: const TextStyle(
                  color: OpenMuseTokens.textMuted,
                  fontSize: 10,
                ),
              ),
              const SizedBox(width: 4),
            ],
          ],
        ),
      ),
    ),
  );
}

final class OpenMuseSidebarFooterAction extends StatelessWidget {
  const OpenMuseSidebarFooterAction({
    super.key,
    required this.icon,
    required this.label,
    this.onTap,
  });

  final IconData icon;
  final String label;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) => Material(
    color: Colors.transparent,
    child: InkWell(
      borderRadius: BorderRadius.circular(6),
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 7, horizontal: 4),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(icon, size: 15, color: OpenMuseTokens.textMuted),
            const SizedBox(width: 6),
            Text(
              label,
              style: OpenMuseTokens.compactText.copyWith(
                color: Theme.of(context).colorScheme.onSurface,
              ),
            ),
          ],
        ),
      ),
    ),
  );
}

/// Desktop's sidebar structure. Data and file operations come from adapters.
final class OpenMuseWorkspaceSidebar extends StatelessWidget {
  const OpenMuseWorkspaceSidebar({
    super.key,
    required this.projectExpanded,
    required this.onToggleProject,
    required this.tree,
    this.showBrand = true,
    this.onSettings,
    this.onToggleSidebar,
    this.onSearch,
    this.onAddWorkspace,
    this.onPlugins,
    this.onTrash,
  });

  final bool projectExpanded;
  final VoidCallback onToggleProject;
  final Widget tree;
  final bool showBrand;
  final VoidCallback? onSettings;
  final VoidCallback? onToggleSidebar;
  final VoidCallback? onSearch;
  final VoidCallback? onAddWorkspace;
  final VoidCallback? onPlugins;
  final VoidCallback? onTrash;

  @override
  Widget build(BuildContext context) => ColoredBox(
    color: Theme.of(context).brightness == Brightness.light
        ? OpenMuseTokens.sidebar
        : const Color(0xff202228),
    child: SafeArea(
      child: Padding(
        padding: const EdgeInsets.fromLTRB(10, 8, 10, 10),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            if (showBrand) ...[
              Row(
                children: [
                  const OpenMuseBrandMark(),
                  const SizedBox(width: 8),
                  const Expanded(
                    child: Text(
                      'OpenMuse',
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ),
                  OpenMuseSmallIconButton(
                    tooltip: '本地设置',
                    icon: Icons.settings_outlined,
                    onPressed: onSettings,
                  ),
                  OpenMuseSmallIconButton(
                    tooltip: '收起侧栏',
                    icon: Icons.view_sidebar_outlined,
                    onPressed: onToggleSidebar,
                  ),
                ],
              ),
              const SizedBox(height: 11),
            ],
            OpenMuseSidebarAction(
              icon: Icons.search,
              label: '搜索',
              shortcut: '⌘ K',
              onTap: onSearch,
            ),
            const SizedBox(height: 20),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 4),
              child: Row(
                children: [
                  Expanded(
                    child: InkWell(
                      key: const Key('project-workspace-toggle'),
                      borderRadius: BorderRadius.circular(6),
                      onTap: onToggleProject,
                      child: SizedBox(
                        height: 30,
                        child: Row(
                          children: [
                            Flexible(
                              child: Text(
                                'Project Workspace',
                                overflow: TextOverflow.ellipsis,
                                style: OpenMuseTokens.compactText.copyWith(
                                  color: Theme.of(
                                    context,
                                  ).colorScheme.onSurface,
                                ),
                              ),
                            ),
                            const SizedBox(width: 4),
                            Icon(
                              projectExpanded
                                  ? Icons.keyboard_arrow_down
                                  : Icons.chevron_right,
                              size: 14,
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                  OpenMuseSmallIconButton(
                    tooltip: '添加工作区',
                    icon: Icons.add,
                    onPressed: onAddWorkspace,
                  ),
                ],
              ),
            ),
            const SizedBox(height: 7),
            Expanded(child: projectExpanded ? tree : const SizedBox.shrink()),
            const Divider(height: 18),
            Row(
              children: [
                Expanded(
                  child: OpenMuseSidebarFooterAction(
                    icon: Icons.extension_outlined,
                    label: '插件',
                    onTap: onPlugins,
                  ),
                ),
                const SizedBox(width: 6),
                OpenMuseSidebarFooterAction(
                  icon: Icons.delete_outline,
                  label: '回收站',
                  onTap: onTrash,
                ),
              ],
            ),
          ],
        ),
      ),
    ),
  );
}

final class OpenMuseWorkspaceRow extends StatelessWidget {
  const OpenMuseWorkspaceRow({
    super.key,
    required this.name,
    required this.isDirectory,
    required this.expanded,
    required this.depth,
    required this.isMount,
    required this.onTap,
    this.onSecondaryTapDown,
    this.selected = false,
    this.loading = false,
    this.extension = '',
  });

  final String name;
  final bool isDirectory;
  final bool expanded;
  final int depth;
  final bool isMount;
  final bool selected;
  final bool loading;
  final String extension;
  final VoidCallback onTap;
  final ValueChanged<TapDownDetails>? onSecondaryTapDown;

  static IconData iconFor(String extension) =>
      switch (extension.toLowerCase()) {
        'png' || 'jpg' || 'jpeg' => Icons.image_outlined,
        'pdf' => Icons.picture_as_pdf_outlined,
        'native-gate' => Icons.developer_board_outlined,
        'md' => Icons.notes_outlined,
        _ => Icons.insert_drive_file_outlined,
      };

  @override
  Widget build(BuildContext context) => Padding(
    padding: EdgeInsets.only(bottom: isMount ? 0 : 1),
    child: Material(
      color: selected
          ? (Theme.of(context).brightness == Brightness.dark
                ? const Color(0xff383c47)
                : OpenMuseTokens.sidebarSelected)
          : Colors.transparent,
      borderRadius: BorderRadius.circular(6),
      child: InkWell(
        borderRadius: BorderRadius.circular(6),
        onTap: onTap,
        onSecondaryTapDown: onSecondaryTapDown,
        child: SizedBox(
          height: isMount ? OpenMuseTokens.itemHeight : 30,
          child: Row(
            children: [
              SizedBox(width: isMount ? 0 : 4 + depth * 12),
              if (isDirectory)
                Icon(
                  expanded ? Icons.keyboard_arrow_down : Icons.chevron_right,
                  size: 15,
                  color: isMount ? null : OpenMuseTokens.textMuted,
                )
              else
                const SizedBox(width: 15),
              Icon(
                isDirectory
                    ? (expanded
                          ? Icons.folder_open_outlined
                          : Icons.folder_outlined)
                    : iconFor(extension),
                size: 16,
                color: OpenMuseTokens.textMuted,
              ),
              SizedBox(width: isMount ? 6 : 7),
              Expanded(
                child: Text(
                  name,
                  overflow: TextOverflow.ellipsis,
                  style: isMount
                      ? const TextStyle(
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                        )
                      : OpenMuseTokens.compactText.copyWith(
                          color: Theme.of(context).colorScheme.onSurface,
                        ),
                ),
              ),
              if (loading)
                const SizedBox.square(
                  dimension: 12,
                  child: CircularProgressIndicator(strokeWidth: 1.5),
                ),
              SizedBox(width: isMount ? 4 : 8),
            ],
          ),
        ),
      ),
    ),
  );
}

final class OpenMuseEditorTabData {
  const OpenMuseEditorTabData({
    required this.id,
    required this.title,
    required this.extension,
    this.pinned = false,
    this.isDiff = false,
  });
  final String id;
  final String title;
  final String extension;
  final bool pinned;
  final bool isDiff;
}

final class OpenMuseEditorTabStrip extends StatelessWidget {
  const OpenMuseEditorTabStrip({
    super.key,
    required this.tabs,
    required this.activeId,
    required this.onActivate,
    required this.onClose,
    this.paneMenu,
    this.onContextMenu,
    this.onExpandSidebar,
  });

  final List<OpenMuseEditorTabData> tabs;
  final String? activeId;
  final ValueChanged<String> onActivate;
  final ValueChanged<String> onClose;
  final Widget? paneMenu;
  final void Function(BuildContext, Offset, String)? onContextMenu;
  final VoidCallback? onExpandSidebar;

  @override
  Widget build(BuildContext context) => Container(
    height: OpenMuseTokens.topBarHeight,
    decoration: BoxDecoration(
      color: Theme.of(context).brightness == Brightness.dark
          ? const Color(0xff202228)
          : const Color(0xfff7f8fb),
      border: Border(bottom: BorderSide(color: Theme.of(context).dividerColor)),
    ),
    child: Row(
      children: [
        if (onExpandSidebar != null)
          OpenMuseSmallIconButton(
            tooltip: '展开侧栏',
            icon: Icons.view_sidebar_outlined,
            onPressed: onExpandSidebar,
          ),
        Expanded(
          child: ListView(
            scrollDirection: Axis.horizontal,
            children: [
              if (tabs.isEmpty)
                SizedBox(
                  width: 112,
                  child: Center(
                    child: Text(
                      'Blank page',
                      style: OpenMuseTokens.compactText.copyWith(
                        color: Theme.of(context).colorScheme.onSurface,
                      ),
                    ),
                  ),
                ),
              for (final tab in tabs)
                _OpenMuseEditorTab(
                  tab: tab,
                  active: tab.id == activeId,
                  onActivate: () => onActivate(tab.id),
                  onClose: () => onClose(tab.id),
                  onContextMenu: onContextMenu == null
                      ? null
                      : (position) => onContextMenu!(context, position, tab.id),
                ),
            ],
          ),
        ),
        ?paneMenu,
        const SizedBox(width: 4),
      ],
    ),
  );
}

final class _OpenMuseEditorTab extends StatefulWidget {
  const _OpenMuseEditorTab({
    required this.tab,
    required this.active,
    required this.onActivate,
    required this.onClose,
    this.onContextMenu,
  });
  final OpenMuseEditorTabData tab;
  final bool active;
  final VoidCallback onActivate;
  final VoidCallback onClose;
  final ValueChanged<Offset>? onContextMenu;

  @override
  State<_OpenMuseEditorTab> createState() => _OpenMuseEditorTabState();
}

final class _OpenMuseEditorTabState extends State<_OpenMuseEditorTab> {
  bool hovered = false;

  @override
  Widget build(BuildContext context) {
    final tab = widget.tab;
    final active = widget.active;
    return MouseRegion(
      onEnter: (_) => setState(() => hovered = true),
      onExit: (_) => setState(() => hovered = false),
      child: GestureDetector(
        onSecondaryTapDown: (details) =>
            widget.onContextMenu?.call(details.globalPosition),
        child: Material(
          color: Theme.of(context).brightness == Brightness.dark
              ? (active ? const Color(0xff292c34) : const Color(0xff202228))
              : (active ? Colors.white : const Color(0xfff7f8fb)),
          child: InkWell(
            onTap: widget.onActivate,
            child: Container(
              constraints: BoxConstraints(
                minWidth: tab.pinned ? 54 : (active ? 128 : 88),
                maxWidth: tab.pinned ? 54 : 168,
              ),
              padding: EdgeInsets.only(left: active ? 14 : 10, right: 4),
              decoration: BoxDecoration(
                border: Border(
                  right: BorderSide(color: Theme.of(context).dividerColor),
                ),
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(
                    tab.isDiff
                        ? Icons.difference_outlined
                        : OpenMuseWorkspaceRow.iconFor(tab.extension),
                    size: 15,
                    color: OpenMuseTokens.textMuted,
                  ),
                  if (!tab.pinned) ...[
                    const SizedBox(width: 6),
                    Flexible(
                      child: Text(
                        tab.title,
                        overflow: TextOverflow.ellipsis,
                        style: OpenMuseTokens.compactText.copyWith(
                          color: Theme.of(context).colorScheme.onSurface,
                        ),
                      ),
                    ),
                  ],
                  if (!tab.pinned && (active || hovered))
                    IconButton(
                      tooltip: '关闭',
                      visualDensity: VisualDensity.compact,
                      constraints: const BoxConstraints.tightFor(
                        width: 24,
                        height: 24,
                      ),
                      padding: EdgeInsets.zero,
                      onPressed: widget.onClose,
                      icon: const Icon(Icons.close, size: 14),
                    ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

final class OpenMuseWorkbenchWelcome extends StatelessWidget {
  const OpenMuseWorkbenchWelcome({super.key, this.onSearch, this.onCreate});
  final VoidCallback? onSearch;
  final VoidCallback? onCreate;

  @override
  Widget build(BuildContext context) => Center(
    child: ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 420),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 48,
            height: 48,
            decoration: BoxDecoration(
              color: const Color(0xffeef0ff),
              borderRadius: BorderRadius.circular(14),
            ),
            child: const Icon(
              Icons.auto_awesome,
              color: OpenMuseTokens.accent,
              size: 24,
            ),
          ),
          const SizedBox(height: 16),
          const Text(
            '从工作区开始',
            style: TextStyle(fontSize: 19, fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 7),
          const Text(
            '文档保存在工作区。编辑器、Viewer 与助手按需由插件激活。',
            textAlign: TextAlign.center,
            style: TextStyle(color: OpenMuseTokens.textMuted, height: 1.45),
          ),
          const SizedBox(height: 18),
          Wrap(
            alignment: WrapAlignment.center,
            spacing: 10,
            runSpacing: 8,
            children: [
              OutlinedButton.icon(
                onPressed: onSearch,
                icon: const Icon(Icons.search, size: 16),
                label: const Text('搜索资源'),
              ),
              FilledButton.icon(
                onPressed: onCreate,
                icon: const Icon(Icons.add, size: 16),
                label: const Text('新建文档'),
              ),
            ],
          ),
        ],
      ),
    ),
  );
}
