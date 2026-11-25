import 'package:test/test.dart';
import 'package:muse_original_sync/muse_original_sync.dart';
import 'original_sync_journal_test.dart' as fixture;

void main() {
  test(
    'ACK retains exact identity across cold checkpoint and prevents new-account or model writes',
    () async {
      final journal = OriginalSyncJournal(
        OriginalSyncJournalState.empty(),
        (_) async {},
      );
      final pending = await fixture.prepare(journal);
      await journal.acknowledge(fixture.receipt(pending));
      final saved = journal.state.toJson();
      expect(saved['boundScope'], pending.scope.toJson());
      final restored = OriginalSyncJournal(
        OriginalSyncJournalState.fromJson(saved),
        (_) async {},
      );
      for (final changed in [
        fixture.scope({'principalId': '9007199254740994'}),
        fixture.scope({'generation': '44444444-1111-2222-3333-444444444444'}),
        fixture.scope({'epoch': '1'}),
      ]) {
        await expectLater(
          fixture.prepare(restored, binding: changed, epoch: 8),
          throwsStateError,
        );
      }
      expect(restored.state.toJson(), saved);
      expect(restored.state.pending, isNull);
      expect(
        (await fixture.prepare(
          restored,
          epoch: 8,
        )).scope.matches(pending.scope),
        isTrue,
      );
    },
  );
  test(
    'identity persists even when first acknowledged request has local epoch zero',
    () async {
      final journal = OriginalSyncJournal(
        OriginalSyncJournalState.empty(),
        (_) async {},
      );
      final pending = await fixture.prepare(journal, epoch: 0);
      await journal.acknowledge(fixture.receipt(pending));
      final cold = OriginalSyncJournalState.fromJson(journal.state.toJson());
      expect(cold.boundScope!.matches(pending.scope), isTrue);
      expect(cold.durableLocalEpoch, 0);
    },
  );
  test(
    'missing or mismatched persisted identity cannot adopt a pending body',
    () async {
      final journal = OriginalSyncJournal(
        OriginalSyncJournalState.empty(),
        (_) async {},
      );
      await fixture.prepare(journal);
      final a = journal.state.toJson()..['boundScope'] = null;
      expect(() => OriginalSyncJournalState.fromJson(a), throwsFormatException);
      final b = journal.state.toJson()
        ..['boundScope'] = fixture.scope({'epoch': '1'}).toJson();
      expect(() => OriginalSyncJournalState.fromJson(b), throwsFormatException);
    },
  );
}
