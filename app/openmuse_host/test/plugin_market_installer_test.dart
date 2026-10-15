import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/plugin_market.dart';
import 'package:path/path.dart' as p;

void main() {
  if (Platform.isWindows) return;

  late Directory root;
  late File cli;
  late File node;
  late File pnpm;

  setUp(() {
    root = Directory.systemTemp.createTempSync('openmuse-market-check-');
    cli = File(p.join(root.path, 'fake-dsh'))
      ..writeAsStringSync('#!/bin/sh\nprintf "0.2.0-rc.2\\n"\n');
    node = File(p.join(root.path, 'fake-node'))
      ..writeAsStringSync('#!/bin/sh\nprintf "v24.0.0\\n"\n');
    pnpm = File(p.join(root.path, 'fake-pnpm'))
      ..writeAsStringSync('#!/bin/sh\nprintf "10.34.1\\n"\n');
    expect(
      Process.runSync('chmod', ['+x', cli.path, node.path, pnpm.path]).exitCode,
      0,
    );
  });

  tearDown(() => root.deleteSync(recursive: true));

  DshMarketInstaller installer() => DshMarketInstaller(
    dshHome: Directory(p.join(root.path, 'dsh')),
    profile: 'test',
    cliPath: cli.path,
    nodeExecutable: node.path,
    packageManagerExecutable: pnpm.path,
  );

  test('rejects an unverified DSH runtime before changing a profile', () async {
    cli.writeAsStringSync('#!/bin/sh\nprintf "0.1.6-rc.1\\n"\n');
    await expectLater(
      installer().plan(builtInMarketPlugins.single),
      throwsA(isA<StateError>()),
    );
    expect(
      Directory(p.join(root.path, 'dsh', 'profiles')).existsSync(),
      isFalse,
    );
  });

  test('reports a missing bundled pnpm before changing a profile', () async {
    pnpm.deleteSync();
    await expectLater(
      installer().plan(builtInMarketPlugins.single),
      throwsA(isA<StateError>()),
    );
    expect(
      Directory(p.join(root.path, 'dsh', 'profiles')).existsSync(),
      isFalse,
    );
  });

  test(
    'rejects a forged lockfile even when the package version matches',
    () async {
      final profile = Directory(p.join(root.path, 'dsh', 'profiles', 'test'))
        ..createSync(recursive: true);
      final package = Directory(
        p.join(profile.path, 'node_modules', '@tt-a1i', 'archify-dsh'),
      )..createSync(recursive: true);
      File(p.join(package.path, 'package.json')).writeAsStringSync(
        jsonEncode({'name': '@tt-a1i/archify-dsh', 'version': '1.0.0'}),
      );
      File(p.join(profile.path, 'package.json')).writeAsStringSync(
        jsonEncode({
          'dependencies': {'@tt-a1i/archify-dsh': '1.0.0'},
          'dsh': {
            'profile': {
              'bundles': ['@tt-a1i/archify-dsh'],
            },
          },
        }),
      );
      File(p.join(profile.path, 'pnpm-lock.yaml')).writeAsStringSync('''
lockfileVersion: '9.0'
packages:
  '@tt-a1i/archify-dsh@1.0.0':
    resolution: {integrity: sha512-wrong}
''');
      await expectLater(
        installer().install(builtInMarketPlugins.single),
        throwsA(isA<StateError>()),
      );
    },
  );

  test(
    'disables and re-enables an integrity-checked installed plugin',
    () async {
      final plugin = builtInMarketPlugins.single;
      final profile = Directory(p.join(root.path, 'dsh', 'profiles', 'test'))
        ..createSync(recursive: true);
      final package = Directory(
        p.join(profile.path, 'node_modules', '@tt-a1i', 'archify-dsh'),
      )..createSync(recursive: true);
      File(p.join(package.path, 'package.json')).writeAsStringSync(
        jsonEncode({'name': plugin.packageName, 'version': plugin.version}),
      );
      final manifest = File(p.join(profile.path, 'package.json'))
        ..writeAsStringSync(
          jsonEncode({
            'dependencies': {plugin.packageName: plugin.version},
            'dsh': {
              'profile': {
                'bundles': [plugin.packageName],
              },
            },
          }),
        );
      File(p.join(profile.path, 'pnpm-lock.yaml')).writeAsStringSync('''
lockfileVersion: '9.0'
packages:
  '${plugin.exactPackage}':
    resolution: {integrity: ${plugin.integrity}}
''');
      final installed = installer();
      expect((await installed.plan(plugin))['action'], 'already-installed');
      expect(
        (await installed.setEnabled(plugin, false))['restartRequired'],
        true,
      );
      expect(
        (jsonDecode(manifest.readAsStringSync())['dsh']['profile']['bundles']
            as List),
        isEmpty,
      );
      expect((await installed.plan(plugin))['action'], 'enable');
      await installed.setEnabled(plugin, true);
      expect(
        (jsonDecode(manifest.readAsStringSync())['dsh']['profile']['bundles']
            as List),
        [plugin.packageName],
      );
    },
  );
}
