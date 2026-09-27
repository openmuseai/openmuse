import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter_pty/flutter_pty.dart';
import 'package:xterm/xterm.dart';

import 'helix_preferences.dart';
import 'helix_control_channel.dart';

export 'helix_control_channel.dart' show HelixResourceEvent, HelixCommandResult;

enum HelixRuntimeState { stopped, starting, ready, failed }

final class HelixRuntimePool extends ChangeNotifier {
  HelixRuntimePool({String? executable})
    : executable = executable ?? resolveHelixExecutable();

  final String executable;
  final Terminal _idleTerminal = Terminal(maxLines: 5000);
  final Map<String, _HelixSession> _sessions = {};
  void Function(String path)? onActiveResourceChanged;
  void Function(HelixResourceEvent event)? onResourceEvent;
  Terminal get terminal => _sessions[activePath]?.terminal ?? _idleTerminal;
  Future<void>? _starting;
  HelixRuntimeState state = HelixRuntimeState.stopped;
  Object? lastError;
  int launchCount = 0;
  int? get pid => _sessions[activePath]?.pty.pid;
  String? activePath;
  HelixPreferences preferences = const HelixPreferences();
  bool supportsNonmodal = false;
  bool _capabilityChecked = false;
  bool get nonmodalSelectable =>
      supportsNonmodal &&
      Platform.environment['OPENMUSE_EXPERIMENTAL_NONMODAL'] == '1';
  final String _configInstance = DateTime.now().microsecondsSinceEpoch
      .toString();
  String get _generatedConfigDirectory =>
      '${Directory.systemTemp.path}/openmuse-helix-$_configInstance';

  Future<void> configure(HelixPreferences value) async {
    if (!_capabilityChecked) await probeCapabilities();
    if (value.inputProfile == HelixInputProfile.standardNonmodal &&
        !nonmodalSelectable) {
      throw StateError('非模态模式尚未通过跨平台输入、剪贴板和 Host 事件门禁');
    }
    if (_sessions.isNotEmpty &&
        value.inputProfile != preferences.inputProfile) {
      throw StateError('请先关闭所有 Helix 编辑会话，再切换输入模式');
    }
    if (_sessions.isNotEmpty &&
        preferences.inputProfile == HelixInputProfile.standardNonmodal &&
        jsonEncode(value.toJson()) != jsonEncode(preferences.toJson())) {
      throw StateError('非模态会话运行期间不能热重载设置；请先关闭编辑会话');
    }
    preferences = value;
    notifyListeners();
    await _writeConfig();
    if (_sessions.isNotEmpty) {
      for (final session in _sessions.values) {
        await _sendCommand(session.pty, ':config-reload');
        await _sendCommand(session.pty, ':theme ${value.theme}');
      }
    }
  }

  Future<bool> probeCapabilities() async {
    _capabilityChecked = true;
    try {
      final result = await Process.run(executable, const ['--version']);
      supportsNonmodal =
          result.exitCode == 0 &&
          result.stdout.toString().contains('openmuse-nonmodal.3');
    } on ProcessException {
      supportsNonmodal = false;
    }
    notifyListeners();
    return supportsNonmodal;
  }

  Future<File> _writeConfig() async {
    final directory = Directory(_generatedConfigDirectory);
    await directory.create(recursive: true);
    final config = File('${directory.path}/config.toml');
    await config.writeAsString(preferences.configToml, flush: true);
    final languages = File('${directory.path}/helix/languages.toml');
    await languages.parent.create(recursive: true);
    await languages.writeAsString(preferences.languagesToml, flush: true);
    return config;
  }

  Future<void> _sendCommand(Pty pty, String command) async {
    pty.write(Uint8List.fromList(const [0x1b]));
    await Future<void>.delayed(const Duration(milliseconds: 35));
    pty.write(Uint8List.fromList(utf8.encode(command)));
    await Future<void>.delayed(const Duration(milliseconds: 35));
    pty.write(Uint8List.fromList(const [0x0d]));
  }

  Future<HelixCommandResult> semanticCommand(String name) async {
    final session = _sessions[activePath];
    if (session == null) throw StateError('Helix 尚未启动');
    return _semanticForSession(session, name);
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
    String name,
  ) async {
    final state = session.latestState;
    final channel = session.channel;
    if (state == null || channel == null) {
      throw StateError('Helix 尚未提供可信的活动文件状态');
    }
    for (var attempt = 0; attempt < 2; attempt++) {
      final observed = session.latestState ?? state;
      final result = await channel.command(
        name: name,
        path: observed.path,
        revision: observed.revision,
      );
      if (result.ok) return result;
      if (result.error == 'stale_revision' && attempt == 0) {
        continue;
      }
      throw StateError('Helix $name 失败：${result.error ?? '未知错误'}');
    }
    throw StateError('Helix $name 失败：文档版本持续变化');
  }

  Future<void> openDocument(String path) async {
    if (_sessions[path] case final session?) {
      activePath = path;
      state = HelixRuntimeState.ready;
      notifyListeners();
      if (session.reportedPath != path) {
        onActiveResourceChanged?.call(session.reportedPath);
      }
      return;
    }
    final inFlight = _starting;
    if (inFlight != null) {
      await inFlight;
      return openDocument(path);
    }
    final operation = _start(path);
    _starting = operation;
    try {
      await operation;
    } finally {
      _starting = null;
    }
  }

  Future<void> _start(String path) async {
    state = HelixRuntimeState.starting;
    activePath = path;
    lastError = null;
    notifyListeners();
    HelixControlChannel? channel;
    Pty? spawnedPty;
    try {
      final runtimePath = File(executable).parent.path;
      final environment = helixProcessEnvironment(Platform.environment);
      environment['XDG_CONFIG_HOME'] = _generatedConfigDirectory;
      if (supportsNonmodal) {
        channel = await HelixControlChannel.bind((event) {
          onResourceEvent?.call(event);
          final session = _sessions[path];
          if (event.type == 'state' && session != null) {
            session.latestState = event;
          }
          if (event.type == 'state' &&
              session != null &&
              session.reportedPath != event.path) {
            session.reportedPath = event.path;
            if (activePath == path) onActiveResourceChanged?.call(event.path);
          }
        });
        environment['OPENMUSE_HELIX_CONTROL_ADDR'] = channel.address;
        environment['OPENMUSE_HELIX_CONTROL_TOKEN'] = channel.token;
      }
      if (File(executable).existsSync() &&
          Directory('$runtimePath/runtime').existsSync()) {
        environment['HELIX_RUNTIME'] = '$runtimePath/runtime';
      }
      if (Platform.isWindows && !File(executable).existsSync()) {
        throw StateError('未找到 Windows Helix 可执行文件 hx.exe');
      }
      final config = await _writeConfig();
      // Launch with the file as an argument, as in the original self-authored
      // surface. Typing an absolute path via :open triggers Helix completion
      // on every character and exposes the command prompt in the editor.
      final pty = Pty.start(
        executable,
        arguments: ['--config', config.path, path],
        workingDirectory: File(path).parent.path,
        environment: environment,
        rows: 30,
        columns: 100,
      );
      spawnedPty = pty;
      channel?.expectedPid = pty.pid;
      final session = _HelixSession(pty, channel, path);
      session.terminal.onOutput = (data) {
        pty.write(Uint8List.fromList(utf8.encode(data)));
      };
      session.terminal.onResize = (width, height, _, _) =>
          pty.resize(height, width);
      _sessions[path] = session;
      launchCount++;
      activePath = path;
      session.output = pty.output.listen(
        (bytes) =>
            session.terminal.write(utf8.decode(bytes, allowMalformed: true)),
      );
      if (preferences.inputProfile == HelixInputProfile.standardNonmodal) {
        if (channel == null) throw StateError('非模态引擎缺少可信控制通道');
        await channel.firstState.timeout(const Duration(seconds: 8));
      }
      unawaited(
        pty.exitCode.then((code) {
          if (_sessions[path] != session) return;
          _sessions.remove(path);
          unawaited(session.output?.cancel());
          unawaited(session.channel?.close());
          if (activePath == path) {
            activePath = null;
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
    final sessions = _sessions.values.toList();
    _sessions.clear();
    for (final session in sessions) {
      session.pty.kill();
      await session.output?.cancel();
      await session.channel?.close();
    }
    activePath = null;
    state = HelixRuntimeState.stopped;
    notifyListeners();
  }

  @override
  void dispose() {
    for (final session in _sessions.values) {
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
  final Pty pty;
  final HelixControlChannel? channel;
  String reportedPath;
  HelixResourceEvent? latestState;
  final Terminal terminal = Terminal(maxLines: 5000);
  StreamSubscription<List<int>>? output;
}

/// Environment passed to flutter_pty.
///
/// The Windows backend builds a fresh environment block and only copies
/// `HOME` and `PATH` from the parent. Entries supplied here are kept, so
/// Windows must forward `SystemRoot` and `Path` or CreateProcess fails.
Map<String, String> helixProcessEnvironment(Map<String, String> parent) {
  final environment = Platform.isWindows
      ? Map<String, String>.from(parent)
      : <String, String>{};
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
