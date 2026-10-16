import 'dart:io';

import 'package:appflowy_editor_native_plugin/appflowy_editor_native_plugin.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_host/src/host/mcp_file_plugin.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('production Registry installs a Markdown editor in macOS Host', (
    tester,
  ) async {
    expect(Platform.isMacOS, isTrue);
    final directory = await Directory.systemTemp.createTemp(
      'openmuse-editor-production-',
    );
    addTearDown(() => directory.delete(recursive: true));
    final market = McpRegistryMarket(
      origin: Uri.parse(
        const String.fromEnvironment(
          'OPENMUSE_MCP_REGISTRY_URL',
          defaultValue: 'https://plugins.openmuseai.com',
        ),
      ),
      installRoot: Directory('${directory.path}/installed'),
    );
    final entry = (await market.list()).singleWhere(
      (entry) =>
          entry.name == 'io.openmuse/appflowy-editor' &&
          entry.version == '0.1.5',
    );
    final receipt = await market.install(entry);
    expect(receipt['installedVersion'], '0.1.5');
    final runtime = McpBundleRuntime(
      bundleDirectory: market.directoryFor(entry),
      nodeExecutable: 'node',
    );
    final markdownTool = (await runtime.tools()).singleWhere(
      (tool) => tool['name'] == 'open_markdown',
    );
    final content =
        (markdownTool['inputSchema'] as Map)['properties']['content'] as Map;
    expect(content['contentMediaType'], 'text/markdown');
    final file = File('${directory.path}/desktop-e2e.md')
      ..writeAsStringSync('# macOS production acceptance\n');
    final plugin = OpenMuseMcpFilePlugin(
      entry: entry,
      runtime: runtime,
      toolName: 'open_markdown',
      mediaType: 'text/markdown',
      extension: 'md',
      nativeFlutterEditor: true,
    );
    final resource = OpenMuseResource(
      uri: file.uri,
      displayName: 'desktop-e2e.md',
      mediaType: 'text/markdown',
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Builder(
            builder: (context) => plugin.buildEditor(context, resource),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle(
      const Duration(milliseconds: 200),
      EnginePhase.sendSemanticsUpdate,
      const Duration(seconds: 45),
    );
    expect(find.byType(AppFlowyMarkdownEditor), findsOneWidget);
    expect(find.byType(AppKitView), findsNothing);
    expect(find.text('macOS production acceptance'), findsWidgets);
  });
}
