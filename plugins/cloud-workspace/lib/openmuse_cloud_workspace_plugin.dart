library openmuse_cloud_workspace_plugin;

import 'package:openmuse_i18n/openmuse_i18n.dart';

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'src/workspace_access_dialog.dart';
import 'src/workspace_invitations_dialog.dart';
import 'src/workspace_invite_code_join_dialog.dart';
export 'src/workspace_invite_code_join_controller.dart';
export 'src/workspace_invite_code_join_dialog.dart';
export 'src/workspace_invite_code_controller.dart';
export 'src/workspace_invite_code_panel.dart';
export 'src/workspace_invitations_dialog.dart';
export 'src/workspace_invitations_controller.dart';
import 'src/workspace_pages.dart';
export 'src/workspace_access_controller.dart';
export 'src/owner_invitations_controller.dart';
export 'src/owner_invitations_panel.dart';
export 'src/workspace_access_dialog.dart';
export 'src/workspace_pages.dart';
export 'src/workspace_catalog_controller.dart';
export 'src/document_creation_controller.dart';
export 'src/document_creation_dialog.dart';
export 'src/page_rename_controller.dart';
export 'src/page_rename_dialog.dart';
export 'src/document_browser_link_dialog.dart';

/// The account-scoped Cloud Workspace domain plugin.
///
/// Authentication remains owned by the selected authentication contributor;
/// this plugin consumes only its short-lived token capability. DSH is exposed
/// through [DshRuntimeConnector], so local and remote runtimes differ at the
/// transport boundary rather than in Host workspace semantics.
final class OpenMuseCloudWorkspacePlugin
    implements OpenMusePlugin, OpenMuseWorkspaceActionsContributor {
  OpenMuseCloudWorkspacePlugin({
    required this.authentication,
    required Uri cloudOrigin,
    required String deviceId,
    bool allowInsecureLoopback = false,
    AppFlowyCloudWorkspaceService? service,
    this.onOpenDocument,
    this.onOpenDatabase,
  }) : service =
           service ??
           AppFlowyCloudWorkspaceService(
             baseUri: cloudOrigin,
             accessToken: authentication.accessToken,
             refreshAccessToken: () =>
                 authentication.accessToken(forceRefresh: true),
             accountSubject: () => authentication.snapshot.isAuthenticated
                 ? authentication.snapshot.identity?.subject
                 : null,
             deviceId: deviceId,
             allowHttpForTesting: allowInsecureLoopback,
           );

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final OpenCloudDocument? onOpenDocument;
  final OpenCloudDatabase? onOpenDatabase;
  late final CloudWorkspacePluginController controller =
      CloudWorkspacePluginController(
        authentication: authentication,
        service: service,
        onOpenDocument: onOpenDocument,
        onOpenDatabase: onOpenDatabase,
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

  @override
  List<Widget> buildWorkspaceActions(BuildContext context) => [
    TextButton.icon(
      key: const ValueKey('cloud-workspace.open'),
      icon: const Icon(Icons.group_outlined, size: 18),
      label: Text(
        Localizations.localeOf(context).languageCode == 'zh'
            ? '协作工作区'
            : 'Shared workspaces',
      ),
      onPressed: () => Navigator.of(context).push(
        MaterialPageRoute<void>(
          settings: const RouteSettings(name: 'openmuse.cloud-workspaces'),
          builder: (_) => Scaffold(
            appBar: AppBar(title: Text(stringsOf(context).cloudTitle())),
            body: CloudWorkspacePluginPanel(controller: controller),
          ),
        ),
      ),
    ),
  ];
}

@immutable
final class CloudWorkspacePluginSnapshot {
  const CloudWorkspacePluginSnapshot({
    this.loading = false,
    this.workspaces = const [],
    this.sessions = const [],
    this.openingWorkspaceRef,
    this.failureMessage,
    this.failureCode,
  });

  final bool loading;
  final List<CloudWorkspaceRecord> workspaces;
  final List<DshSessionSummary> sessions;
  final String? openingWorkspaceRef;
  final String? failureMessage;
  final String? failureCode;

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
    this.onOpenDocument,
    this.onOpenDatabase,
  });

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final OpenCloudDocument? onOpenDocument;
  final OpenCloudDatabase? onOpenDatabase;
  Future<void> openPage(AppFlowyWorkspacePage page) async {
    switch (page) {
      case AppFlowyDocumentPage():
        if (onOpenDocument == null)
          throw StateError('document handler unavailable');
        await onOpenDocument!(page);
      case AppFlowyDatabasePage():
        if (onOpenDatabase == null)
          throw StateError('database handler unavailable');
        await onOpenDatabase!(page);
      default:
        throw StateError('view handler unavailable');
    }
  }

  CloudWorkspacePluginSnapshot _snapshot = const CloudWorkspacePluginSnapshot();
  String? _subject;
  bool _disposed = false;
  int _generation = 0;

  CloudWorkspacePluginSnapshot get snapshot => _snapshot;

  Future<void> activate() async {
    authentication.addListener(_authenticationChanged);
    _subject = authentication.snapshot.identity?.subject;
    if (authentication.snapshot.isAuthenticated) await refresh();
  }

  void _authenticationChanged() {
    final subject = authentication.snapshot.isAuthenticated
        ? authentication.snapshot.identity?.subject
        : null;
    if (_subject != subject) {
      service.invalidateAccountOperations();
      _generation++;
      _subject = subject;
      _publish(const CloudWorkspacePluginSnapshot());
    }
    if (!authentication.snapshot.isAuthenticated) {
      return;
    }
    unawaited(refresh());
  }

  Future<void> refresh() async {
    if (!authentication.snapshot.isAuthenticated || _snapshot.loading) return;
    final subject = authentication.snapshot.identity?.subject;
    final generation = _generation;
    final token = await authentication.accessToken();
    if (subject != _subject || generation != _generation || _disposed) return;
    if (token == null || token.isEmpty) {
      _publish(const CloudWorkspacePluginSnapshot());
      return;
    }
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
      if (subject != _subject || generation != _generation || _disposed) return;
      _publish(
        CloudWorkspacePluginSnapshot(
          workspaces: workspaces,
          sessions: sessions,
        ),
      );
    } catch (_) {
      if (subject != _subject || generation != _generation || _disposed) return;
      _publish(
        CloudWorkspacePluginSnapshot(
          workspaces: _snapshot.workspaces,
          sessions: _snapshot.sessions,
          failureCode: 'CLOUD_UNAVAILABLE',
        ),
      );
    }
  }

  Future<DshSessionDescriptor?> openWorkspace(String workspaceRef) async {
    if (_snapshot.openingWorkspaceRef != null) return null;
    final subject = _subject;
    final generation = _generation;
    _publish(
      CloudWorkspacePluginSnapshot(
        workspaces: _snapshot.workspaces,
        sessions: _snapshot.sessions,
        openingWorkspaceRef: workspaceRef,
      ),
    );
    try {
      final descriptor = await service.open(workspaceRef, 0);
      if (subject != _subject || generation != _generation || _disposed)
        return null;
      await refresh();
      return descriptor;
    } catch (_) {
      if (subject != _subject || generation != _generation || _disposed)
        return null;
      _publish(
        CloudWorkspacePluginSnapshot(
          workspaces: _snapshot.workspaces,
          sessions: _snapshot.sessions,
          failureCode: 'DSH_UNAVAILABLE',
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
                Expanded(
                  child: Text(
                    stringsOf(context).cloudTitle(),
                    style: TextStyle(fontSize: 11, letterSpacing: 0.8),
                  ),
                ),
                IconButton(
                  key: const ValueKey('cloud-workspace.join-code'),
                  tooltip: Localizations.localeOf(context).languageCode == 'zh'
                      ? '输入邀请码加入'
                      : 'Join with code',
                  iconSize: 18,
                  constraints: const BoxConstraints(
                    minWidth: 44,
                    minHeight: 44,
                  ),
                  visualDensity: VisualDensity.standard,
                  icon: const Icon(Icons.group_add_outlined),
                  onPressed: !controller.authentication.snapshot.isAuthenticated
                      ? null
                      : () => showWorkspaceInviteCodeJoinDialog(
                          context,
                          authentication: controller.authentication,
                          service: controller.service,
                          onJoined: controller.refresh,
                        ),
                ),
                IconButton(
                  key: const ValueKey('cloud-workspace.invitations'),
                  tooltip: Localizations.localeOf(context).languageCode == 'zh'
                      ? '群组邀请'
                      : 'Workspace invitations',
                  iconSize: 18,
                  icon: const Icon(Icons.mail_outline),
                  onPressed: !controller.authentication.snapshot.isAuthenticated
                      ? null
                      : () => showWorkspaceInvitationsDialog(
                          context,
                          authentication: controller.authentication,
                          service: controller.service,
                          onJoined: controller.refresh,
                        ),
                ),
                IconButton(
                  key: const ValueKey('cloud-workspace.refresh'),
                  tooltip: stringsOf(context).cloudRefresh(),
                  iconSize: 17,
                  onPressed: snapshot.loading ? null : controller.refresh,
                  icon: const Icon(Icons.refresh),
                ),
              ],
            ),
          ),
          if (snapshot.loading) const LinearProgressIndicator(minHeight: 2),
          if (snapshot.failureCode != null || snapshot.failureMessage != null)
            Padding(
              padding: const EdgeInsets.all(12),
              child: Text(switch (snapshot.failureCode) {
                'DSH_UNAVAILABLE' => stringsOf(context).cloudDshUnavailable(),
                _ => stringsOf(context).cloudUnavailable(),
              }, style: const TextStyle(fontSize: 12)),
            ),
          Expanded(
            child: snapshot.workspaces.isEmpty && !snapshot.loading
                ? Center(child: Text(stringsOf(context).cloudEmpty()))
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
                        onTap: () => Navigator.of(context).push(
                          MaterialPageRoute<void>(
                            settings: const RouteSettings(
                              name: 'openmuse.cloud-pages',
                            ),
                            builder: (_) => CloudWorkspacePages(
                              authentication: controller.authentication,
                              service: controller.service,
                              workspaceId: workspace.workspaceRef,
                              title: workspace.title,
                              onOpen: controller.openPage,
                              supportsDocumentCreation:
                                  controller.onOpenDocument != null,
                            ),
                          ),
                        ),
                        subtitle: Text(
                          session == null
                              ? stringsOf(context).cloudWorkspace()
                              : stringsOf(context).cloudRunning(
                                  devices: session.attachedDeviceCount,
                                ),
                        ),
                        trailing: opening
                            ? const SizedBox.square(
                                dimension: 18,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                ),
                              )
                            : Row(
                                mainAxisSize: MainAxisSize.min,
                                children: [
                                  IconButton(
                                    key: ValueKey(
                                      'cloud-workspace.members.${workspace.workspaceRef}',
                                    ),
                                    tooltip:
                                        Localizations.localeOf(
                                              context,
                                            ).languageCode ==
                                            'zh'
                                        ? '群组成员'
                                        : 'Workspace members',
                                    icon: const Icon(
                                      Icons.group_outlined,
                                      size: 18,
                                    ),
                                    onPressed: () => showWorkspaceAccessDialog(
                                      context,
                                      authentication: controller.authentication,
                                      service: controller.service,
                                      workspaceId: workspace.workspaceRef,
                                      title: workspace.title,
                                    ),
                                  ),
                                  TextButton(
                                    onPressed: () => controller.openWorkspace(
                                      workspace.workspaceRef,
                                    ),
                                    child: Text(
                                      session == null
                                          ? stringsOf(context).cloudConnect()
                                          : stringsOf(context).cloudReuse(),
                                    ),
                                  ),
                                ],
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
