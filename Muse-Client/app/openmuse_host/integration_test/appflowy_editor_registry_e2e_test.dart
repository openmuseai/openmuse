import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets(
    'macOS installs and opens AppFlowy Editor from the reviewed Registry',
    (tester) async {
      const registry = String.fromEnvironment('EDITOR_E2E_REGISTRY');
      const node = String.fromEnvironment('EDITOR_E2E_NODE');
      if (registry.isEmpty || node.isEmpty) {
        fail('EDITOR_E2E_REGISTRY and EDITOR_E2E_NODE are required');
      }
      final directory = await Directory.systemTemp.createTemp(
        'editor-mcpb-e2e-',
      );
      addTearDown(() => directory.delete(recursive: true));
      final market = McpRegistryMarket(
        origin: Uri.parse(registry),
        installRoot: directory,
        nodeExecutable: node,
      );
      final entries = await market.list();
      final editor = entries.singleWhere(
        (item) => item.name == 'io.openmuse/appflowy-editor',
      );
      final installed = await market.install(editor);
      expect(installed['action'], 'installed');
      expect(await market.installedEntries(), hasLength(1));
      final runtime = McpBundleRuntime(
        bundleDirectory: market.directoryFor(editor),
        nodeExecutable: node,
      );
      final tools = await runtime.tools();
      expect(
        tools.any(
          (item) =>
              item['name'] == 'open_editor' && item['uiResourceUri'] is String,
        ),
        isTrue,
      );
      final uri = await runtime.openUi();
      expect(uri?.path, '/editor/');
      final http = HttpClient();
      try {
        final response = await (await http.getUrl(uri!)).close();
        expect(response.statusCode, 200);
        expect(response.headers.contentType?.mimeType, 'text/html');
      } finally {
        http.close(force: true);
      }
    },
  );
}
