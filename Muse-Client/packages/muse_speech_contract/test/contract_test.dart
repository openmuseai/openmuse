import 'package:muse_speech_contract/muse_speech_contract.dart';
import 'package:test/test.dart';

void main() {
  test('context bounds and normalizes hotwords', () {
    final context = SpeechContext(
      terms: [' OpenMuse ', '', ...List.generate(80, (index) => 'term$index')],
    );

    final hotwords = context.hotwords.split('\n');
    expect(hotwords.first, 'OpenMuse');
    expect(hotwords, hasLength(64));
  });

  test('context removes line injection and bounds each hotword', () {
    final context = SpeechContext(
      terms: ['first\nsecond', List.filled(80, 'x').join()],
    );

    final hotwords = context.hotwords.split('\n');
    expect(hotwords, hasLength(2));
    expect(hotwords.first, 'first second');
    expect(hotwords.last, hasLength(64));
  });

  test('session refs are value objects', () {
    expect(
      const SpeechSessionRef('speech.1'),
      const SpeechSessionRef('speech.1'),
    );
    expect(
      const SpeechSessionRef('speech.1'),
      isNot(const SpeechSessionRef('speech.2')),
    );
  });
}
