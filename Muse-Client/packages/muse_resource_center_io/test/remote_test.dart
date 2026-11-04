import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:muse_resource_center_io/muse_resource_center_io.dart';

void main() {
  late Directory root;
  late ResourceCenterHub pc, phone;
  late LocalResourceProvider pcFiles, phoneFiles, snapshots;
  late ResourceRpcService service;
  late ResourceRpcHttpHost server;
  late ResourceHttpTransport transport;
  late ResourceRpcClient rpc;
  late ResourceOriginBridge bridge;
  setUp(() async {
    root = await Directory.systemTemp.createTemp('resource-cross-end-');
    pc = ResourceCenterHub(scope: 'account-1');
    phone = ResourceCenterHub(scope: 'account-1');
    pcFiles = await LocalResourceProvider.open(
      Directory('${root.path}/pc'),
      scope: pc.scope,
      label: 'PC workspace',
      deviceLabel: 'PC',
      workspace: 'workspace-1',
    );
    phoneFiles = await LocalResourceProvider.open(
      Directory('${root.path}/phone'),
      scope: phone.scope,
      label: 'Phone documents',
      deviceLabel: 'Phone',
    );
    snapshots = await LocalResourceProvider.open(
      Directory('${root.path}/snapshots'),
      scope: pc.scope,
      label: 'Conversation versions',
      workspace: 'workspace-1',
      deviceLabel: 'PC',
      editable: false,
      uniqueNames: false,
    );
    pc.register(pcFiles);
    pc.register(snapshots);
    phone.register(phoneFiles);
    service = ResourceRpcService(
      pc,
      snapshots: snapshots,
      readTimeout: const Duration(milliseconds: 500),
      peerStore: FileResourceStateStore(File('${root.path}/peers.json')),
    );
    server = ResourceRpcHttpHost(
      service: service,
      authorize: (request) async {
        if (request.headers.value('authorization') != 'Bearer acceptance-only')
          return null;
        return ResourcePeer(
          scope: 'account-1',
          deviceRef: 'phone-1',
          workspaceRef: 'workspace-1',
          expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
        );
      },
    );
    await server.start();
    transport = ResourceHttpTransport(
      Uri.parse('http://127.0.0.1:${server.port}/openmuse/resource-center/v1'),
      headers: () => {'authorization': 'Bearer acceptance-only'},
    );
    rpc = ResourceRpcClient(transport.call);
    bridge = ResourceOriginBridge(
      client: rpc,
      localHub: phone,
      pollInterval: const Duration(milliseconds: 20),
    );
  });
  tearDown(() async {
    await bridge.stop();
    transport.close();
    await service.close();
    await server.close();
    await pcFiles.close();
    await phoneFiles.close();
    await snapshots.close();
    await pc.dispose();
    await phone.dispose();
    await root.delete(recursive: true);
  });
  Future<ResourceRecord> mobileFile() => phoneFiles.create(
    parentRef: 'root',
    name: '手机笔记.md',
    mediaType: 'text/markdown',
    bytes: utf8.encode('Phone document V1'),
    idempotencyKey: 'phone-file',
    access: phone.access('phone-ui'),
  );
  test(
    'an authorized PC conversation replica is addressable on the phone, hidden from browsing and never writable',
    () async {
      final record = await mobileFile();
      final ref = await bridge.share(
        record,
        retain: true,
        idempotencyKey: 'retain-roundtrip',
      );
      for (final p in await rpc.catalog(
        expectedScope: phone.scope,
        generation: () => phone.generation,
      )) {
        phone.register(p, visible: !p.source.internal);
      }
      final copy = phone.provider(ref.retained!.identity);
      expect(copy.source.internal, isTrue);
      expect(phone.visibleProviders.contains(copy), isFalse);
      await bridge.stop();
      phoneFiles.available = false;
      final opened = await phone.resolve(
        ref,
        phone.access('phone-ui', allowOfflineReplica: true),
      );
      expect(opened.fromSnapshot, isTrue);
      expect(utf8.decode(opened.content.bytes), 'Phone document V1');
      await expectLater(
        copy.create(
          parentRef: 'root',
          name: 'injected.md',
          mediaType: 'text/plain',
          bytes: [],
          idempotencyKey: 'bad-create',
          access: phone.access('test'),
        ),
        throwsA(isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED')),
      );
      await expectLater(
        copy.list('root', phone.access('test')),
        throwsA(isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED')),
      );
    },
  );
  test(
    'PC → phone uses the same live provider operations, exact versions, rename and CAS',
    () async {
      final sources = await rpc.catalog(
        expectedScope: phone.scope,
        generation: () => phone.generation,
      );
      expect(sources.where((p) => !p.source.internal), hasLength(1));
      for (final p in sources) {
        phone.register(p, visible: !p.source.internal);
      }
      final remote = sources.firstWhere((p) => !p.source.internal);
      final r = await remote.create(
        parentRef: 'root',
        name: 'new.md',
        mediaType: 'text/markdown',
        bytes: utf8.encode('PC V1'),
        idempotencyKey: 'remote-create',
        access: phone.access('phone-ui'),
      );
      final v2 = await remote.commit(
        r.identity.resource,
        r.version,
        utf8.encode('PC V2'),
        'remote-edit',
        phone.access('phone-ui'),
      );
      expect(v2.version, isNot(r.version));
      expect(
        utf8.decode(
          (await remote.read(
            r.identity.resource,
            r.version,
            phone.access('phone-ui'),
          )).bytes,
        ),
        'PC V1',
      );
      final renamed = await remote.rename(
        r.identity.resource,
        'renamed.md',
        1,
        phone.access('phone-ui'),
      );
      expect(renamed.identity, r.identity);
      expect(
        (await remote.list(
          'root',
          phone.access('phone-ui'),
        )).entries.single.name,
        'renamed.md',
      );
    },
  );
  test(
    'phone → PC reverse read is limited to explicitly shared references',
    () async {
      final r = await mobileFile();
      final ref = await bridge.share(
        r,
        retain: false,
        idempotencyKey: 'share-1',
      );
      final result = await pc.resolve(ref, pc.access('pc-editor'));
      expect(utf8.decode(result.content.bytes), 'Phone document V1');
      final other = await phoneFiles.create(
        parentRef: 'root',
        name: 'private.md',
        mediaType: 'text/markdown',
        bytes: [42],
        idempotencyKey: 'private',
        access: phone.access('phone-ui'),
      );
      await expectLater(
        pc.resolve(other.reference(pin: true), pc.access('pc-editor')),
        throwsA(isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED')),
      );
    },
  );
  test(
    'temporary PC execution cleanup rehydrates the same phone version while connected',
    () async {
      final ref = await bridge.share(
        await mobileFile(),
        retain: false,
        idempotencyKey: 'share-1',
      );
      final projections = ResourceProjectionManager(
        Directory('${root.path}/task-temp'),
      );
      final first = await projections.materialize(
        await pc.resolve(ref, pc.access('pc-agent')),
      );
      final path = first.file.path;
      await projections.clear();
      expect(await File(path).exists(), false);
      final second = await projections.materialize(
        await pc.resolve(ref, pc.access('pc-editor')),
      );
      expect(second.file.path, isNot(path));
      expect(await second.file.readAsString(), 'Phone document V1');
      await projections.clear();
    },
  );
  test(
    'online/snapshot, offline/snapshot and offline/no-snapshot behave distinctly',
    () async {
      final r = await mobileFile();
      final live = await bridge.share(
        r,
        retain: false,
        idempotencyKey: 'share-1',
      );
      final retained = await bridge.share(
        r,
        retain: true,
        idempotencyKey: 'share-retained',
      );
      expect(
        (await pc.resolve(
          retained,
          pc.access('pc-ui', allowOfflineReplica: true),
        )).fromSnapshot,
        true,
      );
      await bridge.stop();
      final offline = await pc.resolve(
        retained,
        pc.access('pc-ui', allowOfflineReplica: true),
      );
      expect(offline.sourceOffline, true);
      expect(utf8.decode(offline.content.bytes), 'Phone document V1');
      await expectLater(
        pc.resolve(live, pc.access('pc-ui')),
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
  test('historical attachment stays V1 after phone commits V2', () async {
    final r = await mobileFile();
    final ref = await bridge.share(r, retain: false, idempotencyKey: 'share-1');
    await phoneFiles.commit(
      r.identity.resource,
      r.version,
      utf8.encode('Phone V2'),
      'edit-phone',
      phone.access('phone-ui'),
    );
    expect(
      utf8.decode((await pc.resolve(ref, pc.access('pc-ui'))).content.bytes),
      'Phone document V1',
    );
    expect(
      utf8.decode(
        (await pc.resolve(
          ResourceReference(identity: ref.identity),
          pc.access('pc-ui'),
        )).content.bytes,
      ),
      'Phone V2',
    );
  });
  test(
    'same account does not permit another peer to claim an origin source or answer its pending read',
    () async {
      final r = await mobileFile();
      await bridge.share(r, retain: false, idempotencyKey: 'share-1');
      final stranger = ResourcePeer(
        scope: pc.scope,
        deviceRef: 'another-phone',
        workspaceRef: 'workspace-1',
        expiresAtMs: DateTime.now().millisecondsSinceEpoch + 10000,
      );
      await expectLater(
        service.dispatch({
          'protocol': ResourceRpcService.protocol,
          'operation': 'share',
          'arguments': {
            'record': r.toJson(),
            'source': phoneFiles.source.toJson(),
          },
        }, stranger),
        throwsA(isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED')),
      );
      await expectLater(
        rpc.call('read', {'identity': r.identity.toJson()}),
        throwsA(isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED')),
      );
    },
  );
  test(
    'HTTP grant required and workspace scope cannot be widened by RPC arguments',
    () async {
      final denied = ResourceHttpTransport(
        transport.endpoint,
        headers: () => {},
      );
      await expectLater(
        ResourceRpcClient(denied.call).call('catalog'),
        throwsA(isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED')),
      );
      denied.close();
      final r = await pcFiles.create(
        parentRef: 'root',
        name: 'pc.txt',
        mediaType: 'text/plain',
        bytes: [1],
        idempotencyKey: 'pc-file',
        access: pc.access('pc-ui'),
      );
      final wrong = ResourceIdentity(
        scope: r.identity.scope,
        source: r.identity.source,
        workspace: 'another-workspace',
        mount: r.identity.mount,
        resource: r.identity.resource,
      );
      await expectLater(
        rpc.call('read', {'identity': wrong.toJson()}),
        throwsA(isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED')),
      );
    },
  );
  test(
    'retained copies remain after PC restart, while metadata alone never makes a phone online',
    () async {
      final ref = await bridge.share(
        await mobileFile(),
        retain: true,
        idempotencyKey: 'retain-1',
      );
      await bridge.stop();
      await service.close();
      final recovered = ResourceCenterHub(scope: pc.scope);
      recovered.register(snapshots);
      final restarted = ResourceRpcService(
        recovered,
        snapshots: snapshots,
        peerStore: service.peerStore,
      );
      await restarted.restore();
      final result = await recovered.resolve(
        ref,
        recovered.access('pc-ui', allowOfflineReplica: true),
      );
      expect(result.sourceOffline, true);
      expect(utf8.decode(result.content.bytes), 'Phone document V1');
      await expectLater(
        recovered.resolve(
          ResourceReference(identity: ref.identity, version: ref.version),
          recovered.access('pc-ui'),
        ),
        throwsA(isA<ResourceFailure>()),
      );
      await restarted.close();
      await recovered.dispose();
    },
  );
}
