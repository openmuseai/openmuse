import 'mobile_build_locale.dart';

import 'dart:convert';
import 'dart:io';

/// Deletes the signed-in account through the existing AppFlowy Cloud endpoint.
/// The server removes the GoTrue identity and owned workspaces; the client must
/// only clear its local session after the server confirms success.
Future<void> deleteOpenMuseAccount({
  OpenMuseStrings? strings,
  required Uri cloudOrigin,
  required Future<String?> Function() accessToken,
  HttpClient? client,
}) async {
  final token = await accessToken();
  if (token == null || token.isEmpty) {
    throw AccountDeletionException(
      ((strings ?? OpenMuseStrings.fallback)
          .mobileYourSessionHasExpiredSignInAgainAnd9fd9856b()),
    );
  }
  if (cloudOrigin.scheme != 'https' &&
      !(cloudOrigin.scheme == 'http' &&
          (cloudOrigin.host == '127.0.0.1' ||
              cloudOrigin.host == 'localhost'))) {
    throw AccountDeletionException(
      ((strings ?? OpenMuseStrings.fallback)
          .mobileTheAccountServiceUrlIsNotSecureThe656db052()),
    );
  }

  final ownedClient = client == null;
  final httpClient = client ?? HttpClient();
  try {
    final uri = cloudOrigin.resolve('/api/user');
    final request = await httpClient
        .deleteUrl(uri)
        .timeout(const Duration(seconds: 20));
    request.headers.set(HttpHeaders.authorizationHeader, 'Bearer $token');
    request.headers.set(HttpHeaders.acceptHeader, 'application/json');
    final response = await request.close().timeout(const Duration(seconds: 30));
    final body = await utf8.decoder.bind(response).join();
    if (response.statusCode < 200 || response.statusCode >= 300) {
      throw AccountDeletionException(
        ((strings ?? OpenMuseStrings.fallback)
            .mobileTheDeletionRequestDidNotCompleteHttp82b79147(
              value1: response.statusCode,
            )),
      );
    }
    final decoded = jsonDecode(body);
    if (decoded is! Map<String, dynamic> || decoded['code'] != 0) {
      throw AccountDeletionException(
        ((strings ?? OpenMuseStrings.fallback)
            .mobileTheServerDidNotConfirmAccountDeletio6b42db38()),
      );
    }
  } on AccountDeletionException {
    rethrow;
  } on SocketException {
    throw AccountDeletionException(
      ((strings ?? OpenMuseStrings.fallback)
          .mobileTheNetworkIsUnavailableYourAccountHa29677dbd()),
    );
  } on FormatException {
    throw AccountDeletionException(
      ((strings ?? OpenMuseStrings.fallback)
          .mobileAnUnexpectedServerResponsePreventedCo46d0ec61()),
    );
  } finally {
    if (ownedClient) httpClient.close();
  }
}

final class AccountDeletionException implements Exception {
  const AccountDeletionException(this.message);
  final String message;

  @override
  String toString() => message;
}
