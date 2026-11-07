import 'package:flutter_test/flutter_test.dart';
import 'package:muse_appflowy_realtime/muse_appflowy_realtime.dart';

void main() {
  test('original awareness is scoped and transient, preserves numeric actor UID strings and explicit leave', () {
    final core = NativeRealtimeWireCore();
    const id = '041c01d0-831a-408a-b8fa-190d40dd9e91';
    final a = RealtimePresence(core: core, objectId: id, clientId: '9007199254740993');
    final b = RealtimePresence(core: core, objectId: id, clientId: '7');
    final foreign = RealtimePresence(core: core, objectId: '041c01d0-831a-408a-b8fa-190d40dd9e92', clientId: '8');
    try {
      final packet = a.update({'actor': {'uid': '18446744073709551615'}}, collabType: 0);
      b.apply(packet);
      expect(b.clients.single.clientId, '9007199254740993');
      expect(b.clients.single.actorUid, '18446744073709551615');
      expect(b.clients.single.local, isFalse);
      expect(() => foreign.apply(packet), throwsA(isA<RealtimeFailure>()));
      b.apply(a.leave(collabType: 0));
      expect(b.clients, isEmpty);
    } finally { a.close(); b.close(); foreign.close(); }
  });
}
