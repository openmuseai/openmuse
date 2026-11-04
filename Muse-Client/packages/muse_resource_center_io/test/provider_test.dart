import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:muse_resource_center_io/muse_resource_center_io.dart';

void main() {
  late Directory root;
  late LocalResourceProvider phone, snapshots;
  late ResourceCenterHub hub;
  late ResourceAccess owner;
  setUp(() async {
    root = await Directory.systemTemp.createTemp('openmuse-resource-tck-');
    hub = ResourceCenterHub(
      scope: 'account-1',
      store: FileResourceStateStore(File('${root.path}/state/history.json')),
    );
    phone = await LocalResourceProvider.open(
      Directory('${root.path}/phone'),
      scope: hub.scope,
      label: 'Phone documents',
      deviceLabel: 'Phone',
      generation: () => hub.generation,
    );
    snapshots = await LocalResourceProvider.open(
      Directory('${root.path}/snapshots'),
      scope: hub.scope,
      label: 'Conversation snapshots',
      deviceLabel: 'PC',
      editable: false,
      generation: () => hub.generation,
    );
    hub.register(phone);
    hub.register(snapshots);
    owner = hub.access('owner-ui', allowOfflineReplica: true);
  });
  tearDown(() async {
    await phone.close();
    await snapshots.close();
    await hub.dispose();
    await root.delete(recursive: true);
  });
  Future<ResourceRecord> create({
    String name = 'notes.md',
    String text = 'V1',
    String key = 'create-1',
  }) => phone.create(
    parentRef: 'root',
    name: name,
    mediaType: 'text/markdown',
    bytes: utf8.encode(text),
    idempotencyKey: key,
    access: owner,
  );
  test(
    'create is durable and idempotent, preserving resource identity over restart',
    () async {
      final r = await create();
      expect((await create()).identity, r.identity);
      expect((await phone.list('root', owner)).entries, hasLength(1));
      final id = phone.source.id;
      await phone.close();
      phone = await LocalResourceProvider.open(
        Directory('${root.path}/phone'),
        scope: hub.scope,
        label: 'Phone documents',
        deviceLabel: 'Phone',
        generation: () => hub.generation,
      );
      expect(phone.source.id, id);
      expect(
        (await phone.describe(r.identity.resource, owner)).identity,
        r.identity,
      );
      expect(
        utf8.decode(
          (await phone.read(r.identity.resource, r.version, owner)).bytes,
        ),
        'V1',
      );
    },
  );
  test(
    'an idempotency key cannot be reused with a different operation payload',
    () async {
      await create();
      await expectLater(
        create(text: 'changed'),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'IDEMPOTENCY_CONFLICT',
          ),
        ),
      );
      expect((await phone.list('root', owner)).entries, hasLength(1));
    },
  );
  test(
    'edit CAS, retry and exact historical version never substitute the current version',
    () async {
      final first = await create();
      final second = await phone.commit(
        first.identity.resource,
        first.version,
        utf8.encode('V2'),
        'edit-1',
        owner,
      );
      expect(second.identity, first.identity);
      expect(second.version, isNot(first.version));
      expect(
        (await phone.commit(
          first.identity.resource,
          first.version,
          utf8.encode('V2'),
          'edit-1',
          owner,
        )).version,
        second.version,
      );
      expect(
        utf8.decode(
          (await phone.read(
            first.identity.resource,
            first.version,
            owner,
          )).bytes,
        ),
        'V1',
      );
      expect(
        utf8.decode(
          (await phone.read(first.identity.resource, null, owner)).bytes,
        ),
        'V2',
      );
      await expectLater(
        phone.commit(
          first.identity.resource,
          first.version,
          [3],
          'edit-stale',
          owner,
        ),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'REVISION_CONFLICT',
          ),
        ),
      );
    },
  );
  test(
    'rename preserves resource/content identity and rejects stale metadata',
    () async {
      final r = await create();
      final renamed = await phone.rename(
        r.identity.resource,
        'renamed.md',
        1,
        owner,
      );
      expect(renamed.identity, r.identity);
      expect(renamed.version, r.version);
      expect(renamed.name, 'renamed.md');
      await expectLater(
        phone.rename(r.identity.resource, 'other.md', 1, owner),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'METADATA_CONFLICT',
          ),
        ),
      );
    },
  );
  test(
    'folders and pagination are stable; traversal never becomes a filesystem path',
    () async {
      final folder = await phone.createDirectory('root', 'Folder', owner);
      await phone.create(
        parentRef: folder,
        name: 'inside.txt',
        mediaType: 'text/plain',
        bytes: [1],
        idempotencyKey: 'inside',
        access: owner,
      );
      await create();
      final one = await phone.list('root', owner, limit: 1);
      expect(one.entries.single.isDirectory, true);
      expect(one.nextCursor, isNotNull);
      final two = await phone.list(
        'root',
        owner,
        cursor: one.nextCursor,
        limit: 1,
      );
      expect(two.entries.single.isDirectory, false);
      expect(
        (await phone.list(folder, owner)).entries.single.name,
        'inside.txt',
      );
      await expectLater(
        create(name: '../escape'),
        throwsA(isA<ResourceFailure>()),
      );
      await expectLater(
        phone.list('../root', owner),
        throwsA(isA<ResourceFailure>()),
      );
      expect(await File('${root.parent.path}/escape').exists(), false);
    },
  );
  test(
    'snapshot keeps exact V1 after source edit and original goes offline',
    () async {
      final r = await create();
      final pinned = await hub.retain(
        r.reference(pin: true),
        snapshots,
        owner,
        name: 'snapshot.md',
        idempotencyKey: 'retain-1',
      );
      await phone.commit(
        r.identity.resource,
        r.version,
        utf8.encode('V2'),
        'edit-1',
        owner,
      );
      phone.available = false;
      final opened = await hub.resolve(pinned, owner);
      expect(opened.fromSnapshot, true);
      expect(opened.sourceOffline, true);
      expect(utf8.decode(opened.content.bytes), 'V1');
      expect(opened.content.record.readOnly, true);
      await expectLater(
        snapshots.commit(
          opened.content.record.identity.resource,
          opened.content.record.version,
          [2],
          'attempt-edit',
          owner,
        ),
        throwsA(
          isA<ResourceFailure>().having((e) => e.code, 'code', 'READ_ONLY'),
        ),
      );
    },
  );
  test(
    'availability matrix: online/no-copy recovers; offline/no-copy preserves reference and fails',
    () async {
      final r = await create();
      final ref = r.reference(pin: true);
      expect(utf8.decode((await hub.resolve(ref, owner)).content.bytes), 'V1');
      phone.available = false;
      await expectLater(
        hub.resolve(ref, owner),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'SOURCE_UNAVAILABLE',
          ),
        ),
      );
      phone.available = true;
      expect((await hub.resolve(ref, owner)).content.record.version, r.version);
    },
  );
  test(
    'offline snapshot requires explicit offline-replica permission',
    () async {
      final r = await create();
      final ref = await hub.retain(
        r.reference(pin: true),
        snapshots,
        owner,
        name: 'snapshot.md',
        idempotencyKey: 'retain-1',
      );
      phone.available = false;
      await expectLater(
        hub.resolve(ref, hub.access('agent-without-offline-grant')),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'SOURCE_UNAVAILABLE',
          ),
        ),
      );
    },
  );
  test(
    'missing historical version cannot fall back to V2, even while source is online',
    () async {
      final r = await create();
      final fake = ResourceVersion(
        revision: 'missing-version',
        digest: r.version.digest,
      );
      await expectLater(
        hub.resolve(
          ResourceReference(identity: r.identity, version: fake),
          owner,
        ),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'VERSION_UNAVAILABLE',
          ),
        ),
      );
    },
  );
  test(
    'temporary execution projection may be deleted and recreated without changing the reference',
    () async {
      final r = await create();
      final manager = ResourceProjectionManager(
        Directory('${root.path}/execution'),
      );
      await manager.root.create();
      final unrelated = File('${manager.root.path}/keep.txt');
      await unrelated.writeAsString('keep');
      final resolved = await hub.resolve(r.reference(pin: true), owner);
      final first = await manager.materialize(resolved);
      final old = first.file.path;
      expect(await first.file.readAsString(), 'V1');
      expect(old.endsWith('.md'), true);
      await manager.clear();
      expect(await File(old).exists(), false);
      final next = await manager.materialize(
        await hub.resolve(r.reference(pin: true), owner),
      );
      expect(next.file.path, isNot(old));
      expect(await next.file.readAsString(), 'V1');
      expect(await unrelated.exists(), true);
      expect(
        jsonEncode(r.reference(pin: true).toJson()),
        isNot(contains(root.path)),
      );
      await manager.clear();
    },
  );
  test(
    'cross-account, expired and stale-generation reads are denied',
    () async {
      final r = await create();
      final foreign = ResourceAccess(
        scope: 'account-2',
        audience: 'editor',
        generation: 1,
        expiresAtMs: DateTime.now().millisecondsSinceEpoch + 10000,
        operations: {ResourceOperation.read},
      );
      await expectLater(
        phone.read(r.identity.resource, null, foreign),
        throwsA(isA<ResourceFailure>()),
      );
      final expired = ResourceAccess(
        scope: hub.scope,
        audience: 'editor',
        generation: 1,
        expiresAtMs: DateTime.now().millisecondsSinceEpoch - 1,
        operations: {ResourceOperation.read},
      );
      await expectLater(
        phone.read(r.identity.resource, null, expired),
        throwsA(isA<ResourceFailure>()),
      );
      hub.generation++;
      await expectLater(
        phone.read(r.identity.resource, null, owner),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'STALE_GENERATION',
          ),
        ),
      );
    },
  );
  test(
    'corrupted or symlinked blobs cannot be treated as valid content',
    () async {
      final r = await create();
      final blob = File('${root.path}/phone/blobs/${r.version.digest}');
      await blob.writeAsBytes([0]);
      await expectLater(
        phone.read(r.identity.resource, r.version, owner),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'DIGEST_MISMATCH',
          ),
        ),
      );
      await blob.delete();
      await Link(blob.path).create('${root.path}/state/history.json');
      await expectLater(
        phone.read(r.identity.resource, r.version, owner),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'UNSAFE_STORAGE',
          ),
        ),
      );
    },
  );
  test(
    'metadata failure does not publish a resource or cache a successful create receipt',
    () async {
      final ledger = File('${root.path}/phone/ledger.json');
      final backup = await ledger.rename('${root.path}/phone/backup.json');
      final blocker = await Directory(ledger.path).create();
      await expectLater(create(), throwsA(isA<FileSystemException>()));
      expect((await phone.list('root', owner)).entries, isEmpty);
      await blocker.delete();
      await backup.rename(ledger.path);
      final r = await create();
      expect(
        (await phone.describe(r.identity.resource, owner)).name,
        'notes.md',
      );
    },
  );
  test(
    'recent history is success-event based, deduplicates and persists by account',
    () async {
      final r = await create();
      expect(hub.recent, isEmpty);
      await hub.resolve(r.reference(), owner);
      expect(hub.recent, isEmpty);
      await hub.recordOpened(r, 'notes');
      await hub.recordOpened(r, 'notes');
      expect(hub.recent, hasLength(1));
      final restored = ResourceCenterHub(scope: hub.scope, store: hub.store);
      await restored.load();
      expect(restored.recent.single.record.identity, r.identity);
      await restored.dispose();
      final foreign = ResourceCenterHub(
        scope: 'different-account',
        store: hub.store,
      );
      await foreign.load();
      expect(foreign.recent, isEmpty);
      await foreign.dispose();
      hub.copy(r.reference());
      expect(hub.clipboard, isNotNull);
      await hub.invalidate();
      expect(hub.clipboard, isNull);
      expect(hub.recent, isEmpty);
    },
  );
}
