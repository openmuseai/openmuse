import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';

void main() {
  test('production config rejects insecure and path-bearing origins', () {
    expect(
      () => GoTrueClientConfig(origin: Uri.parse('http://example.com')),
      throwsA(isA<AuthFailure>()),
    );
    expect(
      () => GoTrueClientConfig(origin: Uri.parse('https://example.com/auth')),
      throwsA(isA<AuthFailure>()),
    );
    expect(
      GoTrueClientConfig(origin: Uri.parse('https://example.com')).origin,
      Uri.parse('https://example.com'),
    );
  });

  test('password and refresh requests follow GoTrue contract', () async {
    final requests = <Map<String, Object?>>[];
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final serving = server.forEach((request) async {
      final body = request.method == 'POST'
          ? jsonDecode(await utf8.decoder.bind(request).join()) as Map
          : <String, Object?>{};
      requests.add({
        'method': request.method,
        'path': request.uri.path,
        'grant': request.uri.queryParameters['grant_type'],
        'body': body.cast<String, Object?>(),
      });
      request.response.headers.contentType = ContentType.json;
      request.response.write(
        jsonEncode({
          'access_token': 'access-${requests.length}',
          'refresh_token': 'refresh-${requests.length}',
          'expires_in': 3600,
          'user': {'id': 'user-1', 'email': 'muse@example.com'},
        }),
      );
      await request.response.close();
    });
    final client = GoTrueHttpClient(
      config: GoTrueClientConfig(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        allowInsecureLoopback: true,
      ),
      clock: () => DateTime.utc(2026, 9, 29),
    );

    final signedIn = await client.signInWithPassword(
      ' muse@example.com ',
      'secret',
    );
    final refreshed = await client.refresh(signedIn.refreshToken);
    await server.close(force: true);
    await serving;

    expect(signedIn.accessToken, 'access-1');
    expect(refreshed.refreshToken, 'refresh-2');
    expect(requests[0], {
      'method': 'POST',
      'path': '/token',
      'grant': 'password',
      'body': {'email': 'muse@example.com', 'password': 'secret'},
    });
    expect(requests[1], {
      'method': 'POST',
      'path': '/token',
      'grant': 'refresh_token',
      'body': {'refresh_token': 'refresh-1'},
    });
  });

  test(
    'unauthorized password response does not expose response body',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final serving = server.forEach((request) async {
        request.response.statusCode = 400;
        request.response.write(
          jsonEncode({'message': 'secret server diagnostic and token'}),
        );
        await request.response.close();
      });
      final client = GoTrueHttpClient(
        config: GoTrueClientConfig(
          origin: Uri.parse('http://127.0.0.1:${server.port}'),
          allowInsecureLoopback: true,
        ),
      );

      AuthFailure? captured;
      try {
        await client.signInWithPassword('muse@example.com', 'wrong');
      } on AuthFailure catch (error) {
        captured = error;
      }
      await server.close(force: true);
      await serving;

      expect(captured?.kind, AuthFailureKind.invalidCredentials);
      expect(captured.toString(), isNot(contains('secret server diagnostic')));
    },
  );
}
