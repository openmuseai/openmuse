import 'package:test/test.dart';
import 'package:muse_collab_contract/muse_collab_contract.dart';

void main() {
  test(
    'multiple live positions aggregate by authenticated actor with stable immutable order',
    () {
      const values = [
        CollaborationPresenceSlot(actorId: 'bob', sessionId: '2'),
        CollaborationPresenceSlot(actorId: 'alice', sessionId: '3'),
        CollaborationPresenceSlot(actorId: 'bob', sessionId: '1'),
        CollaborationPresenceSlot(actorId: 'bob', sessionId: '2'),
      ];
      final users = groupPresenceSlots(values),
          reverse = groupPresenceSlots(values.reversed);
      expect(users.map((u) => u.actorId), ['alice', 'bob']);
      expect(users.last.sessionIds, ['1', '2']);
      expect(reverse.last.sessionIds, users.last.sessionIds);
      expect(() => users.clear(), throwsUnsupportedError);
      expect(() => users.last.sessionIds.clear(), throwsUnsupportedError);
      expect(
        groupPresenceSlots(
          values.where((s) => s.sessionId != '1'),
        ).last.sessionIds,
        ['2'],
      );
      expect(
        groupPresenceSlots(
          values.where((s) => s.actorId != 'bob'),
        ).map((u) => u.actorId),
        ['alice'],
      );
    },
  );
  test(
    'conflicting identity, unbounded or invalid slots never produce a misleading count',
    () {
      expect(
        () => groupPresenceSlots(const [
          CollaborationPresenceSlot(actorId: 'alice', sessionId: '1'),
          CollaborationPresenceSlot(actorId: 'bob', sessionId: '1'),
        ]),
        throwsFormatException,
      );
      for (final invalid in ['', 'a\n', 'x' * 129]) {
        expect(
          () => groupPresenceSlots([
            CollaborationPresenceSlot(actorId: invalid, sessionId: '1'),
          ]),
          throwsFormatException,
        );
      }
      expect(
        () => groupPresenceSlots(
          List.generate(
            1025,
            (i) => CollaborationPresenceSlot(actorId: 'a', sessionId: '$i'),
          ),
        ),
        throwsFormatException,
      );
    },
  );
}
