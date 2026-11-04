import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/mcp_file_plugin.dart';
import 'package:openmuse_host/src/host/mcp_market.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

const mime =
    'application/vnd.openxmlformats-officedocument.presentationml.presentation';

ResourceRecord record({String revision = 'v1', List<int> bytes = const [1]}) =>
    ResourceRecord(
      identity: ResourceIdentity(
        scope: 'qa',
        source: 'local',
        workspace: 'w',
        mount: 'm',
        resource: 'r',
      ),
      version: ResourceVersion(
        revision: revision,
        digest: resourceDigest(bytes),
      ),
      descriptor: MuseResourceDescriptorV1(
        resourceRef: 'r',
        revision: revision,
        displayName: 'Deck.pptx',
        mediaType: mime,
        format: const MuseResourceFormat(
          formatId: 'pptx',
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

ResourceEditorSession session({
  bool editable = true,
  required Future<ResourceRecord> Function(ResourceVersion, List<int>, String)
  commit,
}) {
  final initial = record();
  return ResourceEditorSession(
    opened: OpenedResource(
      reference: ResourceReference(identity: initial.identity),
      content: ResourceContent(initial, [1]),
    ),
    allowEditing: editable,
    commitContent: commit,
    onOpened: (_) {},
    onChanged: (_) {},
  );
}

Map<String, Object?> download({String mediaType = mime}) => {
  'contents': [
    {
      'type': 'resource',
      'resource': {
        'uri': 'file:///untrusted/path.pptx',
        'mimeType': mediaType,
        'blob': base64Encode([2, 3]),
      },
    },
  ],
};

void main() {
  test(
    'active bundled MCP editor participates in shared resource discovery',
    () async {
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final plugin = OpenMuseMcpFilePlugin(
        entry: McpMarketEntry(
          name: 'io.openmuse/genoffice-pptx',
          title: 'Slides',
          description: '',
          version: '1.0.0',
          packageUri: Uri(),
          sha256: 'a' * 64,
        ),
        runtime: McpBundleRuntime(
          bundleDirectory: Directory.systemTemp,
          nodeExecutable: 'node',
        ),
        toolName: 'inspect_pptx',
        mediaType: mime,
        extension: 'pptx',
      );
      registry.install(plugin);
      final hub = ResourceCenterHub(scope: 'qa');
      final controller = ResourceCenterController(hub: hub, registry: registry);
      addTearDown(controller.dispose);
      addTearDown(registry.dispose);
      expect(await controller.handlers(record()), isEmpty);
      await registry.activate(plugin.descriptor.id);
      final handlers = await controller.handlers(record());
      expect(handlers.single.plugin, same(plugin));
      expect(handlers.single.spec.editable, isTrue);
    },
  );

  test(
    'MCP save commits selected resource version and ignores exported path',
    () async {
      var calls = 0;
      final editor = session(
        commit: (base, bytes, key) async {
          calls++;
          expect(base.revision, 'v1');
          expect(bytes, [2, 3]);
          expect(key, startsWith('edit-'));
          return record(revision: 'v2', bytes: bytes);
        },
      );
      addTearDown(editor.dispose);
      await commitMcpResourceDownload(editor, download(), mime);
      expect(calls, 1);
      expect(editor.record.version.revision, 'v2');
      expect(editor.bytes, [2, 3]);
      expect(editor.dirty, isFalse);
    },
  );

  test(
    'read-only resource and different export type cannot overwrite source',
    () async {
      var calls = 0;
      Future<ResourceRecord> commit(
        ResourceVersion _,
        List<int> _,
        String _,
      ) async {
        calls++;
        return record();
      }

      final readOnly = session(editable: false, commit: commit);
      final editable = session(commit: commit);
      addTearDown(readOnly.dispose);
      addTearDown(editable.dispose);
      await expectLater(
        commitMcpResourceDownload(readOnly, download(), mime),
        throwsA(isA<ResourceFailure>()),
      );
      await expectLater(
        commitMcpResourceDownload(
          editable,
          download(mediaType: 'application/pdf'),
          mime,
        ),
        throwsFormatException,
      );
      expect(calls, 0);
      expect(editable.bytes, [1]);
    },
  );

  test(
    'failed versioned save preserves old version and unsaved-close warning',
    () async {
      final editor = session(
        commit: (_, _, _) async =>
            throw const ResourceFailure('REVISION_CONFLICT'),
      );
      addTearDown(editor.dispose);
      await expectLater(
        commitMcpResourceDownload(editor, download(), mime),
        throwsA(isA<ResourceFailure>()),
      );
      expect(editor.record.version.revision, 'v1');
      expect(editor.bytes, [1]);
      expect(editor.dirty, isTrue);
      expect(await editor.prepareToClose(), isFalse);
    },
  );
}
