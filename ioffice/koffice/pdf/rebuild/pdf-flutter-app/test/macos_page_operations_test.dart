@TestOn('vm')
library;

import 'dart:io';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:pdf_rebuild_flutter/pdf_page_operations.dart';
import 'package:pdf_rebuild_flutter/pdf_page_operations_factory_io.dart';

void main() {
  final sourcePath = Platform.environment['KSP_PAGE_OPS_TEST_PDF'];

  test(
    'product ABI extracts, merges, rejects bad indexes, and reopens bytes',
    () async {
      final backend = NativePdfPageOperations();
      final source = await File(sourcePath!).readAsBytes();
      final count = await backend.pageCount(source);
      expect(count, greaterThanOrEqualTo(2));

      final extracted = await backend.compose([
        PdfPageSelection(source, [1, 0]),
      ]);
      expect(await backend.pageCount(extracted), 2);
      expect(String.fromCharCodes(extracted.take(4)), '%PDF');

      final merged = await backend.compose([
        PdfPageSelection(extracted, [0, 1]),
        PdfPageSelection(source, [0]),
      ]);
      expect(await backend.pageCount(merged), 3);

      await expectLater(
        backend.compose([
          PdfPageSelection(source, [count]),
        ]),
        throwsStateError,
      );
      expect(await backend.pageCount(Uint8List.fromList(merged)), 3);
    },
    skip: !Platform.isMacOS || sourcePath == null,
  );
}
