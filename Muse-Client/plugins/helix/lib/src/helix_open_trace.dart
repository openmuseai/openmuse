import 'dart:convert';
import 'dart:io';

/// Opt-in timing points for measuring a real packaged Windows editor session.
/// Set OPENMUSE_HELIX_TRACE to a writable JSONL file before launching Host.
final class HelixOpenTrace {
  static final String? _path = Platform.environment['OPENMUSE_HELIX_TRACE'];

  static void mark(
    String event, {
    int? elapsedMs,
    int? childPid,
    Map<String, Object?> data = const {},
  }) {
    final path = _path;
    if (path == null || path.isEmpty) return;
    try {
      File(path).writeAsStringSync(
        '${jsonEncode({'at': DateTime.now().toUtc().toIso8601String(), 'source': 'host', 'event': event, 'host_pid': pid, if (childPid != null) 'child_pid': childPid, if (elapsedMs != null) 'elapsed_ms': elapsedMs, ...data})}\n',
        mode: FileMode.append,
      );
    } on FileSystemException {
      // Tracing must never prevent a file from opening.
    }
  }
}
