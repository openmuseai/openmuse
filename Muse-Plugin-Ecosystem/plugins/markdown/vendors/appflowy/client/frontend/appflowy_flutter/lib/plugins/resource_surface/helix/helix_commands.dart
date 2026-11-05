import 'package:flutter/services.dart';
import 'package:xterm/xterm.dart';

/// Host-side Helix navigation. These map to function / control keys that
/// xterm encodes as CSI (or C-o as 0x0F). Do **not** send Escape+letter
/// sequences such as `ESC g d`: terminals treat that as Alt-g, so Helix
/// never enters goto-mode.
enum HelixNavCommand {
  gotoDefinition,
  findUsages,
  gotoType,
  gotoImplementation,
  jumpBack,
  jumpForward,
  rename,
}

extension HelixNavCommandX on HelixNavCommand {
  String get label => switch (this) {
        HelixNavCommand.gotoDefinition => '转到定义',
        HelixNavCommand.findUsages => '查找用法',
        HelixNavCommand.gotoType => '转到类型定义',
        HelixNavCommand.gotoImplementation => '转到实现',
        HelixNavCommand.jumpBack => '后退',
        HelixNavCommand.jumpForward => '前进',
        HelixNavCommand.rename => '重命名',
      };

  String get shortcutHint => switch (this) {
        HelixNavCommand.gotoDefinition => 'F12',
        HelixNavCommand.findUsages => '⇧F12',
        HelixNavCommand.gotoType => '⌃F12',
        HelixNavCommand.gotoImplementation => '⌥F12',
        HelixNavCommand.jumpBack => '⌃O / ⌥←',
        HelixNavCommand.jumpForward => 'F6 / ⌥→',
        HelixNavCommand.rename => 'F2',
      };

  /// Keys Helix actually receives after xterm encoding.
  void sendTo(Terminal terminal) {
    switch (this) {
      case HelixNavCommand.gotoDefinition:
        terminal.keyInput(TerminalKey.f12);
      case HelixNavCommand.findUsages:
        terminal.keyInput(TerminalKey.f12, shift: true);
      case HelixNavCommand.gotoType:
        terminal.keyInput(TerminalKey.f12, ctrl: true);
      case HelixNavCommand.gotoImplementation:
        terminal.keyInput(TerminalKey.f12, alt: true);
      case HelixNavCommand.jumpBack:
        terminal.keyInput(TerminalKey.keyO, ctrl: true);
      case HelixNavCommand.jumpForward:
        terminal.keyInput(TerminalKey.f6);
      case HelixNavCommand.rename:
        terminal.keyInput(TerminalKey.f2);
    }
  }
}

/// Match a physical key event to a nav command. Returns null if the Host
/// should let xterm handle the key.
HelixNavCommand? helixNavCommandForKey(LogicalKeyboardKey key) {
  final meta = HardwareKeyboard.instance.isMetaPressed;
  final alt = HardwareKeyboard.instance.isAltPressed;
  final ctrl = HardwareKeyboard.instance.isControlPressed;
  final shift = HardwareKeyboard.instance.isShiftPressed;

  if (key == LogicalKeyboardKey.f12) {
    if (shift) return HelixNavCommand.findUsages;
    if (ctrl) return HelixNavCommand.gotoType;
    if (alt) return HelixNavCommand.gotoImplementation;
    return HelixNavCommand.gotoDefinition;
  }
  if (key == LogicalKeyboardKey.f2 && !shift && !ctrl && !alt) {
    return HelixNavCommand.rename;
  }
  if (key == LogicalKeyboardKey.f6 && !shift && !ctrl && !alt) {
    return HelixNavCommand.jumpForward;
  }
  if (key == LogicalKeyboardKey.keyO && ctrl && !shift && !alt) {
    return HelixNavCommand.jumpBack;
  }
  // macOS xterm encodes Alt+Left as ESC+b (word-back), not CSI A-left.
  // Intercept here and send C-o instead.
  if (key == LogicalKeyboardKey.arrowLeft && (alt || (meta && alt))) {
    return HelixNavCommand.jumpBack;
  }
  if (key == LogicalKeyboardKey.arrowRight && (alt || (meta && alt))) {
    return HelixNavCommand.jumpForward;
  }
  if (key == LogicalKeyboardKey.bracketLeft && meta && !shift && !alt) {
    return HelixNavCommand.jumpBack;
  }
  if (key == LogicalKeyboardKey.bracketRight && meta && !shift && !alt) {
    return HelixNavCommand.jumpForward;
  }
  return null;
}

/// Shared Helix keymap fragment so toolbar / menu keys work in insert, normal,
/// and select. C-o is used instead of A-left as the Host-injected back key
/// because xterm on macOS rewrites Alt+Left to word-back.
const helixHostNavigationKeymap = '''
F12 = "goto_definition"
S-F12 = "goto_reference"
C-F12 = "goto_type_definition"
A-F12 = "goto_implementation"
F2 = "rename_symbol"
C-o = "jump_backward"
F6 = "jump_forward"
"A-left" = "jump_backward"
"A-right" = "jump_forward"
''';
