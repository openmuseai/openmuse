import 'dart:convert';
import 'dart:io';

import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import '../remote_dsh_page.dart';

final class MobileScheduledTaskClient {
  MobileScheduledTaskClient(this.connection, {HttpClient? client})
    : _client = client ?? HttpClient();

  final PairedDesktopConnection connection;
  final HttpClient _client;

  Future<Map<String, Object?>> request(Map<String, Object?> body) async {
    if (connection.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw StateError('Desktop 授权已过期');
    }
    final target = resolveRemoteDshUri(connection.session);
    final origin = Uri(
      scheme: target.scheme,
      host: target.host,
      port: target.hasPort ? target.port : null,
    );
    final uri = origin.resolve('/openmuse/scheduled-tasks/v1');
    final request = await _client
        .postUrl(uri)
        .timeout(const Duration(seconds: 12));
    request.headers
      ..contentType = ContentType.json
      ..set(HttpHeaders.cookieHeader, 'OpenMuse-Paired=${connection.grantRef}');
    request.add(utf8.encode(jsonEncode(body)));
    final response = await request.close().timeout(const Duration(seconds: 12));
    final bytes = await response
        .fold<List<int>>(<int>[], (out, chunk) {
          if (out.length + chunk.length > 128 * 1024) {
            throw const FormatException('定时任务响应过大');
          }
          return out..addAll(chunk);
        })
        .timeout(const Duration(seconds: 12));
    if (response.statusCode != HttpStatus.ok) {
      throw StateError('Desktop 定时任务请求失败：HTTP ${response.statusCode}');
    }
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map) throw const FormatException('定时任务响应无效');
    return Map<String, Object?>.from(value);
  }

  void close() => _client.close(force: true);
}
