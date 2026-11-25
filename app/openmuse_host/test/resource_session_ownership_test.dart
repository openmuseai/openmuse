import 'dart:async';
import 'dart:io';
import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

ResourceEditorSession session(
  List<int> bytes, {
  bool historical = false,
  bool allowEditing = true,
  ResourceSessionLifetime? lifetime,
  String account = 'test',
  String revision = 'v1',
  Future<void> Function()? beforeCommit,
}) {
  final record = ResourceRecord(
    identity: ResourceIdentity(
      scope: account,
      source: 'local',
      workspace: 'w',
      mount: 'm',
      resource: 'r',
    ),
    version: ResourceVersion(revision: revision, digest: resourceDigest(bytes)),
    descriptor: MuseResourceDescriptorV1(
      resourceRef: 'r',
      revision: revision,
      displayName: 'Resource',
      mediaType: 'text/plain',
      format: const MuseResourceFormat(
        formatId: 'text',
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
  return ResourceEditorSession(
    lifetime: lifetime,
    opened: OpenedResource(
      reference: ResourceReference(identity: record.identity),
      content: ResourceContent(record, bytes),
      fromSnapshot: historical,
    ),
    allowEditing: allowEditing,
    commitContent: (base, next, _) async {
      await beforeCommit?.call();
      return ResourceRecord.fromJson({
        ...record.toJson(),
        'descriptor': {
          ...record.descriptor.toJson(),
          'revision': '${base.revision}-next',
        },
        'version': ResourceVersion(
          revision: '${base.revision}-next',
          digest: resourceDigest(next),
        ).toJson(),
      });
    },
    onOpened: (_) {},
    onChanged: (_) {},
  );
}

WorkspaceResourceSession binding(ResourceEditorSession s) =>
    WorkspaceResourceSession(
      session: s,
      builder: (_) => const SizedBox.shrink(),
      confirmClose: () async => false,
    );
OpenMuseResource resource(ResourceEditorSession s) => OpenMuseResource(
  uri: ResourceReference(
    identity: s.record.identity,
    version: s.opened.fromSnapshot ? s.record.version : null,
  ).toUri(),
  displayName: s.record.name,
);

class PrivateCloseTicket implements PreparedPrivateRecoveryClose {
  bool active = true, committed = false;
  @override
  bool get isActive => active;
  @override
  Future<void> checkpoint() async => throw StateError('Resource save rejected');
  @override
  Future<void> commit() async => throw StateError('Normal commit must not run');
  @override
  Future<void> commitPrivateRecovery() async {
    committed = true;
    active = false;
  }

  @override
  void cancel() {
    active = false;
  }
}

void main() {
  test(
    'same-resource readonly and write presentations keep distinct identities and deduplicate only matching policy',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'resource-policy-route-',
      );
      final c = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final life = ResourceSessionLifetime();
      final read = session([1], allowEditing: false, lifetime: life),
          write = session([1], lifetime: life),
          readAgain = session([1], allowEditing: false, lifetime: life),
          writeAgain = session([1], lifetime: life);
      WorkspaceResourceSession shared(ResourceEditorSession s) =>
          WorkspaceResourceSession(
            session: s,
            builder: (_) => const SizedBox.shrink(),
            confirmClose: () async => false,
            supportsSharedPresentations: true,
          );
      unawaited(c.openResourceSession(resource(read), shared(read)));
      final readerTab = c.activeTab!;
      unawaited(c.openResourceSession(resource(write), shared(write)));
      final writerTab = c.activeTab!;
      expect(c.tabs, hasLength(2));
      expect(readerTab.id, isNot(writerTab.id));
      unawaited(c.openResourceSession(resource(readAgain), shared(readAgain)));
      expect(c.activeTab, same(readerTab));
      expect(c.tabs, hasLength(2));
      unawaited(
        c.openResourceSession(resource(writeAgain), shared(writeAgain)),
      );
      expect(c.activeTab, same(writerTab));
      expect(c.tabs, hasLength(2));
      c.dispose();
      for (final s in [read, write, readAgain, writeAgain]) {
        s.dispose();
      }
      life.dispose();
      await root.delete(recursive: true);
    },
  );
  test(
    'a fresh account lifetime does not focus an ended same-resource presentation',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'resource-life-route-',
      );
      final c = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final previous = ResourceSessionLifetime(),
          current = ResourceSessionLifetime();
      final old = session([1], lifetime: previous),
          next = session([1], lifetime: current);
      unawaited(c.openResourceSession(resource(old), binding(old)));
      final endedTab = c.activeTab!;
      previous.invalidate();
      unawaited(c.openResourceSession(resource(next), binding(next)));
      expect(c.tabs, hasLength(2));
      expect(c.activeTab, isNot(same(endedTab)));
      expect(c.activeTab!.id, isNot(endedTab.id));
      c.dispose();
      old.dispose();
      next.dispose();
      previous.dispose();
      current.dispose();
      await root.delete(recursive: true);
    },
  );
  test(
    'a handler without shared presentations refuses a conflicting policy instead of focusing a more permissive view',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'resource-policy-conflict-',
      );
      final c = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final writer = session([1]), reader = session([1], allowEditing: false);
      unawaited(c.openResourceSession(resource(writer), binding(writer)));
      final original = c.activeTab!;
      expect(
        () => c.openResourceSession(resource(reader), binding(reader)),
        throwsA(isA<ResourceFailure>()),
      );
      expect(c.activeTab, same(original));
      expect(c.tabs, hasLength(1));
      c.dispose();
      writer.dispose();
      reader.dispose();
      await root.delete(recursive: true);
    },
  );

  test(
    'Host private recovery failure keeps the tab, retries the typed choice, and removes a finalized dirty session without a second prompt',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'host-private-recovery-',
      );
      final c = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final source = session([1]);
      source.enterEditing();
      source.markDirty(true);
      var rejectPrivate = false, prompts = 0, feedback = 0;
      PrivateCloseTicket? ticket;
      source.closeParticipant = ResourceCloseParticipant(
        supportsPrivateRecovery: true,
        prepare: (_) async {
          if (rejectPrivate) throw StateError('Private IO rejected');
          return ticket = PrivateCloseTicket();
        },
      );
      // A runtime participant is sufficient even without a legacy widget callback.
      source.flushPendingChanges = () async {};
      final b = WorkspaceResourceSession(
        session: source,
        builder: (_) => const SizedBox.shrink(),
        confirmClose: () async =>
            throw StateError('Legacy discard must not run'),
        chooseClose: () async {
          prompts++;
          rejectPrivate = prompts == 1;
          return ResourceCloseDecision.preservePrivate;
        },
        onRecoveryFailed: () {
          feedback++;
        },
      );
      final closed = c.openResourceSession(resource(source), b);
      final tab = c.activeTab!;
      expect(await c.requestCloseTab(tab), isFalse);
      expect(c.activeTab, same(tab));
      expect(feedback, 1);
      expect(prompts, 1);
      expect(source.closeFinalized, isFalse);
      expect(await c.requestCloseTab(tab), isTrue);
      await closed;
      expect(source.dirty, isTrue);
      expect(ticket!.committed, isTrue);
      expect(prompts, 2);
      expect(c.tabs, isEmpty);
      c.dispose();
      source.dispose();
      await root.delete(recursive: true);
    },
  );
  test(
    'batch and editor-group closing use the retryable final owner barrier',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'resource-pane-close-',
      );
      final c = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final secondary = c.createEditorGroup();
      c.focusEditorGroup(secondary.id);
      final s = session([1]);
      var released = 0;
      var reject = true;
      s.flushPendingChanges = () async {};
      s.releaseForClose = () async {
        if (reject) throw const ResourceFailure('FINAL_CHECKPOINT_FAILED');
        released++;
      };
      final b = binding(s);
      final closed = c.openResourceSession(resource(s), b);
      final tab = c.activeTab!;
      c.removeEditorGroup(secondary.id);
      expect(c.tabsFor(secondary.id), [tab]);
      expect(await c.closeEditorGroup(secondary.id), isFalse);
      expect(c.tabsFor(secondary.id), [tab]);
      expect(released, 0);
      final other = OpenMuseResource(
        uri: Uri.file('${root.path}/retained.txt'),
        displayName: 'Retained',
      );
      c.openResource(other);
      final retained = c.activeTab!;
      c.closeOtherTabs(retained);
      for (var i = 0; i < 50 && b.confirming; i++) {
        await Future<void>.delayed(Duration.zero);
      }
      expect(c.tabsFor(secondary.id), contains(tab));
      reject = false;
      c.closeOtherTabs(retained);
      await closed;
      expect(released, 1);
      expect(c.tabsFor(secondary.id), [retained]);
      expect(await c.closeEditorGroup(secondary.id), isTrue);
      c.dispose();
      s.dispose();
      await root.delete(recursive: true);
    },
  );
  test(
    'final owner failure retains its mounted tab and retries before removal',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'resource-final-owner-',
      );
      final c = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final source = session([1]);
      var attempts = 0, confirmations = 0;
      source.flushPendingChanges = () async => source.markDirty(false);
      source.releaseForClose = () async {
        attempts++;
        expect(c.tabs, hasLength(1));
        if (attempts == 1) {
          throw const ResourceFailure('FINAL_CHECKPOINT_FAILED');
        }
      };
      final b = WorkspaceResourceSession(
        session: source,
        builder: (_) => const SizedBox.shrink(),
        confirmClose: () async {
          confirmations++;
          return false;
        },
      );
      final closed = c.openResourceSession(resource(source), b);
      final tab = c.activeTab!;
      c.closeTab(tab);
      for (var i = 0; i < 50 && b.confirming; i++) {
        await Future<void>.delayed(Duration.zero);
      }
      expect(attempts, 1);
      expect(confirmations, 1);
      expect(c.activeTab, same(tab));
      expect(b.closed.isCompleted, isFalse);
      expect(source.closePrepared, isFalse);
      source.markDirty(true);
      c.closeTab(tab);
      await closed;
      expect(attempts, 2);
      expect(source.closeFinalized, isTrue);
      expect(c.tabs, isEmpty);
      c.dispose();
      source.dispose();
      await root.delete(recursive: true);
    },
  );
  test(
    'shared presentations require both handlers to opt in and deduplicate within each group',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'resource-multi-owner-',
      );
      final c = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final sessions = List.generate(4, (_) => session([1]));
      WorkspaceResourceSession shared(ResourceEditorSession s) =>
          WorkspaceResourceSession(
            session: s,
            builder: (_) => const SizedBox.shrink(),
            confirmClose: () async => false,
            supportsSharedPresentations: true,
          );
      final firstClosed = c.openResourceSession(
        resource(sessions[0]),
        shared(sessions[0]),
      );
      final first = c.activeTab!;
      final secondGroup = c.createEditorGroup();
      c.focusEditorGroup(secondGroup.id);
      final secondClosed = c.openResourceSession(
        resource(sessions[1]),
        shared(sessions[1]),
      );
      final second = c.activeTab!;
      expect(second, isNot(same(first)));
      expect(c.tabsFor(secondGroup.id), [second]);
      expect(c.tabsFor(LocalWorkspaceController.primaryEditorGroupId), [first]);
      expect(
        c.openResourceSession(resource(sessions[2]), shared(sessions[2])),
        same(secondClosed),
      );
      expect(c.tabsFor(secondGroup.id), [second]);
      final thirdGroup = c.createEditorGroup();
      c.focusEditorGroup(thirdGroup.id);
      expect(
        c.openResourceSession(resource(sessions[3]), binding(sessions[3])),
        same(firstClosed),
      );
      expect(
        c.focusedEditorGroupId,
        LocalWorkspaceController.primaryEditorGroupId,
      );
      expect(c.tabsFor(thirdGroup.id), isEmpty);
      c.closeTab(first);
      expect(c.tabsFor(secondGroup.id), [second]);
      c.focusEditorGroup(secondGroup.id);
      c.closeTab(second);
      await Future.wait([firstClosed, secondClosed]);
      c.dispose();
      for (final s in sessions) {
        s.dispose();
      }
      await root.delete(recursive: true);
    },
  );
  test(
    'cross-group opens focus the same writer and preserve its failed-save barrier',
    () async {
      final root = await Directory.systemTemp.createTemp('resource-owner-');
      final controller = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final first = session([1]), duplicate = session([1]);
      final owner = binding(first);
      final closed = controller.openResourceSession(resource(first), owner);
      final tab = controller.activeTab!;
      final secondary = controller.createEditorGroup();
      controller.focusEditorGroup(secondary.id);
      final reused = controller.openResourceSession(
        resource(duplicate),
        binding(duplicate),
      );
      expect(reused, same(closed));
      expect(
        controller.focusedEditorGroupId,
        LocalWorkspaceController.primaryEditorGroupId,
      );
      expect(controller.activeTab, same(tab));
      expect(controller.tabsFor(secondary.id), isEmpty);
      final gate = Completer<void>();
      first.markDirty(true);
      first.flushPendingChanges = () async {
        await gate.future;
        throw const ResourceFailure('REVISION_CONFLICT');
      };
      controller.closeTab(tab);
      gate.complete();
      for (var i = 0; i < 20 && owner.confirming; i++) {
        await Future<void>.delayed(Duration.zero);
      }
      expect(first.dirty, isTrue);
      expect(controller.activeTab, same(tab));
      expect(owner.closed.isCompleted, isFalse);
      first.flushPendingChanges = () async => first.markDirty(false);
      controller.closeTab(tab);
      await closed;
      expect(owner.closed.isCompleted, isTrue);
      controller.dispose();
      first.dispose();
      duplicate.dispose();
      await root.delete(recursive: true);
    },
  );
  test(
    'historical revisions and another account keep distinct resource tabs',
    () async {
      final root = await Directory.systemTemp.createTemp('resource-owner-');
      final controller = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      final live = session([1]),
          pin = session([1], historical: true),
          pin2 = session([1], historical: true, revision: 'v2'),
          account = session([1], account: 'other');
      final futures = <Future<void>>[];
      for (final s in [live, pin, pin2, account]) {
        futures.add(controller.openResourceSession(resource(s), binding(s)));
      }
      expect(controller.tabs, hasLength(4));
      for (final tab in controller.tabs.toList()) {
        controller.closeTab(tab);
      }
      await Future.wait(futures);
      controller.dispose();
      for (final s in [live, pin, pin2, account]) {
        s.dispose();
      }
      await root.delete(recursive: true);
    },
  );
}
