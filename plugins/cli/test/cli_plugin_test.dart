import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cli_plugin/openmuse_cli_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:xterm/xterm.dart';

void main() {
  test('terminal launch request keeps the selected workspace directory', () {
    final controller = OpenMuseCliController();
    addTearDown(controller.dispose);
    controller.open(r'D:\work\project');
    expect(controller.directory, r'D:\work\project');
    expect(controller.serial, 1);
  });

  testWidgets('CLI plugin exposes a bottom console without starting a shell', (
    tester,
  ) async {
    final plugin = OpenMuseCliPlugin();
    expect(plugin.descriptor.id, 'com.openmuse.cli');
    expect(
      plugin.descriptor.panels.single.region,
      OpenMuseSurfaceRegion.bottomPanel,
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SizedBox(
            height: 260,
            child: const OpenMuseCliConsole(workingDirectory: '/'),
          ),
        ),
      ),
    );
    expect(find.byKey(const ValueKey('cli-terminal-tab')), findsOneWidget);
    expect(find.byKey(const ValueKey('cli-new-terminal')), findsOneWidget);
    expect(find.byType(DropdownButton), findsNothing);
    expect(find.textContaining('PowerShell'), findsNothing);
  });

  testWidgets('Windows terminal commits typed text to the PTY stream', (
    tester,
  ) async {
    if (!Platform.isWindows) return;
    final terminal = Terminal();
    final output = StringBuffer();
    terminal.onOutput = output.write;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(body: OpenMuseCliTerminalView(terminal: terminal)),
      ),
    );
    await tester.pump();
    expect(find.byType(EditableText), findsOneWidget);
    await tester.tap(find.byType(TerminalView));
    await tester.pump();
    await tester.pump();
    expect(
      tester.widget<EditableText>(find.byType(EditableText)).focusNode.hasFocus,
      isTrue,
    );
    tester.testTextInput.updateEditingValue(
      const TextEditingValue(
        text: 'hello',
        selection: TextSelection.collapsed(offset: 5),
      ),
    );
    await tester.pump();
    expect(output.toString(), 'hello');
    await tester.pump(const Duration(milliseconds: 301));
  });
}
