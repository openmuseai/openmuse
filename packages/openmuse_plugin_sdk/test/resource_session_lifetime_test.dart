import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'resource_close_barrier_test.dart' as fixture;

void main() {
  test(
    'checkpoint completion distinguishes ended authority from provider failures',
    () async {
      for (final code in <String?>[
        null,
        'RESOURCE_SESSION_CLOSED',
        'DENIED',
        'REVISION_CONFLICT',
      ]) {
        for (final endAuthority in [false, true]) {
          final base = fixture.session(), lifetime = ResourceSessionLifetime();
          final gate = Completer<ResourceRecord>();
          var changed = 0;
          final s = ResourceEditorSession(
            opened: base.opened,
            lifetime: lifetime,
            allowEditing: true,
            commitContent: (_, _, _) => gate.future,
            onOpened: (_) {},
            onChanged: (_) => changed++,
          );
          s.enterEditing();
          s.markDirty(true);
          final pending = s.commitCheckpointIfActive([]);
          if (endAuthority) lifetime.invalidate();
          final completes =
              code == null ||
              (endAuthority && code == 'RESOURCE_SESSION_CLOSED');
          final checked = completes
              ? expectLater(pending, completion(!endAuthority))
              : expectLater(pending, throwsA(isA<ResourceFailure>()));
          if (code == null) {
            gate.complete(base.record);
          } else {
            gate.completeError(ResourceFailure(code));
          }
          await checked;
          expect(changed, code == null && !endAuthority ? 1 : 0);
          expect(s.dirty, code != null || endAuthority);
          s.dispose();
          base.dispose();
        }
      }
    },
  );
  test('an ended checkpoint never invokes its resource provider', () async {
    final base = fixture.session(), lifetime = ResourceSessionLifetime();
    var calls = 0;
    final s = ResourceEditorSession(
      opened: base.opened,
      lifetime: lifetime,
      allowEditing: true,
      commitContent: (_, _, _) async {
        calls++;
        return base.record;
      },
      onOpened: (_) {},
      onChanged: (_) {},
    );
    s.enterEditing();
    s.markDirty(true);
    lifetime.invalidate();
    expect(await s.commitCheckpointIfActive([]), isFalse);
    expect(calls, 0);
    expect(s.dirty, isTrue);
    s.dispose();
    base.dispose();
  });
  test(
    'owner retention outlives one presentation but never its account authority',
    () async {
      final base = fixture.session(), lifetime = ResourceSessionLifetime();
      var commits = 0;
      final s = ResourceEditorSession(
        opened: base.opened,
        lifetime: lifetime,
        allowEditing: true,
        commitContent: (a, b, c) async {
          commits++;
          return base.record;
        },
        onOpened: (_) {},
        onChanged: (_) {},
      );
      final hold = s.retain();
      s.dispose();
      s.dispose();
      expect(s.isActive, isTrue);
      s.enterEditing();
      await s.commit([]);
      expect(commits, 1);
      lifetime.invalidate();
      expect(s.isActive, isFalse);
      await expectLater(s.commit([]), throwsA(isA<ResourceFailure>()));
      expect(commits, 1);
      hold.release();
      hold.release();
      base.dispose();
    },
  );
  test(
    'authority ending during a provider commit does not publish a stale checkpoint',
    () async {
      final base = fixture.session(),
          lifetime = ResourceSessionLifetime(),
          gate = Completer<ResourceRecord>();
      var changed = 0;
      final s = ResourceEditorSession(
        opened: base.opened,
        lifetime: lifetime,
        allowEditing: true,
        commitContent: (_, _, _) => gate.future,
        onOpened: (_) {},
        onChanged: (_) => changed++,
      );
      s.enterEditing();
      final pending = s.commit([]);
      lifetime.invalidate();
      gate.complete(base.record);
      await expectLater(pending, throwsA(isA<ResourceFailure>()));
      expect(changed, 0);
      s.dispose();
      base.dispose();
    },
  );
  test(
    'child checkpoint failure blocks parent close and later child edits invalidate readiness',
    () async {
      final base = fixture.session();
      ResourceEditorSession child(String id) {
        final record = ResourceRecord.fromJson({
          ...base.record.toJson(),
          'identity': {...base.record.identity.toJson(), 'resource': id},
          'descriptor': {...base.record.descriptor.toJson(), 'resourceRef': id},
        });
        return ResourceEditorSession(
          opened: OpenedResource(
            reference: record.reference(),
            content: ResourceContent(record, []),
          ),
          allowEditing: true,
          commitContent: (_, _, _) async => record,
          onOpened: (_) {},
          onChanged: (_) {},
        );
      }

      final nested = child('child');
      final remove = base.addDependentSession(nested);
      nested.markDirty(true);
      nested.flushPendingChanges = () async =>
          throw const ResourceFailure('REVISION_CONFLICT');
      expect(await base.prepareToClose(), isFalse);
      expect(nested.dirty, isTrue);
      nested.flushPendingChanges = () async => nested.markDirty(false);
      expect(await base.prepareToClose(), isTrue);
      expect(base.closePrepared, isTrue);
      nested.markDirty(true);
      expect(base.closePrepared, isFalse);
      await expectLater(
        Future.sync(() => nested.addDependentSession(base)),
        throwsA(isA<ResourceFailure>()),
      );
      remove();
      remove();
      expect(await base.prepareToClose(), isTrue);
      nested.dispose();
      base.dispose();
    },
  );
}
