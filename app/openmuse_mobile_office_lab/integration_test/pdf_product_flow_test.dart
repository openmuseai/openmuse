import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_pdf_product/openmuse_pdf_product.dart';
import 'package:pdf_rebuild_flutter/main.dart' show PdfWorkspace;
import 'package:pdf_rebuild_flutter/pdf_product_chrome.dart';

import 'package:openmuse_mobile_office_lab/pdf_product_screen.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('mobile Host verifies PDF product and renders native pages', (
    tester,
  ) async {
    final identity =
        jsonDecode(
              await rootBundle.loadString('../openmuse_mobile/assets/pdf_product/identity.json'),
            )
            as Map;
    final key = await rootBundle.loadString('../openmuse_mobile/assets/pdf_product/public.pem');
    final bytes = await rootBundle.load('../openmuse_mobile/assets/pdf_product/pdf.mcpb');
    final product = await VerifiedPdfProduct.loadBundle(
      bundle: bytes.buffer.asUint8List(
        bytes.offsetInBytes,
        bytes.lengthInBytes,
      ),
      trustedPublicKeyPem: key,
      expectedKeyId: identity['keyId'] as String,
      expectedVersion: identity['version'] as String,
    );
    expect(product.chrome.tabs, hasLength(7));

    // The embedded editor saves through a Host channel. A missing registration
    // must fail this device test before a user reaches the system save picker.
    await expectLater(
      const MethodChannel(
        'cn.wps.pdf_rebuild_flutter/document_io',
      ).invokeMethod<bool>('savePdf', {
        'name': 'probe.pdf',
        'sourcePath': '/does-not-exist.pdf',
      }),
      throwsA(
        isA<PlatformException>().having(
          (error) => error.code,
          'code',
          'source',
        ),
      ),
    );
    await expectLater(
      const MethodChannel(
        'cn.wps.pdf_rebuild_flutter/document_io',
      ).invokeMethod<int>('savePdfBatch', {
        'files': [
          {'name': 'probe.pdf', 'sourcePath': '/does-not-exist.pdf'},
        ],
      }),
      throwsA(
        isA<PlatformException>().having(
          (error) => error.code,
          'code',
          'source',
        ),
      ),
    );

    await tester.pumpWidget(
      const MaterialApp(home: MobilePdfProductScreen(title: 'PDF')),
    );
    for (
      var i = 0;
      i < 50 && find.byType(PdfProductShell).evaluate().isEmpty;
      i++
    ) {
      await tester.pump(const Duration(milliseconds: 100));
    }
    expect(find.byType(PdfProductShell), findsOneWidget);
    expect(find.byKey(const Key('pdf-tab-home')), findsOneWidget);

    final fixture = await rootBundle.load(
      'packages/pdf_rebuild_flutter/assets/test/editable-two-page.pdf',
    );
    await tester.pumpWidget(
      MaterialApp(
        home: PdfWorkspace(
          chrome: product.chrome,
          commands: product.commands,
          enabledFeatures: product.enabledFeatures,
          initialBytes: Uint8List.fromList(fixture.buffer.asUint8List()),
          initialName: 'editable-two-page.pdf',
        ),
      ),
    );
    for (
      var i = 0;
      i < 100 && find.text('第 1 页，共 2 页').evaluate().isEmpty;
      i++
    ) {
      await tester.pump(const Duration(milliseconds: 100));
    }
    expect(find.text('第 1 页，共 2 页'), findsOneWidget);
    await tester.pump(const Duration(milliseconds: 500));
    expect(find.byType(PdfProductShell), findsOneWidget);
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pump(const Duration(seconds: 1));
  });
}
