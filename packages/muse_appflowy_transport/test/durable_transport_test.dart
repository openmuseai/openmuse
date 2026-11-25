import 'dart:async';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:muse_appflowy_transport/muse_appflowy_transport.dart';
import 'package:test/test.dart';
import 'original_sync_http_contract_test.dart' as fixture;
import 'write_capabilities_test.dart' as discovery;

void main() {
  late PendingOriginalSync op;
  setUp(
    () async => op = await fixture.prepare(
      OriginalSyncJournal(OriginalSyncJournalState.empty(), (_) async {}),
    ),
  );
  AppFlowyObjectTransport port(
    Future<http.Response> Function(http.Request) send, {
    Future<String> Function()? token,
    void Function()? authority,
  }) => AppFlowyObjectTransport(
    origin: Uri.parse(op.scope.origin),
    workspaceId: op.scope.workspaceId,
    boundObjectId: op.scope.objectId,
    collabType: 0,
    binding: const CollaborationBinding(
      protocol: 'appflowy.full-sync/1',
      engine: 'test',
      encoding: 'yrs.update-v1',
      schema: 'doc',
      epoch: 0,
    ),
    accessToken: token ?? () async => 'synthetic',
    deviceId: 'durable-test',
    checkAuthority: authority,
    client: MockClient(send),
  );
  test(
    'durable POST carries exact immutable body and expected principal once',
    () async {
      var calls = 0;
      final p = port((request) async {
        calls++;
        expect(request.method, 'POST');
        expect(request.url, fixture.endpoint(op));
        expect(request.bodyBytes, op.request);
        expect(request.followRedirects, isFalse);
        expect(
          request.headers['X-OpenMuse-Sync-Principal'],
          op.scope.principalId,
        );
        expect(request.headers['X-OpenMuse-Sync-Digest'], op.requestSha256);
        expect(request.headers['X-OpenMuse-Sync-Operation'], op.operationId);
        return http.Response(jsonEncode(fixture.response(op)), 200);
      });
      addTearDown(p.close);
      expect(
        (await p.exchangeOriginal(op)).receipt.operationId,
        op.operationId,
      );
      expect(calls, 1);
    },
  );
  test('uncertain POST is never retried by the HTTP adapter', () async {
    var calls = 0;
    final p = port((_) async {
      calls++;
      throw http.ClientException('synthetic disconnected socket');
    });
    addTearDown(p.close);
    await expectLater(
      p.exchangeOriginal(op),
      throwsA(isA<http.ClientException>()),
    );
    expect(calls, 1);
  });
  for (final close in [true, false]) {
    test(
      'awaited token cannot send after ${close ? 'close' : 'authority loss'}',
      () async {
        final gate = Completer<String>(), entered = Completer<void>();
        var active = true, calls = 0;
        final p = port(
          (_) async {
            calls++;
            throw StateError('must not send');
          },
          token: () {
            entered.complete();
            return gate.future;
          },
          authority: () {
            if (!active) throw const AppFlowyTransportFailure('FORBIDDEN');
          },
        );
        addTearDown(p.close);
        final result = expectLater(
          p.exchangeOriginal(op),
          discovery.failure(close ? 'SESSION_CLOSED' : 'FORBIDDEN'),
        );
        await entered.future;
        if (close) {
          p.close();
        } else {
          active = false;
        }
        gate.complete('synthetic');
        await result;
        expect(calls, 0);
      },
    );
  }
  test(
    'readonly exact receipt lookup is GET and does not retire journal itself',
    () async {
      final p = port((request) async {
        expect(request.method, 'GET');
        expect(request.url, fixture.endpoint(op, lookup: true));
        expect(request.bodyBytes, isEmpty);
        return http.Response(
          jsonEncode({
            'code': 0,
            'data': {'found': true, 'receipt': fixture.wire(op)},
          }),
          200,
        );
      });
      addTearDown(p.close);
      expect(
        (await p.readOriginalSyncReceipt(op))!.scope.principalId,
        op.scope.principalId,
      );
    },
  );
}
