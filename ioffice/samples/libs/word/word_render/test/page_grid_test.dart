import 'dart:ui' as ui;

import 'package:flutter_test/flutter_test.dart';
import 'package:word_render/word_render.dart';

void main() {
  test('dark page remaps authored strokes without changing render commands',
      () async {
    final line = WcDrawLine(
      page: 0,
      x1: 300,
      y1: 300,
      x2: 900,
      y2: 300,
      kind: 'test',
      color: 0xFF000000,
      width: 30,
    );
    final envelope = RenderEnvelope(
      ok: true,
      pipeline: 'test',
      page: PageMetrics(
        pageCount: 1,
        pageWidthTwip: 1440,
        pageHeightTwip: 1440,
        marginLeftTwip: 0,
        marginRightTwip: 0,
        marginTopTwip: 0,
        marginBottomTwip: 0,
        paperSize: 'test',
      ),
      commands: [line],
    );
    Future<List<int>> pixel(bool dark, int x, int y) async {
      final recorder = ui.PictureRecorder();
      PagePainter(envelope: envelope, pageIndex: 0, zoom: 1, darkPage: dark)
          .paint(ui.Canvas(recorder), const ui.Size(96, 96));
      final picture = recorder.endRecording();
      final image = await picture.toImage(96, 96);
      final bytes =
          (await image.toByteData(format: ui.ImageByteFormat.rawRgba))!
              .buffer
              .asUint8List();
      final offset = (y * 96 + x) * 4;
      final rgba = bytes.sublist(offset, offset + 4);
      image.dispose();
      picture.dispose();
      return rgba;
    }

    expect(await pixel(false, 5, 5), [255, 255, 255, 255]);
    expect(await pixel(true, 5, 5), [30, 30, 30, 255]);
    expect((await pixel(false, 30, 20))[0], lessThan(30));
    expect((await pixel(true, 30, 20))[0], greaterThan(240));
    expect(line.color, 0xFF000000);
  });

  test('gridlines stay inside word-core page margins', () async {
    final envelope = RenderEnvelope(
      ok: true,
      pipeline: 'test',
      page: PageMetrics(
        pageCount: 1,
        pageWidthTwip: 1440,
        pageHeightTwip: 1440,
        marginLeftTwip: 240,
        marginRightTwip: 240,
        marginTopTwip: 240,
        marginBottomTwip: 240,
        paperSize: 'test',
      ),
      commands: const [],
    );
    Future<List<int>> render(bool gridlines) async {
      final recorder = ui.PictureRecorder();
      PagePainter(
        envelope: envelope,
        pageIndex: 0,
        zoom: 1,
        showGridlines: gridlines,
      ).paint(ui.Canvas(recorder), const ui.Size(96, 96));
      final picture = recorder.endRecording();
      final image = await picture.toImage(96, 96);
      final pixels =
          (await image.toByteData(format: ui.ImageByteFormat.rawRgba))!
              .buffer
              .asUint8List();
      image.dispose();
      picture.dispose();
      return pixels;
    }

    final plain = await render(false);
    final grid = await render(true);
    int red(List<int> pixels, int x, int y) => pixels[(y * 96 + x) * 4];
    expect(red(plain, 16, 30), 255);
    expect(red(grid, 16, 30), lessThan(255));
    expect(red(grid, 10, 30), 255);
    expect(red(grid, 30, 30), 255);
  });
}
