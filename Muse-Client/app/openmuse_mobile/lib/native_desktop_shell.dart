import 'mobile_build_locale.dart';

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import 'native_dsh_page.dart';
import 'plugin_interaction_remote.dart';

/// Native Mobile information architecture for one paired Desktop.
///
/// The Desktop DSH workspace registry and session controller are authoritative;
/// this shell owns no shadow task database and never manufactures replies.
final class NativeDesktopShell extends StatefulWidget {
  const NativeDesktopShell({
    super.key,
    required this.connection,
    this.initialSessionId,
  });

  final PairedDesktopConnection connection;
  final String? initialSessionId;

  @override
  State<NativeDesktopShell> createState() => _NativeDesktopShellState();
}

final class _NativeDesktopShellState extends State<NativeDesktopShell> {
  DshNativeGatewayClient? _catalog;
  Timer? _refreshTimer;
  List<DshNativeWorkspaceSummary> _workspaces = const [];
  List<DshNativeSessionSummary> _sessions = const [];
  String? _workspaceId;
  String? _sessionId;
  String? _failure;
  bool _loading = true;
  bool _creating = false;
  bool _refreshing = false;
  late bool _followDesktopActivity = widget.initialSessionId == null;
  String? _lastCatalogLog;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (_catalog == null) unawaited(_initialize());
  }

  DshNativeGatewayClient _newClient() => DshNativeGatewayClient(
    origin: Uri.parse(widget.connection.session.origin),
    bootstrapPath: widget.connection.session.path,
    allowInsecureLoopback: widget.connection.session.allowInsecureLoopback,
    allowInsecurePrivateNetworkForTesting:
        widget.connection.session.allowInsecurePrivateNetworkForTesting,
  );

  Future<void> _initialize() async {
    final localeStrings = stringsOf(context);

    final client = _newClient();
    _catalog = client;
    try {
      final hello = await client.initialize();
      if (!hello.supported ||
          !hello.capabilities.contains('workspaces.list') ||
          !hello.capabilities.contains('session.create')) {
        throw DshNativeGatewayException(
          (localeStrings.mobileDesktopDoesNotYetSupportTheComplete8ad183cf()),
        );
      }
      final negotiation = await client.negotiate(
        DshNativeCapabilities.standard(),
      );
      if (negotiation.requiresWebFallback) {
        throw DshNativeGatewayException(
          (localeStrings.mobileThisDesktopPluginRequiresItsWebConve49ea4be8()),
        );
      }
      await _refreshCatalog(selectActive: true);
      _refreshTimer = Timer.periodic(
        const Duration(seconds: 2),
        (_) => unawaited(_refreshCatalog(selectActive: true)),
      );
    } catch (error) {
      if (!mounted) return;
      setState(() {
        _failure = error.toString();
        _loading = false;
      });
    }
  }

  Future<void> _refreshCatalog({bool selectActive = false}) async {
    final client = _catalog;
    if (client == null || _refreshing) return;
    _refreshing = true;
    try {
      final values = await Future.wait<Object>([
        client.listWorkspaces(),
        client.listSessions(),
      ]);
      if (!mounted) return;
      final workspaces = values[0] as List<DshNativeWorkspaceSummary>;
      final sessions = values[1] as List<DshNativeSessionSummary>;
      final byId = {for (final session in sessions) session.sessionId: session};
      var workspaceId = _workspaceId;
      if (workspaceId == null ||
          !workspaces.any((item) => item.workspaceId == workspaceId)) {
        workspaceId = workspaces.isEmpty ? null : workspaces.first.workspaceId;
      }
      var workspace = _findWorkspace(workspaces, workspaceId);
      var sessionId = _sessionId;
      if (sessionId != null &&
          (workspace == null || !workspace.sessionIds.contains(sessionId))) {
        sessionId = null;
      }
      final requested = widget.initialSessionId;
      if (requested != null && _sessionId == null) {
        sessionId = requested;
        for (final candidate in workspaces) {
          if (candidate.sessionIds.contains(requested)) {
            workspace = candidate;
            workspaceId = candidate.workspaceId;
            break;
          }
        }
      }
      if (selectActive && _followDesktopActivity && sessionId == null) {
        final workspaceBySession = <String, DshNativeWorkspaceSummary>{
          for (final candidate in workspaces)
            for (final id in candidate.sessionIds) id: candidate,
        };
        final candidates =
            sessions
                .where((item) => workspaceBySession.containsKey(item.sessionId))
                .toList()
              ..sort((a, b) => b.updatedAt.compareTo(a.updatedAt));
        final latestRunning = candidates
            .where((item) => item.running)
            .firstOrNull;
        final current = sessionId == null ? null : byId[sessionId];
        final latest = latestRunning ?? candidates.firstOrNull;
        if (latest != null &&
            (current == null ||
                latest.running ||
                latest.updatedAt > current.updatedAt)) {
          sessionId = latest.sessionId;
          workspace = workspaceBySession[latest.sessionId];
          workspaceId = workspace?.workspaceId;
        }
      }
      if (sessionId == null && workspace != null) {
        final candidates =
            workspace.sessionIds
                .map((id) => byId[id])
                .whereType<DshNativeSessionSummary>()
                .toList()
              ..sort((a, b) => b.updatedAt.compareTo(a.updatedAt));
        sessionId = candidates.firstOrNull?.sessionId;
      }
      final newest = sessions.isEmpty
          ? null
          : (sessions.toList()
                  ..sort((a, b) => b.updatedAt.compareTo(a.updatedAt)))
                .first;
      final summary =
          'OpenMuse catalog: follow=$_followDesktopActivity '
          'select=$selectActive before=$_sessionId chosen=$sessionId '
          'sessions=${sessions.length} '
          'newest=${newest?.sessionId} running=${newest?.running} '
          'blank=${newest?.blank} title=${newest?.title ?? ''}';
      if (summary != _lastCatalogLog) {
        _lastCatalogLog = summary;
        debugPrint(summary);
      }
      setState(() {
        _workspaces = workspaces;
        _sessions = sessions;
        _workspaceId = workspaceId;
        _sessionId = sessionId;
        _failure = null;
        _loading = false;
      });
    } catch (error) {
      if (!mounted) return;
      setState(() {
        _failure = error.toString();
        _loading = false;
      });
    } finally {
      _refreshing = false;
    }
  }

  DshNativeWorkspaceSummary? _findWorkspace(
    List<DshNativeWorkspaceSummary> values,
    String? id,
  ) {
    if (id == null) return null;
    for (final value in values) {
      if (value.workspaceId == id) return value;
    }
    return null;
  }

  DshNativeSessionSummary? _session(String id) {
    for (final value in _sessions) {
      if (value.sessionId == id) return value;
    }
    return null;
  }

  Future<void> _createSession() async {
    final client = _catalog;
    final workspaceId = _workspaceId;
    if (client == null || workspaceId == null || _creating) return;
    setState(() => _creating = true);
    try {
      final created = await client.createSession(workspaceId: workspaceId);
      await _refreshCatalog();
      if (!mounted) return;
      setState(() {
        _sessionId = created.sessionId;
        _followDesktopActivity = false;
      });
    } catch (error) {
      if (!mounted) return;
      setState(() => _failure = error.toString());
    } finally {
      if (mounted) setState(() => _creating = false);
    }
  }

  void _selectWorkspace(String id) {
    final workspace = _findWorkspace(_workspaces, id);
    final byId = {for (final item in _sessions) item.sessionId: item};
    final candidates =
        (workspace?.sessionIds ?? const <String>[])
            .map((sessionId) => byId[sessionId])
            .whereType<DshNativeSessionSummary>()
            .toList()
          ..sort((a, b) => b.updatedAt.compareTo(a.updatedAt));
    setState(() {
      _workspaceId = id;
      _sessionId = candidates.firstOrNull?.sessionId;
      _followDesktopActivity = false;
    });
    Navigator.maybePop(context);
  }

  void _selectSession(String workspaceId, String sessionId) {
    setState(() {
      _workspaceId = workspaceId;
      _sessionId = sessionId;
      _followDesktopActivity = false;
    });
    Navigator.maybePop(context);
  }

  @override
  void dispose() {
    _refreshTimer?.cancel();
    _catalog?.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final workspace = _findWorkspace(_workspaces, _workspaceId);
    return Scaffold(
      key: const ValueKey('native-desktop-shell'),
      drawer: _CatalogDrawer(
        deviceName: widget.connection.deviceName,
        workspaces: _workspaces,
        sessions: _sessions,
        selectedWorkspaceId: _workspaceId,
        selectedSessionId: _sessionId,
        creating: _creating,
        onCreate: _createSession,
        onWorkspace: _selectWorkspace,
        onSession: _selectSession,
        onRefresh: () => _refreshCatalog(),
      ),
      appBar: AppBar(
        titleSpacing: 0,
        title: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              _sessionId == null
                  ? (workspace?.title ?? widget.connection.deviceName)
                  : (_session(_sessionId!)?.title ??
                        (stringsOf(context).mobileNewConversationBcec2f30())),
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: const TextStyle(fontSize: 17, fontWeight: FontWeight.w600),
            ),
            Text(
              '${widget.connection.deviceName} · ${workspace?.title ?? (stringsOf(context).mobileChooseAWorkspace49d3d398())}',
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: Theme.of(context).textTheme.bodySmall,
            ),
          ],
        ),
        actions: [
          IconButton(
            key: const ValueKey('native-follow-desktop'),
            tooltip: _followDesktopActivity
                ? (stringsOf(context).mobileFollowingDesktopActivity15d954b3())
                : (stringsOf(
                    context,
                  ).mobileResumeFollowingDesktopActivity5eeb7004()),
            onPressed: () {
              setState(() => _followDesktopActivity = true);
              unawaited(_refreshCatalog(selectActive: true));
            },
            icon: Icon(
              _followDesktopActivity ? Icons.sync : Icons.sync_disabled,
            ),
          ),
          IconButton(
            key: const ValueKey('native-new-session'),
            tooltip: (stringsOf(context).mobileNewConversation4e67ed63()),
            onPressed: _workspaceId == null || _creating
                ? null
                : _createSession,
            icon: _creating
                ? const SizedBox.square(
                    dimension: 18,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  )
                : const Icon(Icons.add_comment_outlined),
          ),
        ],
      ),
      body: Stack(
        children: [
          Positioned.fill(child: _body(workspace)),
          PairedPluginInteractionLayer(connection: widget.connection),
        ],
      ),
    );
  }

  Widget _body(DshNativeWorkspaceSummary? workspace) {
    if (_loading) {
      return const Center(child: CircularProgressIndicator.adaptive());
    }
    if (_failure case final failure?) {
      return Center(
        child: Padding(
          padding: const EdgeInsets.all(28),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(Icons.sync_problem_outlined, size: 42),
              const SizedBox(height: 12),
              Text(failure, textAlign: TextAlign.center),
              const SizedBox(height: 16),
              FilledButton.icon(
                onPressed: () => _refreshCatalog(),
                icon: const Icon(Icons.refresh),
                label: Text(
                  (stringsOf(context).mobileSyncDesktopAgainFd673371()),
                ),
              ),
            ],
          ),
        ),
      );
    }
    if (workspace == null) {
      return Center(
        child: Text(
          (stringsOf(
            context,
          ).mobileNoWorkspacesAreAvailableOnDesktopAb22cd6e()),
        ),
      );
    }
    final sessionId = _sessionId;
    if (sessionId == null) {
      return Center(
        child: FilledButton.icon(
          key: const ValueKey('native-empty-create'),
          onPressed: _createSession,
          icon: const Icon(Icons.add),
          label: Text(
            (stringsOf(
              context,
            ).mobileNewConversationIn6c1f1a27(value1: workspace.title)),
          ),
        ),
      );
    }
    return NativeDshPage(
      key: ValueKey('native-session-$sessionId'),
      session: widget.connection.session,
      workspaceTitle: workspace.title,
      requestedSessionId: sessionId,
      embedded: true,
    );
  }
}

final class _CatalogDrawer extends StatelessWidget {
  const _CatalogDrawer({
    required this.deviceName,
    required this.workspaces,
    required this.sessions,
    required this.selectedWorkspaceId,
    required this.selectedSessionId,
    required this.creating,
    required this.onCreate,
    required this.onWorkspace,
    required this.onSession,
    required this.onRefresh,
  });

  final String deviceName;
  final List<DshNativeWorkspaceSummary> workspaces;
  final List<DshNativeSessionSummary> sessions;
  final String? selectedWorkspaceId;
  final String? selectedSessionId;
  final bool creating;
  final VoidCallback onCreate;
  final ValueChanged<String> onWorkspace;
  final void Function(String workspaceId, String sessionId) onSession;
  final VoidCallback onRefresh;

  @override
  Widget build(BuildContext context) {
    final byId = {for (final item in sessions) item.sessionId: item};
    return Drawer(
      child: SafeArea(
        child: Column(
          children: [
            ListTile(
              leading: const Icon(Icons.desktop_windows_outlined),
              title: Text(deviceName),
              subtitle: Text(
                (stringsOf(context).mobileDesktopConnectedLiveF96eebdd()),
              ),
              trailing: IconButton(
                tooltip: (stringsOf(context).mobileRefresh20ada9a3()),
                onPressed: onRefresh,
                icon: const Icon(Icons.refresh),
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              child: SizedBox(
                width: double.infinity,
                child: FilledButton.icon(
                  key: const ValueKey('native-drawer-create'),
                  onPressed: selectedWorkspaceId == null || creating
                      ? null
                      : onCreate,
                  icon: const Icon(Icons.add_comment_outlined),
                  label: Text(
                    (stringsOf(context).mobileNewConversation4e67ed63()),
                  ),
                ),
              ),
            ),
            const Divider(),
            Expanded(
              child: ListView(
                key: const ValueKey('native-workspace-session-list'),
                children: [
                  for (final workspace in workspaces) ...[
                    ListTile(
                      key: ValueKey(
                        'native-workspace-${workspace.workspaceId}',
                      ),
                      selected: workspace.workspaceId == selectedWorkspaceId,
                      leading: const Icon(Icons.folder_outlined),
                      title: Text(workspace.title),
                      subtitle: Text(
                        (stringsOf(context).mobileConversations03595e53(
                          value1: workspace.sessionIds.length,
                        )),
                      ),
                      onTap: () => onWorkspace(workspace.workspaceId),
                    ),
                    for (final id in workspace.sessionIds)
                      if (byId[id] case final session?)
                        ListTile(
                          key: ValueKey('native-history-$id'),
                          contentPadding: const EdgeInsets.only(
                            left: 48,
                            right: 16,
                          ),
                          selected: id == selectedSessionId,
                          leading: Icon(
                            session.running
                                ? Icons.sync
                                : Icons.chat_bubble_outline,
                            size: 18,
                          ),
                          title: Text(
                            session.title ??
                                (session.blank
                                    ? (stringsOf(
                                        context,
                                      ).mobileNewConversationBcec2f30())
                                    : (stringsOf(
                                        context,
                                      ).mobilePreviousConversation196d79ba())),
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          ),
                          subtitle: Text(_timeLabel(session.updatedAt)),
                          onTap: () => onSession(workspace.workspaceId, id),
                        ),
                  ],
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  String _timeLabel(int value) {
    final time = DateTime.fromMillisecondsSinceEpoch(value).toLocal();
    return '${time.month.toString().padLeft(2, '0')}-${time.day.toString().padLeft(2, '0')} '
        '${time.hour.toString().padLeft(2, '0')}:${time.minute.toString().padLeft(2, '0')}';
  }
}

extension _FirstOrNull<T> on Iterable<T> {
  T? get firstOrNull => isEmpty ? null : first;
}
