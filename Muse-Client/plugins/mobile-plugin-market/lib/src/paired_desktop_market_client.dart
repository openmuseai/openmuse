import 'dart:convert';
import 'dart:io';

/// Uses the existing short-lived paired Desktop grant. Plugin IDs and actions
/// are validated again by the Desktop gateway before reaching its installer.
final class PairedDesktopMarketClient {
  const PairedDesktopMarketClient({
    required this.origin,
    required this.grantRef,
    required this.expiresAtMs,
  });

  final Uri origin;
  final String grantRef;
  final int expiresAtMs;

  Future<Map<String, Object?>> request(
    String operation, {
    String? pluginId,
  }) async {
    if (DateTime.now().millisecondsSinceEpoch >= expiresAtMs) {
      throw StateError('Desktop 配对已过期，请重新连接');
    }
    if (!const {
          'list',
          'plan',
          'install',
          'enable',
          'disable',
        }.contains(operation) ||
        (operation != 'list' && (pluginId == null || pluginId.isEmpty))) {
      throw const FormatException('无效插件市场操作');
    }
    if (origin.scheme != 'https' && origin.scheme != 'http') {
      throw const FormatException('Desktop 地址无效');
    }
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 10);
    try {
      final uri = origin.replace(
        path: '/openmuse/plugin-market/v1',
        query: null,
        fragment: null,
      );
      final request = await client
          .postUrl(uri)
          .timeout(const Duration(seconds: 10));
      request.headers.contentType = ContentType.json;
      request.cookies.add(Cookie('OpenMuse-Paired', grantRef));
      request.write(
        jsonEncode({
          'operation': operation,
          if (pluginId != null) 'pluginId': pluginId,
        }),
      );
      final response = await request.close().timeout(
        const Duration(minutes: 5),
      );
      final bytes = await response.fold<List<int>>(<int>[], (value, chunk) {
        if (value.length + chunk.length > 1024 * 1024) {
          throw const FormatException('Desktop 市场响应过大');
        }
        value.addAll(chunk);
        return value;
      });
      Object? decoded;
      try {
        decoded = jsonDecode(utf8.decode(bytes));
      } on FormatException {
        throw StateError('当前 Desktop 尚未提供插件市场接口，请更新并重启 Desktop');
      }
      if (decoded is! Map) throw const FormatException('Desktop 市场响应无效');
      final result = Map<String, Object?>.from(decoded);
      if (response.statusCode != HttpStatus.ok) {
        throw StateError('${result['message'] ?? 'Desktop 市场操作失败'}');
      }
      return result;
    } finally {
      client.close(force: true);
    }
  }
}
