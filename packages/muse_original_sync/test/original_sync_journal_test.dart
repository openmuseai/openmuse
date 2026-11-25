import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';
import 'package:muse_content_store/io.dart';
import 'package:muse_original_sync/muse_original_sync.dart';
import 'package:test/test.dart';

const operationId = 'aaaaaaaa-1111-2222-3333-444444444444';
OriginalSyncScope scope([Map<String, Object> changes = const {}]) =>
    OriginalSyncScope.fromJson({
      'origin': 'http://127.0.0.1:18001',
      'principalId': '9007199254740993',
      'workspaceId': '11111111-1111-2222-3333-444444444444',
      'objectId': '22222222-1111-2222-3333-444444444444',
      'collabType': 0,
      'generation': '33333333-1111-2222-3333-444444444444',
      'modelProfile': 'appflowy.original-collab/1',
      'epoch': '0',
      ...changes,
    });
Future<PendingOriginalSync> prepare(
  OriginalSyncJournal j, {
  int epoch = 7,
  OriginalSyncScope? binding,
  Uint8List? body,
}) => j.prepare(
  scope: binding ?? scope(),
  operationId: operationId,
  request: body ?? Uint8List.fromList([0, 1, 255, 3]),
  sentLocalEpoch: epoch,
);
Map<String, Object?> receiptJson(PendingOriginalSync p) => {
  'profile': 'openmuse.original-sync-receipt/1',
  'status': 'committed',
  'scope': p.scope.toJson(),
  'operationId': p.operationId,
  'requestSha256': p.requestSha256,
  'docStateSha256': 'a' * 64,
  'recordedAt': '2026-11-21T11:00:00.123456Z',
};
CommittedOriginalSyncReceipt receipt(PendingOriginalSync p) =>
    CommittedOriginalSyncReceipt.fromJson(receiptJson(p));

void main() {
  test(
    'send ticket waits for successful atomic checkpoint and owns exact bytes',
    () async {
      final gate = Completer<void>();
      var wrote = false;
      final journal = OriginalSyncJournal(OriginalSyncJournalState.empty(), (
        state,
      ) async {
        await gate.future;
        expect(state.pending!.request, [0, 1, 255, 3]);
        wrote = true;
      });
      final bytes = Uint8List.fromList([0, 1, 255, 3]);
      final pending = prepare(journal, body: bytes);
      bytes[0] = 9;
      var returned = false;
      unawaited(pending.then((_) => returned = true));
      await Future<void>.delayed(Duration.zero);
      expect(returned, false);
      expect(journal.state.pending, isNull);
      gate.complete();
      final ticket = await pending;
      expect(wrote, true);
      expect(ticket.request, [0, 1, 255, 3]);
      expect(() => ticket.request[0] = 9, throwsUnsupportedError);
    },
  );
  test(
    'failed write exposes no ticket, next explicit attempt can persist',
    () async {
      var fail = true;
      final journal = OriginalSyncJournal(OriginalSyncJournalState.empty(), (
        _,
      ) async {
        if (fail) throw const FileSystemException('synthetic');
      });
      await expectLater(prepare(journal), throwsA(isA<FileSystemException>()));
      expect(journal.state.pending, isNull);
      fail = false;
      expect((await prepare(journal)).sentLocalEpoch, 7);
    },
  );
  test(
    'unknown result keeps first operation despite newer edits and queued calls',
    () async {
      var writes = 0;
      final journal = OriginalSyncJournal(OriginalSyncJournalState.empty(), (
        _,
      ) async {
        writes++;
      });
      final first = prepare(journal);
      final second = prepare(journal, epoch: 8, body: Uint8List.fromList([99]));
      expect(identical(await first, await second), true);
      expect(writes, 1);
      final restored = OriginalSyncJournal(
        OriginalSyncJournalState.fromJson(
          jsonDecode(jsonEncode(journal.state.toJson()))
              as Map<String, Object?>,
        ),
        (_) async {
          writes++;
        },
      );
      final retry = await prepare(
        restored,
        epoch: 9,
        body: Uint8List.fromList([98]),
      );
      expect(retry.request, [0, 1, 255, 3]);
      expect(retry.sentLocalEpoch, 7);
      expect(writes, 1);
    },
  );
  test(
    'origin/account/object/kind/generation/profile/epoch changes preserve recovery queue',
    () async {
      final journal = OriginalSyncJournal(
        OriginalSyncJournalState.empty(),
        (_) async {},
      );
      final first = await prepare(journal);
      for (final change in <Map<String, Object>>[
        {'origin': 'https://other.example.com'},
        {'principalId': '9007199254740992'},
        {'workspaceId': '44444444-1111-2222-3333-444444444444'},
        {'objectId': '55555555-1111-2222-3333-444444444444'},
        {'collabType': 4},
        {'generation': '66666666-1111-2222-3333-444444444444'},
        {'modelProfile': 'future/1'},
        {'epoch': '1'},
      ]) {
        await expectLater(
          prepare(journal, binding: scope(change)),
          throwsStateError,
        );
        final drift = receiptJson(first)..['scope'] = scope(change).toJson();
        await expectLater(
          journal.acknowledge(CommittedOriginalSyncReceipt.fromJson(drift)),
          throwsStateError,
        );
        expect(identical(journal.state.pending, first), true);
      }
    },
  );
  test(
    'legacy acceptance and unrelated operation or digest cannot retire bytes',
    () async {
      final journal = OriginalSyncJournal(
        OriginalSyncJournalState.empty(),
        (_) async {},
      );
      final first = await prepare(journal);
      for (final state in ['accepted', 'pending', 'failed']) {
        expect(
          () => CommittedOriginalSyncReceipt.fromJson(
            receiptJson(first)..['status'] = state,
          ),
          throwsFormatException,
        );
      }
      for (final change in <Map<String, Object>>[
        {'operationId': '77777777-1111-2222-3333-444444444444'},
        {'requestSha256': 'b' * 64},
      ]) {
        await expectLater(
          journal.acknowledge(
            CommittedOriginalSyncReceipt.fromJson(
              receiptJson(first)..addAll(change),
            ),
          ),
          throwsStateError,
        );
        expect(journal.state.pending, isNotNull);
      }
    },
  );
  test('receipt retirement failure preserves pending exact retry', () async {
    var fail = false;
    final journal = OriginalSyncJournal(OriginalSyncJournalState.empty(), (
      _,
    ) async {
      if (fail) throw const FileSystemException('synthetic');
    });
    final first = await prepare(journal);
    fail = true;
    await expectLater(
      journal.acknowledge(receipt(first)),
      throwsA(isA<FileSystemException>()),
    );
    expect(identical(journal.state.pending, first), true);
    expect(journal.state.durableLocalEpoch, 0);
    fail = false;
    expect(await journal.acknowledge(receipt(first)), 7);
    expect(journal.state.pending, isNull);
  });
  test(
    'one original checkpoint survives fresh process; receipt retires only sent epoch',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'muse-sync-journal-',
      );
      addTearDown(() => directory.delete(recursive: true));
      final file = File('${directory.path}/original-checkpoint.json');
      final store = FileSnapshotStore(file);
      var currentEpoch = 7;
      final docState = base64Encode([10, 20, 30]);
      Future<void> persist(OriginalSyncJournalState state) => store.write({
        'profile': 'muse.smart-document/phase1',
        'docState': docState,
        'localMutationEpoch': currentEpoch,
        'pendingLocalChanges': currentEpoch > state.durableLocalEpoch,
        'syncJournal': state.toJson(),
      });
      final original = OriginalSyncJournal(
        OriginalSyncJournalState.empty(),
        persist,
      );
      final first = await prepare(original);
      final result = await Process.run(Platform.resolvedExecutable, [
        'run',
        'tool/reopen.dart',
        file.path,
      ]);
      expect(result.exitCode, 0, reason: result.stderr.toString());
      final cold = jsonDecode(result.stdout as String) as Map<String, dynamic>;
      expect(cold['docState'], docState);
      expect(cold['syncJournal'], original.state.toJson());
      final restored = OriginalSyncJournal(
        OriginalSyncJournalState.fromJson(
          (cold['syncJournal'] as Map).cast<String, Object?>(),
        ),
        persist,
      );
      // A network response may have been lost after server COMMIT. Reopening and
      // a new edit still resend the same bytes/id before any new operation.
      currentEpoch = 8;
      final retry = await prepare(
        restored,
        epoch: 8,
        body: Uint8List.fromList([9]),
      );
      expect(retry.toJson(), first.toJson());
      expect(await restored.acknowledge(receipt(first)), 7);
      final saved = await store.read();
      expect(saved!['pendingLocalChanges'], true);
      expect(saved['localMutationEpoch'], 8);
      expect(saved['docState'], docState);
      final next = await restored.prepare(
        scope: scope(),
        operationId: 'bbbbbbbb-1111-2222-3333-444444444444',
        request: Uint8List.fromList([9]),
        sentLocalEpoch: 8,
      );
      expect(next.operationId, isNot(first.operationId));
      expect(next.sentLocalEpoch, 8);
    },
  );
  test(
    'tampered bytes/digest, impossible epochs and malformed checkpoint fail closed',
    () async {
      final journal = OriginalSyncJournal(
        OriginalSyncJournalState.empty(),
        (_) async {},
      );
      await prepare(journal);
      Map<String, Object?> copy() =>
          jsonDecode(jsonEncode(journal.state.toJson()))
              as Map<String, Object?>;
      for (final field in [
        'request',
        'requestSha256',
        'operationId',
        'sentLocalEpoch',
      ]) {
        final value = copy();
        final p = value['pending'] as Map<String, dynamic>;
        p[field] = switch (field) {
          'request' => base64Encode([9]),
          'requestSha256' => 'a' * 64,
          'operationId' => 'bad',
          _ => -1,
        };
        expect(
          () => OriginalSyncJournalState.fromJson(value),
          throwsFormatException,
        );
      }
      expect(
        () => OriginalSyncJournalState.fromJson(
          copy()..['durableLocalEpoch'] = 8,
        ),
        throwsFormatException,
      );
      expect(
        () => OriginalSyncJournalState.fromJson(copy()..['unknown'] = true),
        throwsFormatException,
      );
      expect(
        () => scope({'principalId': '9223372036854775808'}),
        throwsFormatException,
      );
      expect(
        () => scope({'principalId': '09007199254740993'}),
        throwsFormatException,
      );
      expect(() => scope({'collabType': 0.0}), throwsFormatException);
      expect(
        () => scope({'origin': 'https://example.com/path'}),
        throwsFormatException,
      );
    },
  );
}
