import 'dart:math' as math;
import 'dart:ui';

/// The PDF reading session owns the active arrangement; signed product data
/// supplies the button presentation and spacing through the page viewport.
enum PdfPageArrangement {
  single,
  spread;

  int get columns => this == PdfPageArrangement.spread ? 2 : 1;

  int rowFirst(int oneBasedPage) =>
      ((oneBasedPage - 1) ~/ columns) * columns + 1;

  int stepPage(int oneBasedPage, int direction, int pageCount) {
    if (pageCount < 1) return oneBasedPage;
    final row = (oneBasedPage - 1) ~/ columns;
    final lastRow = (pageCount - 1) ~/ columns;
    return (row + direction).clamp(0, lastRow) * columns + 1;
  }
}

final class PdfPageArrangementPresentation {
  PdfPageArrangementPresentation.fromJson(Map<String, dynamic>? json)
    : command = json?['command'] as String? ?? 'pdf.view.toggleSpread',
      singleButton = PdfArrangementButton.fromJson(
        json?['singleButton'] as Map<String, dynamic>?,
        fallbackLabel: '双页',
        fallbackIcon: 'IconSpread',
      ),
      spreadButton = PdfArrangementButton.fromJson(
        json?['spreadButton'] as Map<String, dynamic>?,
        fallbackLabel: '单页',
        fallbackIcon: 'IconSinglePage',
      );

  final String command;
  final PdfArrangementButton singleButton;
  final PdfArrangementButton spreadButton;

  PdfArrangementButton buttonFor(PdfPageArrangement arrangement) =>
      arrangement == PdfPageArrangement.single ? singleButton : spreadButton;
}

final class PdfArrangementButton {
  PdfArrangementButton.fromJson(
    Map<String, dynamic>? json, {
    required String fallbackLabel,
    required String fallbackIcon,
  }) : label = json?['label'] as String? ?? fallbackLabel,
       icon = json?['icon'] as String? ?? fallbackIcon;

  final String label;
  final String icon;
}

/// Pure page geometry, independent of PDF rendering and command handling.
final class PdfArrangementGeometry {
  const PdfArrangementGeometry(this.pageRects, this.documentSize);

  final List<Rect> pageRects;
  final Size documentSize;

  Rect rowBounds(int oneBasedPage, PdfPageArrangement arrangement) {
    final first = arrangement.rowFirst(oneBasedPage) - 1;
    var bounds = pageRects[first];
    for (
      var index = first + 1;
      index < math.min(first + arrangement.columns, pageRects.length);
      index++
    ) {
      bounds = bounds.expandToInclude(pageRects[index]);
    }
    return bounds;
  }

  static PdfArrangementGeometry layout(
    List<Size> pages, {
    required PdfPageArrangement arrangement,
    required double horizontalPadding,
    required double verticalPadding,
    required double pageGap,
  }) {
    var widestRow = 0.0;
    for (var first = 0; first < pages.length; first += arrangement.columns) {
      var width = 0.0;
      for (
        var index = first;
        index < math.min(first + arrangement.columns, pages.length);
        index++
      ) {
        if (index > first) width += pageGap;
        width += pages[index].width;
      }
      widestRow = math.max(widestRow, width);
    }
    final rects = <Rect>[];
    var y = verticalPadding;
    for (var first = 0; first < pages.length; first += arrangement.columns) {
      final end = math.min(first + arrangement.columns, pages.length);
      var rowWidth = 0.0;
      var rowHeight = 0.0;
      for (var index = first; index < end; index++) {
        if (index > first) rowWidth += pageGap;
        rowWidth += pages[index].width;
        rowHeight = math.max(rowHeight, pages[index].height);
      }
      var x = horizontalPadding + (widestRow - rowWidth) / 2;
      for (var index = first; index < end; index++) {
        rects.add(Rect.fromLTWH(x, y, pages[index].width, pages[index].height));
        x += pages[index].width + pageGap;
      }
      y += rowHeight + pageGap;
    }
    if (pages.isNotEmpty) y -= pageGap;
    return PdfArrangementGeometry(
      List.unmodifiable(rects),
      Size(widestRow + horizontalPadding * 2, y + verticalPadding),
    );
  }
}
