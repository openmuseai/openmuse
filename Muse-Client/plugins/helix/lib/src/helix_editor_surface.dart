import 'dart:async';
import 'dart:io';
import 'dart:math' as math;

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:xterm/xterm.dart';
import 'package:xterm/src/ui/input_map.dart' show keyToTerminalKey;

import 'helix_runtime.dart';
import 'helix_preferences.dart';
import 'helix_open_trace.dart';

final class HelixEditorSurface extends StatefulWidget {
  const HelixEditorSurface({
    super.key,
    required this.runtime,
    required this.resource,
  });

  final HelixRuntimePool runtime;
  final OpenMuseResource resource;

  @override
  State<HelixEditorSurface> createState() => _HelixEditorSurfaceState();
}

final class _HelixEditorSurfaceState extends State<HelixEditorSurface> {
  final GlobalKey<TerminalViewState> _terminalKey =
      GlobalKey<TerminalViewState>();
  final FocusNode _terminalFocusNode = FocusNode();
  final FocusNode _windowsInputFocusNode = FocusNode();
  final TextEditingController _windowsInputController = TextEditingController();
  int _pointerDownButtons = 0;
  bool _clearingWindowsInput = false;

  void _flushWindowsInput() {
    if (_clearingWindowsInput || widget.runtime.isSwitching) return;
    final value = _windowsInputController.value;
    if (!value.composing.isCollapsed || value.text.isEmpty) return;
    _clearingWindowsInput = true;
    _windowsInputController.clear();
    _clearingWindowsInput = false;
    HelixOpenTrace.mark(
      'windows_text_commit',
      data: {'length': value.text.length},
    );
    widget.runtime.terminal.textInput(value.text);
  }

  bool _handleWindowsHardwareKey(KeyEvent event) {
    if (!_windowsInputFocusNode.hasPrimaryFocus) return false;
    if (widget.runtime.isSwitching) return true;
    if (_handlePlatformShortcut(_windowsInputFocusNode, event) ==
        KeyEventResult.handled) {
      return true;
    }
    if (event is! KeyDownEvent) return false;
    if (!_windowsInputController.value.composing.isCollapsed) return false;
    final logicalKey = event.logicalKey;
    if (logicalKey == LogicalKeyboardKey.space ||
        (logicalKey.keyLabel.length == 1 &&
            !HardwareKeyboard.instance.isControlPressed &&
            !HardwareKeyboard.instance.isAltPressed)) {
      return false;
    }
    final key = keyToTerminalKey(logicalKey);
    if (key == null) return false;
    return widget.runtime.terminal.keyInput(
      key,
      ctrl: HardwareKeyboard.instance.isControlPressed,
      alt: HardwareKeyboard.instance.isAltPressed,
      shift: HardwareKeyboard.instance.isShiftPressed,
    );
  }

  void _focusWindowsInput() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      setState(() {});
      _windowsInputFocusNode.requestFocus();
      scheduleMicrotask(() {
        if (mounted) {
          HelixOpenTrace.mark(
            'windows_text_focus',
            data: {'input_focus': _windowsInputFocusNode.hasFocus},
          );
        }
      });
    });
  }

  void _runShortcut(Future<void> Function() action) {
    unawaited(
      action().catchError((Object error, StackTrace _) {
        if (!mounted) return;
        ScaffoldMessenger.maybeOf(
          context,
        )?.showSnackBar(SnackBar(content: Text('$error')));
      }),
    );
  }

  Future<void> _copySelection({required bool cut}) async {
    final selected = await widget.runtime.semanticCommand('copy');
    final text = selected.text;
    if (text == null) throw StateError('没有可复制的文本选区');
    await Clipboard.setData(ClipboardData(text: text));
    if (cut) {
      await widget.runtime.semanticCommand(
        'cut',
        text: text,
        expectedRevision: selected.revision,
        expectedPath: selected.path,
      );
    }
  }

  Future<void> _pasteClipboard() async {
    final data = await Clipboard.getData(Clipboard.kTextPlain);
    if (data?.text case final text?) {
      await widget.runtime.semanticCommand('paste', text: text);
    }
  }

  KeyEventResult _handlePlatformShortcut(FocusNode _, KeyEvent event) {
    if (Platform.isWindows && event is KeyDownEvent) {
      HelixOpenTrace.mark(
        'windows_key_down',
        data: {
          'has_character': event.character?.isNotEmpty == true,
          'input_focus': _windowsInputFocusNode.hasPrimaryFocus,
          'composing': !_windowsInputController.value.composing.isCollapsed,
        },
      );
    }
    if (widget.runtime.isSwitching) return KeyEventResult.handled;
    if (widget.runtime.preferences.inputProfile !=
            HelixInputProfile.standardNonmodal ||
        event is! KeyDownEvent) {
      return KeyEventResult.ignored;
    }
    final keyboard = HardwareKeyboard.instance;
    if (Platform.isMacOS
        ? !keyboard.isMetaPressed
        : !keyboard.isControlPressed) {
      return KeyEventResult.ignored;
    }
    if (keyboard.isAltPressed) return KeyEventResult.ignored;
    final key = event.logicalKey;
    if (key == LogicalKeyboardKey.keyC) {
      _runShortcut(() => _copySelection(cut: false));
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.keyX) {
      _runShortcut(() => _copySelection(cut: true));
      return KeyEventResult.handled;
    }
    if (key == LogicalKeyboardKey.keyV) {
      _runShortcut(_pasteClipboard);
      return KeyEventResult.handled;
    }
    final String command;
    if (key == LogicalKeyboardKey.keyS) {
      command = 'save';
    } else if (key == LogicalKeyboardKey.keyF) {
      command = 'find';
    } else if (key == LogicalKeyboardKey.keyZ) {
      command = keyboard.isShiftPressed ? 'redo' : 'undo';
    } else if (!Platform.isMacOS && key == LogicalKeyboardKey.keyY) {
      command = 'redo';
    } else if (key == LogicalKeyboardKey.keyA) {
      command = 'select_all';
    } else {
      return KeyEventResult.ignored;
    }
    _runShortcut(() async {
      await widget.runtime.semanticCommand(command);
    });
    return KeyEventResult.handled;
  }

  void _navigate(_HelixNavCommand command) {
    final terminal = widget.runtime.terminal;
    switch (command) {
      case _HelixNavCommand.definition:
        terminal.keyInput(TerminalKey.f12);
      case _HelixNavCommand.references:
        terminal.keyInput(TerminalKey.f12, shift: true);
      case _HelixNavCommand.typeDefinition:
        terminal.keyInput(TerminalKey.f12, ctrl: true);
      case _HelixNavCommand.implementation:
        terminal.keyInput(TerminalKey.f12, alt: true);
      case _HelixNavCommand.back:
        terminal.keyInput(TerminalKey.keyO, ctrl: true);
      case _HelixNavCommand.forward:
        terminal.keyInput(TerminalKey.f6);
      case _HelixNavCommand.rename:
        terminal.keyInput(TerminalKey.f2);
    }
  }

  void _placeCursor(Offset globalPosition) {
    final render = _terminalKey.currentState?.renderTerminal;
    if (render == null) return;
    final local = render.globalToLocal(globalPosition);
    render.mouseEvent(
      TerminalMouseButton.left,
      TerminalMouseButtonState.down,
      local,
    );
    render.mouseEvent(
      TerminalMouseButton.left,
      TerminalMouseButtonState.up,
      local,
    );
  }

  Future<void> _showContextMenu(Offset position) async {
    final overlay = Overlay.of(context).context.findRenderObject() as RenderBox;
    final command = await showMenu<_HelixNavCommand>(
      context: context,
      position: RelativeRect.fromRect(
        Rect.fromLTWH(position.dx, position.dy, 0, 0),
        Offset.zero & overlay.size,
      ),
      items: [
        for (final value in _HelixNavCommand.values) ...[
          if (value == _HelixNavCommand.back ||
              value == _HelixNavCommand.rename)
            const PopupMenuDivider(),
          PopupMenuItem<_HelixNavCommand>(
            value: value,
            child: Row(
              children: [
                Text(value.label),
                const Spacer(),
                Text(
                  value.shortcut,
                  style: const TextStyle(color: Colors.grey, fontSize: 11),
                ),
              ],
            ),
          ),
        ],
      ],
    );
    if (command != null && mounted) _navigate(command);
  }

  @override
  void initState() {
    super.initState();
    if (Platform.isWindows) {
      _windowsInputController.addListener(_flushWindowsInput);
      HardwareKeyboard.instance.addHandler(_handleWindowsHardwareKey);
    }
    widget.runtime.onFirstOutput = (childPid) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) HelixOpenTrace.mark('first_frame', childPid: childPid);
      });
    };
    unawaited(_open());
  }

  @override
  void dispose() {
    widget.runtime.onFirstOutput = null;
    if (Platform.isWindows) {
      HardwareKeyboard.instance.removeHandler(_handleWindowsHardwareKey);
      _windowsInputController.removeListener(_flushWindowsInput);
    }
    _windowsInputController.dispose();
    _windowsInputFocusNode.dispose();
    _terminalFocusNode.dispose();
    super.dispose();
  }

  @override
  void didUpdateWidget(covariant HelixEditorSurface oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.resource.uri != widget.resource.uri) unawaited(_open());
  }

  Future<void> _open() async {
    if (!widget.resource.uri.isScheme('file')) return;
    try {
      await widget.runtime.openDocument(widget.resource.uri.toFilePath());
    } catch (_) {
      // The runtime exposes its structured failure state below.
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: widget.runtime,
      builder: (context, _) => ColoredBox(
        color: widget.runtime.preferences.terminalTheme.background,
        child: Stack(
          children: [
            Positioned.fill(
              child: AbsorbPointer(
                absorbing: widget.runtime.isSwitching,
                child: Listener(
                  onPointerDown: (event) {
                    _pointerDownButtons = event.buttons;
                    if (event.buttons == kSecondaryMouseButton) {
                      _placeCursor(event.position);
                      unawaited(_showContextMenu(event.position));
                    } else if (event.buttons == kPrimaryMouseButton) {
                      if (Platform.isWindows) {
                        _focusWindowsInput();
                      } else {
                        _terminalFocusNode.requestFocus();
                        _terminalKey.currentState?.requestKeyboard();
                      }
                    }
                  },
                  onPointerUp: (event) {
                    final primary = _pointerDownButtons == kPrimaryMouseButton;
                    _pointerDownButtons = 0;
                    if (primary &&
                        (HardwareKeyboard.instance.isMetaPressed ||
                            HardwareKeyboard.instance.isControlPressed)) {
                      unawaited(
                        Future<void>.delayed(
                          const Duration(milliseconds: 40),
                          () => _navigate(_HelixNavCommand.definition),
                        ),
                      );
                    }
                  },
                  child: TerminalView(
                    widget.runtime.terminal,
                    key: _terminalKey,
                    focusNode: _terminalFocusNode,
                    keyboardType: TextInputType.text,
                    hardwareKeyboardOnly: Platform.isWindows,
                    readOnly: Platform.isWindows,
                    alwaysShowCursor: Platform.isWindows,
                    theme: widget.runtime.preferences.terminalTheme,
                    textStyle: widget.runtime.preferences.terminalStyle,
                    keyboardAppearance: widget.runtime.preferences.dark
                        ? Brightness.dark
                        : Brightness.light,
                    onKeyEvent: _handlePlatformShortcut,
                    autofocus: true,
                  ),
                ),
              ),
            ),
            if (Platform.isWindows)
              Positioned.fill(
                child: IgnorePointer(
                  child: LayoutBuilder(
                    builder: (context, constraints) {
                      final cursor = _terminalKey.currentState?.cursorRect;
                      final left = math.max(0.0, cursor?.left ?? 0.0);
                      final top = math.max(0.0, cursor?.top ?? 0.0);
                      return Stack(
                        children: [
                          Positioned(
                            left: math.min(left, constraints.maxWidth - 1),
                            top: math.min(top, constraints.maxHeight - 1),
                            width: math.max(
                              1,
                              math.min(320, constraints.maxWidth - left),
                            ),
                            height: math.max(24, cursor?.height ?? 24),
                            child: EditableText(
                              controller: _windowsInputController,
                              focusNode: _windowsInputFocusNode,
                              style: widget.runtime.preferences.terminalStyle
                                  .toTextStyle(
                                    color: widget
                                        .runtime
                                        .preferences
                                        .terminalTheme
                                        .foreground,
                                  ),
                              cursorColor: Colors.transparent,
                              backgroundCursorColor: Colors.transparent,
                              selectionColor: Colors.transparent,
                              keyboardType: TextInputType.text,
                              autocorrect: false,
                              enableSuggestions: false,
                            ),
                          ),
                        ],
                      );
                    },
                  ),
                ),
              ),
            if (widget.runtime.isSwitching)
              Positioned.fill(
                child: ColoredBox(
                  color: widget.runtime.preferences.terminalTheme.background,
                  child: const Center(child: CircularProgressIndicator()),
                ),
              ),
            if (widget.runtime.lastError case final error?)
              Center(
                child: Container(
                  constraints: const BoxConstraints(maxWidth: 480),
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: const Color(0xff2c2023),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Text(
                    '$error',
                    style: const TextStyle(color: Color(0xffffb4ab)),
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}

enum _HelixNavCommand {
  definition('转到定义', 'F12'),
  references('查找用法', '⇧F12'),
  typeDefinition('转到类型定义', '⌃F12'),
  implementation('转到实现', '⌥F12'),
  back('后退', '⌃O'),
  forward('前进', 'F6'),
  rename('重命名', 'F2');

  const _HelixNavCommand(this.label, this.shortcut);
  final String label;
  final String shortcut;
}
