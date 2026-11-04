import 'dart:convert';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:cryptography/cryptography.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config_io.dart';

String b64(List<int> value) => base64UrlEncode(value).replaceAll('=', '');

final class FakeTransport implements ConfigTransport {
  final responses = <Uri, List<int>>{};
  final requests = <Uri>[];
  @override
  Future<List<int>> get(Uri uri, {required int maxBytes}) async {
    requests.add(uri);
    return responses[uri] ?? (throw const SocketException('offline'));
  }
}

final class FailingStore implements ConfigStore {
  @override
  Future<String?> read() async => null;
  @override
  Future<void> commit(String record) async =>
      throw const FileSystemException('disk full');
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late Map<String, dynamic> source;
  late DeploymentBundle bundle;
  late SimpleKeyPair keyPair;
  late FakeTransport transport;
  late MemoryConfigStore store;
  final now = DateTime.utc(2026, 10, 30, 2);
  final endpoint = Uri.parse('https://config.example/active.jws');
  final resourceUrl = Uri.parse('https://cdn.example/shell.json');

  RuntimeConfigClient client({
    ConfigStore? cache,
    DateTime Function()? clock,
  }) => RuntimeConfigClient(
    bundle: bundle,
    transport: transport,
    store: cache ?? store,
    build: 10,
    capabilities: {'config.v1'},
    clock: clock ?? () => now,
  );

  Map<String, dynamic> snapshot({int sequence = 1}) {
    final data =
        jsonDecode(jsonEncode(source['snapshot'])) as Map<String, dynamic>;
    data['sequence'] = sequence;
    data['releaseId'] = 'remote-$sequence';
    data['issuedAt'] = now
        .subtract(const Duration(minutes: 1))
        .toIso8601String();
    data['expiresAt'] = now.add(const Duration(days: 1)).toIso8601String();
    return data;
  }

  Future<String> sign(
    Map<String, dynamic> data, {
    Map<String, dynamic>? header,
  }) async {
    final encodedHeader = b64(
      utf8.encode(
        jsonEncode(
          header ??
              {
                'alg': 'EdDSA',
                'typ': 'openmuse.runtime-config+json',
                'kid': 'test-key',
              },
        ),
      ),
    );
    final payload = b64(utf8.encode(jsonEncode(data)));
    final input = '$encodedHeader.$payload';
    final signature = await Ed25519().sign(
      utf8.encode(input),
      keyPair: keyPair,
    );
    return '$input.${b64(signature.bytes)}';
  }

  Future<void> publish(Map<String, dynamic> data) async {
    transport.responses[endpoint] = utf8.encode(await sign(data));
  }

  setUp(() async {
    source =
        jsonDecode(await File('assets/deployments/default.json').readAsString())
            as Map<String, dynamic>;
    keyPair = await Ed25519().newKeyPair();
    final publicKey = await keyPair.extractPublicKey();
    source['bootstrap']['publicKeys'] = {'test-key': b64(publicKey.bytes)};
    source['bootstrap']['discoveryUrls'] = [endpoint.toString()];
    source['bootstrap']['allowedResourceOrigins'] = [resourceUrl.origin];
    bundle = DeploymentBundle.parse(jsonEncode(source));
    transport = FakeTransport();
    store = MemoryConfigStore();
  });

  test('bundled profiles validate and do not need a remote service', () async {
    for (final file in Directory(
      'assets/deployments',
    ).listSync().whereType<File>()) {
      final installed = DeploymentBundle.parse(await file.readAsString());
      expect(installed.snapshot.serviceUri(ServiceId.cloud).scheme, 'https');
      expect(installed.snapshot.linkUri(LinkId.privacy).scheme, 'https');
    }
    final loaded = await loadDeploymentBundle();
    expect(loaded.snapshot.sequence, 0);
  });

  test('locale aliases and links come from data and preserve script', () {
    final config = bundle.snapshot;
    expect(config.selectLocale(['en-GB']), 'en');
    expect(config.selectLocale(['zh-CN']), 'zh-Hans');
    expect(config.linkUri(LinkId.privacy, locale: 'en').path, '/en/privacy');
    final data = snapshot();
    data['localization']['defaultLocale'] = 'en';
    expect(RuntimeConfig.fromJson(data).selectLocale(['zh-TW']), 'en');
  });

  test('parsed nested data is immutable and detached from source', () {
    final data = snapshot();
    final config = RuntimeConfig.fromJson(data);
    data['deployment']['services']['cloud']['baseUrl'] =
        'https://changed.example';
    expect(
      config.serviceUri(ServiceId.cloud),
      bundle.snapshot.serviceUri(ServiceId.cloud),
    );
    expect(
      () => (config.json['localization'] as Map)['defaultLocale'] = 'en',
      throwsUnsupportedError,
    );
  });

  test('rejects unsafe links and service URLs before publication', () {
    for (final path in [
      '//evil.example',
      '/%2e%2e/private',
      '/x%2fy',
      '/\\evil',
      'javascript:alert(1)',
    ]) {
      final data = snapshot();
      data['links']['legal.privacy']['pathByLocale']['en'] = path;
      expect(
        () => RuntimeConfig.fromJson(data),
        throwsFormatException,
        reason: path,
      );
    }
    final data = snapshot();
    data['deployment']['services']['auth']['baseUrl'] =
        'https://user@auth.example';
    expect(() => RuntimeConfig.fromJson(data), throwsFormatException);
  });

  test(
    'activates signed metadata and resources together and restores after restart',
    () async {
      final data = snapshot();
      final bytes = utf8.encode('{"hello":"Hello"}');
      data['resources'] = [
        {
          'id': 'shell-en',
          'url': resourceUrl.toString(),
          'sha256': sha256.convert(bytes).toString(),
          'maxBytes': 1024,
        },
      ];
      data['localization']['bundleRefs'] = ['shell-en'];
      transport.responses[resourceUrl] = bytes;
      await publish(data);
      final first = client();
      expect((await first.refresh()).sequence, 1);
      expect(first.resources['shell-en'], bytes);
      transport.responses.clear();
      final restarted = client();
      await restarted.restore();
      expect(restarted.current.releaseId, 'remote-1');
      expect(restarted.resources['shell-en'], bytes);
      expect(
        () => restarted.resources['shell-en']!.add(1),
        throwsUnsupportedError,
      );
    },
  );

  test(
    'tampered payload and unexpected signing headers are rejected',
    () async {
      final data = snapshot();
      final valid = await sign(data);
      final parts = valid.split('.');
      data['releaseId'] = 'tampered';
      parts[1] = b64(utf8.encode(jsonEncode(data)));
      final verifier = SignedConfigVerifier(publicKeys: bundle.publicKeys);
      await expectLater(
        verifier.verify(parts.join('.')),
        throwsFormatException,
      );
      for (final header in [
        {
          'alg': 'none',
          'typ': 'openmuse.runtime-config+json',
          'kid': 'test-key',
        },
        {'alg': 'EdDSA', 'typ': 'JWT', 'kid': 'test-key'},
        {
          'alg': 'EdDSA',
          'typ': 'openmuse.runtime-config+json',
          'kid': 'unknown',
        },
        {
          'alg': 'EdDSA',
          'typ': 'openmuse.runtime-config+json',
          'kid': 'test-key',
          'jku': 'https://evil.example/key',
        },
      ]) {
        await expectLater(
          verifier.verify(await sign(snapshot(), header: header)),
          throwsFormatException,
        );
      }
    },
  );

  test(
    'signed data cannot change scope, installed service trust or build capability',
    () async {
      final changes = <void Function(Map<String, dynamic>)>[
        (d) => d['scope']['dataRegion'] = 'other-region',
        (d) => d['environment'] = 'staging',
        (d) => d['deployment']['authBindingId'] = 'other-issuer',
        (d) => d['deployment']['services']['cloud']['baseUrl'] =
            'https://evil.example',
        (d) => d['compatibility']['minBuild'] = 11,
        (d) =>
            d['compatibility']['requiredCapabilities'] = ['unimplemented.v2'],
        (d) => d['localization']['enabledLocales'].add('ja'),
        (d) => d['authentication']['sessionStorageKey'] = 'other-account',
      ];
      for (final change in changes) {
        final data = snapshot();
        change(data);
        await publish(data);
        final runtime = client();
        await expectLater(runtime.refresh(), throwsFormatException);
        expect(runtime.current.sequence, 0);
      }
    },
  );

  test(
    'failed resource hash or cache commit preserves active snapshot',
    () async {
      final runtime = client();
      await publish(snapshot());
      await runtime.refresh();
      final data = snapshot(sequence: 2);
      data['resources'] = [
        {
          'id': 'bad',
          'url': resourceUrl.toString(),
          'sha256': '0' * 64,
          'maxBytes': 1024,
        },
      ];
      transport.responses[resourceUrl] = [1, 2, 3];
      await publish(data);
      await expectLater(runtime.refresh(), throwsFormatException);
      expect(runtime.current.sequence, 1);
      expect(runtime.resources, isEmpty);
      await publish(snapshot(sequence: 3));
      final fullDisk = client(cache: FailingStore());
      await expectLater(
        fullDisk.refresh(),
        throwsA(isA<FileSystemException>()),
      );
      expect(fullDisk.current.sequence, 0);
    },
  );

  test(
    'rejects old sequences across restart; rollback uses a new sequence',
    () async {
      await publish(snapshot(sequence: 8));
      await client().refresh();
      final restarted = client();
      await publish(snapshot(sequence: 7));
      await expectLater(restarted.refresh(), throwsFormatException);
      expect(restarted.current.sequence, 8);
      final restoredContent = snapshot(sequence: 9)
        ..['releaseId'] = 'restored-content';
      await publish(restoredContent);
      expect((await restarted.refresh()).releaseId, 'restored-content');
      final conflicting = snapshot(sequence: 9)..['releaseId'] = 'conflict';
      await publish(conflicting);
      await expectLater(restarted.refresh(), throwsFormatException);
    },
  );

  test(
    'expired authentic cache remains usable offline but cannot open cloud sessions',
    () async {
      await publish(snapshot(sequence: 5));
      await client().refresh();
      final restarted = client(clock: () => now.add(const Duration(days: 2)));
      await restarted.restore();
      expect(restarted.current.sequence, 5);
      expect(restarted.networkConfigExpired, isTrue);
      expect(() => restarted.currentForNetwork, throwsStateError);
      await publish(snapshot(sequence: 4));
      await expectLater(restarted.refresh(), throwsFormatException);
    },
  );

  test('corrupt cache recovers from a signed remote version', () async {
    await store.commit('{broken');
    await publish(snapshot(sequence: 2));
    final runtime = client();
    expect((await runtime.refresh()).sequence, 2);
    expect(runtime.restoreError, isA<FormatException>());
  });

  test('a missing referenced language package prevents activation', () async {
    final data = snapshot();
    data['localization']['bundleRefs'] = ['missing'];
    await publish(data);
    final runtime = client();
    await expectLater(runtime.refresh(), throwsFormatException);
    expect(runtime.current.sequence, 0);
  });

  test(
    'concurrent refreshes coalesce and HTTP adapter refuses redirects',
    () async {
      await publish(snapshot());
      final runtime = client();
      await Future.wait([runtime.refresh(), runtime.refresh()]);
      expect(transport.requests.where((uri) => uri == endpoint).length, 1);
      final httpTransport = HttpConfigTransport(
        client: MockClient((request) async {
          expect(request.followRedirects, isFalse);
          expect(request.headers.containsKey('Authorization'), isFalse);
          return http.Response(
            '',
            302,
            headers: {'location': 'https://evil.example'},
          );
        }),
      );
      await expectLater(
        httpTransport.get(endpoint, maxBytes: 1024),
        throwsFormatException,
      );
    },
  );

  test('file cache replaces records and leaves no temporary files', () async {
    final directory = await Directory.systemTemp.createTemp(
      'openmuse-config-test-',
    );
    addTearDown(() => directory.delete(recursive: true));
    final cache = FileConfigStore(File('${directory.path}/cache/active.json'));
    expect(await cache.read(), isNull);
    await cache.commit('first');
    await cache.commit('second');
    expect(await cache.read(), 'second');
    expect(cache.file.parent.listSync(), hasLength(1));
  });
}
