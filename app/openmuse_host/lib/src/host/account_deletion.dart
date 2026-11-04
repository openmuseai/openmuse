import 'dart:convert';
import 'dart:io';

/// Clear the local session only after this server-confirmed operation returns.
Future<void> deleteHostAccount({
  required Uri origin,
  required String token,
}) async {
  if (token.isEmpty ||
      (origin.scheme != 'https' &&
          !(origin.scheme == 'http' &&
              ['127.0.0.1', 'localhost'].contains(origin.host)))) {
    throw StateError('Sign in again to securely delete your account.');
  }
  final client = HttpClient();
  try {
    final request = await client
        .deleteUrl(origin.resolve('/api/user'))
        .timeout(const Duration(seconds: 20));
    request.headers.set(HttpHeaders.authorizationHeader, 'Bearer $token');
    request.headers.set(HttpHeaders.acceptHeader, 'application/json');
    final response = await request.close().timeout(const Duration(seconds: 30));
    final body = await utf8.decoder
        .bind(response)
        .join()
        .timeout(const Duration(seconds: 30));
    if (response.statusCode < 200 || response.statusCode >= 300) {
      throw StateError('Account deletion was not completed. Please try again.');
    }
    final result = jsonDecode(body);
    if (result is! Map || result['code'] != 0) {
      throw StateError('The service did not confirm account deletion.');
    }
  } finally {
    client.close(force: true);
  }
}
