import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

void main() {
  test(
    'plugin messages verify on demand, remain separate from Host ICU, and restore offline',
    () async {
      final result = await Process.run('node', [
        '../../services/config-control/test/plugin-messages-fixture.mjs',
      ]);
      expect(result.exitCode, 0, reason: '${result.stderr}');
      final fixture =
          jsonDecode(result.stdout as String) as Map<String, dynamic>;
      final bundle = DeploymentBundle.parse(jsonEncode(fixture['bundle']));
      final transport = _Transport(fixture), store = MemoryConfigStore();
      final locale = OpenMuseLocaleController(
        defaultTag: bundle.snapshot.defaultLocale,
      );
      OpenMuseClientRuntime create(
        OpenMuseLocaleController controller,
        ConfigTransport network,
      ) => OpenMuseClientRuntime(
        bundle: bundle,
        transport: network,
        store: store,
        build: 1,
        locale: controller,
        clock: () => DateTime.parse(fixture['now'] as String),
      );
      final runtime = create(locale, transport);
      await runtime.bootstrap();
      expect(runtime.lastError, isNull);
      expect(transport.downloads, 0);
      expect(
        runtime
            .presentation
            .settings
            .features['io.example.plugin.editing.enabled'],
        isFalse,
      );
      transport.corrupt = true;
      await expectLater(
        runtime.preparePluginMessages('en'),
        throwsFormatException,
      );
      expect(runtime.client.resources, isEmpty);
      expect(locale.strings.commonSave(), 'Save');
      transport.corrupt = false;
      await runtime.preparePluginMessages('en');
      expect(runtime.client.resources.length, 1);
      final bytes = runtime.client.resources.values.single;
      expect(jsonDecode(utf8.decode(bytes)), fixture['pack']);
      final requests = transport.downloads;
      await runtime.preparePluginMessages('en');
      expect(transport.downloads, requests);
      runtime.dispose();
      locale.dispose();
      final restoredLocale = OpenMuseLocaleController(
        defaultTag: bundle.snapshot.defaultLocale,
      );
      final restored = create(restoredLocale, _Offline());
      await restored.restore();
      expect(restored.client.resources.values.single, bytes);
      expect(restored.lastError, isNull);
      restored.dispose();
      restoredLocale.dispose();
    },
  );
}

final class _Transport implements ConfigTransport {
  _Transport(this.fixture);
  final Map<String, dynamic> fixture;
  int downloads = 0;
  bool corrupt = false;
  @override
  Future<List<int>> get(Uri uri, {required int maxBytes}) async {
    if (uri.path.endsWith('.jws')) return utf8.encode(fixture['jws'] as String);
    downloads++;
    return corrupt
        ? utf8.encode('tampered')
        : base64Decode((fixture['resources'] as Map)[uri.toString()] as String);
  }
}

final class _Offline implements ConfigTransport {
  @override
  Future<List<int>> get(Uri uri, {required int maxBytes}) async =>
      throw StateError('Offline');
}
