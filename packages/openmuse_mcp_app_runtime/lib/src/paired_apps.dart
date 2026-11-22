import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'remote_app.dart';

/// Public paired gateway transport; no backend endpoint or backend secret is
/// accepted from App HTML. The existing account/device grant is authoritative.
Future<List<RemoteMcpApp>> loadPairedMcpApps({
  required Uri origin,
  required String grant,
  required String Function() scope,
  required bool Function() sessionCurrent,
}) async {
  if (origin.scheme != 'https' ||
      origin.userInfo.isNotEmpty ||
      origin.hasQuery ||
      origin.hasFragment ||
      grant.contains(RegExp(r'[\r\n;]'))) {
    throw const FormatException('Trusted paired HTTPS origin required');
  }
  final account = scope();
  Future<Map<String, Object?>> call(
    String method,
    Map<String, Object?> params, {
    String? appId,
  }) async {
    if (!sessionCurrent() || scope() != account)
      throw StateError('APP_SESSION_REVOKED');
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 15);
    try {
      final request = await client.postUrl(
        origin.replace(
          path: '/openmuse/mcp-apps/v1',
          query: null,
          fragment: null,
        ),
      );
      request.followRedirects = false;
      request.cookies.add(Cookie('OpenMuse-Paired', grant));
      request.headers.contentType = ContentType.json;
      request.write(
        jsonEncode({
          'method': method,
          if (appId != null) 'appId': appId,
          'params': params,
        }),
      );
      final response = await request.close().timeout(
        const Duration(seconds: 90),
      );
      if (response.statusCode != 200)
        throw HttpException('PAIRED_APP_HTTP_${response.statusCode}');
      final bytes = await response
          .fold<List<int>>([], (value, chunk) {
            if (value.length + chunk.length > 12 * 1024 * 1024)
              throw StateError('APP_RESULT_TOO_LARGE');
            value.addAll(chunk);
            return value;
          })
          .timeout(const Duration(seconds: 90));
      if (!sessionCurrent() || scope() != account)
        throw StateError('APP_SESSION_REVOKED');
      final decoded = jsonDecode(utf8.decode(bytes));
      if (decoded is! Map)
        throw const FormatException('Invalid paired App response');
      return Map<String, Object?>.from(decoded);
    } finally {
      client.close(force: true);
    }
  }

  final listed = await call('apps/list', {}), rows = listed['apps'];
  if (rows is! List || rows.length > 100)
    throw const FormatException('Invalid paired App list');
  final apps = <RemoteMcpApp>[];
  for (final row in rows) {
    final definition = Map<String, dynamic>.from(row as Map),
        id = definition['id'];
    if (apps.any((app) => app.id == id))
      throw const FormatException('Duplicate paired App');
    apps.add(
      RemoteMcpApp.paired(
        definition: definition,
        scope: scope,
        request: (method, params) => call(method, params, appId: id as String),
      ),
    );
  }
  return apps;
}
