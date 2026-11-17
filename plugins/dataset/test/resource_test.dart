import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_dataset_plugin/openmuse_dataset_plugin.dart';

class Memory implements SnapshotStore {
  Map<String, dynamic>? data;
  @override
  Future<Map<String, dynamic>?> read() async => data;
  @override
  Future<void> write(Map<String, dynamic> value) async {
    data = value;
  }
}

ResourceEditorSession session(
  List<int> bytes, {
  bool historical = false,
  String account = 'test',
  String resourceId = 'r',
  String revision = 'v1',
  bool allowEditing = true,
  ResourceSessionLifetime? lifetime,
  Future<void> Function()? beforeCommit,
}) {
  final record = ResourceRecord(
    identity: ResourceIdentity(
      scope: account,
      source: 'local',
      workspace: 'w',
      mount: 'm',
      resource: resourceId,
    ),
    version: ResourceVersion(revision: revision, digest: resourceDigest(bytes)),
    descriptor: MuseResourceDescriptorV1(
      resourceRef: resourceId,
      revision: revision,
      displayName: 'Resource',
      mediaType: datasetMediaType,
      format: const MuseResourceFormat(
        formatId: 'text',
        confidence: MuseFormatConfidence.claimed,
      ),
      capabilities: const [
        MuseResourceCapability.describe,
        MuseResourceCapability.materialize,
      ],
      security: const MuseResourceSecurity(
        classification: MuseSecurityClassification.internal,
        activeContent: MuseActiveContent.unknown,
      ),
    ),
    sourceLabel: 'Local',
    location: const ['Documents'],
    parentRef: 'root',
    metadataRevision: 1,
  );
  return ResourceEditorSession(
    opened: OpenedResource(
      reference: ResourceReference(identity: record.identity),
      content: ResourceContent(record, bytes),
      fromSnapshot: historical,
    ),
    allowEditing: allowEditing,
    lifetime: lifetime,
    commitContent: (base, next, _) async {
      await beforeCommit?.call();
      return ResourceRecord.fromJson({
        ...record.toJson(),
        'descriptor': {
          ...record.descriptor.toJson(),
          'revision': '${base.revision}-next',
        },
        'version': ResourceVersion(
          revision: '${base.revision}-next',
          digest: resourceDigest(next),
        ).toJson(),
      });
    },
    onOpened: (_) {},
    onChanged: (_) {},
  );
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  test(
    'account ending during a checkpoint keeps private data and closes its lease',
    () async {
      final entered = Completer<void>(), release = Completer<void>();
      final lifetime = ResourceSessionLifetime(), store = Memory();
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('dataset.database', {});
      final editor = session(
        seed.bytes,
        lifetime: lifetime,
        beforeCommit: () async {
          if (!entered.isCompleted) entered.complete();
          await release.future;
          if (!lifetime.active)
            throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
        },
      );
      final lease = await plugin.acquireResourceSession(editor);
      await lease.controller.addRow(
        cells: {'title': 'Private dataset draft after account ended'},
      );
      final closing = lease.close();
      await entered.future;
      lifetime.invalidate();
      release.complete();
      await closing;
      expect(lease.isClosed, isTrue);
      final recovered = session(seed.bytes);
      final next = await plugin.acquireResourceSession(recovered);
      expect(
        next.controller.views.single.rows.single.cells['title']!.value,
        'Private dataset draft after account ended',
      );
      await next.close();
      recovered.dispose();
      expect(editor.dirty, isTrue);
      await plugin.deactivate();
      editor.dispose();
    },
  );
  test(
    'public SDK resource keeps original row objects, uses stable scoped store and refuses duplicate owner',
    () async {
      final store = Memory();
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => store,
      );
      final seed = await plugin.createResource('dataset.database', {});
      final editor = session(seed.bytes);
      final c = await plugin.openController(editor);
      final id = await c.addRow(cells: {'title': '资源记录😀'});
      await expectLater(
        plugin.openController(editor),
        throwsA(isA<ResourceFailure>()),
      );
      final bytes = OpenMuseDatasetPlugin.exportBundle(c.bundle);
      final bundle = OpenMuseDatasetPlugin.decode(bytes);
      expect((bundle['objects'] as List).map((o) => o['collabType']).toSet(), {
        1,
        4,
      });
      expect(bundle.toString(), isNot(contains('accessToken')));
      await plugin.release(editor, c);
      final reopened = await plugin.openController(editor);
      expect(reopened.views.single.rows.single.id, id);
      expect(reopened.views.single.rows.single.cells['title']!.value, '资源记录😀');
      await plugin.release(editor, reopened);
      editor.dispose();
    },
  );
  test(
    'embedded public session retains owner and failed resource checkpoint for explicit retry',
    () async {
      var fail = true;
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => Memory(),
      );
      final seed = await plugin.createResource('dataset.database', {});
      final editor = session(
        seed.bytes,
        beforeCommit: () async {
          if (fail) throw StateError('resource offline');
        },
      );
      final lease = await plugin.openEmbeddedSession(editor);
      await lease.controller.addRow(cells: {'title': '未丢失的草稿'});
      await expectLater(lease.close(), throwsStateError);
      expect(lease.checkpointFailure?.call(), isNotNull);
      expect(await editor.prepareToClose(), isFalse);
      expect(
        plugin.activeController(editor.record.identity),
        same(lease.controller),
      );
      final borrowedEditor = session(seed.bytes);
      final borrowed = await plugin.openEmbeddedSession(borrowedEditor);
      expect(borrowed.controller, same(lease.controller));
      expect(borrowed.checkpointFailure?.call(), isNotNull);
      fail = false;
      await lease.flushCheckpoint!();
      expect(lease.checkpointFailure?.call(), isNull);
      expect(editor.dirty, isFalse);
      final exported = OpenMuseDatasetPlugin.decode(editor.bytes);
      expect(
        (exported['objects'] as List).map((o) => o['collabType']).toSet(),
        {1, 4},
      );
      await borrowed.close();
      borrowedEditor.dispose();
      await lease.close();
      expect(plugin.activeController(editor.record.identity), isNull);
      final reopened = await plugin.openController(editor);
      expect(reopened.views.single.rows.single.cells['title']!.value, '未丢失的草稿');
      await plugin.release(editor, reopened);
      editor.dispose();
    },
  );
  test(
    'historical resource ignores live cache and collaboration capabilities and stays read only',
    () async {
      final store = Memory();
      var storeCalls = 0, networkCalls = 0;
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async {
          storeCalls++;
          return store;
        },
        collaborationFactory: (_) {
          networkCalls++;
          return null;
        },
      );
      final seed = await plugin.createResource('dataset.database', {});
      final live = session(seed.bytes);
      final c = await plugin.openController(live);
      await c.addRow(cells: {'title': '后来的内容'});
      await plugin.release(live, c);
      final beforeStore = storeCalls, beforeNetwork = networkCalls;
      final historical = session(seed.bytes, historical: true);
      final old = await plugin.openController(historical);
      expect(old.views.single.rows, isEmpty);
      expect(old.canWrite, isFalse);
      expect(storeCalls, beforeStore);
      expect(networkCalls, beforeNetwork);
      await expectLater(old.addRow(), throwsA(isA<DatasetFailure>()));
      await plugin.release(historical, old);
      live.dispose();
      historical.dispose();
    },
  );
  test(
    'live database coexists with two pins and account-isolated recovery',
    () async {
      final stores = <ResourceSessionScope, Memory>{};
      var storeCalls = 0, networkCalls = 0;
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (s) async {
          storeCalls++;
          return stores.putIfAbsent(s.ownershipScope, Memory.new);
        },
        collaborationFactory: (_) {
          networkCalls++;
          return null;
        },
      );
      final seed = await plugin.createResource('dataset.database', {});
      final live = session(seed.bytes),
          pin = session(seed.bytes, historical: true);
      final pin2 = session(seed.bytes, historical: true, revision: 'v2');
      final otherAccount = session(seed.bytes, account: 'other');
      final c = await plugin.openController(live);
      final rowId = await c.addRow(cells: {'title': '当前账号草稿😀'});
      final calls = [storeCalls, networkCalls];
      final old = await plugin.openController(pin),
          old2 = await plugin.openController(pin2);
      expect([storeCalls, networkCalls], calls);
      expect(old.views.single.rows, isEmpty);
      expect(old2, isNot(same(old)));
      expect(old.canWrite, isFalse);
      await expectLater(old.addRow(), throwsA(isA<DatasetFailure>()));
      final other = await plugin.openController(otherAccount);
      expect(other.views.single.rows, isEmpty);
      await expectLater(
        plugin.release(pin, c),
        throwsA(isA<ResourceFailure>()),
      );
      expect(plugin.activeController(live.record.identity), same(c));
      expect(
        plugin.activeController(
          pin.record.identity,
          snapshotVersion: pin.record.version,
        ),
        same(old),
      );
      await plugin.release(pin, old);
      await plugin.release(pin2, old2);
      expect(c.views.single.rows.single.id, rowId);
      await c.addRow(cells: {'title': '历史关闭后继续编辑'});
      await plugin.release(otherAccount, other);
      await plugin.release(live, c);
      for (final s in [live, pin, pin2, otherAccount]) {
        s.dispose();
      }
      await plugin.deactivate();
    },
  );
  test(
    'standalone and two child leases keep one owner after original UI disposal',
    () async {
      var stores = 0, transports = 0;
      final versions = <String>[];
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async {
          stores++;
          return Memory();
        },
        realtimeFactory: (_, _) async {
          transports++;
          return null;
        },
      );
      final seed = await plugin.createResource('dataset.database', {});
      final lifetime = ResourceSessionLifetime();
      final first = session(seed.bytes, lifetime: lifetime),
          second = session(seed.bytes, lifetime: lifetime),
          third = session(seed.bytes, lifetime: lifetime);
      final leases = await Future.wait([
        plugin.acquireResourceSession(first),
        plugin.acquireResourceSession(second),
        plugin.acquireResourceSession(third),
      ]);
      expect(leases[1].controller, same(leases[0].controller));
      expect(leases[2].controller, same(leases[0].controller));
      expect(stores, 1);
      expect(transports, 1);
      final c = leases[0].controller;
      await expectLater(
        plugin.release(first, c),
        throwsA(isA<ResourceFailure>()),
      );
      expect(plugin.activeController(first.record.identity), same(c));
      final id = await c.addRow(cells: {'title': '同一原始记录😀'});
      await leases[0].checkpoint.flush();
      versions.add(first.record.version.revision);
      expect(second.record.version, first.record.version);
      await leases[0].close();
      first.dispose();
      expect(
        first.isActive,
        isTrue,
        reason: 'owner retention survives its original presentation',
      );
      await c.addRow(cells: {'title': '独立页关闭后继续编辑'});
      await leases[1].checkpoint.flush();
      versions.add(second.record.version.revision);
      expect(versions.toSet(), hasLength(2));
      expect(c.views.first.rows.first.id, id);
      await leases[1].close();
      second.dispose();
      expect(plugin.activeController(third.record.identity), same(c));
      await c.addRow(cells: {'title': '最后的嵌入仍可编辑'});
      await leases[2].close();
      await leases[2].close();
      third.dispose();
      expect(plugin.activeController(first.record.identity), isNull);
      expect(first.isActive, isFalse);
      await plugin.deactivate();
    },
  );
  test(
    'last lease failed checkpoint retains owner and idempotent retry',
    () async {
      var fail = true;
      final keys = <String>[];
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => Memory(),
      );
      final seed = await plugin.createResource('dataset.database', {});
      final base = session(seed.bytes);
      final editor = ResourceEditorSession(
        opened: base.opened,
        allowEditing: true,
        commitContent: (version, bytes, key) async {
          keys.add(key);
          if (fail) throw const ResourceFailure('REVISION_CONFLICT');
          return base.commitContent(version, bytes, key);
        },
        onOpened: (_) {},
        onChanged: (_) {},
      );
      final lease = await plugin.acquireResourceSession(editor);
      await lease.controller.addRow(cells: {'title': '失败时保留的记录'});
      editor.dispose();
      await expectLater(lease.close(), throwsA(isA<ResourceFailure>()));
      expect(
        plugin.activeController(editor.record.identity),
        same(lease.controller),
      );
      expect(editor.dirty, isTrue);
      expect(editor.isActive, isTrue);
      fail = false;
      await lease.close();
      expect(keys, hasLength(2));
      expect(keys.toSet(), hasLength(1));
      expect(editor.isActive, isFalse);
      base.dispose();
      await plugin.deactivate();
    },
  );
  test(
    'a read-only lease cannot widen UI capabilities and lifetime invalidation is terminal',
    () async {
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => Memory(),
      );
      final seed = await plugin.createResource('dataset.database', {});
      final lifetime = ResourceSessionLifetime();
      final first = session(seed.bytes, lifetime: lifetime),
          reader = session(seed.bytes, lifetime: lifetime, allowEditing: false);
      final writer = await plugin.acquireResourceSession(first),
          read = await plugin.acquireResourceSession(reader);
      expect(read.controller, same(writer.controller));
      expect(read.allowEditing, isFalse);
      expect(reader.editing, isFalse);
      final wrongLifetime = session(
        seed.bytes,
        lifetime: ResourceSessionLifetime(),
      );
      await expectLater(
        plugin.acquireResourceSession(wrongLifetime),
        throwsA(isA<ResourceFailure>()),
      );
      final replacement = await plugin.createResource('dataset.database', {});
      final wrongContent = session(replacement.bytes, lifetime: lifetime);
      await expectLater(
        plugin.acquireResourceSession(wrongContent),
        throwsA(isA<ResourceFailure>()),
      );
      lifetime.invalidate();
      expect(writer.controller.canWrite, isFalse);
      expect(writer.controller.canRead, isFalse);
      writer.controller.acceptRealtimeAccess(
        id: writer.controller.objectId,
        collabType: 1,
        canRead: true,
        canWrite: true,
        reason: 'grant',
      );
      expect(writer.controller.canWrite, isFalse);
      await expectLater(
        writer.controller.addRow(),
        throwsA(isA<DatasetFailure>()),
      );
      await expectLater(
        plugin.acquireResourceSession(first),
        throwsA(isA<ResourceFailure>()),
      );
      await read.close();
      await writer.close();
      for (final s in [first, reader, wrongLifetime, wrongContent]) {
        s.dispose();
      }
      await plugin.deactivate();
    },
  );
  test(
    'deactivation during opening never publishes an owner after shutdown',
    () async {
      final gate = Completer<void>();
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async {
          await gate.future;
          return Memory();
        },
      );
      final seed = await plugin.createResource('dataset.database', {});
      final editor = session(seed.bytes);
      final pending = plugin.acquireResourceSession(editor);
      final rejected = expectLater(pending, throwsA(isA<ResourceFailure>()));
      final closing = plugin.deactivate();
      gate.complete();
      await rejected;
      await closing;
      expect(plugin.activeController(editor.record.identity), isNull);
      editor.dispose();
    },
  );
  test(
    'deactivation racing the last lease shares one close operation',
    () async {
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => Memory(),
      );
      final seed = await plugin.createResource('dataset.database', {});
      final editor = session(seed.bytes);
      final lease = await plugin.acquireResourceSession(editor);
      await lease.controller.addRow();
      await Future.wait([lease.close(), plugin.deactivate()]);
      await lease.close();
      expect(plugin.activeController(editor.record.identity), isNull);
      editor.dispose();
    },
  );
  test(
    'last lease commits a late pre-fence edit and refuses edits inside the final fence',
    () async {
      final entered = Completer<void>(),
          release = Completer<void>(),
          finalEntered = Completer<void>(),
          finalRelease = Completer<void>();
      var commits = 0;
      final plugin = OpenMuseDatasetPlugin(
        core: NativeDatasetCore(),
        storeFactory: (_) async => Memory(),
      );
      final seed = await plugin.createResource('dataset.database', {});
      final base = session(seed.bytes);
      final editor = ResourceEditorSession(
        opened: base.opened,
        allowEditing: true,
        commitContent: (version, bytes, key) async {
          commits++;
          if (commits == 1) {
            entered.complete();
            await release.future;
          } else if (commits == 2) {
            finalEntered.complete();
            await finalRelease.future;
          }
          return base.commitContent(version, bytes, key);
        },
        onOpened: (_) {},
        onChanged: (_) {},
      );
      final lease = await plugin.acquireResourceSession(editor);
      await lease.controller.addRow(cells: {'title': '第一笔'});
      final closing = lease.close();
      await entered.future;
      await lease.controller.addRow(cells: {'title': '冻结前最后一笔'});
      release.complete();
      await finalEntered.future;
      expect(lease.controller.canWrite, isFalse);
      await expectLater(
        lease.controller.addRow(),
        throwsA(isA<DatasetFailure>()),
      );
      finalRelease.complete();
      await closing;
      expect(commits, 2);
      final reopened = await plugin.openController(editor);
      expect(reopened.views.first.rows, hasLength(2));
      await plugin.release(editor, reopened);
      editor.dispose();
      base.dispose();
      await plugin.deactivate();
    },
  );
}
