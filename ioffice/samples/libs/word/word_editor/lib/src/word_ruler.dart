import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:word_render/word_render.dart' as wr;

/// View-only ruler geometry. All page dimensions come from word-core's layout.
enum WordRulerUnit { centimeters, inches }

class WordRulerTick {
  const WordRulerTick(this.positionTwip, this.kind, [this.label]);
  final double positionTwip;
  final WordRulerTickKind kind;
  final String? label;
}

enum WordRulerTickKind { number, middle, minor }

class WordRulerGeometry {
  WordRulerGeometry(wr.PageMetrics page, this.unit)
      : pageWidthTwip = page.pageWidthTwip.clamp(0, 72000).toInt(),
        marginLeftTwip = page.marginLeftTwip.clamp(0, 72000).toInt(),
        marginRightTwip = page.marginRightTwip.clamp(0, 72000).toInt();

  final int pageWidthTwip;
  final int marginLeftTwip;
  final int marginRightTwip;
  final WordRulerUnit unit;

  double get width => pageWidthTwip * 96 / 1440;
  double get left =>
      math.min(marginLeftTwip.toDouble(), pageWidthTwip.toDouble());
  double get right =>
      math.min(marginRightTwip.toDouble(), pageWidthTwip.toDouble());

  Iterable<WordRulerTick> get ticks sync* {
    final minor = unit == WordRulerUnit.centimeters ? 360 / 2.54 : 180.0;
    final majorSteps = unit == WordRulerUnit.centimeters ? 4 : 8;
    final middleSteps = majorSteps ~/ 2;
    final origin = left;
    for (final direction in [1, -1]) {
      for (var step = 1; step <= 500; step++) {
        final position = origin + direction * step * minor;
        if (position < -0.5 || position > pageWidthTwip + 0.5) break;
        if (step % majorSteps == 0) {
          yield WordRulerTick(
              position, WordRulerTickKind.number, '${step ~/ majorSteps}');
        } else {
          yield WordRulerTick(
              position,
              step % middleSteps == 0
                  ? WordRulerTickKind.middle
                  : WordRulerTickKind.minor);
        }
      }
    }
  }

  Iterable<double> get defaultTabStops sync* {
    final bodyEnd = math.max(left, pageWidthTwip - right);
    for (double position = left + 720; position < bodyEnd; position += 720) {
      yield position;
    }
  }
}

/// GenOffice's 20px horizontal ruler plus 6px gap, zoomed with the page.
class WordRuler extends StatelessWidget {
  const WordRuler({super.key, required this.page, required this.zoom});

  final wr.PageMetrics page;
  final double zoom;

  @override
  Widget build(BuildContext context) {
    final locale = Localizations.localeOf(context);
    final inchRegion = const {'US', 'LR', 'MM'}.contains(locale.countryCode) ||
        (locale.countryCode == null && locale.languageCode == 'en');
    final geometry = WordRulerGeometry(
        page, inchRegion ? WordRulerUnit.inches : WordRulerUnit.centimeters);
    final scale = zoom.clamp(0.1, 5.0).toDouble();
    final fontFamily = DefaultTextStyle.of(context).style.fontFamily ??
        Theme.of(context).textTheme.bodyMedium?.fontFamily;
    return Semantics(
      label: '标尺',
      child: SizedBox(
        key: const Key('word-page-ruler'),
        width: geometry.width * scale,
        height: 26 * scale,
        child: CustomPaint(
            painter: _WordRulerPainter(geometry, scale, fontFamily)),
      ),
    );
  }
}

class _WordRulerPainter extends CustomPainter {
  const _WordRulerPainter(this.geometry, this.zoom, this.fontFamily);
  final WordRulerGeometry geometry;
  final double zoom;
  final String? fontFamily;

  double px(double twips) => twips * 96 / 1440;

  @override
  void paint(Canvas canvas, Size size) {
    canvas.save();
    canvas.scale(zoom);
    final width = geometry.width;
    final body = Rect.fromLTWH(0, 0, width, 20);
    canvas.drawRect(body, Paint()..color = const Color(0xFFF3F2F1));
    final zone = Paint()..color = const Color(0x14000000);
    canvas.drawRect(Rect.fromLTWH(0, 0, px(geometry.left), 20), zone);
    canvas.drawRect(
        Rect.fromLTWH(width - px(geometry.right), 0, px(geometry.right), 20),
        zone);
    final stroke = Paint()
      ..color = const Color(0xFFD6D6D6)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1;
    canvas.drawRRect(
        RRect.fromRectAndRadius(body.deflate(0.5), const Radius.circular(2)),
        stroke);
    final tickPaint = Paint()
      ..color = const Color(0xFFB9B9B9)
      ..strokeWidth = 1;
    for (final tick in geometry.ticks) {
      final x = px(tick.positionTwip);
      if (tick.kind == WordRulerTickKind.number) {
        final label = TextPainter(
          text: TextSpan(
              text: tick.label,
              style: TextStyle(
                  fontSize: 9,
                  fontFamily: fontFamily,
                  color: const Color(0xFF666666))),
          textDirection: TextDirection.ltr,
        )..layout();
        label.paint(canvas, Offset(x - label.width / 2, 3));
        label.dispose();
      } else {
        final height = tick.kind == WordRulerTickKind.middle ? 6.0 : 3.0;
        canvas.drawLine(Offset(x, 20 - height), Offset(x, 20), tickPaint);
      }
    }
    final guide = Paint()
      ..color = const Color(0xFFAAAAAA)
      ..strokeWidth = 1;
    for (final position in geometry.defaultTabStops) {
      final x = px(position);
      canvas.drawLine(Offset(x, 16), Offset(x, 20), guide);
    }
    canvas.drawRect(const Rect.fromLTWH(0, 0, 20, 20),
        Paint()..color = const Color(0xFFE0E0E0));
    canvas.drawLine(const Offset(20, 0), const Offset(20, 20), stroke);
    final tab = TextPainter(
      text: TextSpan(
          text: 'L',
          style: TextStyle(
              fontSize: 9,
              fontFamily: fontFamily,
              fontWeight: FontWeight.bold,
              color: const Color(0xFF333333))),
      textDirection: TextDirection.ltr,
    )..layout();
    tab.paint(canvas, Offset(10 - tab.width / 2, 10 - tab.height / 2));
    tab.dispose();
    canvas.restore();
  }

  @override
  bool shouldRepaint(_WordRulerPainter old) =>
      old.zoom != zoom ||
      old.geometry.pageWidthTwip != geometry.pageWidthTwip ||
      old.geometry.marginLeftTwip != geometry.marginLeftTwip ||
      old.geometry.marginRightTwip != geometry.marginRightTwip ||
      old.geometry.unit != geometry.unit ||
      old.fontFamily != fontFamily;
}
