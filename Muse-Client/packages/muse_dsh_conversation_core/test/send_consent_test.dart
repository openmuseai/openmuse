import 'dart:async';
import 'dart:io';
import 'package:test/test.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

void main() {
  test(
    'declining AI sharing sends no prompt and creates no pending message',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var requests = 0;
      server.listen((r) async {
        requests++;
        r.response.write('{"accepted":true}');
        await r.response.close();
      });
      final store = DshConversationStore()..connecting('review-session');
      final controller = DshConversationController(
        client: DshNativeGatewayClient(
          origin: Uri.parse('http://127.0.0.1:${server.port}'),
          bootstrapPath: '',
          allowInsecureLoopback: true,
        ),
        store: store,
        approveSend: (_, text) async => false,
      );
      await expectLater(
        controller.send('Private review draft'),
        throwsA(isA<DshSendDeclined>()),
      );
      expect(requests, 0);
      await controller.dispose();
      await server.close(force: true);
    },
  );
  test(
    'recipient changes during consent cannot receive the approved prompt',
    () async {
      final store = DshConversationStore()..connecting('first');
      final controller = DshConversationController(
        client: DshNativeGatewayClient(
          origin: Uri.parse('http://127.0.0.1:9'),
          bootstrapPath: '',
          allowInsecureLoopback: true,
        ),
        store: store,
        approveSend: (_, text) async {
          store.connecting('different-recipient');
          return true;
        },
      );
      await expectLater(
        controller.send('Private review draft'),
        throwsStateError,
      );
      await controller.dispose();
    },
  );
  test('an explicitly approved prompt uses the normal gateway', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    var requests = 0;
    server.listen((r) async {
      await r.drain<void>();
      requests++;
      r.response.headers.contentType = ContentType.json;
      r.response.write('{"accepted":true}');
      await r.response.close();
    });
    final store = DshConversationStore()..connecting('review-session');
    final controller = DshConversationController(
      client: DshNativeGatewayClient(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        bootstrapPath: '',
        allowInsecureLoopback: true,
      ),
      store: store,
      approveSend: (_, text) async => true,
    );
    await controller.send('Approved review draft');
    expect(requests, 1);
    await controller.dispose();
    await server.close(force: true);
  });
}
