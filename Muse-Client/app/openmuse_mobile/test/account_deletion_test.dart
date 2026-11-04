import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/account_deletion.dart';

void main() {
  test('deletes the current user with its bearer token', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final handled = server.first.then((request) async {
      expect(request.method, 'DELETE');
      expect(request.uri.path, '/api/user');
      expect(
        request.headers.value(HttpHeaders.authorizationHeader),
        'Bearer release-test-token',
      );
      request.response.headers.contentType = ContentType.json;
      request.response.write('{"code":0,"message":""}');
      await request.response.close();
    });
    try {
      await deleteOpenMuseAccount(
        cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'release-test-token',
      );
      await handled;
    } finally {
      await server.close(force: true);
    }
  });

  test('does not treat a server error as completed deletion', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final handled = server.first.then((request) async {
      request.response.statusCode = HttpStatus.serviceUnavailable;
      await request.response.close();
    });
    try {
      await expectLater(
        deleteOpenMuseAccount(
          cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
          accessToken: () async => 'release-test-token',
        ),
        throwsA(isA<AccountDeletionException>()),
      );
      await handled;
    } finally {
      await server.close(force: true);
    }
  });
}
