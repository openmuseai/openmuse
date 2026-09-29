import 'dart:async';
import 'dart:io';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:xterm/xterm.dart';

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
  int _pointerDownButtons = 0;

  KeyEventResult _handlePlatformShortcut(FocusNode _, KeyEvent event) {
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
    final key = event.logicalKey;
    final String? command;
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
    unawaited(
      widget.runtime
          .semanticCommand(command)
          .then<void>(
            (_) {},
            onError: (Object error, StackTrace _) {
              if (!mounted) return;
              ScaffoldMessenger.maybeOf(
                context,
              )?.showSnackBar(SnackBar(content: Text('$error')));
            },
          ),
    );
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
