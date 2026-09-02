import 'package:appflowy/core/config/kv.dart';
import 'package:appflowy/core/config/kv_keys.dart';
import 'package:appflowy/env/cloud_env.dart';
import 'package:appflowy/env/muse_cloud_profile.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter_test/flutter_test.dart';

const _deployment = {
  'MUSE_CLOUD_URL': 'https://openmuseai.com',
  'MUSE_CLOUD_GOTRUE_URL': 'https://openmuseai.com/gotrue',
  'MUSE_CLOUD_WS_URL': 'wss://openmuseai.com/ws/v1',
  'MUSE_CLOUD_WEB_URL': 'https://www.openmuseai.com',
};

void main() {
  test('remote profile shares Cloud auth while retaining native WS protocol',
      () {
    final profile = MuseCloudProfile.fromMap(_deployment);
    expect(profile.cloud.origin, 'https://openmuseai.com');
    expect(profile.gotrue.toString(), 'https://openmuseai.com/gotrue');
    expect(profile.websocket.toString(), 'wss://openmuseai.com/ws/v1');
  });

  test('rejects partial, insecure, loopback and credential-bearing profiles',
      () {
    for (final base in [
      '',
      'http://localhost',
      'https://localhost',
      'https://127.0.0.1',
      'https://[::1]',
      'https://openmuseai.com/app',
      'https://user:secret@openmuseai.com',
      'https://openmuseai.com?token=x',
    ]) {
      expect(
        () => MuseCloudProfile.fromMap(
          {..._deployment, 'MUSE_CLOUD_URL': base},
        ),
        throwsFormatException,
      );
    }
    expect(
      () => MuseCloudProfile.fromMap(
        {'MUSE_CLOUD_URL': 'https://openmuseai.com'},
      ),
      throwsFormatException,
    );
  });

  test('rejects split auth origin and copying the Web v2 socket into native',
      () {
    for (final override in [
      {'MUSE_CLOUD_GOTRUE_URL': 'https://foreign.invalid/gotrue'},
      {'MUSE_CLOUD_WS_URL': 'wss://openmuseai.com/ws/v2'},
      {'MUSE_CLOUD_WS_URL': 'wss://openmuseai.com:8443/ws/v1'},
    ]) {
      expect(
        () => MuseCloudProfile.fromMap({..._deployment, ...override}),
        throwsFormatException,
      );
    }
  });

  group(
    'compiled deployment startup',
    () {
      late _MemoryKv store;
      setUp(() {
        store = _MemoryKv();
        getIt.registerSingleton<KeyValueStorage>(store);
      });
      tearDown(() async => getIt.reset());

      test('ignores stale development URLs for FFI, password and share helpers',
          () async {
        store.values.addAll({
          KVKeys.kCloudType: '4',
          KVKeys.kAppflowyCloudBaseURL: 'http://localhost',
          'document-data-sentinel': 'preserve',
        });
        final profile = MuseCloudProfile.fromEnvironment()!;
        final env = await AppFlowyCloudSharedEnv.fromEnv();
        expect(env.authenticatorType, AuthenticatorType.appflowyCloud);
        expect(env.appflowyCloudConfig.base_url, profile.cloud.origin);
        expect(env.appflowyCloudConfig.gotrue_url, profile.gotrue.toString());
        expect(
          env.appflowyCloudConfig.ws_base_url,
          profile.websocket.toString(),
        );
        expect(await getAppFlowyCloudUrl(), profile.cloud.origin);
        expect(await getAppFlowyShareDomain(), profile.web.authority);
        expect(store.values['document-data-sentinel'], 'preserve');
        expect(store.values[KVKeys.kAppflowyCloudBaseURL], 'http://localhost');
      });

      test('fresh install uses remote Cloud without seeding beta defaults',
          () async {
        final env = await AppFlowyCloudSharedEnv.fromEnv();
        expect(env.authenticatorType, AuthenticatorType.appflowyCloud);
        expect(
          env.appflowyCloudConfig.base_url,
          MuseCloudProfile.fromEnvironment()!.cloud.origin,
        );
        expect(store.values, isEmpty);
      });

      test(
          'explicit offline mode remains available without deleting local data',
          () async {
        store.values[KVKeys.kCloudType] = '0';
        expect(
          (await AppFlowyCloudSharedEnv.fromEnv()).authenticatorType,
          AuthenticatorType.local,
        );
        await useAppFlowyBetaCloudWithURL(
          'https://beta.appflowy.cloud',
          AuthenticatorType.appflowyCloud,
        );
        final env = await AppFlowyCloudSharedEnv.fromEnv();
        expect(env.authenticatorType, AuthenticatorType.appflowyCloud);
        expect(
          env.appflowyCloudConfig.base_url,
          MuseCloudProfile.fromEnvironment()!.cloud.origin,
        );
      });
    },
    skip: !MuseCloudProfile.isConfigured
        ? 'Run with --dart-define-from-file=config/mobile/openmuse.json'
        : false,
  );
}

class _MemoryKv implements KeyValueStorage {
  final values = <String, String>{};
  @override
  Future<String?> get(String key) async => values[key];
  @override
  Future<void> set(String key, String value) async {
    values[key] = value;
  }

  @override
  Future<void> remove(String key) async {
    values.remove(key);
  }

  @override
  Future<void> clear() async {
    values.clear();
  }

  @override
  Future<T?> getWithFormat<T>(String key, T Function(String) formatter) async =>
      values[key] == null ? null : formatter(values[key]!);
}
