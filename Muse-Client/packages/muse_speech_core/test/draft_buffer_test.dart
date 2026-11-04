import 'package:muse_speech_core/muse_speech_core.dart';
import 'package:test/test.dart';

void main() {
  test('partial hypotheses replace one temporary segment', () {
    final buffer = SpeechDraftBuffer.begin(
      text: '请帮我',
      selectionStart: 3,
      selectionEnd: 3,
    );

    expect(buffer.applyPartial('打开').text, '请帮我打开');
    expect(buffer.applyPartial('打开文件').text, '请帮我打开文件');
    expect(buffer.applyFinal('打开 README').text, '请帮我打开 README');
  });

  test('latin words retain a separator at an insertion point', () {
    final buffer = SpeechDraftBuffer.begin(
      text: 'Please',
      selectionStart: 6,
      selectionEnd: 6,
    );

    expect(buffer.applyFinal('open README').text, 'Please open README');
  });

  test('cancel restores text and selection', () {
    final buffer = SpeechDraftBuffer.begin(
      text: 'OpenMuse',
      selectionStart: 0,
      selectionEnd: 4,
    );
    buffer.applyPartial('新的');

    final restored = buffer.cancel();
    expect(restored.text, 'OpenMuse');
    expect(restored.selectionStart, 0);
    expect(restored.selectionEnd, 4);
  });
}
