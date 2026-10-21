import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:webview_flutter/webview_flutter.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Mobile market installs and opens WebView task board', (
    tester,
  ) async {
    final directory = await Directory.systemTemp.createTemp(
      'openmuse-market-e2e-',
    );
    addTearDown(() => directory.delete(recursive: true));
    final store = MobilePluginStore(directory);
    await tester.pumpWidget(MaterialApp(home: MobileMarketPage(store: store)));
    await tester.pumpAndSettle();
    const id = 'com.openmuse.mobile.task-board';
    expect(find.text('发现'), findsOneWidget);
    await tester.ensureVisible(
      find.byKey(const ValueKey('market-category-生活')),
    );
    await tester.tap(find.byKey(const ValueKey('market-category-生活')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('market-empty-category')), findsOneWidget);
    await tester.ensureVisible(
      find.byKey(const ValueKey('market-category-办公')),
    );
    await tester.tap(find.byKey(const ValueKey('market-category-办公')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('market-entry-$id')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('detail-action-$id')), findsOneWidget);
    await tester.pageBack();
    await tester.pumpAndSettle();
    await tester.ensureVisible(find.byKey(const ValueKey('install-$id')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('install-$id')), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('install-$id')));
    await tester.pumpAndSettle();
    expect(store.isInstalled(id), isTrue);
    await tester.tap(find.byKey(const ValueKey('open-$id')));
    await tester.pumpAndSettle(
      const Duration(milliseconds: 200),
      EnginePhase.sendSemanticsUpdate,
      const Duration(seconds: 90),
    );
    final controller = tester
        .widget<WebViewWidget>(find.byType(WebViewWidget))
        .platform
        .params
        .controller;
    await Future<void>.delayed(const Duration(seconds: 2));
    expect(
      (await controller.runJavaScriptReturningResult(
        'document.body.innerText',
      )).toString(),
      contains('梳理 Mobile 插件协议'),
    );
    await controller.runJavaScript(
      "document.getElementById('task').value='设备端新任务';"
      "document.getElementById('add').click()",
    );
    expect(
      (await controller.runJavaScriptReturningResult(
        'document.body.innerText',
      )).toString(),
      contains('设备端新任务'),
    );
    await tester.pageBack();
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('preview-$id')));
    await tester.pumpAndSettle(
      const Duration(milliseconds: 200),
      EnginePhase.sendSemanticsUpdate,
      const Duration(seconds: 90),
    );
    expect(find.byType(WebViewWidget), findsOneWidget);
    expect(
      find.textContaining('Should not initialize flutter_rust_bridge twice'),
      findsNothing,
    );
  });
}
