import 'dart:io';

import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_docx/openmuse_office_docx.dart';
import 'package:test/test.dart';

void main() {
  final libraryPath = Platform.environment['OPENMUSE_DOCX_TEST_LIBRARY'];

  test(
    'loads ABI and maps a bounded native failure',
    () async {
      final engine = DocxFfiEngine.open(libraryPath: libraryPath);
      expect(engine.abi, 'openmuse-docx-ffi@1');
      await expectLater(
        engine.inspect(OfficeFormat.word, const [1, 2, 3]),
        throwsA(isA<DocxEngineException>()),
      );
      await expectLater(
        engine.inspect(OfficeFormat.sheet, const [1]),
        throwsA(isA<DocxEngineException>()),
      );
    },
    skip: libraryPath == null ? 'native test library not supplied' : false,
  );
}
