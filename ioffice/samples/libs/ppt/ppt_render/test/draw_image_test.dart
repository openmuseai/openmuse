import 'dart:ui';

import 'package:flutter_test/flutter_test.dart';
import 'package:ppt_render/ppt_render.dart';

void paint(RenderPainter p, {Size size = const Size(960, 540)}) {
  final rec = PictureRecorder();
  p.paint(Canvas(rec), size);
}

void main() {
  test('T01 DrawImage is painted not skipped', () {
    final p = RenderPainter.fromRoot([
      {
        'type': 'DrawImage',
        'x': 0,
        'y': 0,
        'w': 10,
        'h': 20,
        'media_id': 1,
        'bytes_b64': 'AAAA',
      }
    ]);
    paint(p);
    expect(p.ops, contains('DrawImage'));
    expect(p.skipped, isEmpty);
  });

  test('T02 DrawImage without bytes still records op', () {
    final p = RenderPainter.fromRoot([
      {
        'type': 'DrawImage',
        'x': 1,
        'y': 2,
        'w': 3,
        'h': 4,
        'media_id': 0,
      }
    ]);
    paint(p);
    expect(p.ops, contains('DrawImage'));
  });

  test('T03 unknown type still skipped beside DrawImage', () {
    final p = RenderPainter.fromRoot([
      {'type': 'DrawImage', 'x': 0, 'y': 0, 'w': 1, 'h': 1},
      {'type': 'NotACommand'},
    ]);
    paint(p);
    expect(p.ops, contains('DrawImage'));
    expect(p.skipped, contains('NotACommand'));
  });
}
