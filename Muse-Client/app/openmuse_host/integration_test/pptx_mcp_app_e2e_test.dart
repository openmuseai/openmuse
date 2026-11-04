import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';
import 'package:openmuse_host/src/host/mcp_file_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized().framePolicy =
      LiveTestWidgetsFlutterBindingFramePolicy.fullyLive;
  testWidgets('reviewed MCP Apps file capability opens in the macOS Host', (
    tester,
  ) async {
    final root = await Directory.systemTemp.createTemp('mcp-pptx-e2e-');
    final market = McpRegistryMarket(
      origin: Uri.parse('http://127.0.0.1:18803'),
      installRoot: Directory('${root.path}/plugins'),
    );
    final entry = (await market.list()).singleWhere(
      (e) => e.name == 'io.openmuse/genoffice-pptx',
    );
    await market.install(entry);
    final runtime = McpBundleRuntime(
      bundleDirectory: market.directoryFor(entry),
      nodeExecutable: const String.fromEnvironment(
        'TEST_NODE',
        defaultValue: 'node',
      ),
    );
    final plugin = await discoverMcpFilePlugin(entry: entry, runtime: runtime);
    expect(plugin, isNotNull);
    expect(await runtime.uiDocument(plugin!.toolName), isNotNull);
    final file = await File(
      '/Users/mac/src/ioffice/samples/ppt-render/flutter/assets/Sample_ppt.pptx',
    ).copy('${root.path}/Sample_ppt.pptx');
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          appBar: AppBar(title: Text('PPTX MCP Apps E2E · ${entry.version}')),
          body: Builder(
            builder: (context) => plugin.buildEditor(
              context,
              OpenMuseResource(
                uri: file.uri,
                displayName: 'Sample_ppt.pptx',
                mediaType: plugin.mediaType,
              ),
            ),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    await Future<void>.delayed(const Duration(seconds: 15));
    await tester.pump();
    expect(find.byType(AppKitView), findsOneWidget);
    await tester.pumpWidget(const SizedBox());
    debugPrint('PPTX_MACOS_VIEW_MOUNTED ${file.path}');
    await Future<void>.delayed(const Duration(seconds: 70));
    await tester.pumpWidget(const SizedBox());
    await tester.pumpAndSettle();
  });
}
