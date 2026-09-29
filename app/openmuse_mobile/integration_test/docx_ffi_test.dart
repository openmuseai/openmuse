import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_docx/openmuse_office_docx.dart';
import 'package:openmuse_office_viewers/openmuse_office_viewers.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('packaged DOCX ABI is callable', (tester) async {
    final engine = DocxFfiEngine.open();
    expect(engine.abi, 'openmuse-docx-ffi@1');
    await expectLater(
      engine.inspect(OfficeFormat.word, const [1, 2, 3]),
      throwsA(isA<DocxEngineException>()),
    );
  });

  testWidgets('packaged viewer ABI is callable and view only', (tester) async {
    final engine = OfficeViewersFfiEngine.open();
    expect(engine.abi, 'openmuse-office-viewers-ffi@1');
    await expectLater(
      engine.inspect(OfficeFormat.sheet, const [1, 2, 3]),
      throwsA(isA<OfficeViewerException>()),
    );
    await expectLater(
      engine.inspect(OfficeFormat.slides, const [1, 2, 3]),
      throwsA(isA<OfficeViewerException>()),
    );
    await expectLater(
      engine.inspect(OfficeFormat.pdf, const [1, 2, 3]),
      throwsA(isA<OfficeViewerException>()),
    );
    expect(
      () => engine.exportSimple(OfficeFormat.sheet, const [1], const ['x']),
      throwsA(isA<OfficeViewerException>()),
    );
  });
}
