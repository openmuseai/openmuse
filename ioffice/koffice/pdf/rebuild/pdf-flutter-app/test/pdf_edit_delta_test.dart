import 'package:flutter_test/flutter_test.dart';
import 'package:pdf_rebuild_flutter/pdf_edit_backend.dart';

void main() {
  test('insertion uses native caret even when surrounding text repeats', () {
    final delta = derivePdfTextReplacement(
      currentText: 'aaaa',
      selectionStart: 2,
      selectionEnd: 2,
      targetText: 'aaaaa',
      targetCaret: 3,
    );
    expect(delta.start, 2);
    expect(delta.end, 2);
    expect(delta.text, 'a');
  });

  test('backspace and forward delete preserve the untouched suffix', () {
    final backspace = derivePdfTextReplacement(
      currentText: 'abcdef',
      selectionStart: 3,
      selectionEnd: 3,
      targetText: 'abdef',
      targetCaret: 2,
    );
    expect((backspace.start, backspace.end, backspace.text), (2, 3, ''));

    final forwardDelete = derivePdfTextReplacement(
      currentText: 'abcdef',
      selectionStart: 3,
      selectionEnd: 3,
      targetText: 'abcef',
      targetCaret: 3,
    );
    expect(
      (forwardDelete.start, forwardDelete.end, forwardDelete.text),
      (3, 4, ''),
    );
  });

  test('selected range is the primary replacement boundary', () {
    final delta = derivePdfTextReplacement(
      currentText: 'same same same',
      selectionStart: 5,
      selectionEnd: 9,
      targetText: 'same X same',
      targetCaret: 6,
    );
    expect((delta.start, delta.end, delta.text), (5, 9, 'X'));
  });
}
