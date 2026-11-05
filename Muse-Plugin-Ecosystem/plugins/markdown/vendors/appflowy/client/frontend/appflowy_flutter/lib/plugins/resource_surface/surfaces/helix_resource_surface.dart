import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:appflowy/core/performance/muse_performance_trace.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_commands.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_install.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_settings.dart';
import 'package:appflowy/plugins/resource_surface/resource_surface_session.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_pty/flutter_pty.dart';
import 'package:path/path.dart' as p;
import 'package:xterm/xterm.dart';

class HelixResourceSurface extends StatefulWidget {
  const HelixResourceSurface({
    super.key,
    required this.file,
    this.initialLine,
    this.performanceTrace,
  });

  final File file;
  final int? initialLine;
  final MusePerformanceTrace? performanceTrace;

  @override
  State<HelixResourceSurface> createState() => _HelixResourceSurfaceState();
}

class _HelixResourceSurfaceState extends State<HelixResourceSurface> {
  late final Terminal _terminal;
  late final HelixSettingsController _settingsController;
  Pty? _pty;
  StreamSubscription<Uint8List>? _output;
  Directory? _configRoot;
  bool _booted = false;

  /// Helix paints its first frame only after the process has started, so the
  /// pane stays covered by the loading indicator until the PTY reports output.
  bool _painted = false;
  bool _firstFrameScheduled = false;
  Timer? _paintTimeout;
  String? _error;
  HelixSettings _applied = HelixSettings.defaults;
  int _lspRevision = -1;
  final GlobalKey<TerminalViewState> _terminalViewKey =
      GlobalKey<TerminalViewState>();
  int _pointerDownButtons = 0;

  @override
  void initState() {
    super.initState();
    _settingsController = getIt<HelixSettingsController>();
    _settingsController.addListener(_onSettingsChanged);
    _terminal = Terminal(maxLines: 10000);
    _terminal.onOutput = (data) {
      _pty?.write(Uint8List.fromList(utf8.encode(data)));
      if (_booted) {
        MuseResourceSurfaceSession.instance.markDirty(widget.file);
      }
    };
    _terminal.onResize = (width, height, pixelWidth, pixelHeight) {
      _pty?.resize(height, width);
    };
    MuseResourceSurfaceSession.instance.attach(
      widget.file,
      flush: _flushToOriginal,
    );
    unawaited(_boot());
  }

  Future<void> _boot() async {
    await _settingsController.ensureLoaded();
    if (!mounted) return;
    _applied = _settingsController.settings;
    try {
      final install = await HelixInstall.resolve();
      final installer = _settingsController.installer;
      await installer.ensureRoot();
      final xdgRuntime = installer.grammarRuntime;
      final runtimes = installer.grammarRuntimes(install.runtime);
      final grammarCount = await copyHelixGrammars(
        destRuntime: xdgRuntime,
        extraSearchRuntimes: [install.runtime],
      );
      final needed = helixGrammarNameForFile(widget.file.path);
      final missingHighlight = grammarCount == 0 ||
          (needed != null && !helixHasGrammar(needed, runtimes));
      // Grammar downloads and C compilation must never block a file's first
      // frame. Missing grammars remain an explicit Settings action; Helix can
      // still open and edit the file without syntax highlighting.
      _settingsController.setGrammarStatus(
        missingHighlight
            ? needed == null
                ? '未找到 grammars：打开 设置 → Plugin → 安装语法高亮'
                : '缺少 $needed grammar：打开 设置 → Plugin → 安装语法高亮'
            : grammarCount > 0
                ? '语法高亮已就绪（$grammarCount 个 tree-sitter grammars）'
                : '未找到 grammars：打开 设置 → Plugin → 安装语法高亮',
      );
      final configDir =
          await Directory.systemTemp.createTemp('muse-helix-config-');
      _configRoot = configDir;
      final overlay = File(
        p.join(install.runtime, 'themes', 'openmuse_host.toml'),
      );
      var themeName = 'openmuse_host';
      try {
        await overlay.writeAsString(_applied.overlayThemeToml);
      } on Object {
        themeName = _applied.theme;
      }
      final config = File(p.join(configDir.path, 'helix-host.toml'));
      await config.writeAsString(
        _applied.configToml.replaceFirst(
          'theme = "openmuse_host"',
          'theme = "$themeName"',
        ),
      );
      await installer.writeLanguagesToml();
      final project = helixProjectRoot(widget.file);
      final arguments = <String>[
        '--config',
        config.path,
        '--working-dir',
        project.path,
        if (widget.initialLine != null) '+${widget.initialLine}',
        widget.file.path,
      ];
      final environment = helixGrammarProcessEnvironment(
        helixRuntime: install.runtime,
        xdgConfigHome: installer.xdgConfigHome.path,
      );
      environment['PATH'] = installer.pathPrefix(
        environment['PATH'] ?? '/usr/bin:/bin',
      );
      // Starting a fresh process is faster and more predictable than typing an
      // absolute path through Helix's `:open` prompt. The latter recomputes
      // path completion for every character and was 2-4x slower in real
      // workspaces. Keep native PTY creation off the Flutter UI isolate.
      final pty = await Pty.startAsync(
        install.binary,
        arguments: arguments,
        workingDirectory: project.path,
        environment: environment,
        rows: 30,
        columns: 100,
      );
      if (!mounted) {
        pty.kill();
        return;
      }
      _pty = pty;
      _output = pty.output.listen(_onPtyData, onError: _onPtyError);
      setState(() {});
      _booted = true;
      _armPaintTimeout();
      _lspRevision = _settingsController.installer.revision;
      WidgetsBinding.instance.addPostFrameCallback((_) => _syncPtySize());
      unawaited(_enterInsertMode(pty));
    } on Object catch (error) {
      widget.performanceTrace?.fail(error);
      if (mounted) setState(() => _error = 'Helix runtime unavailable: $error');
    }
  }

  void _onPtyData(Uint8List data) {
    if (!_painted) {
      _paintTimeout?.cancel();
      _paintTimeout = null;
      if (mounted) {
        setState(() => _painted = true);
      } else {
        _painted = true;
      }
      _finishFirstInteractiveFrame();
    }
    _terminal.write(utf8.decode(data, allowMalformed: true));
  }

  void _onPtyError(Object error) {
    widget.performanceTrace?.fail(error);
    if (mounted) setState(() => _error = 'Helix PTY error: $error');
  }

  void _finishFirstInteractiveFrame() {
    final trace = widget.performanceTrace;
    if (trace == null || trace.isFinished || _firstFrameScheduled) return;
    _firstFrameScheduled = true;
    final span = trace.startSpan(
      'engine.first-interactive-frame',
      category: 'engine',
    );
    WidgetsBinding.instance.addPostFrameCallback((_) {
      span.end();
      trace.finish();
    });
  }

  /// Safety net: never leave the loading cover up if Helix stays silent.
  void _armPaintTimeout() {
    _paintTimeout?.cancel();
    _paintTimeout = Timer(const Duration(seconds: 8), () {
      if (mounted && !_painted) setState(() => _painted = true);
    });
  }

  void _onSettingsChanged() {
    final next = _settingsController.settings;
    final lspRevision = _settingsController.installer.revision;
    final reboot = next.theme != _applied.theme ||
        next.keymap != _applied.keymap ||
        next.enableLsp != _applied.enableLsp ||
        next.backgroundColor != _applied.backgroundColor ||
        lspRevision != _lspRevision;
    final appearanceChanged = next.fontFamily != _applied.fontFamily ||
        next.fontSize != _applied.fontSize;
    _applied = next;
    if ((appearanceChanged || reboot) && mounted) setState(() {});
    if (reboot && _booted) {
      unawaited(_reboot());
    }
  }

  Future<void> _reboot() async {
    await _flushToOriginal();
    _pty?.kill();
    await _output?.cancel();
    _pty = null;
    _booted = false;
    _painted = false;
    if (mounted) await _boot();
  }

  Future<void> _enterInsertMode(Pty pty) async {
    await Future<void>.delayed(const Duration(milliseconds: 280));
    if (!mounted || _pty != pty) return;
    pty.write(Uint8List.fromList(utf8.encode('i')));
  }

  void _syncPtySize() {
    final pty = _pty;
    if (!mounted || pty == null) return;
    pty.resize(_terminal.viewHeight, _terminal.viewWidth);
  }

  void _runNav(HelixNavCommand command) {
    if (_pty == null) return;
    command.sendTo(_terminal);
  }

  KeyEventResult _onTerminalKey(FocusNode node, KeyEvent event) {
    if (event is! KeyDownEvent && event is! KeyRepeatEvent) {
      return KeyEventResult.ignored;
    }
    final command = helixNavCommandForKey(event.logicalKey);
    if (command == null) return KeyEventResult.ignored;
    _runNav(command);
    return KeyEventResult.handled;
  }

  void _onPointerDown(PointerDownEvent event) {
    _pointerDownButtons = event.buttons;
    if (event.buttons == kSecondaryMouseButton) {
      _placeCursorAtGlobal(event.position);
      unawaited(_showEditorContextMenu(event.position));
    }
  }

  void _onPointerUp(PointerUpEvent event) {
    final wasPrimary = _pointerDownButtons == kPrimaryMouseButton;
    _pointerDownButtons = 0;
    if (!wasPrimary) return;
    if (HardwareKeyboard.instance.isMetaPressed ||
        HardwareKeyboard.instance.isControlPressed) {
      Future<void>.delayed(const Duration(milliseconds: 40), () {
        if (mounted) _runNav(HelixNavCommand.gotoDefinition);
      });
    }
  }

  void _placeCursorAtGlobal(Offset globalPosition) {
    final state = _terminalViewKey.currentState;
    if (state == null) return;
    try {
      final local = state.renderTerminal.globalToLocal(globalPosition);
      state.renderTerminal.mouseEvent(
        TerminalMouseButton.left,
        TerminalMouseButtonState.down,
        local,
      );
      state.renderTerminal.mouseEvent(
        TerminalMouseButton.left,
        TerminalMouseButtonState.up,
        local,
      );
    } on Object {
      // Render object may be detached during reboot.
    }
  }

  Future<void> _showEditorContextMenu(Offset globalPosition) async {
    if (!mounted) return;
    final overlay =
        Overlay.of(context).context.findRenderObject()! as RenderBox;
    final selected = await showMenu<HelixNavCommand>(
      context: context,
      position: RelativeRect.fromRect(
        Rect.fromLTWH(globalPosition.dx, globalPosition.dy, 0, 0),
        Offset.zero & overlay.size,
      ),
      items: [
        for (final command in const [
          HelixNavCommand.gotoDefinition,
          HelixNavCommand.findUsages,
          HelixNavCommand.gotoType,
          HelixNavCommand.gotoImplementation,
        ])
          _navMenuItem(command),
        const PopupMenuDivider(),
        _navMenuItem(HelixNavCommand.jumpBack),
        _navMenuItem(HelixNavCommand.jumpForward),
        const PopupMenuDivider(),
        _navMenuItem(HelixNavCommand.rename),
      ],
    );
    if (!mounted || selected == null) return;
    _runNav(selected);
  }

  PopupMenuItem<HelixNavCommand> _navMenuItem(HelixNavCommand command) {
    return PopupMenuItem(
      value: command,
      child: Row(
        children: [
          Expanded(child: Text(command.label)),
          const SizedBox(width: 16),
          Text(
            command.shortcutHint,
            style: TextStyle(
              fontSize: 12,
              color: Theme.of(context).hintColor,
            ),
          ),
        ],
      ),
    );
  }

  Future<void> _flushToOriginal() async {
    final pty = _pty;
    if (pty == null) return;
    pty.write(Uint8List.fromList(utf8.encode('\x1b:w\r')));
    await Future<void>.delayed(const Duration(milliseconds: 200));
    MuseResourceSurfaceSession.instance.clearDirty(widget.file);
  }

  @override
  void dispose() {
    _settingsController.removeListener(_onSettingsChanged);
    _paintTimeout?.cancel();
    MuseResourceSurfaceSession.instance.detach(widget.file);
    unawaited(_output?.cancel());
    final pty = _pty;
    final root = _configRoot;
    unawaited(() async {
      if (pty != null) {
        pty.write(Uint8List.fromList(utf8.encode('\x1b:w\r')));
        await Future<void>.delayed(const Duration(milliseconds: 200));
        pty.kill();
      }
      if (root != null) {
        try {
          await root.delete(recursive: true);
        } on Object {
          // Config dir is best-effort.
        }
      }
    }());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_error != null) return Center(child: Text(_error!));
    if (!_booted) {
      // Helix is spawned per file and needs a moment before it paints its first
      // frame; show progress instead of an empty editor pane in the meantime.
      return Center(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const SizedBox(
                width: 22,
                height: 22,
                child: CircularProgressIndicator(strokeWidth: 2.4),
              ),
              const SizedBox(height: 14),
              Text(
                '正在启动 Helix 编辑器…',
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      );
    }
    final settings = _applied;
    return Stack(
      children: [
        Column(
          children: [
            GestureDetector(
              onSecondaryTapDown: (details) =>
                  unawaited(_showEditorContextMenu(details.globalPosition)),
              child: Container(
                height: 32,
                color: const Color(0xFF20252E),
                padding: const EdgeInsets.symmetric(horizontal: 8),
                child: Row(
                  children: [
                    Expanded(
                      child: Text(
                        settings.keymap == HelixKeymapPreset.vscode
                            ? '⌘-点击转到定义 · 右键查找用法 · F12 定义 · ⇧F12 用法 · ⌥← 后退'
                            : '右键打开导航菜单 · Ctrl+S 保存 · Esc 进入 Helix 命令模式',
                        style: const TextStyle(
                          color: Colors.white70,
                          fontSize: 12,
                        ),
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                    _NavButton(
                      label: '定义',
                      tooltip: '转到定义 (F12)',
                      onPressed: () => _runNav(HelixNavCommand.gotoDefinition),
                    ),
                    _NavButton(
                      label: '用法',
                      tooltip: '查找用法 (Shift+F12)',
                      onPressed: () => _runNav(HelixNavCommand.findUsages),
                    ),
                    _NavButton(
                      label: '后退',
                      tooltip: '返回上一位置 (Ctrl+O / Alt+←)',
                      onPressed: () => _runNav(HelixNavCommand.jumpBack),
                    ),
                    _NavButton(
                      label: '前进',
                      tooltip: '前进到下一位置 (F6 / Alt+→)',
                      onPressed: () => _runNav(HelixNavCommand.jumpForward),
                    ),
                    Builder(
                      builder: (buttonContext) {
                        return _NavButton(
                          label: '⋯',
                          tooltip: '更多导航操作',
                          onPressed: () {
                            final box =
                                buttonContext.findRenderObject() as RenderBox?;
                            final pos = box?.localToGlobal(
                                  Offset(0, box.size.height),
                                ) ??
                                Offset.zero;
                            unawaited(_showEditorContextMenu(pos));
                          },
                        );
                      },
                    ),
                  ],
                ),
              ),
            ),
            Expanded(
              child: ColoredBox(
                color: settings.backgroundColor,
                child: SizedBox.expand(
                  child: Listener(
                    onPointerDown: _onPointerDown,
                    onPointerUp: _onPointerUp,
                    child: TerminalView(
                      _terminal,
                      key: _terminalViewKey,
                      padding: EdgeInsets.zero,
                      autofocus: true,
                      theme: settings.terminalTheme,
                      textStyle: settings.terminalStyle,
                      onKeyEvent: _onTerminalKey,
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
        // The PTY exists before Helix has drawn anything, so cover the still
        // empty terminal until the first bytes arrive instead of showing a
        // blank editor pane.
        if (!_painted)
          Positioned.fill(
            child: ColoredBox(
              color: settings.backgroundColor,
              child: Center(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    const SizedBox(
                      width: 22,
                      height: 22,
                      child: CircularProgressIndicator(strokeWidth: 2.4),
                    ),
                    const SizedBox(height: 14),
                    const Text('正在启动 Helix 编辑器…', textAlign: TextAlign.center),
                  ],
                ),
              ),
            ),
          ),
      ],
    );
  }
}

class _NavButton extends StatelessWidget {
  const _NavButton({
    required this.label,
    required this.tooltip,
    required this.onPressed,
  });

  final String label;
  final String tooltip;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 4),
      child: Tooltip(
        message: tooltip,
        child: TextButton(
          style: TextButton.styleFrom(
            foregroundColor: Colors.white,
            minimumSize: const Size(0, 24),
            tapTargetSize: MaterialTapTargetSize.shrinkWrap,
            padding: const EdgeInsets.symmetric(horizontal: 8),
            visualDensity: VisualDensity.compact,
          ),
          onPressed: onPressed,
          child: Text(label, style: const TextStyle(fontSize: 12)),
        ),
      ),
    );
  }
}
