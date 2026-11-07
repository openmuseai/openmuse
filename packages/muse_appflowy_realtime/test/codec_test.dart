import 'dart:typed_data';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_appflowy_realtime/muse_appflowy_realtime.dart';

void main() {
  test(
    'native codec uses original manifest including 64-bit RID without editor dependencies',
    () {
      final codec = AppFlowyRealtimeCodec(NativeRealtimeWireCore());
      final message = RealtimeManifest(
        objectId: '041c01d0-831a-408a-b8fa-190d40dd9e94',
        collabType: 4,
        stateVector: Uint8List.fromList([0]),
        lastMessageId: '18446744073709551615-65535',
      );
      final decoded = codec.decode(codec.encode(message)) as RealtimeManifest;
      expect(decoded.objectId, message.objectId);
      expect(decoded.collabType, 4);
      expect(decoded.stateVector, [0]);
      expect(decoded.lastMessageId, message.lastMessageId);
      expect(
        () => codec.decode(
          Uint8List(AppFlowyRealtimeCodec.maximumFrameBytes + 1),
        ),
        throwsA(isA<RealtimeFailure>()),
      );
    },
  );
}
