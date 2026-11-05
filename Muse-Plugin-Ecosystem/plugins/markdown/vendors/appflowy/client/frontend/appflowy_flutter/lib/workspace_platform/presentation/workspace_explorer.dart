import 'dart:async';
import 'dart:io';

import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/plugins/resource_surface/resource_file_plugin.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/plugins/resource_surface/resource_surface_dialog.dart';
import 'package:appflowy/plugins/resource_surface/resource_tab_action_registry.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:appflowy/workspace/presentation/home/home_sizes.dart';
import 'package:appflowy/workspace/presentation/widgets/dialog_v2.dart';
import 'package:appflowy/workspace/presentation/widgets/muse_context_menu.dart';
import 'package:appflowy/workspace_platform/application/workspace_controller.dart';
import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:appflowy/workspace/presentation/home/menu/sidebar/shared/sidebar_stack_panes.dart';
import 'package:flowy_infra/file_picker/file_picker_service.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path/path.dart' as p;

const _kWorkspaceRowHeight = 30.0;
const _kWorkspaceIndent = 10.0;

final class MuseWorkspaceExplorer extends StatefulWidget {
  const MuseWorkspaceExplorer({
    super.key,
    required this.accountSpaceRef,
    required this.accountSpaceTitle,
    this.expanded,
    this.onExpandedChanged,
    this.fillRemaining = false,
    this.onContentHeight,
    this.onActivate,
  });

  final String accountSpaceRef;
  final String accountSpaceTitle;
  final bool? expanded;
  final ValueChanged<bool>? onExpandedChanged;
  final bool fillRemaining;
  final ValueChanged<double>? onContentHeight;
  final VoidCallback? onActivate;

  @override
  State<MuseWorkspaceExplorer> createState() => _MuseWorkspaceExplorerState();
}

final class _MuseWorkspaceExplorerState extends State<MuseWorkspaceExplorer> {
  late final MuseWorkspaceController _controller;
  bool _sectionExpanded = true;

  @override
  void initState() {
    super.initState();
    _controller = getIt<MuseWorkspaceController>();
    unawaited(_open());
  }

  @override
  void didUpdateWidget(covariant MuseWorkspaceExplorer oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.accountSpaceRef != widget.accountSpaceRef ||
        oldWidget.accountSpaceTitle != widget.accountSpaceTitle) {
      unawaited(_open());
    }
    if ((oldWidget.expanded ?? true) && widget.expanded == false) {
      widget.onContentHeight?.call(0);
    }
  }

  Future<void> _open() async {
    await _controller.open(
      accountSpaceRef: widget.accountSpaceRef,
      title: widget.accountSpaceTitle,
    );
  }

  bool get _expanded => widget.expanded ?? _sectionExpanded;

  void _toggleExpanded() {
    final next = !_expanded;
    final onChanged = widget.onExpandedChanged;
    if (onChanged != null) {
      onChanged(next);
    } else {
      setState(() => _sectionExpanded = next);
    }
  }

  @override
  Widget build(BuildContext context) => AnimatedBuilder(
        animation: _controller,
        builder: (context, _) {
          final hasMounts = _controller.mounts.isNotEmpty;
          final body = _controller.loading
              ? const Padding(
                  padding: EdgeInsets.symmetric(vertical: 12),
                  child: Center(
                    child: SizedBox.square(
                      dimension: 18,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    ),
                  ),
                )
              : hasMounts
                  ? _buildTree(context)
                  : _buildEmpty(context);
          final measured = _expanded
              ? SidebarContentHeightReporter(
                  onHeight: (height) => widget.onContentHeight?.call(height),
                  child: body,
                )
              : body;
          return LayoutBuilder(
            builder: (context, constraints) {
              // A bounded max height means this pane was given a cap by the
              // sidebar stack. The tree must scroll inside it — never grow
              // the parent Column (that paints the yellow overflow bar).
              final bounded = widget.fillRemaining ||
                  (constraints.hasBoundedHeight &&
                      constraints.maxHeight.isFinite);
              return Column(
                mainAxisSize: bounded ? MainAxisSize.max : MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  _buildHeader(context),
                  if (_expanded) const VSpace(4),
                  if (_expanded)
                    bounded
                        ? Expanded(
                            child: SingleChildScrollView(child: measured),
                          )
                        : measured,
                ],
              );
            },
          );
        },
      );

  Widget _buildHeader(BuildContext context) {
    return SizedBox(
      height: HomeSizes.workspaceSectionHeight,
      child: Padding(
        padding: const EdgeInsets.only(left: 6, right: 4),
        child: FlowyButton(
          margin: const EdgeInsets.only(left: 6, right: 4),
          onTap: _toggleExpanded,
          text: Row(
            children: [
              const FlowyText('Project Workspace'),
              const HSpace(4),
              FlowySvg(
                _expanded
                    ? FlowySvgs.workspace_drop_down_menu_show_s
                    : FlowySvgs.workspace_drop_down_menu_hide_s,
              ),
            ],
          ),
          rightIcon: FlowyTooltip(
            message: 'Open folder as workspace',
            child: FlowyIconButton(
              width: 24,
              iconPadding: const EdgeInsets.all(4),
              icon: const FlowySvg(FlowySvgs.view_item_add_s),
              onPressed: _pickDirectory,
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildEmpty(BuildContext context) => Padding(
        padding: const EdgeInsets.fromLTRB(12, 4, 12, 8),
        child: FlowyButton(
          onTap: _pickDirectory,
          margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
          leftIcon: const Icon(Icons.folder_open_outlined, size: 16),
          text: const FlowyText(
            'Open a folder to browse files and versions',
            fontSize: 13,
            maxLines: 2,
          ),
        ),
      );

  Widget _buildTree(BuildContext context) => Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          for (final mount in _controller.mounts) ...[
            if (_controller.roots[mount.mountRef] case final root?)
              _EntryNode(
                controller: _controller,
                entry: root,
                depth: 0,
                onOpen: _openEntry,
                onContextMenu: _showEntryMenu,
              ),
            const VSpace(2),
          ],
          if (_controller.error case final error?)
            Padding(
              padding: const EdgeInsets.all(8),
              child: FlowyText(
                error,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                color: Theme.of(context).colorScheme.error,
              ),
            ),
        ],
      );

  Future<void> _pickDirectory() async {
    final path = await getIt<FilePickerService>().getDirectoryPath(
      title: 'Open Project Workspace',
    );
    if (path == null || path.isEmpty) return;
    await _guard(() => _controller.mountLocalDirectory(path));
  }

  Future<void> _openEntry(MuseWorkspaceEntry entry) async {
    _controller.select(entry.entryRef);
    if (entry.isDirectory) {
      widget.onActivate?.call();
      await _controller.toggleExpanded(entry);
      return;
    }
    if (!entry.isFile || !mounted) return;
    await MuseResourceSurfaceOpener.open(
      context,
      MuseResourceOpenRequest(
        path: entry.locator,
        origin: MuseResourceOpenOrigin.hostPicker,
      ),
    );
  }

  Future<void> _showEntryMenu(
    MuseWorkspaceEntry entry,
    Offset globalPosition,
  ) async {
    final pluginActions = <PluginTabMenuAction>[];
    _ExplorerResourceTarget? resourceTarget;
    if (entry.isFile) {
      try {
        final resolved = await MuseLocalResourceRouter().resolve(
          MuseResourceOpenRequest(
            path: entry.locator,
            origin: MuseResourceOpenOrigin.hostPicker,
          ),
        );
        resourceTarget = _ExplorerResourceTarget(resolved);
        pluginActions.addAll(
          getIt<MuseResourceTabActionRegistry>()
              .actionsFor(resourceTarget)
              .where((action) => action.id != 'host.resource.reveal'),
        );
      } on Object {
        // Generic workspace actions still remain available.
      }
    }
    if (!mounted) return;
    final hostCopyIds = {
      'host.resource.copy-path',
      'host.resource.copy-relative-path',
    };
    pluginActions.removeWhere((action) => hostCopyIds.contains(action.id));
    final items = <MuseContextMenuEntry>[
      if (entry.isFile)
        const MuseContextMenuAction(
          id: 'workspace.open',
          label: 'Open',
          icon: Icons.open_in_new,
        ),
      if (entry.isDirectory) ...[
        const MuseContextMenuAction(
          id: 'workspace.new-file',
          label: 'New File…',
          icon: Icons.note_add_outlined,
        ),
        const MuseContextMenuAction(
          id: 'workspace.new-directory',
          label: 'New Folder…',
          icon: Icons.create_new_folder_outlined,
        ),
        const MuseContextMenuAction(
          id: 'workspace.import',
          label: 'Import Files…',
          icon: Icons.file_upload_outlined,
        ),
        const MuseContextMenuAction(
          id: 'workspace.refresh',
          label: 'Refresh',
          icon: Icons.refresh,
        ),
      ],
      if (pluginActions.isNotEmpty) const MuseContextMenuDivider(),
      for (final action in pluginActions)
        MuseContextMenuAction(
          id: 'plugin:${action.id}',
          label: action.label,
          icon: action.icon,
          enabled: action.enabled,
          submenuBuilder: action.submenuBuilder == null
              ? null
              : (ctx, close) => action.submenuBuilder!(ctx, close),
        ),
      const MuseContextMenuDivider(),
      const MuseContextMenuAction(
        id: 'workspace.copy-path',
        label: 'Copy Path',
        icon: Icons.content_copy_outlined,
      ),
      const MuseContextMenuAction(
        id: 'workspace.copy-relative-path',
        label: 'Copy Relative Path',
        icon: Icons.account_tree_outlined,
      ),
      const MuseContextMenuDivider(),
      if (entry.parentEntryRef != null)
        const MuseContextMenuAction(
          id: 'workspace.rename',
          label: 'Rename…',
          icon: Icons.drive_file_rename_outline,
        ),
      const MuseContextMenuAction(
        id: 'workspace.reveal',
        label: 'Reveal in Finder',
        icon: Icons.folder_open_outlined,
      ),
      if (entry.parentEntryRef != null)
        const MuseContextMenuAction(
          id: 'workspace.delete',
          label: 'Delete',
          icon: Icons.delete_outline,
          destructive: true,
        )
      else
        const MuseContextMenuAction(
          id: 'workspace.unmount',
          label: 'Remove Folder from Workspace',
          icon: Icons.link_off,
        ),
    ];
    final selected = await showMuseContextMenu(
      context: context,
      globalPosition: globalPosition,
      entries: items,
    );
    if (!mounted || selected == null) return;
    if (selected.startsWith('plugin:')) {
      final id = selected.substring('plugin:'.length);
      final action =
          pluginActions.where((candidate) => candidate.id == id).firstOrNull;
      if (action != null) await action.invoke(context);
      return;
    }
    await _runWorkspaceAction(selected, entry);
  }

  Future<void> _runWorkspaceAction(
    String action,
    MuseWorkspaceEntry entry,
  ) async {
    switch (action) {
      case 'workspace.open':
        await _openEntry(entry);
        return;
      case 'workspace.new-file':
        final name =
            await _prompt('New File', 'File name, including extension');
        if (name != null) {
          await _guard(() => _controller.createFile(entry, name));
        }
        return;
      case 'workspace.new-directory':
        final name = await _prompt('New Folder', 'Folder name');
        if (name != null) {
          await _guard(() => _controller.createDirectory(entry, name));
        }
        return;
      case 'workspace.import':
        final result = await getIt<FilePickerService>().pickFiles(
          dialogTitle: 'Import files into ${entry.name}',
          allowMultiple: true,
        );
        final files = result?.files
                .map((file) => file.path)
                .whereType<String>()
                .map(File.new)
                .toList(growable: false) ??
            const <File>[];
        if (files.isNotEmpty) {
          await _guard(() => _controller.importFiles(entry, files));
        }
        return;
      case 'workspace.refresh':
        await _guard(() => _controller.refresh(entry));
        return;
      case 'workspace.copy-path':
        await Clipboard.setData(ClipboardData(text: entry.locator));
        return;
      case 'workspace.copy-relative-path':
        await Clipboard.setData(ClipboardData(text: _relativePath(entry)));
        return;
      case 'workspace.rename':
        final name = await _prompt('Rename', 'New name', initial: entry.name);
        if (name != null && name != entry.name) {
          await _guard(() => _controller.rename(entry, name));
        }
        return;
      case 'workspace.reveal':
        await Process.run('open', ['-R', entry.locator]);
        return;
      case 'workspace.delete':
        final confirmed = await _confirmDelete(entry);
        if (confirmed) await _guard(() => _controller.delete(entry));
        return;
      case 'workspace.unmount':
        await _guard(() => _controller.unmount(entry.mountRef));
        return;
    }
  }

  Future<String?> _prompt(
    String title,
    String hint, {
    String initial = '',
  }) async {
    final result = await showAFTextFieldDialog(
      context: context,
      title: title,
      initialValue: initial,
      hintText: hint,
    );
    final trimmed = result?.trim();
    return trimmed == null || trimmed.isEmpty ? null : trimmed;
  }

  Future<bool> _confirmDelete(MuseWorkspaceEntry entry) async {
    var confirmed = false;
    await showSimpleAFDialog(
      context: context,
      title: 'Delete ${entry.name}?',
      content: entry.isDirectory
          ? 'This permanently deletes the folder and all its contents.'
          : 'This permanently deletes the file.',
      isDestructive: true,
      primaryAction: (
        'Delete',
        (_) => confirmed = true,
      ),
      secondaryAction: ('Cancel', null),
    );
    return confirmed;
  }

  String _relativePath(MuseWorkspaceEntry entry) {
    final mount = _controller.mounts
        .where((candidate) => candidate.mountRef == entry.mountRef)
        .firstOrNull;
    if (mount == null) return entry.name;
    if (p.equals(mount.rootLocator, entry.locator)) return '.';
    if (p.isWithin(mount.rootLocator, entry.locator)) {
      return p.relative(entry.locator, from: mount.rootLocator);
    }
    return entry.locator;
  }

  Future<T?> _guard<T>(Future<T> Function() operation) async {
    try {
      return await operation();
    } on Object catch (error) {
      if (!mounted) return null;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Workspace operation failed: $error')),
      );
      return null;
    }
  }
}

final class _EntryNode extends StatelessWidget {
  const _EntryNode({
    required this.controller,
    required this.entry,
    required this.depth,
    required this.onOpen,
    required this.onContextMenu,
  });

  final MuseWorkspaceController controller;
  final MuseWorkspaceEntry entry;
  final int depth;
  final Future<void> Function(MuseWorkspaceEntry) onOpen;
  final Future<void> Function(MuseWorkspaceEntry, Offset) onContextMenu;

  @override
  Widget build(BuildContext context) {
    final expanded = controller.expandedEntryRefs.contains(entry.entryRef);
    final loading = controller.loadingEntryRefs.contains(entry.entryRef);
    final selected = controller.selectedEntryRef == entry.entryRef;
    final childEntries = controller.children[entry.entryRef] ?? const [];
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        GestureDetector(
          behavior: HitTestBehavior.opaque,
          onSecondaryTapDown: (details) =>
              onContextMenu(entry, details.globalPosition),
          child: SizedBox(
            height: _kWorkspaceRowHeight,
            child: ClipRect(
              child: FlowyButton(
                isSelected: selected,
                onTap: () => onOpen(entry),
                margin: EdgeInsets.only(
                  left: 6.0 + depth * _kWorkspaceIndent,
                  right: 4,
                ),
                text: Row(
                  children: [
                    SizedBox(
                      width: 16,
                      child: entry.isDirectory
                          ? FlowySvg(
                              expanded
                                  ? FlowySvgs.workspace_drop_down_menu_show_s
                                  : FlowySvgs.workspace_drop_down_menu_hide_s,
                            )
                          : null,
                    ),
                    const HSpace(2),
                    Icon(
                      _icon(entry),
                      size: 14,
                      color: _iconColor(context, entry),
                    ),
                    const HSpace(4),
                    Expanded(
                      child: FlowyText(
                        entry.name,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                    if (loading)
                      const SizedBox.square(
                        dimension: 12,
                        child: CircularProgressIndicator(strokeWidth: 1.5),
                      ),
                  ],
                ),
              ),
            ),
          ),
        ),
        if (expanded)
          for (final child in childEntries)
            _EntryNode(
              controller: controller,
              entry: child,
              depth: depth + 1,
              onOpen: onOpen,
              onContextMenu: onContextMenu,
            ),
      ],
    );
  }

  IconData _icon(MuseWorkspaceEntry entry) {
    if (entry.kind == MuseWorkspaceEntryKind.directory) {
      return Icons.folder_outlined;
    }
    if (entry.kind == MuseWorkspaceEntryKind.symlink) return Icons.link;
    final extension = p.extension(entry.name).toLowerCase();
    if ({'.md', '.txt'}.contains(extension)) return Icons.notes_outlined;
    if ({'.rs', '.ts', '.js', '.java', '.c', '.cpp', '.dart', '.py'}
        .contains(extension)) {
      return Icons.code;
    }
    if ({'.doc', '.docx', '.pdf', '.ppt', '.pptx', '.xls', '.xlsx'}
        .contains(extension)) {
      return Icons.description_outlined;
    }
    if ({'.png', '.jpg', '.jpeg', '.gif', '.svg', '.webp'}
        .contains(extension)) {
      return Icons.image_outlined;
    }
    return Icons.insert_drive_file_outlined;
  }

  Color? _iconColor(BuildContext context, MuseWorkspaceEntry entry) {
    if (!entry.isDirectory) return null;
    final base = Theme.of(context).iconTheme.color ??
        Theme.of(context).colorScheme.onSurface;
    return base.withValues(alpha: 0.55);
  }
}

final class _ExplorerResourceTarget implements MuseResourceTabTarget {
  _ExplorerResourceTarget(this.resource) : _selectedEngine = resource.engine;

  @override
  final MuseResolvedResource resource;

  MuseLocalEngine _selectedEngine;

  @override
  MuseLocalEngine get selectedEngine => _selectedEngine;

  @override
  void selectEngine(MuseLocalEngine engine) {
    _selectedEngine = engine;
    final resolved = MuseResolvedResource(
      file: resource.file,
      engine: engine,
      origin: resource.origin,
      line: resource.line,
      performanceTrace: resource.performanceTrace,
    );
    getIt<TabsBloc>().openExternalPlugin(MuseResourceFilePlugin(resolved));
  }
}

extension<T> on Iterable<T> {
  T? get firstOrNull {
    final iterator = this.iterator;
    return iterator.moveNext() ? iterator.current : null;
  }
}
