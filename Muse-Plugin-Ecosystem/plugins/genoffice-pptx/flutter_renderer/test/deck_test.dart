import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:genoffice_pptx_flutter/genoffice_pptx_flutter.dart';

void main() {
  test('parses the independent GenOffice fixture into slide shapes', () {
    final file = File('../vendor/pptx-engine/tests/fixtures/01_standard_business.pptx');
    final deck = PptxDeck.fromBytes(file.readAsBytesSync());
    expect(deck.slides, isNotEmpty);
    expect(deck.width, greaterThan(0));
    expect(deck.slides.first.shapes, isNotEmpty);
  });

  test('opens the 23 page visual acceptance deck when available', () {
    final file = File(Platform.environment['PPTX_SAMPLE'] ??
        '/Users/mac/src/ioffice/samples/ppt-render/flutter/assets/Sample_ppt.pptx');
    if (!file.existsSync()) return;
    final deck = PptxDeck.fromBytes(file.readAsBytesSync());
    expect(deck.slides, hasLength(23));
    expect(deck.slides.first.shapes.any((s) => s.text.contains('年终汇报')), isTrue);
    expect(deck.slides.first.shapes.any((s) => s.image != null), isTrue);
    expect(deck.slides.first.shapes.first.image, isNotNull);
  });
}
