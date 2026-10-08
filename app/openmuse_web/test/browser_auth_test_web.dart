import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';
import 'package:openmuse_web/auth/browser_auth.dart';

void main() {
  test('browser adapter uses the shared GoTrue password contract', () async {
    final provider = BrowserGoTrueProvider(
      origin: Uri.parse('https://auth.example.test/gotrue'),
      client: MockClient((request) async {
        expect(request.method, 'POST');
        expect(request.url.path, '/gotrue/token');
        expect(request.url.queryParameters['grant_type'], 'password');
        expect(jsonDecode(request.body), {
          'email': 'person@example.test',
          'password': 'fixture-secret',
        });
        return http.Response(
          jsonEncode({
            'access_token': 'access',
            'refresh_token': 'refresh',
            'expires_in': 3600,
            'user': {'id': 'person', 'email': 'person@example.test'},
          }),
          200,
        );
      }),
    );
    final session = await provider.signInWithPassword(
      ' person@example.test ',
      'fixture-secret',
    );
    expect(session.user.id, 'person');
    expect(session.accessToken, 'access');
  });

  test(
    'server failures stay distinguishable from invalid credentials',
    () async {
      final provider = BrowserGoTrueProvider(
        origin: Uri.parse('https://auth.example.test/gotrue'),
        client: MockClient(
          (_) async => http.Response('upstream unavailable', 504),
        ),
      );
      await expectLater(
        provider.signInWithPassword('person@example.test', 'fixture-secret'),
        throwsA(
          isA<AuthFailure>()
              .having((error) => error.kind, 'kind', AuthFailureKind.server)
              .having((error) => error.statusCode, 'status', 504),
        ),
      );
    },
  );
}
