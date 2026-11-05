import 'package:appflowy/plugins/dsh_agent/dsh_device_token_service.dart';
import 'package:appflowy_backend/protobuf/flowy-user/protobuf.dart';
import 'package:fixnum/fixnum.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test(
      'issue/cache/refresh bind to account and server without leaking credentials',
      () async {
    final store = _CredentialMemory();
    var now = 1000;
    var calls = 0;
    final service = DshDeviceTokenService(
      store: store,
      clock: () => now,
      deviceIdLoader: () async => 'fixture-device',
      issue: (request) async {
        calls++;
        expect(request.sessionRef, '');
        return MuseDshDeviceTokenPB(
          token: 'payload.signature',
          deviceId: request.deviceId,
          expiresAt: Int64(now + 120000),
          kid: 'v1',
        );
      },
    );
    Future<DshDeviceCredential> load({
      String account = 'account-1',
      String cloud = 'https://openmuseai.com',
    }) =>
        service.loadOrIssue(
          sessionRef: '',
          accountRef: account,
          cloudOrigin: cloud,
          dshOrigin: 'https://dsh.openmuseai.com',
        );
    final first = await load();
    expect((await load()).deviceId, first.deviceId);
    expect(calls, 1);
    await load(account: 'account-2');
    expect(calls, 2);
    await load(account: 'account-2', cloud: 'https://other.invalid');
    expect(calls, 3);
    now += 100000;
    await load(account: 'account-2', cloud: 'https://other.invalid');
    expect(calls, 4);
    await service.clear();
    expect(store.values.keys, ['muse.dsh.deviceId.v1']);
    expect((await load()).deviceId, first.deviceId);
    expect(calls, 5);
  });

  test('missing login and mismatched device never cache an issued credential',
      () async {
    final store = _CredentialMemory();
    var calls = 0;
    final service = DshDeviceTokenService(
      store: store,
      clock: () => 1000,
      deviceIdLoader: () async => 'fixture',
      issue: (_) async {
        calls++;
        return MuseDshDeviceTokenPB(
          token: 'payload.signature',
          expiresAt: Int64(100000),
          deviceId: 'foreign',
          kid: 'v1',
        );
      },
    );
    Future<DshDeviceCredential> load(String account) => service.loadOrIssue(
          sessionRef: '',
          accountRef: account,
          cloudOrigin: 'https://openmuseai.com',
          dshOrigin: 'https://dsh.openmuseai.com',
        );
    await expectLater(load(''), throwsA(isA<DshCredentialException>()));
    expect(calls, 0);
    await expectLater(
      load('account-1'),
      throwsA(isA<DshCredentialException>()),
    );
    expect(store.values.containsKey('muse.dsh.deviceCredential.v2'), false);
  });

  test('Cloud refusal is distinct from Host capability failure and not cached',
      () async {
    final store = _CredentialMemory();
    final service = DshDeviceTokenService(
      store: store,
      deviceIdLoader: () async => 'fixture',
      issue: (_) async => throw const DshCredentialException(
        'CLOUD_DEVICE_TOKEN_REJECTED',
        backendCode: 2,
      ),
    );
    await expectLater(
      service.loadOrIssue(
        sessionRef: '',
        accountRef: 'account-1',
        cloudOrigin: 'https://openmuseai.com',
        dshOrigin: 'https://dsh.openmuseai.com',
      ),
      throwsA(
        isA<DshCredentialException>()
            .having((e) => e.code, 'code', 'CLOUD_DEVICE_TOKEN_REJECTED'),
      ),
    );
    expect(store.values.containsKey('muse.dsh.deviceCredential.v2'), false);
  });
  test('device credential accepts only bounded two-part non-model tokens', () {
    const now = 1000;
    const valid = DshDeviceCredential(
      token: 'payload.signature',
      expiresAt: 100000,
      deviceId: 'android.device',
      kid: 'v1',
    );
    expect(valid.isUsableAt(now), isTrue);
    expect(
      const DshDeviceCredential(
        token: 'sk-secret',
        expiresAt: 100000,
        deviceId: 'android.device',
        kid: 'v1',
      ).isUsableAt(now),
      isFalse,
    );
    expect(
      const DshDeviceCredential(
        token: 'jwt.with.three-parts',
        expiresAt: 100000,
        deviceId: 'android.device',
        kid: 'v1',
      ).isUsableAt(now),
      isFalse,
    );
  });

  test('device credential JSON does not accept wrong field types', () {
    expect(
      DshDeviceCredential.fromJson({
        'token': 'payload.signature',
        'expiresAt': 'tomorrow',
        'deviceId': 'android.device',
        'kid': 'v1',
      }),
      isNull,
    );
  });
}

class _CredentialMemory implements DshCredentialStore {
  final values = <String, String>{};
  @override
  Future<String?> read(String key) async => values[key];
  @override
  Future<void> write(String key, String value) async {
    values[key] = value;
  }

  @override
  Future<void> delete(String key) async {
    values.remove(key);
  }
}
