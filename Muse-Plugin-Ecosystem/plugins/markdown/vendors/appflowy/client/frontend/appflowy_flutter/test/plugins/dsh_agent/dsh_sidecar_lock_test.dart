import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_sidecar.dart';
import 'package:flutter_test/flutter_test.dart';

/// A pid that has provably exited by the time this returns.
Future<int> _deadPid() async {
  final process = await Process.start(
    Platform.isWindows ? 'cmd' : 'true',
    Platform.isWindows ? ['/c', 'exit'] : const [],
  );
  final pid = process.pid;
  await process.exitCode;
  return pid;
}

void main() {
  test('clears the writer lock a crashed sidecar left behind', () async {
    final root = Directory.systemTemp.createTempSync('muse-lock-');
    addTearDown(() => root.deleteSync(recursive: true));
    final profiles = Directory('${root.path}/dsh/profiles')
      ..createSync(recursive: true);
    // The shape that bricked an install: `healProfilesModuleFallback` takes
    // `profiles/node_modules.lock` with a two second deadline, and its owner was
    // killed mid-write, so every later boot timed out and the panel reported
    // SIDECAR_EXIT.
    final orphan = File('${profiles.path}/node_modules.lock')
      ..writeAsStringSync('${await _deadPid()}\n');

    final removed = DshSidecar.clearOrphanedWriterLocksIn('${root.path}/dsh');

    expect(removed, hasLength(1));
    expect(removed.single, endsWith('node_modules.lock'));
    expect(orphan.existsSync(), isFalse);
  });

  test('removes a lock whose pid no longer names a dsh writer', () async {
    final root = Directory.systemTemp.createTempSync('muse-lock-');
    addTearDown(() => root.deleteSync(recursive: true));
    final profiles = Directory('${root.path}/dsh/profiles')
      ..createSync(recursive: true);
    // Windows reuses pids, so a live pid is not proof of a live writer unless it
    // is a node process. This test process is the counter-example.
    final reused = File('${profiles.path}/node_modules.lock')
      ..writeAsStringSync('$pid\n');

    final removed = DshSidecar.clearOrphanedWriterLocksIn('${root.path}/dsh');

    expect(removed, hasLength(1));
    expect(reused.existsSync(), isFalse);
  });

  test('leaves locks it cannot attribute, and never enters package trees',
      () async {
    final root = Directory.systemTemp.createTempSync('muse-lock-');
    addTearDown(() => root.deleteSync(recursive: true));
    final profiles = Directory('${root.path}/dsh/profiles')
      ..createSync(recursive: true);
    final unparsable = File('${profiles.path}/settings.lock')
      ..writeAsStringSync('{"pid":1}');
    // A package tree holds thousands of entries and its own `Cargo.lock`s; a
    // scan that descended into it would be slow and could delete the wrong file.
    final cargo = File('${profiles.path}/node_modules/@muse/x/rust/Cargo.lock')
      ..createSync(recursive: true)
      ..writeAsStringSync('version = 3\n');

    final removed = DshSidecar.clearOrphanedWriterLocksIn('${root.path}/dsh');

    expect(removed, isEmpty);
    expect(unparsable.existsSync(), isTrue);
    expect(cargo.existsSync(), isTrue);
  });

  test('a missing DSH home is not an error', () {
    expect(
      DshSidecar.clearOrphanedWriterLocksIn(
        '${Directory.systemTemp.path}/muse-absent-${DateTime.now().microsecondsSinceEpoch}',
      ),
      isEmpty,
    );
  });
}
