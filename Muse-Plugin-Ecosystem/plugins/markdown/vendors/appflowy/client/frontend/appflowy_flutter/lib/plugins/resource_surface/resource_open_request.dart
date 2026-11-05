import 'dart:io';

import 'package:appflowy/core/performance/muse_performance_trace.dart';
import 'package:appflowy/plugins/resource_surface/engine_registry.dart';
import 'package:appflowy/plugins/resource_surface/engines/register.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_defaults.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:path/path.dart' as p;

enum MuseResourceOpenOrigin { hostPicker, dshConversation }

enum MuseLocalEngine { ioffice, helix, openFileViewer }

final class MuseResourceOpenRequest {
  const MuseResourceOpenRequest({
    required this.path,
    required this.origin,
    this.sessionCwd,
    this.line,
    this.performanceTrace,
  });

  final String path;
  final MuseResourceOpenOrigin origin;
  final String? sessionCwd;
  final int? line;
  final MusePerformanceTrace? performanceTrace;

  MuseResourceOpenRequest withPerformanceTrace(MusePerformanceTrace trace) =>
      MuseResourceOpenRequest(
        path: path,
        origin: origin,
        sessionCwd: sessionCwd,
        line: line,
        performanceTrace: trace,
      );
}

final class MuseResolvedResource {
  const MuseResolvedResource({
    required this.file,
    required this.engine,
    required this.origin,
    this.line,
    this.performanceTrace,
  });

  final File file;
  final MuseLocalEngine engine;
  final MuseResourceOpenOrigin origin;
  final int? line;
  final MusePerformanceTrace? performanceTrace;
}

final class MuseLocalResourceRouter {
  MuseLocalResourceRouter({
    MuseResourceEngineRegistry? engines,
    MuseResourceOpenDefaults? defaults,
  })  : engines =
            engines ?? _registeredEngines() ?? builtinResourceEngineRegistry(),
        defaults = defaults ?? _registeredDefaults();

  final MuseResourceEngineRegistry engines;
  final MuseResourceOpenDefaults? defaults;

  static MuseResourceEngineRegistry? _registeredEngines() {
    if (!getIt.isRegistered<MuseResourceEngineRegistry>()) return null;
    return getIt<MuseResourceEngineRegistry>();
  }

  static MuseResourceOpenDefaults? _registeredDefaults() {
    if (!getIt.isRegistered<MuseResourceOpenDefaults>()) return null;
    return getIt<MuseResourceOpenDefaults>();
  }

  Future<MuseResolvedResource> resolve(MuseResourceOpenRequest request) async {
    if (request.path.trim().isEmpty) {
      throw const MuseResourceOpenException('EMPTY_PATH');
    }
    final candidate = p.isAbsolute(request.path)
        ? File(request.path)
        : File(
            p.join(request.sessionCwd ?? Directory.current.path, request.path),
          );
    // A path that cannot be resolved (missing file, dangling link, unreadable
    // parent) is "not a file" — the same refusal the type check below reports,
    // instead of an untyped FileSystemException callers cannot map.
    final File canonical;
    try {
      canonical = File(await candidate.resolveSymbolicLinks());
    } on FileSystemException {
      throw const MuseResourceOpenException('NOT_A_FILE');
    }
    final stat = await canonical.stat();
    if (stat.type != FileSystemEntityType.file) {
      throw const MuseResourceOpenException('NOT_A_FILE');
    }
    if (request.origin == MuseResourceOpenOrigin.dshConversation) {
      final cwd = request.sessionCwd;
      if (cwd == null || cwd.trim().isEmpty) {
        throw const MuseResourceOpenException('DSH_CWD_REQUIRED');
      }
      final canonicalCwd = await Directory(cwd).resolveSymbolicLinks();
      if (!p.isWithin(canonicalCwd, canonical.path) &&
          !p.equals(canonicalCwd, canonical.path)) {
        throw const MuseResourceOpenException('OUTSIDE_SESSION_WORKSPACE');
      }
    }
    final extension = MuseResourceEngineRegistry.extensionOf(canonical.path);
    if (defaults != null) {
      await defaults!.ensureLoaded();
    }
    final engine = engines.resolve(
      canonical.path,
      preferred: defaults?.engineFor(extension),
    );
    return MuseResolvedResource(
      file: canonical,
      engine: engine,
      origin: request.origin,
      line: request.line,
      performanceTrace: request.performanceTrace,
    );
  }
}

final class MuseResourceOpenException implements Exception {
  const MuseResourceOpenException(this.code);
  final String code;
  @override
  String toString() => 'MuseResourceOpenException($code)';
}
