import 'dart:convert';

import 'package:appflowy/user/application/auth/device_id.dart';
import 'package:appflowy_backend/dispatch/dispatch.dart';
import 'package:appflowy_backend/protobuf/flowy-error/code.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-user/protobuf.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:nanoid/nanoid.dart';

@immutable
class DshDeviceCredential {
  const DshDeviceCredential({
    required this.token,
    required this.expiresAt,
    required this.deviceId,
    required this.kid,
  });

  final String token;
  final int expiresAt;
  final String deviceId;
  final String kid;

  bool isUsableAt(int nowMs) =>
      token.isNotEmpty &&
      token.length <= 8192 &&
      !token.startsWith('sk-') &&
      token.split('.').length == 2 &&
      token.split('.').every((part) => part.isNotEmpty) &&
      deviceId.isNotEmpty &&
      expiresAt > nowMs + const Duration(seconds: 30).inMilliseconds;

  Map<String, Object?> toJson() => {
        'token': token,
        'expiresAt': expiresAt,
        'deviceId': deviceId,
        'kid': kid,
      };

  static DshDeviceCredential? fromJson(Object? value) {
    if (value is! Map<String, Object?>) return null;
    final token = value['token'];
    final expiresAt = value['expiresAt'];
    final deviceId = value['deviceId'];
    final kid = value['kid'];
    if (token is! String ||
        expiresAt is! int ||
        deviceId is! String ||
        kid is! String) {
      return null;
    }
    return DshDeviceCredential(
      token: token,
      expiresAt: expiresAt,
      deviceId: deviceId,
      kid: kid,
    );
  }
}

abstract interface class DshCredentialStore {
  Future<String?> read(String key);
  Future<void> write(String key, String value);
  Future<void> delete(String key);
}

final class AndroidKeystoreDshCredentialStore implements DshCredentialStore {
  AndroidKeystoreDshCredentialStore({FlutterSecureStorage? storage})
      : _storage = storage ??
            const FlutterSecureStorage(
              aOptions: AndroidOptions(encryptedSharedPreferences: true),
            );

  final FlutterSecureStorage _storage;

  @override
  Future<String?> read(String key) => _storage.read(key: key);

  @override
  Future<void> write(String key, String value) =>
      _storage.write(key: key, value: value);

  @override
  Future<void> delete(String key) => _storage.delete(key: key);
}

final class DshCredentialException implements Exception {
  const DshCredentialException(this.code, {this.backendCode});
  final String code;
  final int? backendCode;
  @override
  String toString() => 'DshCredentialException($code, $backendCode)';
}

class DshDeviceTokenService {
  DshDeviceTokenService({
    DshCredentialStore? store,
    int Function()? clock,
    Future<String> Function()? deviceIdLoader,
    Future<MuseDshDeviceTokenPB> Function(IssueMuseDshDeviceTokenPB)? issue,
  })  : _store = store ?? AndroidKeystoreDshCredentialStore(),
        _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch),
        _deviceIdLoader = deviceIdLoader ?? getDeviceId,
        _issue = issue ?? _issueViaBackend;

  static const _credentialKey = 'muse.dsh.deviceCredential.v2';
  static const _deviceIdKey = 'muse.dsh.deviceId.v1';

  final DshCredentialStore _store;
  final int Function() _clock;
  final Future<String> Function() _deviceIdLoader;
  final Future<MuseDshDeviceTokenPB> Function(IssueMuseDshDeviceTokenPB) _issue;

  static Future<MuseDshDeviceTokenPB> _issueViaBackend(
    IssueMuseDshDeviceTokenPB request,
  ) async {
    final result = await UserEventIssueMuseDshDeviceToken(request).send();
    return result.fold(
      (value) => value,
      (error) => throw DshCredentialException(
        error.code == ErrorCode.UserUnauthorized
            ? 'CLOUD_DEVICE_TOKEN_REJECTED'
            : 'CLOUD_DEVICE_TOKEN_UNAVAILABLE',
        backendCode: error.code.value,
      ),
    );
  }

  Future<DshDeviceCredential> loadOrIssue({
    required String sessionRef,
    required String accountRef,
    required String cloudOrigin,
    required String dshOrigin,
  }) async {
    if (accountRef.isEmpty || cloudOrigin.isEmpty) {
      throw const DshCredentialException('CLOUD_LOGIN_REQUIRED');
    }
    final scope = jsonEncode([accountRef, cloudOrigin, dshOrigin, sessionRef]);
    final deviceId = await _getOrCreateDeviceId();
    final cached = await _readCredential(scope);
    if (cached != null &&
        cached.deviceId == deviceId &&
        cached.isUsableAt(_clock())) {
      return cached;
    }

    final request = IssueMuseDshDeviceTokenPB(
      deviceId: deviceId,
      sessionRef: sessionRef,
    );
    final value = await _issue(request);
    final credential = DshDeviceCredential(
      token: value.token,
      expiresAt: value.expiresAt.toInt(),
      deviceId: value.deviceId,
      kid: value.kid,
    );
    if (!credential.isUsableAt(_clock()) || credential.deviceId != deviceId) {
      throw const DshCredentialException('DSH_DEVICE_TOKEN_INVALID');
    }
    await _store.write(
      _credentialKey,
      jsonEncode({'scope': scope, 'credential': credential.toJson()}),
    );
    return credential;
  }

  Future<void> clear() async {
    await _store.delete(_credentialKey);
    await _store.delete('muse.dsh.deviceCredential.v1');
  }

  Future<DshDeviceCredential?> _readCredential(String scope) async {
    final raw = await _store.read(_credentialKey);
    if (raw == null || raw.isEmpty) return null;
    try {
      final envelope = (jsonDecode(raw) as Map).cast<String, Object?>();
      if (envelope['scope'] != scope) {
        await clear();
        return null;
      }
      return DshDeviceCredential.fromJson(envelope['credential']);
    } catch (_) {
      await clear();
      return null;
    }
  }

  Future<String> _getOrCreateDeviceId() async {
    final existing = (await _store.read(_deviceIdKey))?.trim();
    if (existing != null && existing.isNotEmpty) return existing;

    // Keep the DSH identity stable across launches without relying on Android's
    // hardware identifiers. This value is scoped to the app's Keystore.
    final appDeviceId = (await _deviceIdLoader()).trim();
    final generated =
        'android.${appDeviceId.isEmpty ? nanoid(24) : appDeviceId}.${nanoid(12)}';
    await _store.write(_deviceIdKey, generated);
    return generated;
  }
}
