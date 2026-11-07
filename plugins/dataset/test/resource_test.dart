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
  Future<void> Function()? beforeCommit,
}) {
  final record = ResourceRecord(
    identity: ResourceIdentity(
      scope: 'test',
      source: 'local',
      workspace: 'w',
      mount: 'm',
      resource: 'r',
    ),
    version: ResourceVersion(revision: 'v1', digest: resourceDigest(bytes)),
    descriptor: MuseResourceDescriptorV1(
      resourceRef: 'r',
      revision: 'v1',
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
    allowEditing: true,
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
      await expectLater(
        plugin.openEmbeddedSession(editor),
        throwsA(isA<ResourceFailure>()),
      );
      fail = false;
      await lease.flushCheckpoint!();
      expect(lease.checkpointFailure?.call(), isNull);
      expect(editor.dirty, isFalse);
      final exported = OpenMuseDatasetPlugin.decode(editor.bytes);
      expect(
        (exported['objects'] as List).map((o) => o['collabType']).toSet(),
        {1, 4},
      );
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
}
