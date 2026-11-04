import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/services.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_mobile/mobile_plugins/mobile_my_page.dart';
import 'package:openmuse_mobile_plugin_market/src/market_discover_view.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'support/design_fixture.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  PackageInfo.setMockInitialValues(
    appName: 'OpenMuse',
    packageName: 'com.openmuseai.mobile',
    version: '1.0.0',
    buildNumber: '6',
    buildSignature: '',
  );
  for (final brightness in Brightness.values) {
    for (final language in ['en', 'zh-Hans']) {
      testWidgets('settings fit 320px with large text: $brightness $language', (
        tester,
      ) async {
        tester.view.physicalSize = const Size(320, 900);
        tester.view.devicePixelRatio = 1;
        addTearDown(tester.view.resetPhysicalSize);
        addTearDown(tester.view.resetDevicePixelRatio);
        final locale = OpenMuseLocaleController(initialTag: language);
        final auth = DesignAuthentication();
        addTearDown(locale.dispose);
        addTearDown(auth.dispose);
        await tester.pumpWidget(
          DesignHost(
            locale: locale,
            brightness: brightness,
            textScale: 1.5,
            home: MobileMyPage(
              themeMode: ThemeMode.system,
              onThemeMode: (_) {},
              workspaces: const [],
              deviceName: 'Desktop',
              authentication: auth,
              onSignOut: () {},
              onDeleteAccount: () async {},
            ),
          ),
        );
        await tester.pumpAndSettle();
        expect(find.byKey(const ValueKey('my-plugins')), findsOneWidget);
        expect(find.byKey(const ValueKey('my-devices')), findsNothing);
        expect(find.byKey(const ValueKey('my-delete-account')), findsNothing);
        expect(tester.takeException(), isNull);
        await tester.tap(find.byKey(const ValueKey('my-account')));
        await tester.pumpAndSettle();
        expect(find.byKey(const ValueKey('my-sign-out')), findsOneWidget);
        expect(find.byKey(const ValueKey('my-delete-account')), findsOneWidget);
        expect(tester.takeException(), isNull);
        await tester.tap(find.byKey(const ValueKey('settings-back')));
        await tester.pumpAndSettle();
        await tester.ensureVisible(find.byKey(const ValueKey('my-language')));
        await tester.tap(find.byKey(const ValueKey('my-language')));
        await tester.pumpAndSettle();
        expect(find.text('English'), findsOneWidget);
        expect(find.text('中文'), findsOneWidget);
        expect(tester.takeException(), isNull);
      });
    }
  }
  testWidgets(
    'Discover headings stay on one line and removed sections are absent',
    (tester) async {
      tester.view.physicalSize = const Size(320, 760);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final locale = OpenMuseLocaleController(initialTag: 'en');
      addTearDown(locale.dispose);
      await tester.pumpWidget(
        DesignHost(
          locale: locale,
          brightness: Brightness.light,
          textScale: 1.5,
          home: MarketDiscoverView(
            entries: const [],
            store: MobilePluginStore(Directory('/tmp/mobile-design-empty')),
            category: '精选',
            mine: false,
            query: '',
            onSection: (_) {},
            onCategory: (_) {},
            onSearch: (_) {},
            onDetail: (_) {},
            onInstall: (_) {},
            onOpen: (_) {},
            onPreview: (_) {},
            desktopSection: const Text('desktop sentinel'),
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('market-category-办公')), findsNothing);
      expect(find.text('desktop sentinel'), findsNothing);
      final a = tester.getCenter(
        find.byKey(const ValueKey('mobile-market-section-发现')),
      );
      final b = tester.getCenter(
        find.byKey(const ValueKey('mobile-market-section-我的')),
      );
      expect(a.dy, b.dy);
      await tester.drag(
        find.byKey(const ValueKey('mobile-market-sections')),
        const Offset(-200, 0),
      );
      await tester.pumpAndSettle();
      expect(tester.takeException(), isNull);
    },
  );
  testWidgets('settings visual reference at phone size', (tester) async {
    const out = String.fromEnvironment('OPENMUSE_UI_EVIDENCE_DIR');
    if (out.isEmpty) return;
    tester.view.physicalSize = const Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final locale = OpenMuseLocaleController(initialTag: 'en');
    addTearDown(locale.dispose);
    const key = ValueKey('capture');
    await tester.runAsync(() async {
      final font = FontLoader('UIEvidence');
      font.addFont(
        File(
          '/System/Library/Fonts/Supplemental/Arial.ttf',
        ).readAsBytes().then((v) => ByteData.sublistView(v)),
      );
      await font.load();
      final icons = FontLoader('MaterialIcons');
      icons.addFont(rootBundle.load('fonts/MaterialIcons-Regular.otf'));
      await icons.load();
    });
    for (final b in Brightness.values) {
      await tester.pumpWidget(
        DesignHost(
          locale: locale,
          brightness: b,
          captureKey: key,
          fontFamily: 'UIEvidence',
          home: MobileMyPage(
            themeMode: ThemeMode.system,
            onThemeMode: (_) {},
            workspaces: const [],
            deviceName: 'Desktop',
            authentication: DesignAuthentication(),
          ),
        ),
      );
      await tester.pumpAndSettle();
      final boundary = tester.renderObject<RenderRepaintBoundary>(
        find.byKey(key),
      );
      await tester.runAsync(() async {
        final image = await boundary.toImage(pixelRatio: 2);
        final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
        image.dispose();
        final p = File('$out/account-${b.name}.png');
        await p.parent.create(recursive: true);
        await p.writeAsBytes(bytes!.buffer.asUint8List());
      });
    }
  });
}
