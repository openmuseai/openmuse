import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Android installs and opens reviewed AppFlowy Editor', (
    tester,
  ) async {
    if (!Platform.isAndroid) fail('This integration test requires Android');
    const registry = String.fromEnvironment('EDITOR_E2E_REGISTRY');
    if (registry.isEmpty) fail('EDITOR_E2E_REGISTRY required');
    final directory = await getApplicationSupportDirectory();
    final receiptFile = File(
      '${directory.path}/editor-offline-e2e-receipts.json',
    );
    if (await receiptFile.exists()) await receiptFile.delete();
    addTearDown(() => receiptFile.delete().then((_) {}, onError: (_) {}));
    final source = RemoteMcpMarket(Uri.parse(registry), receiptFile);
    final entry = (await source.list()).singleWhere(
      (item) => item.name == 'io.openmuse/appflowy-editor',
    );
    final page = RemoteMcpMarketPage(
      origin: Uri.parse(registry),
      receiptFile: receiptFile,
    );
    await tester.pumpWidget(MaterialApp(home: page));
    for (
      var i = 0;
      i < 60 && find.text('AppFlowy Editor').evaluate().isEmpty;
      i++
    ) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 500)),
      );
      await tester.pump();
    }
    expect(find.text('AppFlowy Editor'), findsOneWidget);
    final tile = find.ancestor(
      of: find.text('AppFlowy Editor'),
      matching: find.byType(ListTile),
    );
    await tester.tap(find.descendant(of: tile, matching: find.text('安装')));
    for (
      var i = 0;
      i < 60 &&
          find
              .descendant(of: tile, matching: find.text('打开 / 使用'))
              .evaluate()
              .isEmpty;
      i++
    ) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 500)),
      );
      await tester.pump();
    }
    expect(
      find.descendant(of: tile, matching: find.text('打开 / 使用')),
      findsOneWidget,
    );
    expect(source.hasOfflineUi(entry), isTrue);
    expect(
      await File(
        '${source.packageDirectory(entry).path}/main.dart.wasm',
      ).exists(),
      isTrue,
    );
    debugPrint('EDITOR_E2E_OFFLINE_INSTALLED');
    if (const bool.fromEnvironment('EDITOR_E2E_PAUSE_BEFORE_OPEN')) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(seconds: 15)),
      );
    }
    await tester.tap(find.descendant(of: tile, matching: find.text('打开 / 使用')));
    for (
      var i = 0;
      i < 60 && find.byType(AndroidView).evaluate().isEmpty;
      i++
    ) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 500)),
      );
      await tester.pump();
    }
    expect(find.byType(AndroidView), findsOneWidget);
    debugPrint('EDITOR_E2E_OFFLINE_OPENED');
    if (const bool.fromEnvironment('EDITOR_E2E_CAPTURE_EVIDENCE')) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(seconds: 60)),
      );
    }
  });
}
