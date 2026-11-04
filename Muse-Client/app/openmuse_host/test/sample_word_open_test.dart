import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_word_product/openmuse_word_product.dart';
import 'package:word_editor/word_editor.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('sample DOCX opens in the signed Word product engine', (
    tester,
  ) async {
    final samplePath = Platform.environment['OPENMUSE_SAMPLE_DOCX']!;
    final bundlePath = Platform.environment['WORD_STACK_BUNDLE']!;
    final keyPath = Platform.environment['WORD_STACK_PUBLIC_KEY']!;
    final keyId = Platform.environment['WORD_STACK_KEY_ID']!;
    await tester.runAsync(() async {
      final product = await VerifiedWordProduct.loadBundle(
        bundle: await File(bundlePath).readAsBytes(),
        trustedPublicKeyPem: await File(keyPath).readAsString(),
        expectedKeyId: keyId,
      );
      expect(product.definition.ribbon.tabs, isNotEmpty);
      final editor = WordEditorController();
      try {
        await editor.init();
        expect(editor.engine.ffiReady, isTrue, reason: editor.status);
        await editor.openBytes(
          await File(samplePath).readAsBytes(),
          name: 'sample.docx',
        );
        expect(editor.envelope, isNotNull, reason: editor.status);
        expect(editor.pageCount, greaterThan(0));
        expect(editor.envelope!.commands, isNotEmpty);
      } finally {
        editor.dispose();
      }
    });
  });
}
