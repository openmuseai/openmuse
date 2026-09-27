import 'dart:convert';
import 'dart:io';

import 'package:flutter_pty/flutter_pty.dart';
import 'package:flutter_test/flutter_test.dart';
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
}
