import 'dart:ui';

import 'package:flutter_test/flutter_test.dart';
import 'package:ppt_render/ppt_render.dart';

void main() {
  test('T01 empty fill/stroke DrawRect does not default to black stroke', () {
    final painter = RenderPainter([
      <String, dynamic>{
        'type': 'DrawRect',
        'x': 0,
        'y': 0,
        'w': 10,
        'h': 10,
      },
    ]);
    final recorder = PictureRecorder();
    final canvas = Canvas(recorder);
    painter.paint(canvas, const Size(20, 20));
    expect(painter.skipped, contains('emptyStyle'));
  });

  test('T02 filled rect still fills', () {
    final painter = RenderPainter([
      <String, dynamic>{
        'type': 'DrawRect',
        'x': 0,
        'y': 0,
        'w': 10,
        'h': 10,
        'fill': 0xFF112233,
      },
    ]);
    final recorder = PictureRecorder();
    painter.paint(Canvas(recorder), const Size(20, 20));
    expect(painter.ops, contains('DrawRect'));
    expect(painter.skipped, isNot(contains('emptyStyle')));
  });

  test('T03 stroked rect still strokes', () {
    final painter = RenderPainter([
      <String, dynamic>{
        'type': 'DrawRect',
        'x': 0,
        'y': 0,
        'w': 10,
        'h': 10,
        'stroke': 0xFF0000FF,
      },
    ]);
    final recorder = PictureRecorder();
    painter.paint(Canvas(recorder), const Size(20, 20));
    expect(painter.ops, contains('DrawRect'));
    expect(painter.skipped, isNot(contains('emptyStyle')));
  });
}
