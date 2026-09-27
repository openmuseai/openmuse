import 'dart:convert';
import 'dart:io';

import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;

/// Idempotently registers Host-authorized Mounts through the local DSH plugin.
/// The DSH 0.1.7 Remote controller is not the old /api/workspace.* RPC.
final class DshWorkspaceSynchronizer {
  DshWorkspaceSynchronizer({
    required this.context,
    required this.endpoint,
    required this.bridgeToken,
  });

  final OpenMusePluginContext context;
  final Uri? Function() endpoint;
  final String bridgeToken;
  Future<void> _pending = Future<void>.value();
  String? _fingerprint;

  Future<void> sync() {
    _pending = _pending.catchError((Object _) {}).then((_) => _syncNow());
    return _pending;
  }

  Future<void> _syncNow() async {
    final base = endpoint();
    if (base == null) return;
    if (base.scheme != 'http' || base.host != '127.0.0.1') {
      throw const FormatException('DSH endpoint 必须是本机 loopback');
    }
    final raw = await context.executeHostCommand('workspace.snapshot', null);
    if (raw is! Map || raw['mounts'] is! List) {
      throw const FormatException('无效 Host Workspace 快照');
    }
    final mounts = <String>[];
    for (final value in raw['mounts'] as List) {
      if (value is! Map || value['path'] is! String) continue;
      final path = value['path'] as String;
      if (p.isAbsolute(path) && !mounts.contains(path)) mounts.add(path);
    }
    final fingerprint = jsonEncode([base.toString(), mounts]);
    if (_fingerprint == fingerprint) return;
    final client = HttpClient()..connectionTimeout = const Duration(seconds: 3);
    try {
      final uri = base.resolve('/openmuse-bridge/workspaces');
      final request = await client
          .postUrl(uri)
          .timeout(const Duration(seconds: 5));
      request.headers.contentType = ContentType.json;
      request.headers.set('x-openmuse-bridge-token', bridgeToken);
      request.write(jsonEncode({'mounts': mounts}));
      final response = await request.close().timeout(
        const Duration(seconds: 10),
      );
      final rawBody = await utf8.decoder.bind(response).join();
      if (rawBody.isEmpty) {
        throw StateError(
          'DSH 工作区同步响应为空：HTTP ${response.statusCode}, redirect=${response.redirects}',
        );
      }
      final body = jsonDecode(rawBody);
      if (response.statusCode != HttpStatus.ok ||
          body is! Map ||
          body['items'] is! List) {
        throw StateError('DSH 工作区同步失败：HTTP ${response.statusCode} $body');
      }
      _fingerprint = fingerprint;
    } finally {
      client.close(force: true);
    }
  }
}
