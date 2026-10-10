import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import '../../../plugins/easel/tool/easel_package.dart';
import 'package:openmuse_host/src/host/local_settings.dart';
import 'package:openmuse_host/src/host/plugin_cli.dart';
import 'package:openmuse_host/src/host/plugin_command_registry.dart';
import 'package:openmuse_host/src/host/plugin_distribution.dart';
import 'package:openmuse_host/src/host/settings_dialog.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// The upstream Easel checkout lives in third_party/Easel, which .gitignore
/// deliberately excludes. Every test here installs the package built from that
/// source tree, so without it they report themselves as skipped instead of
/// failing on the missing source files.
final Directory easelSource = Directory('../../third_party/Easel');
final String? easelSkipReason =
    easelSource.existsSync() ? null : 'third_party/Easel is not vendored';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  late Directory cliRoot;
  late OpenMuseLocalSettings installedSettings;

  setUpAll(() async {
    if (easelSkipReason != null) return;
    cliRoot = Directory.systemTemp.createTempSync('openmuse-easel-cli-');
    final output = Directory('${cliRoot.path}/dist');
    final dataDir = Directory('${cliRoot.path}/OpenMuse');
    final workspace = Directory('${cliRoot.path}/EaselWorkspace');
    final easel = Directory('../../third_party/Easel').absolute.path;
    writeEaselPackage(
      output,
      buildEaselPackage(
        Directory(easel),
        Directory('../../plugins/easel'),
        includeInstallHook: false,
      ),
    );
    expect(File('${output.path}/catalog.json').existsSync(), isTrue);
    expect(
      File('${output.path}/${EaselPluginPackage.fileName}').existsSync(),
      isTrue,
    );
    expect(
      await runPluginCli([
        'install',
        '--catalog',
        '${output.path}/catalog.json',
        '--workspace',
        workspace.path,
        '--plugin',
        EaselPluginPackage.pluginId,
        '--data-dir',
        dataDir.path,
      ]),
      0,
    );
    installedSettings = OpenMuseLocalSettings(
      file: File('${dataDir.path}/settings-v1.json'),
    );
    await installedSettings.load();
  });

  tearDownAll(() {
    if (easelSkipReason == null && cliRoot.existsSync()) {
      cliRoot.deleteSync(recursive: true);
    }
  });

  testWidgets('CLI install shows Easel as installed in settings', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1400, 900);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Builder(
            builder: (context) => TextButton(
              onPressed: () =>
                  showOpenMuseSettings(context, installedSettings, registry),
              child: const Text('打开设置'),
            ),
          ),
        ),
      ),
    );
    await tester.tap(find.text('打开设置'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('插件'));
    await tester.pumpAndSettle();

    expect(
      find.byKey(const ValueKey('distributed-plugin-com.openmuse.easel')),
      findsOneWidget,
    );
    expect(find.text('Easel'), findsOneWidget);
    expect(find.text('已安装'), findsOneWidget);
    expect(find.text(EaselPluginPackage.version), findsWidgets);
    expect(find.textContaining('EaselWorkspace'), findsOneWidget);
    // testWidgets only accepts a bool for skip, so the reason is the one the
    // plain tests in this file report.
  }, skip: easelSkipReason != null);

  test('OpenMuse does not ship Easel', () {
    expect(
      createOpenMuseBuiltInPlugins().map((plugin) => plugin.descriptor.id),
      isNot(contains('com.openmuse.easel')),
    );
  }, skip: easelSkipReason);

  test('installed Easel contributes gated CLI commands', () {
    final discovery = discoverInstalledCliCommands(
      Directory('${cliRoot.path}/OpenMuse/plugins'),
    );
    expect(discovery.errors, isEmpty);
    expect(
      discovery.commands.keys,
      containsAll([
        'easel/douyin/plan',
        'easel/douyin/selftest',
        'easel/douyin/check',
        'easel/zhihu/inspect',
        'easel/zhihu/login',
      ]),
    );
    final plan = discovery.commands['easel/douyin/plan']!;
    expect(plan.verifiedEntrypoint().existsSync(), isTrue);
    expect(plan.argv(['--title', '离线测试', '--content', '简介']), [
      'douyin',
      'plan',
      '--title',
      '离线测试',
      '--content',
      '简介',
    ]);
    expect(() => plan.argv(['--exec']), throwsFormatException);
    expect(() => plan.argv(['--title']), throwsFormatException);
  }, skip: easelSkipReason);

  test('tampered Easel CLI artifact cannot be invoked', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-cli-tamper-');
    addTearDown(() => root.delete(recursive: true));
    final workspace = Directory('${root.path}/workspace');
    final installRoot = Directory('${root.path}/plugins');
    await installDistributedPlugin(
      catalogUri: Uri.file('${cliRoot.path}/dist/catalog.json'),
      pluginId: EaselPluginPackage.pluginId,
      workspacePath: workspace.path,
      installRoot: installRoot,
      target: currentDesktopPluginTarget(),
    );
    final plan = discoverInstalledCliCommands(
      installRoot,
    ).commands['easel/douyin/plan']!;
    final script = plan.verifiedEntrypoint();
    script.writeAsStringSync(
      '\n# changed after install',
      mode: FileMode.append,
    );
    expect(() => plan.verifiedEntrypoint(), throwsFormatException);
  }, skip: easelSkipReason);

  test('disabled Easel workspace withdraws its CLI commands', () async {
    final dataDir = Directory('${cliRoot.path}/disabled-data');
    final installRoot = Directory('${dataDir.path}/plugins');
    await installDistributedPlugin(
      catalogUri: Uri.file('${cliRoot.path}/dist/catalog.json'),
      pluginId: EaselPluginPackage.pluginId,
      workspacePath: '${cliRoot.path}/disabled-workspace',
      installRoot: installRoot,
      target: currentDesktopPluginTarget(),
    );
    File('${dataDir.path}/settings-v1.json').writeAsStringSync(
      jsonEncode({
        'version': 1,
        'plugins': {
          EaselPluginPackage.pluginId: {'workspaceEnabled': false},
        },
      }),
    );
    expect(discoverInstalledCliCommands(installRoot).commands, isEmpty);
  }, skip: easelSkipReason);

  test('Easel installs from a network catalog into its workspace', () async {
    await _realHttp(() async {
      final built = _easelPackage();
      final server = await _serve(built.bytes, built.sha256);
      addTearDown(() => server.server.close(force: true));
      final root = await Directory.systemTemp.createTemp(
        'openmuse-plugin-install-',
      );
      addTearDown(() => root.delete(recursive: true));
      final workspacePath = '${root.path}/EaselWorkspace';
      final installRoot = Directory('${root.path}/installed');
      final settings = OpenMuseLocalSettings(
        file: File('${root.path}/settings.json'),
      );
      final controller = LocalWorkspaceController(
        rootPath: '${root.path}/host-workspace',
        initialResources: const [],
      );
      addTearDown(controller.dispose);

      final receipt = await acceptDistributedPlugin(
        catalogUri: server.catalog,
        pluginId: 'com.openmuse.easel',
        workspacePath: workspacePath,
        installRoot: installRoot,
        target: currentDesktopPluginTarget(),
        workspace: controller,
        settings: settings,
      );

      final script = File(
        '${receipt.workspacePath}/skills/shared/scripts/douyin_publish.py',
      );
      for (final module in [
        'login_state.py',
        'content_guard.py',
        'platform_readback.py',
        'human_pace.py',
      ]) {
        expect(
          File(
            '${receipt.workspacePath}/skills/shared/scripts/$module',
          ).existsSync(),
          isTrue,
        );
      }
      expect(script.readAsBytesSync(), built.script);
      expect(
        Directory('${receipt.workspacePath}/skills/openclaw').existsSync(),
        isFalse,
      );
      for (final name in ['profiles', 'cache', 'outputs', 'state']) {
        expect(
          Directory('${receipt.workspacePath}/$name').existsSync(),
          isTrue,
        );
      }
      expect(Directory('${receipt.workspacePath}/.venv').existsSync(), isFalse);
      expect(
        Directory(
          '${installRoot.path}/com.openmuse.easel/payload/easel-web',
        ).existsSync(),
        isFalse,
      );
      expect(File('${receipt.workspacePath}/app.py').existsSync(), isFalse);
      expect(
        controller.mounts.map((mount) => mount.path),
        contains(receipt.workspacePath),
      );
      expect(
        settings.pluginValues('com.openmuse.easel')['workspacePath'],
        receipt.workspacePath,
      );

      File(
        '${receipt.workspacePath}/profiles/session.txt',
      ).writeAsStringSync('keep');
      final again = await acceptDistributedPlugin(
        catalogUri: server.catalog,
        pluginId: 'com.openmuse.easel',
        workspacePath: workspacePath,
        installRoot: installRoot,
        target: currentDesktopPluginTarget(),
        workspace: controller,
        settings: settings,
      );
      expect(again.packageSha256, receipt.packageSha256);
      expect(
        File(
          '${receipt.workspacePath}/profiles/session.txt',
        ).readAsStringSync(),
        'keep',
      );

      await uninstallDistributedPlugin(
        installRoot: installRoot,
        pluginId: 'com.openmuse.easel',
      );
      expect(
        Directory('${installRoot.path}/com.openmuse.easel').existsSync(),
        isFalse,
      );
      expect(
        File(
          '${receipt.workspacePath}/profiles/session.txt',
        ).readAsStringSync(),
        'keep',
      );
    });
  }, skip: easelSkipReason);

  test('a tampered catalog package is refused', () async {
    await _realHttp(() async {
      final built = _easelPackage();
      final tampered = List<int>.of(built.bytes);
      tampered[tampered.length ~/ 2] ^= 0xff;
      final server = await _serve(tampered, built.sha256);
      addTearDown(() => server.server.close(force: true));
      final root = await Directory.systemTemp.createTemp(
        'openmuse-plugin-bad-',
      );
      addTearDown(() => root.delete(recursive: true));
      expect(
        installDistributedPlugin(
          catalogUri: server.catalog,
          pluginId: 'com.openmuse.easel',
          workspacePath: '${root.path}/workspace',
          installRoot: Directory('${root.path}/installed'),
          target: currentDesktopPluginTarget(),
        ),
        throwsFormatException,
      );
      expect(Directory('${root.path}/workspace').existsSync(), isFalse);
    });
  }, skip: easelSkipReason);

  test('package urls must stay on the catalog origin', () async {
    await _realHttp(() async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      addTearDown(() => server.close(force: true));
      server.listen((request) async {
        request.response.headers.contentType = ContentType.json;
        request.response.write(
          jsonEncode({
            'packages': [
              {
                'id': 'com.openmuse.easel',
                'version': EaselPluginPackage.version,
                'url': 'http://example.com/easel.omplugin',
                'sha256': 'aa',
                'size': 1,
              },
            ],
          }),
        );
        await request.response.close();
      });
      final root = await Directory.systemTemp.createTemp(
        'openmuse-plugin-origin-',
      );
      addTearDown(() => root.delete(recursive: true));
      expect(
        installDistributedPlugin(
          catalogUri: Uri.parse('http://127.0.0.1:${server.port}/catalog.json'),
          pluginId: 'com.openmuse.easel',
          workspacePath: '${root.path}/workspace',
          installRoot: Directory('${root.path}/installed'),
          target: currentDesktopPluginTarget(),
        ),
        throwsFormatException,
      );
    });
  }, skip: easelSkipReason);

  test('mobile targets cannot install the desktop package', () async {
    await _realHttp(() async {
      final built = _easelPackage();
      final server = await _serve(built.bytes, built.sha256);
      addTearDown(() => server.server.close(force: true));
      final root = await Directory.systemTemp.createTemp(
        'openmuse-plugin-mobile-',
      );
      addTearDown(() => root.delete(recursive: true));
      expect(
        installDistributedPlugin(
          catalogUri: server.catalog,
          pluginId: 'com.openmuse.easel',
          workspacePath: '${root.path}/workspace',
          installRoot: Directory('${root.path}/installed'),
          target: const OpenMuseTarget(
            os: OpenMuseTargetOs.android,
            arch: OpenMuseTargetArch.aarch64,
            libc: OpenMuseTargetLibc.bionic,
          ),
        ),
        throwsFormatException,
      );
      expect(Directory('${root.path}/workspace').existsSync(), isFalse);
    });
  }, skip: easelSkipReason);
}

final class _RealHttp extends HttpOverrides {}

Future<T> _realHttp<T>(Future<T> Function() body) =>
    HttpOverrides.runWithHttpOverrides(body, _RealHttp());

final class _Served {
  const _Served(this.server, this.catalog);
  final HttpServer server;
  final Uri catalog;
}

final class _Package {
  const _Package({
    required this.bytes,
    required this.sha256,
    required this.script,
  });

  final List<int> bytes;
  final String sha256;
  final List<int> script;
}

Future<_Served> _serve(List<int> bytes, String sha256) async {
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  final catalog = Uri.parse('http://127.0.0.1:${server.port}/catalog.json');
  server.listen((request) async {
    if (request.uri.path == '/catalog.json') {
      request.response.headers.contentType = ContentType.json;
      request.response.write(
        jsonEncode({
          'packages': [
            {
              'id': 'com.openmuse.easel',
              'version': EaselPluginPackage.version,
              'url': '/easel.omplugin',
              'sha256': sha256,
              'size': bytes.length,
            },
          ],
        }),
      );
    } else if (request.uri.path == '/easel.omplugin') {
      request.response.headers.contentType = ContentType.binary;
      request.response.add(bytes);
    } else {
      request.response.statusCode = HttpStatus.notFound;
    }
    await request.response.close();
  });
  return _Served(server, catalog);
}

_Package _easelPackage() {
  final easel = Directory('../../third_party/Easel');
  final built = buildEaselPackage(
    easel,
    Directory('../../plugins/easel'),
    includeInstallHook: false,
  );
  return _Package(
    bytes: built.bytes,
    sha256: built.sha256,
    script: File(
      '${easel.path}/skills/shared/scripts/douyin_publish.py',
    ).readAsBytesSync(),
  );
}
