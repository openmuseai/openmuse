import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart'
    show isPrivateNativeTransportHost, ConnectionTrace;
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
    this.connectionBudget = const Duration(seconds: 45),
    HttpClient? client,
    ConnectionTrace? trace,
  }) : trace = trace ?? ConnectionTrace(),
       _client = client ?? HttpClient() {
    _client.connectionTimeout = requestTimeout;
  }

  final ConnectionTrace trace;
  final Uri origin;
  final PairedAccessTokenProvider accessToken;
  final String deviceRef;
  final bool allowInsecureLoopback;
  final bool allowInsecurePrivateNetworkForTesting;
  final Duration requestTimeout;
  final Duration connectionBudget;
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
  }) => trace.measure(
    'pair.connect',
    (_) => _connectSameAccount(
      targetDeviceRef: targetDeviceRef,
      workspaceRef: workspaceRef,
    ),
  );

  Future<PairedDesktopConnection> _connectSameAccount({
    required String targetDeviceRef,
    required String workspaceRef,
  }) async {
    final elapsed = Stopwatch()..start();
    var delaySeconds = 1;
    while (true) {
      try {
        final remaining = connectionBudget - elapsed.elapsed;
        if (remaining <= Duration.zero) _pairTimeout();
        return await _open(
          '/v1/account/open',
          targetDeviceRef: targetDeviceRef,
          workspaceRef: workspaceRef,
        ).timeout(remaining, onTimeout: _pairTimeout);
      } on PairedDesktopFailure catch (error) {
        if (!_retryableRelayFailure(error.code) ||
            elapsed.elapsed + Duration(seconds: delaySeconds) >=
                connectionBudget) {
          rethrow;
        }
      } on SocketException {
        if (elapsed.elapsed + Duration(seconds: delaySeconds) >=
            connectionBudget) {
          rethrow;
        }
      } on HttpException {
        if (elapsed.elapsed + Duration(seconds: delaySeconds) >=
            connectionBudget) {
          rethrow;
        }
      }
      await trace.measure(
        'pair.retry_wait',
        (_) => Future<void>.delayed(Duration(seconds: delaySeconds)),
      );
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
    return trace.measure('pair.${ConnectionTrace.route(path)}', (span) async {
      _validateOrigin();
      final token = await span.child.measure(
        'mobile.access_token',
        (_) => accessToken(),
      );
      if (token == null || token.isEmpty) {
        throw const PairedDesktopFailure('SIGNED_OUT', '请先登录同一个账号。');
      }
      final request = await _client
          .postUrl(origin.resolve(path))
          .timeout(requestTimeout, onTimeout: _pairTimeout);
      span.mark('connected');
      request.headers.set('traceparent', span.traceparent);
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
      span.mark('headers', status: response.statusCode);
      final bytes = await response
          .fold<List<int>>(<int>[], (value, chunk) {
            value.addAll(chunk);
            return value;
          })
          .timeout(requestTimeout, onTimeout: _pairTimeout);
      span.mark('body', bytes: bytes.length);
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
        trace: trace,
        allowInsecurePrivateNetworkForTesting:
            allowInsecurePrivateNetworkForTesting,
      );
    });
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
    final privateNetwork = isPrivateNativeTransportHost(origin.host);
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
}
