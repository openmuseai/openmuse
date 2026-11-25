import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

ResourceEditorSession session() {
  final record = ResourceRecord(
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
    opened: OpenedResource(
      reference: ResourceReference(identity: record.identity),
      content: ResourceContent(record, []),
    ),
    allowEditing: true,
    commitContent: (_, _, _) async => record,
    onOpened: (_) {},
    onChanged: (_) {},
  );
}

void main() {
  test(
    'overlapping checkpoint holds preserve private recovery on final resume and reset that intent for the next pause',
    () async {
      final barrier = ResourceCheckpointBarrier(), inFlight = Completer<void>();
      var ordinary = 0, privately = 0;
      final a = barrier.pause(
        inFlight.future,
        () => ordinary++,
        onPrivateResume: () => privately++,
      );
      final b = barrier.pause(
        null,
        () => ordinary++,
        onPrivateResume: () => privately++,
      );
      var drained = false;
      final drain = a.drain().then((_) => drained = true);
      await Future<void>.delayed(Duration.zero);
      expect(drained, isFalse);
      inFlight.complete();
      await drain;
      (a as ResourcePrivateRecoveryCheckpointHold).resumeAfterPrivateRecovery();
      a.resume();
      expect(barrier.isPaused, isTrue);
      expect(privately, 0);
      expect(ordinary, 0);
      b.resume();
      await barrier.resumed;
      expect(privately, 1);
      expect(ordinary, 0);
      expect(barrier.isPaused, isFalse);
      final next = barrier.pause(
        null,
        () => ordinary++,
        onPrivateResume: () => privately++,
      );
      next.resume();
      expect(ordinary, 1);
      expect(privately, 1);
    },
  );

  test(
    'final release follows dependent save barriers and remains retryable',
    () async {
      final editor = session();
      var calls = 0;
      editor.releaseForClose = () async {
        calls++;
        if (calls == 1) throw const ResourceFailure('FINAL_CHECKPOINT_FAILED');
      };
      expect(await editor.finalizeToClose(), isFalse);
      expect(calls, 0);
      expect(await editor.prepareToClose(), isTrue);
      expect(await editor.finalizeToClose(), isFalse);
      expect(editor.closePrepared, isFalse);
      expect(editor.closeFinalized, isFalse);
      expect(await editor.prepareToClose(), isTrue);
      expect(await editor.finalizeToClose(), isTrue);
      expect(await editor.finalizeToClose(), isTrue);
      expect(calls, 2);
      editor.dispose();
    },
  );
  test(
    'close waits for one shared save barrier; a later edit invalidates it',
    () async {
      final editor = session();
      addTearDown(editor.dispose);
      final pending = Completer<void>();
      var calls = 0;
      editor.markDirty(true);
      editor.flushPendingChanges = () async {
        calls++;
        await pending.future;
        editor.markDirty(false);
      };
      final first = editor.prepareToClose(), second = editor.prepareToClose();
      expect(identical(first, second), isTrue);
      expect(editor.closePrepared, isFalse);
      pending.complete();
      expect(await first, isTrue);
      expect(await second, isTrue);
      expect(calls, 1);
      expect(editor.closePrepared, isTrue);
      editor.markDirty(true);
      expect(editor.closePrepared, isFalse);
    },
  );

  test(
    'failed autosave retains dirty state and supports retry or explicit discard',
    () async {
      final editor = session();
      addTearDown(editor.dispose);
      editor.markDirty(true);
      editor.flushPendingChanges = () async =>
          throw const ResourceFailure('REVISION_CONFLICT');
      expect(await editor.prepareToClose(), isFalse);
      expect(editor.dirty, isTrue);
      expect(editor.closePrepared, isFalse);
      editor.flushPendingChanges = () async => editor.markDirty(false);
      expect(await editor.prepareToClose(), isTrue);
      editor.markDirty(true);
      editor.discardChangesForClose();
      expect(editor.dirty, isFalse);
      expect(editor.closePrepared, isTrue);
    },
  );
}
