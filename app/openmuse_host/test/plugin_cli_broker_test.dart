import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/plugin_cli_broker.dart';
import 'package:openmuse_host/src/host/plugin_distribution.dart';
import '../../../plugins/easel/tool/easel_package.dart';

/// The upstream Easel checkout lives in third_party/Easel, which .gitignore
/// deliberately excludes, so this suite only runs on a machine that has it.
/// Elsewhere it reports itself as skipped instead of failing on the missing
/// source files.
final Directory easelSource = Directory('../../third_party/Easel');
final String? easelSkipReason =
    easelSource.existsSync() ? null : 'third_party/Easel is not vendored';

void main() {
  test('broker streams a verified command and rejects missing grant', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-cli-broker-');
    addTearDown(() => root.delete(recursive: true));
    final output = Directory('${root.path}/dist');
    writeEaselPackage(
      output,
      buildEaselPackage(
        Directory('../../third_party/Easel').absolute,
        Directory('../../plugins/easel').absolute,
        includeInstallHook: false,
      ),
    );
    final installRoot = Directory('${root.path}/plugins');
    await installDistributedPlugin(
      catalogUri: Uri.file('${output.path}/catalog.json'),
      pluginId: EaselPluginPackage.pluginId,
      workspacePath: '${root.path}/workspace',
      installRoot: installRoot,
      target: currentDesktopPluginTarget(),
    );
    final article = File('${root.path}/article.md')
      ..writeAsStringSync('# Test article\n\nBody');
    final broker = PluginCliBroker(installRoot: installRoot);
    await broker.start();
    addTearDown(broker.close);
    expect(await invokeBrokeredCliCommand(
      origin: broker.origin,
      token: broker.token,
      identity: 'easel/zhihu/inspect',
      args: ['--article', article.path],
    ), 0);
    final client = HttpClient();
    final denied = await client.postUrl(broker.origin.replace(path: '/v1/execute'));
    denied.write(jsonEncode({
      'protocol': 'openmuse.cli-broker/v1',
      'identity': 'easel/zhihu/inspect',
      'args': ['--article', article.path],
    }));
    expect((await denied.close()).statusCode, HttpStatus.unauthorized);
    client.close(force: true);
  }, skip: easelSkipReason);
}
