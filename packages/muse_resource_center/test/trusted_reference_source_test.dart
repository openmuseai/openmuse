import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';

ResourceIdentity id(String scope, {String source = 'trusted-cloud'}) =>
    ResourceIdentity(
      scope: scope,
      source: source,
      workspace: 'workspace',
      mount: 'pages',
      resource: 'doc',
    );
Matcher failure(String code) =>
    throwsA(isA<ResourceFailure>().having((e) => e.code, 'code', code));

final class Bridge implements ResourceReferenceSource {
  Bridge(this.hub, this.provider);
  final ResourceCenterHub hub;
  final Cloud provider;
  int calls = 0;
  Completer<void>? barrier;
  @override
  Future<void> prepare(ResourceReference ref, ResourceAccess access) async {
    calls++;
    if (ref.version != null) throw const ResourceFailure('VERSION_UNAVAILABLE');
    if (barrier != null) await barrier!.future;
    hub.register(provider);
  }
}

final class Cloud extends Fake implements ResourceProvider {
  Cloud(this.hub);
  final ResourceCenterHub hub;
  @override
  late final source = ResourceSource(
    scope: hub.scope,
    id: 'trusted-cloud',
    workspace: 'workspace',
    mount: 'pages',
    label: 'Cloud',
    workspaceLabel: 'W',
    mountLabel: 'Pages',
    deviceLabel: 'Cloud',
    local: false,
    exactVersions: false,
  );
  @override
  bool available = true;
  bool authorized = true;
  int reads = 0;
  Completer<void>? barrier;
  @override
  Future<ResourceContent> read(
    String resource,
    ResourceVersion? version,
    ResourceAccess access,
  ) async {
    reads++;
    if (barrier != null) await barrier!.future;
    if (!authorized) throw const ResourceFailure('DENIED');
    final bytes = [1, 2, 3], digest = resourceDigest([1, 2, 3]);
    return ResourceContent(
      ResourceRecord(
        identity: source.identity(resource),
        version: ResourceVersion(revision: 'device-$digest', digest: digest),
        descriptor: MuseResourceDescriptorV1(
          format: const MuseResourceFormat(
            formatId: 'text',
            confidence: MuseFormatConfidence.verified,
          ),
          capabilities: const [
            MuseResourceCapability.describe,
            MuseResourceCapability.snapshot,
          ],
          security: const MuseResourceSecurity(
            classification: MuseSecurityClassification.confidential,
            activeContent: MuseActiveContent.none,
          ),

          resourceRef: resource,
          revision: 'device-$digest',
          displayName: 'Doc',
          mediaType: 'text/plain',
        ),
        sourceLabel: 'Cloud',
        location: const [],
        parentRef: 'root',
        metadataRevision: 0,
      ),
      bytes,
    );
  }
}

void main() {
  late ResourceCenterHub hub;
  late Cloud cloud;
  late Bridge bridge;
  setUp(() {
    hub = ResourceCenterHub(scope: 'recipient');
    cloud = Cloud(hub);
    bridge = Bridge(hub, cloud);
  });
  tearDown(() async => hub.dispose());
  test(
    'only Host registered sources admit cross-account live references',
    () async {
      final foreign = ResourceReference(identity: id('sender'));
      await expectLater(
        hub.resolve(foreign, hub.access('ui')),
        failure('DENIED'),
      );
      hub.registerReferenceSource('trusted-cloud', bridge);
      final opened = await hub.resolve(foreign, hub.access('ui'));
      expect(opened.content.record.identity, id('recipient'));
      expect(opened.reference.identity, id('recipient'));
      expect(bridge.calls, 1);
      expect(cloud.reads, 1);
      await expectLater(
        hub.resolve(
          ResourceReference(identity: id('sender', source: 'phone')),
          hub.access('ui'),
        ),
        failure('DENIED'),
      );
      expect(cloud.reads, 1);
    },
  );
  test(
    'source registration cannot broaden resource limits or edit-only access',
    () async {
      hub.registerReferenceSource('trusted-cloud', bridge);
      final ref = ResourceReference(identity: id('sender'));
      for (final access in [
        hub.access('agent', resourceKeys: {id('sender').key}),
        hub.access('editor', operations: {ResourceOperation.edit}),
      ]) {
        await expectLater(hub.resolve(ref, access), failure('DENIED'));
      }
      expect(bridge.calls, 0);
      expect(cloud.reads, 0);
      await hub.resolve(
        ref,
        hub.access('ui', resourceKeys: {id('recipient').key}),
      );
      expect(cloud.reads, 1);
    },
  );
  test(
    'cross-account pins and retained copies never become live reads',
    () async {
      hub.registerReferenceSource('trusted-cloud', bridge);
      final version = ResourceVersion(
        revision: 'v1',
        digest: resourceDigest([1]),
      );
      for (final ref in [
        ResourceReference(identity: id('sender'), version: version),
        ResourceReference(
          identity: id('sender'),
          version: version,
          retained: RetainedResource(
            id('sender', source: 'personal-snapshots'),
            version,
          ),
        ),
      ]) {
        await expectLater(
          hub.resolve(ref, hub.access('ui', allowOfflineReplica: true)),
          failure('DENIED'),
        );
      }
      await expectLater(
        hub.resolve(
          ResourceReference(identity: id('recipient'), version: version),
          hub.access('ui'),
        ),
        failure('VERSION_UNAVAILABLE'),
      );
      expect(cloud.reads, 0);
    },
  );
  test(
    'every live resolve reads current source authorization after revocation',
    () async {
      hub.registerReferenceSource('trusted-cloud', bridge);
      final ref = ResourceReference(identity: id('sender'));
      await hub.resolve(ref, hub.access('ui'));
      cloud.authorized = false;
      await expectLater(
        hub.resolve(ref, hub.access('ui', allowOfflineReplica: true)),
        failure('DENIED'),
      );
      expect(cloud.reads, 2);
    },
  );
  test('unregistering an in-flight bridge fences its delayed result', () async {
    bridge.barrier = Completer<void>();
    hub.registerReferenceSource('trusted-cloud', bridge);
    final read = hub.resolve(
      ResourceReference(identity: id('sender')),
      hub.access('ui'),
    );
    final checked = expectLater(read, failure('SOURCE_UNAVAILABLE'));
    hub.unregisterReferenceSource('trusted-cloud', bridge);
    bridge.barrier!.complete();
    await checked;
    expect(cloud.reads, 0);
  });
  test(
    'account generation is checked again after a delayed remote read',
    () async {
      hub.registerReferenceSource('trusted-cloud', bridge);
      cloud.barrier = Completer<void>();
      final read = hub.resolve(
        ResourceReference(identity: id('sender')),
        hub.access('ui'),
      );
      final checked = expectLater(read, failure('STALE_GENERATION'));
      await Future<void>.delayed(Duration.zero);
      expect(cloud.reads, 1);
      await hub.invalidate();
      cloud.barrier!.complete();
      await checked;
      // A retired bridge cannot automatically repopulate an account's registry.
      await expectLater(
        hub.resolve(
          ResourceReference(identity: id('sender')),
          hub.access('ui'),
        ),
        failure('DENIED'),
      );
    },
  );
  test('duplicate bridges cannot replace a trusted source implicitly', () {
    hub.registerReferenceSource('trusted-cloud', bridge);
    expect(
      () => hub.registerReferenceSource('trusted-cloud', Bridge(hub, cloud)),
      failure('SOURCE_ALREADY_REGISTERED'),
    );
    hub.unregisterReferenceSource('trusted-cloud', Bridge(hub, cloud));
    expect(
      () => hub.registerReferenceSource('trusted-cloud', bridge),
      returnsNormally,
    );
  });
}
