import 'dart:io';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_resource_notes/openmuse_resource_notes.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'support/design_fixture.dart';

void main() {
  late Directory root;
  late NativeResourceCenter center;
  late OpenMusePluginRegistry registry;
  late ResourceRecord record;
  setUp(() async {
    PackageInfo.setMockInitialValues(
      appName: 'OpenMuse',
      packageName: 'com.openmuseai.mobile',
      version: '1.0.0',
      buildNumber: '14',
      buildSignature: '',
    );
    root = await Directory.systemTemp.createTemp('resource-keyboard-');
    registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (a, b) async => null),
    );
    registry.install(OpenMuseResourceNotesPlugin());
    await registry.activate('com.openmuse.resource-notes');
    center = await NativeResourceCenter.open(
      scope: 'keyboard-test',
      deviceRef: 'phone-1',
      deviceLabel: 'This phone',
      registry: registry,
      storageRoot: root,
      historyStore: MemoryResourceStateStore(),
    );
    record = await center.documents.create(
      parentRef: 'root',
      name: 'Project brief with a longer title.md',
      mediaType: 'text/markdown',
      bytes: [65],
      idempotencyKey: 'fixture',
      access: center.hub.access('setup'),
    );
  });
  tearDown(() async {
    await center.close();
    registry.dispose();
    await root.delete(recursive: true);
  });
  testWidgets(
    'resource paste appears only in the requested text-selection menu',
    (tester) async {
      var clipboardReads = 0;
      const channel = MethodChannel('io.openmuse/resource_clipboard');
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(channel, (call) async {
            if (call.method == 'read') {
              clipboardReads++;
              return jsonEncode(record.reference(pin: true).toJson());
            }
            return null;
          });
      addTearDown(
        () => TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
            .setMockMethodCallHandler(channel, null),
      );
      final buddy = WorkBuddyController()..selectTab(WbTab.tasks);
      final locale = OpenMuseLocaleController(initialTag: 'en');
      await tester.pumpWidget(
        DesignHost(
          locale: locale,
          brightness: Brightness.light,
          home: WorkBuddyShell(
            controller: buddy,
            pluginRegistry: registry,
            resources: center,
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(find.text('Paste resource'), findsNothing);
      expect(clipboardReads, 0);
      await tester.tap(find.byKey(const ValueKey('wb-plus')));
      await tester.pumpAndSettle();
      expect(find.text('Paste resource'), findsNothing);
      Navigator.of(
        tester.element(find.byKey(const ValueKey('wb-composer'))),
      ).pop();
      await tester.pumpAndSettle();
      await tester.longPress(find.byKey(const ValueKey('wb-composer')));
      await tester.pumpAndSettle();
      expect(find.text('Paste resource'), findsOneWidget);
      expect(
        clipboardReads,
        0,
        reason: 'Opening the menu must not read the resource clipboard',
      );
      await tester.runAsync(() async {
        await tester.tap(find.text('Paste resource'));
        await Future<void>.delayed(const Duration(milliseconds: 100));
      });
      await tester.pumpAndSettle();
      expect(clipboardReads, 1);
      expect(center.draft.items.single.record.identity, record.identity);
      expect(find.text('Paste resource'), findsNothing);
      await tester.pumpWidget(const SizedBox.shrink());
      buddy.dispose();
      locale.dispose();
    },
  );
  testWidgets(
    'Tasks composer and tabs remain bottom aligned with resource drafts and keyboard transitions',
    (tester) async {
      tester.view.physicalSize = const Size(360, 791);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      addTearDown(tester.view.resetPadding);
      addTearDown(tester.view.resetViewPadding);
      addTearDown(tester.view.resetViewInsets);
      final buddy = WorkBuddyController();
      final locale = OpenMuseLocaleController(initialTag: 'en');
      await tester.pumpWidget(
        DesignHost(
          locale: locale,
          brightness: Brightness.light,
          home: WorkBuddyShell(
            controller: buddy,
            pluginRegistry: registry,
            resources: center,
          ),
        ),
      );
      for (final attached in [false, true]) {
        if (attached) center.draft.attach(record);
        for (final keyboard in [0.0, 300.0, 0.0]) {
          final safeBottom = keyboard == 0 ? 24.0 : 0.0;
          tester.view.padding = FakeViewPadding(bottom: safeBottom);
          tester.view.viewPadding = const FakeViewPadding(bottom: 24);
          tester.view.viewInsets = FakeViewPadding(bottom: keyboard);
          buddy.selectTab(WbTab.home);
          await tester.pumpAndSettle();
          final homeBottom = tester
              .getBottomLeft(find.byKey(const ValueKey('wb-tab-Home')))
              .dy;
          await tester.tap(find.byKey(const ValueKey('wb-tab-Tasks')));
          await tester.pumpAndSettle();
          final tasksBottom = tester
              .getBottomLeft(find.byKey(const ValueKey('wb-tab-Tasks')))
              .dy;
          expect(
            tasksBottom,
            closeTo(homeBottom, 1),
            reason:
                'Switching to Tasks must not leave unused flex space below the tabs',
          );
          expect(
            791 - keyboard - safeBottom - tasksBottom,
            inInclusiveRange(0, 12),
          );
          final composerBottom = tester
              .getBottomLeft(
                find
                    .ancestor(
                      of: find.byKey(const ValueKey('wb-composer')),
                      matching: find.byType(DecoratedBox),
                    )
                    .first,
              )
              .dy;
          final tabsTop = tester
              .getTopLeft(find.byKey(const ValueKey('wb-tab-Tasks')))
              .dy;
          expect(tabsTop - composerBottom, inInclusiveRange(0, 32));
          expect(tester.takeException(), isNull);
        }
      }
      await tester.pumpWidget(const SizedBox.shrink());
      buddy.dispose();
      locale.dispose();
    },
  );
  testWidgets(
    'reference cards and retention controls fit above keyboard at large text sizes',
    (tester) async {
      tester.view.physicalSize = const Size(360, 800);
      tester.view.devicePixelRatio = 1;
      tester.view.viewInsets = const FakeViewPadding(bottom: 300);
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      addTearDown(tester.view.resetViewInsets);
      final buddy = WorkBuddyController();
      final locale = OpenMuseLocaleController(initialTag: 'en');
      center.draft.attach(record);
      for (final brightness in [Brightness.light, Brightness.dark]) {
        await tester.pumpWidget(
          DesignHost(
            locale: locale,
            brightness: brightness,
            textScale: 1.5,
            home: WorkBuddyShell(
              controller: buddy,
              pluginRegistry: registry,
              resources: center,
            ),
          ),
        );
        await tester.pumpAndSettle();
        expect(
          find.byKey(const ValueKey('resource-retain-toggle')),
          findsOneWidget,
        );
        expect(tester.takeException(), isNull);
      }
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpAndSettle();
      await tester.runAsync(center.close);
      buddy.dispose();
      locale.dispose();
    },
  );
  testWidgets(
    'Home storage failure exposes retry and leaves Tasks navigation usable',
    (tester) async {
      final buddy = WorkBuddyController()..selectTab(WbTab.home);
      final locale = OpenMuseLocaleController(initialTag: 'en');
      var retries = 0;
      await tester.pumpWidget(
        DesignHost(
          locale: locale,
          brightness: Brightness.light,
          home: WorkBuddyShell(
            controller: buddy,
            resourceFailure: const ResourceFailure('UNSAFE_STORAGE'),
            onRetryResources: () => retries++,
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(
        find.byKey(const ValueKey('resource-initialization-error')),
        findsOneWidget,
      );
      await tester.tap(find.text('Try again'));
      await tester.pumpAndSettle();
      expect(retries, 1);
      await tester.tap(find.byKey(const ValueKey('wb-tab-Tasks')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('wb-composer')), findsOneWidget);
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox.shrink());
      buddy.dispose();
      locale.dispose();
    },
  );
}
