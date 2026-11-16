import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_market_ui/openmuse_market_ui.dart';

void main() {
  testWidgets(
    'configuration uses host revision; detect and disable wait for host acknowledgement',
    (tester) async {
      final calls = <(String, Map<String, Object?>)>[];
      var revision = 4, enabled = false, detected = false;
      Map<String, Object?> config = {'idleSeconds': 300, 'maxInstances': 2};
      await tester.pumpWidget(
        MaterialApp(
          home: CodeNavigationSettingsPage(
            pluginId: 'com.openmuse.code-navigation',
            title: '代码导航服务',
            targetLabel: 'Desktop',
            request: (operation, id, options) async {
              calls.add((operation, options));
              if (operation == 'configure') {
                expect(options['expectedRevision'], revision);
                config = Map<String, Object?>.from(options['values'] as Map);
                revision++;
              }
              if (operation == 'detect') detected = true;
              if (operation == 'enable') enabled = true;
              if (operation == 'disable') enabled = false;
              return {
                'installedVersion': '0.1.1',
                'revision': revision,
                'configuration': config,
                'detected': detected,
                'enabled': enabled,
                'instances': 0,
              };
            },
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(find.textContaining('已安装 · 已关闭'), findsOneWidget);
      await tester.enterText(find.byType(TextField).first, '60');
      await tester.tap(find.text('保存配置'));
      await tester.pumpAndSettle();
      expect(config['idleSeconds'], 60);
      await tester.tap(find.text('检测配置'));
      await tester.pumpAndSettle();
      expect(find.text('检测通过 · 临时语言进程已释放'), findsOneWidget);
      await tester.tap(find.text('启用插件'));
      await tester.pumpAndSettle();
      expect(enabled, true);
      await tester.tap(find.text('关闭并释放'));
      await tester.pumpAndSettle();
      expect(enabled, false);
      expect(calls.map((x) => x.$1), [
        'plan',
        'configure',
        'detect',
        'enable',
        'disable',
      ]);
    },
  );
  testWidgets('stale configuration and failed release never show success', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: CodeNavigationSettingsPage(
          pluginId: 'com.openmuse.code-navigation',
          title: '代码导航服务',
          targetLabel: 'Desktop',
          request: (operation, id, options) async {
            if (operation == 'configure')
              throw StateError('CONFIGURATION_CONFLICT');
            if (operation == 'disable') throw StateError('STOP_FAILED');
            return {
              'installedVersion': '0.1.1',
              'revision': 3,
              'configuration': {'idleSeconds': 300, 'maxInstances': 2},
              'detected': true,
              'enabled': true,
              'instances': 1,
            };
          },
        ),
      ),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('保存配置'));
    await tester.pumpAndSettle();
    expect(find.textContaining('另一端更新'), findsOneWidget);
    await tester.tap(find.text('关闭并释放'));
    await tester.pumpAndSettle();
    expect(find.textContaining('STOP_FAILED'), findsOneWidget);
    expect(find.textContaining('已启用 · 按需启动'), findsOneWidget);
    expect(find.text('已关闭，语言进程已释放'), findsNothing);
  });
}
