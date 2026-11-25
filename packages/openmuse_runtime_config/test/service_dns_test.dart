import 'dart:io';
import 'dart:convert';
import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config_io.dart';

void main() {
  for (final scheme in ['https', 'wss']) {
    test(
      'custom $scheme connects TLS before HTTP or credentials can be sent',
      () async {
        final server = await ServerSocket.bind(InternetAddress.loopbackIPv4, 0);
        final observed = Completer<List<int>>();
        final subscription = server.listen((socket) {
          socket.listen((bytes) {
            if (!observed.isCompleted) observed.complete(bytes);
            socket.destroy();
          });
        });
        final client = ServiceDnsOverrides(
          hosts: {},
          resolver: Uri.parse('https://dns.alidns.com/resolve'),
        ).createHttpClient(null);
        try {
          await expectLater(() async {
            if (scheme == 'wss') {
              final socket = await WebSocket.connect(
                'wss://127.0.0.1:${server.port}/events',
                customClient: client,
                headers: {'Authorization': 'Bearer test-secret-must-not-leak'},
              ).timeout(const Duration(seconds: 5));
              await socket.close();
              return;
            }
            final request = await client
                .postUrl(Uri.parse('https://127.0.0.1:${server.port}/token'))
                .timeout(const Duration(seconds: 5));
            request.headers.set(
              'Authorization',
              'Bearer test-secret-must-not-leak',
            );
            request.write('test-private-password');
            await request.close();
          }(), throwsA(anything));
          final bytes = await observed.future.timeout(
            const Duration(seconds: 5),
          );
          expect(
            bytes.first,
            22,
            reason: 'TLS handshake record, not an HTTP POST',
          );
          expect(
            utf8.decode(bytes, allowMalformed: true),
            isNot(contains('test-private-password')),
          );
          expect(
            utf8.decode(bytes, allowMalformed: true),
            isNot(contains('test-secret-must-not-leak')),
          );
        } finally {
          client.close(force: true);
          await subscription.cancel();
          await server.close();
        }
      },
    );
  }
  test(
    'DNS recovery accepts bounded IP answers and rejects invalid envelopes',
    () {
      expect(
        decodeServiceDnsAnswers(
          '{"Status":0,"Answer":[{"type":1,"data":"180.184.29.195"}]}',
        ).single.address,
        '180.184.29.195',
      );
      for (final response in [
        '{"Status":3,"Answer":[]}',
        '{"Status":0,"Answer":[{"type":1,"data":"127.0.0.1"}]}',
        '{"Status":0,"Answer":[{"type":1,"data":"user:password@evil"}]}',
      ]) {
        expect(() => decodeServiceDnsAnswers(response), throwsFormatException);
      }
    },
  );
  test('an embedding network override is preserved', () {
    final previous = HttpOverrides.current;
    final custom = _ExistingOverrides();
    HttpOverrides.global = custom;
    try {
      installServiceDnsFallback(
        origins: {Uri.parse('https://cn.openmuseai.com')},
        resolver: Uri.parse('https://dns.alidns.com/resolve'),
      );
      expect(identical(HttpOverrides.current, custom), isTrue);
    } finally {
      HttpOverrides.global = previous;
    }
  });
}

final class _ExistingOverrides extends HttpOverrides {}
