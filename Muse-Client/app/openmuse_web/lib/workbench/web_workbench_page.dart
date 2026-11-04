import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_file_viewer_flutter/openmuse_file_viewer_flutter.dart';
import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';

import '../office_routes.dart';
import 'dsh_pane.dart';
import 'web_layout_storage.dart';
import 'workspace_tree.dart';

/// Desktop's workbench widgets with browser-only resource and DSH adapters.
final class WebWorkbenchPage extends StatefulWidget {
  const WebWorkbenchPage({
    super.key,
    this.desktopRef,
    this.workspaceRef,
    this.mirrorPort,
    this.previewBuilder,
    this.dshPaneBuilder,
    this.officeDocument,
    this.onSettings,
    this.disconnectedWorkspace,
    this.workspaceStatus,
    this.onReconnect,
  });

  final String? desktopRef;
  final String? workspaceRef;
  final WorkspaceMirrorPort? mirrorPort;
  final Widget Function(BuildContext, WorkspaceMirrorNode)? previewBuilder;
  final WidgetBuilder? dshPaneBuilder;
  final OpenMuseWebOfficeDocument? officeDocument;
  final VoidCallback? onSettings;
  final Widget? disconnectedWorkspace;
  final Widget? workspaceStatus;
  final VoidCallback? onReconnect;

  @override
  State<WebWorkbenchPage> createState() => _WebWorkbenchPageState();
}

final class _WebWorkbenchPageState extends State<WebWorkbenchPage> {
  late WorkbenchLayoutController _layout;
  final WorkspaceMirrorController _mirror = WorkspaceMirrorController();
  Timer? _saveDebounce;
  WorkspaceMirrorNode? _selectedFile;
  final List<WorkspaceMirrorNode> _tabs = [];
  bool _actionBusy = false;
  bool _sidebarVisible = true;
  bool _projectExpanded = true;
  Size _lastSize = Size.zero;

  String get _scope =>
      '${widget.desktopRef ?? 'browser'}:${widget.workspaceRef ?? 'home'}';

  @override
  void initState() {
    super.initState();
    _layout = _loadLayout();
    _layout.addListener(_layoutChanged);
    _connectMirror();
  }

  @override
  void didUpdateWidget(covariant WebWorkbenchPage oldWidget) {
    super.didUpdateWidget(oldWidget);
    final oldScope =
        '${oldWidget.desktopRef ?? 'browser'}:${oldWidget.workspaceRef ?? 'home'}';
    if (oldScope != _scope) {
      _saveDebounce?.cancel();
      _layout.removeListener(_layoutChanged);
      _layout.dispose();
      _layout = _loadLayout()..addListener(_layoutChanged);
      _selectedFile = null;
      _tabs.clear();
    }
    if (oldWidget.mirrorPort != widget.mirrorPort ||
        oldWidget.workspaceRef != widget.workspaceRef ||
        oldWidget.desktopRef != widget.desktopRef) {
      _connectMirror();
    }
  }

  WorkbenchLayoutController _loadLayout() {
    final saved = readWebLayout(_scope);
    final layout = WorkbenchLayoutController(
      saved == null
          ? createDefaultWorkbenchLayout()
          : WorkbenchLayoutSnapshot.tryDecode(saved) ??
                createDefaultWorkbenchLayout(),
    );
    if (_removeUnusedCloudPane(layout)) {
      writeWebLayout(_scope, layout.snapshot.encode());
    }
    return layout;
  }

  bool _removeUnusedCloudPane(WorkbenchLayoutController layout) {
    var removed = false;
    for (final paneId in layout.snapshot.paneIds.toList()) {
      if (layout.snapshot.bindingFor(paneId)?.surfaceRef !=
          'plugin.panel:cloud.workspace') {
        continue;
      }
      layout.unbind(paneId);
      layout.closeEmptyPane(paneId);
      removed = true;
    }
    return removed;
  }

  void _connectMirror() {
    final port = widget.mirrorPort;
    final workspaceRef = widget.workspaceRef;
    if (port == null || workspaceRef == null) {
      _mirror.disconnect();
    } else {
      unawaited(_mirror.connect(workspaceRef, port));
    }
  }

  void _layoutChanged() {
    _saveDebounce?.cancel();
    _saveDebounce = Timer(const Duration(milliseconds: 350), () {
      writeWebLayout(_scope, _layout.snapshot.encode());
    });
  }

  @override
  void dispose() {
    _saveDebounce?.cancel();
    _layout.removeListener(_layoutChanged);
    _layout.dispose();
    _mirror.dispose();
    super.dispose();
  }

  Future<void> _paneAction(String paneId, OpenMusePaneAction action) async {
    switch (action) {
      case OpenMusePaneAction.splitRight:
        _layout.splitPane(paneId, axis: Axis.horizontal);
      case OpenMusePaneAction.splitDown:
        _layout.splitPane(paneId, axis: Axis.vertical);
      case OpenMusePaneAction.bindNewEditor:
        if (_layout.snapshot.bindingFor(paneId) != null) return;
        final id = 'editor.web.${DateTime.now().microsecondsSinceEpoch}';
        _layout.bind(
          paneId,
          SurfaceBinding(
            bindingId: 'binding.$id',
            surfaceRef: 'host.editorGroup:$id',
            instanceRef: 'surface.$id',
            mobility: SurfaceMobility.snapshotRestore,
          ),
        );
      case OpenMusePaneAction.swapLeft:
        _swapNeighbor(paneId, PaneDirection.left);
      case OpenMusePaneAction.swapRight:
        _swapNeighbor(paneId, PaneDirection.right);
      case OpenMusePaneAction.swapUp:
        _swapNeighbor(paneId, PaneDirection.up);
      case OpenMusePaneAction.swapDown:
        _swapNeighbor(paneId, PaneDirection.down);
      case OpenMusePaneAction.close:
        final binding = _layout.snapshot.bindingFor(paneId);
        if (binding != null) {
          final confirmed =
              await showDialog<bool>(
                context: context,
                builder: (context) => AlertDialog(
                  title: const Text('关闭窗格和内容？'),
                  content: const Text('该操作会销毁此内容的视图状态。可先使用方向交换将内容移动到相邻窗格。'),
                  actions: [
                    TextButton(
                      onPressed: () => Navigator.pop(context, false),
                      child: const Text('取消'),
                    ),
                    FilledButton(
                      onPressed: () => Navigator.pop(context, true),
                      child: const Text('关闭'),
                    ),
                  ],
                ),
              ) ??
              false;
          if (!confirmed || !mounted) return;
          _layout.unbind(paneId);
        }
        _layout.closeEmptyPane(paneId);
      case OpenMusePaneAction.reset:
        _layout.reset();
        _removeUnusedCloudPane(_layout);
    }
  }

  void _swapNeighbor(String paneId, PaneDirection direction) {
    final neighbor = const WorkbenchLayoutSolver()
        .solve(_layout.snapshot.root, _lastSize)
        .findNeighbor(paneId, direction);
    if (neighbor == null ||
        _layout.snapshot.bindingFor(paneId)?.surfaceRef ==
            'host.workspaceExplorer' ||
        _layout.snapshot.bindingFor(neighbor)?.surfaceRef ==
            'host.workspaceExplorer') {
      return;
    }
    _layout.swap(paneId, neighbor);
  }

  Widget _menu(String paneId, SurfaceBinding? binding) => OpenMusePaneMenu(
    paneId: paneId,
    hasBinding: binding != null,
    panelNames: const ['DSH Agent · dsh.agent'],
    onAction: (action) => unawaited(_paneAction(paneId, action)),
    onBindPanel: (index) => _bindPanel(paneId, index),
  );

  void _bindPanel(String paneId, int index) {
    if (index != 0) return;
    const panelId = 'dsh.agent';
    final surfaceRef = 'plugin.panel:$panelId';
    final source = _layout.snapshot.bindings.entries
        .where((entry) => entry.value.surfaceRef == surfaceRef)
        .firstOrNull;
    final target = _layout.snapshot.bindingFor(paneId);
    if (source != null) {
      if (source.key == paneId) return;
      if (target == null) {
        _layout.move(source.key, paneId);
      } else {
        _layout.swap(source.key, paneId);
      }
      return;
    }
    if (target != null) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(const SnackBar(content: Text('请先切分或选择一个空窗格，再绑定新的插件面板。')));
      return;
    }
    _layout.bind(
      paneId,
      SurfaceBinding(
        bindingId: 'binding.panel.web.$panelId',
        surfaceRef: surfaceRef,
        instanceRef: 'surface.panel.web.$panelId',
        mobility: SurfaceMobility.snapshotRestore,
      ),
    );
  }

  Widget _pane(BuildContext context, String paneId, SurfaceBinding? binding) {
    final ref = binding?.surfaceRef;
    if (ref == 'host.workspaceExplorer') {
      return OpenMuseWorkspaceSidebar(
        onSettings: widget.onSettings,
        onAddWorkspace: widget.mirrorPort is WorkspaceActionPort && !_actionBusy
            ? () => unawaited(_addWorkspace())
            : null,
        projectExpanded: _projectExpanded,
        onToggleProject: () =>
            setState(() => _projectExpanded = !_projectExpanded),
        onToggleSidebar: () =>
            setState(() => _sidebarVisible = !_sidebarVisible),
        tree: widget.workspaceStatus == null
            ? WorkspaceMirrorTree(
                controller: _mirror,
                disconnectedView: widget.disconnectedWorkspace,
                onRetry: widget.onReconnect ?? _connectMirror,
                onFileSelected: _openFile,
                onContextMenu: (node, position) =>
                    unawaited(_treeMenu(node, position)),
              )
            : Column(
                children: [
                  widget.workspaceStatus!,
                  Expanded(
                    child: WorkspaceMirrorTree(
                      controller: _mirror,
                      disconnectedView: widget.disconnectedWorkspace,
                      onRetry: widget.onReconnect ?? _connectMirror,
                      onFileSelected: _openFile,
                      onContextMenu: (node, position) =>
                          unawaited(_treeMenu(node, position)),
                    ),
                  ),
                ],
              ),
      );
    }
    if (ref?.startsWith('host.editorGroup:') == true) {
      final file = _selectedFile;
      return Column(
        children: [
          OpenMuseEditorTabStrip(
            tabs: [
              for (final tab in _tabs)
                OpenMuseEditorTabData(
                  id: tab.nodeRef,
                  title: tab.name,
                  extension: tab.name.split('.').last,
                ),
            ],
            activeId: file?.nodeRef,
            onActivate: (id) => setState(
              () =>
                  _selectedFile = _tabs.firstWhere((tab) => tab.nodeRef == id),
            ),
            onClose: _closeTab,
            onContextMenu: (_, position, id) =>
                unawaited(_tabMenu(position, id)),
            onExpandSidebar: _sidebarVisible
                ? null
                : () => setState(() => _sidebarVisible = true),
            paneMenu: _menu(paneId, binding),
          ),
          Expanded(child: _viewer(context)),
        ],
      );
    }
    if (ref == 'plugin.panel:dsh.agent') {
      return ColoredBox(
        color: Theme.of(context).brightness == Brightness.dark
            ? const Color(0xff202228)
            : const Color(0xfffbfbfc),
        child: widget.dshPaneBuilder?.call(context) ?? const OpenMuseDshPane(),
      );
    }
    return const SizedBox.expand();
  }

  void _openFile(WorkspaceMirrorNode node) => setState(() {
    if (!_tabs.any((tab) => tab.nodeRef == node.nodeRef)) _tabs.add(node);
    _selectedFile = node;
  });

  void _closeTab(String id) => setState(() {
    final index = _tabs.indexWhere((tab) => tab.nodeRef == id);
    if (index < 0) return;
    _tabs.removeAt(index);
    if (_selectedFile?.nodeRef == id) {
      _selectedFile = _tabs.isEmpty
          ? null
          : _tabs[index.clamp(0, _tabs.length - 1)];
    }
  });

  Future<String?> _menuAt(Offset position, Map<String, String> items) {
    final overlay =
        Overlay.of(context).context.findRenderObject()! as RenderBox;
    return showMenu<String>(
      context: context,
      position: RelativeRect.fromSize(
        Rect.fromLTWH(position.dx, position.dy, 0, 0),
        overlay.size,
      ),
      items: [
        for (final item in items.entries)
          PopupMenuItem(value: item.key, child: Text(item.value)),
      ],
    );
  }

  Future<void> _tabMenu(Offset position, String id) async {
    final command = await _menuAt(position, {
      'close': '关闭',
      'others': '关闭其他标签',
      'right': '关闭右侧标签',
      'all': '关闭所有标签',
      'copy': '复制文件名',
    });
    if (!mounted || command == null) return;
    final index = _tabs.indexWhere((tab) => tab.nodeRef == id);
    if (index < 0) return;
    if (command == 'copy') {
      await Clipboard.setData(ClipboardData(text: _tabs[index].name));
    } else {
      final closing = switch (command) {
        'others' => _tabs.where((tab) => tab.nodeRef != id).toList(),
        'right' => _tabs.skip(index + 1).toList(),
        'all' => _tabs.toList(),
        _ => [_tabs[index]],
      };
      for (final tab in closing) {
        _closeTab(tab.nodeRef);
      }
    }
  }

  Future<String?> _prompt(
    String title,
    String label, {
    String initial = '',
    String? hint,
  }) async {
    final input = TextEditingController(text: initial);
    final value = await showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(title),
        content: SizedBox(
          width: 420,
          child: TextField(
            controller: input,
            autofocus: true,
            decoration: InputDecoration(
              labelText: label,
              helperText: hint,
              helperMaxLines: 3,
            ),
            onSubmitted: (value) {
              if (value.trim().isNotEmpty) Navigator.pop(context, value.trim());
            },
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('取消'),
          ),
          FilledButton(
            onPressed: () {
              if (input.text.trim().isNotEmpty) {
                Navigator.pop(context, input.text.trim());
              }
            },
            child: const Text('确定'),
          ),
        ],
      ),
    );
    // The closing animation can still reference the controller in this frame.
    WidgetsBinding.instance.addPostFrameCallback((_) => input.dispose());
    return value;
  }

  Future<void> _addWorkspace() async {
    final name = await _prompt(
      '添加远程 Workspace',
      '工作区名称或远程绝对路径',
      hint: '输入名称会在 Linux 账号目录中新建工作区；输入绝对路径会挂载该设备上已有的目录。',
    );
    if (name != null && mounted) await _mutate('add-workspace', name: name);
  }

  Future<void> _mutate(
    String operation, {
    WorkspaceMirrorNode? node,
    String? name,
  }) async {
    final port = widget.mirrorPort;
    final workspaceRef = widget.workspaceRef;
    if (port is! WorkspaceActionPort || workspaceRef == null || _actionBusy) {
      return;
    }
    setState(() => _actionBusy = true);
    try {
      await (port as WorkspaceActionPort).performWorkspaceAction(
        workspaceRef: workspaceRef,
        operation: operation,
        nodeRef: node?.nodeRef,
        name: name,
      );
      if (!mounted) return;
      if (operation == 'rename' || operation == 'trash') {
        // Invalidate previews whose opaque reference may now point to an old path.
        for (final tab in _tabs.toList()) {
          _closeTab(tab.nodeRef);
        }
      }
      await _mirror.connect(workspaceRef, widget.mirrorPort!);
      for (final mount in _mirror.mounts) {
        await _mirror.expand(mount);
      }
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              operation == 'trash'
                  ? '已移至 .openmuse-trash，可右键恢复到工作区根目录。'
                  : 'Workspace 已更新',
            ),
          ),
        );
      }
    } on Object catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('$error')));
      }
    } finally {
      if (mounted) setState(() => _actionBusy = false);
    }
  }

  Future<void> _treeMenu(WorkspaceMirrorNode node, Offset position) async {
    final root = _mirror.mounts.contains(node);
    final writable = widget.mirrorPort is WorkspaceActionPort && !_actionBusy;
    final command = await _menuAt(position, {
      'open': node.isDirectory ? (node.expanded ? '收起' : '展开') : '打开',
      if (node.isDirectory) 'refresh': '刷新目录',
      'copy': '复制名称',
      if (writable && node.isDirectory) ...{
        'new-file': '新建文件…',
        'new-folder': '新建文件夹…',
        if (!root) 'mount': '添加为 Workspace',
      },
      if (writable && node.canRestore) 'restore': '恢复到工作区根目录',
      if (writable && !root) ...{'rename': '重命名…', 'trash': '移至回收站'},
      if (writable && root && _mirror.mounts.length > 1)
        'unmount': '移除 Workspace',
    });
    if (!mounted || command == null) return;
    if (command == 'open') {
      if (!node.isDirectory) {
        _openFile(node);
      } else if (node.expanded) {
        _mirror.collapse(node);
      } else {
        await _mirror.expand(node);
      }
    } else if (command == 'refresh') {
      await _mirror.refresh(node);
    } else if (command == 'copy') {
      await Clipboard.setData(ClipboardData(text: node.name));
    } else if (command == 'rename' ||
        command == 'new-file' ||
        command == 'new-folder') {
      final name = await _prompt(
        command == 'rename'
            ? '重命名'
            : command == 'new-file'
            ? '新建文件'
            : '新建文件夹',
        '名称',
        initial: command == 'rename' ? node.name : '',
      );
      if (name != null && mounted) {
        await _mutate(command, node: node, name: name);
      }
    } else if (command == 'trash') {
      final confirmed = await showDialog<bool>(
        context: context,
        builder: (context) => AlertDialog(
          title: const Text('移至回收站？'),
          content: Text('将「${node.name}」移至该工作区的 .openmuse-trash 目录。'),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context, false),
              child: const Text('取消'),
            ),
            FilledButton(
              onPressed: () => Navigator.pop(context, true),
              child: const Text('移至回收站'),
            ),
          ],
        ),
      );
      if (confirmed == true && mounted) await _mutate(command, node: node);
    } else {
      await _mutate(command, node: node);
    }
  }

  Widget _viewer(BuildContext context) {
    final document = widget.officeDocument;
    if (document != null) {
      return Center(
        child: FilledButton.icon(
          onPressed: () => Navigator.of(context).push<void>(
            MaterialPageRoute(builder: (_) => document.buildScreen()),
          ),
          icon: const Icon(Icons.description_outlined),
          label: Text(document.title),
        ),
      );
    }
    final file = _selectedFile;
    if (file == null) return const OpenMuseWorkbenchWelcome();
    final builder = widget.previewBuilder;
    if (builder != null) return builder(context, file);
    final port = widget.mirrorPort;
    final WorkspaceResourcePort? resourcePort = port is WorkspaceResourcePort
        ? port as WorkspaceResourcePort
        : null;
    final workspaceRef = widget.workspaceRef;
    final resourceRef = file.resourceRef;
    if (resourcePort != null && workspaceRef != null && resourceRef != null) {
      return OpenMuseFileViewerBody(
        key: ValueKey(resourceRef),
        resourceName: file.name,
        readBytes: () => resourcePort.readResource(
          workspaceRef: workspaceRef,
          resourceRef: resourceRef,
        ),
      );
    }
    if (resourceRef == null) {
      return const Center(child: Text('没有已安装的插件可以打开这个资源。'));
    }
    return const Center(child: Text('文件预览连接不可用。'));
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    body: WorkbenchCanvas(
      controller: _layout,
      hiddenPaneIds: _sidebarVisible ? const {} : const {'workspace'},
      onSizeChanged: (size) => _lastSize = size,
      paneBuilder: _pane,
      overlayMenuBuilder: (context, paneId, binding) {
        final ref = binding?.surfaceRef;
        return ref == 'host.workspaceExplorer' ||
                ref?.startsWith('host.editorGroup:') == true
            ? null
            : _menu(paneId, binding);
      },
    ),
  );
}
