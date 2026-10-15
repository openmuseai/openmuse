import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:webview_flutter/webview_flutter.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Android installs and opens reviewed AppFlowy Editor', (
    tester,
  ) async {
    const registry = String.fromEnvironment('EDITOR_E2E_REGISTRY');
    if (registry.isEmpty) fail('EDITOR_E2E_REGISTRY required');
    final directory = await Directory.systemTemp.createTemp(
      'editor-remote-e2e-',
    );
    addTearDown(() => directory.delete(recursive: true));
    final page = RemoteMcpMarketPage(
      origin: Uri.parse(registry),
      receiptFile: File('${directory.path}/receipts.json'),
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
    await tester.tap(find.descendant(of: tile, matching: find.text('打开 / 使用')));
    for (
      var i = 0;
      i < 60 && find.byType(WebViewWidget).evaluate().isEmpty;
      i++
    ) {
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 500)),
      );
      await tester.pump();
    }
    expect(find.byType(WebViewWidget), findsOneWidget);
    final webview = tester.widget<WebViewWidget>(find.byType(WebViewWidget));
    final controller = webview.platform.params.controller;
    for (var i = 0; i < 60; i++) {
      final title = await controller.runJavaScriptReturningResult(
        'document.title',
      );
      if ('$title'.contains('AppFlowy Editor')) break;
      await Future<void>.delayed(const Duration(milliseconds: 500));
    }
    expect(
      '${await controller.runJavaScriptReturningResult('document.title')}',
      contains('AppFlowy Editor'),
    );
  });
}
