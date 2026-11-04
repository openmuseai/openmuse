// Runs the production bootstrap and real registry on a connected iPhone.
import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_mobile/main.dart' as mobile;
import 'package:openmuse_mobile/mobile_plugins/mobile_workbench_page.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path_provider/path_provider.dart';
import 'support/device_result.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  SemanticsHandle? semantics;
  setUpAll(() async {
    semantics = binding.ensureSemantics();
    await Future<void>.delayed(const Duration(seconds: 1));
  });
  tearDownAll(() => semantics?.dispose());
  testWidgets('iOS production four-tab Apps Center UI acceptance', (
    tester,
  ) async {
    tester.testTextInput.register();
    addTearDown(tester.testTextInput.unregister);
    final support = await getApplicationSupportDirectory();
    final output = await Directory(
      '${support.path}/acceptance-apps-center',
    ).create(recursive: true);
    final proof = <String, Object?>{
      'run': DateTime.now().toUtc().toIso8601String(),
      'productionBootstrap': true,
    };
    Future<void> phase(String name) async {
      proof['phase'] = name;
      await File(
        '${output.path}/proof.json',
      ).writeAsString(jsonEncode(proof), flush: true);
      debugPrint('Apps acceptance phase: $name');
    }

    Future<void> until(bool Function() condition, String label) async {
      final deadline = DateTime.now().add(const Duration(seconds: 90));
      while (DateTime.now().isBefore(deadline)) {
        await tester.pump(const Duration(milliseconds: 150));
        if (condition()) return;
        await Future<void>.delayed(const Duration(milliseconds: 150));
      }
      throw TestFailure('Timed out: $label');
    }

    Future<void> tap(String key) async {
      final finder = find.byKey(ValueKey(key));
      await until(() => finder.evaluate().isNotEmpty, key);
      await tester.ensureVisible(finder);
      await tester.pump(const Duration(milliseconds: 200));
      await tester.tap(finder);
      await tester.pump(const Duration(milliseconds: 450));
    }

    Future<void> shot(String name) async {
      await tester.pump(const Duration(milliseconds: 500));
      await File(
        '${output.path}/$name.png',
      ).writeAsBytes(await binding.takeScreenshot(name));
    }

    await phase('boot');
    await mobile.main();
    await until(
      () =>
          find.byType(mobile.OpenMuseMobileApplication).evaluate().isNotEmpty ||
          find.byKey(const ValueKey('region-us')).evaluate().isNotEmpty,
      'bootstrap',
    );
    if (find.byKey(const ValueKey('region-us')).evaluate().isNotEmpty) {
      await tap('region-us');
      await tap('region-continue');
    }
    await until(
      () => find.byType(WorkBuddyShell).evaluate().isNotEmpty,
      'shell',
    );
    final app = tester.widget<mobile.OpenMuseMobileApplication>(
      find.byType(mobile.OpenMuseMobileApplication),
    );
    final originalLanguage = app.localeController?.tag;
    app.localeController?.selectTag('en');
    await tester.pump(const Duration(seconds: 1));
    WorkBuddyShell shell() =>
        tester.widget<WorkBuddyShell>(find.byType(WorkBuddyShell));
    final originalTheme = shell().themeMode;
    shell().onThemeMode?.call(ThemeMode.light);
    await tester.pump(const Duration(seconds: 1));
    final buddy = shell().controller;
    final registry = shell().pluginRegistry!;
    final market = shell().mobileMarketPlugin!;
    final values =
        shell().deviceSelectionStore ?? const FlutterSecureValueStore();
    String? layoutKey, originalLayout;
    try {
      await phase('overview');
      await tap('wb-tab-Apps');
      expect(buddy.appsSection, AppsSection.overview);
      expect(
        find.byWidgetPredicate(
          (w) =>
              w is GestureDetector &&
              w.key is ValueKey<String> &&
              (w.key as ValueKey<String>).value.startsWith('wb-tab-'),
        ),
        findsNWidgets(4),
      );
      await shot('01-overview-en-light');
      await phase('discover-state');
      await tap('apps-section-discover');
      await until(
        () => find
            .byKey(const ValueKey('mobile-market-search'))
            .evaluate()
            .isNotEmpty,
        'catalog loaded',
      );
      await shot('02-discover-en-light');
      await tester.enterText(
        find.byKey(const ValueKey('mobile-market-search')),
        'notes',
      );
      await tester.pump(const Duration(milliseconds: 500));
      await tap('wb-tab-Tasks');
      await tap('wb-tab-Apps');
      expect(buddy.appsSection, AppsSection.discover);
      expect(
        tester
            .widget<TextField>(
              find.byKey(const ValueKey('mobile-market-search')),
            )
            .controller!
            .text,
        'notes',
      );
      await tap('wb-tab-Apps');
      expect(buddy.appsSection, AppsSection.overview);
      await phase('edit-guard-and-save');
      final workbench = tester.widget<MobileWorkbenchPage>(
        find.byType(MobileWorkbenchPage),
      );
      layoutKey =
          'openmuse.mobile.workbench.v1.${workbench.subject}.${workbench.scope}';
      originalLayout = await values.read(layoutKey);
      final contributor = registry.descriptors
          .map((d) => registry.plugin(d.id))
          .whereType<OpenMuseMobileWorkbenchContributor>()
          .firstWhere(
            (p) =>
                registry.stateOf((p as OpenMusePlugin).descriptor.id) ==
                OpenMusePluginState.active,
          );
      final pluginId = (contributor as OpenMusePlugin).descriptor.id;
      await tap('apps-edit');
      await tap('workbench-visible-$pluginId');
      await tap('wb-tab-Account');
      await shot('03-edit-leave-guard');
      await tap('apps-edit-keep');
      expect(buddy.tab, WbTab.apps);
      await tap('wb-tab-Account');
      await tap('apps-edit-discard');
      expect(buddy.tab, WbTab.mine);
      await tap('wb-tab-Apps');
      await tap('apps-edit');
      await tap('workbench-visible-$pluginId');
      await tap('apps-edit-done');
      await until(
        () => find.byKey(const ValueKey('apps-edit-done')).evaluate().isEmpty,
        'saved layout',
      );
      expect(await values.read(layoutKey), contains('"hidden"'));
      await tap('apps-edit');
      await tap('workbench-visible-$pluginId');
      await tap('apps-edit-done');
      await phase('account-and-management');
      await tap('wb-tab-Account');
      await shot('04-account-en-light');
      await tap('my-plugins');
      await shot('05-my-plugins-en-light');
      expect(
        find.byKey(const ValueKey('wb-tab-Apps')).hitTestable(),
        findsNothing,
      );
      await tap('market-category-已停用');
      await tap('market-discover-more');
      expect(buddy.tab, WbTab.apps);
      expect(buddy.appsSection, AppsSection.discover);
      // Real pointer access to the root search proves no hidden modal barrier.
      await tester.enterText(
        find.byKey(const ValueKey('mobile-market-search')),
        'calendar',
      );
      await tester.pump(const Duration(milliseconds: 500));
      await tap('apps-return-to-plugins');
      expect(buddy.tab, WbTab.mine);
      expect(
        tester
            .widget<ChoiceChip>(
              find.byKey(const ValueKey('market-category-已停用')),
            )
            .selected,
        isTrue,
      );
      await tap('market-discover-more');
      await tester.dragFrom(const Offset(4, 150), const Offset(170, 0));
      await tester.pump(const Duration(milliseconds: 600));
      expect(buddy.tab, WbTab.mine);
      await tap('market-category-全部');
      await tap('market-built-in-$pluginId');
      await shot('06-plugin-management');
      await tap('installed-plugin-toggle');
      await until(
        () => registry.stateOf(pluginId) != OpenMusePluginState.active,
        'deactivated',
      );
      expect(
        await MobileBuiltInPreferences(
          market.builtInPreferences.file,
        ).isEnabled(pluginId),
        isFalse,
      );
      await tap('installed-plugin-toggle');
      await until(
        () => registry.stateOf(pluginId) == OpenMusePluginState.active,
        'reactivated',
      );
      await tester.pageBack();
      await tester.pump(const Duration(milliseconds: 500));
      await tester.pageBack();
      await tester.pump(const Duration(milliseconds: 500));
      await phase('localized-dark');
      app.localeController?.selectTag('zh-Hans');
      shell().onThemeMode?.call(ThemeMode.dark);
      await tester.pump(const Duration(seconds: 1));
      await shot('07-account-zh-dark');
      await tap('wb-tab-应用');
      await tap('wb-tab-应用');
      await shot('08-overview-zh-dark');
      await tap('apps-section-discover');
      await tester.enterText(
        find.byKey(const ValueKey('mobile-market-search')),
        '',
      );
      await tester.pump(const Duration(milliseconds: 500));
      await shot('09-discover-zh-dark');
      expect(tester.takeException(), isNull);
      proof['checks'] = [
        'four-tabs',
        'overview-default',
        'search-retained',
        'reselection-overview',
        'edit-keep-discard-save',
        'inventory-filter-return',
        'root-pointer-unblocked',
        'enablement-durable',
        'light-dark',
        'en-zh',
      ];
      proof['passed'] = true;
      await phase('complete');
    } finally {
      if (layoutKey != null) {
        if (originalLayout == null) {
          await values.delete(layoutKey);
        } else {
          await values.write(layoutKey, originalLayout);
        }
      }
      if (originalLanguage != null) {
        app.localeController?.selectTag(originalLanguage);
      }
      if (find.byType(WorkBuddyShell).evaluate().isNotEmpty) {
        shell().onThemeMode?.call(originalTheme);
      }
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pump(const Duration(seconds: 1));
    }
  });
}
