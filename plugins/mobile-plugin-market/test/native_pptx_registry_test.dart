import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';

void main() {
  test('approved native PPTX package installs from the live Registry', () async {
    final origin = Platform.environment['OPENMUSE_MCP_REGISTRY_URL'];
    if (origin == null || origin.isEmpty) return;
    final root = await Directory.systemTemp.createTemp('native-pptx-market-');
    try {
      final market = RemoteMcpMarket(
        Uri.parse(origin),
        File('${root.path}/receipts.json'),
        nativeMediaTypes: const {
          'io.openmuse/genoffice-pptx': {
            'application/vnd.openxmlformats-officedocument.presentationml.presentation',
          },
        },
      );
      final entry = (await market.list()).singleWhere(
        (value) => value.name == 'io.openmuse/genoffice-pptx',
      );
      expect(entry.version, matches(RegExp(r'^\d+\.\d+\.\d+$')));
      await market.install(entry);
      expect(await market.installed(), contains('${entry.name}@${entry.version}'));
      expect((await market.installedEntries()).single.version, entry.version);
      expect(
        await market.resourceMediaTypes(entry),
        contains('application/vnd.openxmlformats-officedocument.presentationml.presentation'),
      );
    } finally {
      await root.delete(recursive: true);
    }
  }, timeout: const Timeout(Duration(minutes: 3)));
}
