import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import '../remote_dsh_page.dart';

/// Mobile adapter for the Desktop-owned opaque workspace mirror protocol.
/// The paired grant is never passed into plugin HTML or JavaScript.
final class MobileWorkspaceMirrorClient {
  MobileWorkspaceMirrorClient(this.connection, {HttpClient? client})
    : _client = client ?? HttpClient();

  final PairedDesktopConnection connection;
  final HttpClient _client;

  Uri get _origin {
    final target = resolveRemoteDshUri(connection.session);
    if (connection.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw StateError('Desktop 授权已过期');
    }
    return Uri(
      scheme: target.scheme,
      host: target.host,
      port: target.hasPort ? target.port : null,
    );
  }

  Future<Map<String, Object?>> mounts() =>
      _get({'operation': 'mounts', 'workspaceRef': connection.workspaceRef});

  Future<Map<String, Object?>> children({
    required String mountRef,
    required String parentRef,
    String? cursor,
  }) => _get({
    'operation': 'children',
    'workspaceRef': connection.workspaceRef,
    'mountRef': mountRef,
    'parentRef': parentRef,
    'limit': '100',
    'cursor': ?cursor,
  });

  Future<Map<String, Object?>> search(String query) {
    final trimmed = query.trim();
    if (trimmed.isEmpty || trimmed.length > 120) {
      throw const FormatException('搜索词长度无效');
    }
    return _get({
      'operation': 'search',
      'workspaceRef': connection.workspaceRef,
      'limit': '100',
      'q': trimmed,
    });
  }

  Future<Map<String, Object?>> preview(String resourceRef) async {
    if (resourceRef.isEmpty || resourceRef.length > 128) {
      throw const FormatException('资源引用无效');
    }
    final uri = _origin
        .resolve('/openmuse/workspace-mirror/resource/v1')
        .replace(
          queryParameters: {
            'workspaceRef': connection.workspaceRef,
            'resourceRef': resourceRef,
          },
        );
    final response = await _request(uri);
    final mediaType = response.$1;
    final bytes = response.$2;
    if (bytes.length > 2 * 1024 * 1024) {
      throw const FormatException('文件太大，无法在手机预览');
    }
    if (mediaType.startsWith('text/') ||
        mediaType.startsWith('application/json')) {
      return {
        'type': 'text',
        'mediaType': mediaType,
        'content': utf8.decode(bytes, allowMalformed: true),
      };
    }
    if (const {
      'image/png',
      'image/jpeg',
      'image/gif',
      'image/webp',
    }.contains(mediaType.split(';').first.trim())) {
      return {
        'type': 'image',
        'mediaType': mediaType,
        'dataUrl': 'data:$mediaType;base64,${base64Encode(bytes)}',
      };
    }
    return {'type': 'unsupported', 'mediaType': mediaType};
  }

  Future<Map<String, Object?>> _get(Map<String, String> query) async {
    final uri = _origin
        .resolve('/openmuse/workspace-mirror/v1')
        .replace(queryParameters: query);
    final (_, bytes) = await _request(uri);
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map) throw const FormatException('目录响应无效');
    return Map<String, Object?>.from(value);
  }

  Future<(String, Uint8List)> _request(Uri uri) async {
    final request = await _client
        .getUrl(uri)
        .timeout(const Duration(seconds: 12));
    request.headers.set(
      HttpHeaders.cookieHeader,
      'OpenMuse-Paired=${connection.grantRef}',
    );
    final response = await request.close().timeout(const Duration(seconds: 12));
    final bytes = <int>[];
    await for (final chunk in response.timeout(const Duration(seconds: 12))) {
      bytes.addAll(chunk);
      if (bytes.length > 2 * 1024 * 1024 + 4096) {
        throw const FormatException('Desktop 响应过大');
      }
    }
    if (response.statusCode != HttpStatus.ok) {
      throw StateError('Desktop 请求失败：HTTP ${response.statusCode}');
    }
    return (
      response.headers.contentType?.toString() ?? 'application/octet-stream',
      Uint8List.fromList(bytes),
    );
  }

  void close() => _client.close(force: true);
}
