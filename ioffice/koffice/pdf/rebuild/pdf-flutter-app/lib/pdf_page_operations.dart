import 'dart:typed_data';

import 'pdf_page_operations_factory_stub.dart'
    if (dart.library.io) 'pdf_page_operations_factory_io.dart'
    if (dart.library.js_interop) 'pdf_page_operations_factory_web.dart'
    as page_factory;

class PdfPageSelection {
  const PdfPageSelection(this.bytes, this.pageIndices);

  final Uint8List bytes;
  final List<int> pageIndices;
}

abstract interface class PdfPageOperations {
  bool get isAvailable;
  String get unavailableReason;
  Future<int> pageCount(Uint8List bytes);
  Future<Uint8List> compose(List<PdfPageSelection> sources);
}

PdfPageOperations createPdfPageOperations() =>
    page_factory.createPdfPageOperations();
