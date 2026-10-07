library;

import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'src/dsh_panel.dart';
import 'src/dsh_sidecar.dart';
import 'src/dsh_workspace_binding.dart';
import 'src/dsh_workspace_sync.dart';
import 'src/plugin_interaction.dart';

export 'src/dsh_sidecar.dart';
export 'src/dsh_web_view.dart' show DshNativeOverlay, DshPopupRouteObserver;
export 'src/plugin_interaction.dart' show PluginInteraction;

final class OpenMuseDshPlugin
    implements
        OpenMusePlugin,
        OpenMuseSettingsContributor,
        OpenMusePluginLogContributor {
  OpenMuseDshPlugin({
    DshSidecarSupervisor? supervisor,
    Future<bool> Function(PluginInteraction)? routeInteraction,
  }) : _supervisor = supervisor,
       _routeInteraction = routeInteraction;

  DshSidecarSupervisor? _supervisor;
  final Future<bool> Function(PluginInteraction)? _routeInteraction;
  DshWorkspaceBinding? _binding;
  DshWorkspaceSynchronizer? _workspaceSync;
  OpenMusePluginContext? _context;
  final ValueNotifier<String?> _activeMount = ValueNotifier(null);
  final ValueNotifier<PluginInteraction?> _desktopInteraction = ValueNotifier(null);
  String? _pluginInteractionDir;
  PluginInteractionListener? _interactionListener;

  @override
  Future<String> readLog() async {
    final supervisor = _supervisor;
    if (supervisor == null) return 'DSH Sidecar 尚未启动。';
    return [
      'DSH 状态: ${supervisor.state.name}',
      if (supervisor.lastError != null) '运行错误: ${supervisor.lastError}',
      '运行时: ${supervisor.cliPath ?? '未配置'}',
      ...supervisor.logTail,
    ].join('\n');
  }

  void _workspaceChanged() {
    final binding = _binding;
    if (binding != null) {
      // Publication is serialized and content-deduplicated by the binding.
      binding.publish().catchError((Object error) {
        debugPrint('DSH workspace binding failed: $error');
      });
    }
    _syncWorkspaces();
  }

  void _syncWorkspaces() {
    _workspaceSync?.sync().catchError((Object error) {
      debugPrint('DSH workspace catalog sync failed: $error');
    });
  }

  void _sidecarChanged() {
    if (_supervisor?.state != DshSidecarState.ready) return;
    _syncWorkspaces();
  }

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.dsh-agent',
    name: 'DSH Agent',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.nativeProcess,
    activationEvents: ['onStartup', 'onPanel:dsh.agent'],
    permissions: {
      'workspace.context.read',
      'resource.open.request',
      'process.sidecar',
      'credentials.model.use',
    },
    panels: [
      OpenMusePanelContribution(
        id: 'dsh.agent',
        region: OpenMuseSurfaceRegion.rightSidebar,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    _context = context;
    final snapshot = await context.executeHostCommand(
      'workspace.snapshot',
      null,
    );
    if (snapshot is Map && snapshot['activeMountPath'] is String) {
      _activeMount.value = snapshot['activeMountPath'] as String;
    }
    if (snapshot is Map && snapshot['pluginInteractionDir'] is String) {
      _pluginInteractionDir = snapshot['pluginInteractionDir'] as String;
    }
    if (_pluginInteractionDir case final directory?) {
      _interactionListener = PluginInteractionListener(directory, (interaction) {
        unawaited(() async {
          try {
            if (await _routeInteraction?.call(interaction) == true) return;
          } catch (error) {
            debugPrint('Plugin interaction routing failed: $error');
          }
          _desktopInteraction.value = interaction;
        }());
      })..start();
    }
    _binding = DshWorkspaceBinding(
      context,
      activeMountPath: () => _activeMount.value,
    );
    await _binding!.publish();
    context.hostChanges?.addListener(_workspaceChanged);
    _supervisor ??= DshSidecarSupervisor(
      environment: {
        ...Platform.environment,
        if (_binding!.dshHome case final home?) 'DSH_HOME': home,
      },
    );
    _workspaceSync = DshWorkspaceSynchronizer(
      context: context,
      endpoint: () => _supervisor?.endpoint,
      bridgeToken: _supervisor!.bridgeToken,
    );
    _supervisor!.addListener(_sidecarChanged);
    _workspaceChanged();
    unawaited(_startSidecar());
  }

  Future<void> _startSidecar() async {
    try {
      await _supervisor?.ensureStarted();
    } catch (error) {
      debugPrint('DSH sidecar start deferred: $error');
    }
  }

  @override
  Future<void> deactivate() async {
    _interactionListener?.stop();
    _interactionListener = null;
    _desktopInteraction.value = null;
    _context?.hostChanges?.removeListener(_workspaceChanged);
    _supervisor?.removeListener(_sidecarChanged);
    _context = null;
    _binding = null;
    _workspaceSync = null;
    await _supervisor?.stop();
    _supervisor?.dispose();
    _supervisor = null;
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnsupportedError('DSH contributes a panel, not an editor');

  @override
  Widget? buildPanel(BuildContext context, String panelId) {
    if (panelId != 'dsh.agent') return null;
    final supervisor = _supervisor;
    if (supervisor == null) {
      throw StateError('DSH plugin must be active before building its panel');
    }
    return DshPanel(
      supervisor: supervisor,
      desktopInteraction: _desktopInteraction,
      activeMount: _activeMount,
      onActivateWorkspace: (path) async {
        _activeMount.value = path;
        await _binding?.publish();
        await _context?.executeHostCommand('workspace.activateMount', {
          'path': path,
        });
      },
      onOpenResource: (request) async {
        await _context?.executeHostCommand('workspace.openResource', {
          'path': request.path,
          'cwd': request.cwd,
          if (request.line != null) 'line': request.line,
        });
      },
    );
  }

  @override
  Widget buildSettings(BuildContext context) {
    final supervisor = _supervisor;
    if (supervisor == null) {
      return const Text('DSH Sidecar 尚未启动。');
    }
    return ListenableBuilder(
      listenable: supervisor,
      builder: (context, _) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const SizedBox(height: 18),
          const Text(
            'DSH Sidecar',
            style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 8),
          Text('状态：${supervisor.state.name}'),
          Text('运行时：${supervisor.cliPath == null ? '未配置' : '已配置'}'),
          const Text('模型供应商与模型可在 DSH 启动后设置；启动不要求 DeepSeek API Key。'),
          const SizedBox(height: 8),
          Text(
            '运行时路径由 OPENMUSE_DSH_CLI 配置；工作区绑定自动同步。',
            style: TextStyle(
              fontSize: 12,
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
          ),
        ],
      ),
    );
  }
}
