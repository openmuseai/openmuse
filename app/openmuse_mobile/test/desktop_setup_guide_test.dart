import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';

void main() {
  for (final tag in ['en', 'zh-Hans']) {
    testWidgets('current Desktop guide opens in $tag at 320px and 200% text', (
      tester,
    ) async {
      tester.view.physicalSize = const Size(320, 760);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final locale = OpenMuseLocaleController(initialTag: tag);
      addTearDown(locale.dispose);
      await tester.pumpWidget(
        OpenMuseLocaleScope(
          controller: locale,
          child: MaterialApp(
            locale: locale.locale,
            supportedLocales: locale.supportedLocales,
            localizationsDelegates: [
              OpenMuseStringsDelegate(locale.catalog),
              ...GlobalMaterialLocalizations.delegates,
            ],
            builder: (context, child) => MediaQuery(
              data: MediaQuery.of(
                context,
              ).copyWith(textScaler: TextScaler.linear(2)),
              child: child!,
            ),
            home: Builder(
              builder: (context) => Scaffold(
                body: TextButton(
                  onPressed: () => openDesktopSetupGuide(context),
                  child: const Text('Open guide'),
                ),
              ),
            ),
          ),
        ),
      );
      await tester.tap(find.text('Open guide'));
      final heading = tag == 'en' ? 'Connect a computer for tasks' : '连接电脑运行任务';
      for (
        var i = 0;
        i < 40 && find.textContaining(heading).evaluate().isEmpty;
        i++
      ) {
        await tester.runAsync(
          () => Future<void>.delayed(const Duration(milliseconds: 10)),
        );
        await tester.pump();
      }
      await tester.pumpAndSettle();
      expect(find.textContaining(heading), findsOneWidget);
      expect(
        find.textContaining(
          tag == 'en' ? 'Review AI sharing before sending' : '发送前核对 AI 数据共享',
        ),
        findsNothing,
      );
      expect(tester.takeException(), isNull);
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();
      expect(find.text('Open guide'), findsOneWidget);
    });
  }
}
