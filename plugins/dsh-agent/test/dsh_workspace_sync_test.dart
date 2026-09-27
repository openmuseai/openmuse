import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:openmuse_dsh_plugin/src/dsh_workspace_sync.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  test(
    'Host mounts are adopted in DSH order and repeated updates are deduplicated',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      addTearDown(() => server.close(force: true));
      final calls = <List<String>>[];
      server.listen((request) async {
        expect(request.uri.path, '/openmuse-bridge/workspaces');
        expect(request.headers.value('x-openmuse-bridge-token'), 'test-secret');
        final body = jsonDecode(await utf8.decoder.bind(request).join()) as Map;
        calls.add((body['mounts'] as List).cast<String>());
        request.response.headers.contentType = ContentType.json;
        request.response.write(jsonEncode({'items': const <Map<String, String>>[]}));
        await request.response.close();
      });
      var active = '/tmp/openmuse-two';
      final context = OpenMusePluginContext(
        executeHostCommand: (command, _) async {
          expect(command, 'workspace.snapshot');
          return {
            'activeMountPath': active,
            'mounts': [
              {'path': '/tmp/openmuse-one'},
              {'path': '/tmp/openmuse-two'},
            ],
          };
        },
      );
      final sync = DshWorkspaceSynchronizer(
        context: context,
        endpoint: () => Uri.parse('http://127.0.0.1:${server.port}/'),
        bridgeToken: 'test-secret',
      );
      await sync.sync();
      expect(calls, hasLength(1));
      expect(calls.single, ['/tmp/openmuse-one', '/tmp/openmuse-two']);
      await sync.sync();
      expect(calls, hasLength(1));
      active = '/tmp/openmuse-one';
      await sync.sync();
      expect(calls, hasLength(1));
    },
  );

  test(
    'latest DSH adopts a Host mount without an API key',
    () async {
      final root = await Directory.systemTemp.createTemp('openmuse-dsh-sync-');
      final mount = Directory('${root.path}/project');
      await mount.create();
      final supervisor = DshSidecarSupervisor(
        environment: {
          'OPENMUSE_DSH_CLI': Platform.environment['OPENMUSE_DSH_CLI']!,
          'DSH_HOME': '${root.path}/dsh',
          'DEEPSEEK_API_KEY': '',
        },
      );
      try {
        await supervisor.ensureStarted();
        final sync = DshWorkspaceSynchronizer(
          context: OpenMusePluginContext(
            executeHostCommand: (_, _) async => {
              'activeMountPath': mount.path,
              'mounts': [
                {'path': mount.path},
              ],
            },
          ),
          endpoint: () => supervisor.endpoint,
          bridgeToken: supervisor.bridgeToken,
        );
        await sync.sync();
        final client = HttpClient();
        try {
          final request = await client.postUrl(supervisor.endpoint!.resolve('/openmuse-bridge/workspaces'));
          request.headers.contentType = ContentType.json;
          request.headers.set('x-openmuse-bridge-token', supervisor.bridgeToken);
          request.write(jsonEncode({'mounts': [mount.path]}));
          final response = await request.close();
          final body =
              jsonDecode(await utf8.decoder.bind(response).join()) as Map;
          final items = body['items'] as List;
          expect(
            items.any(
              (item) => (item as Map)['path'].toString().endsWith('/project'),
            ),
            isTrue,
          );
        } finally {
          client.close(force: true);
        }
      } finally {
        await supervisor.stop();
        supervisor.dispose();
        await root.delete(recursive: true);
      }
    },
    skip: Platform.environment['OPENMUSE_DSH_CLI'] == null,
  );
}
