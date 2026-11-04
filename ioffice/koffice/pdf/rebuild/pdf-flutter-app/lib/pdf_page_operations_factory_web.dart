import 'dart:js_interop';
import 'dart:typed_data';

import 'pdf_page_operations.dart';
import 'pdf_web_bridge.dart';

PdfPageOperations createPdfPageOperations() => const WebPdfPageOperations();

class WebPdfPageOperations implements PdfPageOperations {
  const WebPdfPageOperations();

  @override
  bool get isAvailable => true;
  @override
  String get unavailableReason => '';

  @override
  Future<int> pageCount(Uint8List bytes) async {
    if (bytes.isEmpty) throw ArgumentError('PDF 字节为空');
    final bridge = await webProductBridge;
    return webCall<JSNumber>(bridge, 'pageCount', [bytes.toJS]).toDartInt;
  }

  @override
  Future<Uint8List> compose(List<PdfPageSelection> sources) async {
    if (sources.isEmpty) throw ArgumentError('没有待组合的 PDF');
    final bridge = await webProductBridge;
    final files = sources.map((source) => source.bytes.toJS).toList().toJS;
    final pages = sources
        .map(
          (source) =>
              source.pageIndices.map((index) => index.toJS).toList().toJS,
        )
        .toList()
        .toJS;
    return webBytes(
      webCall<JSUint8Array>(bridge, 'composeFlat', [files, pages]),
    );
  }
}
