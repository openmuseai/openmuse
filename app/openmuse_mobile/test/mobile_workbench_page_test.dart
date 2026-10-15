import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_mobile/mobile_plugins/mobile_my_page.dart';
import 'package:openmuse_mobile/mobile_plugins/mobile_workbench_page.dart';
import 'package:openmuse_mobile/mobile_plugins/scheduled_tasks_plugin.dart';
import 'package:openmuse_mobile/mobile_plugins/workspace_files_plugin.dart';

void main() {
  testWidgets(
    'active plugin previews appear and disabled plugins leave the workbench',
    (tester) async {
      tester.view.physicalSize = const Size(390, 844);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final schedules = MobileScheduledTasksPlugin();
      final files = MobileWorkspaceFilesPlugin();
      registry.install(schedules);
      registry.install(files);
      await registry.activate(schedules.descriptor.id);
      await registry.activate(files.descriptor.id);
      final values = _MemoryValues();
      await tester.pumpWidget(
        MaterialApp(
          home: MobileWorkbenchPage(
            registry: registry,
            values: values,
            subject: 'test-user',
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(
        find.byKey(ValueKey('workbench-card-${schedules.descriptor.id}')),
        findsOneWidget,
      );
      expect(
        find.byKey(ValueKey('workbench-card-${files.descriptor.id}')),
        findsOneWidget,
      );
      expect(tester.takeException(), isNull);

      await tester.tap(find.byKey(const ValueKey('workbench-edit')));
      await tester.pump();
      await tester.tap(
        find.byKey(ValueKey('workbench-pin-${files.descriptor.id}')),
      );
      await tester.tap(find.byKey(const ValueKey('workbench-done')));
      await tester.pumpAndSettle();
      expect(values.data.values.single, contains(files.descriptor.id));
      await registry.deactivate(files.descriptor.id);
      await tester.pumpAndSettle();
      expect(
        find.byKey(ValueKey('workbench-card-${files.descriptor.id}')),
        findsNothing,
      );
    },
  );

  testWidgets(
    'My theme selector fits a narrow phone and invokes the app theme',
    (tester) async {
      tester.view.physicalSize = const Size(360, 800);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      ThemeMode? selected;
      await tester.pumpWidget(
        MaterialApp(
          home: MobileMyPage(
            themeMode: ThemeMode.system,
            onThemeMode: (value) => selected = value,
            workspaces: const [],
            deviceName: 'Desktop',
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(tester.takeException(), isNull);
      await tester.tap(find.text('深色'));
      await tester.pump();
      expect(selected, ThemeMode.dark);
      expect(tester.takeException(), isNull);
    },
  );

  testWidgets('workbench drops to one column on a 320 dp phone', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(320, 740);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    final schedules = MobileScheduledTasksPlugin();
    final files = MobileWorkspaceFilesPlugin();
    registry.install(schedules);
    registry.install(files);
    await registry.activate(schedules.descriptor.id);
    await registry.activate(files.descriptor.id);
    await tester.pumpWidget(
      MaterialApp(
        home: MobileWorkbenchPage(
          registry: registry,
          values: _MemoryValues(),
          subject: 'narrow',
        ),
      ),
    );
    await tester.pumpAndSettle();
    final first = tester.getTopLeft(
      find.byKey(ValueKey('workbench-card-${schedules.descriptor.id}')),
    );
    final second = tester.getTopLeft(
      find.byKey(ValueKey('workbench-card-${files.descriptor.id}')),
    );
    expect(first.dx, second.dx);
    expect(second.dy, greaterThan(first.dy));
    expect(tester.takeException(), isNull);
  });

  testWidgets('Discover and My show registry-backed first-party plugins', (
    tester,
  ) async {
    final root = Directory(
      '${Directory.systemTemp.path}/mobile-market-test-only',
    );
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    final schedules = MobileScheduledTasksPlugin();
    registry.install(schedules);
    await registry.activate(schedules.descriptor.id);
    OpenMusePlugin? opened;
    await tester.pumpWidget(
      MaterialApp(
        home: MobileMarketPage(
          store: MobilePluginStore(root),
          pluginRegistry: registry,
          onOpenBuiltIn: (plugin) => opened = plugin,
        ),
      ),
    );
    await tester.pump(const Duration(milliseconds: 500));
    final key = ValueKey('market-built-in-${schedules.descriptor.id}');
    expect(find.byKey(key), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('mobile-market-section-我的')));
    await tester.pump(const Duration(milliseconds: 500));
    expect(find.byKey(key), findsOneWidget);
    await tester.ensureVisible(find.byKey(key));
    await tester.tap(find.byKey(key));
    expect(opened, same(schedules));
  });
}

final class _MemoryValues implements SecureValueStore {
  final data = <String, String>{};

  @override
  Future<String?> read(String key) async => data[key];

  @override
  Future<void> write(String key, String value) async => data[key] = value;

  @override
  Future<void> delete(String key) async => data.remove(key);
}
