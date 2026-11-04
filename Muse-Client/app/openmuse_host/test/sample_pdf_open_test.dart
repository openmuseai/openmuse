import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_pdf_product/openmuse_pdf_product.dart';
import 'package:pdf_rebuild_flutter/main.dart' show PdfWorkspace;
import 'package:pdf_rebuild_flutter/pdf_product_backend.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('sample PDF opens in the signed PDF product', (tester) async {
    final samplePath = Platform.environment['OPENMUSE_SAMPLE_PDF'];
    final bundlePath = Platform.environment['PDF_PRODUCT_E2E_BUNDLE'];
    final keyPath = Platform.environment['PDF_PRODUCT_E2E_PUBLIC_KEY'];
    final keyId = Platform.environment['PDF_PRODUCT_E2E_KEY_ID'];
    if (samplePath == null ||
        bundlePath == null ||
        keyPath == null ||
        keyId == null) {
      fail('sample PDF and signed product paths are required');
    }
    late Uint8List bytes;
    late VerifiedPdfProduct product;
    late int pageCount;
    await tester.runAsync(() async {
      bytes = await File(samplePath).readAsBytes();
      product = await VerifiedPdfProduct.loadBundle(
        bundle: await File(bundlePath).readAsBytes(),
        trustedPublicKeyPem: await File(keyPath).readAsString(),
        expectedKeyId: keyId,
      );
      final backend = PdfProductBackend();
      try {
        final document = await backend.open(bytes, 'sample.pdf');
        pageCount = document.pageCount;
        final raster = await backend.renderPageAt(0, 128, 128);
        expect(raster.rgba, hasLength(128 * 128 * 4));
        expect(raster.rgba.any((byte) => byte != 255), isTrue);
      } finally {
        backend.close();
      }
    });
    expect(pageCount, greaterThan(0));
    await tester.pumpWidget(
      MaterialApp(
        home: PdfWorkspace(
          chrome: product.chrome,
          commands: product.commands,
          enabledFeatures: product.enabledFeatures,
          initialBytes: bytes,
          initialName: 'sample.pdf',
        ),
      ),
    );
    for (var i = 0; i < 80; i++) {
      await tester.runAsync(
        () async => Future<void>.delayed(const Duration(milliseconds: 100)),
      );
      await tester.pump();
      if (find.text('第 1 页，共 $pageCount 页').evaluate().isNotEmpty) break;
    }
    expect(find.byType(PdfWorkspace), findsOneWidget);
    expect(find.text('第 1 页，共 $pageCount 页'), findsOneWidget);
    await tester.pump(const Duration(milliseconds: 500));
    await tester.runAsync(
      () async => Future<void>.delayed(const Duration(milliseconds: 300)),
    );
    await tester.pump(const Duration(milliseconds: 300));
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pump(const Duration(seconds: 1));
  });
}
