import 'dart:convert';
import 'dart:io';

/// Trusted local adapter. Only this Host selects the account partition and
/// supplies the bridge token. Remote market bodies cannot override either.
final class CodeNavigationMarket {
  CodeNavigationMarket({
    required this.account,
    required this.ready,
    required this.endpoint,
    required this.token,
  });
  final String? Function() account;
  final Future<void> Function() ready;
  final Uri? Function() endpoint;
  final String Function() token;

  Future<Map<String, Object?>> request(
    String operation,
    String? id, {
    Map<String, Object?> options = const {},
  }) async {
    final partition = account();
    if (partition == null) throw StateError('请先登录国内账号');
    await ready();
    if (account() != partition) throw StateError('Desktop 登录状态已变化');
    final origin = endpoint();
    if (origin == null) throw StateError('资源宿主尚未连接');
    final client = HttpClient()..connectionTimeout = const Duration(seconds: 5);
    try {
      final request = await client.postUrl(
        origin.resolve('/openmuse-native/v1/code-navigation/manage'),
      );
      request.headers
        ..contentType = ContentType.json
        ..set('x-openmuse-bridge-token', token());
      request.write(
        jsonEncode({
          'partition': partition,
          'operation': operation,
          'pluginId': id,
          'options': options,
        }),
      );
      final response = await request.close().timeout(
        const Duration(seconds: 25),
      );
      final bytes = <int>[];
      await for (final chunk in response.timeout(const Duration(seconds: 25))) {
        if (bytes.length + chunk.length > 1024 * 1024) {
          throw StateError('插件管理响应过大');
        }
        bytes.addAll(chunk);
      }
      if (account() != partition) throw StateError('Desktop 登录状态已变化');
      final value = jsonDecode(utf8.decode(bytes));
      if (value is! Map) throw StateError('插件管理响应无效');
      if (response.statusCode != 200) {
        throw StateError('${value['error'] ?? 'MANAGEMENT_FAILED'}');
      }
      return Map<String, Object?>.from(value);
    } finally {
      client.close(force: true);
    }
  }

  Future<List<Map<String, Object?>>> list() async {
    final result = await request('list', null);
    return [
      for (final x in result['plugins'] as List)
        Map<String, Object?>.from(x as Map),
    ];
  }
}
