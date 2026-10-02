import 'dart:math' as math;

import 'package:flutter/material.dart';

enum WbGlyph {
  menu,
  desktop,
  chevron,
  chevronDown,
  plus,
  voice,
  bubble,
  bubblePlus,
  expert,
  library,
  alarm,
  project,
  folder,
  folderPlus,
  personPlus,
  close,
  check,
  diamond,
}

final class WbIcon extends StatelessWidget {
  const WbIcon(this.glyph, {super.key, this.size = 22, this.color = Colors.white});

  final WbGlyph glyph;
  final double size;
  final Color color;

  @override
  Widget build(BuildContext context) => CustomPaint(
    size: Size.square(size),
    painter: _GlyphPainter(glyph, color),
  );
}

final class _GlyphPainter extends CustomPainter {
  _GlyphPainter(this.glyph, this.color);

  final WbGlyph glyph;
  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = color
      ..style = PaintingStyle.stroke
      ..strokeWidth = math.max(1.4, size.width * 0.075)
      ..strokeCap = StrokeCap.round
      ..strokeJoin = StrokeJoin.round;
    final fill = Paint()
      ..color = color
      ..style = PaintingStyle.fill;
    final w = size.width;
    final h = size.height;
    switch (glyph) {
      case WbGlyph.menu:
        for (final dy in [0.28, 0.5, 0.72]) {
          canvas.drawLine(Offset(w * 0.18, h * dy), Offset(w * 0.82, h * dy), paint);
        }
      case WbGlyph.desktop:
        final rect = RRect.fromRectAndRadius(
          Rect.fromLTWH(w * 0.12, h * 0.16, w * 0.76, h * 0.52),
          Radius.circular(w * 0.08),
        );
        canvas.drawRRect(rect, paint);
        canvas.drawLine(Offset(w * 0.5, h * 0.68), Offset(w * 0.5, h * 0.82), paint);
        canvas.drawLine(Offset(w * 0.32, h * 0.82), Offset(w * 0.68, h * 0.82), paint);
      case WbGlyph.chevron:
        final path = Path()
          ..moveTo(w * 0.34, h * 0.22)
          ..lineTo(w * 0.66, h * 0.5)
          ..lineTo(w * 0.34, h * 0.78);
        canvas.drawPath(path, paint);
      case WbGlyph.chevronDown:
        final path = Path()
          ..moveTo(w * 0.22, h * 0.36)
          ..lineTo(w * 0.5, h * 0.68)
          ..lineTo(w * 0.78, h * 0.36);
        canvas.drawPath(path, paint);
      case WbGlyph.plus:
        canvas.drawLine(Offset(w * 0.5, h * 0.18), Offset(w * 0.5, h * 0.82), paint);
        canvas.drawLine(Offset(w * 0.18, h * 0.5), Offset(w * 0.82, h * 0.5), paint);
      case WbGlyph.voice:
        canvas.drawCircle(Offset(w * 0.5, h * 0.5), w * 0.34, paint);
        final bars = Paint()
          ..color = color
          ..strokeWidth = math.max(1.3, w * 0.07)
          ..strokeCap = StrokeCap.round;
        for (var i = 0; i < 3; i++) {
          final x = w * (0.36 + i * 0.14);
          final bar = 0.16 + (i == 1 ? 0.18 : 0.08);
          canvas.drawLine(Offset(x, h * (0.5 - bar)), Offset(x, h * (0.5 + bar)), bars);
        }
      case WbGlyph.bubble:
        _bubble(canvas, size, fill, tail: true);
      case WbGlyph.bubblePlus:
        final bubble = RRect.fromRectAndRadius(
          Rect.fromLTWH(w * 0.08, h * 0.16, w * 0.7, h * 0.52),
          Radius.circular(w * 0.22),
        );
        canvas.drawRRect(bubble, paint);
        final tail = Path()
          ..moveTo(w * 0.22, h * 0.62)
          ..lineTo(w * 0.16, h * 0.82)
          ..lineTo(w * 0.42, h * 0.64);
        canvas.drawPath(tail, paint);
        final plus = Paint()
          ..color = color
          ..strokeWidth = math.max(1.5, w * 0.07)
          ..strokeCap = StrokeCap.round;
        canvas.drawLine(Offset(w * 0.72, h * 0.42), Offset(w * 0.72, h * 0.78), plus);
        canvas.drawLine(Offset(w * 0.54, h * 0.6), Offset(w * 0.9, h * 0.6), plus);
      case WbGlyph.expert:
        canvas.drawCircle(Offset(w * 0.5, h * 0.46), w * 0.3, paint);
        canvas.drawCircle(Offset(w * 0.4, h * 0.42), w * 0.045, fill);
        canvas.drawCircle(Offset(w * 0.6, h * 0.42), w * 0.045, fill);
        canvas.drawArc(
          Rect.fromCircle(center: Offset(w * 0.5, h * 0.48), radius: w * 0.12),
          0.2,
          2.6,
          false,
          paint,
        );
        canvas.drawLine(Offset(w * 0.28, h * 0.22), Offset(w * 0.22, h * 0.08), paint);
        canvas.drawCircle(Offset(w * 0.22, h * 0.08), w * 0.045, fill);
      case WbGlyph.library:
        final page = RRect.fromRectAndRadius(
          Rect.fromLTWH(w * 0.22, h * 0.14, w * 0.56, h * 0.72),
          Radius.circular(w * 0.08),
        );
        canvas.drawRRect(page, paint);
        canvas.drawLine(Offset(w * 0.36, h * 0.38), Offset(w * 0.64, h * 0.38), paint);
        canvas.drawLine(Offset(w * 0.36, h * 0.54), Offset(w * 0.64, h * 0.54), paint);
      case WbGlyph.alarm:
        canvas.drawCircle(Offset(w * 0.5, h * 0.56), w * 0.28, paint);
        canvas.drawLine(Offset(w * 0.5, h * 0.56), Offset(w * 0.5, h * 0.4), paint);
        canvas.drawLine(Offset(w * 0.5, h * 0.56), Offset(w * 0.64, h * 0.62), paint);
        canvas.drawLine(Offset(w * 0.22, h * 0.22), Offset(w * 0.34, h * 0.34), paint);
        canvas.drawLine(Offset(w * 0.78, h * 0.22), Offset(w * 0.66, h * 0.34), paint);
      case WbGlyph.project:
        canvas.drawCircle(Offset(w * 0.28, h * 0.32), w * 0.1, paint);
        canvas.drawCircle(Offset(w * 0.74, h * 0.3), w * 0.1, paint);
        canvas.drawCircle(Offset(w * 0.5, h * 0.74), w * 0.1, paint);
        canvas.drawLine(Offset(w * 0.36, h * 0.36), Offset(w * 0.66, h * 0.34), paint);
        canvas.drawLine(Offset(w * 0.34, h * 0.4), Offset(w * 0.46, h * 0.66), paint);
        canvas.drawLine(Offset(w * 0.66, h * 0.38), Offset(w * 0.56, h * 0.66), paint);
      case WbGlyph.folder:
        final folder = Path()
          ..moveTo(w * 0.14, h * 0.32)
          ..lineTo(w * 0.14, h * 0.78)
          ..lineTo(w * 0.86, h * 0.78)
          ..lineTo(w * 0.86, h * 0.4)
          ..lineTo(w * 0.48, h * 0.4)
          ..lineTo(w * 0.4, h * 0.28)
          ..lineTo(w * 0.14, h * 0.28)
          ..close();
        canvas.drawPath(folder, paint);
      case WbGlyph.folderPlus:
        // unused stroke variant; folder is enough
        break;
      case WbGlyph.personPlus:
        canvas.drawCircle(Offset(w * 0.4, h * 0.32), w * 0.14, paint);
        canvas.drawArc(
          Rect.fromCircle(center: Offset(w * 0.4, h * 0.78), radius: w * 0.24),
          math.pi,
          math.pi,
          false,
          paint,
        );
        canvas.drawLine(Offset(w * 0.72, h * 0.28), Offset(w * 0.72, h * 0.52), paint);
        canvas.drawLine(Offset(w * 0.6, h * 0.4), Offset(w * 0.84, h * 0.4), paint);
      case WbGlyph.close:
        canvas.drawLine(Offset(w * 0.28, h * 0.28), Offset(w * 0.72, h * 0.72), paint);
        canvas.drawLine(Offset(w * 0.72, h * 0.28), Offset(w * 0.28, h * 0.72), paint);
      case WbGlyph.check:
        final box = RRect.fromRectAndRadius(
          Rect.fromLTWH(w * 0.16, h * 0.16, w * 0.68, h * 0.68),
          Radius.circular(w * 0.12),
        );
        canvas.drawRRect(box, paint);
        final tick = Path()
          ..moveTo(w * 0.32, h * 0.52)
          ..lineTo(w * 0.46, h * 0.66)
          ..lineTo(w * 0.7, h * 0.36);
        canvas.drawPath(tick, paint);
      case WbGlyph.diamond:
        final path = Path()
          ..moveTo(w * 0.5, h * 0.12)
          ..lineTo(w * 0.86, h * 0.5)
          ..lineTo(w * 0.5, h * 0.88)
          ..lineTo(w * 0.14, h * 0.5)
          ..close();
        canvas.drawPath(path, paint);
    }
  }

  void _bubble(Canvas canvas, Size size, Paint paint, {required bool tail}) {
    final w = size.width;
    final h = size.height;
    final rect = RRect.fromRectAndRadius(
      Rect.fromLTWH(w * 0.12, h * 0.12, w * 0.76, h * 0.58),
      Radius.circular(w * 0.18),
    );
    if (paint.style == PaintingStyle.fill) {
      canvas.drawRRect(rect, paint);
      if (tail) {
        final path = Path()
          ..moveTo(w * 0.28, h * 0.62)
          ..lineTo(w * 0.22, h * 0.86)
          ..lineTo(w * 0.48, h * 0.66)
          ..close();
        canvas.drawPath(path, paint);
      }
    } else {
      canvas.drawRRect(rect, paint);
    }
  }

  @override
  bool shouldRepaint(covariant _GlyphPainter oldDelegate) =>
      oldDelegate.glyph != glyph || oldDelegate.color != color;
}
