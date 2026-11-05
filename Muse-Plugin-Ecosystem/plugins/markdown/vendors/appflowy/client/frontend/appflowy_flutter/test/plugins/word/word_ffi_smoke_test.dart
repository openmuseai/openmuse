import 'package:appflowy/plugins/word/minimal_docx.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:word_render/word_render.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  test('word_render resolves bundled FFI and layouts a minimal docx', () async {
    final engine = WordCoreEngine();
    await engine.initFfi();
    expect(
      engine.ffiReady,
      isTrue,
      reason: 'FFI source: ${engine.source}',
    );

    await engine.ensurePaintFont();
    expect(engine.paintFontReady, isTrue);

    final envelope = await engine.layoutDocument(
      buildMinimalDocx(text: 'OpenMuse Word'),
    );
    expect(envelope.ok, isTrue);
    expect(envelope.page.pageCount, greaterThan(0));
    expect(envelope.commands, isNotEmpty);

    final texts = envelope.commands.whereType<WcDrawText>().map((c) => c.text);
    expect(texts.join(), contains('OpenMuse Word'));
  });
}
