import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:genoffice_pptx_flutter/genoffice_pptx_flutter.dart';

void main() {
  testWidgets(
    'verified message overrides update the preview without reopening the deck',
    (tester) async {
      final bytes = File(
        '../vendor/pptx-engine/tests/fixtures/01_standard_business.pptx',
      ).readAsBytesSync();
      Widget preview(Map<String, String> messages) => MaterialApp(
        home: Scaffold(
          body: GenOfficePptxViewer(
            bytes: bytes,
            title: 'Fixture.pptx',
            messages: messages,
          ),
        ),
      );
      await tester.pumpWidget(preview({}));
      await tester.pumpAndSettle();
      await tester.tap(find.byTooltip('Next slide'));
      await tester.pump();
      expect(find.text('2 / 5'), findsOneWidget);
      await tester.pumpWidget(
        preview({'nextPage': '下一页', 'previousPage': '上一页'}),
      );
      await tester.pump();
      expect(find.byTooltip('下一页'), findsOneWidget);
      expect(find.text('2 / 5'), findsOneWidget);
      expect(tester.takeException(), isNull);
    },
  );
}
