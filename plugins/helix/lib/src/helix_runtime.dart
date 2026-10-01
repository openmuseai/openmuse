import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:path/path.dart' as p;
import 'package:xterm/xterm.dart';

import 'helix_preferences.dart';
import 'helix_control_channel.dart';
import 'helix_open_trace.dart';
import 'helix_pty.dart';
import 'helix_language_servers.dart';

export 'helix_control_channel.dart' show HelixResourceEvent, HelixCommandResult;

enum HelixRuntimeState { stopped, starting, ready, failed }

final class HelixRuntimePool extends ChangeNotifier {
  HelixRuntimePool({String? executable})
    : executable = executable ?? resolveHelixExecutable(),
      _bundledExecutable =
          executable == null &&
          Platform.environment['OPENMUSE_HELIX_BIN'] == null &&
          File(resolveHelixExecutable()).existsSync();

  final String executable;
  final bool _bundledExecutable;
  final Terminal _idleTerminal = Terminal(maxLines: 5000);
  final Map<String, _HelixSession> _sessions = {};
  void Function(String path)? onActiveResourceChanged;
  void Function(HelixResourceEvent event)? onResourceEvent;
  void Function(int childPid)? onFirstOutput;
  Terminal get terminal => _sessions[activePath]?.terminal ?? _idleTerminal;
  Future<void>? _starting;
  HelixRuntimeState state = HelixRuntimeState.stopped;
  bool isSwitching = false;
  Object? lastError;
  int launchCount = 0;
  int? get pid => _sessions[activePath]?.pty.pid;
  String? activePath;
  HelixPreferences preferences = const HelixPreferences();
  bool supportsNonmodal = false;
  /// Mounted workspaces the host reported. Rust projects found inside them are
  /// linked into rust-analyzer while the opened buffer sits outside every crate.
  List<String> rustWorkspaceRoots = const [];
  List<String> _linkedRustProjects = const [];
  final bool reuseSessions =
      Platform.environment['OPENMUSE_HELIX_REUSE'] != '0';
  bool _capabilityChecked = false;
  File? _configFile;
  String? _resolvedRustAnalyzer;
  bool get nonmodalSelectable => supportsNonmodal;
  final String _configInstance = DateTime.now().microsecondsSinceEpoch
      .toString();
  String get _generatedConfigDirectory =>
      '${Directory.systemTemp.path}/openmuse-helix-$_configInstance';

  File get generatedLanguagesFile =>
      File('$_generatedConfigDirectory/helix/languages.toml');

  File get logFile => File('$_generatedConfigDirectory/helix.log');

  /// Cargo manifests to hand to rust-analyzer for [documentPath]. Empty while
  /// the buffer already belongs to a crate, while LSP is disabled, or while the
  /// user configured rust-analyzer themselves — then their choice wins.
  List<String> linkedRustProjectsFor(String documentPath) {
    if (!preferences.enableLsp) return const [];
    if (p.extension(documentPath).toLowerCase() != '.rs') return const [];
    if (rustLanguageServerRoot(documentPath) != null) return const [];
    final configured = preferences.languageServerConfigPaths['rust-analyzer'];
    if (configured != null &&
        p.isAbsolute(configured) &&
        File(configured).existsSync() &&
        File(configured).readAsStringSync().trim().isNotEmpty) {
      return const [];
    }
    return discoverRustProjectManifests(rustWorkspaceRoots);
  }

  Future<void> configure(HelixPreferences value) async {
    if (!_capabilityChecked) await probeCapabilities();
    if (value.inputProfile == HelixInputProfile.standardNonmodal &&
        !nonmodalSelectable) {
      throw StateError('当前 Helix 引擎不支持 VS Code 输入模式');
    }
    if (_sessions.isNotEmpty &&
        value.inputProfile != preferences.inputProfile) {
      await _switchActiveSessions(value);
      return;
    }
    if (_sessions.isNotEmpty &&
        preferences.inputProfile == HelixInputProfile.standardNonmodal &&
        jsonEncode(value.toJson()) != jsonEncode(preferences.toJson())) {
      throw StateError('非模态会话运行期间不能热重载设置；请先关闭编辑会话');
    }
    preferences = value;
    notifyListeners();
    final configuredRustAnalyzer = value.languageServerPaths['rust-analyzer'];
    if (value.enableLsp &&
        (configuredRustAnalyzer == null ||
            isRustupProxyAnalyzer(configuredRustAnalyzer))) {
      _resolvedRustAnalyzer = await resolveRustAnalyzerExecutable();
    }
    await _writeConfig();
    if (_sessions.isNotEmpty) {
      for (final session in _sessions.values) {
        await _sendCommand(session.pty, ':config-reload');
        await _sendCommand(session.pty, ':theme ${value.theme}');
        if ((session.latestState?.path ?? session.reportedPath)
            .toLowerCase()
            .endsWith('.rs')) {
          await _sendCommand(session.pty, ':lsp-restart rust-analyzer');
        }
      }
    }
  }

  Future<void> _switchActiveSessions(HelixPreferences next) async {
    final previous = preferences;
    final paths = _sessions.keys.toList(growable: false);
    final previouslyActive = activePath;
    for (final session in _sessions.values.toList(growable: false)) {
      final channel = session.channel;
      if (channel == null) throw StateError('当前 Helix 会话不支持安全切换输入模式');
      await channel.firstState.timeout(const Duration(seconds: 8));
      await _semanticForSession(session, 'prepare_switch');
    }

    await stop();
    try {
      preferences = next;
      await _writeConfig();
      await _reopenSessions(paths, previouslyActive);
      notifyListeners();
    } catch (error) {
      await stop();
      preferences = previous;
      await _writeConfig();
      try {
        await _reopenSessions(paths, previouslyActive);
      } catch (restoreError) {
        lastError = StateError('模式切换失败，旧模式也未能重新启动：$restoreError');
        notifyListeners();
      }
      rethrow;
    }
  }

  Future<void> _reopenSessions(List<String> paths, String? selected) async {
    for (final path in paths) {
      await openDocument(path);
    }
    if (selected != null && _sessions.containsKey(selected)) {
      activePath = selected;
      state = HelixRuntimeState.ready;
      notifyListeners();
    }
  }

  Future<bool> probeCapabilities() async {
    _capabilityChecked = true;
    // The bundled fork is pinned by the application build. Starting another
    // Windows process just to ask for its version delays the first editor.
    if (_bundledExecutable &&
        Platform.environment['OPENMUSE_HELIX_FORCE_PROBE'] != '1') {
      supportsNonmodal = true;
      notifyListeners();
      return true;
    }
    try {
      final result = await Process.run(executable, const ['--version']);
      supportsNonmodal =
          result.exitCode == 0 &&
          result.stdout.toString().contains('openmuse-nonmodal.4');
    } on ProcessException {
      supportsNonmodal = false;
    }
    notifyListeners();
    return supportsNonmodal;
  }

  Future<File> _writeConfig({List<String> rustLinkedProjects = const []}) async {
    final directory = Directory(_generatedConfigDirectory);
    await directory.create(recursive: true);
    final config = File('${directory.path}/config.toml');
    await config.writeAsString(preferences.configToml, flush: true);
    final languages = File('${directory.path}/helix/languages.toml');
    await languages.parent.create(recursive: true);
    final serverPaths = {...preferences.languageServerPaths};
    final configuredRustAnalyzer = serverPaths['rust-analyzer'];
    if (configuredRustAnalyzer == null ||
        isRustupProxyAnalyzer(configuredRustAnalyzer)) {
      if (_resolvedRustAnalyzer != null) {
        serverPaths['rust-analyzer'] = _resolvedRustAnalyzer!;
      } else {
        serverPaths.remove('rust-analyzer');
      }
    }
    await languages.writeAsString(
      preferences
          .copyWith(languageServerPaths: serverPaths)
          .languagesTomlFor(rustLinkedProjects: rustLinkedProjects),
      flush: true,
    );
    _linkedRustProjects = rustLinkedProjects;
    _configFile = config;
    return config;
  }

  Future<void> _sendCommand(HelixPty pty, String command) async {
    pty.write(Uint8List.fromList(const [0x1b]));
    await Future<void>.delayed(const Duration(milliseconds: 35));
    pty.write(Uint8List.fromList(utf8.encode(command)));
    await Future<void>.delayed(const Duration(milliseconds: 35));
    pty.write(Uint8List.fromList(const [0x0d]));
  }

  Future<HelixCommandResult> semanticCommand(
    String name, {
    String? text,
    int? expectedRevision,
    String? expectedPath,
  }) async {
    final session = _sessions[activePath];
    if (session == null) throw StateError('Helix 尚未启动');
    return _semanticForSession(
      session,
      name,
      text: text,
      expectedRevision: expectedRevision,
      expectedPath: expectedPath,
    );
  }

  Future<void> flushResource(String path) async {
    _HelixSession? target;
    for (final entry in _sessions.entries) {
      if (entry.key == path || entry.value.latestState?.path == path) {
        target = entry.value;
        break;
      }
    }
    if (target == null) return;
    if (target.latestState?.path != path) {
      throw StateError('Helix 的 $path buffer 不在活动视图，无法安全保存后比较');
    }
    await _semanticForSession(target, 'flush');
  }

  Future<HelixCommandResult> _semanticForSession(
    _HelixSession session,
    String name, {
    String? text,
    int? expectedRevision,
    String? expectedPath,
  }) async {
    final state = session.latestState;
    final channel = session.channel;
    if (state == null || channel == null) {
      throw StateError('Helix 尚未提供可信的活动文件状态');
    }
    for (var attempt = 0; attempt < 2; attempt++) {
      final observed = session.latestState ?? state;
      if (expectedPath != null && observed.path != expectedPath) {
        throw StateError('Helix 活动文件已变化，请重试');
      }
      if (expectedRevision != null && observed.revision != expectedRevision) {
        throw StateError('Helix 选区已变化，请重试');
      }
      final result = await channel.command(
        name: name,
        path: observed.path,
        revision: expectedRevision ?? observed.revision,
        text: text,
      );
      if (result.ok) return result;
      if (result.error == 'stale_revision' &&
          attempt == 0 &&
          expectedRevision == null) {
        continue;
      }
      throw StateError('Helix $name 失败：${result.error ?? '未知错误'}');
    }
    throw StateError('Helix $name 失败：文档版本持续变化');
  }

  Future<void> openDocument(String path) async {
    final inFlight = _starting;
    if (inFlight != null) {
      await inFlight;
      return openDocument(path);
    }
    final session =
        _sessions[path] ??
        (reuseSessions && supportsNonmodal && _sessions.isNotEmpty
            ? _sessions.values.first
            : null);
    final operation = session == null ? _start(path) : _switchTo(session, path);
    _starting = operation;
    try {
      await operation;
    } finally {
      _starting = null;
    }
  }

  Future<void> _switchTo(_HelixSession session, String path) async {
    final watch = Stopwatch()..start();
    HelixOpenTrace.mark('switch_begin', childPid: session.pty.pid);
    final previousPath = activePath;
    _sessions[path] = session;
    activePath = path;
    lastError = null;
    isSwitching = session.latestState?.path != path;
    notifyListeners();
    try {
      if (session.latestState?.path != path) {
        final channel = session.channel;
        if (channel == null) {
          throw StateError('Helix 控制通道尚未就绪');
        }
        // A modal session also publishes state through the fork's authenticated
        // channel. Wait for it before sending the first buffer switch.
        await channel.firstState.timeout(const Duration(seconds: 8));
        for (var attempt = 0; attempt < 2; attempt++) {
          final observed = session.latestState;
          if (observed == null) throw StateError('Helix 尚未提供活动文件状态');
          if (observed.path == path) break;
          session.expectedPath = path;
          session.switchWatch = watch;
          final result = await channel.command(
            name: 'open',
            path: path,
            revision: observed.revision,
          );
          if (result.ok) break;
          if (result.error == 'stale_revision' && attempt == 0) continue;
          throw StateError('Helix 打开文件失败：${result.error ?? '未知错误'}');
        }
      }
      session.reportedPath = path;
      session.expectedPath = null;
      // A reused session keeps the configuration it launched with, and Helix
      // only reads `languages.toml` at startup or on `:config-reload`. Switching
      // into a Rust buffer that lives outside every crate therefore has to
      // re-issue the generated `linkedProjects` and restart the server.
      final linkedProjects = linkedRustProjectsFor(path);
      if (!listEquals(linkedProjects, _linkedRustProjects)) {
        await _writeConfig(rustLinkedProjects: linkedProjects);
        await _sendCommand(session.pty, ':config-reload');
        if (p.extension(path).toLowerCase() == '.rs') {
          await _sendCommand(session.pty, ':lsp-restart rust-analyzer');
        }
      }
      state = HelixRuntimeState.ready;
      notifyListeners();
      HelixOpenTrace.mark(
        'switch_ready',
        elapsedMs: watch.elapsedMilliseconds,
        childPid: session.pty.pid,
      );
    } catch (error) {
      isSwitching = false;
      session.expectedPath = null;
      session.switchWatch = null;
      session.awaitingSwitchOutput = false;
      if (previousPath != null) activePath = previousPath;
      if (_sessions[path] == session && path != previousPath)
        _sessions.remove(path);
      lastError = error;
      notifyListeners();
      rethrow;
    }
  }

  Future<void> _start(String path) async {
    final watch = Stopwatch()..start();
    HelixOpenTrace.mark('open_begin');
    state = HelixRuntimeState.starting;
    isSwitching = false;
    activePath = path;
    lastError = null;
    notifyListeners();
    HelixControlChannel? channel;
    HelixPty? spawnedPty;
    var sawState = false;
    try {
      final runtimePath = File(executable).parent.path;
      final environment = helixProcessEnvironment(
        Platform.environment,
        configHome: _generatedConfigDirectory,
      );
      if (supportsNonmodal) {
        channel = await HelixControlChannel.bind((event) {
          if (event.type == 'state' && !sawState) {
            sawState = true;
            HelixOpenTrace.mark(
              'first_state',
              elapsedMs: watch.elapsedMilliseconds,
            );
          }
          onResourceEvent?.call(event);
          final session = _sessions[path];
          if (event.type == 'state' && session != null) {
            session.latestState = event;
          }
          if (event.type == 'state' &&
              session != null &&
              session.reportedPath != event.path) {
            session.reportedPath = event.path;
            if (session.expectedPath == event.path) {
              session.expectedPath = null;
              session.awaitingSwitchOutput = true;
            } else if (_sessions[activePath] == session) {
              onActiveResourceChanged?.call(event.path);
            }
          }
        });
        environment['OPENMUSE_HELIX_CONTROL_ADDR'] = channel.address;
        environment['OPENMUSE_HELIX_CONTROL_TOKEN'] = channel.token;
        HelixOpenTrace.mark(
          'control_bound',
          elapsedMs: watch.elapsedMilliseconds,
        );
      }
      if (File(executable).existsSync() &&
          Directory('$runtimePath/runtime').existsSync()) {
        environment['HELIX_RUNTIME'] = '$runtimePath/runtime';
      }
      if (Platform.isWindows && !File(executable).existsSync()) {
        throw StateError('未找到 Windows Helix 可执行文件 hx.exe');
      }
      final linkedProjects = linkedRustProjectsFor(path);
      final config =
          _configFile != null &&
              listEquals(linkedProjects, _linkedRustProjects)
          ? _configFile!
          : await _writeConfig(rustLinkedProjects: linkedProjects);
      HelixOpenTrace.mark(
        'config_written',
        elapsedMs: watch.elapsedMilliseconds,
      );
      // Launch with the file as an argument, as in the original self-authored
      // surface. Typing an absolute path via :open triggers Helix completion
      // on every character and exposes the command prompt in the editor.
      HelixOpenTrace.mark(
        'pty_launch_dispatched',
        elapsedMs: watch.elapsedMilliseconds,
      );
      final pty = await HelixPty.start(
        executable,
        arguments: ['--config', config.path, '--log', logFile.path, path],
        workingDirectory: File(path).parent.path,
        environment: environment,
        rows: 30,
        columns: 100,
      );
      spawnedPty = pty;
      HelixOpenTrace.mark(
        'pty_started',
        elapsedMs: watch.elapsedMilliseconds,
        childPid: pty.pid,
      );
      channel?.expectedPid = pty.pid;
      final session = _HelixSession(pty, channel, path);
      session.terminal.onOutput = (data) {
        HelixOpenTrace.mark('terminal_input', data: {
          'length': data.runes.length,
          'printable': data.runes.every((code) => code >= 32 && code != 127),
        });
        pty.write(Uint8List.fromList(utf8.encode(data)));
      };
      session.terminal.onResize = (width, height, _, _) =>
          pty.resize(height, width);
      _sessions[path] = session;
      launchCount++;
      activePath = path;
      var sawOutput = false;
      session.output = pty.output.listen((bytes) {
        final first = !sawOutput;
        if (first) {
          sawOutput = true;
          HelixOpenTrace.mark(
            'first_output',
            elapsedMs: watch.elapsedMilliseconds,
            childPid: pty.pid,
          );
        }
        session.terminal.write(utf8.decode(bytes, allowMalformed: true));
        if (first) onFirstOutput?.call(pty.pid);
        if (session.awaitingSwitchOutput) {
          session.awaitingSwitchOutput = false;
          isSwitching = false;
          HelixOpenTrace.mark(
            'switch_output',
            elapsedMs: session.switchWatch?.elapsedMilliseconds,
            childPid: pty.pid,
          );
          session.switchWatch = null;
          notifyListeners();
          onFirstOutput?.call(pty.pid);
        }
      });
      if (preferences.inputProfile == HelixInputProfile.standardNonmodal) {
        if (channel == null) throw StateError('非模态引擎缺少可信控制通道');
        await channel.firstState.timeout(const Duration(seconds: 8));
      }
      unawaited(
        pty.exitCode.then((code) {
          if (_sessions[path] != session) return;
          _sessions.removeWhere((_, value) => identical(value, session));
          unawaited(session.output?.cancel());
          unawaited(session.channel?.close());
          if (activePath != null && _sessions[activePath] == null) {
            activePath = null;
            isSwitching = false;
            state = code == 0
                ? HelixRuntimeState.stopped
                : HelixRuntimeState.failed;
            if (code != 0)
              lastError = StateError('Helix exited with code $code');
            notifyListeners();
          }
        }),
      );
      state = HelixRuntimeState.ready;
      HelixOpenTrace.mark(
        'open_ready',
        elapsedMs: watch.elapsedMilliseconds,
        childPid: pty.pid,
      );
      notifyListeners();
    } catch (error) {
      final failed = _sessions.remove(path);
      spawnedPty?.kill();
      await failed?.output?.cancel();
      await channel?.close();
      state = HelixRuntimeState.failed;
      lastError = error;
      notifyListeners();
      rethrow;
    }
  }

  Future<void> stop() async {
    final sessions = _sessions.values.toSet();
    _sessions.clear();
    for (final session in sessions) {
      session.pty.kill();
      await session.output?.cancel();
      await session.channel?.close();
    }
    activePath = null;
    isSwitching = false;
    state = HelixRuntimeState.stopped;
    notifyListeners();
  }

  @override
  void dispose() {
    for (final session in _sessions.values.toSet()) {
      session.pty.kill();
      unawaited(session.output?.cancel());
      unawaited(session.channel?.close());
    }
    _sessions.clear();
    super.dispose();
  }
}

final class _HelixSession {
  _HelixSession(this.pty, this.channel, this.reportedPath);
  final HelixPty pty;
  final HelixControlChannel? channel;
  String reportedPath;
  String? expectedPath;
  Stopwatch? switchWatch;
  bool awaitingSwitchOutput = false;
  HelixResourceEvent? latestState;
  final Terminal terminal = Terminal(maxLines: 5000);
  StreamSubscription<List<int>>? output;
}

/// Environment passed to flutter_pty.
///
/// The Windows backend builds a fresh environment block and only copies
/// `HOME` and `PATH` from the parent. Entries supplied here are kept, so
/// Windows must forward `SystemRoot` and `Path` or CreateProcess fails.
Map<String, String> helixProcessEnvironment(
  Map<String, String> parent, {
  String? configHome,
}) {
  final environment = Platform.isWindows
      ? Map<String, String>.from(parent)
      : <String, String>{};
  if (configHome != null) {
    environment['XDG_CONFIG_HOME'] = configHome;
    // etcetera's Windows strategy reads APPDATA, not XDG_CONFIG_HOME.
    // Helix loads languages.toml from config_dir() independently of --config.
    if (Platform.isWindows) environment['APPDATA'] = configHome;
  }
  return environment;
}

String resolveHelixExecutable() {
  final override = Platform.environment['OPENMUSE_HELIX_BIN'];
  if (override != null && override.isNotEmpty) return override;
  final executableDirectory = File(Platform.resolvedExecutable).parent.path;
  final relative = Platform.isMacOS
      ? '../Frameworks/App.framework/Resources/flutter_assets/'
            'packages/openmuse_helix_plugin/assets/engines/helix/hx'
      : Platform.isWindows
      ? 'data/flutter_assets/packages/openmuse_helix_plugin/'
            'assets/engines/helix/hx.exe'
      : 'data/flutter_assets/packages/openmuse_helix_plugin/'
            'assets/engines/helix/hx';
  final bundled = File('$executableDirectory/$relative').absolute.path;
  return File(bundled).existsSync() ? bundled : 'hx';
}
