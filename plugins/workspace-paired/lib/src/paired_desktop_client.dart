import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'paired_desktop_models.dart';

typedef PairedAccessTokenProvider = Future<String?> Function();

final class PairedDesktopClient {
  PairedDesktopClient({
    required this.origin,
    required this.accessToken,
    required this.deviceRef,
    this.allowInsecureLoopback = false,
    this.allowInsecurePrivateNetworkForTesting = false,
    this.requestTimeout = const Duration(seconds: 15),
    HttpClient? client,
  }) : _client = client ?? HttpClient() {
    _client.connectionTimeout = requestTimeout;
  }

  final Uri origin;
  final PairedAccessTokenProvider accessToken;
  final String deviceRef;
  final bool allowInsecureLoopback;
  final bool allowInsecurePrivateNetworkForTesting;
  final Duration requestTimeout;
  final HttpClient _client;

  Future<PairedDesktopConnection> pair({
    required String pairingCode,
    String? targetDeviceRef,
    String workspaceRef = 'openmuse.local.default',
  }) => _open(
    '/v1/pair/open',
    pairingCode: pairingCode,
    targetDeviceRef: targetDeviceRef,
    workspaceRef: workspaceRef,
  );

  Future<PairedDesktopConnection> connectSameAccount({
    required String targetDeviceRef,
    String workspaceRef = 'openmuse.local.default',
  }) async {
    final deadline = DateTime.now().add(const Duration(seconds: 45));
    var delaySeconds = 1;
    while (true) {
      try {
        return await _open(
          '/v1/account/open',
          targetDeviceRef: targetDeviceRef,
          workspaceRef: workspaceRef,
        );
      } on PairedDesktopFailure catch (error) {
        if (!_retryableRelayFailure(error.code) ||
            DateTime.now().add(Duration(seconds: delaySeconds)).isAfter(
              deadline,
            )) {
          rethrow;
        }
      } on SocketException {
        if (DateTime.now().add(Duration(seconds: delaySeconds)).isAfter(
          deadline,
        )) {
          rethrow;
        }
      } on HttpException {
        if (DateTime.now().add(Duration(seconds: delaySeconds)).isAfter(
          deadline,
        )) {
          rethrow;
        }
      }
      await Future<void>.delayed(Duration(seconds: delaySeconds));
      delaySeconds = delaySeconds >= 8 ? 8 : delaySeconds * 2;
    }
  }

  bool _retryableRelayFailure(String code) => const {
    'RELAY_DETACHED',
    'RELAY_BUSY',
    'RELAY_TIMEOUT',
    'PAIRED_DESKTOP_UNAVAILABLE',
    'PAIR_TIMEOUT',
  }.contains(code);

  Future<PairedDesktopConnection> _open(
    String path, {
    String? pairingCode,
    String? targetDeviceRef,
    required String workspaceRef,
  }) async {
    _validateOrigin();
    final token = await accessToken();
    if (token == null || token.isEmpty) {
      throw const PairedDesktopFailure('SIGNED_OUT', '请先登录同一个账号。');
    }
    final request = await _client
        .postUrl(origin.resolve(path))
        .timeout(requestTimeout, onTimeout: _pairTimeout);
    request.headers
      ..contentType = ContentType.json
      ..set(HttpHeaders.authorizationHeader, 'Bearer $token');
    request.add(
      utf8.encode(
        jsonEncode({
          if (pairingCode != null) 'pairingCode': pairingCode.trim(),
          'deviceRef': deviceRef,
          if (targetDeviceRef != null) 'targetDeviceRef': targetDeviceRef,
          'workspaceRef': workspaceRef,
        }),
      ),
    );
    final response = await request.close().timeout(
      requestTimeout,
      onTimeout: _pairTimeout,
    );
    final bytes = await response
        .fold<List<int>>(<int>[], (value, chunk) {
          value.addAll(chunk);
          return value;
        })
        .timeout(requestTimeout, onTimeout: _pairTimeout);
    final decoded = bytes.isEmpty ? null : jsonDecode(utf8.decode(bytes));
    if (response.statusCode != HttpStatus.ok || decoded is! Map) {
      final body = decoded is Map ? decoded.cast<String, Object?>() : null;
      throw PairedDesktopFailure(
        body?['code'] is String ? body!['code']! as String : 'PAIR_FAILED',
        body?['message'] is String
            ? body!['message']! as String
            : '无法连接 Desktop。',
      );
    }
    return PairedDesktopConnection.fromJson(
      decoded.cast<String, Object?>(),
      allowInsecurePrivateNetworkForTesting:
          allowInsecurePrivateNetworkForTesting,
    );
  }

  void close() => _client.close(force: true);

  Never _pairTimeout() => throw const PairedDesktopFailure(
    'PAIR_TIMEOUT',
    '连接 Desktop 超时，请确认 Desktop 在线且网络可达。',
  );

  void _validateOrigin() {
    final loopback =
        origin.host == '127.0.0.1' ||
        origin.host == 'localhost' ||
        origin.host == '::1' ||
        origin.host == '10.0.2.2';
    final privateNetwork = _isPrivateIpv4(origin.host);
    if (origin.scheme != 'https' &&
        !(allowInsecureLoopback && origin.scheme == 'http' && loopback) &&
        !(allowInsecurePrivateNetworkForTesting &&
            origin.scheme == 'http' &&
            privateNetwork)) {
      throw const PairedDesktopFailure(
        'INVALID_ORIGIN',
        'Paired Desktop 仅允许 HTTPS 或调试 loopback。',
      );
    }
  }

  bool _isPrivateIpv4(String host) {
    final parts = host.split('.').map(int.tryParse).toList(growable: false);
    if (parts.length != 4 || parts.any((value) => value == null)) return false;
    final first = parts[0]!;
    final second = parts[1]!;
    return first == 10 ||
        (first == 192 && second == 168) ||
        (first == 172 && second >= 16 && second <= 31);
  }
}
