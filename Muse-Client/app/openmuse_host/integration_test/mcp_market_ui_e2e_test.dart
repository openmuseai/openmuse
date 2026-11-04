import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_host/src/host/layout/layout.dart';
import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';
import 'package:openmuse_host/src/host/workbench_shell.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('online market installs reviewed MCPB and invokes PPTX tool', (
    tester,
  ) async {
    final registryUrl = Platform.environment['OPENMUSE_MCP_REGISTRY_URL'];
    if (registryUrl == null || registryUrl.isEmpty) {
      fail('Set OPENMUSE_MCP_REGISTRY_URL to a reviewed MCP Registry');
    }
    final root = Directory.systemTemp.createTempSync('mcp-market-ui-');
    addTearDown(() => root.deleteSync(recursive: true));
    final market = McpRegistryMarket(
      origin: Uri.parse(registryUrl),
      installRoot: Directory(p.join(root.path, 'mcpb')),
      nodeExecutable: 'node',
    );
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(
        executeHostCommand: (command, args) async {
          if (command == 'plugin.market.list') {
            return [
              for (final entry in await Zone.root.run(market.list))
                entry.toMarketCard(),
            ];
          }
          if (args is! Map || args['id'] is! String) {
            throw const FormatException('Missing id');
          }
          final entry = (await Zone.root.run(
            market.list,
          )).singleWhere((item) => item.name == args['id']);
          return switch (command) {
            'plugin.market.plan' => Zone.root.run(() => market.plan(entry)),
            'plugin.market.install' => Zone.root.run(
              () => market.install(entry),
            ),
            _ => throw UnsupportedError(command),
          };
        },
      ),
    );
    final marketPlugin = createOpenMuseBuiltInPlugins().firstWhere(
      (plugin) => plugin.descriptor.id == 'com.openmuse.plugin-market',
    );
    registry.install(marketPlugin);
    await registry.activate(marketPlugin.descriptor.id);
    final workspaceRoot = Directory(p.join(root.path, 'workspace'))
      ..createSync(recursive: true);
    final workspace = LocalWorkspaceController(
      rootPath: workspaceRoot.path,
      initialResources: const [],
    );
    final settings = OpenMuseLocalSettings(
      file: File(p.join(root.path, 'settings.json')),
    );
    await settings.load();
    await tester.pumpWidget(
      MaterialApp(
        home: OpenMuseWorkbench(
          registry: registry,
          workspace: workspace,
          settings: settings,
          layoutController: WorkbenchLayoutController(
            createDefaultWorkbenchLayout(),
          ),
        ),
      ),
    );
    await tester.tap(find.text('插件').first);
    for (var i = 0; i < 30; i++) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 200)),
      );
      await tester.pump();
      if (find
          .byKey(
            const ValueKey('plugin-market-card-io.openmuse/genoffice-pptx'),
          )
          .evaluate()
          .isNotEmpty) {
        break;
      }
    }
    expect(
      find.byKey(
        const ValueKey('plugin-market-card-io.openmuse/genoffice-pptx'),
      ),
      findsOneWidget,
    );
    await tester.ensureVisible(find.text('安装').first);
    await tester.tap(find.text('安装').first);
    for (var i = 0; i < 60; i++) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 200)),
      );
      await tester.pump();
      if (find.textContaining('已安装 0.2.0').evaluate().isNotEmpty) break;
    }
    expect(find.textContaining('已安装 0.2.0'), findsOneWidget);
    final entry = (await Zone.root.run(market.list)).single;
    final runtime = McpBundleRuntime(
      bundleDirectory: market.directoryFor(entry),
      nodeExecutable: 'node',
    );
    final result = await Zone.root.run(
      () => runtime.callWithFile(
        toolName: 'inspect_pptx',
        filePath:
            '../openmuse_mobile/test_assets/genoffice_standard_business.pptx',
      ),
    );
    expect(result['slides'], greaterThan(0));
  });
}
