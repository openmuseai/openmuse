import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'resource_close_barrier_test.dart' as fixture;

void main() {
  test(
    'verified label observation preserves dirty content, reference and authority',
    () {
      final s = fixture.session();
      addTearDown(s.dispose);
      s.enterEditing();
      s.markDirty(true);
      final before = s.record, bytes = s.bytes, reference = s.opened.reference;
      final location = ['Original location', 'New title'];
      s.observeDisplayMetadata(
        identity: before.identity,
        name: 'New title',
        location: location,
      );
      location.clear();
      expect(s.record.name, 'New title');
      expect(s.record.location, ['Original location', 'New title']);
      expect(s.bytes, bytes);
      expect(s.opened.reference, same(reference));
      expect(s.record.version, before.version);
      expect(s.record.metadataRevision, before.metadataRevision);
      expect(
        s.record.descriptor.security.toJson(),
        before.descriptor.security.toJson(),
      );
      expect(s.record.descriptor.capabilities, before.descriptor.capabilities);
      expect(s.record.readOnly, before.readOnly);
      expect(s.dirty, isTrue);
      expect(s.editing, isTrue);
    },
  );
  test(
    'in-flight content checkpoint cannot regress an observed original name',
    () async {
      final base = fixture.session();
      addTearDown(base.dispose);
      final pending = Completer<ResourceRecord>();
      var commits = 0, changes = 0;
      final s = ResourceEditorSession(
        opened: base.opened,
        allowEditing: true,
        commitContent: (_, _, _) {
          commits++;
          return pending.future;
        },
        onOpened: (_) {},
        onChanged: (_) => changes++,
      );
      addTearDown(s.dispose);
      s.enterEditing();
      s.markDirty(true);
      final bytes = [1, 2, 3];
      final content = ResourceRecord.fromJson({
        ...base.record.toJson(),
        'descriptor': {
          ...base.record.descriptor.toJson(),
          'revision': 'next-content',
        },
        'version': {
          'revision': 'next-content',
          'digest': resourceDigest(bytes),
        },
      });
      final saving = s.commit(bytes);
      s.observeDisplayMetadata(
        identity: s.record.identity,
        name: 'Verified newer title',
      );
      expect(changes, 0);
      expect(s.dirty, isTrue);
      pending.complete(content);
      await saving;
      expect(s.record.name, 'Verified newer title');
      expect(s.bytes, bytes);
      expect(s.record.version.revision, 'next-content');
      expect(commits, 1);
      expect(changes, 1);
    },
  );
  test(
    'shared checkpoint retains presentation metadata and the one content owner',
    () {
      final owner = fixture.session(), peer = fixture.session();
      addTearDown(owner.dispose);
      addTearDown(peer.dispose);
      peer.observeDisplayMetadata(
        identity: peer.record.identity,
        name: 'Observed name',
      );
      peer.observeSharedCheckpoint(owner);
      expect(peer.record.name, 'Observed name');
      expect(peer.record.version, owner.record.version);
      expect(peer.bytes, owner.bytes);
      expect(peer.record.identity, owner.record.identity);
    },
  );
  test(
    'foreign identity, snapshot and retired account cannot receive display updates',
    () {
      final base = fixture.session();
      addTearDown(base.dispose);
      final wrong = ResourceIdentity.fromJson({
        ...base.record.identity.toJson(),
        'scope': 'other-account',
      });
      expect(
        () =>
            base.observeDisplayMetadata(identity: wrong, name: 'Wrong account'),
        throwsA(isA<ResourceFailure>()),
      );
      final pin = ResourceEditorSession(
        opened: OpenedResource(
          reference: base.record.reference(pin: true),
          content: base.opened.content,
          fromSnapshot: true,
        ),
        allowEditing: false,
        commitContent: (_, _, _) async => base.record,
        onOpened: (_) {},
        onChanged: (_) {},
      );
      addTearDown(pin.dispose);
      expect(
        () => pin.observeDisplayMetadata(
          identity: pin.record.identity,
          name: 'Wrong live title',
        ),
        throwsA(isA<ResourceFailure>()),
      );
      final lifetime = ResourceSessionLifetime();
      final s = ResourceEditorSession(
        opened: base.opened,
        allowEditing: false,
        lifetime: lifetime,
        commitContent: (_, _, _) async => base.record,
        onOpened: (_) {},
        onChanged: (_) {},
      );
      addTearDown(s.dispose);
      lifetime.invalidate();
      expect(
        () => s.observeDisplayMetadata(
          identity: s.record.identity,
          name: 'Late label',
        ),
        throwsA(isA<ResourceFailure>()),
      );
      expect(s.record.name, base.record.name);
      expect(pin.record.name, base.record.name);
    },
  );
  test(
    'final-release notification waits for the retained content owner',
    () async {
      final s = fixture.session(), hold = s.retain();
      var releases = 0;
      unawaited(s.released.then((_) => releases++));
      s.dispose();
      await Future<void>.delayed(Duration.zero);
      expect(releases, 0);
      hold.release();
      await s.released;
      await Future<void>.delayed(Duration.zero);
      expect(releases, 1);
      hold.release();
      s.dispose();
      expect(releases, 1);
    },
  );
}
