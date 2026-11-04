import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:pdf_rebuild_flutter/main.dart';
import 'package:pdf_rebuild_flutter/pdf_product_backend.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('native product flow opens, edits, saves and composes pages', (
    tester,
  ) async {
    final fixture = await rootBundle.load('assets/test/editable-two-page.pdf');
    final bytes = Uint8List.fromList(fixture.buffer.asUint8List());
    final product = PdfProductBackend();
    expect(product.canEditText, isTrue);
    expect(product.canComposePages, isTrue);

    await tester.pumpWidget(
      PdfRebuildApp(
        initialBytes: bytes,
        initialName: 'editable-two-page.pdf',
        productBackend: product,
      ),
    );
    for (var attempt = 0; attempt < 100 && !product.edit.isOpen; attempt++) {
      await tester.pump(const Duration(milliseconds: 200));
    }
    if (!product.edit.isOpen) {
      final error = find.textContaining('打开失败');
      if (error.evaluate().isNotEmpty) {
        fail('无法打开初始 PDF：${tester.widget<Text>(error.first).data}');
      }
    }
    expect(product.edit.isOpen, isTrue);
    for (
      var attempt = 0;
      attempt < 100 && find.text('第 1 页，共 2 页').evaluate().isEmpty;
      attempt++
    ) {
      await tester.pump(const Duration(milliseconds: 200));
    }
    if (find.text('第 1 页，共 2 页').evaluate().isEmpty) {
      final visibleText = find
          .byType(Text)
          .evaluate()
          .map((element) => (element.widget as Text).data)
          .whereType<String>()
          .toList();
      fail('页面计数未显示：$visibleText');
    }
    expect(find.text('第 1 页，共 2 页'), findsOneWidget);
    final secondPageSize = await product.pageSize(1);
    expect(secondPageSize.width, greaterThan(0));
    final secondPageText = await product.readPageText(1);
    expect(secondPageText.text, isNotEmpty);
    expect(secondPageText.characters, hasLength(secondPageText.text.length));
    final secondPage = await product.renderPageAt(1, 128, 128);
    expect(secondPage.rgba, hasLength(128 * 128 * 4));
    final region = await product.renderPageRegion(1, 32, 40, 64, 48, 128, 128);
    final expectedRegion = <int>[];
    for (var row = 40; row < 88; row++) {
      expectedRegion.addAll(
        secondPage.rgba.sublist((row * 128 + 32) * 4, (row * 128 + 96) * 4),
      );
    }
    expect(region.rgba, expectedRegion);

    await tester.tap(find.byKey(const Key('pdf-tab-edit')));
    await tester.pump();
    await tester.tap(find.byKey(const Key('pdf-control-edit-editText')));
    await tester.pump(const Duration(seconds: 1));
    final domains = await product.edit.loadPage(0);
    expect(domains, isNotEmpty);
    final original = await product.edit.domainText(domains.first.index);
    await product.replaceDomain(domains.first.index, '$original!');
    expect((await product.readPageText(0)).text, contains('!'));
    expect(product.edit.canUndo, isTrue);
    expect(product.isDirty, isTrue);
    await product.undo();
    expect(await product.edit.domainText(domains.first.index), original);
    await product.redo();
    final saved = await product.snapshot();
    expect(await product.pages.pageCount(saved), 2);

    await tester.tap(find.byKey(const Key('pdf-tab-page')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('pdf-control-page-delete')));
    for (
      var attempt = 0;
      attempt < 100 && find.text('第 1 页，共 1 页').evaluate().isEmpty;
      attempt++
    ) {
      await tester.pump(const Duration(milliseconds: 200));
    }
    expect(find.text('第 1 页，共 1 页'), findsOneWidget);
    expect(await product.pages.pageCount(await product.snapshot()), 1);

    await tester.tap(find.byKey(const Key('pdf-control-page-extract')));
    await tester.pump(const Duration(seconds: 2));
    expect(find.text('第 1 页，共 1 页'), findsOneWidget);
    expect(await product.pages.pageCount(await product.edit.previewBytes()), 1);
  });
}
