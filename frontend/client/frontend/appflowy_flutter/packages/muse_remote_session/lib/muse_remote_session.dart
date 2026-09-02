enum MuseRemoteConnectionState {
  detached,
  connecting,
  online,
  suspended,
  offline,
  revoked
}

enum MuseRemoteOperationDecision {
  run,
  queueIdempotent,
  suspendUnknown,
  reject
}

final class MuseRemoteAttachment {
  const MuseRemoteAttachment(
      {required this.attachmentId,
      required this.sessionRef,
      required this.deviceId,
      required this.role,
      required this.generation,
      required this.incarnation});
  final String attachmentId;
  final String sessionRef;
  final String deviceId;
  final String role;
  final int generation;
  final String incarnation;
}

final class MuseRemoteSessionClient {
  MuseRemoteSessionClient({required this.deviceId, int Function()? clock})
      : _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch);
  final String deviceId;
  final int Function() _clock;
  MuseRemoteConnectionState _state = MuseRemoteConnectionState.detached;
  MuseRemoteAttachment? _attachment;
  var _stateCursor = '0';
  final _control = <String, ({Object? payload, int expiresAt})>{};

  MuseRemoteConnectionState get state => _state;
  MuseRemoteAttachment? get attachment => _attachment;
  String get resumeCursor => _stateCursor;

  void connecting() {
    if (_state == MuseRemoteConnectionState.revoked)
      throw StateError('SESSION_REVOKED');
    _state = MuseRemoteConnectionState.connecting;
  }

  void attached(MuseRemoteAttachment value) {
    if (value.deviceId != deviceId) throw StateError('DEVICE_MISMATCH');
    _attachment = value;
    _state = MuseRemoteConnectionState.online;
  }

  void networkLost({required bool operationInFlight}) {
    if (_state == MuseRemoteConnectionState.revoked) return;
    _state = operationInFlight
        ? MuseRemoteConnectionState.suspended
        : MuseRemoteConnectionState.offline;
    _control.clear();
  }

  void applyStateCursor(String cursor) {
    if (int.parse(cursor) <= int.parse(_stateCursor)) return;
    _stateCursor = cursor;
  }

  void putControl(String key, Object? payload, int expiresAt) {
    if (_state != MuseRemoteConnectionState.online || expiresAt <= _clock())
      return;
    _control[key] = (payload: payload, expiresAt: expiresAt);
  }

  Map<String, Object?> controlSnapshot() {
    final now = _clock();
    _control.removeWhere((_, value) => value.expiresAt <= now);
    return Map.unmodifiable(
        _control.map((key, value) => MapEntry(key, value.payload)));
  }

  MuseRemoteOperationDecision decideAgentOperation(
      {required bool idempotent, required bool alreadyStarted}) {
    if (_state == MuseRemoteConnectionState.revoked)
      return MuseRemoteOperationDecision.reject;
    if (_state == MuseRemoteConnectionState.online)
      return MuseRemoteOperationDecision.run;
    if (alreadyStarted) return MuseRemoteOperationDecision.suspendUnknown;
    return idempotent
        ? MuseRemoteOperationDecision.queueIdempotent
        : MuseRemoteOperationDecision.reject;
  }

  void revoke() {
    _state = MuseRemoteConnectionState.revoked;
    _attachment = null;
    _control.clear();
  }
}
