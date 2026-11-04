@TestOn('vm')
library;

import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:pdf_rebuild_flutter/pdf_edit_backend_factory_io.dart';
import 'package:pdf_rebuild_flutter/pdf_edit_backend.dart';
import 'package:pdf_rebuild_flutter/pdf_product_backend.dart';
import 'package:pdf_rebuild_flutter/product_pdf_document.dart';
import 'package:pdfrx/pdfrx.dart';

void main() {
  final sourcePath = Platform.environment['KSP_MAC_EDIT_TEST_PDF'];

  test(
    'macOS native backend edits, undoes, redoes, previews, and saves',
    () async {
      final backend = NativePdfEditBackend();
      final output = File(
        '${Directory.systemTemp.path}/pdf-rebuild-flutter-edit-${DateTime.now().microsecondsSinceEpoch}.pdf',
      );
      addTearDown(() {
        backend.close();
        if (output.existsSync()) output.deleteSync();
      });

      await backend.openPath(sourcePath!);
      final firstSize = await backend.pageSize(0);
      final secondSize = await backend.pageSize(1);
      expect(firstSize.width, greaterThan(0));
      expect(firstSize.height, greaterThan(0));
      expect(secondSize.width, greaterThan(0));
      final secondText = await backend.readPageText(1);
      expect(secondText.text, isNotEmpty);
      expect(secondText.characters, hasLength(secondText.text.length));
      for (final char in secondText.characters) {
        expect(char.left.isFinite && char.top.isFinite, isTrue);
      }
      await expectLater(backend.readPageText(99), throwsStateError);
      final before = await backend.renderPageAt(0, 320, 480);
      expect(before.rgba, hasLength(320 * 480 * 4));
      final tile = await backend.renderPageRegion(0, 53, 71, 128, 96, 320, 480);
      final expectedTile = <int>[];
      for (var row = 71; row < 71 + 96; row++) {
        expectedTile.addAll(
          before.rgba.sublist((row * 320 + 53) * 4, (row * 320 + 53 + 128) * 4),
        );
      }
      expect(tile.rgba, expectedTile);
      await expectLater(
        backend.renderPageRegion(0, 300, 0, 128, 96, 320, 480),
        throwsArgumentError,
      );
      await expectLater(backend.pageSize(99), throwsStateError);
      final domains = await backend.loadPage(0);
      expect(domains, isNotEmpty);

      final domain = domains.first;
      final initialFocus = await backend.focusAt(
        (domain.left + domain.right) / 2,
        (domain.bottom + domain.top) / 2,
      );
      final original = initialFocus.text;
      expect(original.length, greaterThan(4));
      final movedCaret = await backend.setSelection(1, 1);
      expect(movedCaret.start, 1);
      expect(movedCaret.end, 1);
      // A newline may temporarily have no glyph-backed caret geometry. The
      // Android contract still reports OnChars as successful and keeps editing.
      final newline = await backend.replaceRange(1, 1, '\n');
      expect(newline.text.substring(1, 2), '\n');
      expect(newline.start, 2);
      expect(newline.end, 2);
      await backend.undo();
      final afterNewlineUndo = await backend.focusAt(
        (domain.left + domain.right) / 2,
        (domain.bottom + domain.top) / 2,
      );
      expect(afterNewlineUndo.text, original);
      final replaceEnd = original.length < 8 ? original.length : 8;
      const replacement = 'macOS';
      final changed = await backend.replaceRange(0, replaceEnd, replacement);
      expect(changed.text, startsWith(replacement));
      expect(changed.text, endsWith(original.substring(replaceEnd)));
      expect(backend.canUndo, isTrue);

      await backend.undo();
      final restored = await backend.focusAt(
        (domain.left + domain.right) / 2,
        (domain.bottom + domain.top) / 2,
      );
      expect(restored.text, original);
      expect(backend.canRedo, isTrue);

      await backend.redo();
      final redone = await backend.focusAt(
        (domain.left + domain.right) / 2,
        (domain.bottom + domain.top) / 2,
      );
      expect(redone.text, startsWith(replacement));
      final deleted = await backend.replaceRange(0, 2, '');
      expect(deleted.text, startsWith(replacement.substring(2)));
      expect(deleted.text, endsWith(original.substring(replaceEnd)));

      final raster = await backend.renderPage(320, 480);
      final sameSessionRaster = await backend.renderPageAt(0, 320, 480);
      expect(raster.width, 320);
      expect(raster.height, 480);
      expect(raster.rgba, hasLength(320 * 480 * 4));
      expect(
        raster.rgba.where((value) => value < 250).length,
        greaterThan(100),
      );
      expect(sameSessionRaster.rgba, raster.rgba);
      expect(sameSessionRaster.rgba, isNot(before.rgba));
      final editedText = await backend.readPageText(0);
      expect(editedText.text, contains(replacement.substring(2)));

      final preview = await backend.previewBytes();
      expect(preview, hasLength(greaterThan(100)));
      expect(String.fromCharCodes(preview.take(4)), '%PDF');
      await backend.openBytes(preview);
      final reopenedPreviewDomains = await backend.loadPage(0);
      expect(reopenedPreviewDomains, isNotEmpty);
      expect(
        await backend.domainText(reopenedPreviewDomains.first.index),
        startsWith(replacement.substring(2)),
      );

      await backend.saveTo(output.path);
      expect(await output.exists(), isTrue);
      expect(await output.length(), greaterThan(100));

      await backend.openBytes(await output.readAsBytes());
      expect(await backend.loadPage(0), isNotEmpty);
    },
    skip: !Platform.isMacOS || sourcePath == null,
  );

  test(
    'product session reads URI and internal page links',
    () async {
      final backend = NativePdfEditBackend();
      addTearDown(backend.close);
      await backend.openBytes(
        await File(
          '../pdf-rebuild/harness/assets/product-links.pdf',
        ).readAsBytes(),
      );
      final links = await backend.readPageLinks(0);
      expect(links, hasLength(2));
      expect(links[0].kind, PdfPageLinkKind.uri);
      expect(links[0].uri.toString(), 'https://example.com/pdf');
      expect(links[1].kind, PdfPageLinkKind.page);
      expect(links[1].pageIndex, 1);
      expect(links.every((link) => link.right > link.left), isTrue);
      expect(await backend.readPageLinks(1), isEmpty);
      await expectLater(backend.readPageLinks(99), throwsStateError);
    },
    skip: !Platform.isMacOS || sourcePath == null,
  );

  test(
    'pdfrx adapter reads and renders through the product session',
    () async {
      final product = PdfProductBackend();
      addTearDown(product.close);
      final source = await File(
        '../pdf-rebuild/harness/assets/product-links.pdf',
      ).readAsBytes();
      await product.open(source, 'product-links.pdf');
      final document = await ProductPdfDocument.open(
        product,
        sourceName: 'product-links',
      );
      addTearDown(document.dispose);
      expect(document.pages, hasLength(2));
      expect(document.pages.first.width, 300);
      final rawText = await document.pages.first.loadText();
      expect(rawText!.fullText, contains('Open web link'));
      expect(rawText.charRects, hasLength(rawText.fullText.length));
      final links = await document.pages.first.loadLinks();
      expect(links, hasLength(2));
      expect(links.first.url.toString(), 'https://example.com/pdf');
      expect(links.last.dest!.pageNumber, 2);
      final image = await document.pages.first.render(
        x: 10,
        y: 20,
        width: 96,
        height: 64,
        fullWidth: 300,
        fullHeight: 300,
        flags: PdfPageRenderFlags.limitedImageCache,
      );
      expect(image, isNotNull);
      expect(image!.pixels, hasLength(96 * 64 * 4));
      final decoded = await image.createImage();
      expect(decoded.width, 96);
      decoded.dispose();
      image.dispose();
      final event = document.events.first;
      document.notifyPageModified(0);
      expect(await event, isA<PdfDocumentPageStatusChangedEvent>());
      expect(await document.encodePdf(), source);
      await product.open(source, 'replacement.pdf');
      await expectLater(document.pages.first.loadText(), throwsStateError);
    },
    skip: !Platform.isMacOS || sourcePath == null,
  );
}
