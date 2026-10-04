import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:test/test.dart';

void main() {
  test(
    'inactive conversations stop polling and resume with cached rows',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final releaseFollows = Completer<void>();
      var followRequests = 0;
      var sessionRequests = 0;
      server.listen((request) async {
        switch (request.uri.path) {
          case '/openmuse-native/v1/session/follow':
            followRequests++;
            request.response
              ..bufferOutput = false
              ..headers.contentType = ContentType('text', 'event-stream')
              ..write(
                'event: frame\ndata: ${jsonEncode({
                  'type': 'snapshot',
                  'header': {'id': 's-1'},
                  'cursor': 1,
                  'records': [
                    _record('user/message', 1, {
                      'content': [
                        {'type': 'text', 'text': 'cached message'},
                      ],
                    }),
                  ],
                  'hasMore': false,
                  'projections': {'asOfSeq': 1, 'values': <String, Object?>{}},
                })}\n\n',
              );
            await request.response.flush();
            await releaseFollows.future;
            await request.response.close();
          case '/openmuse-native/v1/sessions':
            sessionRequests++;
            _json(request.response, {
              'items': [
                {
                  'sessionId': 's-1',
                  'updatedAt': 1,
                  'running': false,
                  'blank': false,
                  'headSeq': 1,
                  'projections': {'asOfSeq': 1, 'values': <String, Object?>{}},
                },
              ],
            });
          default:
            request.response.statusCode = HttpStatus.notFound;
            await request.response.close();
        }
      });
      final client = DshNativeGatewayClient(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        bootstrapPath: '',
        allowInsecureLoopback: true,
      );
      final controller = DshConversationController(
        client: client,
        reconciliationInterval: const Duration(milliseconds: 20),
      );
      await controller.start('s-1');
      await _until(() => controller.store.snapshot.rows.isNotEmpty);
      controller.pause();
      await Future<void>.delayed(const Duration(milliseconds: 100));
      final pausedRequests = sessionRequests;
      await Future<void>.delayed(const Duration(milliseconds: 100));
      expect(sessionRequests, pausedRequests);
      expect(controller.store.snapshot.rows.single.text, 'cached message');

      controller.resume();
      await _until(() => followRequests >= 2);
      expect(controller.store.snapshot.rows.single.text, 'cached message');
      expect(controller.store.snapshot.phase, DshConnectionPhase.live);

      releaseFollows.complete();
      await controller.dispose();
      await server.close(force: true);
    },
  );

  test(
    'durable page repairs a silent follow stream and retires pending',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final releaseFollow = Completer<void>();
      var headSeq = 0;
      var records = <Map<String, Object?>>[];

      server.listen((request) async {
        switch (request.uri.path) {
          case '/openmuse-native/v1/session/follow':
            request.response
              ..bufferOutput = false
              ..headers.contentType = ContentType(
                'text',
                'event-stream',
                charset: 'utf-8',
              )
              ..write(
                'event: frame\ndata: ${jsonEncode({
                  'type': 'snapshot',
                  'header': {'id': 's-1'},
                  'cursor': 0,
                  'records': <Object?>[],
                  'hasMore': false,
                  'projections': {'asOfSeq': 0, 'values': <String, Object?>{}},
                })}\n\n',
              );
            await request.response.flush();
            await releaseFollow.future;
            await request.response.close();
          case '/openmuse-native/v1/sessions':
            _json(request.response, {
              'items': [
                {
                  'sessionId': 's-1',
                  'updatedAt': 1,
                  'running': false,
                  'blank': headSeq == 0,
                  'projections': {
                    'asOfSeq': headSeq,
                    'values': {'title': 'Recovery'},
                  },
                },
              ],
            });
          case '/openmuse-native/v1/session/page':
            final body = jsonDecode(await utf8.decodeStream(request)) as Map;
            final throughSeq = body['throughSeq'] as int;
            _json(request.response, {
              'records': records
                  .where(
                    (record) =>
                        ((record['event'] as Map)['seq'] as int) <= throughSeq,
                  )
                  .toList(growable: false),
              'hasMore': false,
            });
          case '/openmuse-native/v1/session/prompt':
            final body = jsonDecode(await utf8.decodeStream(request));
            final requestId = (body as Map)['requestId'] as String;
            headSeq = 4;
            records = [
              _record('turn/start', 1, {'turn': 1}),
              _record('user/message', 2, {
                'content': [
                  {'type': 'text', 'text': 'hello'},
                ],
                'source': {'kind': 'user', 'rpcId': requestId},
              }, surfaceOp: 'append'),
              _record('assistant/message', 3, {
                'message': {
                  'content': [
                    {'type': 'text', 'text': 'recovered'},
                  ],
                },
              }, surfaceOp: 'append'),
              _record('turn/end', 4, {
                'turn': 1,
                'reason': {'kind': 'completed'},
              }),
            ];
            _json(request.response, {'accepted': true});
          default:
            request.response.statusCode = HttpStatus.notFound;
            await request.response.close();
        }
      });

      final client = DshNativeGatewayClient(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        bootstrapPath: '',
        allowInsecureLoopback: true,
      );
      final controller = DshConversationController(
        client: client,
        reconciliationInterval: const Duration(milliseconds: 20),
      );
      await controller.start('s-1');
      await controller.send('hello');

      await _until(() {
        final snapshot = controller.store.snapshot;
        return snapshot.cursor == 4 &&
            snapshot.rows.any((row) => row.text == 'recovered');
      });

      expect(controller.store.hasPendingPrompts, isFalse);
      expect(controller.store.snapshot.rows.map((row) => row.text), [
        'hello',
        'recovered',
      ]);

      releaseFollow.complete();
      await controller.dispose();
      await server.close(force: true);
    },
  );
}

Map<String, Object?> _record(
  String type,
  int seq,
  Map<String, Object?> data, {
  Object? surfaceOp,
}) => {
  'type': 'event',
  'event': {
    'type': type,
    'seq': seq,
    'time': seq,
    'data': data,
    if (surfaceOp != null) 'surfaceOp': surfaceOp,
  },
};

void _json(HttpResponse response, Map<String, Object?> value) {
  response.headers.contentType = ContentType.json;
  response.write(jsonEncode(value));
  unawaited(response.close());
}

Future<void> _until(bool Function() predicate) async {
  final deadline = DateTime.now().add(const Duration(seconds: 3));
  while (!predicate()) {
    if (DateTime.now().isAfter(deadline)) {
      fail('condition was not reached before timeout');
    }
    await Future<void>.delayed(const Duration(milliseconds: 10));
  }
}
