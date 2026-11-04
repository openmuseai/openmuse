import 'dart:ui' as ui;
import 'dart:math' as math;

import 'package:flutter/foundation.dart';
import 'package:flutter/painting.dart';

import 'render_command.dart';
import 'dark_page_color.dart';

/// Default face registered via `loadFontFromList` (same TTF bytes as the kernel).
const kWcDefaultFontFamily = 'WcDefault';

String paintFontFamily(String fontId) {
  if (fontId.isEmpty || fontId == 'default') {
    return kWcDefaultFontFamily;
  }
  return fontId;
}

/// Paint in **twip** after the caller has `canvas.scale(zoom * 1.3333536/20)`.
/// One glyph per `xpos` entry — do not layout the whole run (would re-kern).
void executeCommands(
  ui.Canvas canvas,
  List<WcRenderCommand> commands, {
  required int page,
  bool darkPage = false,
}) {
  // One page can contain hundreds of repeated CJK glyphs. Layout each unique
  // glyph/style once for this paint, while retaining each command's xpos.
  final glyphs = <(String, String, double, bool, int), _GlyphLayout>{};
  try {
    for (final cmd in commands) {
      if (cmd.page != page) continue;
      if (cmd is WcDrawText) {
        _drawText(canvas, cmd, glyphs, darkPage);
      } else if (cmd is WcDrawRect) {
        final paint = ui.Paint()
          ..color = _screenColor(
              Color(cmd.color == 0 ? 0x22000000 : cmd.color), darkPage)
          ..style = ui.PaintingStyle.fill;
        canvas.drawRect(
          ui.Rect.fromLTWH(
            cmd.x.toDouble(),
            cmd.y.toDouble(),
            cmd.w.toDouble(),
            cmd.h.toDouble(),
          ),
          paint,
        );
      } else if (cmd is WcDrawLine) {
        final paint = ui.Paint()
          ..color = _screenColor(
              Color(cmd.color == 0 ? 0xFF000000 : cmd.color), darkPage)
          ..strokeWidth = (cmd.width <= 0 ? 15 : cmd.width).toDouble()
          ..style = ui.PaintingStyle.stroke;
        canvas.drawLine(
          ui.Offset(cmd.x1.toDouble(), cmd.y1.toDouble()),
          ui.Offset(cmd.x2.toDouble(), cmd.y2.toDouble()),
          paint,
        );
      } else if (cmd is WcDrawImage) {
        _drawImage(canvas, cmd);
      }
    }
  } finally {
    for (final layout in glyphs.values) {
      layout.painter.dispose();
    }
  }
}

Color _screenColor(Color authored, bool darkPage) =>
    darkPage ? darkPageColor(authored) : authored;

void _drawImage(ui.Canvas canvas, WcDrawImage cmd) {
  final image = cmd.decoded;
  final wmf = cmd.wmfPreview;
  if ((image == null && wmf == null) || cmd.w <= 0 || cmd.h <= 0) return;
  final left = cmd.cropLeft.clamp(0.0, 1.0);
  final top = cmd.cropTop.clamp(0.0, 1.0);
  final right = (1.0 - cmd.cropRight).clamp(left, 1.0);
  final bottom = (1.0 - cmd.cropBottom).clamp(top, 1.0);
  final target = ui.Rect.fromLTWH(
    cmd.x.toDouble(),
    cmd.y.toDouble(),
    cmd.w.toDouble(),
    cmd.h.toDouble(),
  );
  canvas.save();
  if (cmd.rotation != 0) {
    canvas.translate(target.center.dx, target.center.dy);
    canvas.rotate(cmd.rotation * math.pi / 180.0);
    canvas.translate(-target.center.dx, -target.center.dy);
  }
  if (wmf != null) {
    wmf.paint(canvas, target,
        cropLeft: left,
        cropTop: top,
        cropRight: 1 - right,
        cropBottom: 1 - bottom);
  } else if (image != null) {
    final source = ui.Rect.fromLTRB(
      left * image.width,
      top * image.height,
      right * image.width,
      bottom * image.height,
    );
    canvas.drawImageRect(image, source, target,
        ui.Paint()..filterQuality = ui.FilterQuality.medium);
  }
  canvas.restore();
}

class _GlyphLayout {
  const _GlyphLayout(this.painter, this.baseline);

  final TextPainter painter;
  final double baseline;
}

List<String> visibleGlyphs(String text) {
  final out = <String>[];
  for (final ch in text.runes) {
    if (ch == 0x0d || ch == 0x0a || ch == 0x0c) continue;
    out.add(String.fromCharCode(ch));
  }
  return out;
}

void _drawText(
    ui.Canvas canvas,
    WcDrawText cmd,
    Map<(String, String, double, bool, int), _GlyphLayout> cache,
    bool darkPage) {
  assert(() {
    assert(
      paintFontFamily(cmd.fontId) == kWcDefaultFontFamily ||
          cmd.fontId.isNotEmpty,
      'DrawText.font_id=${cmd.fontId} has no mapped face; do not fall back to sans-serif',
    );
    return true;
  }());

  final family = paintFontFamily(cmd.fontId);
  final glyphs = visibleGlyphs(cmd.text);
  final size = (cmd.sizeTwip <= 0 ? 240 : cmd.sizeTwip).toDouble();
  final useXpos = cmd.xpos.length == glyphs.length && glyphs.isNotEmpty;

  if (!useXpos) {
    if (kDebugMode && cmd.xpos.isEmpty && glyphs.isNotEmpty) {
      debugPrint(
          'P3: DrawText.xpos missing; whole-run fallback for "${cmd.text}"');
    }
    _paintGlyph(
        canvas, cmd, family, size, cmd.text, cmd.x.toDouble(), cache, darkPage);
    return;
  }

  for (var i = 0; i < glyphs.length; i++) {
    _paintGlyph(canvas, cmd, family, size, glyphs[i], cmd.xpos[i].toDouble(),
        cache, darkPage);
  }
}

void _paintGlyph(
  ui.Canvas canvas,
  WcDrawText cmd,
  String family,
  double fontSize,
  String text,
  double x,
  Map<(String, String, double, bool, int), _GlyphLayout> cache,
  bool darkPage,
) {
  final color = _screenColor(cmd.flutterColor, darkPage);
  final key = (text, family, fontSize, cmd.bold, color.toARGB32());
  final layout = cache.putIfAbsent(key, () {
    // Synthetic bold keeps the same face as kernel xpos (CJKFallback / WcDefault).
    final tp = TextPainter(
      text: TextSpan(
        text: text,
        style: TextStyle(
          fontFamily: family,
          fontSize: fontSize,
          fontWeight: cmd.bold ? FontWeight.w700 : FontWeight.w400,
          color: color,
          decoration: TextDecoration.none,
        ),
      ),
      textDirection: TextDirection.ltr,
      maxLines: 1,
    );
    tp.layout(minWidth: 0, maxWidth: double.infinity);
    return _GlyphLayout(
      tp,
      tp.computeDistanceToActualBaseline(TextBaseline.alphabetic),
    );
  });
  final origin = ui.Offset(x, cmd.y.toDouble() - layout.baseline);
  if (cmd.italic) {
    canvas.save();
    canvas.translate(origin.dx, origin.dy);
    canvas.skew(-0.25, 0);
    layout.painter.paint(canvas, ui.Offset.zero);
    canvas.restore();
  } else {
    layout.painter.paint(canvas, origin);
  }
}
