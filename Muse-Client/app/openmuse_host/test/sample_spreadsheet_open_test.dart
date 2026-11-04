import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_spreadsheet_product/openmuse_spreadsheet_product.dart';
import 'package:spreadsheet_editor/spreadsheet_editor.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('both sample XLSX files open in the signed Spreadsheet product', (
    tester,
  ) async {
    final directory = Directory(Platform.environment['OPENMUSE_SAMPLE_DIR']!);
    final bundle = File(
      Platform.environment['SPREADSHEET_PRODUCT_E2E_BUNDLE']!,
    );
    final publicKey = File(
      Platform.environment['SPREADSHEET_PRODUCT_E2E_PUBLIC_KEY']!,
    );
    final keyId = Platform.environment['SPREADSHEET_PRODUCT_E2E_KEY_ID']!;
    await tester.runAsync(() async {
      final product = await VerifiedSpreadsheetProduct.loadBundle(
        bundle: await bundle.readAsBytes(),
        trustedPublicKeyPem: await publicKey.readAsString(),
        expectedKeyId: keyId,
      );
      expect(product.chrome.tabs, isNotEmpty);
      final names =
          Platform.environment['OPENMUSE_SAMPLE_XLSX_FILES']?.split(';') ??
          ['出入库管理明细表.xlsx', '可视化图形图表区.xlsx'];
      expect(names.length, greaterThanOrEqualTo(2));
      for (final name in names) {
        final engine = GridcraftEngine();
        try {
          engine.open(
            name,
            await File('${directory.path}/$name').readAsBytes(),
          );
          final snapshot = engine.snapshot(rows: 8, cols: 8);
          expect(snapshot['cells'], isA<List>());
        } finally {
          engine.dispose();
        }
      }
    });
  });
}
