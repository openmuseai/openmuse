import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';

void main() {
  final registryUrl = Platform.environment['OPENMUSE_MCP_REGISTRY_URL'];
  final installRoot = Platform.environment['OPENMUSE_MCP_INSTALL_ROOT'];
  final samplePath = Platform.environment['OPENMUSE_PPTX_SAMPLE_PATH'] ??
      '../openmuse_mobile/test_assets/genoffice_standard_business.pptx';
  test(
    'approved MCPB installs from online registry and invokes PPTX tool',
    () async {
      if (registryUrl == null || registryUrl.isEmpty) return;
      final temporary = installRoot == null
          ? await Directory.systemTemp.createTemp('mcp-market-host-')
          : await Directory(installRoot).create(recursive: true);
      try {
        final market = McpRegistryMarket(
          origin: Uri.parse(registryUrl),
          installRoot: temporary,
        );
        final entries = await market.list();
        final entry = entries.singleWhere(
          (item) => item.name == 'io.openmuse/genoffice-pptx',
        );
        final plan = await market.plan(entry);
        expect(plan['action'], anyOf('install', 'already-installed'));
        final installed = await market.install(entry);
        expect(installed['action'], anyOf('installed', 'already-installed'));
        final runtime = McpBundleRuntime(
          bundleDirectory: market.directoryFor(entry),
          nodeExecutable: 'node',
        );
        final tools = await runtime.tools();
        expect(tools.any((tool) => tool['name'] == 'inspect_pptx'), true);
        final result = await runtime.callWithFile(
          toolName: 'inspect_pptx',
          filePath: samplePath,
        );
        expect(result['slides'], greaterThan(0));
        expect((await market.plan(entry))['action'], 'already-installed');
      } finally {
        if (installRoot == null) await temporary.delete(recursive: true);
      }
    },
  );
}
