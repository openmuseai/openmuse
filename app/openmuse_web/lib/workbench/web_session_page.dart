import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';

import 'browser_paired_client.dart';
import 'dsh_pane.dart';
import 'paired_workspace_mirror_port.dart';
import 'web_workbench_page.dart';

final class WebSessionPage extends StatefulWidget {
  const WebSessionPage({
    super.key,
    required this.authentication,
    this.pairedClient,
  });

  final GoTrueAuthenticationController authentication;
  final BrowserPairedClient? pairedClient;

  @override
  State<WebSessionPage> createState() => _WebSessionPageState();
}

final class _WebSessionPageState extends State<WebSessionPage> {
  late final BrowserPairedClient _paired =
      widget.pairedClient ??
      BrowserPairedClient(authentication: widget.authentication);
  List<BrowserDesktopDevice>? _desktops;
  BrowserPairedConnection? _connection;
  BrowserDesktopDevice? _activeDesktop;
  PairedWorkspaceMirrorPort? _mirror;
  String? _error;
  bool _busy = false;

  @override
  void initState() {
    super.initState();
    unawaited(_load());
  }

  Future<void> _load() async {
    if (_busy) return;
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      final desktops = await _paired.listDesktops();
      if (!mounted) return;
      setState(() => _desktops = desktops);
      if (desktops.length == 1) await _connect(desktops.single);
    } on Object catch (error) {
      if (mounted) setState(() => _error = error.toString());
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  Future<void> _connect(BrowserDesktopDevice desktop) async {
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      final connection = await _paired.connect(desktop);
      if (!mounted) return;
      _mirror?.close();
      setState(() {
        _connection = connection;
        _activeDesktop = desktop;
        _mirror = PairedWorkspaceMirrorPort();
      });
    } on Object catch (error) {
      if (mounted) setState(() => _error = error.toString());
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  void dispose() {
    _paired.close();
    _mirror?.close();
    super.dispose();
  }

  void _account() => showDialog<void>(
    context: context,
    builder: (context) => AlertDialog(
      title: const Text('OpenMuse 账号'),
      content: Text(widget.authentication.snapshot.identity?.email ?? ''),
      actions: [
        TextButton(
          onPressed: () {
            Navigator.pop(context);
            _mirror?.close();
            setState(() {
              _connection = null;
              _activeDesktop = null;
              _mirror = null;
            });
            unawaited(_load());
          },
          child: const Text('切换 Desktop'),
        ),
        TextButton(
          onPressed: () {
            Navigator.pop(context);
            unawaited(widget.authentication.signOut());
          },
          child: const Text('退出登录'),
        ),
      ],
    ),
  );

  @override
  Widget build(BuildContext context) {
    final connection = _connection;
    final mirror = _mirror;
    return WebWorkbenchPage(
      desktopRef: connection?.desktopRef,
      workspaceRef: connection?.workspaceRef,
      mirrorPort: mirror,
      onSettings: _account,
      disconnectedWorkspace: _connectionPanel(),
      workspaceStatus: connection == null ? null : _connectionStatus(),
      onReconnect: () => unawaited(_reconnect()),
      dshPaneBuilder: (_) => connection == null
          ? Center(
              child: Padding(
                padding: const EdgeInsets.all(12),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    const Text('连接 Desktop 后使用 DSH'),
                    TextButton(
                      onPressed: _busy ? null : () => unawaited(_load()),
                      child: const Text('重试连接'),
                    ),
                  ],
                ),
              ),
            )
          : OpenMuseDshPane(
              key: ValueKey(connection.bootstrapPath),
              bootstrapPath: connection.bootstrapPath,
            ),
    );
  }

  Future<void> _reconnect() async {
    final desktop = _activeDesktop;
    if (desktop == null) {
      await _load();
    } else {
      await _connect(desktop);
    }
  }

  Widget _connectionStatus() => Padding(
    padding: const EdgeInsets.symmetric(horizontal: 4, vertical: 4),
    child: Column(
      children: [
        Row(
          children: [
            const Icon(Icons.desktop_windows_outlined, size: 16),
            const SizedBox(width: 6),
            Expanded(
              child: Text(
                _activeDesktop?.name ?? 'Desktop',
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            ),
            IconButton(
              tooltip: '重连 Desktop',
              icon: const Icon(Icons.refresh, size: 18),
              onPressed: _busy ? null : () => unawaited(_reconnect()),
            ),
          ],
        ),
        if (_error != null)
          Text(
            _error!,
            style: TextStyle(color: Theme.of(context).colorScheme.error),
          ),
      ],
    ),
  );

  Widget _connectionPanel() => ListView(
    padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 18),
    children: [
      const Text(
        'Desktop Workspace',
        style: TextStyle(fontWeight: FontWeight.w600),
      ),
      const SizedBox(height: 8),
      Text(
        widget.authentication.snapshot.identity?.email ?? '',
        overflow: TextOverflow.ellipsis,
        style: Theme.of(context).textTheme.bodySmall,
      ),
      const SizedBox(height: 14),
      if (_busy) const Center(child: CircularProgressIndicator()),
      if (_error != null) ...[
        Text(
          _error!,
          style: TextStyle(color: Theme.of(context).colorScheme.error),
        ),
        const SizedBox(height: 10),
      ],
      if (_desktops?.isEmpty == true)
        const Text('没有在线的 Desktop。请确认 Desktop 已登录同一账号。'),
      if (_desktops case final desktops?)
        for (final desktop in desktops)
          ListTile(
            dense: true,
            contentPadding: EdgeInsets.zero,
            title: Text(desktop.name, overflow: TextOverflow.ellipsis),
            subtitle: Text(desktop.ref, overflow: TextOverflow.ellipsis),
            onTap: _busy ? null : () => unawaited(_connect(desktop)),
          ),
      const SizedBox(height: 12),
      OutlinedButton(
        onPressed: _busy ? null : () => unawaited(_load()),
        child: const Text('刷新并重连'),
      ),
    ],
  );
}
