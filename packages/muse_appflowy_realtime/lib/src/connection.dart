import 'dart:async';
import 'dart:typed_data';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'codec.dart';
import 'core_api.dart';
import 'profile_identity.dart';
import 'socket_factory.dart' if (dart.library.io) 'socket_factory_io.dart';

/// Provider-level socket construction port; defaults to the original channel.
typedef AppFlowySocketFactory = WebSocketChannel Function(Uri uri);

/// Trusted Host override; default resolves the original authenticated Cloud
/// profile through an origin-bound GET and lossless Rust UID decoder.
typedef AppFlowyActorIdentityLoader = Future<String> Function(String token);

enum RealtimeConnectionState {
  idle,
  connecting,
  connected,
  disconnected,
  closed,
}

/// One authenticated original WS v2 connection. A content-domain owner decides
/// subscriptions, reconciliation, outbox and reconnect policy. No content cache,
/// credentials, RID-as-durability promise or editor operations live here.
final class AppFlowyRealtimeConnection {
  AppFlowyRealtimeConnection({
    required this.origin,
    required this.workspaceId,
    required this.deviceId,
    required this.clientId,
    required this.accessToken,
    required this.codec,
    AppFlowySocketFactory? socketFactory,
    AppFlowyActorIdentityLoader? actorIdentityLoader,
  }) : _socketFactory = socketFactory ?? createAppFlowySocket,
       _actorIdentityLoader =
           actorIdentityLoader ??
           ((token) => readAppFlowyActorUid(
             origin: origin,
             token: token,
             codec: codec,
           )) {
    final local = {
      'localhost',
      '127.0.0.1',
      '::1',
      '10.0.2.2',
    }.contains(origin.host);
    if ((origin.scheme != 'https' && !(origin.scheme == 'http' && local)) ||
        origin.userInfo.isNotEmpty ||
        origin.hasQuery ||
        origin.hasFragment ||
        !{'', '/'}.contains(origin.path))
      throw ArgumentError('A bounded Cloud origin is required');
    if (!RegExp(
          r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$',
        ).hasMatch(workspaceId) ||
        deviceId.isEmpty ||
        deviceId.length > 128 ||
        deviceId.codeUnits.any((v) => v < 32 || v == 127))
      throw ArgumentError('Invalid connection identity');
    final numeric = BigInt.tryParse(clientId);
    if (numeric == null ||
        numeric <= BigInt.zero ||
        numeric > BigInt.parse('18446744073709551615') ||
        numeric.toString() != clientId)
      throw ArgumentError('Invalid client ID');
  }
  final AppFlowySocketFactory _socketFactory;
  final AppFlowyActorIdentityLoader _actorIdentityLoader;
  final _actorChanges = StreamController<String?>.broadcast(sync: true);
  Stream<String?> get actorChanges => _actorChanges.stream;
  String? _actorUid;
  String? get actorUid => _actorUid;
  String? actorFailureCode;
  Timer? _identityRetry;
  void _actor(String? uid) {
    if (_actorUid == uid) return;
    _actorUid = uid;
    if (!_actorChanges.isClosed) _actorChanges.add(uid);
  }

  Future<void> _resolveActor(String token, int epoch, {int attempt = 0}) async {
    if (_closed ||
        epoch != _epoch ||
        state != RealtimeConnectionState.connected)
      return;
    try {
      final uid = await _actorIdentityLoader(
        token,
      ).timeout(const Duration(seconds: 6));
      if (_closed ||
          epoch != _epoch ||
          state != RealtimeConnectionState.connected)
        return;
      final numeric = BigInt.tryParse(uid);
      if (numeric == null ||
          numeric <= BigInt.zero ||
          numeric > BigInt.parse('9223372036854775807') ||
          numeric.toString() != uid)
        throw const FormatException('INVALID_ACTOR_UID');
      actorFailureCode = null;
      _identityRetry?.cancel();
      _actor(uid);
    } on Object {
      if (!_closed &&
          epoch == _epoch &&
          state == RealtimeConnectionState.connected) {
        actorFailureCode = 'ACTOR_IDENTITY_UNAVAILABLE';
        if (attempt < 2)
          _identityRetry = Timer(
            Duration(seconds: 5 * (attempt + 1)),
            () => unawaited(_resolveActor(token, epoch, attempt: attempt + 1)),
          );
      }
    }
  }

  final Uri origin;
  final String workspaceId, deviceId, clientId;
  final Future<String> Function() accessToken;
  final AppFlowyRealtimeCodec codec;
  final _packets = StreamController<RealtimePacket>.broadcast(sync: true);
  final _states = StreamController<RealtimeConnectionState>.broadcast(
    sync: true,
  );
  Stream<RealtimePacket> get packets => _packets.stream;
  Stream<RealtimeConnectionState> get states => _states.stream;
  RealtimeConnectionState state = RealtimeConnectionState.idle;
  String? connectionFailureCode;
  String connectionPhase = 'idle';
  WebSocketChannel? _channel;
  StreamSubscription<dynamic>? _subscription;
  Future<void>? _connecting, _closing;
  bool _closed = false;
  int _epoch = 0;
  void _state(RealtimeConnectionState value) {
    state = value;
    if (value != RealtimeConnectionState.connected) {
      _identityRetry?.cancel();
      _actor(null);
      actorFailureCode = null;
    }
    if (!_states.isClosed) _states.add(value);
  }

  Future<void> connect() {
    if (_closed) return Future.error(const RealtimeFailure('SESSION_CLOSED'));
    if (state == RealtimeConnectionState.connected) return Future.value();
    return _connecting ??= _connect().whenComplete(() => _connecting = null);
  }

  Future<void> _connect() async {
    final epoch = ++_epoch;
    connectionFailureCode = null;
    connectionPhase = 'authorizing';
    _state(RealtimeConnectionState.connecting);
    WebSocketChannel? channel;
    var ready = false;
    try {
      final token = await accessToken().timeout(const Duration(seconds: 20));
      if (_closed || epoch != _epoch)
        throw const RealtimeFailure('SESSION_CLOSED');
      // Existing Cloud's browser-compatible query authentication. This URI must
      // never be logged, persisted, rendered in UI or included in exceptions.
      final uri = origin.replace(
        scheme: origin.scheme == 'https' ? 'wss' : 'ws',
        path: '/ws/v2/$workspaceId',
        queryParameters: {
          'token': token,
          'deviceId': deviceId,
          'clientId': clientId,
        },
      );
      channel = _socketFactory(uri);
      connectionPhase = 'handshaking';
      _channel = channel;
      _subscription = channel.stream.listen(
        (dynamic data) {
          if (_closed || epoch != _epoch) return;
          try {
            if (data is! List<int>)
              throw const RealtimeFailure('BINARY_FRAME_REQUIRED');
            final frame = data is Uint8List ? data : Uint8List.fromList(data);
            _packets.add(codec.decode(frame));
          } on Object {
            _packets.addError(const RealtimeFailure('INVALID_SERVER_FRAME'));
            unawaited(disconnect());
          }
        },
        onError: (Object _) {
          if (!_closed && epoch == _epoch && ready) {
            _state(RealtimeConnectionState.disconnected);
            _packets.addError(const RealtimeFailure('CONNECTION_LOST'));
          }
        },
        onDone: () {
          if (!_closed && epoch == _epoch)
            _state(RealtimeConnectionState.disconnected);
        },
      );
      await channel.ready.timeout(const Duration(seconds: 20));
      if (_closed || epoch != _epoch) {
        try {
          await channel.sink.close().timeout(const Duration(seconds: 2));
        } on Object {}
        return;
      }
      ready = true;
      connectionPhase = 'connected';
      _state(RealtimeConnectionState.connected);
      unawaited(_resolveActor(token, epoch));
    } on Object catch (error) {
      final status = RegExp(
        r'status code[^0-9]*([0-9]{3})',
        caseSensitive: false,
      ).firstMatch('$error')?.group(1);
      connectionFailureCode = error is TimeoutException
          ? 'CONNECT_TIMEOUT'
          : status == null
          ? 'CONNECT_FAILED'
          : 'HTTP_$status';
      assert(() {
        print(
          'AppFlowy realtime connection: $connectionPhase $connectionFailureCode',
        );
        return true;
      }());
      connectionPhase = 'disconnected';
      try {
        await channel?.sink.close().timeout(const Duration(seconds: 2));
      } on Object {}
      if (!_closed && epoch == _epoch)
        _state(RealtimeConnectionState.disconnected);
      throw const RealtimeFailure('CONNECT_FAILED');
    }
  }

  void send(RealtimePacket packet) {
    if (_closed || state != RealtimeConnectionState.connected)
      throw const RealtimeFailure('NOT_CONNECTED');
    _channel!.sink.add(codec.encode(packet));
  }

  Future<void> disconnect() async {
    _epoch++;
    final channel = _channel;
    _channel = null;
    final subscription = _subscription;
    _subscription = null;
    _identityRetry?.cancel();
    _actor(null);
    actorFailureCode = null;
    if (!_closed) _state(RealtimeConnectionState.disconnected);
    await subscription?.cancel();
    try {
      await channel?.sink.close();
    } on Object {}
  }

  Future<void> close() => _closing ??= (() async {
    if (_closed) return;
    _closed = true;
    _state(RealtimeConnectionState.closed);
    await disconnect();
    await _packets.close();
    await _states.close();
    await _actorChanges.close();
  })();
}
