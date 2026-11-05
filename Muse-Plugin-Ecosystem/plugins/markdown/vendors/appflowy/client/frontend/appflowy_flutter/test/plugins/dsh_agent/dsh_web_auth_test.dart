import 'package:appflowy/plugins/dsh_agent/dsh_web_auth.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('extracts the launch token from a dsh web stdout line', () {
    expect(
      DshWebAuth.extractLaunchToken(
        'dsh web: http://127.0.0.1:3080/?token=launch-token',
      ),
      'launch-token',
    );
    expect(
      DshWebAuth.extractLaunchUrl(
        'ready\ndsh web: http://127.0.0.1:3080/?token=abc123\n',
      ),
      'http://127.0.0.1:3080/?token=abc123',
    );
    expect(DshWebAuth.extractLaunchToken('listening on 3080'), isNull);
    expect(
      DshWebAuth.extractLaunchToken('dsh web: http://127.0.0.1:3080/'),
      isNull,
    );
    expect(
      DshWebAuth.extractLaunchUrl('dsh web: http://127.0.0.1:3080'),
      'http://127.0.0.1:3080',
    );
  });

  test('does not declare the sidecar ready before the launch token arrives', () {
    expect(DshWebAuth.isStartupProbeHealthy(200, null), isTrue);
    expect(DshWebAuth.isStartupProbeHealthy(401, null), isFalse);
    expect(DshWebAuth.isStartupProbeHealthy(401, 'launch-token'), isTrue);
    expect(DshWebAuth.isStartupProbeHealthy(500, 'launch-token'), isFalse);
  });

  test('readiness probe uses the origin without the launch token', () {
    expect(
      DshWebAuth.probeUri('http://127.0.0.1:3080/?token=launch-token').toString(),
      'http://127.0.0.1:3080/',
    );
    expect(
      DshWebAuth.probeUri(null).toString(),
      'http://127.0.0.1:3080',
    );
  });

  test('parses LISTENING pids for the sidecar loopback port from netstat', () {
    const output = '''
  TCP    127.0.0.1:3080         0.0.0.0:0              LISTENING       25976
  TCP    127.0.0.1:3080         127.0.0.1:50722        TIME_WAIT       0
  TCP    [::1]:3080             [::]:0                 LISTENING       25976
  TCP    0.0.0.0:3080           0.0.0.0:0              LISTENING       111
''';
    expect(DshWebAuth.listeningPidsFromNetstat(output), {25976});
  });

  test('summarizes EADDRINUSE instead of dumping the Node stack', () {
    expect(
      DshWebAuth.summarizeExit(1, [
        'Error: dsh: plugin tree failed to load: listen EADDRINUSE: '
            'address already in use 127.0.0.1:3080',
      ]),
      contains('127.0.0.1:3080 is already in use'),
    );
  });

  test('summarizes ERR_MODULE_NOT_FOUND without the aggregate stack', () {
    expect(
      DshWebAuth.summarizeExit(1, [
        'Error: dsh: plugin tree failed to load: loader entries failed to apply',
        'Cannot find package \'@deepseek-ai/cordis-plugin-timer\' imported from C:\\profile\\web\\',
        'Did you mean to import "@deepseek-ai/cordis-plugin-timer/lib/index.js"?',
        'code: \'ERR_MODULE_NOT_FOUND\'',
      ]),
      'DSH sidecar exited with 1: Cannot find package '
          '\'@deepseek-ai/cordis-plugin-timer\' imported from C:\\profile\\web\\',
    );
  });
}
