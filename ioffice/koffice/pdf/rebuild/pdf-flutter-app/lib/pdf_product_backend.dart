import 'dart:typed_data';

import 'pdf_edit_backend.dart';
import 'pdf_page_operations.dart';

/// The app's platform boundary. New PDF commands belong here before they are
/// exposed in the UI, so Desktop and Mobile share the same command semantics.
class PdfProductBackend {
  PdfProductBackend({PdfEditBackend? edit, PdfPageOperations? pages})
    : edit = edit ?? createPdfEditBackend(),
      pages = pages ?? createPdfPageOperations();

  final PdfEditBackend edit;
  final PdfPageOperations pages;
  PdfProductDocument? _document;
  bool _dirty = false;
  int _sessionGeneration = 0;

  bool get canEditText => edit.supportsMutation;
  bool get canComposePages => pages.isAvailable;
  PdfProductDocument? get currentDocument => _document;
  bool get isDirty => _dirty;
  int get sessionGeneration => _sessionGeneration;

  Future<PdfProductDocument> open(Uint8List bytes, String name) async {
    if (bytes.isEmpty) throw ArgumentError('PDF 字节为空');
    final pageCount = canComposePages ? await pages.pageCount(bytes) : 0;
    if (canEditText) await edit.openBytes(bytes);
    final document = PdfProductDocument(bytes, name, pageCount);
    _document = document;
    _dirty = false;
    _sessionGeneration++;
    return document;
  }

  void markDirty() {
    if (_document == null || !edit.isOpen) {
      throw StateError('编辑会话尚未打开');
    }
    _dirty = true;
  }

  Future<PdfEditFocus> replaceRange(int start, int end, String text) async {
    final result = await edit.replaceRange(start, end, text);
    markDirty();
    return result;
  }

  Future<void> replaceDomain(int index, String text) async {
    await edit.replaceDomain(index, text);
    markDirty();
  }

  Future<void> undo() async {
    await edit.undo();
    markDirty();
  }

  Future<void> redo() async {
    await edit.redo();
    markDirty();
  }

  Future<Uint8List> snapshot() async {
    final document = _document;
    if (document == null) throw StateError('请先打开 PDF');
    if (!_dirty) return document.bytes;
    if (!edit.isOpen) throw StateError('编辑会话已关闭，无法导出当前修改');
    return edit.previewBytes();
  }

  Future<PdfPageSize> pageSize(int pageIndex) => edit.pageSize(pageIndex);

  Future<PdfPageTextData> readPageText(int pageIndex) =>
      edit.readPageText(pageIndex);

  Future<List<PdfPageLinkData>> readPageLinks(int pageIndex) =>
      edit.readPageLinks(pageIndex);

  Future<PdfEditRaster> renderPageAt(int pageIndex, int width, int height) =>
      edit.renderPageAt(pageIndex, width, height);

  Future<PdfEditRaster> renderPageRegion(
    int pageIndex,
    int x,
    int y,
    int width,
    int height,
    int fullWidth,
    int fullHeight,
  ) => edit.renderPageRegion(
    pageIndex,
    x,
    y,
    width,
    height,
    fullWidth,
    fullHeight,
  );

  Future<Uint8List> extractCurrent(int pageIndex) async =>
      extract(await snapshot(), pageIndex);

  Future<Uint8List> extractCurrentRange(List<int> oneBasedPages) async {
    final bytes = await snapshot();
    final count = await pages.pageCount(bytes);
    if (oneBasedPages.isEmpty ||
        oneBasedPages.any((page) => page < 1 || page > count)) {
      throw RangeError('抽取页码超出文档范围');
    }
    return pages.compose([
      PdfPageSelection(bytes, [for (final page in oneBasedPages) page - 1]),
    ]);
  }

  Future<Uint8List> deleteCurrentPage(int pageIndex) async {
    final bytes = await snapshot();
    final count = await pages.pageCount(bytes);
    if (count <= 1) throw StateError('PDF 至少需要保留一页');
    if (pageIndex < 0 || pageIndex >= count) {
      throw RangeError.range(pageIndex, 0, count - 1, 'pageIndex');
    }
    return pages.compose([
      PdfPageSelection(bytes, [
        for (var index = 0; index < count; index++)
          if (index != pageIndex) index,
      ]),
    ]);
  }

  Future<Uint8List> mergeCurrent(Uint8List incoming) async =>
      merge(await snapshot(), incoming);

  Future<Uint8List> reverseCurrentPages() async {
    final bytes = await snapshot();
    final count = await pages.pageCount(bytes);
    if (count < 2) throw StateError('倒序至少需要两页');
    return pages.compose([
      PdfPageSelection(bytes, [
        for (var index = count - 1; index >= 0; index--) index,
      ]),
    ]);
  }

  Future<List<Uint8List>> splitCurrent(int pagesPerFile) async {
    final bytes = await snapshot();
    final count = await pages.pageCount(bytes);
    if (count < 2) throw StateError('拆分至少需要两页');
    if (pagesPerFile < 1 || pagesPerFile >= count) {
      throw RangeError.range(pagesPerFile, 1, count - 1, 'pagesPerFile');
    }
    final results = <Uint8List>[];
    for (var start = 0; start < count; start += pagesPerFile) {
      final end = (start + pagesPerFile).clamp(0, count);
      results.add(
        await pages.compose([
          PdfPageSelection(bytes, [
            for (var page = start; page < end; page++) page,
          ]),
        ]),
      );
    }
    return results;
  }

  Future<Uint8List> extract(Uint8List bytes, int pageIndex) => pages.compose([
    PdfPageSelection(bytes, [pageIndex]),
  ]);

  Future<Uint8List> merge(Uint8List current, Uint8List incoming) async {
    final currentCount = await pages.pageCount(current);
    final incomingCount = await pages.pageCount(incoming);
    return pages.compose([
      PdfPageSelection(current, List<int>.generate(currentCount, (i) => i)),
      PdfPageSelection(incoming, List<int>.generate(incomingCount, (i) => i)),
    ]);
  }

  void close() {
    edit.close();
    _document = null;
    _dirty = false;
    _sessionGeneration++;
  }
}

class PdfProductDocument {
  const PdfProductDocument(this.bytes, this.name, this.pageCount);

  final Uint8List bytes;
  final String name;
  final int pageCount;
}
