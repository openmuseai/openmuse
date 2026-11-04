import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_mobile/mobile_plugins/mobile_my_page.dart';
import 'package:openmuse_mobile/speech_model_manager.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile_plugin_market/src/market_discover_view.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_registry_provider/openmuse_registry_provider.dart';
import 'package:openmuse_speech_input/openmuse_speech_input.dart';
import 'package:path_provider/path_provider.dart';
import '../test/support/design_fixture.dart';
import 'support/device_result.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  // Establish accessibility before testWidgets records its handle baseline.
  // iOS asynchronously echoes the engine's semantics request; doing this inside
  // a test can make its platform-owned handle look like a test-owned leak.
  SemanticsHandle? suiteSemantics;
  setUpAll(() async {
    suiteSemantics = binding.ensureSemantics();
    await Future<void>.delayed(const Duration(seconds: 1));
  });
  tearDownAll(() => suiteSemantics?.dispose());
  testWidgets('mobile settings visual and model download acceptance', (
    tester,
  ) async {
    final support = await getApplicationSupportDirectory();
    final output = Directory('${support.path}/acceptance-design');
    await output.create(recursive: true);
    final locale = OpenMuseLocaleController(initialTag: 'en');
    final auth = DesignAuthentication();
    final providers = McpRegistryProviders(
      file: File('${output.path}/providers.json'),
      defaultOrigin: Uri.parse('https://us.openmuseai.com'),
    );
    await providers.load();
    final models = SpeechModelManager(
      SherpaZipformerConfig.zh14m('${support.path}/speech-model'),
      downloadOrigin: Uri.parse('https://us.openmuseai.com'),
    );
    if (Platform.isAndroid) await binding.convertFlutterSurfaceToImage();
    Future<void> shot(String name) async {
      await tester.pump(const Duration(milliseconds: 500));
      await tester.pumpAndSettle();
      final png = await binding.takeScreenshot(name);
      await File('${output.path}/$name.png').writeAsBytes(png);
    }

    MobileMyPage account(ThemeMode mode) => MobileMyPage(
      themeMode: mode,
      onThemeMode: (_) {},
      workspaces: const [],
      deviceName: 'Linux Desktop',
      authentication: auth,
      registryProviders: providers,
      speechModels: models,
      onSignOut: () {},
      onDeleteAccount: () async {},
    );
    for (final brightness in [Brightness.light, Brightness.dark]) {
      await tester.pumpWidget(
        DesignHost(
          key: ValueKey(brightness),
          locale: locale,
          brightness: brightness,
          home: account(
            brightness == Brightness.light ? ThemeMode.light : ThemeMode.dark,
          ),
        ),
      );
      await shot('account-${brightness.name}-en');
      await tester.tap(find.byKey(const ValueKey('my-account')));
      await shot('account-details-${brightness.name}');
      expect(find.byKey(const ValueKey('my-sign-out')), findsOneWidget);
      expect(find.byKey(const ValueKey('my-delete-account')), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('settings-back')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('my-language')));
      await shot('language-${brightness.name}');
      await tester.tap(find.text('中文'));
      await tester.pumpAndSettle();
      await shot('account-${brightness.name}-zh');
      locale.selectTag('en');
      await tester.pumpAndSettle();
      expect(tester.takeException(), isNull);
    }
    await tester.ensureVisible(find.byKey(const ValueKey('my-store-provider')));
    await tester.tap(find.byKey(const ValueKey('my-store-provider')));
    await shot('store-provider');
    await tester.tap(find.text('us.openmuseai.com'));
    await tester.pumpAndSettle();
    expect(await providers.file.exists(), isTrue);
    await tester.tap(find.byKey(const ValueKey('settings-back')));
    await tester.pumpAndSettle();
    await tester.ensureVisible(find.byKey(const ValueKey('my-speech-models')));
    await tester.tap(find.byKey(const ValueKey('my-speech-models')));
    await shot('speech-model-before');
    if (!models.installed) {
      await tester.tap(find.byKey(const ValueKey('speech-model-install')));
      for (
        var i = 0;
        i < 900 && !models.installed && (models.busy || i == 0);
        i++
      ) {
        await tester.pump(const Duration(milliseconds: 400));
      }
    }
    expect(
      models.failure,
      isNull,
      reason:
          'The real pinned model download must succeed: ${models.diagnostic}',
    );
    expect(models.installed, isTrue);
    await shot('speech-model-installed');
    expect(find.byKey(const ValueKey('speech-model-remove')), findsOneWidget);
    debugPrint(
      'SPEECH_MODEL_DEVICE_ACCEPTANCE_PASS platform=${Platform.operatingSystem} verifiedFiles=4 bytes=${SpeechModelManager.totalBytes}',
    );
    final store = MobilePluginStore(Directory('${output.path}/market'));
    await tester.pumpWidget(
      DesignHost(
        key: const ValueKey('discover'),
        locale: locale,
        brightness: Brightness.light,
        home: MarketDiscoverView(
          entries: const [],
          store: store,
          category: '精选',
          mine: false,
          query: '',
          onSection: (_) {},
          onCategory: (_) {},
          onSearch: (_) {},
          onDetail: (_) {},
          onInstall: (_) {},
          onOpen: (_) {},
          onPreview: (_) {},
          desktopSection: const Text('desktop section must not render'),
        ),
      ),
    );
    await shot('discover');
    expect(find.text('desktop section must not render'), findsNothing);
    final controller = WorkBuddyController(strings: locale.strings);
    await tester.pumpWidget(
      DesignHost(
        key: const ValueKey('tasks'),
        locale: locale,
        brightness: Brightness.light,
        home: WorkBuddyShell(controller: controller),
      ),
    );
    await shot('tasks-light');
    await tester.tap(find.byKey(const ValueKey('wb-input-mode-toggle')));
    await tester.pumpAndSettle();
    await shot('tasks-voice-light');
    expect(tester.takeException(), isNull);
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pumpAndSettle();
    controller.dispose();
    models.dispose();
    auth.dispose();
    providers.dispose();
    locale.dispose();
    debugPrint(
      'MOBILE_DESIGN_DEVICE_ACCEPTANCE_PASS themes=light,dark locales=en,zh settings=navigable discover=singleRow',
    );
  });
}
