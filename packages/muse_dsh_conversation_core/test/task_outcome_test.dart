import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:test/test.dart';

DshWireEvent event(int seq, String type, [String? reason]) =>
    DshWireEvent.fromJson({
      'type': type,
      'seq': seq,
      'time': seq,
      'data': {
        if (reason != null) 'reason': {'kind': reason},
      },
    });

void main() {
  late DshConversationStore store;
  late List<DshTaskOutcome> outcomes;
  setUp(() {
    store = DshConversationStore()..connecting('session-1');
    outcomes = [];
    store.taskOutcomes.listen(outcomes.add);
  });
  tearDown(() => store.dispose());
  test(
    'a received successful or failed running turn produces one safe outcome',
    () {
      store.apply(DshEventFrame(event(1, 'turn/start')));
      store.apply(DshEventFrame(event(2, 'turn/end', 'completed')));
      store.apply(DshEventFrame(event(3, 'turn/start')));
      store.apply(DshEventFrame(event(4, 'turn/end', 'error')));
      expect(outcomes.map((e) => e.result), [
        DshTaskResult.completed,
        DshTaskResult.failed,
      ]);
      expect(outcomes.map((e) => e.sequence), [2, 4]);
      expect(outcomes.every((e) => e.sessionId == 'session-1'), isTrue);
    },
  );
  test(
    'history, retries and out-of-order events do not duplicate notifications',
    () {
      store.apply(
        DshSnapshotFrame(
          header: {'id': 'session-1'},
          cursor: 10,
          records: [event(9, 'turn/start'), event(10, 'turn/end', 'completed')],
          hasMore: false,
          projections: const {},
        ),
      );
      expect(outcomes, isEmpty);
      store.apply(DshEventFrame(event(9, 'turn/start')));
      store.apply(DshEventFrame(event(10, 'turn/end', 'completed')));
      expect(store.snapshot.running, isFalse);
      store.apply(DshEventFrame(event(11, 'turn/start')));
      store.apply(DshEventFrame(event(12, 'turn/end', 'completed')));
      store.apply(DshEventFrame(event(11, 'turn/start')));
      store.apply(DshEventFrame(event(12, 'turn/end', 'completed')));
      expect(outcomes, hasLength(1));
      expect(store.snapshot.running, isFalse);
    },
  );
  test(
    'cancelled, interrupted, unknown and unobserved tasks are not successes',
    () {
      var sequence = 0;
      store.apply(DshEventFrame(event(++sequence, 'turn/end', 'completed')));
      for (final reason in ['cancelled', 'interrupted', 'new-unknown-kind']) {
        store.apply(DshEventFrame(event(++sequence, 'turn/start')));
        store.apply(DshEventFrame(event(++sequence, 'turn/end', reason)));
      }
      expect(outcomes, isEmpty);
    },
  );
  test('transport failure does not become a task failure', () {
    store.apply(DshEventFrame(event(1, 'turn/start')));
    store.failed(StateError('offline'));
    expect(outcomes, isEmpty);
    store.apply(DshEventFrame(event(2, 'turn/end', 'completed')));
    expect(outcomes.single.result, DshTaskResult.completed);
  });
}
