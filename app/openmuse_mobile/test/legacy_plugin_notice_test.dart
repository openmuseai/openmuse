import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/legacy_plugin_notice.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
  for (final width in [320.0, 691.0]) {
    testWidgets(
      'legacy upgrade warning retains an accessible workspace at $width and 200% text',
      (tester) async {
        tester.view.physicalSize = Size(width, 800);
        tester.view.devicePixelRatio = 1;
        tester.view.padding = const FakeViewPadding(top: 59, bottom: 34);
        tester.view.viewPadding = const FakeViewPadding(top: 59, bottom: 34);
        addTearDown(tester.view.resetPhysicalSize);
        addTearDown(tester.view.resetDevicePixelRatio);
        addTearDown(tester.view.resetPadding);
        addTearDown(tester.view.resetViewPadding);
        final directory = (await tester.runAsync(
          () => Directory.systemTemp.createTemp('legacy-notice-'),
        ))!;
        addTearDown(() => directory.delete(recursive: true));
        final store = MobilePluginStore(
          directory,
          allowDownloadedPlugins: false,
        );
        addTearDown(store.dispose);
        await tester.pumpWidget(
          MaterialApp(
            builder: (context, child) => MediaQuery(
              data: MediaQuery.of(
                context,
              ).copyWith(textScaler: TextScaler.linear(2)),
              child: child!,
            ),
            home: LegacyPluginNotice(
              store: store,
              child: Scaffold(
                appBar: AppBar(title: const Text('Document')),
                body: const Center(child: Text('Workspace retained')),
              ),
            ),
          ),
        );
        expect(
          find.byKey(const ValueKey('legacy-plugin-notice')),
          findsNothing,
        );
        await tester.runAsync(() async {
          await File(
            '${directory.path}/state.json',
          ).writeAsString('unrecognized legacy record');
          await store.restore();
        });
        await tester.pumpAndSettle();
        expect(
          find.byKey(const ValueKey('legacy-plugin-notice')),
          findsOneWidget,
        );
        expect(find.text('Workspace retained'), findsOneWidget);
        expect(
          tester.getSize(find.byType(AppBar)).height,
          closeTo(kToolbarHeight, 0.1),
          reason: 'The upgrade banner already protects the system top inset',
        );
        expect(
          find.textContaining(
            'original directories and user files are retained',
          ),
          findsOneWidget,
        );
        expect(tester.takeException(), isNull);
      },
    );
  }
}
