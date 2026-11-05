import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'dart:async';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_cli_plugin/openmuse_cli_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_market_ui/openmuse_market_ui.dart';

import 'design_system.dart';
import 'desktop_update.dart';
import 'layout/layout.dart';
import 'layout/plugin_panel.dart';
import 'layout/surface_mutation_guard.dart';
import 'local_settings.dart';
import 'plugin_surface_host.dart';
import 'settings_dialog.dart';
import 'window_chrome.dart';
import 'workspace_controller.dart';
import 'workspace_picker.dart';

final class OpenMuseWorkbench extends StatefulWidget {
  const OpenMuseWorkbench({
    super.key,
    required this.registry,
    required this.workspace,
    required this.settings,
    this.layoutController,
    this.layoutStore,
    this.mutationGuards,
    this.updates,
    this.desktopUpdate,
    this.onGetUpdate,
    this.resources,
  });

  final NativeResourceCenter? resources;
  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings settings;
  final WorkbenchLayoutController? layoutController;
  final LayoutSnapshotWriter? layoutStore;
  final SurfaceMutationGuards? mutationGuards;
  final OpenMuseUpdateController? updates;
  final OpenMuseDesktopUpdate? desktopUpdate;
  final Future<void> Function(OpenMuseRelease release)? onGetUpdate;

  @override
  State<OpenMuseWorkbench> createState() => _OpenMuseWorkbenchState();
}

final class _OpenMuseWorkbenchState extends State<OpenMuseWorkbench> {
  static const _menuChannel = MethodChannel('com.openmuse.host/menu');

  bool get _macNativeChrome =>
      Platform.isMacOS &&
      !WidgetsBinding.instance.runtimeType.toString().contains('Test');
  static const _gutter = 1.0;
  late final WorkbenchLayoutController _layout =
      widget.layoutController ??
      WorkbenchLayoutController(createDefaultWorkbenchLayout());
  late final bool _ownsLayout = widget.layoutController == null;
  Timer? _saveDebounce;
  Size _lastSize = Size.zero;
  final OpenMuseCliController _terminalController = OpenMuseCliController();
  bool _terminalVisible = false;
  _HostView _view =
      kDebugMode &&
          Platform.environment.containsKey('OPENMUSE_E2E_SUPPORT_DIR') &&
          Platform.environment.containsKey('OPENMUSE_E2E_OPEN_DOCX')
      ? _HostView.ide
      : _HostView.agent;
  bool _hasOpenedTerminal = false;
  bool _marketPageOpen = false;

  bool get _terminalAvailable =>
      widget.settings.pluginValues('com.openmuse.cli')['enabled'] != false &&
      widget.registry.panelProvider(
            OpenMuseSurfaceRegion.bottomPanel,
            panelId: 'cli.console',
          ) !=
          null;

  @override
  void initState() {
    super.initState();
    if (_layout.snapshot == createDefaultWorkbenchLayout()) {
      _layout.restore(_layoutForView(_view));
    }
    _ensureEditorGroups();
    widget.resources?.controller.presentEditor = _presentResourceEditor;
    widget.workspace.interceptResourceOpen = _openResourceDocument;
    _layout.addListener(_layoutChanged);
    _removeLegacyMarketPane();
    if (_macNativeChrome) {
      _menuChannel.setMethodCallHandler(_onMacMenu);
      _publishMacView(_view);
    }
  }

  @override
  void didUpdateWidget(OpenMuseWorkbench oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.resources != widget.resources) {
      oldWidget.resources?.controller.presentEditor = null;
      widget.resources?.controller.presentEditor = _presentResourceEditor;
    }
  }

  @override
  void dispose() {
    widget.resources?.controller.presentEditor = null;
    widget.workspace.interceptResourceOpen = null;
    if (_macNativeChrome) _menuChannel.setMethodCallHandler(null);
    final shouldFlush = _saveDebounce?.isActive ?? false;
    _saveDebounce?.cancel();
    if (shouldFlush) _persistLayout();
    _layout.removeListener(_layoutChanged);
    if (_ownsLayout) _layout.dispose();
    _terminalController.dispose();
    super.dispose();
  }

  bool _openResourceDocument(OpenMuseResource resource) {
    final center = widget.resources;
    final name = resource.displayName.toLowerCase();
    if (center == null ||
        !resource.uri.isScheme('file') ||
        !(name.endsWith('.sydoc') ||
            name.endsWith('.sy.zip') ||
            name.endsWith('.sy'))) {
      return false;
    }
    unawaited(() async {
      try {
        final reference = await center.referenceForMountedFile(
          File.fromUri(resource.uri),
        );
        if (reference == null) {
          throw const ResourceFailure('SOURCE_UNAVAILABLE');
        }
        if (mounted) {
          await openResourcePage(context, center.controller, reference);
        }
      } catch (error) {
        if (mounted) showResourceError(context, error);
      }
    }());
    return true;
  }

  Future<void> _presentResourceEditor(
    ResourceEditorPresentation request,
  ) async {
    if (!mounted) return;
    Navigator.of(
      context,
    ).popUntil((route) => route.settings.name != openMuseResourceRouteName);
    final session = request.session;
    final reference = ResourceReference(
      identity: session.record.identity,
      version: session.opened.fromSnapshot ? session.record.version : null,
    );
    await widget.workspace.openResourceSession(
      OpenMuseResource(
        uri: reference.toUri(),
        displayName: session.record.name,
        mediaType: session.record.mediaType,
        reference: session.opened.reference,
      ),
      WorkspaceResourceSession(
        session: session,
        confirmClose: () async =>
            await showDialog<bool>(
              context: context,
              builder: (dialog) {
                final s = ResourceCenterStrings(dialog);
                return AlertDialog(
                  title: Text(s.unsaved),
                  actions: [
                    TextButton(
                      key: const ValueKey('resource-keep-editing'),
                      onPressed: () => Navigator.pop(dialog, false),
                      child: Text(s.keepEditing),
                    ),
                    TextButton(
                      key: const ValueKey('resource-discard'),
                      onPressed: () => Navigator.pop(dialog, true),
                      child: Text(s.discard),
                    ),
                  ],
                );
              },
            ) ??
            false,
        builder: (context) => ListenableBuilder(
          listenable: widget.registry,
          builder: (context, _) {
            if (widget.registry.stateOf(request.choice.plugin.descriptor.id) !=
                OpenMusePluginState.active) {
              return Center(
                child: Text(ResourceCenterStrings(context).noHandler),
              );
            }
            return request.choice.contributor.buildResourceEditor(
              context,
              request.choice.spec.id,
              session,
            );
          },
        ),
      ),
    );
  }

  void _openTerminal([String? directory]) {
    if (!_terminalAvailable) return;
    unawaited(() async {
      try {
        final plugin = widget.registry.panelProvider(
          OpenMuseSurfaceRegion.bottomPanel,
          panelId: 'cli.console',
        );
        if (plugin == null) return;
        await widget.registry.activate(plugin.descriptor.id);
        if (!mounted) return;
        setState(() => _terminalVisible = true);
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted) {
            _terminalController.open(
              directory ?? widget.workspace.activeMountPath,
            );
            _hasOpenedTerminal = true;
          }
        });
      } catch (error) {
        if (mounted) {
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text('无法启动终端：$error')));
        }
      }
    }());
  }

  Future<void> _openPluginMarket(BuildContext context) async {
    if (RuntimeConfigScope.maybeOf(
          context,
        )?.isEnabled(AppFeature.pluginMarket) ==
        false) {
      return;
    }
    if (_marketPageOpen) return;
    _marketPageOpen = true;
    try {
      const pluginId = 'com.openmuse.plugin-market';
      if (widget.registry.plugin(pluginId) == null) {
        throw StateError('插件市场插件未安装或已禁用');
      }
      await widget.registry.activate(pluginId);
      if (!mounted || !context.mounted) return;
      final candidate = await Navigator.of(context)
          .push<OpenMusePanelCandidate>(
            MaterialPageRoute(
              builder: (_) => _PluginMarketPage(
                registry: widget.registry,
                workspace: widget.workspace,
              ),
            ),
          );
      if (candidate != null) {
        await widget.registry.ensureActive(candidate.plugin);
        if (!mounted) return;
        revealPluginPanel(_layout, candidate);
      }
    } catch (error) {
      if (mounted && context.mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('无法打开插件市场：$error')));
      }
    } finally {
      _marketPageOpen = false;
    }
  }

  void _removeLegacyMarketPane() {
    for (final entry in _layout.snapshot.bindings.entries.toList()) {
      if (entry.value.surfaceRef != 'plugin.panel:plugin.market') continue;
      _layout.unbind(entry.key);
      _layout.closeEmptyPane(entry.key);
    }
  }

  void _toggleTerminal() {
    if (!_terminalAvailable) return;
    if (_terminalVisible) {
      setState(() => _terminalVisible = false);
    } else if (_hasOpenedTerminal) {
      setState(() => _terminalVisible = true);
    } else {
      _openTerminal();
    }
  }

  void _layoutChanged() {
    _ensureEditorGroups();
    final store = widget.layoutStore;
    if (store == null) return;
    _saveDebounce?.cancel();
    _saveDebounce = Timer(const Duration(milliseconds: 350), _persistLayout);
  }

  void _persistLayout() {
    final store = widget.layoutStore;
    if (store == null) return;
    unawaited(
      store.save(_layout.snapshot).catchError((Object error) {
        debugPrint('Unable to persist workbench layout: $error');
      }),
    );
  }

  void _ensureEditorGroups() {
    final existing = widget.workspace.editorGroups
        .map((group) => group.id)
        .toSet();
    for (final binding in _layout.snapshot.bindings.values) {
      const prefix = 'host.editorGroup:';
      if (!binding.surfaceRef.startsWith(prefix)) continue;
      final groupId = binding.surfaceRef.substring(prefix.length);
      if (existing.add(groupId)) {
        widget.workspace.createEditorGroup(groupId: groupId);
      }
    }
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: Listenable.merge([
      widget.workspace,
      widget.settings,
      widget.registry,
      _layout,
      ?widget.updates,
      ?widget.desktopUpdate?.status,
      ?widget.desktopUpdate?.progress,
    ]),
    builder: (context, _) => Scaffold(
      body: Column(
        children: [
          if (_macNativeChrome)
            _MacTitleBar(
              workspace: widget.workspace,
              onDrag: OpenMuseWindowChrome.startDrag,
              onSettings: () => _openLocalSettings(context),
              onShowAgent: _showAgentPanel,
            )
          else
            _WindowsTitleStrip(
              registry: widget.registry,
              workspace: widget.workspace,
              settings: widget.settings,
              view: _view,
              onView: _selectView,
              onViewHelp: _showViewHelp,
              onTerminal: _terminalAvailable ? _toggleTerminal : null,
            ),
          if (widget.resources != null)
            Align(
              alignment: Alignment.centerRight,
              child: TextButton.icon(
                key: const ValueKey('desktop-resource-center'),
                icon: const Icon(Icons.home_outlined, size: 18),
                label: Text(stringsOf(context).resourcecenterHome()),
                onPressed: () async {
                  final center = widget.resources!;
                  await Navigator.of(context).push(
                    MaterialPageRoute<void>(
                      settings: const RouteSettings(
                        name: openMuseResourceRouteName,
                      ),
                      builder: (_) => Scaffold(
                        appBar: AppBar(
                          title: Text(stringsOf(context).resourcecenterHome()),
                        ),
                        body: ResourceCenterView(controller: center.controller),
                      ),
                    ),
                  );
                },
              ),
            ),
          Expanded(
            child: WorkbenchCanvas(
              controller: _layout,
              hiddenPaneIds: _hiddenPaneIds(_layout.snapshot),
              onSizeChanged: (size) => _lastSize = size,
              onPaneFocus: _focusPane,
              paneBuilder: (context, paneId, binding) => binding == null
                  ? const SizedBox.expand()
                  : _BoundSurface(
                      binding: binding,
                      registry: widget.registry,
                      workspace: widget.workspace,
                      settings: widget.settings,
                      terminalController: _terminalController,
                      terminalVisible: _terminalVisible,
                      onCloseTerminal: () =>
                          setState(() => _terminalVisible = false),
                      onOpenTerminal: _openTerminal,
                      onOpenMarket: () => unawaited(_openPluginMarket(context)),
                      updates:
                          RuntimeConfigScope.maybeOf(
                                context,
                              )?.isEnabled(AppFeature.updatePrompts) ==
                              false
                          ? null
                          : widget.updates,
                      desktopUpdate: widget.desktopUpdate,
                      onStartUpdate: () {
                        final release = widget.updates?.available;
                        if (release != null) {
                          unawaited(widget.onGetUpdate?.call(release));
                        }
                      },
                      onShowUpdate: () {
                        final release = widget.updates?.available;
                        if (release == null) return;
                        unawaited(
                          showOpenMuseUpdateDialog(
                            context,
                            release,
                            onUpdate: widget.onGetUpdate ?? (_) async {},
                          ),
                        );
                      },
                      paneMenu: _embeddedPaneMenu(
                        context,
                        _layout.snapshot,
                        paneId,
                      ),
                      onFocus: () => _focusPane(paneId, binding),
                    ),
              overlayMenuBuilder: (context, paneId, binding) =>
                  _showsOverlayPaneMenu(_layout.snapshot, paneId)
                  ? _paneMenuButton(context, _layout.snapshot, paneId)
                  : null,
            ),
          ),
        ],
      ),
    ),
  );

  void _focusPane(String paneId, SurfaceBinding binding) {
    _layout.focus(paneId);
    const prefix = 'host.editorGroup:';
    if (binding.surfaceRef.startsWith(prefix)) {
      widget.workspace.focusEditorGroup(
        binding.surfaceRef.substring(prefix.length),
      );
    }
  }

  bool _isPrimaryWorkspacePane(
    WorkbenchLayoutSnapshot snapshot,
    String paneId,
  ) => snapshot.bindingFor(paneId)?.surfaceRef == 'host.workspaceExplorer';

  bool _isEditorPane(WorkbenchLayoutSnapshot snapshot, String paneId) =>
      snapshot.bindingFor(paneId)?.surfaceRef.startsWith('host.editorGroup:') ??
      false;

  bool _isDshPane(WorkbenchLayoutSnapshot snapshot, String paneId) =>
      snapshot.bindingFor(paneId)?.surfaceRef == 'plugin.panel:dsh.agent';

  bool _showsOverlayPaneMenu(WorkbenchLayoutSnapshot snapshot, String paneId) {
    if (_isPrimaryWorkspacePane(snapshot, paneId) ||
        _isEditorPane(snapshot, paneId)) {
      return false;
    }
    if (_isDshPane(snapshot, paneId) &&
        widget.registry.panelProviderById('dsh.agent') != null) {
      return false;
    }
    return true;
  }

  Widget _paneMenuButton(
    BuildContext context,
    WorkbenchLayoutSnapshot snapshot,
    String paneId,
  ) => _PaneMenuButton(
    paneId: paneId,
    hasBinding: snapshot.bindingFor(paneId) != null,
    panelCandidates: widget.registry
        .panelCandidates()
        .where(
          (candidate) => candidate.panel.region != OpenMuseSurfaceRegion.dialog,
        )
        .toList(),
    onAction: (action) => _handlePaneAction(context, paneId, action),
  );

  Widget? _embeddedPaneMenu(
    BuildContext context,
    WorkbenchLayoutSnapshot snapshot,
    String paneId,
  ) => (_isEditorPane(snapshot, paneId) || _isDshPane(snapshot, paneId))
      ? _paneMenuButton(context, snapshot, paneId)
      : null;

  Future<void> _handlePaneAction(
    BuildContext context,
    String paneId,
    Object action,
  ) async {
    if (_isPrimaryWorkspacePane(_layout.snapshot, paneId)) return;
    if (action case _BindPanelAction(:final candidate)) {
      await _bindPluginPanel(context, paneId, candidate);
      return;
    }
    if (action is! _PaneAction) {
      throw ArgumentError.value(action, 'action', 'Unknown pane action.');
    }
    switch (action) {
      case _PaneAction.splitRight:
        _layout.splitPane(paneId, axis: Axis.horizontal);
      case _PaneAction.splitDown:
        _layout.splitPane(paneId, axis: Axis.vertical);
      case _PaneAction.bindNewEditor:
        final group = widget.workspace.createEditorGroup();
        _layout.bind(
          paneId,
          SurfaceBinding(
            bindingId: 'binding.${group.id}',
            surfaceRef: 'host.editorGroup:${group.id}',
            instanceRef: 'surface.${group.id}',
            mobility: SurfaceMobility.snapshotRestore,
          ),
        );
      case _PaneAction.swapLeft:
        await _swapWithNeighbor(context, paneId, PaneDirection.left);
      case _PaneAction.swapRight:
        await _swapWithNeighbor(context, paneId, PaneDirection.right);
      case _PaneAction.swapUp:
        await _swapWithNeighbor(context, paneId, PaneDirection.up);
      case _PaneAction.swapDown:
        await _swapWithNeighbor(context, paneId, PaneDirection.down);
      case _PaneAction.close:
        await _closePane(context, paneId);
      case _PaneAction.reset:
        _layout.restore(_layoutForView(_view));
        _removeUnboundEditorGroups();
    }
  }

  Future<void> _bindPluginPanel(
    BuildContext context,
    String paneId,
    OpenMusePanelCandidate candidate,
  ) async {
    final surfaceRef = 'plugin.panel:${candidate.panel.id}';
    final source = _layout.snapshot.bindings.entries
        .where((entry) => entry.value.surfaceRef == surfaceRef)
        .firstOrNull;
    if (source != null) {
      await _moveSingletonTo(context, paneId, surfaceRef);
      return;
    }
    if (_layout.snapshot.bindingFor(paneId) != null) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(const SnackBar(content: Text('请先切分或选择一个空窗格，再绑定新的插件面板。')));
      return;
    }
    final identity = '${candidate.plugin.descriptor.id}.${candidate.panel.id}';
    _layout.bind(
      paneId,
      SurfaceBinding(
        bindingId: 'binding.panel.$identity',
        surfaceRef: surfaceRef,
        instanceRef: 'surface.panel.$identity',
        mobility: SurfaceMobility.snapshotRestore,
      ),
    );
  }

  Future<void> _moveSingletonTo(
    BuildContext context,
    String paneId,
    String surfaceRef,
  ) async {
    final source = _layout.snapshot.bindings.entries
        .where((entry) => entry.value.surfaceRef == surfaceRef)
        .firstOrNull;
    if (source == null || source.key == paneId) return;
    final target = _layout.snapshot.bindingFor(paneId);
    if (!await _prepareMutation(context, [
      source.value.instanceRef,
      if (target != null) target.instanceRef,
    ], target == null ? SurfaceMutationKind.move : SurfaceMutationKind.swap)) {
      return;
    }
    if (_layout.snapshot.bindingFor(paneId) == null) {
      _layout.move(source.key, paneId);
    } else {
      _layout.swap(source.key, paneId);
    }
  }

  Future<void> _swapWithNeighbor(
    BuildContext context,
    String paneId,
    PaneDirection direction,
  ) async {
    final visibleRoot = _withoutPanes(
      _layout.snapshot.root,
      _hiddenPaneIds(_layout.snapshot),
    );
    final neighbor = visibleRoot == null
        ? null
        : const WorkbenchLayoutSolver()
              .solve(visibleRoot, _lastSize, gutter: _gutter)
              .findNeighbor(paneId, direction);
    if (neighbor == null) return;
    if (_isPrimaryWorkspacePane(_layout.snapshot, paneId) ||
        _isPrimaryWorkspacePane(_layout.snapshot, neighbor)) {
      return;
    }
    final refs = [
      _layout.snapshot.bindingFor(paneId)?.instanceRef,
      _layout.snapshot.bindingFor(neighbor)?.instanceRef,
    ].whereType<String>();
    if (await _prepareMutation(context, refs, SurfaceMutationKind.swap)) {
      _layout.swap(paneId, neighbor);
    }
  }

  Set<String> _hiddenPaneIds(WorkbenchLayoutSnapshot snapshot) {
    final hidden = <String>{
      for (final entry in snapshot.bindings.entries)
        if ((!widget.workspace.sidebarVisible &&
                entry.value.surfaceRef == 'host.workspaceExplorer') ||
            (!widget.settings.assistantVisible &&
                entry.value.surfaceRef == 'plugin.panel:dsh.agent') ||
            entry.value.surfaceRef == 'plugin.panel:cloud.workspace')
          entry.key,
    };
    if (_view == _HostView.agent &&
        !_editorHasDocument() &&
        !_terminalVisible &&
        !_hasOpenedTerminal) {
      for (final entry in snapshot.bindings.entries) {
        if (entry.value.surfaceRef.startsWith('host.editorGroup:')) {
          hidden.add(entry.key);
        }
      }
    }
    return hidden;
  }

  bool _editorHasDocument() {
    for (final group in widget.workspace.editorGroups) {
      if (widget.workspace.tabsFor(group.id).isNotEmpty) return true;
    }
    return false;
  }

  void _openLocalSettings(BuildContext context) {
    showOpenMuseSettings(
      context,
      widget.settings,
      widget.registry,
      onPluginWorkspace:
          ({required pluginId, required path, required enabled}) =>
              choosePluginWorkspace(
                workspace: widget.workspace,
                pluginId: pluginId,
                path: path,
                enabled: enabled,
              ),
    );
  }

  Future<void> _onMacMenu(MethodCall call) async {
    if (!mounted) return;
    switch (call.method) {
      case 'settings':
        _openLocalSettings(context);
      case 'terminal':
        _toggleTerminal();
      case 'view':
        switch (call.arguments) {
          case 'agent':
            _selectView(_HostView.agent);
          case 'ide':
            _selectView(_HostView.ide);
          case 'help':
            await _showViewHelp();
        }
    }
  }

  Future<void> _showAgentPanel() async {
    final candidate = widget.registry
        .panelCandidates()
        .where((item) => item.plugin is OpenMuseAgentContributor)
        .firstOrNull;
    if (candidate == null) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(const SnackBar(content: Text('请在插件中启用 Agent 面板。')));
      return;
    }
    await widget.registry.ensureActive(candidate.plugin);
    if (!mounted) return;
    await widget.settings.setAssistantVisible(true);
    if (!mounted) return;
    revealPluginPanel(_layout, candidate);
  }

  void _selectView(_HostView view) {
    setState(() => _view = view);
    unawaited(widget.settings.setAssistantVisible(true));
    _layout.restore(_layoutForView(view));
    _publishMacView(view);
  }

  void _publishMacView(_HostView view) {
    if (!_macNativeChrome) return;
    unawaited(() async {
      try {
        await _menuChannel.invokeMethod<void>('setCheckedView', view.name);
      } on MissingPluginException {
        // The menu channel exists only in the macOS runner.
      }
    }());
  }

  Future<void> _showViewHelp() => showDialog<void>(
    context: context,
    builder: (context) => AlertDialog(
      title: Text(_hostText(context, '视图帮助', 'View help')),
      content: SizedBox(
        width: 420,
        child: Text(
          _hostText(
            context,
            'Agent 视图：左侧是工作区，中间是 Agent，右侧是编辑和预览。没有打开文件时，右侧窗口不会出现。\n\n'
                'IDE 视图：中间固定为编辑和预览，Agent 在右侧。\n\n'
                '调整窗口：拖动窗格之间的分界线可以改变宽度。点击窗格右上角的菜单可以向右切分、向下切分，或与左侧、右侧、上方、下方的窗格交换位置。也可以关闭窗格，或重置为当前视图的默认布局。',
            'Agent view puts the workspace on the left, the Agent in the middle, and the editor on the right. The editor stays hidden until a file is open.\n\n'
                'IDE view keeps the editor in the middle and the Agent on the right.\n\n'
                'Drag a divider to resize. The pane menu can split right or down, swap with a neighbor, close a pane, or reset the current view.',
          ),
          style: const TextStyle(height: 1.45),
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: Text(_hostText(context, '知道了', 'OK')),
        ),
      ],
    ),
  );

  void _removeUnboundEditorGroups() {
    const prefix = 'host.editorGroup:';
    final bound = _layout.snapshot.bindings.values
        .where((binding) => binding.surfaceRef.startsWith(prefix))
        .map((binding) => binding.surfaceRef.substring(prefix.length))
        .toSet();
    for (final group in widget.workspace.editorGroups.toList()) {
      if (group.id != LocalWorkspaceController.primaryEditorGroupId &&
          !bound.contains(group.id)) {
        widget.workspace.removeEditorGroup(group.id);
      }
    }
  }

  Future<void> _closePane(BuildContext context, String paneId) async {
    final binding = _layout.snapshot.bindingFor(paneId);
    if (binding != null) {
      if (!await _prepareMutation(context, [
        binding.instanceRef,
      ], SurfaceMutationKind.close)) {
        return;
      }
      if (!context.mounted) return;
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
      const prefix = 'host.editorGroup:';
      if (binding.surfaceRef.startsWith(prefix)) {
        widget.workspace.removeEditorGroup(
          binding.surfaceRef.substring(prefix.length),
        );
      }
    }
    _layout.closeEmptyPane(paneId);
  }

  Future<bool> _prepareMutation(
    BuildContext context,
    Iterable<String> instanceRefs,
    SurfaceMutationKind kind,
  ) async {
    final guards = widget.mutationGuards;
    if (guards == null) return true;
    final result = await guards.prepare(instanceRefs, kind);
    if (result.decision == SurfaceMutationDecision.allow) return true;
    if (!context.mounted) return false;
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          result.reason ??
              (result.decision == SurfaceMutationDecision.defer
                  ? '原生输入正在进行，窗格操作已延迟。'
                  : '当前内容不允许执行此窗格操作。'),
        ),
      ),
    );
    return false;
  }
}

LayoutNode? _withoutPanes(LayoutNode node, Set<String> hiddenPaneIds) {
  switch (node) {
    case PaneNode(:final paneId):
      return hiddenPaneIds.contains(paneId) ? null : node;
    case SplitNode(:final first, :final second):
      final visibleFirst = _withoutPanes(first, hiddenPaneIds);
      final visibleSecond = _withoutPanes(second, hiddenPaneIds);
      if (visibleFirst == null) return visibleSecond;
      if (visibleSecond == null) return visibleFirst;
      return node.copyWith(first: visibleFirst, second: visibleSecond);
  }
}

final class _BoundSurface extends StatelessWidget {
  const _BoundSurface({
    required this.binding,
    required this.registry,
    required this.workspace,
    required this.settings,
    required this.terminalController,
    required this.terminalVisible,
    required this.onCloseTerminal,
    required this.onOpenTerminal,
    required this.onOpenMarket,
    this.updates,
    this.desktopUpdate,
    this.onStartUpdate,
    this.onShowUpdate,
    required this.onFocus,
    this.paneMenu,
  });

  final SurfaceBinding binding;
  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings settings;
  final OpenMuseCliController terminalController;
  final bool terminalVisible;
  final VoidCallback onCloseTerminal;
  final ValueChanged<String> onOpenTerminal;
  final VoidCallback onOpenMarket;
  final OpenMuseUpdateController? updates;
  final OpenMuseDesktopUpdate? desktopUpdate;
  final VoidCallback? onStartUpdate;
  final VoidCallback? onShowUpdate;
  final VoidCallback onFocus;
  final Widget? paneMenu;

  @override
  Widget build(BuildContext context) => Listener(
    behavior: HitTestBehavior.translucent,
    onPointerDown: (_) => onFocus(),
    child: _surface(context),
  );

  Widget _surface(BuildContext context) {
    if (binding.surfaceRef == 'host.workspaceExplorer') {
      return _WorkspaceSidebar(
        registry: registry,
        workspace: workspace,
        settings: settings,
        onOpenTerminal: onOpenTerminal,
        onOpenMarket: onOpenMarket,
        updates: updates,
        desktopUpdate: desktopUpdate,
        onStartUpdate: onStartUpdate,
        onShowUpdate: onShowUpdate,
      );
    }
    const editorPrefix = 'host.editorGroup:';
    if (binding.surfaceRef.startsWith(editorPrefix)) {
      final editor = _EditorArea(
        registry: registry,
        workspace: workspace,
        settings: settings,
        groupId: binding.surfaceRef.substring(editorPrefix.length),
        paneMenu: paneMenu,
      );
      if (binding.surfaceRef !=
              'host.editorGroup:${LocalWorkspaceController.primaryEditorGroupId}' ||
          settings.pluginValues('com.openmuse.cli')['enabled'] == false) {
        return editor;
      }
      final plugin = registry.panelProvider(
        OpenMuseSurfaceRegion.bottomPanel,
        panelId: 'cli.console',
      );
      if (plugin == null) return editor;
      return _EditorCliDock(
        editor: editor,
        visible: terminalVisible,
        onMinimize: onCloseTerminal,
        console: OpenMuseCliConsole(
          key: const ValueKey('cli-bottom-panel'),
          workingDirectory: workspace.activeMountPath,
          environmentProvider: plugin is OpenMuseCliPlugin
              ? plugin.sessionEnvironment
              : null,
          environmentDisposer: plugin is OpenMuseCliPlugin
              ? plugin.releaseEnvironment
              : null,
          controller: terminalController,
          onMinimize: onCloseTerminal,
        ),
      );
    }
    const panelPrefix = 'plugin.panel:';
    if (binding.surfaceRef.startsWith(panelPrefix)) {
      final panelId = binding.surfaceRef.substring(panelPrefix.length);
      final plugin = registry.panelProviderById(panelId);
      if (plugin != null) {
        final panel = PluginPanelHost(
          registry: registry,
          plugin: plugin,
          panelId: panelId,
        );
        if (panelId == 'dsh.agent' && paneMenu != null) {
          return OpenMuseSurfaceChrome(trailing: paneMenu, child: panel);
        }
        return panel;
      }
      return Center(child: Text('插件面板不可用：$panelId'));
    }
    return Center(child: Text('未知 Surface：${binding.surfaceRef}'));
  }
}

final class _EditorCliDock extends StatefulWidget {
  const _EditorCliDock({
    required this.editor,
    required this.console,
    required this.visible,
    required this.onMinimize,
  });

  final Widget editor;
  final Widget console;
  final bool visible;
  final VoidCallback onMinimize;

  @override
  State<_EditorCliDock> createState() => _EditorCliDockState();
}

final class _EditorCliDockState extends State<_EditorCliDock> {
  double _height = 230;

  @override
  Widget build(BuildContext context) => LayoutBuilder(
    builder: (context, constraints) {
      final available = constraints.maxHeight;
      if (available < 220) return widget.editor;
      final height = _height.clamp(120.0, available * 0.65);
      return Column(
        children: [
          Expanded(child: widget.editor),
          if (widget.visible)
            MouseRegion(
              cursor: SystemMouseCursors.resizeRow,
              child: GestureDetector(
                key: const ValueKey('cli-bottom-resizer'),
                behavior: HitTestBehavior.opaque,
                onVerticalDragUpdate: (details) {
                  setState(
                    () => _height = (_height - details.delta.dy).clamp(
                      120.0,
                      available * 0.65,
                    ),
                  );
                },
                child: SizedBox(
                  height: 14,
                  child: Stack(
                    alignment: Alignment.center,
                    children: [
                      Positioned.fill(
                        child: Align(
                          alignment: Alignment.center,
                          child: SizedBox(
                            height: 1,
                            width: double.infinity,
                            child: ColoredBox(
                              color: Theme.of(context).dividerColor,
                            ),
                          ),
                        ),
                      ),
                      Positioned(
                        right: 2,
                        child: InkWell(
                          key: const ValueKey('cli-bottom-collapse'),
                          onTap: widget.onMinimize,
                          child: const Icon(
                            Icons.keyboard_arrow_down,
                            size: 12,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ExcludeFocus(
            excluding: !widget.visible,
            child: Offstage(
              offstage: !widget.visible,
              child: SizedBox(height: height, child: widget.console),
            ),
          ),
        ],
      );
    },
  );
}

/// Tracks pointer movement globally so dragging remains reliable over the
/// embedded DSH WebView, whose native surface can swallow local mouse-up.
enum _PaneAction {
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

final class _BindPanelAction {
  const _BindPanelAction(this.candidate);
  final OpenMusePanelCandidate candidate;
}

final class _PaneMenuButton extends StatelessWidget {
  const _PaneMenuButton({
    required this.paneId,
    required this.hasBinding,
    required this.panelCandidates,
    required this.onAction,
  });

  final String paneId;
  final bool hasBinding;
  final List<OpenMusePanelCandidate> panelCandidates;
  final ValueChanged<Object> onAction;

  @override
  Widget build(BuildContext context) => OpenMusePaneMenu(
    paneId: paneId,
    hasBinding: hasBinding,
    panelNames: [
      for (final candidate in panelCandidates)
        '${candidate.plugin.descriptor.name} · ${candidate.panel.id}',
    ],
    onAction: (action) => onAction(_PaneAction.values.byName(action.name)),
    onBindPanel: (index) => onAction(_BindPanelAction(panelCandidates[index])),
  );
}

final class _WorkspaceSidebar extends StatelessWidget {
  const _WorkspaceSidebar({
    required this.registry,
    required this.workspace,
    required this.settings,
    required this.onOpenTerminal,
    required this.onOpenMarket,
    this.updates,
    this.desktopUpdate,
    this.onStartUpdate,
    this.onShowUpdate,
  });

  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings settings;
  final ValueChanged<String> onOpenTerminal;
  final VoidCallback onOpenMarket;
  final OpenMuseUpdateController? updates;
  final OpenMuseDesktopUpdate? desktopUpdate;
  final VoidCallback? onStartUpdate;
  final VoidCallback? onShowUpdate;

  @override
  Widget build(BuildContext context) => OpenMuseWorkspaceSidebar(
    showBrand: false,
    projectExpanded: workspace.projectSectionExpanded,
    onToggleProject: workspace.toggleProjectSection,
    onSettings: () => showOpenMuseSettings(
      context,
      settings,
      registry,
      onPluginWorkspace:
          ({required pluginId, required path, required enabled}) =>
              choosePluginWorkspace(
                workspace: workspace,
                pluginId: pluginId,
                path: path,
                enabled: enabled,
              ),
    ),
    onToggleSidebar: workspace.toggleSidebar,
    onSearch: () => _showSearch(context, workspace),
    onAddWorkspace: () => _addWorkspace(context, workspace),
    onPlugins: onOpenMarket,
    onNotifications: () => unawaited(_showNotifications(context)),
    hasUnreadNotifications: _hasUnreadUpdate,
    footerNotice: updates?.available == null || _isNoticeHidden
        ? null
        : _updateNotice(context),
    tree: ListView(
      children: [
        for (final mount in workspace.mounts) ...[
          _MountRow(
            mount: mount,
            registry: registry,
            workspace: workspace,
            settings: settings,
            onOpenTerminal: onOpenTerminal,
          ),
          for (final entry in workspace.visibleEntries(mount))
            _ResourceRow(
              entry: entry,
              registry: registry,
              workspace: workspace,
              settings: settings,
              onOpenTerminal: onOpenTerminal,
              selected:
                  workspace.selected?.uri.isScheme('file') == true &&
                  entry.path == workspace.selected!.uri.toFilePath(),
            ),
        ],
      ],
    ),
  );

  static const _notificationSettingsId = 'com.openmuse.notifications';

  String? get _releaseKey {
    final release = updates?.available;
    return release == null
        ? null
        : '${release.version}+${release.build}:${release.sha256}';
  }

  Map<String, Object?> get _notificationPrefs =>
      settings.pluginValues(_notificationSettingsId);

  bool get _hasUnreadUpdate =>
      _releaseKey != null && _notificationPrefs['readRelease'] != _releaseKey;

  bool get _isNoticeHidden =>
      _releaseKey != null &&
      _notificationPrefs['dismissedRelease'] == _releaseKey;

  void _dismissNotice() {
    final key = _releaseKey;
    if (key == null) return;
    unawaited(
      settings.updatePluginValues(_notificationSettingsId, {
        ..._notificationPrefs,
        'dismissedRelease': key,
      }),
    );
  }

  Future<void> _showNotifications(BuildContext context) async {
    final key = _releaseKey;
    if (key != null) {
      await settings.updatePluginValues(_notificationSettingsId, {
        ..._notificationPrefs,
        'readRelease': key,
      });
    }
    if (!context.mounted) return;
    await showDialog<void>(
      context: context,
      builder: (dialogContext) => AlertDialog(
        key: const ValueKey('notifications-dialog'),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(18)),
        title: const Text('通知'),
        content: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 440),
          child: updates?.available == null
              ? const Text('暂无通知')
              : ListenableBuilder(
                  listenable: Listenable.merge([
                    updates!,
                    ?desktopUpdate?.status,
                    ?desktopUpdate?.progress,
                  ]),
                  builder: (context, _) {
                    final release = updates!.available;
                    if (release == null) return const Text('暂无通知');
                    final status = _statusFor(release);
                    final downloading =
                        status == OpenMuseDesktopUpdateStatus.downloading;
                    final ready = status == OpenMuseDesktopUpdateStatus.ready;
                    final failed = status == OpenMuseDesktopUpdateStatus.failed;
                    return Column(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'OpenMuse ${release.version} (${release.build})',
                          style: Theme.of(context).textTheme.titleMedium,
                        ),
                        const SizedBox(height: 8),
                        Text(
                          release.notesFor(
                            Localizations.localeOf(context).languageCode,
                          ),
                        ),
                        if (downloading) ...[
                          const SizedBox(height: 12),
                          LinearProgressIndicator(
                            value: desktopUpdate?.progress.value ?? 0,
                          ),
                          const SizedBox(height: 4),
                          Text(
                            '${((desktopUpdate?.progress.value ?? 0) * 100).round()}%',
                          ),
                        ],
                        if (failed) ...[
                          const SizedBox(height: 8),
                          const Text('下载中断，可以从已下载位置继续。'),
                        ],
                        const SizedBox(height: 14),
                        Align(
                          alignment: Alignment.centerRight,
                          child: FilledButton(
                            key: const ValueKey('notification-update-action'),
                            onPressed: downloading
                                ? null
                                : () {
                                    Navigator.pop(dialogContext);
                                    onStartUpdate?.call();
                                  },
                            child: Text(
                              ready
                                  ? '重启更新'
                                  : failed
                                  ? '继续下载'
                                  : stringsOf(context).updatesUpdate(),
                            ),
                          ),
                        ),
                      ],
                    );
                  },
                ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogContext),
            child: const Text('关闭'),
          ),
        ],
      ),
    );
  }

  Widget _updateNotice(BuildContext context) {
    final release = updates!.available!;
    final status = _statusFor(release);

    final scheme = Theme.of(context).colorScheme;
    final downloading = status == OpenMuseDesktopUpdateStatus.downloading;
    final ready = status == OpenMuseDesktopUpdateStatus.ready;
    final failed = status == OpenMuseDesktopUpdateStatus.failed;
    final progress = desktopUpdate?.progress.value ?? 0;
    return Material(
      key: const ValueKey('app-update-footer'),
      color: scheme.primaryContainer,
      borderRadius: BorderRadius.circular(10),
      child: Padding(
        padding: const EdgeInsets.fromLTRB(10, 8, 8, 8),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Row(
              children: [
                Icon(
                  Icons.system_update_alt,
                  size: 17,
                  color: scheme.onPrimaryContainer,
                ),
                const SizedBox(width: 7),
                Expanded(
                  child: Text(
                    ready
                        ? (stringsOf(context).updatesReadyToUpdateB17ee8fe())
                        : downloading
                        ? (stringsOf(
                            context,
                          ).updatesDownloadingUpdate63359be5())
                        : failed
                        ? (stringsOf(context).updatesUpdateFailed6aaaf1ba())
                        : (stringsOf(context).updatesUpdateAvailableEb64bf34(
                            value1: release.version,
                          )),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      color: scheme.onPrimaryContainer,
                      fontSize: 12,
                      fontWeight: FontWeight.w700,
                    ),
                  ),
                ),
                if (!downloading)
                  FilledButton.tonal(
                    key: ValueKey(
                      ready ? 'app-update-restart-button' : 'app-update-button',
                    ),
                    onPressed: onStartUpdate,
                    style: FilledButton.styleFrom(
                      minimumSize: const Size(0, 30),
                      padding: const EdgeInsets.symmetric(horizontal: 9),
                      visualDensity: VisualDensity.compact,
                    ),
                    child: Text(
                      ready
                          ? (stringsOf(context).updatesRestart8c8f1d01())
                          : failed
                          ? (stringsOf(context).updatesRetry4c2e361c())
                          : stringsOf(context).updatesUpdate(),
                    ),
                  ),
                IconButton(
                  key: const ValueKey('app-update-dismiss'),
                  tooltip: stringsOf(context).updatesDismiss(),
                  onPressed: _dismissNotice,
                  constraints: const BoxConstraints.tightFor(
                    width: 24,
                    height: 24,
                  ),
                  padding: EdgeInsets.zero,
                  icon: Icon(
                    Icons.close,
                    size: 15,
                    color: scheme.onPrimaryContainer,
                  ),
                ),
              ],
            ),
            if (downloading) ...[
              const SizedBox(height: 7),
              LinearProgressIndicator(
                key: const ValueKey('app-update-progress'),
                value: progress,
              ),
              const SizedBox(height: 4),
              Text(
                '${(progress * 100).round()}%',
                style: TextStyle(
                  color: scheme.onPrimaryContainer,
                  fontSize: 11,
                ),
              ),
            ] else
              InkWell(
                onTap: onShowUpdate,
                child: Padding(
                  padding: const EdgeInsets.only(top: 2),
                  child: Text(
                    "What's new ›",
                    style: TextStyle(
                      color: scheme.onPrimaryContainer,
                      fontSize: 11,
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }

  OpenMuseDesktopUpdateStatus _statusFor(OpenMuseRelease release) {
    final update = desktopUpdate;
    if (update?.releaseSha256 != null &&
        update!.releaseSha256 != release.sha256) {
      return OpenMuseDesktopUpdateStatus.idle;
    }
    return update?.status.value ?? OpenMuseDesktopUpdateStatus.idle;
  }
}

final class _MountRow extends StatelessWidget {
  const _MountRow({
    required this.mount,
    required this.registry,
    required this.workspace,
    required this.settings,
    required this.onOpenTerminal,
  });

  final WorkspaceMount mount;
  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings settings;
  final ValueChanged<String> onOpenTerminal;

  @override
  Widget build(BuildContext context) => OpenMuseWorkspaceRow(
    name: mount.name,
    isDirectory: true,
    expanded: mount.root.expanded,
    depth: 0,
    isMount: true,
    loading: mount.root.loading,
    onTap: () => workspace.toggleDirectory(mount.root),
    onSecondaryTapDown: (details) => _showResourceMenu(
      context,
      details.globalPosition,
      mount.root,
      registry,
      workspace,
      settings,
      onOpenTerminal,
    ),
  );
}

final class _ResourceRow extends StatelessWidget {
  const _ResourceRow({
    required this.entry,
    required this.registry,
    required this.workspace,
    required this.settings,
    required this.onOpenTerminal,
    required this.selected,
  });

  final WorkspaceEntry entry;
  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings settings;
  final ValueChanged<String> onOpenTerminal;
  final bool selected;

  @override
  Widget build(BuildContext context) => OpenMuseWorkspaceRow(
    name: entry.name,
    isDirectory: entry.isDirectory,
    expanded: entry.expanded,
    depth: entry.depth,
    isMount: false,
    selected: selected,
    extension: entry.resource.extension,
    onTap: () => entry.isDirectory
        ? workspace.toggleDirectory(entry)
        : workspace.openResource(entry.resource),
    onSecondaryTapDown: (details) => _showResourceMenu(
      context,
      details.globalPosition,
      entry,
      registry,
      workspace,
      settings,
      onOpenTerminal,
    ),
  );
}

Future<void> _addWorkspace(
  BuildContext context,
  LocalWorkspaceController workspace,
) async {
  String? path;
  try {
    path = await const WorkspacePicker().chooseDirectory();
  } on MissingPluginException {
    if (!context.mounted) return;
    path = await _promptText(context, '添加工作区', '本地目录路径');
  } on PlatformException catch (error) {
    if (!context.mounted) return;
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('无法打开文件夹选择器：${error.message ?? error.code}')),
    );
    return;
  }
  if (path == null || path.trim().isEmpty || !context.mounted) return;
  try {
    await workspace.addWorkspace(path);
  } catch (error) {
    if (!context.mounted) return;
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text('无法添加工作区：$error')));
  }
}

PopupMenuItem<String> _menuAction(
  String value,
  String label,
  IconData icon, {
  bool destructive = false,
  bool enabled = true,
  Widget? trailing,
}) => PopupMenuItem<String>(
  value: value,
  enabled: enabled,
  height: 34,
  padding: const EdgeInsets.symmetric(horizontal: 10),
  child: Row(
    children: [
      Icon(
        icon,
        size: 16,
        color: destructive ? const Color(0xffe40046) : OpenMuseTokens.textMuted,
      ),
      const SizedBox(width: 10),
      Expanded(
        child: Text(
          label,
          style: TextStyle(
            fontSize: 14,
            color: destructive ? const Color(0xffe40046) : null,
          ),
        ),
      ),
      ?trailing,
    ],
  ),
);

Future<String?> _showWorkbenchMenu(
  BuildContext context,
  Offset position, {
  required List<PopupMenuEntry<String>> items,
}) => showMenu<String>(
  context: context,
  position: RelativeRect.fromLTRB(
    position.dx,
    position.dy,
    position.dx,
    position.dy,
  ),
  items: items,
  constraints: const BoxConstraints(
    minWidth: 260,
    maxWidth: 280,
    maxHeight: 480,
  ),
  menuPadding: const EdgeInsets.all(6),
  color: Theme.of(context).dialogTheme.backgroundColor,
  surfaceTintColor: Colors.transparent,
  elevation: 12,
  shape: RoundedRectangleBorder(
    borderRadius: BorderRadius.circular(10),
    side: BorderSide(color: Theme.of(context).dividerColor),
  ),
);

/// The menu is driven solely by editor contributions, never by a Host format
/// switch. A plugin can be installed/uninstalled without changing this menu.
Future<void> _showOpenWithMenu(
  BuildContext context,
  Offset origin,
  OpenMuseResource resource,
  OpenMusePluginRegistry registry,
  LocalWorkspaceController workspace,
  OpenMuseLocalSettings settings, {
  bool defaultsOnly = false,
}) async {
  if (registry.descriptors.isNotEmpty &&
      registry
          .editorCandidates(resource)
          .every((item) => item.editor.catchAll)) {
    resource = await workspace.inspectResource(resource);
    if (!context.mounted) return;
  }
  final unique = <String, OpenMuseEditorCandidate>{};
  for (final candidate in registry.editorCandidates(resource)) {
    unique.putIfAbsent(candidate.plugin.descriptor.id, () => candidate);
  }
  final selected = defaultsOnly
      ? settings.defaultEditorFor(resource.extension)
      : workspace.tabs
                .where(
                  (tab) =>
                      tab.kind == WorkspaceTabKind.resource &&
                      tab.resource.uri == resource.uri,
                )
                .firstOrNull
                ?.preferredEditorId ??
            settings.defaultEditorFor(resource.extension) ??
            registry.editorCandidates(resource).firstOrNull?.editor.id;
  final candidates = unique.values.toList();
  final command = await _showWorkbenchMenu(
    context,
    origin,
    items: [
      if (!candidates.any(
        (candidate) =>
            candidate.plugin.descriptor.name.toLowerCase().contains('ioffice'),
      ))
        _menuAction(
          'ioffice',
          'iOffice',
          Icons.description_outlined,
          enabled: false,
        ),
      for (final candidate in candidates)
        _menuAction(
          'editor:${candidate.editor.id}',
          candidate.plugin.descriptor.name == 'Helix Editor'
              ? 'Helix'
              : candidate.plugin.descriptor.name,
          candidate.plugin.descriptor.id == 'com.openmuse.helix'
              ? Icons.code
              : Icons.visibility_outlined,
          trailing: selected == candidate.editor.id
              ? const Icon(Icons.check, size: 16)
              : null,
        ),
      if (!defaultsOnly) ...[
        const PopupMenuDivider(height: 12),
        _menuAction(
          'defaults',
          '默认打开方式',
          Icons.star_outline,
          trailing: const Icon(Icons.chevron_right, size: 16),
        ),
      ],
    ],
  );
  if (command == null || !context.mounted) return;
  if (command == 'defaults') {
    await _showOpenWithMenu(
      context,
      Offset(origin.dx + 270, origin.dy + candidates.length * 34 + 46),
      resource,
      registry,
      workspace,
      settings,
      defaultsOnly: true,
    );
  } else if (command.startsWith('editor:')) {
    final editorId = command.substring('editor:'.length);
    if (defaultsOnly) {
      await settings.setDefaultEditor(resource.extension, editorId);
    }
    workspace.openResource(resource, editorId: editorId);
  }
}

Future<void> _showVersionMenu(
  BuildContext context,
  Offset origin,
  OpenMuseResource resource,
  LocalWorkspaceController workspace,
) async {
  final versions = await workspace.versions(resource);
  if (!context.mounted) return;
  final command = await _showWorkbenchMenu(
    context,
    origin,
    items: [
      _menuAction(
        'current',
        '当前工作副本',
        Icons.history,
        trailing: const Icon(Icons.check_circle_outline, size: 15),
      ),
      const PopupMenuDivider(height: 12),
      if (versions.isEmpty)
        _menuAction('empty', '尚未保存版本', Icons.info_outline, enabled: false),
      for (var index = 0; index < versions.length; index++)
        _menuAction(
          'version:$index',
          '${versions[index].shortId} · ${versions[index].createdAt.toLocal().hour.toString().padLeft(2, '0')}:${versions[index].createdAt.toLocal().minute.toString().padLeft(2, '0')}',
          Icons.radio_button_unchecked,
        ),
    ],
  );
  if (command == null || !command.startsWith('version:')) return;
  final index = int.tryParse(command.substring('version:'.length));
  if (index == null || index < 0 || index >= versions.length) return;
  await workspace.openDiff(resource, versions[index]);
}

Future<void> _showResourceMenu(
  BuildContext context,
  Offset position,
  WorkspaceEntry entry,
  OpenMusePluginRegistry registry,
  LocalWorkspaceController workspace,
  OpenMuseLocalSettings settings,
  ValueChanged<String> onOpenTerminal,
) async {
  final resource = entry.resource;
  final isMountRoot = workspace.mounts.any((m) => identical(m.root, entry));
  final command = await _showWorkbenchMenu(
    context,
    position,
    items: [
      _menuAction(
        'open',
        entry.isDirectory ? '展开／收起' : 'Open',
        Icons.open_in_new,
      ),
      if (entry.isDirectory) ...[
        const PopupMenuDivider(height: 12),
        _menuAction('new-file', 'New File…', Icons.note_add_outlined),
        _menuAction(
          'new-folder',
          'New Folder…',
          Icons.create_new_folder_outlined,
        ),
        _menuAction('refresh', 'Refresh', Icons.refresh),
        _menuAction(
          'terminal',
          _hostText(context, '在终端中打开', 'Open in Terminal'),
          Icons.terminal,
          enabled:
              settings.pluginValues('com.openmuse.cli')['enabled'] != false &&
              registry.panelProvider(
                    OpenMuseSurfaceRegion.bottomPanel,
                    panelId: 'cli.console',
                  ) !=
                  null,
        ),
      ],
      if (!entry.isDirectory)
        _menuAction(
          'open-with',
          '打开方式',
          Icons.open_in_new,
          trailing: const Icon(Icons.chevron_right, size: 16),
        ),
      if (!entry.isDirectory) ...[
        const PopupMenuDivider(height: 12),
        _menuAction('capture', '保存当前版本', Icons.save_outlined),
        _menuAction(
          'version',
          '版本',
          Icons.history,
          trailing: const Icon(Icons.chevron_right, size: 16),
        ),
        _menuAction('history', '版本历史与审计…', Icons.history),
      ],
      const PopupMenuDivider(height: 12),
      _menuAction('copy', 'Copy Path', Icons.content_copy_outlined),
      _menuAction(
        'copy-relative',
        'Copy Relative Path',
        Icons.copy_all_outlined,
      ),
      if (!isMountRoot)
        _menuAction('rename', 'Rename…', Icons.drive_file_rename_outline),
      _menuAction('reveal', 'Reveal in Finder', Icons.folder_open_outlined),
      const PopupMenuDivider(height: 12),
      if (isMountRoot && workspace.mounts.length > 1)
        _menuAction(
          'unmount',
          'Remove Workspace',
          Icons.remove_circle_outline,
          destructive: true,
        ),
      if (!isMountRoot)
        _menuAction(
          'delete',
          'Delete',
          Icons.delete_outline,
          destructive: true,
        ),
    ],
  );
  if (command == null || !context.mounted) return;
  if (command == 'open') {
    entry.isDirectory
        ? await workspace.toggleDirectory(entry)
        : workspace.openResource(resource);
  } else if (command == 'open-with') {
    await _showOpenWithMenu(
      context,
      Offset(position.dx + 270, position.dy + 34),
      resource,
      registry,
      workspace,
      settings,
    );
  } else if (command == 'capture') {
    final snapshot = await workspace.captureVersion(resource);
    if (context.mounted) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('已保存版本 ${snapshot.shortId}')));
    }
  } else if (command == 'history') {
    await _showVersionHistory(context, resource, workspace);
  } else if (command == 'version') {
    await _showVersionMenu(
      context,
      Offset(position.dx + 270, position.dy + 128),
      resource,
      workspace,
    );
  } else if (command == 'new-file' || command == 'new-folder') {
    final directory = command == 'new-folder';
    final name = await _promptText(
      context,
      directory ? '新建文件夹' : '新建文件',
      directory ? '文件夹名称' : '文件名',
    );
    if (name != null) {
      try {
        final created = await workspace.createEntry(
          entry,
          name,
          directory: directory,
        );
        if (!directory) workspace.openResource(created.resource);
      } catch (error) {
        if (context.mounted) {
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text('无法创建：$error')));
        }
      }
    }
  } else if (command == 'refresh') {
    await workspace.refreshDirectory(entry);
  } else if (command == 'terminal') {
    onOpenTerminal(entry.path);
  } else if (command == 'copy') {
    await Clipboard.setData(ClipboardData(text: entry.path));
  } else if (command == 'copy-relative') {
    await Clipboard.setData(
      ClipboardData(text: workspace.relativePath(resource)),
    );
  } else if (command == 'rename') {
    final name = await _promptText(
      context,
      '重命名',
      entry.name,
      initialValue: entry.name,
    );
    if (name != null) await workspace.renameEntry(entry, name);
  } else if (command == 'reveal') {
    await const WorkspacePicker().reveal(entry.path);
  } else if (command == 'delete') {
    final confirmed = await _confirmDelete(context, entry.name);
    if (confirmed) await workspace.deleteEntry(entry);
  } else if (command == 'unmount') {
    final mount = workspace.mounts.firstWhere((m) => identical(m.root, entry));
    await workspace.removeWorkspace(mount);
  }
}

Future<void> _showTabMenu(
  BuildContext context,
  Offset position,
  WorkspaceTab tab,
  OpenMusePluginRegistry registry,
  LocalWorkspaceController workspace,
  OpenMuseLocalSettings settings,
  String groupId,
) async {
  final command = await _showWorkbenchMenu(
    context,
    position,
    items: [
      _menuAction('close', 'Close', Icons.close),
      _menuAction('close-others', 'Close other tabs', Icons.tab_unselected),
      if (tab.kind == WorkspaceTabKind.resource &&
          tab.resource.uri.isScheme('file')) ...[
        const PopupMenuDivider(height: 12),
        _menuAction(
          'open-with',
          '打开方式',
          Icons.open_in_new,
          trailing: const Icon(Icons.chevron_right, size: 16),
        ),
        _menuAction('capture', '保存当前版本', Icons.save_outlined),
        _menuAction(
          'version',
          '版本',
          Icons.history,
          trailing: const Icon(Icons.chevron_right, size: 16),
        ),
        _menuAction('history', '版本历史与审计…', Icons.history),
        const PopupMenuDivider(height: 12),
        _menuAction('copy', 'Copy Path', Icons.content_copy_outlined),
        _menuAction(
          'copy-relative',
          'Copy Relative Path',
          Icons.copy_all_outlined,
        ),
        _menuAction('reveal', '在 Finder 中显示', Icons.folder_open_outlined),
      ],
      _menuAction('pin', tab.pinned ? 'Unpin' : 'Pin', Icons.push_pin_outlined),
    ],
  );
  if (command == null || !context.mounted) return;
  if (command == 'close') {
    workspace.closeTab(tab, groupId: groupId);
  } else if (command == 'close-others') {
    workspace.closeOtherTabs(tab, groupId: groupId);
  } else if (command == 'pin') {
    workspace.togglePinned(tab);
  } else if (command == 'open-with') {
    await _showOpenWithMenu(
      context,
      Offset(position.dx + 270, position.dy + 80),
      tab.resource,
      registry,
      workspace,
      settings,
    );
  } else if (command == 'capture') {
    final snapshot = await workspace.captureVersion(tab.resource);
    if (context.mounted) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('已保存版本 ${snapshot.shortId}')));
    }
  } else if (command == 'history') {
    await _showVersionHistory(context, tab.resource, workspace);
  } else if (command == 'version') {
    await _showVersionMenu(
      context,
      Offset(position.dx + 270, position.dy + 148),
      tab.resource,
      workspace,
    );
  } else if (command == 'copy') {
    await Clipboard.setData(ClipboardData(text: tab.resource.uri.toFilePath()));
  } else if (command == 'copy-relative') {
    await Clipboard.setData(
      ClipboardData(text: workspace.relativePath(tab.resource)),
    );
  } else if (command == 'reveal') {
    await const WorkspacePicker().reveal(tab.resource.uri.toFilePath());
  }
}

Future<void> _showVersionHistory(
  BuildContext context,
  OpenMuseResource resource,
  LocalWorkspaceController workspace,
) async {
  final versions = await workspace.versions(resource);
  if (!context.mounted) return;
  await showDialog<void>(
    context: context,
    builder: (context) => AlertDialog(
      title: Text('${resource.displayName} · 版本历史与审计'),
      content: SizedBox(
        width: 620,
        height: 380,
        child: versions.isEmpty
            ? const Center(child: Text('尚未保存本地版本'))
            : ListView.separated(
                itemCount: versions.length,
                separatorBuilder: (_, _) => const Divider(height: 1),
                itemBuilder: (context, index) {
                  final version = versions[index];
                  return ListTile(
                    leading: const Icon(Icons.history, size: 20),
                    title: Text(version.shortId),
                    subtitle: Text(
                      '${version.createdAt.toLocal()}  ·  ${version.byteLength} bytes',
                    ),
                    trailing: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        TextButton(
                          onPressed: () async {
                            Navigator.pop(context);
                            await workspace.openDiff(resource, version);
                          },
                          child: const Text('与当前比较'),
                        ),
                        PopupMenuButton<LocalVersionSnapshot>(
                          tooltip: '选择另一个历史版本',
                          onSelected: (other) async {
                            Navigator.pop(context);
                            await workspace.openVersionComparison(
                              resource,
                              version,
                              afterVersion: other,
                            );
                          },
                          itemBuilder: (_) => [
                            for (final other in versions)
                              if (other.id != version.id)
                                PopupMenuItem(
                                  value: other,
                                  child: Text('与 ${other.shortId} 比较'),
                                ),
                          ],
                          child: const Padding(
                            padding: EdgeInsets.all(8),
                            child: Text('与版本比较 ▸'),
                          ),
                        ),
                      ],
                    ),
                  );
                },
              ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('关闭'),
        ),
      ],
    ),
  );
}

Future<String?> _promptText(
  BuildContext context,
  String title,
  String label, {
  String? initialValue,
}) async {
  final controller = TextEditingController(text: initialValue);
  final result = await showDialog<String>(
    context: context,
    builder: (context) => AlertDialog(
      title: Text(title),
      content: TextField(
        controller: controller,
        autofocus: true,
        decoration: InputDecoration(labelText: label),
        onSubmitted: (value) => Navigator.pop(context, value),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('取消'),
        ),
        FilledButton(
          onPressed: () => Navigator.pop(context, controller.text),
          child: const Text('确定'),
        ),
      ],
    ),
  );
  controller.dispose();
  return result;
}

Future<bool> _confirmDelete(BuildContext context, String name) async =>
    await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('删除本地资源？'),
        content: Text('“$name”将从磁盘永久删除，此操作无法撤销。'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('取消'),
          ),
          FilledButton(
            style: FilledButton.styleFrom(
              backgroundColor: const Color(0xffd9304f),
            ),
            onPressed: () => Navigator.pop(context, true),
            child: const Text('删除'),
          ),
        ],
      ),
    ) ??
    false;

final class _EditorArea extends StatelessWidget {
  const _EditorArea({
    required this.registry,
    required this.workspace,
    required this.settings,
    this.groupId = LocalWorkspaceController.primaryEditorGroupId,
    this.paneMenu,
  });

  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings settings;
  final String groupId;
  final Widget? paneMenu;

  @override
  Widget build(BuildContext context) {
    final tab = workspace.activeTabFor(groupId);
    final current = tab?.resource;
    final plugin =
        current == null ||
            tab?.kind == WorkspaceTabKind.diff ||
            tab!.inspecting ||
            tab.resourceSession != null
        ? null
        : registry.editorFor(
            current,
            editorId:
                tab.preferredEditorId ??
                settings.defaultEditorFor(current.extension),
          );
    final banner = current != null && plugin is OpenMuseEditorBannerContributor
        ? (plugin as OpenMuseEditorBannerContributor).buildEditorBanner(
            context,
            current,
            canOpenWith: (editorId) => registry
                .editorCandidates(current)
                .any((candidate) => candidate.editor.id == editorId),
            openWith: (editorId) => workspace.openResource(
              current,
              editorId: editorId,
              groupId: groupId,
            ),
          )
        : null;
    return Column(
      children: [
        _TabStrip(
          registry: registry,
          workspace: workspace,
          settings: settings,
          groupId: groupId,
          paneMenu: paneMenu,
        ),
        ?banner,
        Expanded(
          child: tab == null
              ? _Welcome(
                  onSearch: () => _showSearch(context, workspace),
                  onCreate: () => _createDocument(context, workspace),
                )
              : IndexedStack(
                  index: workspace.tabsFor(groupId).indexOf(tab),
                  sizing: StackFit.expand,
                  children: [
                    for (final openTab in workspace.tabsFor(groupId))
                      _buildOpenTab(context, openTab),
                  ],
                ),
        ),
      ],
    );
  }

  Widget _buildOpenTab(BuildContext context, WorkspaceTab tab) {
    if (tab.resourceSession case final binding?) {
      return KeyedSubtree(
        key: ValueKey(tab.id),
        child: Builder(builder: binding.builder),
      );
    }
    if (tab.kind == WorkspaceTabKind.diff) {
      return KeyedSubtree(
        key: ValueKey(tab.id),
        child: _DiffWorkbench(diff: tab.diff!),
      );
    }
    if (tab.inspecting) return const Center(child: Text('正在识别文件格式…'));
    final plugin = registry.editorFor(
      tab.resource,
      editorId:
          tab.preferredEditorId ??
          settings.defaultEditorFor(tab.resource.extension),
    );
    if (plugin == null) return const Center(child: Text('没有已安装的插件可以打开这个资源。'));
    return TickerMode(
      key: ValueKey(
        '${tab.id}:${plugin.descriptor.id}:${plugin.descriptor.version}',
      ),
      enabled: workspace.activeTabFor(groupId) == tab,
      child: PluginEditorHost(
        registry: registry,
        plugin: plugin,
        resource: tab.resource,
      ),
    );
  }
}

final class _TabStrip extends StatelessWidget {
  const _TabStrip({
    required this.registry,
    required this.workspace,
    required this.settings,
    required this.groupId,
    this.paneMenu,
  });

  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings settings;
  final String groupId;
  final Widget? paneMenu;

  @override
  Widget build(BuildContext context) {
    final tabs = workspace.tabsFor(groupId);
    return OpenMuseEditorTabStrip(
      tabs: [
        for (final tab in tabs)
          OpenMuseEditorTabData(
            id: tab.id,
            title:
                '${tab.title}${tab.resourceSession?.session.dirty == true ? ' •' : ''}',
            extension: tab.resource.extension,
            pinned: tab.pinned,
            isDiff: tab.kind == WorkspaceTabKind.diff,
          ),
      ],
      activeId: workspace.activeTabFor(groupId)?.id,
      paneMenu: paneMenu,
      onExpandSidebar: !workspace.sidebarVisible && !Platform.isWindows
          ? workspace.toggleSidebar
          : null,
      onActivate: (id) => workspace.activateTab(
        tabs.firstWhere((tab) => tab.id == id),
        groupId: groupId,
      ),
      onClose: (id) => workspace.closeTab(
        tabs.firstWhere((tab) => tab.id == id),
        groupId: groupId,
      ),
      onContextMenu: (context, position, id) => _showTabMenu(
        context,
        position,
        tabs.firstWhere((tab) => tab.id == id),
        registry,
        workspace,
        settings,
        groupId,
      ),
    );
  }
}

enum _DiffRowKind { equal, added, deleted }

final class _DiffRow {
  const _DiffRow(this.kind, this.oldLine, this.newLine, this.text);
  final _DiffRowKind kind;
  final int? oldLine;
  final int? newLine;
  final String text;
}

final class _DiffWorkbench extends StatefulWidget {
  const _DiffWorkbench({required this.diff});
  final WorkspaceDiff diff;

  @override
  State<_DiffWorkbench> createState() => _DiffWorkbenchState();
}

final class _DiffWorkbenchState extends State<_DiffWorkbench> {
  late final List<_DiffRow> rows = _lineDiff(
    widget.diff.before,
    widget.diff.after,
  );
  bool sideBySide = false;

  int get additions =>
      rows.where((row) => row.kind == _DiffRowKind.added).length;
  int get deletions =>
      rows.where((row) => row.kind == _DiffRowKind.deleted).length;

  @override
  Widget build(BuildContext context) => Column(
    children: [
      Container(
        height: 36,
        padding: const EdgeInsets.symmetric(horizontal: 9),
        decoration: BoxDecoration(
          color: Theme.of(context).brightness == Brightness.dark
              ? const Color(0xff202228)
              : const Color(0xfff7f8fb),
          border: Border(
            bottom: BorderSide(color: Theme.of(context).dividerColor),
          ),
        ),
        child: Row(
          children: [
            _DiffPill(
              '$additions 处变更',
              const Color(0xffe5f4ff),
              const Color(0xff1581bd),
            ),
            const SizedBox(width: 6),
            _DiffPill(
              '+$additions',
              const Color(0xffe8f8e9),
              const Color(0xff2e9b45),
            ),
            const SizedBox(width: 6),
            _DiffPill(
              '-$deletions',
              const Color(0xffffeeee),
              const Color(0xffd74b4b),
            ),
            const SizedBox(width: 12),
            SegmentedButton<bool>(
              segments: const [
                ButtonSegment(value: false, label: Text('统一')),
                ButtonSegment(value: true, label: Text('并排')),
              ],
              selected: {sideBySide},
              onSelectionChanged: (value) =>
                  setState(() => sideBySide = value.single),
              style: const ButtonStyle(visualDensity: VisualDensity.compact),
            ),
            const Spacer(),
            Text(
              '快照 ${widget.diff.snapshot.shortId}  ↔  ${widget.diff.afterLabel}',
              style: const TextStyle(
                fontSize: 11,
                color: OpenMuseTokens.textMuted,
              ),
            ),
          ],
        ),
      ),
      Expanded(
        child: sideBySide
            ? _SideBySideDiff(rows: rows)
            : _UnifiedDiff(rows: rows),
      ),
    ],
  );
}

final class _DiffPill extends StatelessWidget {
  const _DiffPill(this.label, this.background, this.foreground);
  final String label;
  final Color background;
  final Color foreground;

  @override
  Widget build(BuildContext context) => Container(
    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
    decoration: BoxDecoration(
      color: background,
      borderRadius: BorderRadius.circular(12),
    ),
    child: Text(label, style: TextStyle(fontSize: 11, color: foreground)),
  );
}

final class _UnifiedDiff extends StatelessWidget {
  const _UnifiedDiff({required this.rows});
  final List<_DiffRow> rows;

  @override
  Widget build(BuildContext context) => ListView.builder(
    itemCount: rows.length,
    itemExtent: 22,
    itemBuilder: (context, index) {
      final row = rows[index];
      final dark = Theme.of(context).brightness == Brightness.dark;
      final colors = switch (row.kind) {
        _DiffRowKind.added => (
          dark ? const Color(0xff20382a) : const Color(0xffe8f6e9),
          dark ? const Color(0xff85d79b) : const Color(0xff35a853),
          '+',
        ),
        _DiffRowKind.deleted => (
          dark ? const Color(0xff42282a) : const Color(0xffffecec),
          dark ? const Color(0xffff9292) : const Color(0xffd94b4b),
          '-',
        ),
        _DiffRowKind.equal => (
          Theme.of(context).scaffoldBackgroundColor,
          OpenMuseTokens.textMuted,
          ' ',
        ),
      };
      return ColoredBox(
        color: colors.$1,
        child: Row(
          children: [
            SizedBox(
              width: 48,
              child: Text(
                '${row.oldLine ?? ''}',
                textAlign: TextAlign.right,
                style: const TextStyle(
                  fontSize: 10,
                  color: OpenMuseTokens.textMuted,
                ),
              ),
            ),
            const SizedBox(width: 8),
            SizedBox(
              width: 48,
              child: Text(
                '${row.newLine ?? ''}',
                textAlign: TextAlign.right,
                style: const TextStyle(
                  fontSize: 10,
                  color: OpenMuseTokens.textMuted,
                ),
              ),
            ),
            const SizedBox(width: 8),
            Text(
              colors.$3,
              style: TextStyle(color: colors.$2, fontFamily: 'Menlo'),
            ),
            const SizedBox(width: 6),
            Expanded(
              child: Text(
                row.text,
                overflow: TextOverflow.ellipsis,
                style: const TextStyle(fontFamily: 'Menlo', fontSize: 12),
              ),
            ),
          ],
        ),
      );
    },
  );
}

final class _SideBySideDiff extends StatelessWidget {
  const _SideBySideDiff({required this.rows});
  final List<_DiffRow> rows;

  @override
  Widget build(BuildContext context) => ListView.builder(
    itemCount: rows.length,
    itemExtent: 24,
    itemBuilder: (context, index) {
      final row = rows[index];
      return Row(
        children: [
          Expanded(
            child: _DiffCell(
              line: row.oldLine,
              text: row.kind == _DiffRowKind.added ? '' : row.text,
              deleted: row.kind == _DiffRowKind.deleted,
            ),
          ),
          const VerticalDivider(width: 1),
          Expanded(
            child: _DiffCell(
              line: row.newLine,
              text: row.kind == _DiffRowKind.deleted ? '' : row.text,
              added: row.kind == _DiffRowKind.added,
            ),
          ),
        ],
      );
    },
  );
}

final class _DiffCell extends StatelessWidget {
  const _DiffCell({
    this.line,
    required this.text,
    this.added = false,
    this.deleted = false,
  });
  final int? line;
  final String text;
  final bool added;
  final bool deleted;

  @override
  Widget build(BuildContext context) => ColoredBox(
    color: added
        ? (Theme.of(context).brightness == Brightness.dark
              ? const Color(0xff20382a)
              : const Color(0xffe8f6e9))
        : deleted
        ? (Theme.of(context).brightness == Brightness.dark
              ? const Color(0xff42282a)
              : const Color(0xffffecec))
        : Theme.of(context).scaffoldBackgroundColor,
    child: Row(
      children: [
        SizedBox(
          width: 42,
          child: Text(
            '${line ?? ''}',
            textAlign: TextAlign.right,
            style: const TextStyle(
              fontSize: 10,
              color: OpenMuseTokens.textMuted,
            ),
          ),
        ),
        const SizedBox(width: 10),
        Expanded(
          child: Text(
            text,
            overflow: TextOverflow.ellipsis,
            style: const TextStyle(fontFamily: 'Menlo', fontSize: 12),
          ),
        ),
      ],
    ),
  );
}

List<_DiffRow> _lineDiff(String before, String after) {
  final oldLines = before.replaceAll('\r\n', '\n').split('\n');
  final newLines = after.replaceAll('\r\n', '\n').split('\n');
  if (oldLines.length * newLines.length > 4000000) {
    return [
      for (var i = 0; i < oldLines.length; i++)
        _DiffRow(_DiffRowKind.deleted, i + 1, null, oldLines[i]),
      for (var i = 0; i < newLines.length; i++)
        _DiffRow(_DiffRowKind.added, null, i + 1, newLines[i]),
    ];
  }
  final matrix = List.generate(
    oldLines.length + 1,
    (_) => List<int>.filled(newLines.length + 1, 0),
  );
  for (var i = oldLines.length - 1; i >= 0; i--) {
    for (var j = newLines.length - 1; j >= 0; j--) {
      matrix[i][j] = oldLines[i] == newLines[j]
          ? matrix[i + 1][j + 1] + 1
          : (matrix[i + 1][j] >= matrix[i][j + 1]
                ? matrix[i + 1][j]
                : matrix[i][j + 1]);
    }
  }
  final rows = <_DiffRow>[];
  var i = 0;
  var j = 0;
  while (i < oldLines.length || j < newLines.length) {
    if (i < oldLines.length &&
        j < newLines.length &&
        oldLines[i] == newLines[j]) {
      rows.add(_DiffRow(_DiffRowKind.equal, i + 1, j + 1, oldLines[i]));
      i++;
      j++;
    } else if (j < newLines.length &&
        (i == oldLines.length || matrix[i][j + 1] >= matrix[i + 1][j])) {
      rows.add(_DiffRow(_DiffRowKind.added, null, j + 1, newLines[j++]));
    } else {
      rows.add(_DiffRow(_DiffRowKind.deleted, i + 1, null, oldLines[i++]));
    }
  }
  return rows;
}

final class _Welcome extends StatelessWidget {
  const _Welcome({required this.onSearch, required this.onCreate});
  final VoidCallback onSearch;
  final VoidCallback onCreate;

  @override
  Widget build(BuildContext context) =>
      OpenMuseWorkbenchWelcome(onSearch: onSearch, onCreate: onCreate);
}

final class _MacTitleBar extends StatelessWidget {
  const _MacTitleBar({
    required this.workspace,
    required this.onDrag,
    required this.onSettings,
    required this.onShowAgent,
  });

  final LocalWorkspaceController workspace;
  final VoidCallback onDrag;
  final VoidCallback onSettings;
  final VoidCallback onShowAgent;

  @override
  Widget build(BuildContext context) {
    OpenMuseWindowChrome.ensureMacFullScreen();
    return ValueListenableBuilder<bool>(
      valueListenable: OpenMuseWindowChrome.fullScreen,
      builder: (context, fullScreen, _) => SizedBox(
        height: 36,
        child: Padding(
          padding: EdgeInsets.only(left: fullScreen ? 10 : 78),
          child: Row(
            children: [
              IconButton(
                key: const ValueKey('title-sidebar-toggle'),
                tooltip: workspace.sidebarVisible
                    ? _hostText(context, '收起侧边栏', 'Collapse Sidebar')
                    : _hostText(context, '展开侧边栏', 'Expand Sidebar'),
                onPressed: workspace.toggleSidebar,
                visualDensity: VisualDensity.compact,
                padding: EdgeInsets.zero,
                constraints: const BoxConstraints.tightFor(
                  width: 28,
                  height: 28,
                ),
                icon: const _SidebarToggleIcon(),
              ),
              const SizedBox(width: 12),
              IconButton(
                key: const ValueKey('title-settings'),
                tooltip: _hostText(context, '设置', 'Settings'),
                onPressed: onSettings,
                visualDensity: VisualDensity.compact,
                padding: EdgeInsets.zero,
                constraints: const BoxConstraints.tightFor(
                  width: 28,
                  height: 28,
                ),
                icon: Icon(
                  Icons.settings_outlined,
                  size: 16,
                  color: Theme.of(
                    context,
                  ).colorScheme.onSurface.withValues(alpha: 0.7),
                ),
              ),
              IconButton(
                key: const ValueKey('title-show-agent'),
                tooltip: _hostText(context, '显示 Agent 面板', 'Show Agent panel'),
                onPressed: onShowAgent,
                visualDensity: VisualDensity.compact,
                constraints: const BoxConstraints.tightFor(
                  width: 28,
                  height: 28,
                ),
                padding: EdgeInsets.zero,
                icon: const Icon(Icons.forum_outlined, size: 16),
              ),
              Expanded(
                child: Listener(
                  behavior: HitTestBehavior.opaque,
                  onPointerDown: (event) {
                    if ((event.buttons & kPrimaryMouseButton) != 0) onDrag();
                  },
                  child: const SizedBox.expand(),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

final class _WindowsTitleStrip extends StatelessWidget {
  const _WindowsTitleStrip({
    required this.registry,
    required this.workspace,
    required this.settings,
    required this.view,
    required this.onView,
    required this.onViewHelp,
    this.onTerminal,
  });

  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings settings;
  final _HostView view;
  final ValueChanged<_HostView> onView;
  final VoidCallback onViewHelp;
  final VoidCallback? onTerminal;

  Future<void> _showViewMenu(BuildContext context) async {
    final box = context.findRenderObject() as RenderBox?;
    if (box == null || !box.hasSize) return;
    final origin = box.localToGlobal(Offset.zero);
    final selected = await _showWorkbenchMenu(
      context,
      Offset(origin.dx, origin.dy + box.size.height),
      items: [
        _menuAction(
          'agent',
          _hostText(context, 'Agent 视图', 'Agent view'),
          Icons.forum_outlined,
          trailing: Icon(
            Icons.check,
            size: 16,
            color: view == _HostView.agent
                ? OpenMuseTokens.text
                : Colors.transparent,
          ),
        ),
        _menuAction(
          'ide',
          _hostText(context, 'IDE 视图', 'IDE view'),
          Icons.web_asset_outlined,
          trailing: Icon(
            Icons.check,
            size: 16,
            color: view == _HostView.ide
                ? OpenMuseTokens.text
                : Colors.transparent,
          ),
        ),
        const PopupMenuDivider(height: 8),
        _menuAction(
          'help',
          _hostText(context, '视图帮助', 'View help'),
          Icons.help_outline,
        ),
      ],
    );
    switch (selected) {
      case 'agent':
        onView(_HostView.agent);
      case 'ide':
        onView(_HostView.ide);
      case 'help':
        onViewHelp();
    }
  }

  @override
  Widget build(BuildContext context) {
    OpenMuseWindowChrome.ensureBound();
    final dark = Theme.of(context).brightness == Brightness.dark;
    final buttonStyle = TextButton.styleFrom(
      minimumSize: const Size(48, 32),
      tapTargetSize: MaterialTapTargetSize.shrinkWrap,
      visualDensity: VisualDensity.compact,
      padding: const EdgeInsets.symmetric(horizontal: 10),
    );
    return Container(
      height: OpenMuseTokens.topBarHeight,
      decoration: BoxDecoration(
        color: dark ? const Color(0xff202228) : OpenMuseTokens.sidebar,
        border: Border(
          bottom: BorderSide(
            color: dark ? const Color(0x24FFFFFF) : const Color(0x14000000),
          ),
        ),
      ),
      child: Row(
        children: [
          const SizedBox(width: 10),
          const OpenMuseBrandMark(),
          const SizedBox(width: 8),
          const Text(
            'OpenMuse',
            style: TextStyle(fontSize: 14, fontWeight: FontWeight.w600),
          ),
          const SizedBox(width: 12),
          IconButton(
            key: const ValueKey('title-sidebar-toggle'),
            tooltip: workspace.sidebarVisible
                ? _hostText(context, '收起侧边栏', 'Collapse Sidebar')
                : _hostText(context, '展开侧边栏', 'Expand Sidebar'),
            onPressed: workspace.toggleSidebar,
            visualDensity: VisualDensity.compact,
            padding: EdgeInsets.zero,
            constraints: const BoxConstraints.tightFor(width: 32, height: 32),
            icon: const _SidebarToggleIcon(),
          ),
          TextButton(
            key: const ValueKey('title-settings'),
            style: buttonStyle,
            onPressed: () => showOpenMuseSettings(
              context,
              settings,
              registry,
              onPluginWorkspace:
                  ({required pluginId, required path, required enabled}) =>
                      choosePluginWorkspace(
                        workspace: workspace,
                        pluginId: pluginId,
                        path: path,
                        enabled: enabled,
                      ),
            ),
            child: Text(_hostText(context, '设置', 'Settings')),
          ),
          Builder(
            builder: (buttonContext) => TextButton(
              key: const ValueKey('title-view'),
              style: buttonStyle,
              onPressed: () => _showViewMenu(buttonContext),
              child: Text(_hostText(context, '视图', 'View')),
            ),
          ),
          TextButton(
            key: const ValueKey('title-terminal'),
            style: buttonStyle,
            onPressed: onTerminal,
            child: Text(_hostText(context, '终端', 'Terminal')),
          ),
          Expanded(
            child: Listener(
              behavior: HitTestBehavior.opaque,
              onPointerDown: (event) {
                if ((event.buttons & kPrimaryMouseButton) != 0) {
                  OpenMuseWindowChrome.startDrag();
                }
              },
              child: const SizedBox.expand(),
            ),
          ),
          if (Platform.isWindows) const _WindowsCaptionButtons(),
        ],
      ),
    );
  }
}

String _hostText(BuildContext context, String zh, String en) {
  final code =
      OpenMuseUiLocaleScope.maybeOf(context)?.locale.languageCode ??
      Localizations.maybeLocaleOf(context)?.languageCode ??
      WidgetsBinding.instance.platformDispatcher.locale.languageCode;
  return code == 'zh' ? zh : en;
}

enum _HostView { agent, ide }

WorkbenchLayoutSnapshot _layoutForView(_HostView view) {
  if (view == _HostView.agent) return createDefaultWorkbenchLayout();
  final workspace = PaneNode(paneId: 'workspace', minWidth: 232);
  final editor = PaneNode(paneId: 'editor', minWidth: 280);
  final agent = PaneNode(paneId: 'dsh', minWidth: 280);
  return WorkbenchLayoutSnapshot(
    root: SplitNode(
      axis: Axis.horizontal,
      ratio: 0.2,
      first: workspace,
      second: SplitNode(
        axis: Axis.horizontal,
        ratio: 0.62,
        first: editor,
        second: agent,
      ),
    ),
    bindings: createDefaultWorkbenchLayout().bindings,
    focusedPaneId: 'editor',
  );
}

final class _SidebarToggleIcon extends StatelessWidget {
  const _SidebarToggleIcon();

  @override
  Widget build(BuildContext context) => CustomPaint(
    size: const Size(17, 15),
    painter: _SidebarTogglePainter(
      Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7),
    ),
  );
}

final class _SidebarTogglePainter extends CustomPainter {
  const _SidebarTogglePainter(this.color);
  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    final stroke = Paint()
      ..color = color
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.3;
    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromLTWH(1, 1, size.width - 2, size.height - 2),
        const Radius.circular(3),
      ),
      stroke,
    );
    canvas.drawLine(
      const Offset(6.5, 1.5),
      Offset(6.5, size.height - 1.5),
      stroke,
    );
  }

  @override
  bool shouldRepaint(covariant _SidebarTogglePainter oldDelegate) =>
      color != oldDelegate.color;
}

final class _WindowsCaptionButtons extends StatelessWidget {
  const _WindowsCaptionButtons();

  @override
  Widget build(BuildContext context) => ValueListenableBuilder<bool>(
    valueListenable: OpenMuseWindowChrome.maximized,
    builder: (context, maximized, _) {
      final color = OpenMuseTokens.textMuted;
      return Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          _CaptionButton(
            tooltip: '最小化',
            icon: Icons.remove,
            color: color,
            onPressed: OpenMuseWindowChrome.minimize,
          ),
          _CaptionButton(
            tooltip: maximized ? '还原' : '最大化',
            icon: maximized ? Icons.filter_none : Icons.crop_square,
            color: color,
            iconSize: maximized ? 12 : 14,
            onPressed: OpenMuseWindowChrome.toggleMaximized,
          ),
          _CaptionButton(
            tooltip: '关闭',
            icon: Icons.close,
            color: color,
            destructive: true,
            onPressed: OpenMuseWindowChrome.close,
          ),
        ],
      );
    },
  );
}

final class _CaptionButton extends StatefulWidget {
  const _CaptionButton({
    required this.tooltip,
    required this.icon,
    required this.color,
    required this.onPressed,
    this.iconSize = 16,
    this.destructive = false,
  });

  final String tooltip;
  final IconData icon;
  final Color color;
  final VoidCallback onPressed;
  final double iconSize;
  final bool destructive;

  @override
  State<_CaptionButton> createState() => _CaptionButtonState();
}

final class _CaptionButtonState extends State<_CaptionButton> {
  bool _hover = false;

  @override
  Widget build(BuildContext context) {
    final background = !_hover
        ? Colors.transparent
        : widget.destructive
        ? const Color(0xffe81123)
        : Theme.of(context).brightness == Brightness.dark
        ? const Color(0x22ffffff)
        : const Color(0x14000000);
    final iconColor = _hover && widget.destructive
        ? Colors.white
        : widget.color;
    return MouseRegion(
      onEnter: (_) => setState(() => _hover = true),
      onExit: (_) => setState(() => _hover = false),
      child: Tooltip(
        message: widget.tooltip,
        waitDuration: const Duration(milliseconds: 400),
        child: GestureDetector(
          onTap: widget.onPressed,
          child: ColoredBox(
            color: background,
            child: SizedBox(
              width: 46,
              height: OpenMuseTokens.topBarHeight,
              child: Icon(widget.icon, size: widget.iconSize, color: iconColor),
            ),
          ),
        ),
      ),
    );
  }
}

Future<void> _showSearch(
  BuildContext context,
  LocalWorkspaceController workspace,
) => showDialog<void>(
  context: context,
  barrierColor: OpenMuseTokens.scrim,
  builder: (context) => _SearchDialog(workspace: workspace),
);

final class _SearchDialog extends StatefulWidget {
  const _SearchDialog({required this.workspace});

  final LocalWorkspaceController workspace;

  @override
  State<_SearchDialog> createState() => _SearchDialogState();
}

final class _SearchDialogState extends State<_SearchDialog> {
  final query = TextEditingController();

  @override
  void dispose() {
    query.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final results = widget.workspace.search(query.text);
    return Dialog(
      alignment: const Alignment(0, -0.48),
      insetPadding: const EdgeInsets.symmetric(horizontal: 24),
      child: SizedBox(
        width: 780,
        height: 490,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(12, 12, 12, 8),
              child: TextField(
                controller: query,
                autofocus: true,
                onChanged: (_) => setState(() {}),
                decoration: InputDecoration(
                  hintText: '搜索本地工作区…',
                  prefixIcon: const Icon(Icons.search, size: 19),
                  suffixIcon: IconButton(
                    tooltip: '关闭',
                    onPressed: () => Navigator.pop(context),
                    icon: const Icon(Icons.close, size: 18),
                  ),
                  isDense: true,
                  filled: true,
                  fillColor: const Color(0xfffafafd),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(8),
                    borderSide: const BorderSide(color: OpenMuseTokens.border),
                  ),
                ),
              ),
            ),
            const Padding(
              padding: EdgeInsets.fromLTRB(18, 4, 18, 8),
              child: Text(
                '本地资源',
                style: TextStyle(
                  fontSize: 12,
                  color: OpenMuseTokens.textMuted,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
            const Divider(height: 1),
            Expanded(
              child: results.isEmpty
                  ? const Center(
                      child: Text(
                        '没有匹配的资源',
                        style: TextStyle(color: OpenMuseTokens.textMuted),
                      ),
                    )
                  : ListView.builder(
                      padding: const EdgeInsets.all(8),
                      itemCount: results.length,
                      itemBuilder: (context, index) {
                        final resource = results[index];
                        return ListTile(
                          dense: true,
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(7),
                          ),
                          leading: Icon(
                            OpenMuseWorkspaceRow.iconFor(resource.extension),
                            size: 18,
                          ),
                          title: Text(resource.displayName),
                          subtitle: Text(
                            resource.uri.toFilePath(),
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          ),
                          onTap: () {
                            widget.workspace.select(resource);
                            Navigator.pop(context);
                          },
                        );
                      },
                    ),
            ),
          ],
        ),
      ),
    );
  }
}

Future<void> _createDocument(
  BuildContext context,
  LocalWorkspaceController workspace,
) async {
  final result = await showDialog<String>(
    context: context,
    builder: (dialogContext) => const _CreateDocumentDialog(),
  );
  if (result == null || !context.mounted) return;
  try {
    await workspace.createMarkdown(result);
  } on FileSystemException catch (error) {
    if (!context.mounted) return;
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text('无法创建文档：${error.message}')));
  }
}

final class _CreateDocumentDialog extends StatefulWidget {
  const _CreateDocumentDialog();

  @override
  State<_CreateDocumentDialog> createState() => _CreateDocumentDialogState();
}

final class _CreateDocumentDialogState extends State<_CreateDocumentDialog> {
  final name = TextEditingController();

  @override
  void dispose() {
    name.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => AlertDialog(
    title: const Text('新建本地文档'),
    content: SizedBox(
      width: 380,
      child: TextField(
        controller: name,
        autofocus: true,
        decoration: const InputDecoration(
          labelText: '名称',
          hintText: '未命名文档',
          suffixText: '.md',
          helperText: '文件会保存在当前本地工作区。',
        ),
        onSubmitted: (value) => Navigator.pop(context, value),
      ),
    ),
    actions: [
      TextButton(
        onPressed: () => Navigator.pop(context),
        child: const Text('取消'),
      ),
      FilledButton(
        onPressed: () => Navigator.pop(context, name.text),
        child: const Text('创建'),
      ),
    ],
  );
}

final class _PluginMarketPage extends StatefulWidget {
  const _PluginMarketPage({required this.registry, required this.workspace});

  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;

  @override
  State<_PluginMarketPage> createState() => _PluginMarketPageState();
}

final class _PluginMarketPageState extends State<_PluginMarketPage> {
  var _selected = 0;
  final _category = ValueNotifier<String>('精选');
  final _searchFocusNode = FocusNode();

  @override
  void dispose() {
    _category.dispose();
    _searchFocusNode.dispose();
    super.dispose();
  }

  void _showCategory(String category) {
    setState(() => _selected = 0);
    _category.value = category;
  }

  void _focusSearch() {
    if (_selected != 0) setState(() => _selected = 0);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _searchFocusNode.requestFocus();
    });
  }

  @override
  Widget build(BuildContext context) {
    if (RuntimeConfigScope.maybeOf(
          context,
        )?.isEnabled(AppFeature.pluginMarket) ==
        false) {
      return Scaffold(
        body: Center(
          child: Text(stringsOf(context).commonFeatureUnavailable()),
        ),
      );
    }
    final theme = openMuseMarketTheme();
    final size = MediaQuery.sizeOf(context);
    final wide = size.width >= 850;
    final market = widget.registry.panelProvider(
      OpenMuseSurfaceRegion.dialog,
      panelId: 'plugin.market',
    );
    return Theme(
      data: theme,
      child: CallbackShortcuts(
        bindings: {
          const SingleActivator(LogicalKeyboardKey.keyK, meta: true):
              _focusSearch,
          const SingleActivator(LogicalKeyboardKey.keyK, control: true):
              _focusSearch,
        },
        child: Focus(
          autofocus: true,
          child: Scaffold(
            key: const ValueKey('desktop-market-page'),
            backgroundColor: const Color(0xFFFBFCFE),
            body: SafeArea(
              child: Row(
                children: [
                  Container(
                    width: wide ? 224 : 72,
                    decoration: const BoxDecoration(
                      gradient: LinearGradient(
                        begin: Alignment.topLeft,
                        end: Alignment.bottomRight,
                        colors: [Color(0xFFE9F4FF), Color(0xFFF4FAFA)],
                      ),
                      border: Border(
                        right: BorderSide(color: MarketColors.line),
                      ),
                    ),
                    child: Column(
                      children: [
                        const SizedBox(height: 34),
                        Padding(
                          padding: const EdgeInsets.fromLTRB(12, 0, 12, 24),
                          child: Material(
                            color: const Color(0xFFE3EBF7),
                            borderRadius: BorderRadius.circular(11),
                            child: InkWell(
                              key: const ValueKey(
                                'plugin-market-sidebar-search',
                              ),
                              borderRadius: BorderRadius.circular(11),
                              onTap: _focusSearch,
                              child: SizedBox(
                                height: 44,
                                child: Row(
                                  children: [
                                    const SizedBox(width: 12),
                                    const Icon(
                                      Icons.search_rounded,
                                      size: 21,
                                      color: MarketColors.secondary,
                                    ),
                                    if (wide) ...[
                                      const SizedBox(width: 9),
                                      Expanded(
                                        child: Text(
                                          Platform.isMacOS
                                              ? '⌘ K  快速搜索'
                                              : 'Ctrl K  快速搜索',
                                          style: TextStyle(
                                            color: MarketColors.secondary,
                                            fontSize: 12,
                                          ),
                                        ),
                                      ),
                                    ],
                                  ],
                                ),
                              ),
                            ),
                          ),
                        ),
                        ValueListenableBuilder<String>(
                          valueListenable: _category,
                          builder: (context, category, _) => Column(
                            children: [
                              _navigationItem(
                                label: '探索',
                                icon: Icons.explore_outlined,
                                selected: _selected == 0 && category == '精选',
                                wide: wide,
                                key: const ValueKey('plugin-market-tab-0'),
                                onTap: () => _showCategory('精选'),
                              ),
                              for (final item in const [
                                ('办公', Icons.work_outline),
                                ('UI', Icons.palette_outlined),
                                ('编程', Icons.code_rounded),
                                ('生活', Icons.local_cafe_outlined),
                              ])
                                _navigationItem(
                                  label: item.$1,
                                  icon: item.$2,
                                  selected:
                                      _selected == 0 && category == item.$1,
                                  wide: wide,
                                  key: ValueKey('plugin-market-nav-${item.$1}'),
                                  onTap: () => _showCategory(item.$1),
                                ),
                            ],
                          ),
                        ),
                        const Spacer(),
                        _navigationItem(
                          label: '我的插件',
                          icon: Icons.apps_outlined,
                          selected: _selected == 1,
                          wide: wide,
                          key: const ValueKey('plugin-market-tab-1'),
                          onTap: () => setState(() => _selected = 1),
                        ),
                        const SizedBox(height: 12),
                        const Divider(height: 1, color: MarketColors.line),
                        const SizedBox(height: 8),
                        _navigationItem(
                          label: 'OpenMuse',
                          icon: Icons.account_circle_outlined,
                          selected: false,
                          wide: wide,
                          key: const ValueKey('plugin-market-return-workbench'),
                          onTap: () => Navigator.pop(context),
                        ),
                        const SizedBox(height: 12),
                      ],
                    ),
                  ),
                  Expanded(
                    child: Stack(
                      children: [
                        MarketCategoryScope(
                          category: _category,
                          searchFocusNode: _searchFocusNode,
                          child: _selected == 0
                              ? market?.buildPanel(context, 'plugin.market') ??
                                    const Center(child: Text('插件市场暂不可用'))
                              : SingleChildScrollView(
                                  child: Column(
                                    children: [
                                      market?.buildPanel(
                                            context,
                                            'plugin.market.installed',
                                          ) ??
                                          const Center(
                                            child: Text('DSH 插件列表暂不可用'),
                                          ),
                                      Divider(
                                        height: 1,
                                        color: theme.dividerColor,
                                      ),
                                      _InstalledOpenMusePlugins(
                                        registry: widget.registry,
                                        workspace: widget.workspace,
                                        onOpen: (candidate) => Navigator.of(
                                          context,
                                        ).pop(candidate),
                                      ),
                                    ],
                                  ),
                                ),
                        ),
                        Positioned(
                          right: 18,
                          top: 14,
                          child: IconButton(
                            tooltip: '关闭插件市场',
                            onPressed: () => Navigator.pop(context),
                            icon: const Icon(Icons.close_rounded),
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _navigationItem({
    required String label,
    required IconData icon,
    required bool selected,
    required bool wide,
    required Key key,
    required VoidCallback onTap,
  }) => Padding(
    padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 3),
    child: Tooltip(
      message: label,
      child: Material(
        color: selected ? const Color(0xFFD7E7FF) : Colors.transparent,
        borderRadius: BorderRadius.circular(11),
        child: InkWell(
          key: key,
          borderRadius: BorderRadius.circular(11),
          onTap: onTap,
          child: SizedBox(
            height: 46,
            child: Row(
              children: [
                const SizedBox(width: 12),
                Icon(
                  icon,
                  size: 21,
                  color: selected ? MarketColors.blue : MarketColors.ink,
                ),
                if (wide) ...[
                  const SizedBox(width: 12),
                  Text(
                    label,
                    style: TextStyle(
                      color: selected ? MarketColors.blue : MarketColors.ink,
                      fontWeight: selected ? FontWeight.w700 : FontWeight.w500,
                    ),
                  ),
                ],
              ],
            ),
          ),
        ),
      ),
    ),
  );
}

final class _InstalledOpenMusePlugins extends StatelessWidget {
  const _InstalledOpenMusePlugins({
    required this.registry,
    required this.workspace,
    required this.onOpen,
  });

  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final ValueChanged<OpenMusePanelCandidate> onOpen;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: registry,
    builder: (context, _) => ListView.separated(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      padding: const EdgeInsets.fromLTRB(20, 12, 20, 18),
      itemCount: registry.descriptors.length + 1,
      separatorBuilder: (_, _) => const Divider(height: 1),
      itemBuilder: (context, index) {
        if (index == 0) {
          return const Padding(
            padding: EdgeInsets.fromLTRB(4, 2, 4, 10),
            child: Text(
              'OpenMuse 插件',
              style: TextStyle(fontWeight: FontWeight.w600),
            ),
          );
        }
        final descriptor = registry.descriptors.elementAt(index - 1);
        final state = registry.stateOf(descriptor.id);
        final candidate = registry
            .panelCandidates()
            .where(
              (item) =>
                  item.plugin.descriptor.id == descriptor.id &&
                  item.panel.region != OpenMuseSurfaceRegion.dialog,
            )
            .firstOrNull;
        return ListTile(
          contentPadding: const EdgeInsets.symmetric(
            vertical: 5,
            horizontal: 4,
          ),
          leading: Container(
            width: 36,
            height: 36,
            decoration: BoxDecoration(
              color: Theme.of(context).colorScheme.primaryContainer,
              borderRadius: BorderRadius.circular(9),
            ),
            child: Icon(
              Icons.extension_outlined,
              size: 19,
              color: Theme.of(context).colorScheme.onPrimaryContainer,
            ),
          ),
          title: Text(descriptor.name),
          subtitle: Text(
            '${descriptor.id}  ·  ${descriptor.version}  ·  ${descriptor.runtime.name}',
          ),
          trailing: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              _StatePill(state: state),
              const SizedBox(width: 8),
              if (candidate != null)
                TextButton(
                  key: ValueKey('plugin-open-${descriptor.id}'),
                  onPressed: () => onOpen(candidate),
                  child: const Text('打开'),
                ),
              TextButton(
                onPressed: () =>
                    _openPluginLog(context, registry, workspace, descriptor),
                child: const Text('日志输出'),
              ),
            ],
          ),
        );
      },
    ),
  );
}

Future<void> _openPluginLog(
  BuildContext context,
  OpenMusePluginRegistry registry,
  LocalWorkspaceController workspace,
  OpenMusePluginDescriptor descriptor,
) async {
  try {
    final plugin = registry.plugin(descriptor.id);
    final runtimeLog = plugin is OpenMusePluginLogContributor
        ? await (plugin as OpenMusePluginLogContributor).readLog()
        : '插件没有提供运行时日志。';
    final directory = Directory(
      '${Directory.systemTemp.path}/openmuse-plugin-logs',
    );
    await directory.create(recursive: true);
    final safeId = descriptor.id.replaceAll(RegExp(r'[^A-Za-z0-9._-]'), '_');
    final file = File(
      '${directory.path}/$safeId-${DateTime.now().microsecondsSinceEpoch}.log',
    );
    await file.writeAsString(
      '${descriptor.name} (${descriptor.id})\n'
      '状态: ${registry.stateOf(descriptor.id)?.name ?? '未知'}\n\n'
      '${registry.logOf(descriptor.id)}\n\n$runtimeLog\n',
      flush: true,
    );
    if (!context.mounted) return;
    Navigator.pop(context);
    workspace.openResource(
      OpenMuseResource(
        uri: file.uri,
        displayName: '${descriptor.name} 日志.log',
        mediaType: 'text/plain',
      ),
      editorId: 'viewer.text',
    );
  } catch (error) {
    if (!context.mounted) return;
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text('无法打开插件日志：$error')));
  }
}

final class _StatePill extends StatelessWidget {
  const _StatePill({required this.state});

  final OpenMusePluginState? state;

  @override
  Widget build(BuildContext context) {
    final active = state == OpenMusePluginState.active;
    final failed = state == OpenMusePluginState.failed;
    final label = switch (state) {
      OpenMusePluginState.active => '运行中',
      OpenMusePluginState.activating => '启动中',
      OpenMusePluginState.deactivating => '停止中',
      OpenMusePluginState.failed => '启动失败',
      _ => '已安装',
    };
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 4),
      decoration: BoxDecoration(
        color: failed
            ? const Color(0xffffece8)
            : active
            ? const Color(0xffe9f7ec)
            : const Color(0xfff0f1f5),
        borderRadius: BorderRadius.circular(20),
      ),
      child: Text(
        label,
        style: TextStyle(
          fontSize: 11,
          color: failed
              ? const Color(0xffb42318)
              : active
              ? const Color(0xff267a3b)
              : OpenMuseTokens.textMuted,
        ),
      ),
    );
  }
}
