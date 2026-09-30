library openmuse_cloud_workspace_plugin;

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// The account-scoped Cloud Workspace domain plugin.
///
/// Authentication remains owned by the selected authentication contributor;
/// this plugin consumes only its short-lived token capability. DSH is exposed
/// through [DshRuntimeConnector], so local and remote runtimes differ at the
/// transport boundary rather than in Host workspace semantics.
final class OpenMuseCloudWorkspacePlugin implements OpenMusePlugin {
  OpenMuseCloudWorkspacePlugin({
    required this.authentication,
    required Uri cloudOrigin,
    required String deviceId,
    bool allowInsecureLoopback = false,
    AppFlowyCloudWorkspaceService? service,
  }) : service =
           service ??
           AppFlowyCloudWorkspaceService(
             baseUri: cloudOrigin,
             accessToken: authentication.accessToken,
             refreshAccessToken: () =>
                 authentication.accessToken(forceRefresh: true),
             deviceId: deviceId,
             allowHttpForTesting: allowInsecureLoopback,
           );

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  late final CloudWorkspacePluginController controller =
      CloudWorkspacePluginController(
        authentication: authentication,
        service: service,
      );

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.workspace.cloud',
    name: 'Cloud Workspace',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup', 'onPanel:cloud.workspace'],
    permissions: {
      'network.cloud',
      'workspace.catalog.read',
      'dsh.remote.attach',
    },
    panels: [
      OpenMusePanelContribution(
        id: 'cloud.workspace',
        region: OpenMuseSurfaceRegion.rightSidebar,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) => controller.activate();

  @override
  Future<void> deactivate() async {
    controller.dispose();
    service.closeClient();
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnsupportedError('Cloud Workspace contributes a panel');

  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      panelId == 'cloud.workspace'
      ? CloudWorkspacePluginPanel(controller: controller)
      : null;
}

@immutable
final class CloudWorkspacePluginSnapshot {
  const CloudWorkspacePluginSnapshot({
    this.loading = false,
    this.workspaces = const [],
    this.sessions = const [],
    this.openingWorkspaceRef,
    this.failureMessage,
  });

  final bool loading;
  final List<CloudWorkspaceRecord> workspaces;
  final List<DshSessionSummary> sessions;
  final String? openingWorkspaceRef;
  final String? failureMessage;

  DshSessionSummary? sessionFor(String workspaceRef) {
    for (final session in sessions) {
      if (session.workspaceRef == workspaceRef &&
          (session.state == 'running' || session.state == 'ready')) {
        return session;
      }
    }
    return null;
  }
}

final class CloudWorkspacePluginController extends ChangeNotifier {
  CloudWorkspacePluginController({
    required this.authentication,
    required this.service,
  });

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  CloudWorkspacePluginSnapshot _snapshot = const CloudWorkspacePluginSnapshot();
  bool _disposed = false;

  CloudWorkspacePluginSnapshot get snapshot => _snapshot;

  Future<void> activate() async {
    authentication.addListener(_authenticationChanged);
    if (authentication.snapshot.isAuthenticated) await refresh();
  }

  void _authenticationChanged() {
    if (!authentication.snapshot.isAuthenticated) {
      _publish(const CloudWorkspacePluginSnapshot());
      return;
    }
    unawaited(refresh());
  }

  Future<void> refresh() async {
    if (!authentication.snapshot.isAuthenticated || _snapshot.loading) return;
    _publish(
      CloudWorkspacePluginSnapshot(
        loading: true,
        workspaces: _snapshot.workspaces,
        sessions: _snapshot.sessions,
      ),
    );
    try {
      final workspaces = await service.listWorkspaces();
      var sessions = const <DshSessionSummary>[];
      try {
        sessions = await service.listSessions();
      } on Object {
        // DSH session presence is optional catalog enrichment. Keep the
        // account's workspaces usable while the execution pool recovers.
      }
      _publish(
        CloudWorkspacePluginSnapshot(
          workspaces: workspaces,
          sessions: sessions,
        ),
      );
    } catch (_) {
      _publish(
        CloudWorkspacePluginSnapshot(
          workspaces: _snapshot.workspaces,
          sessions: _snapshot.sessions,
          failureMessage: 'Cloud Workspace 暂时不可用，请稍后重试。',
        ),
      );
    }
  }

  Future<DshSessionDescriptor?> openWorkspace(String workspaceRef) async {
    if (_snapshot.openingWorkspaceRef != null) return null;
    _publish(
      CloudWorkspacePluginSnapshot(
        workspaces: _snapshot.workspaces,
        sessions: _snapshot.sessions,
        openingWorkspaceRef: workspaceRef,
      ),
    );
    try {
      final descriptor = await service.open(workspaceRef, 0);
      await refresh();
      return descriptor;
    } catch (_) {
      _publish(
        CloudWorkspacePluginSnapshot(
          workspaces: _snapshot.workspaces,
          sessions: _snapshot.sessions,
          failureMessage: '无法连接 Remote DSH，请检查 Workspace 与服务状态。',
        ),
      );
      return null;
    }
  }

  void _publish(CloudWorkspacePluginSnapshot value) {
    if (_disposed) return;
    _snapshot = value;
    notifyListeners();
  }

  @override
  void dispose() {
    if (_disposed) return;
    _disposed = true;
    authentication.removeListener(_authenticationChanged);
    super.dispose();
  }
}

final class CloudWorkspacePluginPanel extends StatelessWidget {
  const CloudWorkspacePluginPanel({super.key, required this.controller});

  final CloudWorkspacePluginController controller;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: controller,
    builder: (context, _) {
      final snapshot = controller.snapshot;
      return Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(12, 8, 6, 6),
            child: Row(
              children: [
                const Expanded(
                  child: Text(
                    'CLOUD WORKSPACES',
                    style: TextStyle(fontSize: 11, letterSpacing: 0.8),
                  ),
                ),
                IconButton(
                  key: const ValueKey('cloud-workspace.refresh'),
                  tooltip: '刷新',
                  iconSize: 17,
                  onPressed: snapshot.loading ? null : controller.refresh,
                  icon: const Icon(Icons.refresh),
                ),
              ],
            ),
          ),
          if (snapshot.loading) const LinearProgressIndicator(minHeight: 2),
          if (snapshot.failureMessage case final message?)
            Padding(
              padding: const EdgeInsets.all(12),
              child: Text(message, style: const TextStyle(fontSize: 12)),
            ),
          Expanded(
            child: snapshot.workspaces.isEmpty && !snapshot.loading
                ? const Center(child: Text('没有可用的 Cloud Workspace'))
                : ListView.builder(
                    itemCount: snapshot.workspaces.length,
                    itemBuilder: (context, index) {
                      final workspace = snapshot.workspaces[index];
                      final session = snapshot.sessionFor(
                        workspace.workspaceRef,
                      );
                      final opening =
                          snapshot.openingWorkspaceRef ==
                          workspace.workspaceRef;
                      return ListTile(
                        key: ValueKey(
                          'cloud-workspace.${workspace.workspaceRef}',
                        ),
                        dense: true,
                        leading: Icon(
                          session == null
                              ? Icons.cloud_outlined
                              : Icons.cloud_done_outlined,
                          size: 19,
                        ),
                        title: Text(workspace.title),
                        subtitle: Text(
                          session == null
                              ? 'Cloud Workspace'
                              : 'Remote DSH 运行中 · ${session.attachedDeviceCount} 设备',
                        ),
                        trailing: opening
                            ? const SizedBox.square(
                                dimension: 18,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                ),
                              )
                            : TextButton(
                                onPressed: () => controller.openWorkspace(
                                  workspace.workspaceRef,
                                ),
                                child: Text(session == null ? '连接' : '复用'),
                              ),
                      );
                    },
                  ),
          ),
        ],
      );
    },
  );
}
