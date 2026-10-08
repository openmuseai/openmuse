import 'dart:convert';
import 'dart:math';

import 'package:http/browser_client.dart';
import 'package:http/http.dart' as http;
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';

final class BrowserDesktopDevice {
  const BrowserDesktopDevice({required this.ref, required this.name});
  final String ref;
  final String name;
}

final class BrowserPairedConnection {
  const BrowserPairedConnection({
    required this.desktopRef,
    required this.workspaceRef,
    required this.bootstrapPath,
  });
  final String desktopRef;
  final String workspaceRef;
  final String bootstrapPath;
}

final class BrowserPairedFailure implements Exception {
  const BrowserPairedFailure(this.message);
  final String message;

  @override
  String toString() => message;
}

/// Same-origin Edge API. Bearer authorizes device discovery and grant creation;
/// the granted DSH/Workspace requests use the paired HttpOnly cookie.
final class BrowserPairedClient {
  BrowserPairedClient({
    required this.authentication,
    Uri? origin,
    http.Client? client,
  }) : origin =
           origin ??
           (Uri.base.origin.isEmpty ? Uri.base : Uri.parse(Uri.base.origin)),
       _client = client ?? (BrowserClient()..withCredentials = true);

  final GoTrueAuthenticationController authentication;
  final Uri origin;
  final http.Client _client;
  final String requesterRef = _newRequesterRef();

  static String _newRequesterRef() {
    final random = Random.secure();
    return 'web.${List.generate(20, (_) => random.nextInt(256).toRadixString(16).padLeft(2, '0')).join()}';
  }

  Future<List<BrowserDesktopDevice>> listDesktops() async {
    final response = await _request('GET', '/api/muse/devices');
    final body = _json(response);
    if (body['code'] != 0 || body['data'] is! List) {
      throw const FormatException('设备目录响应无效');
    }
    return [
      for (final item in body['data'] as List)
        if (item is Map<String, dynamic> &&
            item['deviceKind'] == 'desktop' &&
            item['online'] == true &&
            item['transportOrigin'] is String &&
            item['capabilities'] is List &&
            (item['capabilities'] as List).contains(
              'paired-desktop.transport',
            ) &&
            item['deviceId'] is String)
          BrowserDesktopDevice(
            ref: item['deviceId'] as String,
            name: item['displayName'] is String
                ? item['displayName'] as String
                : 'OpenMuse Desktop',
          ),
    ];
  }

  Future<BrowserPairedConnection> connect(BrowserDesktopDevice desktop) async {
    final response = await _request(
      'POST',
      '/v1/account/open',
      body: {
        'deviceRef': requesterRef,
        'targetDeviceRef': desktop.ref,
        'workspaceRef': 'openmuse.local.default',
      },
    );
    final body = _json(response);
    final session = body['session'];
    if (body['deviceRef'] != desktop.ref ||
        body['workspaceRef'] is! String ||
        session is! Map<String, dynamic> ||
        session['path'] is! String ||
        !RegExp(r'^/u/[a-fA-F0-9]{64}$').hasMatch(session['path'] as String)) {
      throw const FormatException('Desktop 配对响应无效');
    }
    return BrowserPairedConnection(
      desktopRef: desktop.ref,
      workspaceRef: body['workspaceRef'] as String,
      bootstrapPath: session['path'] as String,
    );
  }

  Future<http.Response> _request(
    String method,
    String path, {
    Map<String, Object?>? body,
  }) async {
    final token = await authentication.accessToken();
    if (token == null) throw const AuthFailure.sessionExpired();
    final request = http.Request(method, origin.resolve(path))
      ..headers.addAll({
        'Accept': 'application/json',
        'Authorization': 'Bearer $token',
        if (body != null) 'Content-Type': 'application/json',
      });
    if (body != null) request.body = jsonEncode(body);
    final response = await _client
        .send(request)
        .then(http.Response.fromStream)
        .timeout(const Duration(seconds: 45));
    if (response.statusCode < 200 || response.statusCode >= 300) {
      final error = _tryJson(response);
      if (response.statusCode == 401) {
        throw const BrowserPairedFailure('设备连接授权已失效，请重新登录后重试。');
      }
      throw BrowserPairedFailure(
        error?['message'] is String
            ? error!['message'] as String
            : '${path == '/api/muse/devices' ? '设备目录' : 'Desktop 连接'}请求失败：HTTP ${response.statusCode}',
      );
    }
    return response;
  }

  Map<String, dynamic> _json(http.Response response) =>
      _tryJson(response) ?? (throw const FormatException('服务响应无效'));

  Map<String, dynamic>? _tryJson(http.Response response) {
    try {
      final decoded = jsonDecode(response.body);
      return decoded is Map<String, dynamic> ? decoded : null;
    } on FormatException {
      return null;
    }
  }

  void close() => _client.close();
}
