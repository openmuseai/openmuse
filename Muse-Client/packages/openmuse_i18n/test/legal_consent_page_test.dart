import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/src/generated_strings.dart';
import 'package:openmuse_i18n/src/legal_consent_page.dart';

void main() {
  setUpAll(() async {
    if (Platform.environment['OPENMUSE_CAPTURE_LEGAL'] != '1') return;
    // System fonts are loaded for local visual review only, never bundled.
    for (final font in [
      ('OpenMusePreview', '/System/Library/Fonts/SFNS.ttf'),
      ('OpenMuseCJK', '/System/Library/Fonts/Supplemental/Arial Unicode.ttf'),
      (
        'MaterialIcons',
        '/Users/mac/src/flutter/bin/cache/artifacts/material_fonts/MaterialIcons-Regular.otf',
      ),
    ]) {
      final file = File(font.$2);
      if (await file.exists()) {
        final loader = FontLoader(font.$1);
        loader.addFont(
          file.readAsBytes().then((bytes) => ByteData.sublistView(bytes)),
        );
        await loader.load();
      }
    }
  });
  for (final scenario in [
    ('phone-en-light', const Size(390, 844), 'en', Brightness.light, 1.0),
    ('phone-zh-dark', const Size(390, 844), 'zh-Hans', Brightness.dark, 1.0),
    ('small-phone', const Size(320, 568), 'en', Brightness.light, 1.0),
    ('large-text', const Size(320, 640), 'en', Brightness.dark, 2.0),
    ('tablet', const Size(768, 1024), 'en', Brightness.light, 1.0),
    ('desktop-en-light', const Size(1440, 900), 'en', Brightness.light, 1.0),
    ('desktop-en-dark', const Size(1440, 900), 'en', Brightness.dark, 1.0),
    ('desktop-short', const Size(1024, 600), 'zh-Hans', Brightness.light, 1.0),
  ]) {
    testWidgets('legal consent adapts: ${scenario.$1}', (tester) async {
      tester.view.physicalSize = scenario.$2;
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      var checked = false;
      var accepted = 0, declined = 0, terms = 0, privacy = 0, region = 0;
      final strings = OpenMuseStrings.forLocale(scenario.$3);
      await tester.pumpWidget(
        MaterialApp(
          debugShowCheckedModeBanner: false,
          theme: openMuseOnboardingTheme(scenario.$4).copyWith(
            textTheme: openMuseOnboardingTheme(scenario.$4).textTheme.apply(
              fontFamily: Platform.environment['OPENMUSE_CAPTURE_LEGAL'] == '1'
                  ? 'OpenMusePreview'
                  : null,
              fontFamilyFallback: const ['OpenMuseCJK'],
            ),
            filledButtonTheme: FilledButtonThemeData(
              style: openMuseOnboardingTheme(scenario.$4)
                  .filledButtonTheme
                  .style!
                  .copyWith(
                    textStyle: const WidgetStatePropertyAll(
                      TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                        fontFamily: 'OpenMusePreview',
                        fontFamilyFallback: ['OpenMuseCJK'],
                      ),
                    ),
                  ),
            ),
            textButtonTheme: TextButtonThemeData(
              style: openMuseOnboardingTheme(scenario.$4).textButtonTheme.style!
                  .copyWith(
                    textStyle: const WidgetStatePropertyAll(
                      TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w500,
                        fontFamily: 'OpenMusePreview',
                        fontFamilyFallback: ['OpenMuseCJK'],
                      ),
                    ),
                  ),
            ),
          ),
          home: MediaQuery(
            data: MediaQueryData(
              size: scenario.$2,
              textScaler: TextScaler.linear(scenario.$5),
            ),
            child: RepaintBoundary(
              key: const ValueKey('capture'),
              child: StatefulBuilder(
                builder: (context, setState) => OpenMuseLegalConsentPage(
                  strings: strings,
                  regionLabel: scenario.$3 == 'en' ? 'United States' : '中国大陆',
                  checked: checked,
                  busy: false,
                  failed: false,
                  onChecked: (value) => setState(() => checked = value),
                  onTerms: () => terms++,
                  onPrivacy: () => privacy++,
                  onAccept: () => accepted++,
                  onDecline: () => setState(() {
                    checked = false;
                    declined++;
                  }),
                  onChangeRegion: () => region++,
                ),
              ),
            ),
          ),
        ),
      );
      await tester.runAsync(
        () => precacheImage(
          const AssetImage(
            'packages/openmuse_i18n/assets/brand/openmuse-mark.png',
          ),
          tester.element(find.byKey(const ValueKey('capture'))),
        ),
      );
      await tester.pumpAndSettle();
      expect(tester.takeException(), isNull);
      final accept = find.byKey(const ValueKey('legal-accept'));
      expect(tester.widget<FilledButton>(accept).onPressed, isNull);
      // Even on short/large-type screens, consent controls are reachable.
      await tester.ensureVisible(find.byKey(const ValueKey('legal-checkbox')));
      await tester.tap(find.byKey(const ValueKey('legal-checkbox')));
      await tester.pumpAndSettle();
      expect(checked, isTrue);
      await tester.ensureVisible(accept);
      await tester.tap(accept);
      expect(accepted, 1);
      await tester.ensureVisible(find.byKey(const ValueKey('legal-decline')));
      await tester.tap(find.byKey(const ValueKey('legal-decline')));
      await tester.pumpAndSettle();
      expect(checked, isFalse);
      expect(declined, 1);
      expect(tester.widget<FilledButton>(accept).onPressed, isNull);
      for (final key in [
        'legal-terms',
        'legal-privacy',
        'legal-change-region',
      ]) {
        final finder = find.byKey(ValueKey(key));
        await tester.ensureVisible(finder);
        await tester.pumpAndSettle();
        await tester.tap(finder);
        await tester.pump();
      }
      expect([terms, privacy, region], [1, 1, 1]);
      expect(tester.takeException(), isNull);
      if (Platform.environment['OPENMUSE_CAPTURE_LEGAL'] == '1') {
        // Capture the clean first-use state, not a scrolled interaction state.
        for (final scroll in tester.widgetList<Scrollable>(
          find.byType(Scrollable),
        )) {
          scroll.controller?.jumpTo(0);
        }
        final scrollables = tester.stateList<ScrollableState>(
          find.byType(Scrollable),
        );
        for (final scroll in scrollables) {
          scroll.position.jumpTo(0);
        }
        await tester.pumpAndSettle();
        final boundary = tester.renderObject<RenderRepaintBoundary>(
          find.byKey(const ValueKey('capture')),
        );
        await tester.runAsync(() async {
          final picture = await boundary.toImage(pixelRatio: 1.5);
          final bytes = (await picture.toByteData(
            format: ui.ImageByteFormat.png,
          ))!;
          final output = File(
            '../../docs/design/legal-onboarding/evidence/${scenario.$1}.png',
          );
          await output.parent.create(recursive: true);
          await output.writeAsBytes(bytes.buffer.asUint8List());
          picture.dispose();
        });
      }
    });
  }
}
