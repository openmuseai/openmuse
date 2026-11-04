import 'dart:convert';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'dart:io';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/mobile_build_locale.dart';
import 'package:openmuse_mobile/mobile_plugins/mobile_my_page.dart';
import 'package:openmuse_mobile/mobile_plugins/scheduled_tasks_plugin.dart';
import 'package:openmuse_mobile/mobile_plugins/workspace_files_plugin.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';

// Exercise installed profiles explicitly; these tests do not depend on build flags.
final _config = DeploymentBundle.parse(
  File(
    '../../packages/openmuse_runtime_config/assets/deployments/global-us.json',
  ).readAsStringSync(),
).snapshot;
OpenMuseLocaleController _locale() => OpenMuseLocaleController(
  initialTag: _config.defaultLocale,
  defaultTag: _config.defaultLocale,
  enabledLocales: _config.enabledLocales,
);
void main() {
  testWidgets('US host launches in English and exposes English navigation', (
    tester,
  ) async {
    final locale = _locale();
    addTearDown(locale.dispose);
    final controller = WorkBuddyController(strings: locale.strings);
    addTearDown(controller.dispose);
    await tester.pumpWidget(
      _app(WorkBuddyShell(controller: controller), locale),
    );
    expect(find.text('Create with OpenMuse'), findsOneWidget);
    expect(find.text('Apps'), findsOneWidget);
    expect(find.text('Discover'), findsNothing);
    expect(find.text('Account'), findsOneWidget);
    expect(find.text('OpenMuse，与你一起创造'), findsNothing);
  });

  testWidgets('US account and legal page omit unresolved China filing', (
    tester,
  ) async {
    final locale = _locale();
    addTearDown(locale.dispose);
    await tester.pumpWidget(
      _app(
        MobileMyPage(
          themeMode: ThemeMode.system,
          onThemeMode: (_) {},
          workspaces: const [],
          deviceName: 'Review iPhone',
        ),
        locale,
      ),
    );
    expect(find.text('Not signed in'), findsOneWidget);
    expect(find.text('English'), findsOneWidget);
    expect(find.byKey(const ValueKey('my-app-filing-number')), findsNothing);
    await tester.scrollUntilVisible(
      find.byKey(const ValueKey('my-about')),
      200,
    );
    await tester.tap(find.byKey(const ValueKey('my-about')));
    await tester.pumpAndSettle();
    expect(find.text('Privacy Policy'), findsOneWidget);
    expect(find.text('Terms of Service'), findsOneWidget);
    expect(find.text('Account deletion information'), findsOneWidget);
    expect(find.byKey(const ValueKey('app-filing-number')), findsNothing);
  });

  testWidgets(
    'narrow composer retains its draft when switching languages with keyboard and large text',
    (tester) async {
      tester.view.physicalSize = const Size(320, 760);
      tester.view.devicePixelRatio = 1;
      tester.view.viewInsets = const FakeViewPadding(bottom: 280);
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      addTearDown(tester.view.resetViewInsets);
      final locale = _locale();
      final controller = WorkBuddyController(strings: locale.strings);
      addTearDown(() async {
        await tester.pumpWidget(const SizedBox.shrink());
        controller.dispose();
        locale.dispose();
      });
      final shell = WorkBuddyShell(controller: controller);
      await tester.pumpWidget(
        ListenableBuilder(
          listenable: locale,
          builder: (context, _) => _app(
            Builder(
              builder: (context) => MediaQuery(
                data: MediaQuery.of(
                  context,
                ).copyWith(textScaler: const TextScaler.linear(1.5)),
                child: shell,
              ),
            ),
            locale,
          ),
        ),
      );
      final composer = find.byKey(const ValueKey('wb-composer'));
      await tester.enterText(composer, 'Keep my draft 原始内容');
      await tester.pump();
      expect(tester.takeException(), isNull);
      final editor = tester.widget<TextField>(composer).controller!;
      final originalHint = tester
          .widget<TextField>(composer)
          .decoration!
          .hintText;
      locale.selectTag('zh-Hans');
      await tester.pumpAndSettle();
      expect(tester.widget<TextField>(composer).controller, same(editor));
      expect(
        tester.widget<TextField>(composer).decoration!.hintText,
        isNot(originalHint),
      );
      expect(editor.text, 'Keep my draft 原始内容');
      expect(tester.takeException(), isNull);
      locale.selectTag('en');
      await tester.pumpAndSettle();
      expect(
        tester.widget<TextField>(composer).decoration!.hintText,
        originalHint,
      );
      expect(editor.text, 'Keep my draft 原始内容');
      expect(tester.takeException(), isNull);
    },
  );

  testWidgets(
    'runtime market switch changes a mounted app without losing the composer draft',
    (tester) async {
      final locale = _locale(), controller = WorkBuddyController();
      final settings = ValueNotifier<RuntimeConfig>(_config);
      addTearDown(() async {
        await tester.pumpWidget(const SizedBox.shrink());
        settings.dispose();
        controller.dispose();
        locale.dispose();
      });
      final shell = WorkBuddyShell(controller: controller);
      await tester.pumpWidget(
        ValueListenableBuilder<RuntimeConfig>(
          valueListenable: settings,
          builder: (context, config, _) => _app(shell, locale, config: config),
        ),
      );
      final composer = find.byKey(const ValueKey('wb-composer'));
      await tester.enterText(composer, 'Retain this draft');
      final editor = tester.widget<TextField>(composer).controller;
      final snapshot = Map<String, dynamic>.from(
        jsonDecode(jsonEncode(_config.json)) as Map,
      );
      (snapshot['compatibility']['requiredCapabilities'] as List).add(
        'settings.v1',
      );
      snapshot['settings'] = {
        'features': {'plugins.market.enabled': false},
        'parameters': {'runtime.refreshIntervalSeconds': 60},
      };
      settings.value = RuntimeConfig.fromJson(snapshot);
      await tester.pumpAndSettle();
      expect(tester.widget<TextField>(composer).controller, same(editor));
      controller.selectAppsSection(AppsSection.discover);
      await tester.pumpAndSettle();
      expect(
        find.text(locale.strings.commonFeatureUnavailable()),
        findsOneWidget,
      );
      settings.value = _config;
      await tester.pumpAndSettle();
      expect(
        find.text(locale.strings.commonFeatureUnavailable()),
        findsNothing,
      );
      controller.selectTab(WbTab.tasks);
      await tester.pumpAndSettle();
      expect(editor!.text, 'Retain this draft');
    },
  );

  test('US built-in file and scheduled-task pages have English controls', () {
    final locale = _locale();
    addTearDown(locale.dispose);
    final files = MobileWorkspaceFilesPlugin(strings: locale.strings);
    final tasks = MobileScheduledTasksPlugin(strings: locale.strings);
    final fileHtml = files.buildMobileDetailDocument(Brightness.light);
    final taskHtml = tasks.buildMobileDetailDocument(Brightness.light);
    expect(fileHtml, contains('lang="en"'));
    expect(fileHtml, contains('Search filenames'));
    expect(fileHtml, isNot(contains('搜索文件名')));
    expect(taskHtml, contains('Schedule task'));
    expect(taskHtml, isNot(contains('创建定时任务')));
  });
}

Widget _app(
  Widget home,
  OpenMuseLocaleController locale, {
  RuntimeConfig? config,
}) => RuntimeConfigScope(
  config: config ?? _config,
  child: OpenMuseLocaleScope(
    controller: locale,
    child: MaterialApp(
      locale: locale.locale,
      supportedLocales: locale.supportedLocales,
      localizationsDelegates: [
        OpenMuseStringsDelegate(locale.catalog),
        ...GlobalMaterialLocalizations.delegates,
      ],
      home: home,
    ),
  ),
);
