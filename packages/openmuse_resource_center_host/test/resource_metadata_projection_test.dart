import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_resource_notes/openmuse_resource_notes.dart';
import 'device_checkpoint_test.dart' as fixture;

void main() {
  test(
    'one verified name reaches live sessions and recents without content commit or snapshot mutation',
    () async {
      final source = fixture.CollaborativeSource(),
          hub = ResourceCenterHub(scope: 'account');
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final plugin = OpenMuseResourceNotesPlugin();
      registry.install(plugin);
      await registry.activate(plugin.descriptor.id);
      hub.register(source);
      final center = ResourceCenterController(hub: hub, registry: registry);
      final choice = ResourceHandlerChoice(
        plugin,
        plugin,
        plugin.resourceHandlers.first,
      );
      ResourceEditorSession make(
        List<int> bytes, {
        bool pin = false,
        bool readOnly = false,
      }) {
        final record = source.record(bytes);
        return center.createEditorSession(
          OpenedResource(
            reference: record.reference(pin: pin),
            content: ResourceContent(record, bytes),
            fromSnapshot: pin,
          ),
          choice,
          readOnly: readOnly,
        );
      }

      final a = make([1]),
          b = make([2], readOnly: true),
          pin = make([3], pin: true);
      try {
        a.enterEditing();
        a.markDirty(true);
        await hub.recordOpened(b.record, 'notes');
        final before = [a.record.version, b.record.version, pin.record.version];
        await center.observeResourceDisplayMetadata(
          a.record.identity,
          name: '原目录名称😀',
          location: ['原目录名称😀'],
        );
        expect(a.record.name, '原目录名称😀');
        expect(b.record.name, '原目录名称😀');
        expect(pin.record.name, 'Doc.md');
        expect([
          a.record.version,
          b.record.version,
          pin.record.version,
        ], before);
        expect(a.bytes, [1]);
        expect(b.bytes, [2]);
        expect(pin.bytes, [3]);
        expect(a.dirty, isTrue);
        expect(a.editing, isTrue);
        expect(b.allowEditing, isFalse);
        expect(hub.recent.single.record.name, '原目录名称😀');
        expect(hub.recent.single.record.version, before[1]);
        expect(source.bases, isEmpty);
        expect(source.remoteCommits, 0);
        final retention = a.retain();
        a.dispose();
        await center.observeResourceDisplayMetadata(
          b.record.identity,
          name: 'Retained owner label',
        );
        expect(a.record.name, 'Retained owner label');
        retention.release();
        await a.released;
        await center.observeResourceDisplayMetadata(
          b.record.identity,
          name: 'Remaining presentation',
        );
        expect(b.record.name, 'Remaining presentation');
        expect(a.record.name, 'Retained owner label');
        center.editorLifetime.invalidate();
        await expectLater(
          center.observeResourceDisplayMetadata(
            b.record.identity,
            name: 'Late account',
          ),
          throwsA(isA<ResourceFailure>()),
        );
        expect(b.record.name, 'Remaining presentation');
      } finally {
        a.dispose();
        b.dispose();
        pin.dispose();
        center.dispose();
        await registry.deactivate(plugin.descriptor.id);
        registry.dispose();
        await hub.dispose();
      }
    },
  );
}
