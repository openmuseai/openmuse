import 'dart:ui';

import 'package:flutter_test/flutter_test.dart';
import 'package:ppt_render/ppt_render.dart';

void paint(RenderPainter p, {Size size = const Size(960, 540)}) {
  final rec = PictureRecorder();
  p.paint(Canvas(rec), size);
}

void main() {
  test('T01 DrawRect paints a rectangle', () {
    final p = RenderPainter.fromRoot([
      {
        'type': 'DrawRect',
        'x': 0,
        'y': 0,
        'w': 10,
        'h': 20,
        'fill': 0xFF4472C4,
      }
    ]);
    paint(p);
    expect(p.ops, contains('DrawRect'));
    expect(p.skipped, isEmpty);
  });

  test('T02 DrawPath paints a path', () {
    final p = RenderPainter.fromRoot([
      {
        'type': 'DrawPath',
        'path': [
          [0, 0],
          [10, 0],
          [10, 10],
        ],
        'stroke': 0xFF000000,
      }
    ]);
    paint(p);
    expect(p.ops, contains('DrawPath'));
  });

  test('T03 DrawText paints Hello', () {
    final p = RenderPainter.fromRoot([
      {
        'type': 'DrawText',
        'text': 'Hello',
        'x': 8,
        'y': 12,
        'size': 18,
      }
    ]);
    paint(p);
    expect(p.ops.single, 'DrawText:Hello');
  });

  test('T04 unknown variant skip+log, rest still paint', () {
    final p = RenderPainter.fromRoot([
      {'type': 'DrawRect', 'x': 0, 'y': 0, 'w': 1, 'h': 1, 'fill': 0xFFFFFFFF},
      {'type': 'NotACommand', 'x': 0},
      {'type': 'DrawText', 'text': 'Hello', 'x': 0, 'y': 0},
    ]);
    paint(p);
    expect(p.skipped, contains('NotACommand'));
    expect(p.ops, contains('DrawRect'));
    expect(p.ops, contains('DrawText:Hello'));
  });

  test('envelope parses pipeline JSON', () {
    const raw =
        '{"ok":true,"pipeline":"t","width":960,"height":540,"commands":[{"type":"DrawText","text":"Hi"}]}';
    final env = RenderEnvelope.fromJsonString(raw);
    expect(env.ok, isTrue);
    expect(env.commands.single['text'], 'Hi');
  });
}
