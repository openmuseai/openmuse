import 'dart:convert';
import 'dart:typed_data';
import 'core.dart';

sealed class RealtimePacket {
  const RealtimePacket();
  Map<String, dynamic> toJson();
}

sealed class ObjectRealtimePacket extends RealtimePacket {
  const ObjectRealtimePacket(this.objectId, this.collabType);
  final String objectId;
  final int collabType;
  Map<String, dynamic> address(String kind) => {
    'kind': kind,
    'objectId': objectId,
    'collabType': collabType,
  };
}

final class RealtimeManifest extends ObjectRealtimePacket {
  RealtimeManifest({
    required String objectId,
    required int collabType,
    required Uint8List stateVector,
    this.lastMessageId = '0-0',
  }) : stateVector = Uint8List.fromList(stateVector).asUnmodifiableView(),
       super(objectId, collabType);
  final Uint8List stateVector;
  final String lastMessageId;
  @override
  Map<String, dynamic> toJson() => {
    ...address('manifest'),
    'stateVector': base64Encode(stateVector),
    'lastMessageId': lastMessageId,
  };
}

final class RealtimeUpdate extends ObjectRealtimePacket {
  RealtimeUpdate({
    required String objectId,
    required int collabType,
    required Uint8List update,
    this.lastMessageId = '0-0',
  }) : update = Uint8List.fromList(update).asUnmodifiableView(),
       super(objectId, collabType);
  final Uint8List update;

  /// Original Redis RID is an accepted-stream cursor, never a durable-write ACK.
  final String lastMessageId;
  @override
  Map<String, dynamic> toJson() => {
    ...address('update'),
    'update': base64Encode(update),
    'lastMessageId': lastMessageId,
  };
}

final class RealtimeAwareness extends ObjectRealtimePacket {
  RealtimeAwareness({
    required String objectId,
    required int collabType,
    required Uint8List awareness,
  }) : awareness = Uint8List.fromList(awareness).asUnmodifiableView(),
       super(objectId, collabType);
  final Uint8List awareness;
  @override
  Map<String, dynamic> toJson() => {
    ...address('awareness'),
    'awareness': base64Encode(awareness),
  };
}

final class RealtimeAccess extends ObjectRealtimePacket {
  const RealtimeAccess({
    required String objectId,
    required int collabType,
    required this.canRead,
    required this.canWrite,
    required this.reason,
  }) : super(objectId, collabType);
  final bool canRead, canWrite;
  final String reason;
  @override
  Map<String, dynamic> toJson() => {
    ...address('access'),
    'canRead': canRead,
    'canWrite': canWrite,
    'reason': reason,
  };
}

final class RealtimeNotification extends RealtimePacket {
  RealtimeNotification(Map<String, dynamic> value)
    : data = Map.unmodifiable(value);
  final Map<String, dynamic> data;
  @override
  Map<String, dynamic> toJson() => Map.of(data);
}

/// Calls original AppFlowy types behind the bounded native/WASM protocol codec.
/// No document, field, cell or UI-specific operation is defined here.
final class AppFlowyRealtimeCodec {
  const AppFlowyRealtimeCodec(this.core);
  final RealtimeWireCore core;

  /// Input must be the bounded body from the provider's authenticated Cloud
  /// profile GET. Rust extracts the original i64 before Dart/Web JSON rounding.
  String profileActorUid(Uint8List body) {
    if (body.length > 65536)
      throw const RealtimeFailure('PROFILE_QUOTA_EXCEEDED');
    return core.call({
          'action': 'profileIdentity',
          'body': base64Encode(body),
        })['uid']
        as String;
  }

  static const maximumFrameBytes = 8 * 1024 * 1024;
  Uint8List encode(RealtimePacket packet) => base64Decode(
    core.call({'action': 'encode', 'message': packet.toJson()})['frame']
        as String,
  );
  RealtimePacket decode(Uint8List frame) {
    if (frame.length > maximumFrameBytes)
      throw const RealtimeFailure('QUOTA_EXCEEDED');
    final value = core.call({'action': 'decode', 'frame': base64Encode(frame)});
    if (value['kind'] == 'notification') return RealtimeNotification(value);
    final id = value['objectId'] as String, type = value['collabType'] as int;
    return switch (value['kind']) {
      'manifest' => RealtimeManifest(
        objectId: id,
        collabType: type,
        stateVector: base64Decode(value['stateVector'] as String),
        lastMessageId: value['lastMessageId'] as String,
      ),
      'update' => RealtimeUpdate(
        objectId: id,
        collabType: type,
        update: base64Decode(value['update'] as String),
        lastMessageId: value['lastMessageId'] as String,
      ),
      'awareness' => RealtimeAwareness(
        objectId: id,
        collabType: type,
        awareness: base64Decode(value['awareness'] as String),
      ),
      'access' => RealtimeAccess(
        objectId: id,
        collabType: type,
        canRead: value['canRead'] as bool,
        canWrite: value['canWrite'] as bool,
        reason: value['reason'] as String,
      ),
      _ => throw const RealtimeFailure('UNSUPPORTED_MESSAGE'),
    };
  }
}
