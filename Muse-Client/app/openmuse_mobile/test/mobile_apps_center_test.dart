import 'dart:ui' as ui;
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_mobile_plugin_market/src/market_discover_view.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/mobile_plugins/scheduled_tasks_plugin.dart';
import 'package:openmuse_mobile/mobile_plugins/workspace_files_plugin.dart';
import 'support/design_fixture.dart';

final class Values implements SecureValueStore {
  final data = <String, String>{};
  bool fail = false;
  @override
  Future<String?> read(String key) async => data[key];
  @override
  Future<void> write(String key, String value) async {
    if (fail) throw const FileSystemException('test write failure');
    data[key] = value;
  }

  @override
  Future<void> delete(String key) async => data.remove(key);
}

final class EmptyCatalog implements MobileMarketSource {
  @override
  Future<List<MobileMarketEntry>> list() async => [];
  @override
  Future<List<int>> artifact(MobileMarketEntry entry) async =>
      throw UnimplementedError();
}

final class PendingCatalog implements MobileMarketSource {
  final response = Completer<List<MobileMarketEntry>>();
  int calls = 0;
  @override
  Future<List<MobileMarketEntry>> list() {
    calls++;
    return response.future;
  }

  @override
  Future<List<int>> artifact(MobileMarketEntry entry) async =>
      throw StateError('offline');
}

void main() {
  late Directory root;
  late OpenMusePluginRegistry registry;
  late WorkBuddyController controller;
  late OpenMuseLocaleController locale;
  late OpenMuseMobilePluginMarket market;
  late MobileScheduledTasksPlugin schedules;
  late Values values;
  setUp(() async {
    root = Directory.systemTemp.createTempSync('apps-center-');
    locale = OpenMuseLocaleController(initialTag: 'en');
    controller = WorkBuddyController(strings: locale.strings);
    values = Values();
    registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    schedules = MobileScheduledTasksPlugin(strings: locale.strings);
    final files = MobileWorkspaceFilesPlugin(strings: locale.strings);
    market = OpenMuseMobilePluginMarket(
      MobilePluginStore(root),
      source: EmptyCatalog(),
    );
    for (final plugin in <OpenMusePlugin>[schedules, files, market]) {
      registry.install(plugin);
      await registry.activate(plugin.descriptor.id);
    }
  });
  tearDown(() async {
    registry.dispose();
    controller.dispose();
    locale.dispose();
    root.deleteSync(recursive: true);
  });
  Future<void> show(
    WidgetTester tester, {
    double width = 390,
    double scale = 1,
    Brightness brightness = Brightness.light,
    Key? captureKey,
  }) async {
    tester.view.physicalSize = Size(width, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    await tester.pumpWidget(
      DesignHost(
        locale: locale,
        brightness: brightness,
        captureKey: captureKey,
        fontFamily: captureKey == null ? null : 'AppsEvidence',
        textScale: scale,
        home: WorkBuddyShell(
          controller: controller,
          pluginRegistry: registry,
          mobileMarketPlugin: market,
          deviceSelectionStore: values,
        ),
      ),
    );
    await tester.pumpAndSettle();
  }

  Future<void> tap(WidgetTester tester, String key) async {
    final f = find.byKey(ValueKey(key));
    await tester.ensureVisible(f);
    await tester.tap(f);
    await tester.pumpAndSettle();
  }

  testWidgets(
    'four destinations preserve Tasks draft and independent discovery search; reselect returns overview',
    (tester) async {
      await show(tester);
      expect(
        find.byWidgetPredicate(
          (w) =>
              w.key is ValueKey<String> &&
              (w.key as ValueKey<String>).value.startsWith('wb-tab-'),
        ),
        findsNWidgets(4),
      );
      await tester.enterText(
        find.byKey(const ValueKey('wb-composer')),
        'Keep this task draft',
      );
      controller.openTaskId = 'existing-task';
      await tap(tester, 'wb-tab-Apps');
      expect(controller.appsSection, AppsSection.overview);
      expect(find.byKey(const ValueKey('apps-edit')), findsOneWidget);
      await tap(tester, 'apps-section-discover');
      expect(
        find.byKey(const ValueKey('mobile-market-sections')),
        findsNothing,
      );
      await tester.enterText(
        find.byKey(const ValueKey('mobile-market-search')),
        'unmatched query',
      );
      await tester.pumpAndSettle();
      await tap(tester, 'wb-tab-Tasks');
      expect(
        tester
            .widget<TextField>(find.byKey(const ValueKey('wb-composer')))
            .controller!
            .text,
        'Keep this task draft',
      );
      expect(controller.openTaskId, 'existing-task');
      await tap(tester, 'wb-tab-Apps');
      expect(controller.appsSection, AppsSection.discover);
      expect(
        tester
            .widget<TextField>(
              find.byKey(const ValueKey('mobile-market-search')),
            )
            .controller!
            .text,
        'unmatched query',
      );
      await tap(tester, 'wb-tab-Apps');
      expect(controller.appsSection, AppsSection.overview);
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );
  testWidgets(
    'edit leave guard keeps or discards changes; Account inventory uses the same discovery surface and returns to its filter',
    (tester) async {
      await show(tester);
      await tap(tester, 'wb-tab-Apps');
      await tap(tester, 'apps-edit');
      await tap(tester, 'workbench-visible-${schedules.descriptor.id}');
      await tap(tester, 'wb-tab-Account');
      expect(find.byKey(const ValueKey('apps-edit-keep')), findsOneWidget);
      await tap(tester, 'apps-edit-keep');
      expect(controller.tab, WbTab.apps);
      await tap(tester, 'wb-tab-Account');
      await tap(tester, 'apps-edit-discard');
      expect(controller.tab, WbTab.mine);
      await tap(tester, 'my-plugins');
      expect(
        find.byKey(const ValueKey('wb-tab-Apps')).hitTestable(),
        findsNothing,
      );
      await tap(tester, 'market-category-已停用');
      await tap(tester, 'market-discover-more');
      expect(controller.tab, WbTab.apps);
      expect(controller.appsSection, AppsSection.discover);
      expect(
        find.byKey(const ValueKey('apps-return-to-plugins')),
        findsOneWidget,
      );
      expect(
        find.byKey(const ValueKey('mobile-market-search')),
        findsOneWidget,
      );
      await tap(tester, 'apps-section-overview');
      expect(controller.appsSection, AppsSection.overview);
      await tap(tester, 'apps-section-discover');
      await tap(tester, 'apps-return-to-plugins');
      expect(controller.tab, WbTab.mine);
      final selected = tester.widget<ChoiceChip>(
        find.byKey(const ValueKey('market-category-已停用')),
      );
      expect(selected.selected, isTrue);
      await tester.pumpWidget(const SizedBox.shrink());
      expect(tester.takeException(), isNull);
    },
  );
  testWidgets(
    'iOS edge return restores the originating plugin filter',
    (tester) async {
      await show(tester);
      await tap(tester, 'wb-tab-Account');
      await tap(tester, 'my-plugins');
      await tap(tester, 'market-category-已停用');
      await tap(tester, 'market-discover-more');
      await tester.dragFrom(const Offset(4, 150), const Offset(170, 0));
      await tester.pumpAndSettle();
      expect(controller.tab, WbTab.mine);
      expect(
        tester
            .widget<ChoiceChip>(
              find.byKey(const ValueKey('market-category-已停用')),
            )
            .selected,
        isTrue,
      );
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
    },
    variant: TargetPlatformVariant.only(TargetPlatform.iOS),
  );
  testWidgets(
    'failed layout save retains editing and persisted changes restore on next mount',
    (tester) async {
      await show(tester);
      await tap(tester, 'wb-tab-Apps');
      await tap(tester, 'apps-edit');
      await tap(tester, 'workbench-visible-${schedules.descriptor.id}');
      values.fail = true;
      await tap(tester, 'apps-edit-done');
      expect(find.byKey(const ValueKey('apps-edit-done')), findsOneWidget);
      expect(values.data, isEmpty);
      values.fail = false;
      await tap(tester, 'apps-edit-done');
      expect(
        values.data.values.single,
        contains('"hidden":["${schedules.descriptor.id}"]'),
      );
      expect(
        find.byKey(ValueKey('workbench-card-${schedules.descriptor.id}')),
        findsNothing,
      );
      await tester.pumpWidget(const SizedBox.shrink());
      await show(tester);
      await tap(tester, 'wb-tab-Apps');
      expect(
        find.byKey(ValueKey('workbench-card-${schedules.descriptor.id}')),
        findsNothing,
      );
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );
  testWidgets(
    'installed local bundle stays manageable when the catalog hangs or delists it',
    (tester) async {
      final source = PendingCatalog();
      final store = MobilePluginStore(root);
      late MobileMarketEntry entry;
      await tester.runAsync(() async {
        const assets = '../../packages/openmuse_mobile_plugin_runtime/assets';
        final records =
            jsonDecode(await File('$assets/market.json').readAsString())
                as List;
        entry = records
            .map(
              (e) => MobileMarketEntry.fromJson(
                Map<String, Object?>.from(e as Map),
              ),
            )
            .first;
        await store.install(
          entry,
          await File('$assets/${entry.bundle}').readAsBytes(),
        );
      });
      final adapter = OpenMuseWebViewInstalledPlugin(
        store.installed(entry.id)!,
        entry.name,
      );
      registry.install(adapter);
      await registry.activate(entry.id);
      await tester.pumpWidget(
        DesignHost(
          locale: locale,
          brightness: Brightness.light,
          home: MobileMarketPage(
            store: store,
            source: source,
            mode: MobileMarketMode.installed,
            pluginRegistry: registry,
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(
        find.byKey(ValueKey('market-installed-${entry.id}')),
        findsOneWidget,
      );
      expect(source.calls, 1);
      expect(find.byKey(ValueKey('market-built-in-${entry.id}')), findsNothing);
      source.response.complete([]);
      await tester.pumpAndSettle();
      expect(
        find.byKey(ValueKey('market-installed-${entry.id}')),
        findsOneWidget,
      );
      expect(find.byKey(ValueKey('install-${entry.id}')), findsNothing);
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );

  testWidgets(
    'discovery has a skeleton and recoverable inline error with search retained',
    (tester) async {
      final source = PendingCatalog();
      await tester.pumpWidget(
        DesignHost(
          locale: locale,
          brightness: Brightness.light,
          home: MobileMarketPage(
            store: MobilePluginStore(root),
            source: source,
            mode: MobileMarketMode.discover,
          ),
        ),
      );
      await tester.pump();
      expect(find.byType(CircularProgressIndicator), findsNothing);
      expect(find.byKey(const ValueKey('mobile-market-search')), findsNothing);
      source.response.completeError(const SocketException('offline'));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.byKey(const ValueKey('mobile-market-search')),
        'keep query',
      );
      await tap(tester, 'market-retry');
      expect(source.calls, 2);
      expect(
        tester
            .widget<TextField>(
              find.byKey(const ValueKey('mobile-market-search')),
            )
            .controller!
            .text,
        'keep query',
      );
      expect(find.byKey(const ValueKey('market-retry')), findsOneWidget);
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );

  testWidgets(
    'detail follows installation progress, failure, and real store completion',
    (tester) async {
      final store = MobilePluginStore(root);
      late MobileMarketEntry entry;
      late List<int> artifact;
      await tester.runAsync(() async {
        const assets = '../../packages/openmuse_mobile_plugin_runtime/assets';
        final records =
            jsonDecode(await File('$assets/market.json').readAsString())
                as List;
        entry = records
            .map(
              (e) => MobileMarketEntry.fromJson(
                Map<String, Object?>.from(e as Map),
              ),
            )
            .singleWhere((e) => e.bundle == 'usage-budget.json');
        artifact = await File('$assets/${entry.bundle}').readAsBytes();
      });
      final operation = ValueNotifier<String?>(null),
          failure = ValueNotifier<String?>(null);
      addTearDown(operation.dispose);
      addTearDown(failure.dispose);
      var installs = 0, opens = 0;
      await tester.pumpWidget(
        DesignHost(
          locale: locale,
          brightness: Brightness.light,
          home: MarketPluginDetailPage(
            entry: entry,
            store: store,
            operation: operation,
            installationFailure: failure,
            onInstall: () {
              installs++;
              operation.value = entry.id;
            },
            onOpen: () => opens++,
            onPreview: () {},
            onToggle: () {},
            onRemove: () {},
            onShowOverview: () {},
          ),
        ),
      );
      await tester.pumpAndSettle();
      await tap(tester, 'detail-action-${entry.id}');
      expect(installs, 1);
      expect(
        tester
            .widget<FilledButton>(
              find.byKey(ValueKey('detail-action-${entry.id}')),
            )
            .onPressed,
        isNull,
      );
      operation.value = null;
      failure.value = 'Installation failed. Try again.';
      await tester.pumpAndSettle();
      expect(find.text('Installation failed. Try again.'), findsOneWidget);
      failure.value = null;
      await tester.runAsync(() => store.install(entry, artifact));
      await tester.pumpAndSettle();
      await tap(tester, 'detail-action-${entry.id}');
      expect(opens, 1);
      await tester.scrollUntilVisible(
        find.byKey(const ValueKey('market-show-overview')),
        300,
        scrollable: find.byType(Scrollable).first,
      );
      expect(
        find.byKey(const ValueKey('market-show-overview')),
        findsOneWidget,
      );
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
    },
  );

  const captureDirectory = String.fromEnvironment('APPS_CENTER_CAPTURE_DIR');
  if (captureDirectory.isNotEmpty) {
    testWidgets('capture real Flutter app-center surfaces for design review', (
      tester,
    ) async {
      const boundaryKey = ValueKey('apps-center-capture');
      await tester.runAsync(() async {
        final font = FontLoader('AppsEvidence');
        font.addFont(
          File(
            '/System/Library/Fonts/Supplemental/Arial.ttf',
          ).readAsBytes().then(ByteData.sublistView),
        );
        await font.load();
        final icons = FontLoader('MaterialIcons');
        icons.addFont(rootBundle.load('fonts/MaterialIcons-Regular.otf'));
        await icons.load();
      });
      Future<void> shot(String name) async {
        await tester.pumpAndSettle();
        final boundary = tester.renderObject<RenderRepaintBoundary>(
          find.byKey(boundaryKey),
        );
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 2);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          image.dispose();
          final file = File('$captureDirectory/$name.png');
          await file.parent.create(recursive: true);
          await file.writeAsBytes(bytes!.buffer.asUint8List());
        });
      }

      await show(tester, captureKey: boundaryKey);
      await tap(tester, 'wb-tab-Apps');
      await shot('widget-overview-en-light');
      await tap(tester, 'apps-edit');
      await shot('widget-edit-en-light');
      await tap(tester, 'apps-edit-cancel');
      await tap(tester, 'apps-section-discover');
      await shot('widget-discover-empty-en-light');
      await tap(tester, 'wb-tab-Account');
      await shot('widget-account-en-light');
      await tap(tester, 'my-plugins');
      await shot('widget-my-plugins-en-light');
      await tester.pageBack();
      await tester.pumpAndSettle();
      controller.selectTab(WbTab.apps);
      await show(tester, captureKey: boundaryKey, brightness: Brightness.dark);
      await tap(tester, 'wb-tab-Apps');
      await shot('widget-overview-en-dark');
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
    });
  }

  for (final width in [320.0, 390.0, 430.0]) {
    for (final language in ['en', 'zh-Hans']) {
      testWidgets(
        'apps header and edit controls fit $width $language at large text',
        (tester) async {
          locale.selectTag(language);
          controller.selectTab(WbTab.apps);
          await show(
            tester,
            width: width,
            scale: 1.5,
            brightness: Brightness.dark,
          );
          expect(tester.takeException(), isNull);
          await tap(tester, 'apps-edit');
          expect(tester.takeException(), isNull);
          await tap(tester, 'apps-edit-cancel');
          await tap(tester, 'apps-section-discover');
          expect(tester.takeException(), isNull);
          await tester.pumpWidget(const SizedBox.shrink());
        },
      );
    }
  }
  test(
    'legacy named routes migrate explicitly and retain the four-value enum',
    () async {
      expect(WbTab.values.length, 4);
      await controller.restoreTabName('workbench');
      expect(controller.tab, WbTab.apps);
      expect(controller.appsSection, AppsSection.overview);
      await controller.restoreTabName('discover');
      expect(controller.tab, WbTab.apps);
      expect(controller.appsSection, AppsSection.discover);
    },
  );
  test(
    'built-in enablement survives reconstruction and is changed through registry lifecycle',
    () async {
      await market.builtInPreferences.setEnabled(registry, schedules, false);
      expect(
        registry.stateOf(schedules.descriptor.id),
        OpenMusePluginState.installed,
      );
      final restored = MobileBuiltInPreferences(
        File('${root.path}/built-in-enablement.json'),
      );
      expect(await restored.isEnabled(schedules.descriptor.id), isFalse);
      await restored.setEnabled(registry, schedules, true);
      expect(
        registry.stateOf(schedules.descriptor.id),
        OpenMusePluginState.active,
      );
      expect(
        await MobileBuiltInPreferences(
          File('${root.path}/built-in-enablement.json'),
        ).isEnabled(schedules.descriptor.id),
        isTrue,
      );
    },
  );
}
