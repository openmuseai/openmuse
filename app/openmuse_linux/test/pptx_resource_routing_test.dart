import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_linux/src/host/mcp_file_plugin.dart';
import 'package:openmuse_linux/src/host/mcp_market.dart';
import 'package:openmuse_linux/src/host/resource_inspector.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  final sample = File(
    '../openmuse_mobile/test_assets/genoffice_standard_business.pptx',
  ).absolute;
  final entry = McpMarketEntry(
    name: 'io.openmuse/genoffice-pptx',
    title: 'GenOffice PPTX',
    description: '',
    version: '0.4.0',
    packageUri: Uri.parse('https://example.com/genoffice.mcpb'),
    sha256: '0' * 64,
  );

  test('inspected Workspace PPTX routes to installed standard MCP App', () async {
    final plugin = OpenMuseMcpFilePlugin(
      entry: entry,
      runtime: McpBundleRuntime(
        bundleDirectory: Directory.systemTemp,
        nodeExecutable: 'node',
      ),
      toolName: 'inspect_pptx',
      mediaType:
          'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      extension: 'pptx',
    );
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    )..install(plugin);
    addTearDown(registry.dispose);
    final resource = await inspectLocalResource(
      OpenMuseResource(
        uri: sample.uri,
        displayName: 'genoffice_standard_business.pptx',
      ),
    );
    expect(resource.mediaType, plugin.mediaType);
    expect(registry.editorFor(resource), same(plugin));
    expect(
      registry.editorCandidates(resource).single.editor.id,
      'io.openmuse/genoffice-pptx.file',
    );
  });

  test('a renamed non-ZIP file cannot claim the PPTX media type', () async {
    final root = await Directory.systemTemp.createTemp('pptx-routing-');
    addTearDown(() => root.delete(recursive: true));
    final file = File('${root.path}/fake.pptx')
      ..writeAsStringSync('not a deck');
    final resource = await inspectLocalResource(
      OpenMuseResource(uri: file.uri, displayName: 'fake.pptx'),
    );
    expect(
      resource.mediaType,
      isNot(
        'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      ),
    );
  });
}
