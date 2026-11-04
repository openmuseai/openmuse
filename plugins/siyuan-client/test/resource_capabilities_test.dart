import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_siyuan_plugin/openmuse_siyuan_plugin.dart';

ResourceRecord record(String name, String mime) => ResourceRecord(
  identity: ResourceIdentity(
    scope: 'test',
    source: 'local',
    workspace: 'w',
    mount: 'm',
    resource: 'r',
  ),
  version: ResourceVersion(revision: 'v1', digest: resourceDigest([])),
  descriptor: MuseResourceDescriptorV1(
    resourceRef: 'r',
    revision: 'v1',
    displayName: name,
    mediaType: mime,
    format: MuseResourceFormat(
      formatId: mime,
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
void main() {
  final plugin = OpenMuseSiYuanPlugin();
  test(
    'resource plugin contributes capabilities without a nested application panel',
    () {
      expect(plugin.descriptor.panels, isEmpty);
      expect(plugin, isNot(isA<OpenMuseMobileWorkbenchContributor>()));
      expect(plugin.resourceCreators.single.extension, 'sydoc');
      expect(plugin.resourceHandlers.any((h) => h.editable), isTrue);
    },
  );
  test(
    'archive routing declines unrelated ZIP and Markdown resources',
    () async {
      expect(
        await plugin.probeResource(
          record('archive.zip', 'application/zip'),
          ResourceOpenIntent.view,
        ),
        isFalse,
      );
      expect(
        await plugin.probeResource(
          record('notes.md', 'text/markdown'),
          ResourceOpenIntent.edit,
        ),
        isFalse,
      );
      for (final name in ['Note.sydoc', 'Notebook.sy.zip']) {
        final value = record(name, 'application/zip');
        expect(plugin.resourceHandlers.any((h) => h.accepts(value)), isTrue);
        expect(
          await plugin.probeResource(value, ResourceOpenIntent.edit),
          isTrue,
        );
      }
    },
  );
  test(
    'standalone sy is explicitly read only while a complete note package can edit',
    () async {
      final value = record('raw.sy', 'application/json');
      expect(
        plugin.resourceHandlers.where((h) => h.accepts(value)).single.editable,
        isFalse,
      );
      expect(
        await plugin.probeResource(value, ResourceOpenIntent.view),
        isTrue,
      );
      expect(
        await plugin.probeResource(value, ResourceOpenIntent.edit),
        isFalse,
      );
    },
  );
}
