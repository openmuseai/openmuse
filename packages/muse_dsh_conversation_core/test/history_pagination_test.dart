import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:test/test.dart';

DshWireEvent event(int seq) => DshWireEvent.fromJson({
  'type': 'user/message',
  'seq': seq,
  'time': seq,
  'data': {
    'content': [
      {'type': 'text', 'text': 'message $seq'},
    ],
  },
});
DshSnapshotFrame page(List<int> seqs, {int cursor = 10, bool more = true}) =>
    DshSnapshotFrame(
      header: {'id': 's'},
      cursor: cursor,
      records: seqs.map(event).toList(),
      hasMore: more,
      projections: const {},
    );
void main() {
  test('older page racing a live event preserves cursor and recent rows', () {
    final store = DshConversationStore()..apply(page([8, 9, 10]));
    store.apply(DshEventFrame(event(11)));
    store.prepend(page([5, 6, 7]));
    expect(store.snapshot.cursor, 11);
    expect(store.snapshot.rows.map((r) => r.seq), [5, 6, 7, 8, 9, 10, 11]);
    store.prepend(page([1, 2, 3, 4], more: false));
    store.apply(page([9, 10, 11, 12], cursor: 12));
    expect(
      store.snapshot.rows.map((r) => r.seq),
      List.generate(12, (i) => i + 1),
    );
    expect(store.snapshot.hasMore, false);
  });
  test('a disconnected gap resets the tail and allows older pagination', () {
    final store = DshConversationStore()
      ..apply(page([1, 2, 3], cursor: 3, more: false));
    store.apply(page([20, 21], cursor: 21));
    expect(store.snapshot.rows.map((r) => r.seq), [20, 21]);
    expect(store.snapshot.hasMore, true);
  });
  test('recent cache is bounded and marks discarded history as pageable', () {
    final store = DshConversationStore()
      ..apply(page([1, 2, 3, 4], more: false));
    final cached = store.cachePage(maxRecords: 2);
    expect(cached.records.map((r) => r.seq), [3, 4]);
    expect(cached.hasMore, true);
    final restored = DshConversationStore()..apply(cached);
    expect(restored.snapshot.rows.map((r) => r.text), [
      'message 3',
      'message 4',
    ]);
  });
}
