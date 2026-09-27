import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:path/path.dart' as p;

void main() {
  test('JavaScript CLI is launched through node without a shell', () {
    final command = dshCommand('/tmp/dsh/lib/bin.js', ['--version']);
    expect(command.executable, 'node');
    expect(command.arguments, ['/tmp/dsh/lib/bin.js', '--version']);
    final bundled = dshCommand('/tmp/dsh/lib/bin.js', [
      '--version',
    ], nodeExecutable: '/bundle/node/bin/node');
    expect(bundled.executable, '/bundle/node/bin/node');
  });

  test('resolves the Windows bundle beside the executable', () {
    final root = p.join('C:', 'OpenMuse');
    final cli = p.join(
      root,
      'openmuse',
      'dsh',
      'node_modules',
      '@deepseek-ai',
      'dsh',
      'lib',
      'bin.js',
    );
    final node = p.join(root, 'openmuse', 'dsh', 'node', 'node.exe');
    final present = {cli, node};
    final resolved = resolveDshRuntime(
      executablePath: p.join(root, 'OpenMuse.exe'),
      exists: (String path) => present.contains(path),
    );
    expect(resolved.cliPath, cli);
    expect(resolved.nodeExecutable, node);
    expect(dshClosureRoot(cli), p.join(root, 'openmuse', 'dsh'));
  });

  test('resolves the macOS bundle under Contents/Resources', () {
    final root = p.join('C:', 'OpenMuse.app');
    final cli = p.join(
      root,
      'Contents',
      'Resources',
      'openmuse',
      'dsh',
      'node_modules',
      '@deepseek-ai',
      'dsh',
      'lib',
      'bin.js',
    );
    final node = p.join(
      root,
      'Contents',
      'Resources',
      'openmuse',
      'dsh',
      'node',
      'bin',
      'node',
    );
    final resolved = resolveDshRuntime(
      executablePath: p.join(root, 'Contents', 'MacOS', 'OpenMuse'),
      exists: (String path) => {cli, node}.contains(path),
    );
    expect(resolved.cliPath, cli);
    expect(resolved.nodeExecutable, node);
  });

  test('explicit CLI wins and bundled node is still preferred', () {
    final root = p.join('C:', 'OpenMuse');
    final node = p.join(root, 'openmuse', 'dsh', 'node', 'node.exe');
    final resolved = resolveDshRuntime(
      executablePath: p.join(root, 'OpenMuse.exe'),
      environment: const {'OPENMUSE_DSH_CLI': r'D:\custom\bin.js'},
      exists: (String path) => path == node,
    );
    expect(resolved.cliPath, r'D:\custom\bin.js');
    expect(resolved.nodeExecutable, node);
  });

  test('missing bundle leaves the runtime unresolved', () {
    final resolved = resolveDshRuntime(
      executablePath: p.join('C:', 'OpenMuse', 'OpenMuse.exe'),
      exists: (_) => false,
    );
    expect(resolved.cliPath, isNull);
    expect(resolved.nodeExecutable, 'node');
  });

  test('bundled node directory is prepended to Path', () {
    final env = dshLaunchEnvironment({
      'Path': r'C:\Windows',
    }, r'D:\OpenMuse\openmuse\dsh\node\node.exe');
    expect(env['Path'], r'D:\OpenMuse\openmuse\dsh\node;C:\Windows');
  });

  test('web sidecar asks DSH itself for an ephemeral loopback port', () {
    final command = dshWebCommand('/tmp/dsh/lib/bin.js');
    expect(command.executable, 'node');
    expect(command.arguments, [
      '/tmp/dsh/lib/bin.js',
      'web',
      '--host',
      '127.0.0.1',
      '--port',
      '0',
      '--no-open',
    ]);
  });

  test('bundled model plugin patch is applied before web app arguments', () {
    final command = dshWebCommand(
      '/bundle/node_modules/@deepseek-ai/dsh/lib/bin.js',
      patchPath:
          '/bundle/node_modules/dsh-model-capabilities/openmuse.patch.yml',
    );
    expect(
      command.arguments,
      containsAllInOrder([
        'web',
        '--patch',
        '/bundle/node_modules/dsh-model-capabilities/openmuse.patch.yml',
        '--host',
        '127.0.0.1',
      ]),
    );
  });

  test(
    'missing runtime is configuration state, not missing model key',
    () async {
      final supervisor = DshSidecarSupervisor(environment: {});
      await supervisor.ensureStarted();
      expect(supervisor.state, DshSidecarState.configurationRequired);
      expect(supervisor.launchCount, 0);
      supervisor.dispose();
    },
  );

  test('model key is not a launch prerequisite', () async {
    final supervisor = DshSidecarSupervisor(
      environment: {'OPENMUSE_DSH_CLI': '/no/such/dsh-runtime'},
    );
    await expectLater(
      supervisor.ensureStarted(),
      throwsA(isA<ProcessException>()),
    );
    expect(supervisor.state, DshSidecarState.failed);
    supervisor.dispose();
  });

  test('readiness probe accepts a local HTTP endpoint', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    server.listen((request) async {
      request.response.statusCode = HttpStatus.ok;
      await request.response.close();
    });
    await waitForHttp(
      Uri.parse('http://127.0.0.1:${server.port}/'),
      const Duration(seconds: 2),
    );
    await server.close(force: true);
  });

  test(
    'latest CLI starts the local web UI without a model key',
    () async {
      final root = await Directory.systemTemp.createTemp('openmuse-dsh-test-');
      final supervisor = DshSidecarSupervisor(
        environment: {
          'OPENMUSE_DSH_CLI': Platform.environment['OPENMUSE_DSH_CLI']!,
          'DSH_HOME': root.path,
          'DEEPSEEK_API_KEY': '',
        },
      );
      try {
        await supervisor.ensureStarted();
        expect(supervisor.state, DshSidecarState.ready);
        expect(supervisor.endpoint?.host, '127.0.0.1');
      } finally {
        await supervisor.stop();
        supervisor.dispose();
        await root.delete(recursive: true);
      }
    },
    skip: Platform.environment['OPENMUSE_DSH_CLI'] == null,
  );
}
