import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';

import 'browser_paired_client.dart';
import 'dsh_pane.dart';
import 'paired_workspace_mirror_port.dart';
import 'web_workbench_page.dart';
import 'browser_cloud_documents.dart';
import 'package:openmuse_resource_center/openmuse_resource_center.dart';

final class WebSessionPage extends StatefulWidget {
  const WebSessionPage({
    super.key,
    required this.authentication,
    this.pairedClient,
    this.config,
    this.initialDocument,
    this.openCloudDocuments = BrowserCloudDocuments.open,
  });

  final GoTrueAuthenticationController authentication;
  final BrowserPairedClient? pairedClient;
  final RuntimeConfig? config;
  final ({String workspaceId, String viewId})? initialDocument;
  final BrowserCloudDocumentsOpener openCloudDocuments;

  @override
  State<WebSessionPage> createState() => _WebSessionPageState();
}

final class _WebSessionPageState extends State<WebSessionPage> {
  late final BrowserPairedClient _paired =
      widget.pairedClient ??
      BrowserPairedClient(
        authentication: widget.authentication,
        origin: widget.config?.webServiceUri(ServiceId.relay, Uri.base),
        cloudOrigin: widget.config?.webServiceUri(ServiceId.cloud, Uri.base),
      );
  List<BrowserDesktopDevice>? _desktops;
  BrowserPairedConnection? _connection;
  BrowserDesktopDevice? _activeDesktop;
  PairedWorkspaceMirrorPort? _mirror;
  String? _error;
  bool _busy = false;
  BrowserCloudDocuments? _documents;
  bool _openingDocuments = false;
  Object? _documentError;

  @override
  void initState() {
    super.initState();
    unawaited(_load());
    unawaited(_openDocuments());
  }

  Future<void> _openDocuments() async {
    if (_openingDocuments) return;
    setState(() {
      _openingDocuments = true;
      _documentError = null;
    });
    try {
      final previous = _documents;
      _documents = null;
      await previous?.close();
      final value = await widget.openCloudDocuments(
        authentication: widget.authentication,
        origin:
            widget.config?.webServiceUri(ServiceId.cloud, Uri.base) ??
            Uri.parse(Uri.base.origin),
        presentEditor: (presentation) async {
          final value = _documents;
          if (!mounted || value == null) throw const ResourceFailure('DENIED');
          final link = value.documents.browserLinkForResource(
            presentation.session.record,
          );
          await presentResourceEditor(
            context,
            value.center,
            presentation,
            routeName: link == null ? null : '${link.fragment}/edit',
          );
        },
      );
      if (!mounted) {
        await value.close();
        return;
      }
      setState(() => _documents = value);
      final target = widget.initialDocument;
      if (target != null) {
        await value.openDocument(target.workspaceId, target.viewId);
      }
    } on Object catch (error) {
      if (mounted) setState(() => _documentError = error);
    } finally {
      if (mounted) setState(() => _openingDocuments = false);
    }
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
    unawaited(_documents?.close());
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
    if (widget.initialDocument != null && _documentError != null) {
      return Scaffold(
        appBar: AppBar(
          title: const Text('智能文档'),
          actions: [TextButton(onPressed: _account, child: const Text('账号'))],
        ),
        body: Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text('无法打开文档，请确认当前账号具有访问权限。'),
              TextButton(
                onPressed: _openingDocuments ? null : _openDocuments,
                child: const Text('重试'),
              ),
            ],
          ),
        ),
      );
    }
    final connection = _connection;
    final mirror = _mirror;
    return WebWorkbenchPage(
      workspaceActions: [
        if (_documents case final documents?)
          ...documents.cloud.buildWorkspaceActions(context)
        else
          TextButton.icon(
            key: const ValueKey('web-cloud-documents-load'),
            onPressed: _openingDocuments ? null : _openDocuments,
            icon: const Icon(Icons.description_outlined),
            label: Text(_documentError != null ? '重试协作文档' : '协作文档'),
          ),
      ],
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
              key: ObjectKey(connection),
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
