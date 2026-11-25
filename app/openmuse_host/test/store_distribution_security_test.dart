import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';
import 'package:openmuse_host/src/host/mcp_installed_app.dart';

void main() {
  test(
    'Store rejects network installation and development code before execution',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'openmuse-store-policy-',
      );
      try {
        final market = McpRegistryMarket(
          origin: Uri.parse('https://invalid.example'),
          installRoot: root,
        );
        final entry = McpMarketEntry(
          name: 'test/plugin',
          title: 'Test',
          description: 'Test',
          version: '1.0.0',
          packageUri: Uri.parse('https://invalid.example/package.mcpb'),
          sha256: '0' * 64,
        );
        await expectLater(market.install(entry), throwsUnsupportedError);
        final apps = McpAppsEnvironment(
          market: market,
          nodeExecutable: 'must-not-run',
          scope: () => 'test',
        );
        await expectLater(
          apps.openDevelopment('test', {'content': 'not valid base64'}),
          throwsUnsupportedError,
        );
        expect(await root.list().isEmpty, isTrue);
      } finally {
        await root.delete(recursive: true);
      }
    },
    skip: const bool.fromEnvironment('OPENMUSE_MAC_APP_STORE')
        ? false
        : 'Run this channel-specific check with OPENMUSE_MAC_APP_STORE=true',
  );
}
