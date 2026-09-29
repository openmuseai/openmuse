import 'package:flutter/foundation.dart';

@immutable
final class GoTrueUser {
  const GoTrueUser({required this.id, required this.email});

  factory GoTrueUser.fromJson(Map<String, Object?> json) {
    final id = json['id'];
    final email = json['email'];
    if (id is! String || id.isEmpty || email is! String || email.isEmpty) {
      throw const AuthFailure.invalidResponse();
    }
    return GoTrueUser(id: id, email: email);
  }

  final String id;
  final String email;

  Map<String, Object?> toJson() => {'id': id, 'email': email};
}

@immutable
final class GoTrueSession {
  const GoTrueSession({
    required this.accessToken,
    required this.refreshToken,
    required this.expiresAt,
    required this.user,
  });

  factory GoTrueSession.fromJson(
    Map<String, Object?> json, {
    DateTime Function()? clock,
  }) {
    final accessToken = json['access_token'];
    final refreshToken = json['refresh_token'];
    final user = json['user'];
    if (accessToken is! String ||
        accessToken.isEmpty ||
        refreshToken is! String ||
        refreshToken.isEmpty ||
        user is! Map) {
      throw const AuthFailure.invalidResponse();
    }

    final now = (clock ?? DateTime.now)().toUtc();
    final expiresAtValue = json['expires_at'];
    final expiresInValue = json['expires_in'];
    final DateTime expiresAt;
    if (expiresAtValue is num && expiresAtValue.toInt() > 0) {
      expiresAt = DateTime.fromMillisecondsSinceEpoch(
        expiresAtValue.toInt() * 1000,
        isUtc: true,
      );
    } else if (expiresInValue is num && expiresInValue.toInt() > 0) {
      expiresAt = now.add(Duration(seconds: expiresInValue.toInt()));
    } else {
      throw const AuthFailure.invalidResponse();
    }

    return GoTrueSession(
      accessToken: accessToken,
      refreshToken: refreshToken,
      expiresAt: expiresAt,
      user: GoTrueUser.fromJson(user.cast<String, Object?>()),
    );
  }

  final String accessToken;
  final String refreshToken;
  final DateTime expiresAt;
  final GoTrueUser user;

  Map<String, Object?> toJson() => {
    'access_token': accessToken,
    'refresh_token': refreshToken,
    'expires_at': expiresAt.toUtc().millisecondsSinceEpoch ~/ 1000,
    'user': user.toJson(),
  };
}

enum AuthFailureKind {
  invalidCredentials,
  network,
  server,
  invalidResponse,
  sessionExpired,
  bootstrap,
  invalidConfiguration,
}

final class AuthFailure implements Exception {
  const AuthFailure(this.kind, this.safeMessage, {this.statusCode});

  const AuthFailure.invalidCredentials()
    : this(
        AuthFailureKind.invalidCredentials,
        'The email or password is incorrect.',
      );

  const AuthFailure.invalidResponse()
    : this(
        AuthFailureKind.invalidResponse,
        'The authentication service returned an invalid response.',
      );

  const AuthFailure.sessionExpired()
    : this(
        AuthFailureKind.sessionExpired,
        'Your session has expired. Please sign in again.',
      );

  final AuthFailureKind kind;
  final String safeMessage;
  final int? statusCode;

  String get code => switch (kind) {
    AuthFailureKind.invalidCredentials => 'invalid_credentials',
    AuthFailureKind.network => 'network',
    AuthFailureKind.server => 'server',
    AuthFailureKind.invalidResponse => 'invalid_response',
    AuthFailureKind.sessionExpired => 'session_expired',
    AuthFailureKind.bootstrap => 'bootstrap',
    AuthFailureKind.invalidConfiguration => 'invalid_configuration',
  };

  @override
  String toString() => 'AuthFailure($code, statusCode: $statusCode)';
}

@immutable
final class GoTrueSettings {
  const GoTrueSettings({required this.emailEnabled, required this.providers});

  final bool emailEnabled;
  final Set<String> providers;
}
