import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'resource_close_barrier_test.dart' as fixture;

ResourceEditorSession session(String id) {
  final base = fixture.session();
  final identity = ResourceIdentity(
    scope: 'test',
    source: 'local',
    workspace: 'w',
    mount: 'm',
    resource: id,
  );
  final record = ResourceRecord.fromJson({
    ...base.record.toJson(),
    'identity': identity.toJson(),
    'descriptor': {...base.record.descriptor.toJson(), 'resourceRef': id},
  });
  base.dispose();
  return ResourceEditorSession(
    opened: OpenedResource(
      reference: ResourceReference(identity: identity),
      content: ResourceContent(record, []),
    ),
    allowEditing: true,
    commitContent: (_, _, _) async => record,
    onOpened: (_) {},
    onChanged: (_) {},
  );
}

class Owner {
  Owner(this.sessions, this.events, {bool privateRecovery = false}) {
    participant = ResourceCloseParticipant(
      supportsPrivateRecovery: privateRecovery,
      holdCheckpoint: () {
        holds++;
        events.add('pause:$id');
        return Hold(() {
          holds--;
          events.add('resume:$id');
        });
      },
      prepare: (closing) async {
        while (frozen) {
          await available!.future;
        }
        await beforePrepare?.call();
        prepares++;
        frozen = true;
        available = Completer<void>();
        events.add('prepare:$id');
        return current = Ticket(this, closing);
      },
    );
    for (final s in sessions) {
      s.closeParticipant = participant;
      s.enterEditing();
      s.markDirty(true);
    }
  }
  final List<ResourceEditorSession> sessions;
  final List<String> events;
  String get id => sessions.first.record.identity.resource;
  late final ResourceCloseParticipant participant;
  Ticket? current;
  Completer<void>? available, checkpointGate;
  Future<void> Function()? beforeCheckpoint, beforePrepare;
  int holds = 0;
  bool frozen = false, disposed = false, reject = false;
  int prepares = 0, checkpoints = 0, commits = 0, cancels = 0;
}

class Ticket implements PreparedPrivateRecoveryClose {
  Ticket(this.owner, this.closing);
  final Owner owner;
  final Set<ResourceEditorSession> closing;
  bool finished = false;
  @override
  bool get isActive =>
      !finished && owner.frozen && identical(owner.current, this);
  @override
  Future<void> checkpoint() async {
    owner.events.add('checkpoint:${owner.id}');
    owner.checkpoints++;
    await owner.beforeCheckpoint?.call();
    await owner.checkpointGate?.future;
    if (owner.reject) throw StateError('Original resource checkpoint rejected');
    for (final s in owner.sessions) {
      s.markDirty(false);
    }
  }

  void finish() {
    finished = true;
    owner.frozen = false;
    owner.available!.complete();
  }

  @override
  void cancel() {
    if (!isActive) return;
    owner.events.add('cancel:${owner.id}');
    owner.cancels++;
    finish();
  }

  @override
  Future<void> commitPrivateRecovery() async {
    owner.events.add('private:${owner.id}');
    await commit();
  }

  @override
  Future<void> commit() async {
    expect(isActive, isTrue);
    owner.events.add('commit:${owner.id}');
    owner.commits++;
    owner.disposed = owner.sessions.every(closing.contains);
    finish();
  }
}

class Hold implements ResourceCheckpointCloseHold {
  Hold(this.release);
  final void Function() release;
  bool released = false;
  @override
  Future<void> drain() async {}
  @override
  void resume() {
    if (released) return;
    released = true;
    release();
  }
}

void main() {
  test(
    'placement restoration visits only active nonfinalized presentations and never invokes save or release ports',
    () async {
      final root = session('a'), child = session('b');
      var restored = 0;
      root.onPlacementRestored = () async {
        restored++;
      };
      child.onPlacementRestored = () async {
        restored++;
      };
      root.flushPendingChanges = () async =>
          throw StateError('Placement must not save');
      child.releaseForClose = () async =>
          throw StateError('Placement must not release');
      root.addDependentSession(child);
      await root.restorePresentationAfterPlacement();
      expect(restored, 2);
      child.discardChangesForClose();
      await root.restorePresentationAfterPlacement();
      expect(restored, 3);
      root.dispose();
      child.dispose();
      await root.restorePresentationAfterPlacement();
      expect(restored, 3);
    },
  );
  test(
    'private recovery freezes the whole tree, skips resource checkpoints and preserves unsaved resource versions',
    () async {
      final root = session('a'), child = session('b'), events = <String>[];
      final a = Owner([root], events, privateRecovery: true),
          b = Owner([child], events, privateRecovery: true);
      root.addDependentSession(child);
      final version = root.record.version;
      a.beforePrepare = () async {
        expect(b.holds, 1);
      };
      expect(root.canPreserveForClose, isTrue);
      expect(await root.prepareToClose(privateRecoveryOnly: true), isTrue);
      expect(a.checkpoints + b.checkpoints, 0);
      expect(await root.finalizeToClose(), isTrue);
      expect(root.closeFinalized && child.closeFinalized, isTrue);
      expect(root.dirty && child.dirty, isTrue);
      expect(root.record.version, version);
      expect(a.holds + b.holds, 0);
      expect(events, containsAll(['private:a', 'private:b']));
      root.dispose();
      child.dispose();
    },
  );
  test(
    'a private preparation failure restores every owner and autosave hold',
    () async {
      final root = session('a'), child = session('b'), events = <String>[];
      final a = Owner([root], events, privateRecovery: true),
          b = Owner([child], events, privateRecovery: true);
      root.addDependentSession(child);
      b.beforePrepare = () async {
        throw StateError('Private storage unavailable');
      };
      expect(await root.prepareToClose(privateRecoveryOnly: true), isFalse);
      expect(a.holds + b.holds, 0);
      expect(a.frozen || b.frozen, isFalse);
      expect(a.checkpoints + b.checkpoints + a.commits + b.commits, 0);
      b.beforePrepare = null;
      expect(await root.prepareToClose(privateRecoveryOnly: true), isTrue);
      root.cancelClosePreparation();
      expect(a.holds + b.holds, 0);
      expect(a.frozen || b.frozen, isFalse);
      root.dispose();
      child.dispose();
    },
  );
  test(
    'mixed unsupported descendants cannot silently fall back to resource saves for private close',
    () async {
      final root = session('a'), child = session('b'), events = <String>[];
      final a = Owner([root], events, privateRecovery: true),
          b = Owner([child], events);
      root.addDependentSession(child);
      expect(root.canPreserveForClose, isFalse);
      expect(await root.prepareToClose(privateRecoveryOnly: true), isFalse);
      expect(a.prepares + b.prepares + a.checkpoints + b.checkpoints, 0);
      expect(a.holds + b.holds, 0);
      root.dispose();
      child.dispose();
    },
  );
  test(
    'a readiness invalidation cancels every held owner before returning a failed finalization',
    () async {
      final root = session('a'), child = session('b'), events = <String>[];
      final a = Owner([root], events), b = Owner([child], events);
      root.addDependentSession(child);
      expect(await root.prepareToClose(), isTrue);
      child.markDirty(true);
      expect(root.closePrepared, isFalse);
      expect(await root.finalizeToClose(), isFalse);
      expect(a.frozen || b.frozen, isFalse);
      expect(a.commits + b.commits, 0);
      expect(await root.prepareToClose(), isTrue);
      expect(await root.finalizeToClose(), isTrue);
      root.dispose();
      child.dispose();
    },
  );
  test(
    'all owners freeze before any checkpoint; two child surfaces prepare and finalize only one owner',
    () async {
      final root = session('a'),
          b1 = session('b'),
          b2 = session('b'),
          events = <String>[];
      final a = Owner([root], events), b = Owner([b1, b2], events);
      root.addDependentSession(b1);
      root.addDependentSession(b2);
      a.beforeCheckpoint = () async {
        expect(a.frozen && b.frozen, isTrue);
      };
      expect(await root.prepareToClose(), isTrue);
      expect(
        (a.prepares, b.prepares, a.checkpoints, b.checkpoints),
        (1, 1, 1, 1),
      );
      expect(events.take(4), ['pause:a', 'pause:b', 'prepare:a', 'prepare:b']);
      expect(a.disposed || b.disposed, isFalse);
      final finishing = root.finalizeToClose(),
          duplicate = root.finalizeToClose();
      expect(identical(finishing, duplicate), isTrue);
      expect(await finishing, isTrue);
      expect(await duplicate, isTrue);
      expect(a.disposed && b.disposed, isTrue);
      expect((a.commits, b.commits), (1, 1));
      root.dispose();
      b1.dispose();
      b2.dispose();
    },
  );
  test(
    'a later child checkpoint failure cancels every frozen owner and allows a complete same-session retry',
    () async {
      final root = session('a'), child = session('b'), events = <String>[];
      final a = Owner([root], events),
          b = Owner([child], events)..reject = true;
      root.addDependentSession(child);
      expect(await root.prepareToClose(), isFalse);
      expect(a.frozen || b.frozen || a.disposed || b.disposed, isFalse);
      expect(
        (a.checkpoints, b.checkpoints, a.commits, b.commits),
        (1, 1, 0, 0),
      );
      b.reject = false;
      expect(await root.prepareToClose(), isTrue);
      expect(await root.finalizeToClose(), isTrue);
      expect((a.prepares, b.prepares), (2, 2));
      root.dispose();
      child.dispose();
    },
  );
  test(
    'a child that attaches during a checkpoint cancels the stale tree and is included on retry',
    () async {
      final root = session('a'),
          child = session('b'),
          late = session('c'),
          events = <String>[];
      final a = Owner([root], events),
          b = Owner([child], events),
          c = Owner([late], events);
      root.addDependentSession(child);
      a.checkpointGate = Completer<void>();
      final preparing = root.prepareToClose();
      for (var i = 0; i < 30 && a.checkpoints == 0; i++) {
        await Future<void>.delayed(Duration.zero);
      }
      root.addDependentSession(late);
      a.checkpointGate!.complete();
      expect(await preparing, isFalse);
      expect(c.prepares, 0);
      expect(a.frozen || b.frozen, isFalse);
      a.checkpointGate = null;
      expect(await root.prepareToClose(), isTrue);
      expect(c.prepares, 1);
      expect(await root.finalizeToClose(), isTrue);
      root.dispose();
      child.dispose();
      late.dispose();
    },
  );
  test(
    'overlapping trees use the same owner order and cancellation releases a waiting close',
    () async {
      final first = session('root1'),
          second = session('root2'),
          aSession = session('a'),
          bSession = session('b'),
          events = <String>[];
      final a = Owner([aSession], events), b = Owner([bSession], events);
      first.addDependentSession(aSession);
      first.addDependentSession(bSession);
      second.addDependentSession(bSession);
      second.addDependentSession(aSession);
      expect(await first.prepareToClose(), isTrue);
      var completed = false;
      final waiting = second.prepareToClose().then((value) {
        completed = true;
        return value;
      });
      await Future<void>.delayed(Duration.zero);
      expect(completed, isFalse);
      first.cancelClosePreparation();
      expect(await waiting, isTrue);
      expect(await second.finalizeToClose(), isTrue);
      expect((a.commits, b.commits), (1, 1));
      first.dispose();
      second.dispose();
      aSession.dispose();
      bSession.dispose();
    },
  );
  test(
    'editable descendants without a preparation port cannot be treated as a frozen close tree',
    () async {
      final root = session('a'), child = session('b'), events = <String>[];
      final a = Owner([root], events);
      child.enterEditing();
      root.addDependentSession(child);
      expect(await root.prepareToClose(), isFalse);
      expect(a.prepares, 0);
      expect(a.disposed, isFalse);
      root.dispose();
      child.dispose();
    },
  );
}
