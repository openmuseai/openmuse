import 'dart:io';

import 'package:flutter/services.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets(
    'approved registry plugin installs and inspects PPTX over MCP on Android',
    (tester) async {
      const registry = String.fromEnvironment('OPENMUSE_MCP_REGISTRY_URL');
      if (registry.isEmpty) fail('Set OPENMUSE_MCP_REGISTRY_URL');
      final root = await Directory.systemTemp.createTemp('mobile-mcp-tck-');
      addTearDown(() => root.delete(recursive: true));
      final market = RemoteMcpMarket(
        Uri.parse(registry),
        File('${root.path}/installed.json'),
      );
      final entries = await market.list();
      final entry = entries.singleWhere(
        (item) => item.name == 'io.openmuse/genoffice-pptx',
      );
      await market.install(entry);
      expect(
        await market.installed(),
        contains('${entry.name}@${entry.version}'),
      );
      final bytes = await rootBundle.load(
        'test_assets/genoffice_standard_business.pptx',
      );
      final file = File('${root.path}/fixture.pptx');
      await file.writeAsBytes(bytes.buffer.asUint8List());
      final result = await market.inspectFile(entry, file);
      expect(result['slides'], greaterThan(0));
      expect(result['pages'], isNotEmpty);

      await tester.pumpWidget(
        MaterialApp(
          home: RemoteMcpMarketPage(
            origin: Uri.parse(registry),
            receiptFile: File('${root.path}/ui-installed.json'),
            pickFile: () async => file,
          ),
        ),
      );
      for (var i = 0; i < 50 && find.text('安装').evaluate().isEmpty; i++) {
        await tester.runAsync(
          () => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(find.text('安装'), findsOneWidget);
      await tester.tap(find.text('安装'));
      for (var i = 0; i < 50 && find.text('选择文件使用').evaluate().isEmpty; i++) {
        await tester.runAsync(
          () => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(find.text('选择文件使用'), findsOneWidget);
      await tester.tap(find.text('选择文件使用'));
      await tester.pump();
      expect(find.text('发送文件给插件？'), findsOneWidget);
      await tester.tap(find.text('发送'));
      for (
        var i = 0;
        i < 50 && find.textContaining('"slides":').evaluate().isEmpty;
        i++
      ) {
        await tester.runAsync(
          () => Future<void>.delayed(const Duration(milliseconds: 100)),
        );
        await tester.pump();
      }
      expect(find.textContaining('"slides":'), findsOneWidget);
    },
  );
}
