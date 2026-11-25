import 'dart:convert';
import 'dart:io';
import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mcp_dart/mcp_dart.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';

/// Opt-in conformance for an independently packaged, remote-backed MCP App.
/// Credentials stay in test configuration, never in the HTML document.
void main() {
  final bundlePath = Platform.environment['OPENMUSE_MCPB_SMOKE_BUNDLE'];
  final configurationText = Platform.environment['OPENMUSE_MCPB_SMOKE_CONFIG'];
  final toolName = Platform.environment['OPENMUSE_MCPB_SMOKE_TOOL'];
  test(
    'installed remote-backed App forwards a configured server tool',
    () async {
      final bytes = await File(bundlePath!).readAsBytes();
      final archive = ZipDecoder().decodeBytes(bytes);
      final manifest =
          jsonDecode(
                utf8.decode(
                  archive.findFile('manifest.json')!.content as List<int>,
                ),
              )
              as Map;
      final root = await Directory.systemTemp.createTemp(
        'openmuse-remote-app-',
      );
      addTearDown(() => root.delete(recursive: true));
      final market = McpRegistryMarket(
        origin: Uri.parse('http://127.0.0.1'),
        installRoot: root,
        nodeExecutable: 'node',
      );
      final entry = McpMarketEntry(
        name: 'conformance/${manifest['name']}',
        title: '${manifest['display_name']}',
        description: 'Independent App conformance',
        version: manifest['version'] as String,
        packageUri: Uri.parse('http://127.0.0.1/app.mcpb'),
        sha256: sha256.convert(bytes).toString(),
      );
      await market.install(entry, bundleBytes: bytes);
      final runtime = McpBundleRuntime(
        bundleDirectory: market.directoryFor(entry),
        nodeExecutable: 'node',
      );
      final configuration = Map<String, Object?>.from(
        jsonDecode(configurationText!) as Map,
      );
      final session = await runtime.connect(configuration: configuration);
      addTearDown(session.close);
      final tools = await session.client.listTools();
      expect(tools.tools.any((tool) => tool.name == toolName), isTrue);
      final result = await session.client.callTool(
        CallToolRequest(name: toolName!, arguments: const {}),
      );
      expect(result.isError, isNot(true));
      final output = result.toJson();
      expect(output['structuredContent'], isA<Map>());
      final declaredTools = await runtime.tools();
      final uiUri =
          declaredTools.firstWhere(
                (tool) => tool['uiResourceUri'] is String,
              )['uiResourceUri']
              as String;
      final document = await session.client.readResource(
        ReadResourceRequest(uri: uiUri),
      );
      final text = jsonEncode(document.toJson());
      expect(text, contains('<html'));
      for (final key in configuration.keys.where(
        (key) => key.contains('token'),
      )) {
        final token = configuration[key];
        if (token is String && token.isNotEmpty)
          expect(text.contains(token), isFalse);
      }
    },
    skip: bundlePath == null || configurationText == null || toolName == null
        ? 'Set the opt-in bundle, configuration and tool environment.'
        : false,
    timeout: const Timeout(Duration(minutes: 2)),
  );
}
