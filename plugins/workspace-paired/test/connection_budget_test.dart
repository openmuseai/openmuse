import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  test(
    'connect budget bounds slow responses and trace carries no credentials',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final log = <String>[];
      ConnectionTrace.sink = log.add;
      final client = PairedDesktopClient(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'private-token-for-test',
        deviceRef: 'private-device',
        allowInsecureLoopback: true,
        requestTimeout: const Duration(seconds: 5),
        connectionBudget: const Duration(milliseconds: 50),
      );
      server.listen((request) async {
        expect(request.headers.value('traceparent'), isNotNull);
        await Future<void>.delayed(const Duration(milliseconds: 200));
        try {
          await request.response.close();
        } on Object {
          /* client closed */
        }
      });
      try {
        await expectLater(
          client.connectSameAccount(targetDeviceRef: 'private-target'),
          throwsA(
            isA<PairedDesktopFailure>().having(
              (e) => e.code,
              'code',
              'PAIR_TIMEOUT',
            ),
          ),
        );
        expect(
          log.any(
            (line) =>
                line.contains('pair.connect') && line.contains('"ok":false'),
          ),
          true,
        );
        expect(log.join(), isNot(contains('private-token-for-test')));
        expect(log.join(), isNot(contains('private-device')));
      } finally {
        client.close();
        await server.close(force: true);
        ConnectionTrace.sink = null;
      }
    },
  );
}
