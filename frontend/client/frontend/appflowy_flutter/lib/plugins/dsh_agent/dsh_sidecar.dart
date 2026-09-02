import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_runtime.dart';
import 'package:flutter/foundation.dart';

/// Starts the local DSH web sidecar with the Muse AppFlowy Cordis patch.
class DshSidecar {
  DshSidecar(this.controller);

  final DshAgentController controller;
  Process? _process;
  bool _starting = false;
  bool _stopping = false;
  final List<String> _logTail = <String>[];

  static DshRuntimeLayout get layout => DshRuntimeLayout.resolve();

  static String get museRoot => layout.museRoot;

  static String get dshHome => layout.dshHome;

  Future<void> ensureStarted() async {
    if (_starting) {
      while (_starting) {
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
      if (await _isReady()) {
        controller.setError(null);
        controller.setReady(true);
        controller.setLaunching(false);
      }
      return;
    }
    if (await _isReady()) {
      controller.setError(null);
      controller.setReady(true);
      controller.setLaunching(false);
      return;
    }
    _starting = true;
    controller.setLaunching(true);
    controller.setError(null);
    try {
      final key = await _apiKey();
      if (key == null || key.isEmpty) {
        throw StateError(
          'DEEPSEEK_API_KEY is missing. Enter it in the DeepSeek panel.',
        );
      }
      Directory(dshHome).createSync(recursive: true);
      _seedMuseModules(layout);
      _seedDshMarket(layout);
      _logTail.clear();
      _stopping = false;
      _process = await _spawn(key);
      var exited = false;
      unawaited(
        _process!.exitCode.then((code) {
          exited = true;
          if (_stopping || !controller.ready) return;
          final detail = _logTail.isEmpty ? '' : '\n${_logTail.join('\n')}';
          controller.setReady(false);
          controller.setError('DSH sidecar exited with $code$detail');
        }),
      );
      unawaited(
        _process!.stdout.transform(utf8.decoder).forEach((chunk) {
          _rememberLog(chunk, key);
          debugPrint('[dsh-sidecar] $chunk');
        }),
      );
      unawaited(
        _process!.stderr.transform(utf8.decoder).forEach((chunk) {
          _rememberLog(chunk, key);
          debugPrint('[dsh-sidecar:err] $chunk');
        }),
      );
      final deadline = DateTime.now().add(const Duration(seconds: 90));
      while (DateTime.now().isBefore(deadline)) {
        if (await _isReady() && await _stayedReady()) {
          controller.setError(null);
          controller.setReady(true);
          return;
        }
        if (exited) {
          final code = await _process!.exitCode;
          final detail = _logTail.isEmpty ? '' : '\n${_logTail.join('\n')}';
          throw StateError('DSH sidecar exited with $code$detail');
        }
        await Future<void>.delayed(const Duration(milliseconds: 500));
      }
      throw StateError('DSH sidecar did not become ready on ${controller.url}');
    } catch (error) {
      controller.setReady(false);
      controller.setError(error.toString());
      rethrow;
    } finally {
      _starting = false;
      controller.setLaunching(false);
    }
  }

  Future<Process> _spawn(String apiKey) async {
    final resolved = layout;
    final environment = <String, String>{
      ...Platform.environment,
      'DEEPSEEK_API_KEY': apiKey,
      'MUSE_ROOT': resolved.museRoot,
      'DSH_HOME': resolved.dshHome,
      'DSH_WEB_HOST': '127.0.0.1',
      'DSH_WEB_PORT': '3080',
      // ESM package imports resolve from the file's realpath. Muse plugins
      // must therefore live under the harness node_modules tree, not the
      // sibling Resources/muse/packages copies.
      'NODE_PATH': '${resolved.harnessDir}/node_modules',
    };
    if (resolved.bundled) {
      environment['MUSE_BUNDLE_ROOT'] = resolved.museRoot;
      final node = resolved.nodeBin;
      if (node == null) {
        throw StateError('Packed Muse runtime is missing node/bin/node');
      }
      // Finder-launched .app PATH is /usr/bin:/bin. dshmarket shells out to
      // node/corepack/pnpm when installing community plugins.
      final nodeDir = File(node).parent.path;
      final inheritedPath = environment['PATH'];
      environment['PATH'] = inheritedPath == null || inheritedPath.isEmpty
          ? nodeDir
          : '$nodeDir:$inheritedPath';
      return Process.start(
        node,
        [
          '--import',
          'tsx/esm',
          'apps/cli/src/bin.ts',
          '--profile',
          'web',
          '--patch',
          resolved.patchFile,
          '--host',
          '127.0.0.1',
          '--port',
          '3080',
        ],
        workingDirectory: resolved.harnessDir,
        environment: environment,
      );
    }
    final script = File('${resolved.museRoot}/scripts/run-dsh-appflowy.sh');
    if (!script.existsSync()) {
      throw StateError('Missing ${script.path}');
    }
    return Process.start(
      '/bin/bash',
      [script.path],
      workingDirectory: resolved.museRoot,
      environment: environment,
    );
  }

  void _seedMuseModules(DshRuntimeLayout resolved) {
    Directory? source;
    final harness = Directory('${resolved.harnessDir}/node_modules/@muse');
    final bundled = Directory('${resolved.museRoot}/packages');
    // Prefer the copy under dsh/node_modules/@muse so Node ESM can resolve
    // sibling @muse/* and @deepseek-ai/* from that tree. Linking at
    // Resources/muse/packages makes imports like @muse/plugin-kit fail, DSH
    // fail-loud exits, and the WebView is left on "Loading plugins…".
    if (harness.existsSync()) {
      source = harness;
    } else if (bundled.existsSync()) {
      source = bundled;
    }
    if (source == null) return;
    for (final destPath in [
      '${resolved.dshHome}/profiles/node_modules/@muse',
      '${resolved.dshHome}/profiles/web/node_modules/@muse',
    ]) {
      Directory(destPath).createSync(recursive: true);
      for (final entity in source.listSync()) {
        if (entity is! Directory) continue;
        final name =
            entity.uri.pathSegments.where((segment) => segment.isNotEmpty).last;
        final target = '$destPath/$name';
        _replaceWithLink(target, entity.path);
      }
    }
  }

  /// Loader baseUrl is the profile directory; ESM does not consult NODE_PATH.
  void _seedDshMarket(DshRuntimeLayout resolved) {
    final source = Directory('${resolved.harnessDir}/node_modules/dshmarket');
    if (!source.existsSync()) return;
    for (final destDir in [
      '${resolved.dshHome}/profiles/node_modules',
      '${resolved.dshHome}/profiles/web/node_modules',
    ]) {
      Directory(destDir).createSync(recursive: true);
      _replaceWithLink('$destDir/dshmarket', source.path);
    }
  }

  void _replaceWithLink(String target, String sourcePath) {
    try {
      // Always replace: a leftover directory from an older pack would
      // otherwise shadow the bundled package (WKWebView URL patch / dshmarket).
      final existing = FileSystemEntity.typeSync(target, followLinks: false);
      if (existing == FileSystemEntityType.link) {
        Link(target).deleteSync();
      } else if (existing == FileSystemEntityType.directory) {
        Directory(target).deleteSync(recursive: true);
      } else if (existing == FileSystemEntityType.file) {
        File(target).deleteSync();
      }
      Link(target).createSync(sourcePath);
    } catch (_) {}
  }

  void _rememberLog(String chunk, String apiKey) {
    for (final rawLine in const LineSplitter().convert(chunk)) {
      final line = rawLine.replaceAll(apiKey, '[REDACTED]').trim();
      if (line.isEmpty) continue;
      _logTail.add(line);
      if (_logTail.length > 40) _logTail.removeAt(0);
      _appendLogFile(line);
    }
  }

  void _appendLogFile(String line) {
    try {
      File('${DshRuntimeLayout.userMuseHome}/dsh-sidecar.log')
          .writeAsStringSync('$line\n', mode: FileMode.append);
    } catch (_) {}
  }

  /// First HTTP 200 is not "booted": Muse plugins still import after listen.
  /// A failed import fail-loud-exits a second later and leaves Loading plugins.
  Future<bool> _stayedReady() async {
    for (var i = 0; i < 8; i++) {
      await Future<void>.delayed(const Duration(milliseconds: 400));
      if (!await _isReady()) return false;
    }
    return true;
  }

  Future<bool> _isReady() async {
    try {
      final uri = Uri.parse(controller.url);
      final client = HttpClient()
        ..connectionTimeout = const Duration(seconds: 2);
      final request = await client.getUrl(uri);
      final response = await request.close();
      await response.drain<void>();
      client.close(force: true);
      return response.statusCode < 500;
    } catch (_) {
      return false;
    }
  }

  Future<String?> _apiKey() async {
    final fromEnv = Platform.environment['DEEPSEEK_API_KEY'];
    if (fromEnv != null && fromEnv.trim().isNotEmpty) return fromEnv.trim();
    final stored = File(layout.credentialsFile);
    if (stored.existsSync()) {
      for (final raw in stored.readAsLinesSync()) {
        final parsed = _parseEnvLine(raw, 'DEEPSEEK_API_KEY');
        if (parsed != null) return parsed;
      }
    }
    final sourceEnv = File('${layout.museRoot}/.env.dsh.local');
    if (!layout.bundled && sourceEnv.existsSync()) {
      for (final raw in sourceEnv.readAsLinesSync()) {
        final parsed = _parseEnvLine(raw, 'DEEPSEEK_API_KEY');
        if (parsed != null) return parsed;
      }
    }
    return null;
  }

  static String? _parseEnvLine(String raw, String name) {
    final line = raw.trim();
    if (line.isEmpty || line.startsWith('#')) return null;
    final index = line.indexOf('=');
    if (index <= 0) return null;
    if (line.substring(0, index).trim() != name) return null;
    var value = line.substring(index + 1).trim();
    if (value.length >= 2 &&
        ((value.startsWith('"') && value.endsWith('"')) ||
            (value.startsWith("'") && value.endsWith("'")))) {
      value = value.substring(1, value.length - 1);
    }
    return value.isEmpty ? null : value;
  }

  Future<void> saveApiKey(String key) async {
    final trimmed = key.trim();
    if (trimmed.isEmpty) {
      throw StateError('API key is empty');
    }
    final file = File(layout.credentialsFile);
    await file.parent.create(recursive: true);
    await file.writeAsString('DEEPSEEK_API_KEY=$trimmed\n');
    if (Platform.isMacOS || Platform.isLinux) {
      await Process.run('chmod', ['600', file.path]);
    }
  }

  bool get needsApiKey {
    final error = controller.lastError;
    return error != null && error.contains('DEEPSEEK_API_KEY');
  }

  Future<void> stop() async {
    _stopping = true;
    _process?.kill();
    _process = null;
  }
}
