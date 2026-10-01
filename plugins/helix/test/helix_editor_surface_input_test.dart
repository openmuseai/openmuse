import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';
import 'package:openmuse_helix_plugin/src/helix_editor_surface.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:xterm/xterm.dart';

void main() {
  testWidgets('Windows editor commits Latin text and completed IME text', (
    tester,
  ) async {
    if (!Platform.isWindows) return;
    final runtime = HelixRuntimePool(executable: '/unused');
    addTearDown(runtime.dispose);
    final output = StringBuffer();
    runtime.terminal.onOutput = output.write;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: HelixEditorSurface(
            runtime: runtime,
            resource: OpenMuseResource(
              uri: Uri.parse('untitled:input.txt'),
              displayName: 'input.txt',
            ),
          ),
        ),
      ),
    );

    expect(
      tester.widget<TerminalView>(find.byType(TerminalView)).readOnly,
      true,
    );
    expect(
      tester.widget<TerminalView>(find.byType(TerminalView)).alwaysShowCursor,
      true,
    );
    await tester.tap(find.byType(TerminalView));
    await tester.pump();
    await tester.pump();
    expect(
      tester.widget<EditableText>(find.byType(EditableText)).focusNode.hasFocus,
      isTrue,
    );
    output.clear();

    tester.testTextInput.updateEditingValue(
      const TextEditingValue(
        text: 'a',
        selection: TextSelection.collapsed(offset: 1),
      ),
    );
    await tester.pump();
    expect(output.toString(), 'a');

    tester.testTextInput.updateEditingValue(
      const TextEditingValue(
        text: 'ni',
        selection: TextSelection.collapsed(offset: 2),
        composing: TextRange(start: 0, end: 2),
      ),
    );
    await tester.pump();
    expect(output.toString(), 'a');
    tester.testTextInput.updateEditingValue(
      const TextEditingValue(
        text: '你',
        selection: TextSelection.collapsed(offset: 1),
      ),
    );
    await tester.pump();
    expect(output.toString(), 'a你');
    await tester.pump(const Duration(milliseconds: 301));
  });
}
