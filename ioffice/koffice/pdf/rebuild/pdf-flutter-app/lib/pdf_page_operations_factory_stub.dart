import 'dart:typed_data';

import 'pdf_page_operations.dart';

PdfPageOperations createPdfPageOperations() =>
    const UnavailablePdfPageOperations();

class UnavailablePdfPageOperations implements PdfPageOperations {
  const UnavailablePdfPageOperations();

  @override
  bool get isAvailable => false;

  @override
  String get unavailableReason => '当前平台尚未连接页面组合内核。';

  @override
  Future<int> pageCount(Uint8List bytes) async =>
      throw UnsupportedError(unavailableReason);

  @override
  Future<Uint8List> compose(List<PdfPageSelection> sources) async =>
      throw UnsupportedError(unavailableReason);
}
