import 'dart:convert';
import 'dart:typed_data';
import 'codec.dart';
import 'core_api.dart';

/// A transient projection of original Yrs Awareness. Actor data received from
/// Cloud is normalized by authenticated UID/client slot, not a profile claim.
final class PresenceClient {
  PresenceClient(Map value)
    : clientId = value['clientId'] as String,
      local = value['local'] as bool,
      clock = value['clock'] as int,
      state = Map.unmodifiable(
        (jsonDecode(jsonEncode(value['state'])) as Map).cast<String, dynamic>(),
      );
  final String clientId;
  final bool local;
  final int clock;
  final Map<String, dynamic> state;
  String? get actorUid =>
      state['actor'] is Map ? (state['actor'] as Map)['uid'] as String? : null;
}

final class RealtimePresence {
  RealtimePresence({
    required this.core,
    required this.objectId,
    required this.clientId,
  }) {
    final state = core.call({
      'action': 'presenceOpen',
      'objectId': objectId,
      'clientId': clientId,
      'nowMs': 0,
    });
    _handle = state['handle'] as String;
    _clock.start();
  }
  final RealtimeWireCore core;
  final String objectId, clientId;
  late final String _handle;
  final _clock = Stopwatch();
  bool _closed = false;
  Map<String, dynamic> _call(
    String action, [
    Map<String, dynamic> extra = const {},
  ]) {
    if (_closed) throw const RealtimeFailure('SESSION_CLOSED');
    return core.call({
      'action': action,
      'handle': _handle,
      'objectId': objectId,
      'nowMs': _clock.elapsedMilliseconds,
      ...extra,
    });
  }

  List<PresenceClient> get clients => List.unmodifiable(
    (_call('presenceSnapshot')['clients'] as List).map(
      (v) => PresenceClient(v as Map),
    ),
  );
  RealtimeAwareness update(
    Map<String, dynamic> state, {
    required int collabType,
  }) {
    final value = _call('presenceSet', {'state': state});
    return RealtimeAwareness(
      objectId: objectId,
      collabType: collabType,
      awareness: base64Decode(value['awareness'] as String),
    );
  }

  void apply(RealtimeAwareness packet) {
    if (packet.objectId != objectId)
      throw const RealtimeFailure('PRESENCE_SCOPE_MISMATCH');
    _call('presenceApply', {'awareness': base64Encode(packet.awareness)});
  }

  RealtimeAwareness leave({required int collabType}) => RealtimeAwareness(
    objectId: objectId,
    collabType: collabType,
    awareness: Uint8List.fromList(
      base64Decode(_call('presenceLeave')['awareness'] as String),
    ),
  );
  void close() {
    if (_closed) return;
    core.call({'action': 'presenceClose', 'handle': _handle});
    _clock.stop();
    _closed = true;
  }
}
