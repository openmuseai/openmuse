import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_mobile/mobile_dsh_connection.dart';

void main() {
  test(
    'recent history survives restart and isolates accounts/devices',
    () async {
      final dir = await Directory.systemTemp.createTemp('dsh-cache-test');
      try {
        final cache = MobileDshRecentMessages(
          'account-a|desktop-1',
          cacheDirectory: () async => dir,
        );
        await cache.ready;
        cache.put(
          's',
          DshSnapshotFrame(
            header: const {'id': 's'},
            cursor: 3,
            hasMore: true,
            projections: const {},
            records: [
              DshWireEvent.fromJson({
                'type': 'user/message',
                'seq': 3,
                'time': 1,
                'data': {
                  'content': [
                    {'type': 'text', 'text': 'cached'},
                  ],
                },
              }),
            ],
          ),
        );
        await cache.flush();
        final restored = MobileDshRecentMessages(
          'account-a|desktop-1',
          cacheDirectory: () async => dir,
        );
        final other = MobileDshRecentMessages(
          'account-b|desktop-1',
          cacheDirectory: () async => dir,
        );
        await Future.wait([restored.ready, other.ready]);
        expect(restored.read('s')?.records.single.data['content'], isNotEmpty);
        expect(other.read('s'), isNull);
        cache.dispose();
        restored.dispose();
        other.dispose();
        await Future.wait([cache.flush(), restored.flush(), other.flush()]);
      } finally {
        await dir.delete(recursive: true);
      }
    },
  );
}
