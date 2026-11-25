import 'dart:io';
import 'package:flutter/material.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';
import 'package:openmuse_host/src/host/mcp_file_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final root = await Directory.systemTemp.createTemp('mcp-app-smoke-');
  final market = McpRegistryMarket(
    origin: Uri.parse(
      const String.fromEnvironment(
        'TEST_REGISTRY',
        defaultValue: 'http://127.0.0.1:18803',
      ),
    ),
    installRoot: Directory('${root.path}/plugins'),
  );
  final entry = (await market.list()).singleWhere(
    (e) =>
        e.name ==
        const String.fromEnvironment(
          'TEST_PLUGIN',
          defaultValue: 'io.openmuse/genoffice-pptx',
        ),
  );
  await market.install(entry);
  final runtime = McpBundleRuntime(
    bundleDirectory: market.directoryFor(entry),
    nodeExecutable: const String.fromEnvironment(
      'TEST_NODE',
      defaultValue: 'node',
    ),
  );
  final plugin = (await discoverMcpFilePlugin(entry: entry, runtime: runtime))!;
  final file = await File(
    const String.fromEnvironment(
      'TEST_FILE',
      defaultValue:
          '/Users/mac/src/ioffice/samples/ppt-render/flutter/assets/Sample_ppt.pptx',
    ),
  ).copy('${root.path}/Sample_ppt.pptx');
  debugPrint('MCP_APP_SMOKE_RESOURCE ${file.path}');
  runApp(
    MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text('MCP Apps · ${entry.title} ${entry.version}'),
        ),
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
}
