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
  int _generation = 0;
  Object? lastError;
  Uri? endpoint;
  int launchCount = 0;
  final List<String> logTail = [];
  Directory? _resourcePatchDirectory;
  final ValueNotifier<String?> requestedSession = ValueNotifier(null);

  String? get cliPath => resolveDshRuntime(
    executablePath: Platform.resolvedExecutable,
    environment: environment,
  ).cliPath;

  String get nodeExecutable => resolveDshRuntime(
    executablePath: Platform.resolvedExecutable,
    environment: environment,
  ).nodeExecutable;

  String? get modelCapabilitiesPatch =>
      _bundledPatch('dsh-model-capabilities/openmuse.patch.yml');

  String? get marketPatch => _bundledPatch('dshmarket/openmuse.patch.yml');

  List<String> get defaultPatchPaths => [
    for (final path in [modelCapabilitiesPatch, marketPatch])
      if (path != null) path,
  ];

  String? _bundledPatch(String relative) {
    final cli = cliPath;
    if (cli == null || !cli.endsWith('bin.js')) return null;
    final nodeModules = File(cli).parent.parent.parent.parent;
    final patch = File('${nodeModules.path}/$relative');
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
      if (identical(_starting, operation)) _starting = null;
    }
  }

  Future<void> _start() async {
    final generation = _generation;
    final cli = cliPath;
    debugPrint(
      'OpenMuse dsh: start executable=${Platform.resolvedExecutable} cli=${cli ?? 'missing'} node=$nodeExecutable',
    );
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
      final resourcePatches = <String>[];
      final resourceUrl = environment['OPENMUSE_RESOURCE_MCP_URL'];
      final resourceToken = environment['OPENMUSE_RESOURCE_MCP_TOKEN'];
      if (resourceUrl != null && resourceToken != null) {
        final uri = Uri.parse(resourceUrl);
        if (uri.scheme != 'http' || uri.host != '127.0.0.1')
          throw StateError('Resource MCP must be loopback');
        final directory = await Directory.systemTemp.createTemp(
          'openmuse-resource-mcp-',
        );
        _resourcePatchDirectory = directory;
        final patch = File(p.join(directory.path, 'resource.patch.yml'));
        await patch.writeAsString(
          jsonEncode([
            {
              'insert': [
                {
                  'id': 'openmuse-resource-mcp',
                  'name': p.join(
                    dshClosureRoot(cli)!,
                    'node_modules',
                    '@deepseek-ai',
                    'dsh-mcp-client',
                    'lib',
                    'index.js',
                  ),
                  'config': {
                    'serverName': 'resources',
                    'transport': 'streamable-http',
                    'url': resourceUrl,
                    'headers': {'Authorization': 'Bearer $resourceToken'},
                    'failOnStartupError': true,
                  },
                },
              ],
            },
          ]),
        );
        if (!Platform.isWindows)
          await Process.run('chmod', ['600', patch.path]);
        resourcePatches.add(patch.path);
      }
      final command = dshWebCommand(
        cli,
        nodeExecutable: runtime.nodeExecutable,
        patchPaths: [...defaultPatchPaths, ...resourcePatches],
      );
      final reportedEndpoint = Completer<Uri>();
      final launchEnvironment = dshLaunchEnvironment(
        environment,
        runtime.nodeExecutable,
      );
      final cliBin = _openMuseCliBin(environment);
      if (cliBin != null) {
        final key = launchEnvironment.keys.firstWhere(
          (candidate) => candidate.toUpperCase() == 'PATH',
          orElse: () => Platform.isWindows ? 'Path' : 'PATH',
        );
        launchEnvironment[key] =
            '${p.dirname(cliBin)}${Platform.isWindows ? ';' : ':'}${launchEnvironment[key] ?? ''}';
      }
      final process = await Process.start(
        command.executable,
        command.arguments,
        environment: {
          ...launchEnvironment,
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
      await waitForHttp(
        candidate,
        const Duration(seconds: 45),
        cancelled: () => generation != _generation,
      );
      if (generation != _generation) return;
      endpoint = candidate;
      state = DshSidecarState.ready;
      debugPrint('OpenMuse dsh: ready origin=${candidate.origin}');
      notifyListeners();
    } catch (error) {
      if (generation != _generation) return;
      _process?.kill();
      _process = null;
      state = DshSidecarState.failed;
      lastError = error;
      debugPrint(
        'OpenMuse dsh: failed type=${error.runtimeType} error=$error tail=${logTail.join(' | ')}',
      );
      notifyListeners();
      rethrow;
    }
  }

  Future<void> stop() async {
    _generation++;
    _starting = null;
    _process?.kill();
    _process = null;
    endpoint = null;
    final directory = _resourcePatchDirectory;
    _resourcePatchDirectory = null;
    if (directory != null && await directory.exists())
      await directory.delete(recursive: true);
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
        final value = parseDshReadyEndpoint(line);
        if (value != null) reportedEndpoint.complete(value);
      }
      notifyListeners();
    });
  }

  String _redact(String value) {
    final key = environment['DEEPSEEK_API_KEY'];
    final redacted = value.replaceAllMapped(
      RegExp(r'([?&]token=)[^\s&)]+'),
      (match) => '${match[1]}<redacted>',
    );
    return key == null || key.isEmpty
        ? redacted
        : redacted.replaceAll(key, '<redacted>');
  }

  @override
  void dispose() {
    _process?.kill();
    super.dispose();
  }
}

/// Only the authenticated readiness announcement belongs to the Web surface.
/// Model hints and other startup diagnostics also contain clean loopback URLs.
Uri? parseDshReadyEndpoint(String line) {
  final match = RegExp(
    r'dsh web: (http://127\.0\.0\.1:[0-9]+/\?[^\s)]+)',
  ).firstMatch(line);
  final uri = match == null ? null : Uri.tryParse(match.group(1)!);
  return uri?.queryParameters['token']?.isNotEmpty == true ? uri : null;
}

String? _openMuseCliBin(Map<String, String> environment) {
  final explicit = environment['OPENMUSE_CLI_BIN'];
  if (explicit != null && File(explicit).existsSync()) return explicit;
  final executable = Platform.resolvedExecutable;
  final directory = p.dirname(executable);
  final candidates = [
    if (Platform.isMacOS)
      p.join(
        p.dirname(directory),
        'Resources',
        'openmuse',
        'cli',
        'bin',
        'openmuse',
      ),
    p.join(
      directory,
      'openmuse',
      'cli',
      'bin',
      Platform.isWindows ? 'openmuse.exe' : 'openmuse',
    ),
  ];
  for (final candidate in candidates) {
    if (File(candidate).existsSync()) return candidate;
  }
  return null;
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
  List<String> patchPaths = const [],
}) => dshCommand(cli, [
  'web',
  for (final patchPath in patchPaths) ...['--patch', patchPath],
  '--host',
  '127.0.0.1',
  '--port',
  '0',
  '--no-open',
], nodeExecutable: nodeExecutable);

Future<void> waitForHttp(
  Uri uri,
  Duration timeout, {
  bool Function()? cancelled,
}) async {
  final deadline = DateTime.now().add(timeout);
  Object? lastError;
  while (DateTime.now().isBefore(deadline)) {
    if (cancelled?.call() == true) throw StateError('DSH startup superseded');
    final client = HttpClient()..connectionTimeout = const Duration(seconds: 1);
    try {
      final request = await client.getUrl(uri);
      request.followRedirects = false;
      final response = await request.close();
      await response.drain<void>();
      if (response.statusCode == HttpStatus.ok) return;
      // The launch token is exchanged for an HttpOnly cookie before redirecting
      // to a clean URL. HttpClient does not have an automatic cookie jar.
      final location = response.headers.value(HttpHeaders.locationHeader);
      if (response.statusCode >= 300 &&
          response.statusCode < 400 &&
          location != null &&
          response.cookies.isNotEmpty) {
        final target = uri.resolve(location);
        if (target.origin != uri.origin)
          throw StateError('DSH readiness redirected outside loopback origin');
        final next = await client.getUrl(target);
        next.followRedirects = false;
        next.cookies.addAll(response.cookies);
        final verified = await next.close();
        await verified.drain<void>();
        if (verified.statusCode == HttpStatus.ok) return;
      }
    } catch (error) {
      lastError = error;
    } finally {
      client.close(force: true);
    }
    await Future<void>.delayed(const Duration(milliseconds: 250));
  }
  throw TimeoutException(
    'DSH did not become ready at ${uri.origin}; last error: ${lastError.runtimeType}',
  );
}
