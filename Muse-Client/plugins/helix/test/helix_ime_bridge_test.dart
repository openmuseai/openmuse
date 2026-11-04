import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:xterm/xterm.dart';

void main() {
  testWidgets('terminal sends committed Chinese IME text only once', (
    tester,
  ) async {
    final terminal = Terminal();
    final output = StringBuffer();
    terminal.onOutput = output.write;
    final focus = FocusNode();
    addTearDown(focus.dispose);
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: TerminalView(terminal, focusNode: focus, autofocus: true),
        ),
      ),
    );
    await tester.pump();
    focus.requestFocus();
    await tester.pump();
    expect(tester.testTextInput.isRegistered, isTrue);

    tester.testTextInput.updateEditingValue(
      const TextEditingValue(
        text: 'ni',
        selection: TextSelection.collapsed(offset: 2),
        composing: TextRange(start: 0, end: 2),
      ),
    );
    await tester.pump();
    expect(output.toString(), isEmpty);

    tester.testTextInput.updateEditingValue(
      const TextEditingValue(
        text: '你',
        selection: TextSelection.collapsed(offset: 1),
      ),
    );
    await tester.pump();
    expect(output.toString(), '你');
  });
}
