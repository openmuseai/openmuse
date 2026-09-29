import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_docx/openmuse_office_docx.dart';

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
}
