import 'dart:io';

import 'package:flutter/services.dart';
import 'package:path/path.dart' as p;

final class HelixInstall {
  const HelixInstall(this.binary, this.runtime);

  final String binary;
  final String runtime;

  /// The engine ships `hx` on macOS/Linux and `hx.exe` on Windows.
  static const binaryNames = ['hx', 'hx.exe'];

  static File? _findBinary(Directory directory) {
    for (final name in binaryNames) {
      final candidate = File(p.join(directory.path, name));
      if (candidate.existsSync()) return candidate;
    }
    return null;
  }

  /// Where the engine assets live next to the running executable: inside the
  /// `.app` on macOS, and under `data/flutter_assets` on the desktop platforms
  /// that keep the Flutter bundle beside the binary (Windows/Linux).
  static List<Directory> _bundledEngineDirs(File executable) => [
        Directory(
          p.normalize(
            p.join(
              executable.parent.path,
              '..',
              'Frameworks',
              'App.framework',
              'Resources',
              'flutter_assets',
              'assets',
              'engines',
              'helix',
            ),
          ),
        ),
        Directory(
          p.normalize(
            p.join(
              executable.parent.path,
              'data',
              'flutter_assets',
              'assets',
              'engines',
              'helix',
            ),
          ),
        ),
      ];

  static Future<HelixInstall> resolve() async {
    final override = Platform.environment['MUSE_HELIX_BIN']?.trim();
    if (override != null && override.isNotEmpty) {
      final binary = File(override);
      if (binary.existsSync()) {
        final runtime = Platform.environment['HELIX_RUNTIME']?.trim();
        if (runtime == null ||
            runtime.isEmpty ||
            !Directory(runtime).existsSync()) {
          throw StateError('HELIX_RUNTIME_REQUIRED');
        }
        return HelixInstall(binary.path, runtime);
      }
    }

    final executable = File(Platform.resolvedExecutable).absolute;
    for (final bundled in _bundledEngineDirs(executable)) {
      final bundledBin = _findBinary(bundled);
      if (bundledBin == null) continue;
      final bundledRuntime = Directory(p.join(bundled.path, 'runtime'));
      final bundledQueries = Directory(p.join(bundledRuntime.path, 'queries'));
      if (bundledRuntime.existsSync() && bundledQueries.existsSync()) {
        return HelixInstall(bundledBin.path, bundledRuntime.path);
      }
      final runtimeArchive = await rootBundle.load(
        'assets/engines/helix/runtime.tar',
      );
      final extractionRoot = await Directory.systemTemp.createTemp(
        'openmuse-helix-runtime-',
      );
      final archive = File(p.join(extractionRoot.path, 'runtime.tar'));
      await archive.writeAsBytes(
        runtimeArchive.buffer.asUint8List(
          runtimeArchive.offsetInBytes,
          runtimeArchive.lengthInBytes,
        ),
      );
      final result = await Process.run(
        'tar',
        ['-xf', archive.path, '-C', extractionRoot.path],
      );
      if (result.exitCode != 0) {
        throw StateError('HELIX_RUNTIME_EXTRACT_FAILED: ${result.stderr}');
      }
      return HelixInstall(
        bundledBin.path,
        p.join(extractionRoot.path, 'runtime'),
      );
    }

    for (final seed in [Directory.current.absolute, executable.parent]) {
      Directory cursor = seed;
      for (var depth = 0; depth < 12; depth++) {
        final binary = _findBinary(
          Directory(
            p.join(cursor.path, 'vendors', 'helix', 'target', 'release'),
          ),
        );
        final runtime =
            Directory(p.join(cursor.path, 'vendors', 'helix', 'runtime'));
        if (binary != null && runtime.existsSync()) {
          return HelixInstall(binary.path, runtime.path);
        }
        final parent = cursor.parent;
        if (parent.path == cursor.path) break;
        cursor = parent;
      }
    }
    throw StateError(
      'Run tool/build_macos_engine_assets.sh before building the app',
    );
  }
}

Map<String, String> helixGrammarProcessEnvironment({
  required String helixRuntime,
  required String xdgConfigHome,
  Map<String, String>? base,
}) {
  final env = Map<String, String>.from(base ?? Platform.environment);
  env.remove('CARGO_MANIFEST_DIR');
  env['HELIX_RUNTIME'] = helixRuntime;
  env['XDG_CONFIG_HOME'] = xdgConfigHome;
  env['COLORTERM'] = 'truecolor';
  env['TERM'] = 'xterm-256color';
  return env;
}
