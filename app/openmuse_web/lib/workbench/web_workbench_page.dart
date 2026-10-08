import 'dart:async';

import 'package:flutter/material.dart';
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
                onFileSelected: (node) => setState(() => _selectedFile = node),
              )
            : Column(
                children: [
                  widget.workspaceStatus!,
                  Expanded(
                    child: WorkspaceMirrorTree(
                      controller: _mirror,
                      disconnectedView: widget.disconnectedWorkspace,
                      onRetry: widget.onReconnect ?? _connectMirror,
                      onFileSelected: (node) =>
                          setState(() => _selectedFile = node),
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
            tabs: file == null
                ? const []
                : [
                    OpenMuseEditorTabData(
                      id: file.nodeRef,
                      title: file.name,
                      extension: file.name.split('.').last,
                    ),
                  ],
            activeId: file?.nodeRef,
            onActivate: (_) {},
            onClose: (_) => setState(() => _selectedFile = null),
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
