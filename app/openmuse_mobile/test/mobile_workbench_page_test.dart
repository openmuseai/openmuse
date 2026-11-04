import 'support/localized_test_app.dart';
import 'dart:io';
import 'dart:convert';

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
    'active previews stop when disabled and the card keeps a recovery path',
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
      var manageOpened = false;
      await tester.pumpWidget(
        localizedTestApp(
          home: MobileWorkbenchPage(
            registry: registry,
            values: values,
            subject: 'test-user',
            onManagePlugins: () => manageOpened = true,
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
        findsOneWidget,
      );
      expect(
        find.byKey(ValueKey('workbench-preview-${files.descriptor.id}')),
        findsNothing,
      );
      await tester.tap(find.text('管理插件'));
      expect(manageOpened, isTrue);
    },
  );

  testWidgets('card size can be cancelled or saved and restored', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    final schedules = MobileScheduledTasksPlugin();
    registry.install(schedules);
    await registry.activate(schedules.descriptor.id);
    final values = _MemoryValues();
    Widget page() => localizedTestApp(
      home: MobileWorkbenchPage(
        registry: registry,
        values: values,
        subject: 'size-user',
      ),
    );
    final card = find.byKey(
      ValueKey('workbench-card-${schedules.descriptor.id}'),
    );
    final size = find.byKey(
      ValueKey('workbench-size-${schedules.descriptor.id}'),
    );
    await tester.pumpWidget(page());
    await tester.pumpAndSettle();
    final defaultHeight = tester.getSize(card).height;
    final defaultWidth = tester.getSize(card).width;

    await tester.tap(find.byKey(const ValueKey('workbench-edit')));
    await tester.pumpAndSettle();
    await tester.tap(size);
    await tester.pumpAndSettle();
    await tester.tap(find.text('扩展高度'));
    await tester.pumpAndSettle();
    expect(tester.getSize(card).height, greaterThan(defaultHeight));
    await tester.tap(size);
    await tester.pumpAndSettle();
    await tester.tap(find.text('跨两列'));
    await tester.pumpAndSettle();
    expect(tester.getSize(card).width, greaterThan(defaultWidth));
    await tester.tap(find.byKey(const ValueKey('workbench-edit')));
    await tester.pumpAndSettle();
    expect(tester.getSize(card).height, defaultHeight);
    expect(tester.getSize(card).width, defaultWidth);

    await tester.tap(find.byKey(const ValueKey('workbench-edit')));
    await tester.pumpAndSettle();
    await tester.tap(size);
    await tester.pumpAndSettle();
    await tester.tap(find.text('扩展高度'));
    await tester.pumpAndSettle();
    await tester.tap(size);
    await tester.pumpAndSettle();
    await tester.tap(find.text('跨两列'));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('workbench-done')));
    await tester.pumpAndSettle();
    final saved = jsonDecode(values.data.values.single) as Map;
    expect((saved['heights'] as Map)[schedules.descriptor.id], 'large');
    expect((saved['columns'] as Map)[schedules.descriptor.id], 2);
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pumpWidget(page());
    await tester.pumpAndSettle();
    expect(tester.getSize(card).height, greaterThan(defaultHeight));
    expect(tester.getSize(card).width, greaterThan(defaultWidth));
    expect(tester.takeException(), isNull);
  });

  testWidgets(
    'My theme selector fits a narrow phone and invokes the app theme',
    (tester) async {
      tester.view.physicalSize = const Size(360, 800);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      ThemeMode? selected;
      await tester.pumpWidget(
        localizedTestApp(
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

  testWidgets('drag order and pin survive a workbench rebuild', (tester) async {
    tester.view.physicalSize = const Size(390, 844);
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
    final values = _MemoryValues();
    Widget page() => localizedTestApp(
      home: MobileWorkbenchPage(
        registry: registry,
        values: values,
        subject: 'layout-user',
      ),
    );
    await tester.pumpWidget(page());
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('workbench-edit')));
    await tester.pump();
    final source = find.byKey(
      ValueKey('workbench-card-${files.descriptor.id}'),
    );
    final target = find.byKey(
      ValueKey('workbench-card-${schedules.descriptor.id}'),
    );
    final gesture = await tester.startGesture(tester.getCenter(source));
    await tester.pump(const Duration(milliseconds: 650));
    await gesture.moveTo(tester.getCenter(target));
    await tester.pump();
    await gesture.up();
    await tester.pumpAndSettle();
    await tester.tap(
      find.byKey(ValueKey('workbench-pin-${files.descriptor.id}')),
    );
    await tester.tap(find.byKey(const ValueKey('workbench-done')));
    await tester.pumpAndSettle();
    final saved = jsonDecode(values.data.values.single) as Map;
    expect((saved['order'] as List).first, files.descriptor.id);
    expect(saved['pinned'], contains(files.descriptor.id));
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pumpWidget(page());
    await tester.pumpAndSettle();
    expect(
      tester.getTopLeft(source).dx,
      lessThan(tester.getTopLeft(target).dx),
    );
    expect(find.byIcon(Icons.push_pin), findsOneWidget);
    expect(tester.takeException(), isNull);
  });

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
      localizedTestApp(
        builder: (context, child) => MediaQuery(
          data: MediaQuery.of(
            context,
          ).copyWith(textScaler: const TextScaler.linear(1.5)),
          child: child!,
        ),
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

  testWidgets('downloaded WebView plugin contributes its verified A2UI card', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final decoded =
        jsonDecode(
              File(
                '../../packages/openmuse_mobile_plugin_runtime/assets/task-board.json',
              ).readAsStringSync(),
            )
            as Map;
    final bundle = MobilePluginBundle.fromJson(
      Map<String, Object?>.from(decoded),
    );
    final plugin = OpenMuseWebViewInstalledPlugin(bundle, bundle.manifest.name);
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    registry.install(plugin);
    await registry.activate(bundle.id);
    await tester.pumpWidget(
      localizedTestApp(
        home: MobileWorkbenchPage(
          registry: registry,
          values: _MemoryValues(),
          subject: 'web-card-user',
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.byKey(ValueKey('workbench-card-${bundle.id}')), findsOneWidget);
    expect(find.text('本地任务清单'), findsOneWidget);
    expect(tester.takeException(), isNull);
    await registry.deactivate(bundle.id);
    await tester.pumpAndSettle();
    expect(find.byKey(ValueKey('workbench-card-${bundle.id}')), findsOneWidget);
    expect(
      find.byKey(ValueKey('workbench-preview-${bundle.id}')),
      findsNothing,
    );
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
      localizedTestApp(
        home: MobileMarketPage(
          store: MobilePluginStore(root),
          pluginRegistry: registry,
          onOpenBuiltIn: (plugin) => opened = plugin,
        ),
      ),
    );
    await tester.pump(const Duration(milliseconds: 500));
    final key = ValueKey('market-built-in-${schedules.descriptor.id}');
    await tester.scrollUntilVisible(
      find.byKey(key),
      260,
      scrollable: find.byType(Scrollable).first,
    );
    expect(find.byKey(key), findsOneWidget);
    await tester.scrollUntilVisible(
      find.byKey(const ValueKey('mobile-market-section-我的')),
      -260,
      scrollable: find.byType(Scrollable).first,
    );
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
