import 'dart:convert';

import 'package:flutter_secure_storage/flutter_secure_storage.dart';

import 'auth_models.dart';
import 'auth_ports.dart';

abstract interface class SecureValueStore {
  Future<String?> read(String key);
  Future<void> write(String key, String value);
  Future<void> delete(String key);
}

final class FlutterSecureValueStore implements SecureValueStore {
  const FlutterSecureValueStore({
    FlutterSecureStorage storage = const FlutterSecureStorage(),
  }) : _storage = storage;

  /// Uses the login Keychain on macOS so unsigned/ad-hoc development builds
  /// can persist sessions without a provisioning-profile access group.
  FlutterSecureValueStore.macOsCompatible({
    String accountName = 'flutter_secure_storage_service',
  }) : _storage = FlutterSecureStorage(
         mOptions: MacOsOptions(
           useDataProtectionKeyChain: false,
           accountName: accountName,
         ),
       );

  final FlutterSecureStorage _storage;

  @override
  Future<void> delete(String key) => _storage.delete(key: key);

  @override
  Future<String?> read(String key) => _storage.read(key: key);

  @override
  Future<void> write(String key, String value) =>
      _storage.write(key: key, value: value);
}

final class SecureAuthSessionStore implements AuthSessionStore {
  const SecureAuthSessionStore({
    required SecureValueStore values,
    this.key = 'openmuse.auth.gotrue.session.v1',
  }) : _values = values;

  final SecureValueStore _values;
  final String key;

  @override
  Future<GoTrueSession?> read() async {
    final encoded = await _values.read(key);
    if (encoded == null) return null;
    try {
      final decoded = jsonDecode(encoded);
      if (decoded is! Map) throw const FormatException();
      return GoTrueSession.fromJson(decoded.cast<String, Object?>());
    } catch (_) {
      await _values.delete(key);
      return null;
    }
  }

  @override
  Future<void> write(GoTrueSession session) =>
      _values.write(key, jsonEncode(session.toJson()));

  @override
  Future<void> delete() => _values.delete(key);
}
