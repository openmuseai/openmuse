import 'dart:io';

import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_viewers/openmuse_office_viewers.dart';
import 'package:test/test.dart';

void main() {
  final libraryPath = Platform.environment['OPENMUSE_VIEWERS_TEST_LIBRARY'];

  test(
    'loads viewer ABI and keeps invalid inputs fail closed',
    () async {
      final engine = OfficeViewersFfiEngine.open(libraryPath: libraryPath);
      expect(engine.abi, 'openmuse-office-viewers-ffi@1');
      await expectLater(
        engine.inspect(OfficeFormat.sheet, const [1, 2, 3]),
        throwsA(isA<OfficeViewerException>()),
      );
      await expectLater(
        engine.inspect(OfficeFormat.slides, const [1]),
        throwsA(isA<OfficeViewerException>()),
      );
      await expectLater(
        engine.inspect(OfficeFormat.pdf, const [1]),
        throwsA(isA<OfficeViewerException>()),
      );
      expect(
        () => engine.exportSimple(OfficeFormat.sheet, const [1], const ['x']),
        throwsA(isA<OfficeViewerException>()),
      );
    },
    skip: libraryPath == null ? 'native test library not supplied' : false,
  );
}
