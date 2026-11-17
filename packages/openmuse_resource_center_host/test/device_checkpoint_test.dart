import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_resource_notes/openmuse_resource_notes.dart';

final class CollaborativeSource extends Fake
    implements ResourceProvider, ResourceDeviceCheckpointProvider {
  @override
  final source = ResourceSource(
    scope: 'account',
    id: 'cloud',
    workspace: 'w',
    mount: 'docs',
    label: 'Cloud',
    workspaceLabel: 'W',
    mountLabel: 'Docs',
    deviceLabel: 'Cloud',
    local: false,
    canEdit: true,
    exactVersions: false,
  );
  @override
  bool get available => true;
  final bases = <ResourceRecord>[];
  int remoteCommits = 0;
  ResourceRecord record(List<int> bytes) {
    final digest = resourceDigest(bytes);
    return ResourceRecord(
      identity: source.identity('doc'),
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

        resourceRef: 'doc',
        revision: digest,
        displayName: 'Doc.md',
        mediaType: 'text/markdown',
      ),
      version: ResourceVersion(revision: digest, digest: digest),
      sourceLabel: 'Cloud',
      location: const [],
      parentRef: 'root',
      metadataRevision: 0,
    );
  }

  @override
  Future<ResourceRecord> checkpoint(
    ResourceRecord base,
    List<int> bytes,
    String key,
    ResourceAccess access,
  ) async {
    access.check(base.identity, ResourceOperation.edit, currentGeneration: 1);
    bases.add(base);
    return record(bytes);
  }

  @override
  Future<ResourceRecord> commit(
    String ref,
    ResourceVersion base,
    List<int> bytes,
    String key,
    ResourceAccess access,
  ) async {
    remoteCommits++;
    throw const ResourceFailure('COLLABORATION_REQUIRED');
  }
}

void main() {
  test(
    'editor checkpoints use each session base without a provider-wide remote CAS',
    () async {
      final provider = CollaborativeSource(),
          hub = ResourceCenterHub(scope: 'account');
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final plugin = OpenMuseResourceNotesPlugin();
      registry.install(plugin);
      await registry.activate(plugin.descriptor.id);
      hub.register(provider);
      final center = ResourceCenterController(hub: hub, registry: registry);
      final choice = ResourceHandlerChoice(
        plugin,
        plugin,
        plugin.resourceHandlers.first,
      );
      ResourceEditorSession session(List<int> bytes) =>
          center.createEditorSession(
            OpenedResource(
              reference: provider.record(bytes).reference(),
              content: ResourceContent(provider.record(bytes), bytes),
            ),
            choice,
          );
      final first = session([1]), second = session([2]);
      first.enterEditing();
      second.enterEditing();
      await first.commit([3]);
      await second.commit([4]);
      await first.commit([5]);
      expect(provider.bases.map((r) => r.version.digest), [
        resourceDigest([1]),
        resourceDigest([2]),
        resourceDigest([3]),
      ]);
      expect(provider.remoteCommits, 0);
      expect(first.record.version.digest, resourceDigest([5]));
      center.editorLifetime.invalidate();
      await expectLater(second.commit([6]), throwsA(isA<ResourceFailure>()));
      expect(provider.bases, hasLength(3));
      first.dispose();
      second.dispose();
      center.dispose();
      registry.dispose();
      await hub.dispose();
    },
  );
}
