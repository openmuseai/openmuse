import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/version_capability.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  test(
    'account switch during editor flush never captures an old resource into the new store',
    () async {
      final root = await Directory.systemTemp.createTemp('version-account-');
      final oldRoot = await Directory('${root.path}/old').create(),
          newRoot = await Directory('${root.path}/new').create();
      final oldStore = LocalVersionStore(
            Directory('${root.path}/old-versions'),
          ),
          newStore = LocalVersionStore(Directory('${root.path}/new-versions'));
      final file = File('${oldRoot.path}/private.md');
      await file.writeAsString('old account private bytes');
      final resource = OpenMuseResource(
        uri: file.uri,
        displayName: 'private.md',
      );
      final controller = LocalWorkspaceController(
        rootPath: oldRoot.path,
        initialResources: const [],
        versionStore: oldStore,
      );
      try {
        final entered = Completer<void>(), release = Completer<void>();
        controller.flushBeforeDiskRead = (r) async {
          entered.complete();
          await release.future;
        };
        final pending = controller.captureVersion(resource);
        await entered.future;
        final checked = expectLater(pending, throwsStateError);
        await controller.switchStorage(
          rootPath: newRoot.path,
          versionStore: newStore,
          mountStore: WorkspaceMountStore(File('${root.path}/mounts.json')),
        );
        release.complete();
        await checked;
        expect(await oldStore.list(resource), isEmpty);
        expect(await newStore.list(resource), isEmpty);
      } finally {
        controller.dispose();
        await root.delete(recursive: true);
      }
    },
  );
  test(
    'existing Host authority records A B A events with two verified blobs',
    () async {
      final root = await Directory.systemTemp.createTemp('version-events-');
      try {
        final file = File('${root.path}/demo.txt'),
            store = LocalVersionStore(Directory('${root.path}/versions'));
        final resource = OpenMuseResource(
          uri: file.uri,
          displayName: 'demo.txt',
        );
        await file.writeAsString('A');
        final a = await store.capture(resource);
        await file.writeAsString('B');
        await store.capture(resource);
        await file.writeAsString('A');
        final again = await store.capture(resource);
        expect(a.id, again.id);
        expect(a.versionId, isNot(again.versionId));
        expect(await store.list(resource), hasLength(3));
        expect(await Directory('${root.path}/versions/blobs').list().length, 2);
        await File(
          '${root.path}/versions/blobs/${a.id}',
        ).writeAsString('corrupt');
        await expectLater(store.readBytes(a), throwsStateError);
        await expectLater(
          store.readBytes(
            LocalVersionSnapshot(
              id: '../../secret',
              resourcePath: file.path,
              createdAt: DateTime.now(),
              byteLength: 1,
            ),
          ),
          throwsStateError,
        );
      } finally {
        await root.delete(recursive: true);
      }
    },
  );
  test(
    'public tools scope, flush, non-code restore backup, stale guard and binary bytes',
    () async {
      final root = await Directory.systemTemp.createTemp('version-tools-');
      final controller = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
        versionStore: LocalVersionStore(Directory('${root.path}/history')),
      );
      try {
        final file = File('${root.path}/product.md');
        await file.writeAsString('# 产品稿\nA');
        final provider = HostVersionCapability(
          controller,
          resources: ['product.md', 'picture.png'],
          writable: true,
        );
        Future<Map> call(String name, Map<String, Object?> args) async =>
            (await provider.call(name, args))['structuredContent'] as Map;
        final listed = await call('host_versions_list', {}),
            id = (listed['resources'] as List).first['id'] as String;
        final first = await call('host_versions_capture', {'resourceId': id});
        await file.writeAsString('# 产品稿\nB');
        final stale = await provider.call('host_versions_restore', {
          'resourceId': id,
          'versionId': first['versionId'],
          'expectedSha256': '0' * 64,
        });
        expect(stale['isError'], true);
        expect(await file.readAsString(), '# 产品稿\nB');
        final current = await call('host_versions_read', {'resourceId': id});
        final restored = await call('host_versions_restore', {
          'resourceId': id,
          'versionId': first['versionId'],
          'expectedSha256': current['sha256'],
        });
        expect(restored['backupVersionId'], isNotNull);
        expect(await file.readAsString(), '# 产品稿\nA');
        final history = await call('host_versions_list', {'resourceId': id});
        expect(history['versions'], hasLength(3));
        final backup = await call('host_versions_read', {
          'resourceId': id,
          'versionId': restored['backupVersionId'],
        });
        expect(backup['text'], '# 产品稿\nB');
        final png = File('${root.path}/picture.png');
        await png.writeAsBytes([137, 80, 78, 71, 0, 1]);
        final pngId = (listed['resources'] as List)[1]['id'];
        final binary = await call('host_versions_read', {'resourceId': pngId});
        expect(binary['binary'], true);
        expect(base64Decode(binary['base64']), [137, 80, 78, 71, 0, 1]);
        expect(
          binary['sha256'],
          sha256.convert([137, 80, 78, 71, 0, 1]).toString(),
        );
        expect(
          (await provider.call('host_versions_read', {
            'resourceId': 'forged',
          }))['isError'],
          true,
        );
        final readOnly = HostVersionCapability(
          controller,
          resources: ['product.md'],
          writable: false,
        );
        expect(
          readOnly.tools.any((t) => t['name'] == 'host_versions_restore'),
          false,
        );
        controller.flushBeforeDiskRead = (r) =>
            throw StateError('EDITOR_FLUSH_FAILED');
        expect(
          (await provider.call('host_versions_capture', {
            'resourceId': id,
          }))['isError'],
          true,
        );
      } finally {
        controller.dispose();
        await root.delete(recursive: true);
      }
    },
  );
}
