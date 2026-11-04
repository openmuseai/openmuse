import 'dart:typed_data';

import 'pdf_edit_backend_factory_stub.dart'
    if (dart.library.io) 'pdf_edit_backend_factory_io.dart'
    if (dart.library.js_interop) 'pdf_edit_backend_factory_web.dart'
    as backend_factory;

class PdfEditDomain {
  const PdfEditDomain(this.index, this.left, this.bottom, this.right, this.top);
  final int index;
  final double left, bottom, right, top;
}

class PdfEditCaret {
  const PdfEditCaret(this.bottomX, this.bottomY, this.topX, this.topY);
  final double bottomX, bottomY, topX, topY;
}

class PdfEditFocus {
  const PdfEditFocus(this.text, this.start, this.end, this.caret);
  final String text;
  final int start, end;
  // Caret geometry is presentation state. A valid text mutation/selection must
  // not fail merely because a synthetic line end has no glyph geometry yet.
  final PdfEditCaret? caret;
}

class PdfEditRaster {
  const PdfEditRaster(this.width, this.height, this.rgba);
  final int width, height;
  final Uint8List rgba;
}

class PdfPageSize {
  const PdfPageSize(this.width, this.height);
  final double width, height;
}

class PdfPageTextCharacter {
  const PdfPageTextCharacter(
    this.codeUnit,
    this.left,
    this.bottom,
    this.right,
    this.top,
  );
  final int codeUnit;
  final double left, bottom, right, top;
}

class PdfPageTextData {
  const PdfPageTextData(this.text, this.characters);
  final String text;
  // One character box per UTF-16 code unit in text.
  final List<PdfPageTextCharacter> characters;
}

enum PdfPageLinkKind { unsupported, uri, page }

class PdfPageLinkData {
  const PdfPageLinkData({
    required this.kind,
    required this.left,
    required this.bottom,
    required this.right,
    required this.top,
    this.uri,
    this.pageIndex,
  });

  final PdfPageLinkKind kind;
  final double left, bottom, right, top;
  final Uri? uri;
  final int? pageIndex;
}

class PdfTextReplacement {
  const PdfTextReplacement(this.start, this.end, this.text);
  final int start, end;
  final String text;
}

PdfTextReplacement derivePdfTextReplacement({
  required String currentText,
  required int selectionStart,
  required int selectionEnd,
  required String targetText,
  required int targetCaret,
}) {
  var start = selectionStart.clamp(0, currentText.length);
  var end = selectionEnd.clamp(start, currentText.length);
  final before = currentText.substring(0, start);
  final after = currentText.substring(end);
  if (targetText.startsWith(before) && targetText.endsWith(after)) {
    return PdfTextReplacement(
      start,
      end,
      targetText.substring(start, targetText.length - after.length),
    );
  }
  if (start == end && targetText.length < currentText.length) {
    final removed = currentText.length - targetText.length;
    start = targetCaret < start ? (start - removed).clamp(0, start) : start;
    end = (start + removed).clamp(start, currentText.length);
    if (currentText.replaceRange(start, end, '') == targetText) {
      return PdfTextReplacement(start, end, '');
    }
  }
  start = 0;
  final prefixLimit = targetText.length < currentText.length
      ? targetText.length
      : currentText.length;
  while (start < prefixLimit &&
      targetText.codeUnitAt(start) == currentText.codeUnitAt(start)) {
    start++;
  }
  var suffix = 0;
  while (suffix < targetText.length - start &&
      suffix < currentText.length - start &&
      targetText.codeUnitAt(targetText.length - suffix - 1) ==
          currentText.codeUnitAt(currentText.length - suffix - 1)) {
    suffix++;
  }
  return PdfTextReplacement(
    start,
    currentText.length - suffix,
    targetText.substring(start, targetText.length - suffix),
  );
}

abstract interface class PdfEditBackend {
  bool get supportsMutation;
  String get unavailableReason;
  bool get isOpen;
  bool get canUndo;
  bool get canRedo;
  Future<void> openPath(String path);
  Future<void> openBytes(Uint8List bytes);
  Future<List<PdfEditDomain>> loadPage(int zeroBasedPageIndex);
  Future<List<PdfEditDomain>> refreshDomains();
  Future<String> domainText(int domainIndex);
  Future<void> replaceDomain(int domainIndex, String text);
  Future<PdfEditFocus> focusAt(double pageX, double pageY);
  Future<PdfEditFocus> setSelection(int start, int end);
  Future<PdfEditFocus> replaceRange(int start, int end, String text);
  Future<PdfEditRaster> renderPage(int width, int height);
  Future<PdfPageSize> pageSize(int zeroBasedPageIndex);
  Future<PdfPageTextData> readPageText(int zeroBasedPageIndex);
  Future<List<PdfPageLinkData>> readPageLinks(int zeroBasedPageIndex);
  Future<PdfEditRaster> renderPageAt(
    int zeroBasedPageIndex,
    int width,
    int height,
  );
  Future<PdfEditRaster> renderPageRegion(
    int zeroBasedPageIndex,
    int x,
    int y,
    int width,
    int height,
    int fullWidth,
    int fullHeight,
  );
  Future<void> undo();
  Future<void> redo();
  Future<Uint8List> previewBytes();
  Future<void> saveTo(String path);
  void close();
}

PdfEditBackend createPdfEditBackend() => backend_factory.createPdfEditBackend();
