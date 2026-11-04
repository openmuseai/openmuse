import 'dart:typed_data';
import 'pdf_edit_backend.dart';

PdfEditBackend createPdfEditBackend() => const ReadOnlyPdfEditBackend();

class ReadOnlyPdfEditBackend implements PdfEditBackend {
  const ReadOnlyPdfEditBackend();
  @override
  bool get supportsMutation => false;
  @override
  String get unavailableReason => '当前平台尚未连接 rebuild native 编辑内核。';
  @override
  bool get isOpen => false;
  @override
  bool get canUndo => false;
  @override
  bool get canRedo => false;
  @override
  Future<void> openPath(String path) async {}
  @override
  Future<void> openBytes(Uint8List bytes) async {}
  @override
  Future<List<PdfEditDomain>> loadPage(int zeroBasedPageIndex) async =>
      const [];
  @override
  Future<List<PdfEditDomain>> refreshDomains() async => const [];
  @override
  Future<String> domainText(int domainIndex) async => '';
  @override
  Future<void> replaceDomain(int domainIndex, String text) async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<PdfEditFocus> focusAt(double pageX, double pageY) async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<PdfEditFocus> setSelection(int start, int end) async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<PdfEditFocus> replaceRange(int start, int end, String text) async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<PdfEditRaster> renderPage(int width, int height) async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<PdfPageSize> pageSize(int zeroBasedPageIndex) async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<PdfPageTextData> readPageText(int zeroBasedPageIndex) async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<List<PdfPageLinkData>> readPageLinks(int zeroBasedPageIndex) async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<PdfEditRaster> renderPageAt(
    int zeroBasedPageIndex,
    int width,
    int height,
  ) async => throw UnsupportedError(unavailableReason);
  @override
  Future<PdfEditRaster> renderPageRegion(
    int zeroBasedPageIndex,
    int x,
    int y,
    int width,
    int height,
    int fullWidth,
    int fullHeight,
  ) async => throw UnsupportedError(unavailableReason);
  @override
  Future<void> undo() async => throw UnsupportedError(unavailableReason);
  @override
  Future<void> redo() async => throw UnsupportedError(unavailableReason);
  @override
  Future<Uint8List> previewBytes() async =>
      throw UnsupportedError(unavailableReason);
  @override
  Future<void> saveTo(String path) async =>
      throw UnsupportedError(unavailableReason);
  @override
  void close() {}
}
