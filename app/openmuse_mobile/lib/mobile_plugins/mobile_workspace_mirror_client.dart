import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import '../remote_dsh_page.dart';

/// Mobile adapter for the Desktop-owned opaque workspace mirror protocol.
/// The paired grant is never passed into plugin HTML or JavaScript.
final class MobileWorkspaceMirrorClient {
  MobileWorkspaceMirrorClient(
    this.connection, {
    HttpClient? client,
    this.idleTimeout = const Duration(seconds: 45),
    this.totalTimeout = const Duration(minutes: 5),
  }) : _client = client ?? HttpClient();

  final PairedDesktopConnection connection;
  final HttpClient _client;
  final Duration idleTimeout;
  final Duration totalTimeout;

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
    final payload = await readResource(resourceRef);
    final mediaType = payload['mediaType'] as String;
    final bytes = payload['bytes'] as Uint8List;
    if (bytes.length > 2 * 1024 * 1024) {
      throw const FormatException('文件太大，无法在手机内联预览');
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

  Future<Map<String, Object?>> readResource(
    String resourceRef, {
    void Function(String step, int received, int? total)? onProgress,
  }) async {
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
    final response = await _request(uri, onProgress: onProgress);
    final mediaType = response.$1;
    final bytes = response.$2;
    if (bytes.length > 64 * 1024 * 1024) {
      throw const FormatException('文件超过 64 MiB 限额');
    }
    return {'mediaType': mediaType, 'bytes': bytes};
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

  Future<(String, Uint8List)> _request(
    Uri uri, {
    void Function(String step, int received, int? total)? onProgress,
  }) async {
    var step = '连接 Desktop';
    HttpClientRequest? activeRequest;
    final deadline = Timer(totalTimeout, () {
      activeRequest?.abort(TimeoutException('读取文件总时间超限'));
      _client.close(force: true);
    });
    try {
      onProgress?.call(step, 0, null);
      final request = activeRequest = await _client
          .getUrl(uri)
          .timeout(idleTimeout);
      request.headers.set(
        HttpHeaders.cookieHeader,
        'OpenMuse-Paired=${connection.grantRef}',
      );
      step = '等待 Desktop 读取文件';
      onProgress?.call(step, 0, null);
      final response = await request.close().timeout(idleTimeout);
      if (response.statusCode != HttpStatus.ok) {
        final message = switch (response.statusCode) {
          401 || 403 => 'Desktop 授权已失效，请返回工作台重新连接',
          400 || 404 => '文件引用已失效，请刷新目录后重试',
          502 || 503 || 504 => 'Desktop 中继连接不可用，请重连后重试',
          _ => 'Desktop 请求失败：HTTP ${response.statusCode}',
        };
        throw StateError(message);
      }
      step = '接收 Desktop 文件';
      final total = response.contentLength >= 0 ? response.contentLength : null;
      if (total != null && total > 64 * 1024 * 1024) {
        throw const FormatException('Desktop 响应超过 64 MiB 限额');
      }
      final bytes = BytesBuilder(copy: false);
      onProgress?.call(step, 0, total);
      await for (final chunk in response.timeout(idleTimeout)) {
        if (bytes.length + chunk.length > 64 * 1024 * 1024) {
          throw const FormatException('Desktop 响应超过 64 MiB 限额');
        }
        bytes.add(chunk);
        onProgress?.call(step, bytes.length, total);
      }
      return (
        response.headers.contentType?.toString() ?? 'application/octet-stream',
        bytes.takeBytes(),
      );
    } on TimeoutException {
      activeRequest?.abort();
      throw StateError('$step 超时，请检查网络后重试');
    } on SocketException {
      throw StateError('$step 连接中断，请重连 Desktop 后重试');
    } on HttpException {
      throw StateError('$step 传输中断，请重试');
    } finally {
      deadline.cancel();
    }
  }

  void close() => _client.close(force: true);
}
