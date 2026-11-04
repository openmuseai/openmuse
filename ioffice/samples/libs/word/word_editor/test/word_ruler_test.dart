import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:word_editor/src/word_ruler.dart';
import 'package:word_render/word_render.dart' as wr;

void main() {
  final page = wr.PageMetrics(
    pageCount: 1,
    pageWidthTwip: 14400,
    pageHeightTwip: 18000,
    marginLeftTwip: 1440,
    marginRightTwip: 1440,
    marginTopTwip: 1440,
    marginBottomTwip: 1440,
    paperSize: 'test',
  );

  test('ruler measures outward from the word-core text origin', () {
    final inches = WordRulerGeometry(page, WordRulerUnit.inches);
    final ticks = inches.ticks.toList();
    expect(inches.width, 960);
    expect(inches.left, 1440);
    expect(inches.right, 1440);
    expect(ticks.first.positionTwip, 1620);
    expect(ticks.first.kind, WordRulerTickKind.minor);
    expect(
        ticks
            .where((tick) => tick.kind == WordRulerTickKind.number)
            .first
            .positionTwip,
        2880);
    expect(inches.defaultTabStops.first, 2160);
    expect(inches.defaultTabStops.last, 12240);
    final metric = WordRulerGeometry(page, WordRulerUnit.centimeters);
    expect(metric.ticks.first.positionTwip, closeTo(1581.73, 0.01));
  });

  testWidgets('ruler zoom follows the page width and height', (tester) async {
    await tester.pumpWidget(MaterialApp(
      locale: const Locale('zh', 'CN'),
      home: Center(
        child: SizedBox(
          height: 32.5,
          child: SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            child: WordRuler(page: page, zoom: 1.25),
          ),
        ),
      ),
    ));
    expect(tester.getSize(find.byKey(const Key('word-page-ruler'))),
        const Size(1200, 32.5));
    expect(tester.takeException(), isNull);
  });
}
