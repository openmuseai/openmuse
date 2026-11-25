import 'package:test/test.dart';
import 'dart:typed_data';
import 'package:muse_original_sync/muse_original_sync.dart';
import 'package:muse_appflowy_transport/src/original_sync_http_contract.dart';

Future<PendingOriginalSync> prepare(OriginalSyncJournal j) => j.prepare(
  scope: OriginalSyncScope(
    origin: 'http://127.0.0.1:18001',
    principalId: '9007199254740993',
    workspaceId: '11111111-1111-2222-3333-444444444444',
    objectId: '22222222-1111-2222-3333-444444444444',
    collabType: 0,
    generation: '33333333-1111-2222-3333-444444444444',
    modelProfile: 'appflowy.original-collab/1',
    epoch: '0',
  ),
  operationId: 'aaaaaaaa-1111-2222-3333-444444444444',
  request: Uint8List.fromList([0, 1, 255, 3]),
  sentLocalEpoch: 7,
);

Map<String, Object?> wire(PendingOriginalSync p) => {
  'protocolProfile': 'openmuse.original-sync-receipt/1',
  'strength': 'original-pg-commit-v1',
  ...(p.scope.toJson()..remove('origin')),
  'operationId': p.operationId,
  'requestSha256': p.requestSha256,
  'docStateSha256': 'a' * 64,
  'recordedAt': '2026-11-21T11:00:00Z',
};
Uri endpoint(PendingOriginalSync p, {bool lookup = false}) =>
    Uri.parse(
      '${p.scope.origin}/api/workspace/v1/${p.scope.workspaceId}/collab/${p.scope.objectId}/${lookup ? 'sync-receipt/${p.operationId}' : 'durable-sync'}',
    ).replace(
      queryParameters: {
        'collab_type': '${p.scope.collabType}',
        'generation': p.scope.generation,
        'model_profile': p.scope.modelProfile,
        'epoch': p.scope.epoch,
        if (lookup) 'request_sha256': p.requestSha256,
      },
    );
Map<String, Object?> response(
  PendingOriginalSync p, {
  Map<String, Object?> changes = const {},
}) => {
  'code': 0,
  'data': {
    'receipt': {...wire(p), ...changes},
    'missingUpdateBase64': 'AAA=',
    'propagation': 'pending',
  },
};
void main() {
  late OriginalSyncJournal journal;
  late PendingOriginalSync operation;
  setUp(() async {
    journal = OriginalSyncJournal(
      OriginalSyncJournalState.empty(),
      (_) async {},
    );
    operation = await prepare(journal);
  });
  test(
    'explicit commit-strength metadata preserves int64 principal and retires only sent epoch',
    () async {
      final result = parseOriginalDurableExchange(
        response(operation),
        responseUri: endpoint(operation),
        operation: operation,
      );
      expect(result.receipt.scope.principalId, '9007199254740993');
      expect(result.missingUpdate, [0, 0]);
      expect(await journal.acknowledge(result.receipt), 7);
      expect(journal.state.pending, isNull);
    },
  );
  test(
    'legacy acceptance and forged commit strength never retire pending operation',
    () {
      for (final r in [
        <String, Object?>{'code': 0, 'data': {}},
        response(operation, changes: {'strength': 'redis-accepted'}),
        response(
          operation,
          changes: {'protocolProfile': 'appflowy.full-sync/1'},
        ),
      ]) {
        expect(
          () => parseOriginalDurableExchange(
            r,
            responseUri: endpoint(operation),
            operation: operation,
          ),
          throwsFormatException,
        );
      }
      expect(journal.state.pending, same(operation));
    },
  );
  test('origin, target and protocol route are bound before normalization', () {
    final uri = endpoint(operation);
    for (final other in [
      uri.replace(port: 18002),
      uri.replace(path: '/api/workspace/legacy/full-sync'),
      uri.replace(queryParameters: {...uri.queryParameters, 'epoch': '1'}),
    ]) {
      expect(
        () => parseOriginalDurableExchange(
          response(operation),
          responseUri: other,
          operation: operation,
        ),
        throwsFormatException,
      );
    }
    expect(journal.state.pending, same(operation));
  });
  test(
    'cross-scope receipt or lossy numeric identifiers cannot acknowledge',
    () {
      for (final changes in [
        {'origin': 'http://evil.invalid'},
        {'objectId': '44444444-1111-2222-3333-444444444444'},
        {'principalId': '9007199254740994'},
        {'epoch': '1'},
        {'operationId': '44444444-1111-2222-3333-444444444444'},
        {'requestSha256': 'b' * 64},
        {'principalId': 9007199254740993},
        {'epoch': 0},
      ]) {
        if (changes.containsKey('origin')) continue;
        expect(
          () => parseOriginalDurableExchange(
            response(operation, changes: changes),
            responseUri: endpoint(operation),
            operation: operation,
          ),
          throwsFormatException,
        );
      }
      expect(journal.state.pending, same(operation));
    },
  );
  test(
    'readonly lookup absence is not a receipt; exact positive lookup is accepted',
    () {
      expect(
        parseOriginalReceiptLookup(
          {
            'code': 0,
            'data': {'found': false, 'receipt': null},
          },
          responseUri: endpoint(operation, lookup: true),
          operation: operation,
        ),
        isNull,
      );
      final receipt = parseOriginalReceiptLookup(
        {
          'code': 0,
          'data': {'found': true, 'receipt': wire(operation)},
        },
        responseUri: endpoint(operation, lookup: true),
        operation: operation,
      );
      expect(receipt!.operationId, operation.operationId);
      expect(journal.state.pending, same(operation));
      expect(
        () => parseOriginalReceiptLookup(
          {
            'code': 0,
            'data': {'found': false, 'receipt': wire(operation)},
          },
          responseUri: endpoint(operation, lookup: true),
          operation: operation,
        ),
        throwsFormatException,
      );
    },
  );
  test('malformed exchange content cannot yield an acknowledgeable result', () {
    for (final changes in [
      {'missingUpdateBase64': '%%%%'},
      {'missingUpdateBase64': ''},
      {'propagation': 'accepted'},
    ]) {
      final r = response(operation);
      r['data'] = {...(r['data'] as Map<String, Object?>), ...changes};
      expect(
        () => parseOriginalDurableExchange(
          r,
          responseUri: endpoint(operation),
          operation: operation,
        ),
        throwsFormatException,
      );
    }
    expect(journal.state.pending, same(operation));
  });
}
