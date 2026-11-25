import 'dart:convert';
import 'dart:io';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:test/test.dart';

void main() {
  test(
    'shared initialization is single-flight and retryable after failure',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var requests = 0;
      server.listen((request) async {
        requests++;
        request.response.headers.contentType = ContentType.json;
        if (requests == 1) {
          request.response.statusCode = 503;
          request.response.write(jsonEncode({'error': 'unavailable'}));
        } else {
          request.response.write(
            jsonEncode({
              'protocolVersion': 1,
              'dshVersion': DshGatewayHello.supportedDshVersion,
              'capabilities': <String>[],
              'compatibility': {
                'nativeConversationCompatible': true,
                'fallbackRequired': false,
                'unsupportedPlugins': [],
              },
            }),
          );
        }
        await request.response.close();
      });
      final client = DshNativeGatewayClient(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        bootstrapPath: '',
        allowInsecureLoopback: true,
      );
      try {
        await expectLater(
          client.initialize(),
          throwsA(isA<DshNativeGatewayException>()),
        );
        final hellos = await Future.wait([
          client.initialize(),
          client.initialize(),
        ]);
        expect(hellos.every((hello) => hello.contractSupported), true);
        await client.initialize();
        expect(requests, 2);
      } finally {
        client.close();
        await server.close(force: true);
      }
    },
  );
}
