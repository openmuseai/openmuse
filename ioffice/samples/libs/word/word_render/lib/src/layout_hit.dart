import 'dart:ui' show Offset;

import 'render_command.dart';

/// Local page coordinates in twip (origin = paper top-left).
class PageHit {
  const PageHit({
    required this.pageIndex,
    required this.xTwip,
    required this.yTwip,
  });

  final int pageIndex;
  final int xTwip;
  final int yTwip;
}

/// Visual caret snapped to a nearby run (kernel hit-test is not on FFI yet).
class WordCaret {
  const WordCaret({
    required this.pageIndex,
    required this.xTwip,
    required this.yTwip,
    required this.heightTwip,
  });

  final int pageIndex;
  final int xTwip;
  final int yTwip;
  final int heightTwip;
}

/// Convert a pointer on a page widget (size = paper in px) to twip.
PageHit? hitTestPage({
  required int pageIndex,
  required Offset localPx,
  required double zoom,
  required PageMetrics page,
}) {
  final w = twipToPx(page.pageWidthTwip, zoom);
  final h = twipToPx(page.pageHeightTwip, zoom);
  if (localPx.dx < 0 || localPx.dy < 0 || localPx.dx > w || localPx.dy > h) {
    return null;
  }
  return PageHit(
    pageIndex: pageIndex,
    xTwip: pxToTwip(localPx.dx, zoom).round(),
    yTwip: pxToTwip(localPx.dy, zoom).round(),
  );
}

/// Snap a page tap to the nearest laid-out run (baseline + glyph xpos).
WordCaret? caretFromTap(RenderEnvelope envelope, PageHit hit) {
  WcDrawText? best;
  var bestDy = 1 << 30;
  for (final cmd in envelope.commands) {
    if (cmd is! WcDrawText || cmd.page != hit.pageIndex) continue;
    final dy = (cmd.y - hit.yTwip).abs();
    if (dy < bestDy) {
      bestDy = dy;
      best = cmd;
    }
  }
  if (best == null) {
    return WordCaret(
      pageIndex: hit.pageIndex,
      xTwip: hit.xTwip,
      yTwip: hit.yTwip,
      heightTwip: 240,
    );
  }
  var x = best.x;
  if (best.xpos.isNotEmpty) {
    var nearest = best.xpos.first;
    var bestDx = (nearest - hit.xTwip).abs();
    for (final gx in best.xpos) {
      final dx = (gx - hit.xTwip).abs();
      if (dx < bestDx) {
        bestDx = dx;
        nearest = gx;
      }
    }
    x = nearest;
  }
  final h = best.sizeTwip <= 0 ? 240 : best.sizeTwip;
  return WordCaret(
    pageIndex: hit.pageIndex,
    xTwip: x,
    yTwip: best.y,
    heightTwip: h,
  );
}

/// Map visible DrawText scalar cp → caret bar (same order as kernel overlay).
WordCaret? caretFromVisibleCp(RenderEnvelope envelope, int cp) {
  var origin = 0;
  for (final cmd in envelope.commands) {
    if (cmd is! WcDrawText) continue;
    final n = cmd.text.runes.length;
    if (cp >= origin && cp <= origin + n) {
      final i = cp - origin;
      int x = cmd.x;
      if (cmd.xpos.isNotEmpty) {
        if (i < cmd.xpos.length) {
          x = cmd.xpos[i];
        } else if (cmd.xpos.length >= 2) {
          final last = cmd.xpos.last;
          final prev = cmd.xpos[cmd.xpos.length - 2];
          x = last + (last - prev);
        } else {
          x = cmd.xpos.last;
        }
      }
      final h = cmd.sizeTwip <= 0 ? 240 : cmd.sizeTwip;
      return WordCaret(
        pageIndex: cmd.page,
        xTwip: x,
        yTwip: cmd.y,
        heightTwip: h,
      );
    }
    origin += n;
  }
  return null;
}
