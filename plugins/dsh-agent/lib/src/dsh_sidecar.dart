import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:path/path.dart' as p;

enum DshSidecarState { stopped, configurationRequired, starting, ready, failed }

final class DshSidecarSupervisor extends ChangeNotifier {
  DshSidecarSupervisor({Map<String, String>? environment})
    : environment = environment ?? Platform.environment;

  final Map<String, String> environment;
  final String bridgeToken = List<int>.generate(
    32,
    (_) => Random.secure().nextInt(256),
  ).map((value) => value.toRadixString(16).padLeft(2, '0')).join();
  DshSidecarState state = DshSidecarState.stopped;
  Process? _process;
  Future<void>? _starting;
  Object? lastError;
  Uri? endpoint;
  int launchCount = 0;
  final List<String> logTail = [];

  String? get cliPath => resolveDshRuntime(
    executablePath: Platform.resolvedExecutable,
    environment: environment,
  ).cliPath;

  String get nodeExecutable => resolveDshRuntime(
    executablePath: Platform.resolvedExecutable,
    environment: environment,
  ).nodeExecutable;

  String? get modelCapabilitiesPatch {
    final cli = cliPath;
    if (cli == null || !cli.endsWith('bin.js')) return null;
    final nodeModules = File(cli).parent.parent.parent.parent;
    final patch = File(
      '${nodeModules.path}/dsh-model-capabilities/openmuse.patch.yml',
    );
    return patch.existsSync() ? patch.path : null;
  }

  bool get hasModelKey => (environment['DEEPSEEK_API_KEY'] ?? '').isNotEmpty;

  Future<String> probeVersion() async {
    final cli = cliPath;
    if (cli == null || cli.isEmpty) {
      throw StateError('OPENMUSE_DSH_CLI is not configured');
    }
    final command = dshCommand(cli, [
      '--version',
    ], nodeExecutable: nodeExecutable);
    final result = await Process.run(command.executable, command.arguments);
    if (result.exitCode != 0) throw StateError('${result.stderr}');
    return '${result.stdout}'.trim();
  }

  Future<void> ensureStarted() async {
    if (state == DshSidecarState.ready) return;
    final inFlight = _starting;
    if (inFlight != null) return inFlight;
    final operation = _start();
    _starting = operation;
    try {
      await operation;
    } finally {
      _starting = null;
    }
  }

  Future<void> _start() async {
    final cli = cliPath;
    if (cli == null || cli.isEmpty) {
      state = DshSidecarState.configurationRequired;
      lastError = StateError('DSH runtime 尚未安装。');
      notifyListeners();
      return;
    }
    state = DshSidecarState.starting;
    lastError = null;
    endpoint = null;
    logTail.clear();
    notifyListeners();
    try {
      final runtime = resolveDshRuntime(
        executablePath: Platform.resolvedExecutable,
        environment: environment,
      );
      final command = dshWebCommand(
        cli,
        nodeExecutable: runtime.nodeExecutable,
        patchPath: modelCapabilitiesPatch,
      );
      final reportedEndpoint = Completer<Uri>();
      final process = await Process.start(
        command.executable,
        command.arguments,
        environment: {
          ...dshLaunchEnvironment(environment, runtime.nodeExecutable),
          'OPENMUSE_DSH_BRIDGE_TOKEN': bridgeToken,
        },
        workingDirectory: dshClosureRoot(cli),
      );
      _process = process;
      launchCount++;
      _capture(process.stdout, reportedEndpoint: reportedEndpoint);
      _capture(process.stderr, reportedEndpoint: reportedEndpoint);
      unawaited(
        process.exitCode.then((code) {
          if (!reportedEndpoint.isCompleted) {
            reportedEndpoint.completeError(
              StateError('DSH exited with code $code before reporting ready'),
            );
          }
          if (_process != process) return;
          _process = null;
          endpoint = null;
          if (state != DshSidecarState.stopped) {
            state = DshSidecarState.failed;
            lastError = StateError('DSH exited with code $code');
            notifyListeners();
          }
        }),
      );
      final candidate = await reportedEndpoint.future.timeout(
        const Duration(seconds: 45),
        onTimeout: () => throw TimeoutException(
          'DSH did not report its ephemeral loopback endpoint',
        ),
      );
      await waitForHttp(candidate, const Duration(seconds: 45));
      endpoint = candidate;
      state = DshSidecarState.ready;
      notifyListeners();
    } catch (error) {
      state = DshSidecarState.failed;
      lastError = error;
      notifyListeners();
      rethrow;
    }
  }

  Future<void> stop() async {
    _process?.kill();
    _process = null;
    endpoint = null;
    state = DshSidecarState.stopped;
    notifyListeners();
  }

  void _capture(
    Stream<List<int>> stream, {
    required Completer<Uri> reportedEndpoint,
  }) {
    stream.transform(utf8.decoder).transform(const LineSplitter()).listen((
      line,
    ) {
      logTail.add(_redact(line));
      if (logTail.length > 100) logTail.removeAt(0);
      if (!reportedEndpoint.isCompleted) {
        final match = RegExp(
          r'https?://127\.0\.0\.1:[0-9]+(?:/\?[^\s]*)?',
        ).firstMatch(line);
        final value = match?.group(0);
        if (value != null) reportedEndpoint.complete(Uri.parse(value));
      }
      notifyListeners();
    });
  }

  String _redact(String value) {
    final key = environment['DEEPSEEK_API_KEY'];
    return key == null || key.isEmpty
        ? value
        : value.replaceAll(key, '<redacted>');
  }

  @override
  void dispose() {
    _process?.kill();
    super.dispose();
  }
}

final class DshRuntimeLocation {
  const DshRuntimeLocation({
    required this.cliPath,
    required this.nodeExecutable,
  });

  final String? cliPath;
  final String nodeExecutable;
}

/// Resolves the bundled DSH CLI and Node binary.
///
/// macOS packages them inside `Contents/Resources/openmuse/dsh`. Windows
/// packages them beside the executable under `openmuse/dsh`.
DshRuntimeLocation resolveDshRuntime({
  required String executablePath,
  Map<String, String> environment = const {},
  bool Function(String path)? exists,
}) {
  final check = exists ?? (String path) => File(path).existsSync();
  final explicit = environment['OPENMUSE_DSH_CLI'];
  String? cli;
  if (explicit != null && explicit.isNotEmpty) {
    cli = explicit;
  } else {
    for (final candidate in _bundledCliCandidates(executablePath)) {
      if (check(candidate)) {
        cli = candidate;
        break;
      }
    }
  }
  var node = 'node';
  for (final candidate in _bundledNodeCandidates(executablePath)) {
    if (check(candidate)) {
      node = candidate;
      break;
    }
  }
  return DshRuntimeLocation(cliPath: cli, nodeExecutable: node);
}

List<String> _bundledCliCandidates(String executablePath) {
  final executableDir = p.dirname(executablePath);
  final bundleParent = p.dirname(executableDir);
  const tail = [
    'openmuse',
    'dsh',
    'node_modules',
    '@deepseek-ai',
    'dsh',
    'lib',
    'bin.js',
  ];
  return [
    p.joinAll([bundleParent, 'Resources', ...tail]),
    p.joinAll([executableDir, ...tail]),
  ];
}

List<String> _bundledNodeCandidates(String executablePath) {
  final executableDir = p.dirname(executablePath);
  final bundleParent = p.dirname(executableDir);
  return [
    p.join(bundleParent, 'Resources', 'openmuse', 'dsh', 'node', 'bin', 'node'),
    p.join(executableDir, 'openmuse', 'dsh', 'node', 'node.exe'),
    p.join(executableDir, 'openmuse', 'dsh', 'node', 'bin', 'node.exe'),
    p.join(executableDir, 'openmuse', 'dsh', 'node', 'bin', 'node'),
  ];
}

/// Directory that contains the closure's `node_modules`, when [cliPath] is
/// inside one. DSH is started there so package-relative files resolve.
String? dshClosureRoot(String cliPath) {
  final normalized = p.normalize(cliPath);
  final marker = '${p.separator}node_modules${p.separator}';
  final index = normalized.lastIndexOf(marker);
  if (index <= 0) return null;
  return normalized.substring(0, index);
}

/// Puts the bundled Node directory first on PATH so DSH child processes find
/// the same runtime the sidecar was launched with.
Map<String, String> dshLaunchEnvironment(
  Map<String, String> environment,
  String nodeExecutable,
) {
  if (nodeExecutable == 'node') return environment;
  final directory = p.dirname(nodeExecutable);
  final updated = Map<String, String>.from(environment);
  final separator = Platform.isWindows ? ';' : ':';
  final keys = updated.keys
      .where((key) => key.toUpperCase() == 'PATH')
      .toList();
  if (keys.isEmpty) {
    updated[Platform.isWindows ? 'Path' : 'PATH'] = directory;
    return updated;
  }
  for (final key in keys) {
    final current = updated[key] ?? '';
    updated[key] = current.isEmpty ? directory : '$directory$separator$current';
  }
  return updated;
}

({String executable, List<String> arguments}) dshCommand(
  String cli,
  List<String> args, {
  String nodeExecutable = 'node',
}) {
  if (cli.endsWith('.js'))
    return (executable: nodeExecutable, arguments: [cli, ...args]);
  return (executable: cli, arguments: args);
}

({String executable, List<String> arguments}) dshWebCommand(
  String cli, {
  String nodeExecutable = 'node',
  String? patchPath,
}) => dshCommand(cli, [
  'web',
  if (patchPath != null) ...['--patch', patchPath],
  '--host',
  '127.0.0.1',
  '--port',
  '0',
  '--no-open',
], nodeExecutable: nodeExecutable);

Future<void> waitForHttp(Uri uri, Duration timeout) async {
  final deadline = DateTime.now().add(timeout);
  Object? lastError;
  while (DateTime.now().isBefore(deadline)) {
    final client = HttpClient()..connectionTimeout = const Duration(seconds: 1);
    try {
      final request = await client.getUrl(uri);
      final response = await request.close();
      await response.drain<void>();
      if (response.statusCode < 500) return;
    } catch (error) {
      lastError = error;
    } finally {
      client.close(force: true);
    }
    await Future<void>.delayed(const Duration(milliseconds: 250));
  }
  throw TimeoutException(
    'DSH did not become ready at $uri; last error: $lastError',
  );
}
