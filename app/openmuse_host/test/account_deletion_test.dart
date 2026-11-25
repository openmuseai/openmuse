import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/account_deletion.dart';

void main() {
  test(
    'account deletion needs an authenticated request and server confirmation',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var responseCode = 0;
      server.listen((request) async {
        expect(request.method, 'DELETE');
        expect(request.uri.path, '/api/user');
        expect(
          request.headers.value(HttpHeaders.authorizationHeader),
          'Bearer test-session',
        );
        request.response.write('{"code":$responseCode}');
        await request.response.close();
      });
      try {
        final origin = Uri.parse('http://127.0.0.1:${server.port}');
        await deleteHostAccount(origin: origin, token: 'test-session');
        responseCode = 1;
        await expectLater(
          deleteHostAccount(origin: origin, token: 'test-session'),
          throwsStateError,
        );
        await expectLater(
          deleteHostAccount(origin: origin, token: ''),
          throwsStateError,
        );
        await expectLater(
          deleteHostAccount(
            origin: Uri.parse('http://example.com'),
            token: 'test-session',
          ),
          throwsStateError,
        );
      } finally {
        await server.close(force: true);
      }
    },
  );
}
