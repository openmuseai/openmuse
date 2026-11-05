import 'package:appflowy/plugins/resource_surface/helix/helix_commands.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:xterm/xterm.dart';

void main() {
  test('goto definition / usages send CSI function keys, not ESC+g', () {
    final chunks = <String>[];
    final terminal = Terminal();
    terminal.onOutput = chunks.add;

    HelixNavCommand.gotoDefinition.sendTo(terminal);
    HelixNavCommand.findUsages.sendTo(terminal);
    HelixNavCommand.jumpBack.sendTo(terminal);
    HelixNavCommand.jumpForward.sendTo(terminal);

    final output = chunks.join();
    expect(output, contains('\x1b[24~'));
    expect(output, contains('\x1b[24;2~'));
    expect(output, contains('\x0f'));
    expect(output, contains('\x1b[17~'));
    expect(output, isNot(contains('gd')));
    expect(output, isNot(contains('gr')));
  });

  test('host navigation keymap is unambiguous', () {
    expect(helixHostNavigationKeymap, contains('F12 = "goto_definition"'));
    expect(helixHostNavigationKeymap, contains('S-F12 = "goto_reference"'));
    expect(helixHostNavigationKeymap, contains('C-o = "jump_backward"'));
    expect(helixHostNavigationKeymap, contains('F6 = "jump_forward"'));
    expect(helixHostNavigationKeymap, isNot(contains('gd')));
  });

  testWidgets('F12 and Alt+Left map to nav commands', (tester) async {
    expect(
      helixNavCommandForKey(LogicalKeyboardKey.f12),
      HelixNavCommand.gotoDefinition,
    );

    await tester.sendKeyDownEvent(LogicalKeyboardKey.shiftLeft);
    expect(
      helixNavCommandForKey(LogicalKeyboardKey.f12),
      HelixNavCommand.findUsages,
    );
    await tester.sendKeyUpEvent(LogicalKeyboardKey.shiftLeft);

    await tester.sendKeyDownEvent(LogicalKeyboardKey.altLeft);
    expect(
      helixNavCommandForKey(LogicalKeyboardKey.arrowLeft),
      HelixNavCommand.jumpBack,
    );
    await tester.sendKeyUpEvent(LogicalKeyboardKey.altLeft);
  });
}
