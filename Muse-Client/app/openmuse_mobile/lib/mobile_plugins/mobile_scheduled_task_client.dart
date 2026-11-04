import '../mobile_build_locale.dart';

import 'dart:convert';
import 'dart:io';

import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import '../remote_dsh_page.dart';

final class MobileScheduledTaskClient {
  MobileScheduledTaskClient(
    this.connection, {
    HttpClient? client,
    OpenMuseStrings? strings,
  }) : _client = client ?? HttpClient(),
       _strings = strings ?? OpenMuseStrings.fallback;

  final OpenMuseStrings _strings;

  final PairedDesktopConnection connection;
  final HttpClient _client;

  Future<Map<String, Object?>> request(Map<String, Object?> body) async {
    if (connection.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw StateError(
        (_strings.mobileDesktopAuthorizationHasExpiredCff089e8()),
      );
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
            throw FormatException(
              (_strings.mobileTheScheduledTaskResponseIsTooLarge8fb95058()),
            );
          }
          return out..addAll(chunk);
        })
        .timeout(const Duration(seconds: 12));
    if (response.statusCode != HttpStatus.ok) {
      throw StateError(
        (_strings.mobileTheDesktopScheduledTaskRequestFailed04189498(
          value1: response.statusCode,
        )),
      );
    }
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map) {
      throw FormatException(
        (_strings.mobileInvalidScheduledTaskResponse03840268()),
      );
    }
    return Map<String, Object?>.from(value);
  }

  void close() => _client.close(force: true);
}
