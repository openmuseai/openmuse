import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Mobile market installs and runs A2UI QuickJS task board', (
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
    expect(find.text('任务看板'), findsWidgets);
    expect(find.textContaining('梳理 Mobile 插件协议'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, '真机交互已通过');
    await tester.tap(find.text('添加任务'));
    await tester.pumpAndSettle();
    expect(find.textContaining('真机交互已通过'), findsOneWidget);
    await tester.pageBack();
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('preview-$id')));
    await tester.pumpAndSettle(
      const Duration(milliseconds: 200),
      EnginePhase.sendSemanticsUpdate,
      const Duration(seconds: 90),
    );
    expect(find.text('任务看板'), findsWidgets);
    expect(
      find.textContaining('Should not initialize flutter_rust_bridge twice'),
      findsNothing,
    );
  });
}
