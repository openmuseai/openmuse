import 'dart:async';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:muse_appflowy_realtime/muse_appflowy_realtime.dart';
import 'package:muse_appflowy_realtime/src/profile_identity.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  HttpOverrides.global = null;
  test(
    'original profile UID decoder preserves full i64 and transient provider rejects redirects and oversized bodies',
    () async {
      final codec = AppFlowyRealtimeCodec(NativeRealtimeWireCore());
      final client = MockClient((request) async {
        expect(request.method, 'GET');
        expect(request.url.path, '/api/user/profile');
        expect(request.followRedirects, false);
        expect(request.headers['Authorization'], 'Bearer synthetic-test-token');
        return http.Response(
          '{"code":0,"data":{"uid":9223372036854775807,"email":"never-render@example.invalid"}}',
          200,
        );
      });
      expect(
        await readAppFlowyActorUid(
          origin: Uri.parse('https://cloud.example.invalid'),
          token: 'synthetic-test-token',
          codec: codec,
          client: client,
        ),
        '9223372036854775807',
      );
      for (final response in [
        http.Response('', 302),
        http.Response('x' * 65537, 200),
        http.Response('{"code":0,"data":{"uid":"42"}}', 200),
      ]) {
        final rejected = MockClient((_) async => response);
        await expectLater(
          readAppFlowyActorUid(
            origin: Uri.parse('https://cloud.example.invalid'),
            token: 'synthetic-test-token',
            codec: codec,
            client: rejected,
          ),
          throwsA(isA<RealtimeFailure>()),
        );
        rejected.close();
      }
      final chunked = MockClient.streaming(
        (_, __) async =>
            http.StreamedResponse(Stream.value(List.filled(65537, 120)), 200),
      );
      await expectLater(
        readAppFlowyActorUid(
          origin: Uri.parse('https://cloud.example.invalid'),
          token: 'synthetic-test-token',
          codec: codec,
          client: chunked,
        ),
        throwsA(isA<RealtimeFailure>()),
      );
      chunked.close();
      client.close();
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
  test(
    'late identity response cannot cross reconnect epoch or closed connection',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final sockets = <WebSocket>[];
      final requests = server.listen((request) async {
        final socket = await WebSocketTransformer.upgrade(request);
        sockets.add(socket);
        socket.listen((_) {}, onError: (Object _) {});
      });
      final loaders = <Completer<String>>[], changes = <String?>[];
      final connection = AppFlowyRealtimeConnection(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        workspaceId: 'aeb48717-e939-42ba-9b12-9b22a282ac70',
        deviceId: 'identity-fixture',
        clientId: '9007199254740993',
        accessToken: () async => 'synthetic-test-token',
        codec: AppFlowyRealtimeCodec(NativeRealtimeWireCore()),
        actorIdentityLoader: (_) {
          final c = Completer<String>();
          loaders.add(c);
          return c.future;
        },
      );
      final sub = connection.actorChanges.listen(changes.add);
      await connection.connect();
      expect(connection.actorUid, isNull);
      expect(loaders, hasLength(1));
      await connection.disconnect();
      await connection.connect();
      expect(loaders, hasLength(2));
      loaders[1].complete('9007199254740993');
      await Future<void>.delayed(Duration.zero);
      expect(connection.actorUid, '9007199254740993');
      loaders[0].complete('7');
      await Future<void>.delayed(Duration.zero);
      expect(connection.actorUid, '9007199254740993');
      final disconnecting = connection.disconnect();
      expect(connection.actorUid, isNull);
      expect(connection.state, RealtimeConnectionState.disconnected);
      await disconnecting;
      await connection.connect();
      expect(loaders, hasLength(3));
      final closing = connection.close();
      expect(connection.state, RealtimeConnectionState.closed);
      await closing;
      loaders[2].complete('8');
      await Future<void>.delayed(Duration.zero);
      expect(connection.actorUid, isNull);
      expect(changes.whereType<String>(), ['9007199254740993']);
      await sub.cancel();
      for (final socket in sockets) {
        await socket.close();
      }
      await requests.cancel();
      await server.close(force: true);
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
}
