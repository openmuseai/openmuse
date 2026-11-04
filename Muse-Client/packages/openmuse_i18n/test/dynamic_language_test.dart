import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

void main() {
  late Map<String, dynamic> fixture;
  setUpAll(() async {
    final result = await Process.run('node', [
      '../../services/config-control/test/dynamic-language-fixture.mjs',
    ]);
    if (result.exitCode != 0) throw StateError('${result.stderr}');
    fixture = jsonDecode(result.stdout as String) as Map<String, dynamic>;
  });
  test(
    'a new signed language downloads only on selection, preserves current UI on failure and restores offline',
    () async {
      final bundle = DeploymentBundle.parse(jsonEncode(fixture['bundle']));
      final transport = _Transport(fixture),
          store = MemoryConfigStore(),
          preferences = MemoryLocalePreferences();
      final locale = OpenMuseLocaleController(
        defaultTag: bundle.snapshot.defaultLocale,
        preferences: preferences,
      );
      final runtime = OpenMuseClientRuntime(
        bundle: bundle,
        transport: transport,
        store: store,
        build: 1,
        locale: locale,
        clock: () => DateTime.parse(fixture['now'] as String),
      );
      final pluginConfiguration = OpenMusePluginConfigurationAdapter(runtime);
      expect(
        pluginConfiguration.feature('plugins.market.enabled', fallback: true),
        isTrue,
      );
      await runtime.bootstrap();
      expect(
        pluginConfiguration.feature('plugins.market.enabled', fallback: true),
        isFalse,
      );
      expect(
        pluginConfiguration.parameter<int>(
          'plugins.example.limit',
          fallback: 0,
        ),
        17,
      );
      expect(locale.options.map((v) => v.tag), contains('fr'));
      expect(runtime.presentation.isEnabled(AppFeature.pluginMarket), isFalse);
      expect(
        runtime.presentation.settings.parameter<int>(
          'plugins.example.limit',
          fallback: 0,
        ),
        17,
      );
      expect(runtime.refreshInterval, const Duration(seconds: 60));
      expect(runtime.pendingRestart, isNull);
      expect(locale.tag, 'en');
      expect(transport.languageRequests, 0);
      expect(
        runtime.client.current.json['resources'][0]['delivery'],
        'onDemand',
      );
      transport.corrupt = true;
      await locale.requestTag('fr');
      expect(locale.tag, 'en');
      expect(locale.languageError, isNotNull);
      transport.corrupt = false;
      await locale.retryLanguage();
      expect(locale.tag, 'fr');
      expect(locale.strings.commonSave(), 'Enregistrer');
      expect(locale.languageError, isNull);
      await locale.saved;
      runtime.dispose();
      locale.dispose();
      final restoredLocale = OpenMuseLocaleController(
        defaultTag: bundle.snapshot.defaultLocale,
        preferences: preferences,
      );
      await restoredLocale.restore();
      final restored = OpenMuseClientRuntime(
        bundle: bundle,
        transport: _Offline(),
        store: store,
        build: 1,
        locale: restoredLocale,
        clock: () => DateTime.parse(fixture['now'] as String),
      );
      await restored.restore();
      expect(restoredLocale.tag, 'fr');
      expect(restoredLocale.strings.commonCancel(), 'Annuler');
      restored.dispose();
      restoredLocale.dispose();
    },
  );
  test(
    'following the system downloads a missing language without making it an explicit preference',
    () async {
      final bundle = DeploymentBundle.parse(jsonEncode(fixture['bundle']));
      final locale = OpenMuseLocaleController(
        defaultTag: bundle.snapshot.defaultLocale,
        systemLocales: ['fr-FR'],
      );
      final transport = _Transport(fixture);
      final runtime = OpenMuseClientRuntime(
        bundle: bundle,
        transport: transport,
        store: MemoryConfigStore(),
        build: 1,
        locale: locale,
        clock: () => DateTime.parse(fixture['now'] as String),
      );
      await runtime.bootstrap();
      expect(locale.tag, 'fr');
      expect(locale.followsSystem, isTrue);
      expect(transport.languageRequests, 1);
      locale.selectTag('en');
      locale.updateSystemLocales(['fr-FR']);
      expect(locale.tag, 'en');
      runtime.dispose();
      locale.dispose();
    },
  );
}

final class _Transport implements ConfigTransport {
  _Transport(this.fixture);
  final Map<String, dynamic> fixture;
  int languageRequests = 0;
  bool corrupt = false;
  @override
  Future<List<int>> get(Uri uri, {required int maxBytes}) async {
    if (uri.path.endsWith('.jws')) return utf8.encode(fixture['jws'] as String);
    languageRequests++;
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
