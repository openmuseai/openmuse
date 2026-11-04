import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:genoffice_pptx_flutter/genoffice_pptx_flutter.dart';

void main() {
  final samplePath = Platform.environment['OPENMUSE_PPTX_SAMPLE_PATH'];

  testWidgets(
    'sample deck opens in the native Flutter viewer and advances slides',
    (tester) async {
      final bytes = Uint8List.fromList(File(samplePath!).readAsBytesSync());
      await tester.pumpWidget(MaterialApp(
        home: Scaffold(
          body: GenOfficePptxViewer(bytes: bytes, title: 'Sample_ppt.pptx'),
        ),
      ));
      await tester.pump(const Duration(seconds: 1));
      await tester.pump();

      expect(find.text('1 / 23'), findsOneWidget);
      expect(find.textContaining('20XX'), findsWidgets);
      expect(tester.takeException(), isNull);

      await tester.tap(find.byTooltip('Next slide'));
      await tester.pump();
      expect(find.text('2 / 23'), findsOneWidget);
      expect(tester.takeException(), isNull);
    },
    skip: samplePath == null || !File(samplePath).existsSync(),
  );
}
