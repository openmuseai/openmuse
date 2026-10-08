import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';

void main() {
  test('production config rejects insecure and unsafe origins', () {
    expect(
      () => GoTrueClientConfig(origin: Uri.parse('http://example.com')),
      throwsA(isA<AuthFailure>()),
    );
    expect(
      () => GoTrueClientConfig(
        origin: Uri.parse('https://user:secret@example.com/gotrue'),
      ),
      throwsA(isA<AuthFailure>()),
    );
    expect(
      () => GoTrueClientConfig(
        origin: Uri.parse('https://example.com/gotrue?next=1'),
      ),
      throwsA(isA<AuthFailure>()),
    );
    expect(
      GoTrueClientConfig(origin: Uri.parse('https://example.com')).origin,
      Uri.parse('https://example.com'),
    );
    expect(
      GoTrueClientConfig(
        origin: Uri.parse('https://example.com/gotrue'),
      ).origin,
      Uri.parse('https://example.com/gotrue'),
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

  test(
    'email passcode requests follow GoTrue otp and verify contracts',
    () async {
      final requests = <Map<String, Object?>>[];
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final serving = server.forEach((request) async {
        final body = jsonDecode(await utf8.decoder.bind(request).join()) as Map;
        requests.add({
          'path': request.uri.path,
          'body': body.cast<String, Object?>(),
        });
        if (request.uri.path == '/verify') {
          request.response.headers.contentType = ContentType.json;
          request.response.write(
            jsonEncode({
              'access_token': 'access-code',
              'refresh_token': 'refresh-code',
              'expires_in': 3600,
              'user': {'id': 'user-1', 'email': 'muse@example.com'},
            }),
          );
        }
        await request.response.close();
      });
      final client = GoTrueHttpClient(
        config: GoTrueClientConfig(
          origin: Uri.parse('http://127.0.0.1:${server.port}'),
          allowInsecureLoopback: true,
        ),
      );

      await client.requestSignInCode(' muse@example.com ');
      final session = await client.signInWithCode(
        ' muse@example.com ',
        ' 123456 ',
      );
      await server.close(force: true);
      await serving;

      expect(session.accessToken, 'access-code');
      expect(requests, [
        {
          'path': '/otp',
          'body': {'email': 'muse@example.com', 'create_user': false},
        },
        {
          'path': '/verify',
          'body': {
            'type': 'email',
            'email': 'muse@example.com',
            'token': '123456',
          },
        },
      ]);
    },
  );

  test('account bootstrap calls the verify endpoint once', () async {
    final seen = <String>[];
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final serving = server.forEach((request) async {
      seen.add(request.uri.path);
      request.response.headers.contentType = ContentType.json;
      request.response.write(
        jsonEncode({
          'code': 0,
          'data': {'is_new': false},
        }),
      );
      await request.response.close();
    });
    final bootstrapper = AppFlowyAccountBootstrapper(
      cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
      allowInsecureLoopback: true,
    );
    await bootstrapper.bootstrap(
      GoTrueSession(
        accessToken: 'access token',
        refreshToken: 'refresh',
        expiresAt: DateTime.utc(2026, 10, 2),
        user: const GoTrueUser(id: 'user-1', email: 'muse@example.com'),
      ),
    );
    await server.close(force: true);
    await serving;
    expect(seen, ['/api/user/verify/access%20token']);
  });

  test('signup, confirmation and recovery use the GoTrue contracts', () async {
    final requests = <Map<String, Object?>>[];
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final serving = server.forEach((request) async {
      final body = jsonDecode(await utf8.decoder.bind(request).join()) as Map;
      requests.add({
        'method': request.method,
        'path': request.uri.path,
        'body': body.cast<String, Object?>(),
        'authorization': request.headers.value(HttpHeaders.authorizationHeader),
      });
      request.response.headers.contentType = ContentType.json;
      if (request.uri.path == '/signup') {
        request.response.write(
          jsonEncode({'id': 'user-1', 'email': 'muse@example.com'}),
        );
      } else if (request.uri.path == '/verify') {
        request.response.write(
          jsonEncode({
            'access_token': 'access-code',
            'refresh_token': 'refresh-code',
            'expires_in': 3600,
            'user': {'id': 'user-1', 'email': 'muse@example.com'},
          }),
        );
      } else {
        request.response.write('{}');
      }
      await request.response.close();
    });
    final client = GoTrueHttpClient(
      config: GoTrueClientConfig(
        origin: Uri.parse('http://127.0.0.1:${server.port}'),
        allowInsecureLoopback: true,
      ),
    );

    final registration = await client.signUp(' muse@example.com ', 'secret123');
    expect(registration.session, isNull);
    await client.resendSignUpCode('muse@example.com');
    await client.verifySignUpCode('muse@example.com', '123456');
    await client.requestPasswordRecovery('muse@example.com');
    final recovery = await client.verifyRecoveryCode(
      'muse@example.com',
      '654321',
    );
    await client.updatePassword(recovery.accessToken, 'new-secret');
    await server.close(force: true);
    await serving;

    expect(requests.map((r) => r['path']), [
      '/signup',
      '/resend',
      '/verify',
      '/recover',
      '/verify',
      '/user',
    ]);
    expect(requests[0]['body'], {
      'email': 'muse@example.com',
      'password': 'secret123',
    });
    expect(requests[1]['body'], {
      'type': 'signup',
      'email': 'muse@example.com',
    });
    expect(requests[2]['body'], {
      'type': 'signup',
      'email': 'muse@example.com',
      'token': '123456',
    });
    expect(requests[4]['body'], {
      'type': 'recovery',
      'email': 'muse@example.com',
      'token': '654321',
    });
    expect(requests[5]['authorization'], 'Bearer access-code');
    expect(requests[5]['body'], {'password': 'new-secret'});
  });
}
