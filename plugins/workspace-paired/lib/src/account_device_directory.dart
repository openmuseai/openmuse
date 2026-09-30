import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

typedef AccountDeviceAccessTokenProvider = Future<String?> Function();

enum AccountDeviceKind { desktop, mobile, web }

@immutable
final class AccountDevice {
  const AccountDevice({
    required this.deviceRef,
    required this.displayName,
    required this.platform,
    required this.kind,
    required this.capabilities,
    required this.lastSeenAt,
    required this.online,
    this.transportOrigin,
  });

  factory AccountDevice.fromJson(Map<String, Object?> json) {
    final deviceRef = json['deviceId'];
    final displayName = json['displayName'];
    final platform = json['platform'];
    final kindName = json['deviceKind'];
    final capabilities = json['capabilities'];
    final lastSeenAt = json['lastSeenAt'];
    final online = json['online'];
    final transportOrigin = json['transportOrigin'];
    final kind = AccountDeviceKind.values.where(
      (value) => value.name == kindName,
    );
    if (deviceRef is! String ||
        deviceRef.isEmpty ||
        displayName is! String ||
        displayName.isEmpty ||
        platform is! String ||
        platform.isEmpty ||
        kind.length != 1 ||
        capabilities is! List ||
        capabilities.any((value) => value is! String) ||
        lastSeenAt is! num ||
        online is! bool ||
        (transportOrigin != null && transportOrigin is! String)) {
      throw const FormatException('invalid account device');
    }
    return AccountDevice(
      deviceRef: deviceRef,
      displayName: displayName,
      platform: platform,
      kind: kind.single,
      capabilities: capabilities.cast<String>().toSet(),
      lastSeenAt: DateTime.fromMillisecondsSinceEpoch(
        lastSeenAt.toInt(),
        isUtc: true,
      ),
      online: online,
      transportOrigin: transportOrigin == null
          ? null
          : Uri.tryParse(transportOrigin as String),
    );
  }

  final String deviceRef;
  final String displayName;
  final String platform;
  final AccountDeviceKind kind;
  final Set<String> capabilities;
  final DateTime lastSeenAt;
  final bool online;
  final Uri? transportOrigin;

  bool get supportsPairedDesktop =>
      kind == AccountDeviceKind.desktop &&
      capabilities.contains('paired-desktop.transport') &&
      transportOrigin != null;
}

@immutable
final class AccountDeviceRegistration {
  const AccountDeviceRegistration({
    required this.deviceRef,
    required this.displayName,
    required this.platform,
    required this.kind,
    this.capabilities = const {},
    this.transportOrigin,
  });

  final String deviceRef;
  final String displayName;
  final String platform;
  final AccountDeviceKind kind;
  final Set<String> capabilities;
  final Uri? transportOrigin;

  Map<String, Object?> toJson() => {
    'deviceId': deviceRef,
    'displayName': displayName,
    'platform': platform,
    'deviceKind': kind.name,
    'capabilities': capabilities.toList(growable: false)..sort(),
    if (transportOrigin case final value?) 'transportOrigin': value.toString(),
  };
}

final class AccountDeviceDirectoryFailure implements Exception {
  const AccountDeviceDirectoryFailure(this.code, this.safeMessage);
  final String code;
  final String safeMessage;

  @override
  String toString() => safeMessage;
}

final class AccountDeviceDirectoryClient {
  AccountDeviceDirectoryClient({
    required this.cloudOrigin,
    required this.accessToken,
    this.allowInsecureLoopback = false,
    this.requestTimeout = const Duration(seconds: 15),
    HttpClient? client,
  }) : _client = client ?? HttpClient() {
    _validateOrigin();
    _client.connectionTimeout = requestTimeout;
  }

  final Uri cloudOrigin;
  final AccountDeviceAccessTokenProvider accessToken;
  final bool allowInsecureLoopback;
  final Duration requestTimeout;
  final HttpClient _client;

  Future<AccountDevice> register(AccountDeviceRegistration registration) async {
    final data = await _envelope(
      'POST',
      '/api/muse/devices',
      body: registration.toJson(),
    );
    return AccountDevice.fromJson(_object(data));
  }

  Future<AccountDevice> heartbeat(String deviceRef) async {
    final data = await _envelope(
      'POST',
      '/api/muse/devices/heartbeat',
      body: {'deviceId': deviceRef},
    );
    return AccountDevice.fromJson(_object(data));
  }

  Future<List<AccountDevice>> list() async {
    final data = await _envelope('GET', '/api/muse/devices');
    if (data is! List) {
      throw const AccountDeviceDirectoryFailure(
        'INVALID_RESPONSE',
        '设备列表响应无效。',
      );
    }
    return data
        .map((value) => AccountDevice.fromJson(_object(value)))
        .toList(growable: false);
  }

  Future<WebSocket> connectEvents() async {
    final token = await accessToken();
    if (token == null || token.isEmpty) {
      throw const AccountDeviceDirectoryFailure('SIGNED_OUT', '请先登录账号。');
    }
    final scheme = cloudOrigin.scheme == 'https' ? 'wss' : 'ws';
    final uri = cloudOrigin.replace(
      scheme: scheme,
      path: '/api/muse/devices/events',
      query: null,
      fragment: null,
    );
    try {
      return await WebSocket.connect(
        uri.toString(),
        headers: {HttpHeaders.authorizationHeader: 'Bearer $token'},
      ).timeout(requestTimeout, onTimeout: _timeout);
    } on AccountDeviceDirectoryFailure {
      rethrow;
    } on Object {
      throw const AccountDeviceDirectoryFailure(
        'EVENT_STREAM_UNAVAILABLE',
        '设备实时通道暂时不可用。',
      );
    }
  }

  Future<void> revoke(String deviceRef) async {
    await _envelope(
      'POST',
      '/api/muse/devices/${Uri.encodeComponent(deviceRef)}/revoke',
    );
  }

  Future<Object?> _envelope(
    String method,
    String path, {
    Map<String, Object?>? body,
  }) async {
    final token = await accessToken();
    if (token == null || token.isEmpty) {
      throw const AccountDeviceDirectoryFailure('SIGNED_OUT', '请先登录账号。');
    }
    final request = await _client
        .openUrl(method, cloudOrigin.resolve(path))
        .timeout(requestTimeout, onTimeout: _timeout);
    request.headers
      ..set(HttpHeaders.authorizationHeader, 'Bearer $token')
      ..set(HttpHeaders.acceptHeader, 'application/json');
    if (body != null) {
      request.headers.contentType = ContentType.json;
      request.write(jsonEncode(body));
    }
    final response = await request.close().timeout(
      requestTimeout,
      onTimeout: _timeout,
    );
    final bytes = await response
        .fold<List<int>>(<int>[], (all, chunk) => all..addAll(chunk))
        .timeout(requestTimeout, onTimeout: _timeout);
    Object? decoded;
    try {
      decoded = bytes.isEmpty ? null : jsonDecode(utf8.decode(bytes));
    } on FormatException {
      throw const AccountDeviceDirectoryFailure(
        'INVALID_RESPONSE',
        '设备服务响应无效。',
      );
    }
    if (response.statusCode < 200 || response.statusCode >= 300) {
      throw AccountDeviceDirectoryFailure(
        response.statusCode == 401 ? 'SIGNED_OUT' : 'UNAVAILABLE',
        response.statusCode == 401 ? '登录已过期，请重新登录。' : '设备服务暂时不可用。',
      );
    }
    final envelope = _object(decoded);
    final code = envelope['code'];
    if (code is! num || code.toInt() != 0) {
      throw const AccountDeviceDirectoryFailure('BUSINESS_ERROR', '设备服务拒绝了请求。');
    }
    return envelope['data'];
  }

  Map<String, Object?> _object(Object? value) {
    if (value is! Map) {
      throw const AccountDeviceDirectoryFailure(
        'INVALID_RESPONSE',
        '设备服务响应无效。',
      );
    }
    return value.cast<String, Object?>();
  }

  Never _timeout() =>
      throw const AccountDeviceDirectoryFailure('TIMEOUT', '连接设备服务超时。');

  void _validateOrigin() {
    final loopback = const {
      '127.0.0.1',
      'localhost',
      '::1',
      '10.0.2.2',
    }.contains(cloudOrigin.host);
    if (cloudOrigin.host.isEmpty ||
        cloudOrigin.hasQuery ||
        cloudOrigin.hasFragment ||
        (cloudOrigin.scheme != 'https' &&
            !(allowInsecureLoopback &&
                cloudOrigin.scheme == 'http' &&
                loopback))) {
      throw ArgumentError('invalid device directory origin');
    }
  }

  void close() => _client.close(force: true);
}

@immutable
final class AccountDeviceDirectorySnapshot {
  const AccountDeviceDirectorySnapshot({
    this.loading = false,
    this.registered = false,
    this.realtimeConnected = false,
    this.reconnecting = false,
    this.devices = const [],
    this.failureMessage,
  });

  final bool loading;
  final bool registered;
  final bool realtimeConnected;
  final bool reconnecting;
  final List<AccountDevice> devices;
  final String? failureMessage;
}

final class AccountDeviceDirectoryController extends ChangeNotifier {
  AccountDeviceDirectoryController({
    required this.authentication,
    required this.client,
    required this.registration,
    this.heartbeatInterval = const Duration(seconds: 20),
    this.maxReconnectDelay = const Duration(seconds: 30),
  });

  final OpenMuseAuthenticationController authentication;
  final AccountDeviceDirectoryClient client;
  final AccountDeviceRegistration Function() registration;
  final Duration heartbeatInterval;
  final Duration maxReconnectDelay;
  AccountDeviceDirectorySnapshot _snapshot =
      const AccountDeviceDirectorySnapshot();
  Timer? _heartbeat;
  Timer? _retry;
  WebSocket? _events;
  StreamSubscription<Object?>? _eventSubscription;
  bool _active = false;
  bool _operationInFlight = false;
  int _identityGeneration = 0;
  int _consecutiveFailures = 0;
  final Random _jitter = Random();

  AccountDeviceDirectorySnapshot get snapshot => _snapshot;

  Future<void> activate() async {
    if (_active) return;
    _active = true;
    authentication.addListener(_authenticationChanged);
    if (authentication.snapshot.isAuthenticated) {
      await _registerAndRefresh(_identityGeneration);
    }
  }

  void _authenticationChanged() {
    _identityGeneration++;
    if (!authentication.snapshot.isAuthenticated) {
      _stopTransports();
      _publish(const AccountDeviceDirectorySnapshot());
      return;
    }
    unawaited(_registerAndRefresh(_identityGeneration));
  }

  Future<void> refresh() async {
    if (!authentication.snapshot.isAuthenticated || _operationInFlight) return;
    final generation = _identityGeneration;
    _operationInFlight = true;
    _publish(
      AccountDeviceDirectorySnapshot(
        loading: true,
        registered: _snapshot.registered,
        realtimeConnected: _snapshot.realtimeConnected,
        reconnecting: _snapshot.reconnecting,
        devices: _snapshot.devices,
      ),
    );
    try {
      final devices = await client.list();
      if (!_isCurrent(generation)) return;
      _publish(
        AccountDeviceDirectorySnapshot(
          registered: _snapshot.registered,
          realtimeConnected: _snapshot.realtimeConnected,
          devices: devices,
        ),
      );
    } on AccountDeviceDirectoryFailure catch (error) {
      if (!_isCurrent(generation)) return;
      debugPrint('OpenMuse device refresh: ${error.code}');
      _publish(
        AccountDeviceDirectorySnapshot(
          registered: _snapshot.registered,
          realtimeConnected: _snapshot.realtimeConnected,
          reconnecting: _snapshot.reconnecting,
          devices: _snapshot.devices,
          failureMessage: error.safeMessage,
        ),
      );
    } finally {
      _operationInFlight = false;
    }
  }

  Future<void> reconcile() async {
    if (!_active || !authentication.snapshot.isAuthenticated) return;
    _retry?.cancel();
    await _registerAndRefresh(_identityGeneration);
  }

  Future<void> _registerAndRefresh(int generation) async {
    if (_operationInFlight) return;
    _operationInFlight = true;
    _publish(
      AccountDeviceDirectorySnapshot(
        loading: true,
        registered: _snapshot.registered,
        realtimeConnected: _snapshot.realtimeConnected,
        reconnecting: _snapshot.reconnecting,
        devices: _snapshot.devices,
      ),
    );
    try {
      await client.register(registration());
      final devices = await client.list();
      if (!_isCurrent(generation)) return;
      _consecutiveFailures = 0;
      _retry?.cancel();
      _publish(
        AccountDeviceDirectorySnapshot(
          registered: true,
          realtimeConnected: _snapshot.realtimeConnected,
          devices: devices,
        ),
      );
      _heartbeat?.cancel();
      _heartbeat = Timer.periodic(heartbeatInterval, (_) => _tick());
      unawaited(_connectEventStream(generation));
    } on AccountDeviceDirectoryFailure catch (error) {
      if (!_isCurrent(generation)) return;
      debugPrint('OpenMuse device registration: ${error.code}');
      _publish(
        AccountDeviceDirectorySnapshot(
          registered: false,
          reconnecting: true,
          devices: _snapshot.devices,
          failureMessage: error.safeMessage,
        ),
      );
      _scheduleRetry(generation);
    } finally {
      _operationInFlight = false;
      if (_active &&
          authentication.snapshot.isAuthenticated &&
          generation != _identityGeneration) {
        unawaited(_registerAndRefresh(_identityGeneration));
      }
    }
  }

  void _tick() {
    if (!authentication.snapshot.isAuthenticated || _operationInFlight) return;
    unawaited(() async {
      final generation = _identityGeneration;
      _operationInFlight = true;
      try {
        await client.heartbeat(registration().deviceRef);
        final devices = await client.list();
        if (!_isCurrent(generation)) return;
        _consecutiveFailures = 0;
        _publish(
          AccountDeviceDirectorySnapshot(
            registered: true,
            realtimeConnected: _snapshot.realtimeConnected,
            devices: devices,
          ),
        );
      } on AccountDeviceDirectoryFailure catch (error) {
        if (!_isCurrent(generation)) return;
        _publish(
          AccountDeviceDirectorySnapshot(
            registered: false,
            realtimeConnected: _snapshot.realtimeConnected,
            reconnecting: true,
            devices: _snapshot.devices,
            failureMessage: error.safeMessage,
          ),
        );
        _scheduleRetry(generation);
      } finally {
        _operationInFlight = false;
      }
    }());
  }

  Future<void> _connectEventStream(int generation) async {
    if (!_isCurrent(generation) || _events != null) return;
    try {
      final socket = await client.connectEvents();
      if (!_isCurrent(generation)) {
        await socket.close();
        return;
      }
      _events = socket;
      _consecutiveFailures = 0;
      _publish(
        AccountDeviceDirectorySnapshot(
          registered: _snapshot.registered,
          realtimeConnected: true,
          devices: _snapshot.devices,
        ),
      );
      _eventSubscription = socket.listen(
        (_) => unawaited(refresh()),
        onError: (_) => _eventStreamEnded(generation),
        onDone: () => _eventStreamEnded(generation),
        cancelOnError: true,
      );
    } on AccountDeviceDirectoryFailure catch (error) {
      if (!_isCurrent(generation)) return;
      _publish(
        AccountDeviceDirectorySnapshot(
          registered: _snapshot.registered,
          reconnecting: true,
          devices: _snapshot.devices,
          failureMessage: error.safeMessage,
        ),
      );
      _scheduleRetry(generation, eventsOnly: true);
    }
  }

  void _eventStreamEnded(int generation) {
    _eventSubscription = null;
    _events = null;
    if (!_isCurrent(generation)) return;
    _publish(
      AccountDeviceDirectorySnapshot(
        registered: _snapshot.registered,
        reconnecting: true,
        devices: _snapshot.devices,
        failureMessage: '设备实时通道已断开，正在重连。',
      ),
    );
    _scheduleRetry(generation, eventsOnly: true);
  }

  void _scheduleRetry(int generation, {bool eventsOnly = false}) {
    if (!_isCurrent(generation) || _retry?.isActive == true) return;
    final exponent = min(_consecutiveFailures++, 5);
    final baseMs = min(
      maxReconnectDelay.inMilliseconds,
      1000 * (1 << exponent),
    );
    final jitterMs = baseMs <= 4 ? 0 : _jitter.nextInt(baseMs ~/ 4);
    _retry = Timer(Duration(milliseconds: baseMs + jitterMs), () {
      if (!_isCurrent(generation)) return;
      if (eventsOnly && _snapshot.registered) {
        unawaited(_connectEventStream(generation));
      } else {
        unawaited(_registerAndRefresh(generation));
      }
    });
  }

  void _stopTransports() {
    _heartbeat?.cancel();
    _heartbeat = null;
    _retry?.cancel();
    _retry = null;
    unawaited(_eventSubscription?.cancel());
    _eventSubscription = null;
    unawaited(_events?.close());
    _events = null;
    _consecutiveFailures = 0;
  }

  bool _isCurrent(int generation) =>
      _active &&
      generation == _identityGeneration &&
      authentication.snapshot.isAuthenticated;

  void _publish(AccountDeviceDirectorySnapshot value) {
    _snapshot = value;
    notifyListeners();
  }

  @override
  void dispose() {
    _active = false;
    _stopTransports();
    authentication.removeListener(_authenticationChanged);
    client.close();
    super.dispose();
  }
}
