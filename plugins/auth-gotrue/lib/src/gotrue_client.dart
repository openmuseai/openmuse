import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'auth_models.dart';
import 'auth_ports.dart';

typedef AuthHttpClientFactory = HttpClient Function();

final class GoTrueClientConfig {
  GoTrueClientConfig({required Uri origin, this.allowInsecureLoopback = false})
    : origin = _validateOrigin(origin, allowInsecureLoopback);

  final Uri origin;
  final bool allowInsecureLoopback;

  static Uri _validateOrigin(Uri value, bool allowInsecureLoopback) {
    final cleanPath = value.path.isEmpty || value.path == '/';
    if (!value.hasScheme ||
        value.host.isEmpty ||
        value.userInfo.isNotEmpty ||
        value.hasQuery ||
        value.hasFragment ||
        !cleanPath) {
      throw const AuthFailure(
        AuthFailureKind.invalidConfiguration,
        'The authentication endpoint is invalid.',
      );
    }
    if (value.scheme == 'https') {
      return value.replace(path: '');
    }
    final loopback =
        value.host == 'localhost' ||
        value.host == '127.0.0.1' ||
        value.host == '::1' ||
        value.host == '10.0.2.2';
    if (value.scheme != 'http' || !allowInsecureLoopback || !loopback) {
      throw const AuthFailure(
        AuthFailureKind.invalidConfiguration,
        'The authentication endpoint must use HTTPS.',
      );
    }
    return value.replace(path: '');
  }
}

final class GoTrueHttpClient implements GoTrueAuthProvider {
  GoTrueHttpClient({
    required GoTrueClientConfig config,
    AuthHttpClientFactory? clientFactory,
    DateTime Function()? clock,
  }) : _origin = config.origin,
       _clientFactory = clientFactory ?? HttpClient.new,
       _clock = clock ?? DateTime.now;

  final Uri _origin;
  final AuthHttpClientFactory _clientFactory;
  final DateTime Function() _clock;

  @override
  Future<GoTrueSession> signInWithPassword(
    String email,
    String password,
  ) async {
    final result = await _jsonRequest(
      'POST',
      '/token',
      query: {'grant_type': 'password'},
      body: {'email': email.trim(), 'password': password},
      invalidCredentialsOnUnauthorized: true,
    );
    return GoTrueSession.fromJson(result, clock: _clock);
  }

  @override
  Future<void> requestSignInCode(String email) async {
    await _jsonRequest(
      'POST',
      '/otp',
      body: {'email': email.trim(), 'create_user': false},
      acceptEmpty: true,
    );
  }

  @override
  Future<GoTrueSession> signInWithCode(String email, String code) async {
    final result = await _jsonRequest(
      'POST',
      '/verify',
      body: {'type': 'email', 'email': email.trim(), 'token': code.trim()},
      invalidCredentialsOnUnauthorized: true,
    );
    return GoTrueSession.fromJson(result, clock: _clock);
  }

  @override
  Future<GoTrueSession> refresh(String refreshToken) async {
    try {
      final result = await _jsonRequest(
        'POST',
        '/token',
        query: {'grant_type': 'refresh_token'},
        body: {'refresh_token': refreshToken},
      );
      return GoTrueSession.fromJson(result, clock: _clock);
    } on AuthFailure catch (error) {
      if (error.statusCode == 400 || error.statusCode == 401) {
        throw const AuthFailure.sessionExpired();
      }
      rethrow;
    }
  }

  @override
  Future<GoTrueUser> currentUser(String accessToken) async =>
      GoTrueUser.fromJson(
        await _jsonRequest('GET', '/user', accessToken: accessToken),
      );

  @override
  Future<void> logout(String accessToken) async {
    await _jsonRequest(
      'POST',
      '/logout',
      accessToken: accessToken,
      acceptEmpty: true,
    );
  }

  @override
  Future<GoTrueSettings> settings() async {
    final value = await _jsonRequest('GET', '/settings');
    final providers = <String>{};
    final external = value['external'];
    if (external is Map) {
      for (final entry in external.entries) {
        if (entry.value == true) providers.add(entry.key.toString());
      }
    }
    return GoTrueSettings(
      emailEnabled: value['email'] != false,
      providers: providers,
    );
  }

  Future<Map<String, Object?>> _jsonRequest(
    String method,
    String path, {
    Map<String, String>? query,
    Map<String, Object?>? body,
    String? accessToken,
    bool acceptEmpty = false,
    bool invalidCredentialsOnUnauthorized = false,
  }) async {
    final client = _clientFactory();
    try {
      final uri = _origin.replace(path: path, queryParameters: query);
      final request = await client.openUrl(method, uri);
      request.headers.set(HttpHeaders.acceptHeader, 'application/json');
      if (accessToken != null) {
        request.headers.set(
          HttpHeaders.authorizationHeader,
          'Bearer $accessToken',
        );
      }
      if (body != null) {
        request.headers.contentType = ContentType.json;
        request.write(jsonEncode(body));
      }
      final response = await request.close();
      final responseText = await utf8.decoder.bind(response).join();
      if (response.statusCode < 200 || response.statusCode >= 300) {
        if (invalidCredentialsOnUnauthorized &&
            (response.statusCode == 400 || response.statusCode == 401)) {
          throw const AuthFailure.invalidCredentials();
        }
        throw AuthFailure(
          AuthFailureKind.server,
          'The authentication service rejected the request.',
          statusCode: response.statusCode,
        );
      }
      if (responseText.trim().isEmpty && acceptEmpty) return const {};
      try {
        final decoded = jsonDecode(responseText);
        if (decoded is! Map) throw const AuthFailure.invalidResponse();
        return decoded.cast<String, Object?>();
      } on FormatException {
        throw const AuthFailure.invalidResponse();
      }
    } on AuthFailure {
      rethrow;
    } on SocketException {
      throw const AuthFailure(
        AuthFailureKind.network,
        'Cannot reach the authentication service.',
      );
    } on TimeoutException {
      throw const AuthFailure(
        AuthFailureKind.network,
        'The authentication service timed out.',
      );
    } finally {
      client.close(force: true);
    }
  }
}
