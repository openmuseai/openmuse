import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:pdf_rebuild_flutter/pdf_edit_backend_factory_stub.dart';
import 'package:pdf_rebuild_flutter/pdf_page_operations.dart';
import 'package:pdf_rebuild_flutter/pdf_page_ranges.dart';
import 'package:pdf_rebuild_flutter/pdf_product_backend.dart';

class _Pages implements PdfPageOperations {
  @override
  bool get isAvailable => true;
  @override
  String get unavailableReason => '';

  @override
  Future<int> pageCount(Uint8List bytes) async {
    if (bytes.first == 255) throw StateError('invalid PDF');
    return bytes.first;
  }

  @override
  Future<Uint8List> compose(List<PdfPageSelection> sources) async =>
      Uint8List.fromList([for (final source in sources) ...source.pageIndices]);
}

class _Edit extends ReadOnlyPdfEditBackend {
  bool opened = false;
  bool failMutation = false;
  Uint8List? exported;

  @override
  bool get supportsMutation => true;
  @override
  bool get isOpen => opened;

  @override
  Future<void> openBytes(Uint8List bytes) async {
    if (bytes.first == 254) throw StateError('native open failed');
    opened = true;
    exported = bytes;
  }

  @override
  Future<Uint8List> previewBytes() async => exported!;

  @override
  Future<void> replaceDomain(int index, String text) async {
    if (failMutation) throw StateError('edit failed');
  }

  @override
  void close() => opened = false;
}

void main() {
  test('GenOffice page range syntax is ordered, deduplicated and bounded', () {
    expect(parsePdfPageRanges('3-4,1，2–3', 4), [1, 2, 3, 4]);
    expect(parsePdfPageRanges('2,2', 4), [2]);
    expect(parsePdfPageRanges('0', 4), isNull);
    expect(parsePdfPageRanges('3-5', 4), isNull);
    expect(parsePdfPageRanges('4-2', 4), isNull);
    expect(parsePdfPageRanges('abc', 4), isNull);
    expect(parsePdfPageRanges(' , ', 4), isNull);
  });

  test('product commands preserve source order and page indices', () async {
    final product = PdfProductBackend(
      edit: const ReadOnlyPdfEditBackend(),
      pages: _Pages(),
    );
    final first = Uint8List.fromList([2]);
    final second = Uint8List.fromList([3]);
    final document = await product.open(first, 'first.pdf');
    expect(document.pageCount, 2);
    expect(document.name, 'first.pdf');
    expect(product.currentDocument, same(document));
    expect(await product.snapshot(), first);
    expect(await product.extract(first, 1), [1]);
    expect(await product.extractCurrentRange([2, 1]), [1, 0]);
    await expectLater(product.extractCurrentRange([3]), throwsRangeError);
    expect(await product.merge(first, second), [0, 1, 0, 1, 2]);
    expect(await product.deleteCurrentPage(0), [1]);
    expect(await product.reverseCurrentPages(), [1, 0]);
    expect(await product.splitCurrent(1), [
      [0],
      [1],
    ]);
    await expectLater(product.splitCurrent(0), throwsRangeError);
    await expectLater(product.splitCurrent(2), throwsRangeError);
    await expectLater(product.deleteCurrentPage(2), throwsRangeError);
    await product.open(Uint8List.fromList([5]), 'five.pdf');
    expect(await product.splitCurrent(2), [
      [0, 1],
      [2, 3],
      [4],
    ]);
    await product.open(Uint8List.fromList([1]), 'single.pdf');
    await expectLater(product.deleteCurrentPage(0), throwsStateError);
    await expectLater(product.splitCurrent(1), throwsStateError);
    product.close();
    expect(product.currentDocument, isNull);
    await expectLater(product.snapshot(), throwsStateError);
  });

  test(
    'current session exports edits for page tools and survives failed open',
    () async {
      final edit = _Edit();
      final product = PdfProductBackend(edit: edit, pages: _Pages());
      final original = Uint8List.fromList([2]);
      await product.open(original, 'original.pdf');
      edit.failMutation = true;
      await expectLater(product.replaceDomain(0, 'text'), throwsStateError);
      expect(product.isDirty, isFalse);
      edit.failMutation = false;
      edit.exported = Uint8List.fromList([3]);
      await product.replaceDomain(0, 'text');

      expect(await product.snapshot(), [3]);
      expect(await product.extractCurrent(2), [2]);
      expect(await product.deleteCurrentPage(1), [0, 2]);
      expect(await product.mergeCurrent(Uint8List.fromList([1])), [0, 1, 2, 0]);

      await expectLater(
        product.open(Uint8List.fromList([255]), 'bad.pdf'),
        throwsStateError,
      );
      await expectLater(
        product.open(Uint8List.fromList([254]), 'bad.pdf'),
        throwsStateError,
      );
      expect(product.currentDocument?.name, 'original.pdf');
      expect(product.isDirty, isTrue);
      expect(await product.snapshot(), [3]);

      product.close();
      expect(product.isDirty, isFalse);
      await expectLater(product.snapshot(), throwsStateError);
    },
  );
}
