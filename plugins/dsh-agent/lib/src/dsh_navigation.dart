import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'dsh_sidecar.dart';

/// DSH owns the catalog and session selection. Host receives a Widget only.
final class DshNavigation extends StatefulWidget {
  const DshNavigation({
    super.key,
    required this.supervisor,
    required this.activeMount,
    required this.resolveWorkspace,
  });
  final DshSidecarSupervisor supervisor;
  final ValueListenable<String?> activeMount;
  final Future<String?> Function(String path) resolveWorkspace;
  @override
  State<DshNavigation> createState() => _DshNavigationState();
}

final class _DshNavigationState extends State<DshNavigation> {
  DshNativeGatewayClient? _client;
  List<DshNativeSessionSummary> _sessions = [];
  String? _workspaceId;
  bool _loading = false, _failed = false;
  int _generation = 0;
  bool get _allowed => widget.supervisor.aiSharingAllowed;
  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;
  @override
  void initState() {
    super.initState();
    widget.activeMount.addListener(_changed);
    widget.supervisor.addListener(_supervisorChanged);
    unawaited(_load());
  }

  void _supervisorChanged() {
    if (!_allowed) {
      _changed();
    } else if (widget.supervisor.state == DshSidecarState.ready &&
        !_loading &&
        _client == null) {
      unawaited(_load());
    }
  }

  void _changed() {
    _generation++;
    _client?.close();
    _client = null;
    _sessions = [];
    _workspaceId = null;
    _loading = false;
    if (mounted) setState(() {});
    unawaited(_load());
  }

  Future<void> _load() async {
    if (_loading || !_allowed || widget.activeMount.value == null) return;
    final generation = ++_generation;
    final mount = widget.activeMount.value;
    setState(() {
      _loading = true;
      _failed = false;
    });
    try {
      await widget.supervisor.ensureStarted();
      if (!mounted || generation != _generation || !_allowed) return;
      final endpoint = widget.supervisor.endpoint;
      if (endpoint == null) throw StateError('DSH_UNAVAILABLE');
      _client?.close();
      final client = DshNativeGatewayClient(
        origin: endpoint,
        bootstrapPath: '',
        allowInsecureLoopback: true,
        requestTimeout: const Duration(seconds: 10),
        headers: {'x-openmuse-bridge-token': widget.supervisor.bridgeToken},
      );
      _client = client;
      await client.initialize();
      final sessions = await client.listSessions();
      final workspaceId = await widget.resolveWorkspace(mount!);
      final workspaces = await client.listWorkspaces();
      final members =
          workspaces
              .where((w) => w.workspaceId == workspaceId)
              .firstOrNull
              ?.sessionIds
              .toSet() ??
          <String>{};
      if (!mounted ||
          generation != _generation ||
          widget.activeMount.value != mount ||
          !_allowed)
        return;
      setState(() {
        _sessions =
            sessions
                .where((s) => s.cwd == mount || members.contains(s.sessionId))
                .toList()
              ..sort((a, b) => b.updatedAt.compareTo(a.updatedAt));
        _workspaceId = workspaceId;
      });
    } catch (_) {
      if (mounted && generation == _generation) setState(() => _failed = true);
    } finally {
      if (mounted && generation == _generation)
        setState(() => _loading = false);
    }
  }

  Future<void> _newSession() async {
    final client = _client;
    final mount = widget.activeMount.value;
    final workspaceId = _workspaceId;
    if (client == null || workspaceId == null || !_allowed) return;
    final generation = _generation;
    var created = false;
    setState(() => _loading = true);
    try {
      final session = await client.createSession(workspaceId: workspaceId);
      if (!mounted ||
          generation != _generation ||
          widget.activeMount.value != mount ||
          !_allowed)
        return;
      widget.supervisor.requestedSession.value = session.sessionId;
      created = true;
    } catch (_) {
      if (mounted && generation == _generation) setState(() => _failed = true);
    } finally {
      if (mounted && generation == _generation) {
        setState(() => _loading = false);
        if (created) unawaited(_load());
      }
    }
  }

  @override
  void dispose() {
    _generation++;
    widget.activeMount.removeListener(_changed);
    widget.supervisor.removeListener(_supervisorChanged);
    _client?.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Column(
    crossAxisAlignment: CrossAxisAlignment.stretch,
    children: [
      Padding(
        padding: const EdgeInsets.fromLTRB(12, 4, 12, 8),
        child: OutlinedButton.icon(
          key: const ValueKey('agent-new-session'),
          onPressed: _loading || !_allowed || _workspaceId == null
              ? null
              : _newSession,
          icon: const Icon(Icons.add, size: 16),
          label: Text(tr('新建会话', 'New session')),
        ),
      ),
      if (_loading) const LinearProgressIndicator(minHeight: 2),
      if (!_allowed)
        Padding(
          padding: const EdgeInsets.all(16),
          child: Text(
            tr(
              '请在 Agent 主区确认 AI 数据共享。',
              'Confirm AI data sharing in the Agent pane.',
            ),
          ),
        ),
      if (_failed)
        Padding(
          padding: const EdgeInsets.all(12),
          child: Column(
            children: [
              Text(tr('无法加载会话', 'Unable to load sessions')),
              TextButton(
                onPressed: _loading ? null : _load,
                child: Text(tr('重试', 'Retry')),
              ),
            ],
          ),
        ),
      Padding(
        padding: const EdgeInsets.all(12),
        child: Text(
          tr('当前工作区会话', 'WORKSPACE SESSIONS'),
          style: const TextStyle(
            fontSize: 10,
            fontWeight: FontWeight.w600,
            letterSpacing: .5,
          ),
        ),
      ),
      Expanded(
        child: ValueListenableBuilder(
          valueListenable: widget.supervisor.requestedSession,
          builder: (context, selected, _) => ListView(
            children: [
              if (_sessions.isEmpty && !_loading && !_failed && _allowed)
                Padding(
                  padding: const EdgeInsets.all(16),
                  child: Text(
                    tr('暂无会话', 'No conversations'),
                    style: const TextStyle(fontSize: 12),
                  ),
                ),
              for (final session in _sessions)
                ListTile(
                  key: ValueKey('agent-session.${session.sessionId}'),
                  dense: true,
                  selected: selected == session.sessionId,
                  leading: Icon(
                    session.running ? Icons.circle : Icons.chat_bubble_outline,
                    size: session.running ? 8 : 16,
                  ),
                  title: Text(
                    session.title ?? tr('新会话', 'New conversation'),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(fontSize: 12),
                  ),
                  onTap: () => widget.supervisor.requestedSession.value =
                      session.sessionId,
                ),
            ],
          ),
        ),
      ),
      TextButton.icon(
        onPressed: _loading || !_allowed ? null : _load,
        icon: const Icon(Icons.refresh, size: 14),
        label: Text(tr('刷新会话', 'Refresh sessions')),
      ),
    ],
  );
}
