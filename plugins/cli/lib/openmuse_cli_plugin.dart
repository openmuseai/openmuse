import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_pty/flutter_pty.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:xterm/xterm.dart';
import 'package:xterm/src/ui/input_map.dart' show keyToTerminalKey;

final class OpenMuseCliPlugin implements OpenMusePlugin {
  String _workingDirectory = Directory.current.path;

  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'com.openmuse.cli',
    name: 'OpenMuse CLI',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {'workspace.context.read'},
    panels: [
      OpenMusePanelContribution(
        id: 'cli.console',
        region: OpenMuseSurfaceRegion.bottomPanel,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    try {
      final snapshot = await context.executeHostCommand(
        'workspace.snapshot',
        null,
      );
      if (snapshot is Map && snapshot['activeMountPath'] is String) {
        final candidate = snapshot['activeMountPath'] as String;
        if (Directory(candidate).existsSync()) _workingDirectory = candidate;
      }
    } on Object {
      // The console remains usable when the workspace service is unavailable.
    }
  }

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const Center(child: Text('CLI 插件不编辑文件'));

  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      panelId == 'cli.console'
      ? OpenMuseCliConsole(workingDirectory: _workingDirectory)
      : null;
}

@immutable
final class OpenMuseShellProfile {
  const OpenMuseShellProfile(this.name, this.executable, this.arguments);

  final String name;
  final String executable;
  final List<String> arguments;
}

List<OpenMuseShellProfile> availableOpenMuseShells() {
  if (Platform.isWindows) {
    var hasPwsh = false;
    try {
      hasPwsh = Process.runSync('where.exe', ['pwsh.exe']).exitCode == 0;
    } on ProcessException {
      // Windows PowerShell remains available as the system fallback.
    }
    return [
      if (hasPwsh) const OpenMuseShellProfile('PowerShell 7', 'pwsh.exe', []),
      OpenMuseShellProfile('Windows PowerShell', 'powershell.exe', []),
    ];
  }
  if (Platform.isMacOS) {
    return const [
      OpenMuseShellProfile('zsh', '/bin/zsh', ['-l']),
      OpenMuseShellProfile('bash', '/bin/bash', ['-l']),
    ];
  }
  return const [
    OpenMuseShellProfile('bash', '/bin/bash', ['-l']),
  ];
}

final class OpenMuseCliConsole extends StatefulWidget {
  const OpenMuseCliConsole({
    super.key,
    required this.workingDirectory,
    this.controller,
    this.onMinimize,
  });

  final String workingDirectory;
  final OpenMuseCliController? controller;
  final VoidCallback? onMinimize;

  @override
  State<OpenMuseCliConsole> createState() => _OpenMuseCliConsoleState();
}

final class OpenMuseCliController extends ChangeNotifier {
  int _serial = 0;
  String? _directory;

  int get serial => _serial;
  String? get directory => _directory;

  void open(String directory) {
    _directory = directory;
    _serial++;
    notifyListeners();
  }
}

final class _ConsoleSession {
  _ConsoleSession(this.profile, this.pty) : terminal = Terminal(maxLines: 5000);

  final OpenMuseShellProfile profile;
  final Pty pty;
  final Terminal terminal;
  StreamSubscription<String>? output;
  int? exitCode;

  void close() {
    output?.cancel();
    pty.kill();
  }
}

final class _OpenMuseCliConsoleState extends State<OpenMuseCliConsole> {
  final List<_ConsoleSession> _sessions = [];
  late final OpenMuseShellProfile _profile = availableOpenMuseShells().first;
  int _selected = 0;
  int _lastRequest = 0;
  String? _error;

  @override
  void initState() {
    super.initState();
    widget.controller?.addListener(_handleLaunch);
  }

  @override
  void didUpdateWidget(covariant OpenMuseCliConsole oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.controller == widget.controller) return;
    oldWidget.controller?.removeListener(_handleLaunch);
    widget.controller?.addListener(_handleLaunch);
  }

  void _handleLaunch() {
    final controller = widget.controller;
    if (controller == null || controller.serial == _lastRequest) return;
    _lastRequest = controller.serial;
    _start(controller.directory);
  }

  @override
  void dispose() {
    widget.controller?.removeListener(_handleLaunch);
    for (final session in _sessions) {
      session.close();
    }
    super.dispose();
  }

  void _start([String? directory]) {
    try {
      final environment = Map<String, String>.of(Platform.environment);
      final cliBin = environment['OPENMUSE_CLI_BIN'] ?? _bundledCliBin();
      if (cliBin != null && File(cliBin).existsSync()) {
        environment['PATH'] =
            '${File(cliBin).parent.path}${Platform.isWindows ? ';' : ':'}${environment['PATH'] ?? ''}';
      }
      final pty = Pty.start(
        _profile.executable,
        arguments: _profile.arguments,
        workingDirectory: directory ?? widget.workingDirectory,
        environment: environment,
        rows: 24,
        columns: 100,
      );
      final session = _ConsoleSession(_profile, pty);
      session.terminal.onOutput = (data) => pty.write(utf8.encode(data));
      session.terminal.onResize = (columns, rows, _, _) =>
          pty.resize(rows, columns);
      session.output = const Utf8Decoder(
        allowMalformed: true,
      ).bind(pty.output).listen(session.terminal.write);
      unawaited(
        pty.exitCode.then((code) {
          session.exitCode = code;
          if (mounted) setState(() {});
        }),
      );
      setState(() {
        _sessions.add(session);
        _selected = _sessions.length - 1;
        _error = null;
      });
    } on Object catch (error) {
      setState(() => _error = error.toString());
    }
  }

  void _close(int index) {
    final session = _sessions.removeAt(index);
    session.close();
    setState(
      () => _selected = _sessions.isEmpty
          ? 0
          : _selected.clamp(0, _sessions.length - 1),
    );
  }

  @override
  Widget build(BuildContext context) => ColoredBox(
    color: const Color(0xff181818),
    child: Column(
      children: [
        SizedBox(
          height: 36,
          child: Row(
            children: [
              const SizedBox(width: 14),
              Container(
                key: const ValueKey('cli-terminal-tab'),
                height: 35,
                padding: const EdgeInsets.symmetric(horizontal: 10),
                decoration: const BoxDecoration(
                  border: Border(bottom: BorderSide(color: Color(0xff4fa9c6))),
                ),
                alignment: Alignment.center,
                child: Text(
                  _cliText('终端', 'TERMINAL'),
                  style: const TextStyle(
                    color: Color(0xffdedede),
                    fontSize: 11,
                  ),
                ),
              ),
              const SizedBox(width: 8),
              for (var index = 0; index < _sessions.length; index++)
                TextButton(
                  key: ValueKey('cli-tab-$index'),
                  onPressed: () => setState(() => _selected = index),
                  child: Text(
                    '${index + 1}${_sessions[index].exitCode == null ? '' : ' (${_sessions[index].exitCode})'}',
                    style: const TextStyle(fontSize: 11, color: Colors.white70),
                  ),
                ),
              if (_sessions.isNotEmpty)
                IconButton(
                  tooltip: _cliText('关闭终端', 'Close Terminal'),
                  icon: const Icon(Icons.close, size: 16),
                  onPressed: () => _close(_selected),
                ),
              const Spacer(),
              IconButton(
                key: const ValueKey('cli-new-terminal'),
                tooltip: _cliText('新建终端', 'New Terminal'),
                icon: const Icon(Icons.add, size: 18),
                onPressed: () => _start(),
              ),
              if (widget.onMinimize != null)
                IconButton(
                  key: const ValueKey('cli-bottom-collapse'),
                  tooltip: _cliText('最小化终端', 'Minimize Terminal'),
                  icon: const Icon(Icons.keyboard_arrow_down, size: 18),
                  onPressed: widget.onMinimize,
                ),
            ],
          ),
        ),
        Expanded(
          child: _error != null
              ? Center(
                  child: Text(
                    _error!,
                    style: const TextStyle(color: Colors.redAccent),
                  ),
                )
              : _sessions.isEmpty
              ? Center(
                  child: TextButton.icon(
                    onPressed: () => _start(),
                    icon: const Icon(Icons.add),
                    label: Text(_cliText('新建终端', 'New Terminal')),
                  ),
                )
              : IndexedStack(
                  index: _selected,
                  children: [
                    for (final session in _sessions)
                      OpenMuseCliTerminalView(
                        terminal: session.terminal,
                        active: _sessions.indexOf(session) == _selected,
                      ),
                  ],
                ),
        ),
      ],
    ),
  );
}

String _cliText(String zh, String en) =>
    WidgetsBinding.instance.platformDispatcher.locale.languageCode == 'zh'
    ? zh
    : en;

final class OpenMuseCliTerminalView extends StatefulWidget {
  const OpenMuseCliTerminalView({
    super.key,
    required this.terminal,
    this.active = true,
  });
  final Terminal terminal;
  final bool active;

  @override
  State<OpenMuseCliTerminalView> createState() => _CliTerminalViewState();
}

final class _CliTerminalViewState extends State<OpenMuseCliTerminalView> {
  final FocusNode _terminalFocus = FocusNode();
  final FocusNode _windowsInputFocus = FocusNode();
  final TextEditingController _windowsInput = TextEditingController();
  bool _clearing = false;

  @override
  void initState() {
    super.initState();
    if (Platform.isWindows) {
      _windowsInput.addListener(_sendWindowsText);
      _windowsInputFocus.onKeyEvent = _sendWindowsKey;
      _focusIfActive();
    }
  }

  @override
  void didUpdateWidget(covariant OpenMuseCliTerminalView oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (Platform.isWindows && !oldWidget.active && widget.active) {
      _focusIfActive();
    }
  }

  void _focusIfActive() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted && widget.active) _windowsInputFocus.requestFocus();
    });
  }

  void _sendWindowsText() {
    if (_clearing) return;
    final value = _windowsInput.value;
    if (!value.composing.isCollapsed || value.text.isEmpty) return;
    _clearing = true;
    _windowsInput.clear();
    _clearing = false;
    widget.terminal.textInput(value.text);
  }

  KeyEventResult _sendWindowsKey(FocusNode node, KeyEvent event) {
    if (event is! KeyDownEvent || !_windowsInput.value.composing.isCollapsed) {
      return KeyEventResult.ignored;
    }
    final logicalKey = event.logicalKey;
    if (logicalKey == LogicalKeyboardKey.space ||
        (logicalKey.keyLabel.length == 1 &&
            !HardwareKeyboard.instance.isControlPressed &&
            !HardwareKeyboard.instance.isAltPressed)) {
      return KeyEventResult.ignored;
    }
    final key = keyToTerminalKey(logicalKey);
    if (key == null) return KeyEventResult.ignored;
    return widget.terminal.keyInput(
          key,
          ctrl: HardwareKeyboard.instance.isControlPressed,
          alt: HardwareKeyboard.instance.isAltPressed,
          shift: HardwareKeyboard.instance.isShiftPressed,
        )
        ? KeyEventResult.handled
        : KeyEventResult.ignored;
  }

  @override
  void dispose() {
    _windowsInput.removeListener(_sendWindowsText);
    _windowsInput.dispose();
    _windowsInputFocus.dispose();
    _terminalFocus.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Listener(
    behavior: HitTestBehavior.translucent,
    onPointerDown: (_) {
      if (Platform.isWindows) _focusIfActive();
    },
    child: Stack(
      children: [
        Positioned.fill(
          child: TerminalView(
            widget.terminal,
            focusNode: _terminalFocus,
            autofocus: !Platform.isWindows,
            keyboardType: TextInputType.text,
            hardwareKeyboardOnly: Platform.isWindows,
            readOnly: Platform.isWindows,
            alwaysShowCursor: true,
            theme: _cliTerminalTheme,
            textStyle: const TerminalStyle(fontSize: 13, height: 1.2),
          ),
        ),
        if (Platform.isWindows)
          Positioned(
            left: 12,
            bottom: 0,
            width: 320,
            height: 24,
            child: IgnorePointer(
              child: EditableText(
                controller: _windowsInput,
                focusNode: _windowsInputFocus,
                style: const TextStyle(color: Colors.transparent, fontSize: 13),
                cursorColor: Colors.transparent,
                backgroundCursorColor: Colors.transparent,
                selectionColor: Colors.transparent,
                keyboardType: TextInputType.text,
                autocorrect: false,
                enableSuggestions: false,
              ),
            ),
          ),
      ],
    ),
  );
}

const _cliTerminalTheme = TerminalTheme(
  cursor: Color(0xffe4e4e4),
  selection: Color(0xff264f78),
  foreground: Color(0xffd4d4d4),
  background: Color(0xff181818),
  black: Color(0xff181818),
  red: Color(0xfff48771),
  green: Color(0xff89d185),
  yellow: Color(0xffdcdcaa),
  blue: Color(0xff9cdcfe),
  magenta: Color(0xffc586c0),
  cyan: Color(0xff4ec9b0),
  white: Color(0xffd4d4d4),
  brightBlack: Color(0xff808080),
  brightRed: Color(0xfff48771),
  brightGreen: Color(0xffb5cea8),
  brightYellow: Color(0xffdcdcaa),
  brightBlue: Color(0xff9cdcfe),
  brightMagenta: Color(0xffc586c0),
  brightCyan: Color(0xff4ec9b0),
  brightWhite: Color(0xffffffff),
  searchHitBackground: Color(0xffffe6a1),
  searchHitBackgroundCurrent: Color(0xffffc65e),
  searchHitForeground: Color(0xff181818),
);

String? _bundledCliBin() {
  if (!Platform.isMacOS) return null;
  final contents = File(Platform.resolvedExecutable).parent.parent;
  final candidate = File(
    '${contents.path}/Resources/openmuse/cli/bin/openmuse',
  );
  return candidate.existsSync() ? candidate.path : null;
}
