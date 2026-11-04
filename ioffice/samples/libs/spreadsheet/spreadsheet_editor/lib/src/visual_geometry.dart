import 'dart:math' as math;

import 'sheet_axis_geometry.dart';

/// The resize edge or corner chosen on a selected workbook object.
enum VisualResizeHandle {
  nw(true, false, true, false),
  n(false, false, true, false),
  ne(false, true, true, false),
  e(false, true, false, false),
  se(false, true, false, true),
  s(false, false, false, true),
  sw(true, false, false, true),
  w(true, false, false, false);

  const VisualResizeHandle(this.west, this.east, this.north, this.south);
  final bool west;
  final bool east;
  final bool north;
  final bool south;
}

/// Workbook object bounds in unscrolled sheet pixels.
final class VisualFrame {
  const VisualFrame(this.left, this.top, this.width, this.height);
  final double left;
  final double top;
  final double width;
  final double height;

  double get right => left + width;
  double get bottom => top + height;
}

/// Converts viewport gestures into GridCraft's one-cell anchor model. GridCraft
/// and the sheet grid both use 96-dpi workbook pixels before zoom.
/// Input recognition and product-owned command bindings live elsewhere.
final class VisualGeometry {
  const VisualGeometry._();

  static const maxRows = 1048576;
  static const maxCols = 16384;
  static const minFramePixels = 24.0;
  static const maxFramePixels = 10000.0;

  static VisualFrame frame(
    Map<dynamic, dynamic> visual, {
    required SheetAxisGeometry rows,
    required SheetAxisGeometry cols,
    double zoomFactor = 1,
  }) {
    final anchor = visual['anchor'] as Map;
    return VisualFrame(
      cols.leading((anchor['col'] as num).toInt()) +
          (anchor['dx'] as num).toDouble() * zoomFactor,
      rows.leading((anchor['row'] as num).toInt()) +
          (anchor['dy'] as num).toDouble() * zoomFactor,
      (anchor['width'] as num).toDouble() * zoomFactor,
      (anchor['height'] as num).toDouble() * zoomFactor,
    );
  }

  static Map<String, dynamic> move(
    Map<dynamic, dynamic> visual, {
    required double deltaX,
    required double deltaY,
    required SheetAxisGeometry rows,
    required SheetAxisGeometry cols,
    double zoomFactor = 1,
  }) {
    final original = frame(
      visual,
      rows: rows,
      cols: cols,
      zoomFactor: zoomFactor,
    );
    return {
      'kind': visual['kind'],
      'id': visual['id'],
      ..._location(
        original.left + deltaX,
        original.top + deltaY,
        rows: rows,
        cols: cols,
        zoomFactor: zoomFactor,
      ),
    };
  }

  /// Walks only the requested edges. Opposite edges stay fixed, with a 24 px
  /// minimum and a sheet-start clamp for west and north drags.
  static VisualFrame resizedFrame(
    VisualFrame frame,
    VisualResizeHandle handle, {
    required double deltaX,
    required double deltaY,
    double zoomFactor = 1,
  }) {
    var left = frame.left;
    var top = frame.top;
    var width = frame.width;
    var height = frame.height;
    if (handle.west) {
      final minShift = math.max(-left, width - maxFramePixels * zoomFactor);
      final maxShift = math.max(minShift, width - minFramePixels);
      final shift = deltaX.clamp(minShift, maxShift);
      left += shift;
      width -= shift;
    } else if (handle.east) {
      width = (width + deltaX).clamp(
        minFramePixels,
        maxFramePixels * zoomFactor,
      );
    }
    if (handle.north) {
      final minShift = math.max(-top, height - maxFramePixels * zoomFactor);
      final maxShift = math.max(minShift, height - minFramePixels);
      final shift = deltaY.clamp(minShift, maxShift);
      top += shift;
      height -= shift;
    } else if (handle.south) {
      height = (height + deltaY).clamp(
        minFramePixels,
        maxFramePixels * zoomFactor,
      );
    }
    return VisualFrame(left, top, width, height);
  }

  static Map<String, dynamic> resize(
    Map<dynamic, dynamic> visual, {
    required VisualResizeHandle handle,
    required double deltaX,
    required double deltaY,
    required SheetAxisGeometry rows,
    required SheetAxisGeometry cols,
    double zoomFactor = 1,
  }) {
    final next = resizedFrame(
      frame(visual, rows: rows, cols: cols, zoomFactor: zoomFactor),
      handle,
      deltaX: deltaX,
      deltaY: deltaY,
      zoomFactor: zoomFactor,
    );
    return {
      'kind': visual['kind'],
      'id': visual['id'],
      ..._location(
        next.left,
        next.top,
        rows: rows,
        cols: cols,
        zoomFactor: zoomFactor,
      ),
      'width': next.width / zoomFactor,
      'height': next.height / zoomFactor,
    };
  }

  static Map<String, dynamic> _location(
    double left,
    double top, {
    required SheetAxisGeometry rows,
    required SheetAxisGeometry cols,
    double zoomFactor = 1,
  }) {
    final x = left.clamp(0.0, cols.totalExtent - 1);
    final y = top.clamp(0.0, rows.totalExtent - 1);
    final col = cols.indexAt(x);
    final row = rows.indexAt(y);
    return {
      'at': '${_columnLabel(col)}${row + 1}',
      'dx': (x - cols.leading(col)) / zoomFactor,
      'dy': (y - rows.leading(row)) / zoomFactor,
    };
  }

  static String _columnLabel(int col) {
    var n = col + 1;
    var out = '';
    while (n > 0) {
      n--;
      out = String.fromCharCode(65 + n % 26) + out;
      n ~/= 26;
    }
    return out;
  }
}
