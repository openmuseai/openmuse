import 'dart:convert';

import 'package:http/browser_client.dart';
import 'package:http/http.dart' as http;
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';
import 'package:web/web.dart' as web;

/// GoTrue is shared with the native apps; only the HTTP and storage ports differ.
final class BrowserGoTrueProvider implements GoTrueAuthProvider {
  BrowserGoTrueProvider({Uri? origin, http.Client? client})
    : origin = origin ?? Uri.base.resolve('/gotrue'),
      _client = client ?? BrowserClient();

  final Uri origin;
  final http.Client _client;

  Future<Map<String, Object?>> _request(
    String method,
    String path, {
    Map<String, String>? query,
    Map<String, Object?>? body,
    String? token,
    bool emptyOk = false,
  }) async {
    final loopback = {'localhost', '127.0.0.1', '::1'}.contains(origin.host);
    if (origin.host.isEmpty ||
        (origin.scheme != 'https' && !(loopback && origin.scheme == 'http'))) {
      throw const AuthFailure(
        AuthFailureKind.invalidConfiguration,
        'The authentication endpoint must use HTTPS.',
      );
    }
    final uri = origin.replace(
      path: '${origin.path}${path.startsWith('/') ? path : '/$path'}',
      queryParameters: query,
    );
    http.Response response;
    try {
      response = await _client
          .send(
            http.Request(method, uri)
              ..headers.addAll({
                'Accept': 'application/json',
                if (body != null) 'Content-Type': 'application/json',
                if (token != null) 'Authorization': 'Bearer $token',
              })
              ..body = body == null ? '' : jsonEncode(body),
          )
          .then(http.Response.fromStream)
          .timeout(const Duration(seconds: 35));
    } on Object {
      throw const AuthFailure(
        AuthFailureKind.network,
        'Cannot reach the authentication service.',
      );
    }
    if (response.statusCode < 200 || response.statusCode >= 300) {
      if (response.statusCode == 429) {
        throw const AuthFailure(
          AuthFailureKind.rateLimited,
          'Too many requests. Please wait and try again.',
        );
      }
      if (path == '/verify' &&
          {400, 401, 403, 422}.contains(response.statusCode)) {
        throw const AuthFailure(
          AuthFailureKind.invalidCode,
          'The code is invalid or has expired.',
        );
      }
      if (path == '/token' && {400, 401}.contains(response.statusCode)) {
        throw const AuthFailure.invalidCredentials();
      }
      throw AuthFailure(
        AuthFailureKind.server,
        'The authentication service rejected the request.',
        statusCode: response.statusCode,
      );
    }
    if (response.body.trim().isEmpty && emptyOk) return const {};
    try {
      final decoded = jsonDecode(response.body);
      if (decoded is! Map<String, dynamic>) {
        throw const AuthFailure.invalidResponse();
      }
      return decoded;
    } on FormatException {
      throw const AuthFailure.invalidResponse();
    }
  }

  @override
  Future<GoTrueSession> signInWithPassword(
    String email,
    String password,
  ) async => GoTrueSession.fromJson(
    await _request(
      'POST',
      '/token',
      query: {'grant_type': 'password'},
      body: {'email': email.trim(), 'password': password},
    ),
  );

  @override
  Future<GoTrueSession> refresh(String refreshToken) async =>
      GoTrueSession.fromJson(
        await _request(
          'POST',
          '/token',
          query: {'grant_type': 'refresh_token'},
          body: {'refresh_token': refreshToken},
        ),
      );

  @override
  Future<GoTrueUser> currentUser(String accessToken) async =>
      GoTrueUser.fromJson(await _request('GET', '/user', token: accessToken));

  @override
  Future<void> logout(String accessToken) async =>
      _request('POST', '/logout', token: accessToken, emptyOk: true);

  @override
  Future<GoTrueSettings> settings() async {
    final body = await _request('GET', '/settings');
    final external = body['external'];
    return GoTrueSettings(
      emailEnabled: body['email'] != false,
      providers: external is Map
          ? external.entries
                .where((entry) => entry.value == true)
                .map((entry) => entry.key.toString())
                .toSet()
          : const {},
    );
  }

  @override
  Future<GoTrueSignUpResult> signUp(String email, String password) async {
    final body = await _request(
      'POST',
      '/signup',
      body: {'email': email.trim(), 'password': password},
    );
    if (body['access_token'] is String) {
      final session = GoTrueSession.fromJson(body);
      return GoTrueSignUpResult(user: session.user, session: session);
    }
    return GoTrueSignUpResult(user: GoTrueUser.fromJson(body));
  }

  @override
  Future<void> resendSignUpCode(String email) async => _request(
    'POST',
    '/resend',
    body: {'type': 'signup', 'email': email.trim()},
    emptyOk: true,
  );

  @override
  Future<void> requestPasswordRecovery(String email) async => _request(
    'POST',
    '/recover',
    body: {'email': email.trim()},
    emptyOk: true,
  );

  @override
  Future<GoTrueSession> verifySignUpCode(String email, String code) async =>
      _verify(email, code, 'signup');

  @override
  Future<GoTrueSession> verifyRecoveryCode(String email, String code) async =>
      _verify(email, code, 'recovery');

  @override
  Future<GoTrueSession> signInWithCode(String email, String code) async =>
      _verify(email, code, 'email');

  Future<GoTrueSession> _verify(String email, String code, String type) async =>
      GoTrueSession.fromJson(
        await _request(
          'POST',
          '/verify',
          body: {'type': type, 'email': email.trim(), 'token': code.trim()},
        ),
      );

  @override
  Future<void> requestSignInCode(String email) async => _request(
    'POST',
    '/otp',
    body: {'email': email.trim(), 'create_user': false},
    emptyOk: true,
  );

  @override
  Future<void> updatePassword(String accessToken, String password) async =>
      _request(
        'PUT',
        '/user',
        token: accessToken,
        body: {'password': password},
      );

  void close() => _client.close();
}

/// Per-tab persistence avoids carrying credentials in URLs or layout storage.
final class BrowserAuthSessionStore implements AuthSessionStore {
  const BrowserAuthSessionStore();

  static const _key = 'openmuse.auth.session.v1';

  @override
  Future<GoTrueSession?> read() async {
    final encoded = web.window.sessionStorage.getItem(_key);
    if (encoded == null) return null;
    try {
      final body = jsonDecode(encoded);
      return body is Map<String, dynamic> ? GoTrueSession.fromJson(body) : null;
    } on Object {
      await delete();
      return null;
    }
  }

  @override
  Future<void> write(GoTrueSession session) async =>
      web.window.sessionStorage.setItem(_key, jsonEncode(session.toJson()));

  @override
  Future<void> delete() async => web.window.sessionStorage.removeItem(_key);
}
