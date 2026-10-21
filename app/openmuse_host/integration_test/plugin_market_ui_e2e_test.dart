import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_host/src/host/layout/layout.dart';
import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_host/src/host/plugin_market.dart';
import 'package:openmuse_host/src/host/workbench_shell.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('market page installs Archify and DSH loads its skill', (
    tester,
  ) async {
    const cli = String.fromEnvironment('OPENMUSE_MARKET_E2E_DSH_CLI');
    const node = String.fromEnvironment(
      'OPENMUSE_MARKET_E2E_NODE',
      defaultValue: 'node',
    );
    if (cli.isEmpty || !File(cli).existsSync()) {
      fail('Set OPENMUSE_MARKET_E2E_DSH_CLI to the DSH closure bin.js');
    }
    final root = Directory.systemTemp.createTempSync('openmuse-market-ui-');
    addTearDown(() => root.deleteSync(recursive: true));
    final dshHome = Directory(p.join(root.path, 'dsh'));
    final installer = DshMarketInstaller(
      dshHome: dshHome,
      profile: 'market-ui-e2e',
      cliPath: cli,
      nodeExecutable: node,
      baseEnvironment: {...Platform.environment, 'PATH': '/usr/bin:/bin'},
    );
    await Zone.root.run(() => installer.plan(builtInMarketPlugins.single));
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(
        executeHostCommand: (command, args) async {
          if (command == 'plugin.market.list') {
            return [for (final plugin in builtInMarketPlugins) plugin.toJson()];
          }
          if (command == 'plugin.market.plan') {
            return Zone.root.run(
              () => installer.plan(builtInMarketPlugins.single),
            );
          }
          if (command == 'plugin.market.install') {
            return Zone.root.run(
              () => installer.install(builtInMarketPlugins.single),
            );
          }
          if (command == 'plugin.market.enable' ||
              command == 'plugin.market.disable') {
            return Zone.root.run(
              () => installer.setEnabled(
                builtInMarketPlugins.single,
                command == 'plugin.market.enable',
              ),
            );
          }
          throw UnsupportedError(command);
        },
      ),
    );
    final market = createOpenMuseBuiltInPlugins().firstWhere(
      (plugin) => plugin.descriptor.id == 'com.openmuse.plugin-market',
    );
    registry.install(market);
    await registry.activate(market.descriptor.id);
    final workspaceRoot = Directory(p.join(root.path, 'workspace'))
      ..createSync(recursive: true);
    final workspace = LocalWorkspaceController(
      rootPath: workspaceRoot.path,
      initialResources: const [],
    );
    final settings = OpenMuseLocalSettings(
      file: File(p.join(root.path, 'settings-v1.json')),
    );
    await settings.load();
    final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
    const captureKey = ValueKey('desktop-market-capture');
    await tester.pumpWidget(
      RepaintBoundary(
        key: captureKey,
        child: MaterialApp(
          home: OpenMuseWorkbench(
            registry: registry,
            workspace: workspace,
            settings: settings,
            layoutController: layout,
          ),
        ),
      ),
    );

    await tester.tap(find.text('插件').first);
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 200));
    expect(find.byKey(const ValueKey('plugin-market-catalog')), findsOneWidget);
    expect(
      layout.snapshot.bindings.values.where(
        (binding) => binding.surfaceRef == 'plugin.panel:plugin.market',
      ),
      isEmpty,
    );
    expect(find.byType(Dialog), findsNothing);
    expect(find.byKey(const ValueKey('desktop-market-page')), findsOneWidget);
    expect(find.byKey(const ValueKey('plugin-market-tab-0')), findsOneWidget);
    expect(find.byKey(const ValueKey('plugin-market-search')), findsOneWidget);
    expect(find.textContaining('Archify 1.0.0'), findsOneWidget);
    const capturePath = String.fromEnvironment(
      'OPENMUSE_MARKET_E2E_CAPTURE_PATH',
    );
    if (capturePath.isNotEmpty) {
      await tester.pumpAndSettle();
      final boundary = tester.renderObject<RenderRepaintBoundary>(
        find.byKey(captureKey),
      );
      final image = await boundary.toImage(pixelRatio: 1);
      final data = await image.toByteData(format: ui.ImageByteFormat.png);
      expect(data, isNotNull);
      await File(
        capturePath,
      ).writeAsBytes(data!.buffer.asUint8List(), flush: true);
    }

    await tester.tap(
      find.byKey(const ValueKey('plugin-market-sidebar-search')),
    );
    await tester.pump();
    expect(
      tester
          .widget<TextField>(find.byKey(const ValueKey('plugin-market-search')))
          .focusNode
          ?.hasFocus,
      isTrue,
    );
    await tester.enterText(
      find.byKey(const ValueKey('plugin-market-search')),
      '未收录的插件',
    );
    await tester.pump();
    expect(find.text('没有匹配的插件'), findsOneWidget);
    await tester.tap(find.text('清除筛选'));
    await tester.pump();
    await tester.tap(find.byKey(const ValueKey('plugin-market-nav-办公')));
    await tester.pump();
    expect(find.text('没有匹配的插件'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('plugin-market-tab-0')));
    await tester.pump();
    expect(find.text('安装插件'), findsOneWidget);

    await tester.pumpAndSettle();
    await tester.tap(find.text('安装插件'));
    for (var i = 0; i < 120; i++) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(seconds: 1)),
      );
      await tester.pump();
      if (find.textContaining('已安装 1.0.0').evaluate().isNotEmpty) break;
    }
    expect(find.textContaining('已安装 1.0.0'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('plugin-market-tab-1')));
    for (var i = 0; i < 20; i++) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 100)),
      );
      await tester.pump();
      if (find
          .byKey(const ValueKey('plugin-market-card-archify'))
          .evaluate()
          .isNotEmpty) {
        break;
      }
    }
    expect(
      find.byKey(const ValueKey('plugin-market-installed')),
      findsOneWidget,
    );
    expect(
      find.byKey(const ValueKey('plugin-market-card-archify')),
      findsOneWidget,
    );
    if (capturePath.isNotEmpty) {
      await tester.pumpAndSettle();
      final boundary = tester.renderObject<RenderRepaintBoundary>(
        find.byKey(captureKey),
      );
      final image = await boundary.toImage(pixelRatio: 1);
      final data = await image.toByteData(format: ui.ImageByteFormat.png);
      expect(data, isNotNull);
      await File(
        capturePath.replaceFirst('.png', '-installed.png'),
      ).writeAsBytes(data!.buffer.asUint8List(), flush: true);
    }
    await tester.ensureVisible(
      find.byKey(const ValueKey('plugin-market-toggle-archify')),
    );
    await tester.tap(
      find.byKey(const ValueKey('plugin-market-toggle-archify')),
    );
    await tester.pumpAndSettle();
    expect(find.textContaining('已禁用'), findsOneWidget);
    await tester.tap(
      find.byKey(const ValueKey('plugin-market-toggle-archify')),
    );
    await tester.pumpAndSettle();
    expect(find.textContaining('已启用'), findsOneWidget);

    final profile = Directory(
      p.join(dshHome.path, 'profiles', 'market-ui-e2e'),
    );
    final manifest =
        jsonDecode(
              File(p.join(profile.path, 'package.json')).readAsStringSync(),
            )
            as Map;
    expect((manifest['dependencies'] as Map)['@tt-a1i/archify-dsh'], '1.0.0');
    final package = File(
      p.join(
        profile.path,
        'node_modules',
        '@tt-a1i',
        'archify-dsh',
        'skills',
        'archify',
        'SKILL.md',
      ),
    );
    expect(package.existsSync(), isTrue);
    final composed = await tester.runAsync(
      () => Process.run(
        node,
        [cli, 'market-ui-e2e', '--dump-config'],
        environment: {...Platform.environment, 'DSH_HOME': dshHome.path},
      ),
    );
    expect(composed, isNotNull);
    expect(composed!.exitCode, 0, reason: '${composed.stderr}');
    expect('${composed.stdout}', contains('archify-skill-filesystem'));
  });
}
