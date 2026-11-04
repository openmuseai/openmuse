import 'dart:convert';
import 'dart:io';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';

void main() {
  final bundlePath = Platform.environment['OPENMUSE_MCPB_SMOKE_BUNDLE'];
  test(
    'independent MCPB installs and exposes its declared tools or resources',
    () async {
      if (bundlePath == null) return;
      final root = await Directory.systemTemp.createTemp(
        'openmuse-mcpb-smoke-',
      );
      addTearDown(() => root.delete(recursive: true));
      final bytes = await File(bundlePath).readAsBytes();
      final archive = ZipDecoder().decodeBytes(bytes);
      final manifest =
          jsonDecode(
                utf8.decode(
                  archive.findFile('manifest.json')!.content as List<int>,
                ),
              )
              as Map;
      final entry = McpMarketEntry(
        name: 'smoke/${manifest['name']}',
        title: '${manifest['display_name'] ?? manifest['name']}',
        description: '${manifest['description'] ?? ''}',
        version: manifest['version'] as String,
        packageUri: Uri.parse('http://127.0.0.1/smoke.mcpb'),
        sha256: sha256.convert(bytes).toString(),
      );
      final market = McpRegistryMarket(
        origin: Uri.parse('http://127.0.0.1'),
        installRoot: root,
        nodeExecutable: 'node',
      );
      final result = await market.install(entry, bundleBytes: bytes);
      expect(result['action'], 'installed');
      final runtime = McpBundleRuntime(
        bundleDirectory: market.directoryFor(entry),
        nodeExecutable: 'node',
      );
      final tools = await runtime.tools();
      if (tools.isEmpty) {
        final session = await runtime.connect();
        try {
          final resources = await session.client.listResources();
          expect(resources.resources, isNotEmpty);
        } finally {
          await session.close();
        }
        return;
      }
      final launch = tools.firstWhere(
        (tool) => tool['uiResourceUri'] is String,
      );
      expect(
        await runtime.uiDocument(launch['name'] as String),
        contains('<html'),
      );
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
}
