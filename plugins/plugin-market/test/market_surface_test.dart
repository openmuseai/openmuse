import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_market/openmuse_plugin_market.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  for (final width in [580.0, 1100.0]) {
    testWidgets('light market works at ${width.toInt()} px', (tester) async {
      tester.view.physicalSize = Size(width, 900);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      var installed = false;
      var enabled = false;
      final market = OpenMusePluginMarket();
      await market.activate(
        OpenMusePluginContext(
          executeHostCommand: (command, args) async {
            if (command == 'plugin.market.list') {
              return [
                {
                  'id': 'archify',
                  'name': 'Archify',
                  'version': '1.0.0',
                  'package': '@tt-a1i/archify-dsh',
                  'ecosystem': 'dsh',
                  'categories': ['编程'],
                  'description': '把项目结构转为架构图。',
                },
              ];
            }
            if (command == 'plugin.market.install') {
              installed = true;
              enabled = true;
            }
            if (command == 'plugin.market.disable') enabled = false;
            if (command == 'plugin.market.enable') enabled = true;
            return {
              'id': 'archify',
              'version': '1.0.0',
              'installedVersion': installed ? '1.0.0' : null,
              'enabled': enabled,
              'profile': 'test',
              'action': installed ? 'already-installed' : 'install',
            };
          },
        ),
      );
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Builder(
              builder: (context) =>
                  market.buildPanel(context, 'plugin.market')!,
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(find.text('探索插件'), findsOneWidget);
      expect(
        find.byKey(const ValueKey('plugin-market-card-archify')),
        findsOneWidget,
      );
      final categoryCenter = tester.getCenter(
        find.byKey(const ValueKey('plugin-market-category-精选')),
      );
      final searchCenter = tester.getCenter(
        find.byKey(const ValueKey('plugin-market-search')),
      );
      if (width >= 820) {
        expect((categoryCenter.dy - searchCenter.dy).abs(), lessThan(20));
      } else {
        expect(searchCenter.dy, greaterThan(categoryCenter.dy));
      }
      await tester.enterText(
          find.byKey(const ValueKey('plugin-market-search')), '不存在的插件');
      await tester.pumpAndSettle();
      expect(find.text('没有匹配的插件'), findsOneWidget);
      await tester.enterText(
          find.byKey(const ValueKey('plugin-market-search')), 'Archify');
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('plugin-market-card-archify')),
          findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('plugin-market-category-生活')));
      await tester.pumpAndSettle();
      expect(find.text('没有匹配的插件'), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('plugin-market-category-精选')));
      await tester.pumpAndSettle();
      await tester.ensureVisible(
        find.byKey(const ValueKey('plugin-market-card-archify')),
      );
      await tester.tap(
        find.byKey(const ValueKey('plugin-market-card-archify')),
      );
      await tester.pumpAndSettle();
      expect(
        find.byKey(const ValueKey('plugin-market-detail-archify')),
        findsOneWidget,
      );
      await tester.ensureVisible(find.text('安装到 Desktop'));
      await tester.tap(find.text('安装到 Desktop'));
      await tester.pumpAndSettle();
      expect(find.text('禁用插件'), findsOneWidget);
      await tester.tap(find.text('禁用插件'));
      await tester.pumpAndSettle();
      expect(find.text('启用插件'), findsOneWidget);
      await tester.tap(find.text('启用插件'));
      await tester.pumpAndSettle();
      expect(find.text('禁用插件'), findsOneWidget);
      expect(tester.takeException(), isNull);
    });
  }
}
