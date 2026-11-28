import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  test(
    'Home remote directory adapter keeps grant, binding and offline boundaries',
    () async {
      final root = await Directory.systemTemp.createTemp('home-directory-');
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final pc = await NativeResourceCenter.open(
        scope: 'account',
        deviceRef: 'pc',
        deviceLabel: 'PC',
        registry: registry,
        storageRoot: Directory('${root.path}/pc'),
      );
      final phone = await NativeResourceCenter.open(
        scope: 'account',
        deviceRef: 'phone',
        deviceLabel: 'Phone',
        registry: registry,
        storageRoot: Directory('${root.path}/phone'),
      );
      final mount = await Directory('${root.path}/work').create();
      await Directory('${mount.path}/repository').create();
      await pc.mountDirectories({
        mount.path: 'Workspace',
      }, workspace: 'personal');
      final server = ResourceRpcHttpHost(
        service: pc.rpc,
        authorize: (_) async => ResourcePeer(
          scope: 'account',
          deviceRef: 'phone',
          workspaceRef: 'personal',
          expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
        ),
      );
      await server.start();
      addTearDown(() async {
        await phone.close();
        await server.close();
        await pc.close();
        registry.dispose();
        await root.delete(recursive: true);
      });
      await phone.connect(
        key: 'paired',
        peerRef: 'pc',
        endpoint: Uri.parse(
          'http://127.0.0.1:${server.port}/openmuse/resource-center/v1',
        ),
        headers: () => {},
      );
      final source = phone.hub.visibleProviders.singleWhere(
        (p) => !p.source.local && p.source.mountLabel == 'Workspace',
      );
      var authorized = true;
      final cap = HostDirectoryCapability(
        hub: () => phone.hub,
        sourceKeys: {source.source.key},
        bindings: [],
        authorized: () => authorized,
      );
      final catalog = await cap.call('host_directory_sources', {});
      expect(
        ((catalog['structuredContent'] as Map)['sources'] as List)
            .single['device'],
        'PC',
      );
      final page = await cap.call('host_directory_list', {
        'sourceKey': source.source.key,
        'nodeRef': 'root',
      });
      final entries = (page['structuredContent'] as Map)['directories'] as List;
      expect(entries.single['name'], 'repository');
      final args = {
        'sourceKey': source.source.key,
        'nodeRef': entries.single['nodeRef'],
      };
      expect(
        (await cap.call(
          'host_directory_repository',
          args,
        ))['structuredContent'],
        {'repositoryId': null},
      );
      final bound = HostDirectoryCapability(
        hub: () => phone.hub,
        sourceKeys: {source.source.key},
        bindings: [
          HostRepositoryBinding(
            sourceKey: source.source.key,
            nodeRef: entries.single['nodeRef'],
            repositoryId: 'granted-repo',
          ),
        ],
        authorized: () => authorized,
      );
      expect(
        (await bound.call(
          'host_directory_repository',
          args,
        ))['structuredContent'],
        {'repositoryId': 'granted-repo'},
      );
      expect(
        (await cap.call('host_directory_list', {
          'sourceKey': 'foreign',
          'nodeRef': 'root',
        }))['isError'],
        true,
      );
      expect(
        (await cap.call('host_directory_list', {
          'sourceKey': source.source.key,
          'nodeRef': 'root',
          'path': '/private',
        }))['isError'],
        true,
      );
      authorized = false;
      expect((await cap.call('host_directory_sources', {}))['isError'], true);
      authorized = true;
      (source as RemoteResourceProvider).available = false;
      expect(
        (await cap.call('host_directory_repository', args))['isError'],
        true,
      );
    },
  );
}
