import 'dart:math' as math;

import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';

/// Maps scroll and line coordinates across dual text surfaces.
///
/// Transfer is defined by monotonic [MuseSimilarBoundary] pairs. The 1/3
/// viewport line is the sync anchor, matching IntelliJ's paired scrolling.
final class MuseTextDiffSync {
  const MuseTextDiffSync._();

  static List<double> prefix(List<double> extents) {
    final result = <double>[0];
    for (final extent in extents) {
      result.add(result.last + extent);
    }
    return result;
  }

  static int indexAtOffset(List<double> prefix, double offset) {
    if (prefix.length < 2) return 0;
    final y = offset.clamp(prefix.first, prefix.last);
    for (var index = 0; index < prefix.length - 1; index++) {
      if (y < prefix[index + 1] || index == prefix.length - 2) return index;
    }
    return prefix.length - 2;
  }

  /// Zero-based continuous line on the destination side.
  static double transferLine({
    required List<MuseSimilarBoundary> boundaries,
    required double sourceLine,
    required bool fromLeft,
  }) {
    if (boundaries.length < 2) return sourceLine;
    for (var index = 0; index < boundaries.length - 1; index++) {
      final start = boundaries[index];
      final end = boundaries[index + 1];
      final sourceStart = (fromLeft ? start.oldLine : start.newLine).toDouble();
      final sourceEnd = (fromLeft ? end.oldLine : end.newLine).toDouble();
      final targetStart = (fromLeft ? start.newLine : start.oldLine).toDouble();
      final targetEnd = (fromLeft ? end.newLine : end.oldLine).toDouble();
      final last = index == boundaries.length - 2;
      if (sourceLine <= sourceEnd || last) {
        if (sourceEnd == sourceStart) return targetStart;
        final t = ((sourceLine - sourceStart) / (sourceEnd - sourceStart))
            .clamp(0.0, 1.0);
        return targetStart + t * (targetEnd - targetStart);
      }
    }
    return sourceLine;
  }

  static double mapOffset({
    required double sourceOffset,
    required double viewportHeight,
    required List<double> sourcePrefix,
    required List<double> targetPrefix,
    required List<double?> sourceLines,
    required List<double?> targetLines,
    required List<MuseSimilarBoundary> boundaries,
    required bool fromLeft,
  }) {
    if (sourcePrefix.length < 2 || targetPrefix.length < 2) {
      return sourceOffset;
    }
    final maxTarget = math.max(0.0, targetPrefix.last - viewportHeight);
    if (_prefixesAlign(sourcePrefix, targetPrefix)) {
      return sourceOffset.clamp(0.0, maxTarget);
    }
    final anchor =
        sourceOffset.clamp(0.0, sourcePrefix.last) + viewportHeight / 3.0;
    final row = indexAtOffset(sourcePrefix, anchor);
    final rowStart = sourcePrefix[row];
    final rowEnd = sourcePrefix[row + 1];
    final phase = rowEnd > rowStart
        ? ((anchor - rowStart) / (rowEnd - rowStart)).clamp(0.0, 1.0)
        : 0.0;
    final int targetRow;
    if (boundaries.length < 2) {
      targetRow = row.clamp(0, targetPrefix.length - 2).toInt();
    } else {
      final line = sourceLines[row] ?? row.toDouble();
      final transferred = transferLine(
        boundaries: boundaries,
        sourceLine: line,
        fromLeft: fromLeft,
      );
      targetRow = rowForLine(lines: targetLines, line: transferred)
          .clamp(0, targetPrefix.length - 2)
          .toInt();
    }
    final targetStart = targetPrefix[targetRow];
    final targetEnd = targetPrefix[targetRow + 1];
    final targetAnchor = targetStart + phase * (targetEnd - targetStart);
    return (targetAnchor - viewportHeight / 3.0).clamp(0.0, maxTarget);
  }

  static int rowForLine({
    required List<double?> lines,
    required double line,
  }) {
    var best = 0;
    var bestDelta = double.infinity;
    for (var index = 0; index < lines.length; index++) {
      final value = lines[index];
      if (value == null) continue;
      final delta = (value - line).abs();
      if (delta < bestDelta) {
        bestDelta = delta;
        best = index;
      }
    }
    return best;
  }

  static bool isMonotonic(List<MuseSimilarBoundary> boundaries) {
    for (var index = 1; index < boundaries.length; index++) {
      if (boundaries[index].oldLine < boundaries[index - 1].oldLine) {
        return false;
      }
      if (boundaries[index].newLine < boundaries[index - 1].newLine) {
        return false;
      }
    }
    return true;
  }

  static bool _prefixesAlign(List<double> source, List<double> target) {
    if (source.length != target.length) return false;
    for (var index = 0; index < source.length; index++) {
      if ((source[index] - target[index]).abs() > 0.01) return false;
    }
    return true;
  }
}
