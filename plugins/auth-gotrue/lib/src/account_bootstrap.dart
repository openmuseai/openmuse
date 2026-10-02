import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';

import 'auth_models.dart';
import 'auth_ports.dart';
import 'gotrue_client.dart';

/// Creates the AppFlowy account row and default workspace after GoTrue accepts
/// the credential. GoTrue admin signup does not do this by itself.
final class AppFlowyAccountBootstrapper implements CloudAccountBootstrapper {
  AppFlowyAccountBootstrapper({
    required Uri cloudOrigin,
    bool allowInsecureLoopback = false,
    AuthHttpClientFactory? clientFactory,
  }) : _origin = _validateOrigin(cloudOrigin, allowInsecureLoopback),
       _clientFactory = clientFactory ?? HttpClient.new;

  final Uri _origin;
  final AuthHttpClientFactory _clientFactory;

  static Uri _validateOrigin(Uri value, bool allowInsecureLoopback) {
    final path = value.path.isEmpty || value.path == '/';
    final loopback =
        value.host == 'localhost' ||
        value.host == '127.0.0.1' ||
        value.host == '::1' ||
        value.host == '10.0.2.2';
    if (!value.hasScheme ||
        value.host.isEmpty ||
        value.userInfo.isNotEmpty ||
        value.hasQuery ||
        value.hasFragment ||
        !path ||
        (value.scheme != 'https' &&
            !(allowInsecureLoopback &&
                value.scheme == 'http' &&
                loopback))) {
      throw const AuthFailure(
        AuthFailureKind.invalidConfiguration,
        'The account workspace endpoint is invalid.',
      );
    }
    return value.replace(path: '');
  }

  @override
  Future<void> bootstrap(GoTrueSession session) async {
    final client = _clientFactory();
    client.connectionTimeout = const Duration(seconds: 15);
    try {
      final uri = _origin.replace(
        path: '/api/user/verify/${Uri.encodeComponent(session.accessToken)}',
      );
      final request = await client.getUrl(uri);
      request.headers.set(HttpHeaders.acceptHeader, 'application/json');
      final response = await request.close().timeout(
        const Duration(seconds: 20),
      );
      final responseText = await utf8.decoder.bind(response).join();
      debugPrint(
        'OpenMuse auth: verify host=${_origin.host} status=${response.statusCode} bytes=${responseText.length}',
      );
      if (response.statusCode < 200 || response.statusCode >= 300) {
        throw const AuthFailure(
          AuthFailureKind.bootstrap,
          'The account workspace could not be initialized.',
        );
      }
      final decoded = jsonDecode(responseText);
      final code = decoded is Map ? decoded['code'] : null;
      final message = decoded is Map ? decoded['message'] : null;
      debugPrint(
        'OpenMuse auth: verify business code=$code message=$message',
      );
      if (code is! num || code.toInt() != 0) {
        throw const AuthFailure(
          AuthFailureKind.bootstrap,
          'The account workspace could not be initialized.',
        );
      }
    } on AuthFailure {
      rethrow;
    } on FormatException {
      throw const AuthFailure(
        AuthFailureKind.bootstrap,
        'The account workspace could not be initialized.',
      );
    } on SocketException {
      throw const AuthFailure(
        AuthFailureKind.network,
        'Cannot reach the account workspace service.',
      );
    } on HttpException {
      throw const AuthFailure(
        AuthFailureKind.network,
        'Cannot reach the account workspace service.',
      );
    } on TlsException {
      throw const AuthFailure(
        AuthFailureKind.network,
        'Cannot reach the account workspace service.',
      );
    } on TimeoutException {
      throw const AuthFailure(
        AuthFailureKind.network,
        'The account workspace service timed out.',
      );
    } finally {
      client.close(force: true);
    }
  }
}
