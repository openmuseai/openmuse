import 'dart:convert';
import 'dart:io';

import 'package:flutter_pty/flutter_pty.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/src/helix_preferences.dart';
import 'package:openmuse_helix_plugin/src/helix_pty.dart';
import 'package:openmuse_helix_plugin/src/helix_runtime.dart';

void main() {
  test('Windows PTY environment keeps SystemRoot and Path', () {
    final environment = helixProcessEnvironment({
      'SystemRoot': r'C:\Windows',
      'Path': r'C:\Windows\System32',
      'HOME': r'C:\Users\openmuse',
    });
    if (Platform.isWindows) {
      expect(environment['SystemRoot'], r'C:\Windows');
      expect(environment['Path'], r'C:\Windows\System32');
    } else {
      expect(environment, isEmpty);
    }
  });

  test('Helix language configuration uses the generated directory', () {
    final environment = helixProcessEnvironment({
      'APPDATA': r'C:\Users\openmuse\AppData\Roaming',
    }, configHome: r'C:\Temp\openmuse-helix-test');
    expect(environment['XDG_CONFIG_HOME'], r'C:\Temp\openmuse-helix-test');
    if (Platform.isWindows) {
      expect(environment['APPDATA'], r'C:\Temp\openmuse-helix-test');
    }
  });

  test(
    'bundled Windows hx reads the generated language server override',
    () async {
      final hx = Platform.environment['OPENMUSE_HELIX_BIN'];
      if (!Platform.isWindows || hx == null || !File(hx).existsSync()) {
        markTestSkipped('OPENMUSE_HELIX_BIN is not a Windows hx.exe');
        return;
      }
      final root = await Directory.systemTemp.createTemp(
        'openmuse-helix-config-',
      );
      addTearDown(() => root.delete(recursive: true));
      final languages = File('${root.path}/helix/languages.toml');
      await languages.parent.create(recursive: true);
      await languages.writeAsString(
        HelixPreferences(
          languageServerPaths: {'rust-analyzer': hx},
        ).languagesToml,
      );
      final result = await Process.run(
        hx,
        ['--health', 'rust'],
        environment: {
          ...helixProcessEnvironment(
            Platform.environment,
            configHome: root.path,
          ),
          'HELIX_RUNTIME': '${File(hx).parent.path}/runtime',
        },
      ).timeout(const Duration(seconds: 15));
      expect(result.exitCode, 0, reason: '${result.stderr}');
      expect(result.stdout, contains('rust-analyzer: $hx'));
    },
  );

  test('Windows PTY start reports a missing executable cleanly', () async {
    if (!Platform.isWindows) return;
    await expectLater(
      HelixPty.start(
        '${Directory.systemTemp.path}${Platform.pathSeparator}openmuse-missing-hx.exe',
        arguments: const [],
        workingDirectory: Directory.systemTemp.path,
        environment: helixProcessEnvironment(Platform.environment),
        rows: 24,
        columns: 80,
      ),
      throwsStateError,
    );
  });

  test('staged Windows grammars provide code highlighting', () async {
    final hx = Platform.environment['OPENMUSE_HELIX_BIN'];
    if (!Platform.isWindows || hx == null || !File(hx).existsSync()) {
      markTestSkipped('OPENMUSE_HELIX_BIN is not a Windows hx.exe');
      return;
    }
    for (final language in ['rust', 'python', 'javascript']) {
      final result = await Process.run(
        hx,
        ['--health', language],
        stdoutEncoding: utf8,
        stderrEncoding: utf8,
        environment: {
          ...Platform.environment,
          'HELIX_RUNTIME':
              '${File(hx).parent.path}${Platform.pathSeparator}runtime',
        },
      );
      expect(result.exitCode, 0);
      expect(result.stdout, contains('Tree-sitter parser: ✓'));
      expect(result.stdout, contains('Highlight queries: ✓'));
    }
  });

  test('staged hx.exe opens a file through the Windows PTY', () async {
    final configured = Platform.environment['OPENMUSE_HELIX_BIN'];
    final hx = configured == null || configured.isEmpty ? '' : configured;
    if (!Platform.isWindows || hx.isEmpty || !File(hx).existsSync()) {
      markTestSkipped('OPENMUSE_HELIX_BIN is not a Windows hx.exe');
      return;
    }
    final file = File(
      '${Directory.systemTemp.path}${Platform.pathSeparator}openmuse_helix_probe.dart',
    );
    await file.writeAsString('void main() { /* OPENMUSE_ENCODING_PROBE */ }\n');
    final pty = Pty.start(
      hx,
      arguments: [file.path],
      workingDirectory: file.parent.path,
      environment: {
        ...helixProcessEnvironment(Platform.environment),
        'HELIX_RUNTIME':
            '${File(hx).parent.path}${Platform.pathSeparator}runtime',
      },
      rows: 24,
      columns: 80,
    );
    final output = StringBuffer();
    final subscription = pty.output.listen((bytes) {
      output.write(utf8.decode(bytes, allowMalformed: true));
    });
    await Future<void>.delayed(const Duration(seconds: 2));
    expect(pty.pid, greaterThan(0));
    expect(output.toString(), contains('OPENMUSE_ENCODING_PROBE'));
    pty.kill();
    await subscription.cancel();
  });

  test('Windows PTY creation keeps the UI isolate responsive', () async {
    final hx = Platform.environment['OPENMUSE_HELIX_BIN'];
    if (!Platform.isWindows || hx == null || !File(hx).existsSync()) {
      markTestSkipped('OPENMUSE_HELIX_BIN is not a Windows hx.exe');
      return;
    }
    final file = File(
      '${Directory.systemTemp.path}${Platform.pathSeparator}openmuse_helix_async_probe.txt',
    );
    await file.writeAsString('OPENMUSE_ASYNC_PTY_PROBE\n');
    addTearDown(() async {
      await file.delete();
    });

    final watch = Stopwatch()..start();
    final started = HelixPty.start(
      hx,
      arguments: [file.path],
      workingDirectory: file.parent.path,
      environment: {
        ...helixProcessEnvironment(Platform.environment),
        'HELIX_RUNTIME':
            '${File(hx).parent.path}${Platform.pathSeparator}runtime',
      },
      rows: 24,
      columns: 80,
    );
    await Future<void>.delayed(const Duration(milliseconds: 50));
    expect(watch.elapsedMilliseconds, lessThan(250));
    final pty = await started;
    expect(pty.pid, greaterThan(0));
    pty.kill();
    await pty.exitCode.timeout(const Duration(seconds: 5));
  });

  test(
    'Windows Helix reuses one process for two files',
    () async {
      final hx = Platform.environment['OPENMUSE_HELIX_BIN'];
      if (!Platform.isWindows || hx == null || !File(hx).existsSync()) {
        markTestSkipped('OPENMUSE_HELIX_BIN is not a Windows hx.exe');
        return;
      }
      final directory = await Directory.systemTemp.createTemp(
        'openmuse-helix-reuse-',
      );
      final first = File('${directory.path}${Platform.pathSeparator}first.txt');
      final second = File(
        '${directory.path}${Platform.pathSeparator}second.txt',
      );
      await first.writeAsString('FIRST_OPENMUSE_BUFFER\n');
      await second.writeAsString('SECOND_OPENMUSE_BUFFER\n');

      final runtime = HelixRuntimePool(executable: hx);
      addTearDown(() async {
        await runtime.stop();
        runtime.dispose();
        // ConPTY releases the process working directory after its exit event.
        for (var attempt = 0; attempt < 20; attempt++) {
          try {
            await directory.delete(recursive: true);
            break;
          } on PathAccessException {
            if (attempt == 19) rethrow;
            await Future<void>.delayed(const Duration(milliseconds: 100));
          }
        }
      });
      final states = <HelixResourceEvent>[];
      runtime.onResourceEvent = states.add;
      expect(await runtime.probeCapabilities(), isTrue);
      await runtime.configure(const HelixPreferences(enableLsp: false));
      await runtime.openDocument(first.path);
      await _waitForState(states, first.path);
      final firstPid = runtime.pid;
      expect(firstPid, greaterThan(0));

      final beforeSecond = states.length;
      final openSecond = runtime.openDocument(second.path);
      expect(runtime.isSwitching, isTrue);
      await openSecond;
      await _waitForState(states, second.path, afterIndex: beforeSecond);
      await _waitForSwitchFinish(runtime);
      expect(runtime.pid, firstPid);
      expect(runtime.launchCount, 1);

      final beforeReturn = states.length;
      final returnToFirst = runtime.openDocument(first.path);
      expect(runtime.isSwitching, isTrue);
      await returnToFirst;
      await _waitForState(states, first.path, afterIndex: beforeReturn);
      await _waitForSwitchFinish(runtime);
      expect(runtime.pid, firstPid);
      expect(runtime.launchCount, 1);
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
}

Future<void> _waitForSwitchFinish(HelixRuntimePool runtime) async {
  for (var attempt = 0; attempt < 100; attempt++) {
    if (!runtime.isSwitching) return;
    await Future<void>.delayed(const Duration(milliseconds: 20));
  }
  fail('Helix did not paint the requested buffer');
}

Future<void> _waitForState(
  List<HelixResourceEvent> states,
  String path, {
  int afterIndex = 0,
}) async {
  for (var attempt = 0; attempt < 100; attempt++) {
    if (states
        .skip(afterIndex)
        .any((event) => event.type == 'state' && event.path == path)) {
      return;
    }
    await Future<void>.delayed(const Duration(milliseconds: 50));
  }
  fail('Helix did not report $path as its active buffer');
}
