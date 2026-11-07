import 'dart:convert';
import 'dart:typed_data';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:muse_appflowy_transport/muse_appflowy_transport.dart';
import 'package:test/test.dart';

const id = '041c01d0-831a-408a-b8fa-190d40dd9e94';
const workspace = '041c01d0-831a-408a-b8fa-190d40dd9e93';
void main() {
  test(
    'a broken idempotent GET is retried only once with fresh authorization',
    () async {
      var calls = 0, tokens = 0;
      final t = AppFlowyObjectTransport(
        origin: Uri.parse('https://cloud.example.test'),
        workspaceId: workspace,
        deviceId: 'retry-test',
        collabType: 4,
        accessToken: () async => 'test-${++tokens}',
        binding: const CollaborationBinding(
          protocol: 'appflowy.full-sync/1',
          engine: 'test',
          encoding: 'yrs.update-v1',
          schema: 'row',
          epoch: 0,
        ),
        client: MockClient((request) async {
          calls++;
          expect(request.followRedirects, isFalse);
          expect(request.headers['Authorization'], 'Bearer test-$calls');
          if (calls == 1) throw http.ClientException('closed before headers');
          return http.Response('{"code":0,"data":{"doc_state":[1,2]}}', 200);
        }),
      );
      expect(await t.readState(id), [1, 2]);
      expect(calls, 2);
      expect(tokens, 2);
      t.close();
    },
  );
  test(
    'read retry is bounded; denied reads and ambiguous writes are never replayed',
    () async {
      for (final mode in ['broken-get', 'forbidden-get', 'write']) {
        var calls = 0;
        final t = AppFlowyObjectTransport(
          origin: Uri.parse('https://cloud.example.test'),
          workspaceId: workspace,
          deviceId: 'retry-boundary',
          collabType: 0,
          accessToken: () async => 't',
          binding: const CollaborationBinding(
            protocol: 'appflowy.full-sync/1',
            engine: 'test',
            encoding: 'yrs.update-v1',
            schema: 'doc',
            epoch: 0,
          ),
          client: MockClient((request) async {
            calls++;
            if (mode == 'forbidden-get') return http.Response('', 403);
            throw http.ClientException('closed');
          }),
        );
        await expectLater(
          mode == 'write' ? t.exchange(id, Uint8List(0)) : t.readState(id),
          throwsA(
            anyOf(isA<http.ClientException>(), isA<AppFlowyTransportFailure>()),
          ),
        );
        expect(calls, mode == 'broken-get' ? 2 : 1);
        t.close();
      }
    },
  );

  test(
    'content bindings and wire types are injected, document/row payloads remain opaque',
    () async {
      for (final type in [0, 1, 4]) {
        final binding = CollaborationBinding(
          protocol: 'appflowy.full-sync/1',
          engine: 'test-original-engine',
          encoding: 'yrs.update-v1',
          schema: 'opaque-domain-$type',
          epoch: 0,
        );
        final requests = <http.Request>[];
        final transport = AppFlowyObjectTransport(
          origin: Uri.parse('https://cloud.example.test'),
          workspaceId: workspace,
          deviceId: 'test',
          accessToken: () async => 'test-token',
          binding: binding,
          collabType: type,
          client: MockClient((r) async {
            requests.add(r);
            expect(r.followRedirects, isFalse);
            if (r.method == 'GET') {
              expect(r.url.queryParameters['collab_type'], '$type');
              return http.Response(
                jsonEncode({
                  'code': 0,
                  'data': {
                    'doc_state': [1, 2],
                  },
                }),
                200,
              );
            }
            if (r.url.path.endsWith('full-sync'))
              return http.Response.bytes(
                [4, 5],
                200,
                headers: {'content-type': 'application/octet-stream'},
              );
            expect((jsonDecode(r.body) as Map)['collab_type'], type);
            return http.Response('{"code":0}', 200);
          }),
        );
        expect(transport.binding.compatibleWith(binding), isTrue);
        expect(await transport.readState(id), [1, 2]);
        await transport.initializeState(id, Uint8List.fromList([3]));
        expect(await transport.exchange(id, Uint8List.fromList([6])), [4, 5]);
        expect(
          requests.every(
            (r) => r.headers['Authorization'] == 'Bearer test-token',
          ),
          isTrue,
        );
        expect(requests.last.bodyBytes, [6]);
        transport.close();
      }
    },
  );
  test(
    'invalid object routes and redirect responses fail without followup',
    () async {
      var calls = 0;
      final t = AppFlowyObjectTransport(
        origin: Uri.parse('http://127.0.0.1:18000'),
        workspaceId: workspace,
        deviceId: 'test',
        accessToken: () async => 't',
        binding: const CollaborationBinding(
          protocol: 'appflowy.full-sync/1',
          engine: 'test',
          encoding: 'yrs.update-v1',
          schema: 'test',
          epoch: 0,
        ),
        collabType: 0,
        client: MockClient((r) async {
          calls++;
          return http.Response(
            '',
            302,
            headers: {'location': 'https://outside.test'},
          );
        }),
      );
      await expectLater(
        t.readState('../secret'),
        throwsA(isA<AppFlowyTransportFailure>()),
      );
      expect(calls, 0);
      await expectLater(
        t.exchange(id, Uint8List(0)),
        throwsA(isA<AppFlowyTransportFailure>()),
      );
      expect(calls, 1);
      t.close();
    },
  );
  test(
    'original tombstone is a typed terminal result, never absence permitting recreation',
    () async {
      final t = AppFlowyObjectTransport(
        origin: Uri.parse('https://cloud.example.test'),
        workspaceId: workspace,
        deviceId: 'test',
        accessToken: () async => 't',
        collabType: 4,
        binding: const CollaborationBinding(
          protocol: 'appflowy.full-sync/1',
          engine: 'test',
          encoding: 'yrs.update-v1',
          schema: 'row',
          epoch: 0,
        ),
        client: MockClient(
          (r) async => http.Response(
            '{"code":-4,"message":"deleted"}',
            200,
            headers: {'content-type': 'application/json'},
          ),
        ),
      );
      final deleted = throwsA(
        isA<AppFlowyTransportFailure>().having(
          (e) => e.code,
          'code',
          'OBJECT_DELETED',
        ),
      );
      await expectLater(t.readState(id), deleted);
      await expectLater(t.exchange(id, Uint8List(0)), deleted);
      await expectLater(t.initializeState(id, Uint8List(0)), deleted);
      t.close();
    },
  );
}
