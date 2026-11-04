import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_mobile/mobile_plugins/scheduled_tasks_plugin.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_theme.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('theme follows My setting across Discover and the workbench', (
    tester,
  ) async {
    final directory = await Directory.systemTemp.createTemp(
      'openmuse-shell-theme-',
    );
    final controller = WorkBuddyController();
    final market = OpenMuseMobilePluginMarket(MobilePluginStore(directory));
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    final schedules = MobileScheduledTasksPlugin();
    final values = _MemoryValues();
    registry.install(schedules);
    await registry.activate(schedules.descriptor.id);
    addTearDown(() async {
      await tester.pumpWidget(const SizedBox.shrink());
      controller.dispose();
      registry.dispose();
      await directory.delete(recursive: true);
    });
    var mode = ThemeMode.light;
    await tester.pumpWidget(
      StatefulBuilder(
        builder: (context, setTheme) => MaterialApp(
          theme: workBuddyTheme(brightness: Brightness.light),
          darkTheme: workBuddyTheme(brightness: Brightness.dark),
          themeMode: mode,
          home: WorkBuddyShell(
            controller: controller,
            mobileMarketPlugin: market,
            pluginRegistry: registry,
            deviceSelectionStore: values,
            themeMode: mode,
            onThemeMode: (value) => setTheme(() => mode = value),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    for (final tab in ['任务', '工作台', '发现', '我的']) {
      expect(find.byKey(ValueKey('wb-tab-$tab')), findsOneWidget);
    }

    await tester.tap(find.byKey(const ValueKey('wb-tab-发现')));
    await tester.pumpAndSettle();
    final search = find.byKey(const ValueKey('mobile-market-search'));
    expect(search, findsOneWidget);
    expect(Theme.of(tester.element(search)).brightness, Brightness.light);
    await tester.tap(find.byKey(const ValueKey('mobile-market-section-我的')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('market-category-更新')), findsOneWidget);

    await tester.tap(find.byKey(const ValueKey('wb-tab-我的')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('my-theme-mode')), findsOneWidget);
    await tester.tap(find.text('深色'));
    await tester.pumpAndSettle();
    expect(mode, ThemeMode.dark);
    expect(
      Theme.of(
        tester.element(find.byKey(const ValueKey('my-theme-mode'))),
      ).brightness,
      Brightness.dark,
    );

    await tester.tap(find.byKey(const ValueKey('wb-tab-发现')));
    await tester.pumpAndSettle();
    expect(Theme.of(tester.element(search)).brightness, Brightness.dark);
    await tester.tap(find.byKey(const ValueKey('wb-tab-工作台')));
    await tester.pumpAndSettle();
    final card = find.byKey(
      ValueKey('workbench-card-${schedules.descriptor.id}'),
    );
    expect(card, findsOneWidget);
    expect(Theme.of(tester.element(card)).brightness, Brightness.dark);
    expect(tester.takeException(), isNull);
  });
}

final class _MemoryValues implements SecureValueStore {
  final values = <String, String>{};

  @override
  Future<String?> read(String key) async => values[key];

  @override
  Future<void> write(String key, String value) async => values[key] = value;

  @override
  Future<void> delete(String key) async => values.remove(key);
}
