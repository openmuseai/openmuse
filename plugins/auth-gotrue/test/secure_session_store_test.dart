import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';

void main() {
  test('secure store round trips and removes the session', () async {
    final values = _MemorySecureValues();
    final store = SecureAuthSessionStore(values: values);
    final session = GoTrueSession(
      accessToken: 'access',
      refreshToken: 'refresh',
      expiresAt: DateTime.utc(2026, 9, 30),
      user: const GoTrueUser(id: 'user-1', email: 'muse@example.com'),
    );

    await store.write(session);
    final restored = await store.read();
    expect(restored?.accessToken, 'access');
    expect(restored?.refreshToken, 'refresh');
    await store.delete();
    expect(await store.read(), isNull);
  });

  test('corrupt secret is deleted without being returned', () async {
    final values = _MemorySecureValues()..value = '{refresh_token:secret';
    final store = SecureAuthSessionStore(values: values);

    expect(await store.read(), isNull);
    expect(values.value, isNull);
    expect(values.deletes, 1);
  });
}

final class _MemorySecureValues implements SecureValueStore {
  String? value;
  int deletes = 0;

  @override
  Future<void> delete(String key) async {
    deletes++;
    value = null;
  }

  @override
  Future<String?> read(String key) async => value;

  @override
  Future<void> write(String key, String value) async => this.value = value;
}
