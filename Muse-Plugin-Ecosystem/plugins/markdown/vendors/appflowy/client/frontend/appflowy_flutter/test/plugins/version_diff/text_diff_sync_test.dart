import 'package:appflowy/plugins/version_diff/text/text_diff_engine.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_sync.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('identity mapping keeps the 1/3 viewport anchor', () {
    const extents = [25.0, 25.0, 25.0, 25.0, 25.0, 25.0, 25.0, 25.0];
    final prefix = MuseTextDiffSync.prefix(extents);
    final lines = [for (var i = 0; i < extents.length; i++) i.toDouble()];
    const boundaries = [
      MuseSimilarBoundary(oldLine: 0, newLine: 0),
      MuseSimilarBoundary(oldLine: 8, newLine: 8),
    ];

    for (final offset in [0.0, 12.0, 40.0, 80.0]) {
      final mapped = MuseTextDiffSync.mapOffset(
        sourceOffset: offset,
        viewportHeight: 50,
        sourcePrefix: prefix,
        targetPrefix: prefix,
        sourceLines: lines,
        targetLines: lines,
        boundaries: boundaries,
        fromLeft: true,
      );
      expect(mapped, closeTo(offset, 0.51));
    }
  });

  test('boundary transfer is monotonic and never crosses', () {
    final result = const MuseDartTextDiffEngine().compare(
      [for (var i = 0; i < 10; i++) 'old $i'].join('\n'),
      [
        for (var i = 0; i < 4; i++) 'old $i',
        'inserted a',
        'inserted b',
        'inserted c',
        for (var i = 4; i < 10; i++) 'old $i',
      ].join('\n'),
    );
    expect(MuseTextDiffSync.isMonotonic(result.similarBoundaries), isTrue);

    var previous = -1.0;
    for (var line = 0.0; line <= 10.0; line += 0.25) {
      final transferred = MuseTextDiffSync.transferLine(
        boundaries: result.similarBoundaries,
        sourceLine: line,
        fromLeft: true,
      );
      expect(transferred, greaterThanOrEqualTo(previous));
      previous = transferred;
    }
    expect(
      MuseTextDiffSync.transferLine(
        boundaries: result.similarBoundaries,
        sourceLine: 0,
        fromLeft: true,
      ),
      0,
    );
    expect(
      MuseTextDiffSync.transferLine(
        boundaries: result.similarBoundaries,
        sourceLine: 10,
        fromLeft: true,
      ),
      13,
    );
  });

  test('unequal prefixes still map inserts without reversing', () {
    const leftExtents = [25.0, 25.0, 25.0, 25.0];
    const rightExtents = [25.0, 2.0, 25.0, 25.0, 25.0];
    final leftPrefix = MuseTextDiffSync.prefix(leftExtents);
    final rightPrefix = MuseTextDiffSync.prefix(rightExtents);
    const boundaries = [
      MuseSimilarBoundary(oldLine: 0, newLine: 0),
      MuseSimilarBoundary(oldLine: 1, newLine: 1),
      MuseSimilarBoundary(oldLine: 1, newLine: 2),
      MuseSimilarBoundary(oldLine: 4, newLine: 5),
    ];
    final leftLines = [0.0, 1.0, 2.0, 3.0];
    final rightLines = [0.0, 1.0, 2.0, 3.0, 4.0];

    var previous = -1.0;
    for (var offset = 0.0; offset <= 50.0; offset += 5) {
      final mapped = MuseTextDiffSync.mapOffset(
        sourceOffset: offset,
        viewportHeight: 40,
        sourcePrefix: leftPrefix,
        targetPrefix: rightPrefix,
        sourceLines: leftLines,
        targetLines: rightLines,
        boundaries: boundaries,
        fromLeft: true,
      );
      expect(mapped, greaterThanOrEqualTo(previous - 0.01));
      previous = mapped;
    }
  });
}
