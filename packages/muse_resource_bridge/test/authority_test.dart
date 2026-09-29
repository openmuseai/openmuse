import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_bridge/muse_resource_bridge.dart';
import 'package:muse_resource_contract/muse_resource_contract.dart';

void main() {
  for (final kind in [
    MuseWorkspaceProviderKind.local,
    MuseWorkspaceProviderKind.cloud,
  ]) {
    group('$kind authority TCK', () {
      late int now;
      late MuseInMemoryWorkspaceAuthority provider;

      setUp(() {
        now = 1000;
        provider = MuseInMemoryWorkspaceAuthority(
          workspaceRef: 'workspace.primary',
          mountRef: 'mount.primary',
          providerKind: kind,
          clock: () => now,
        )..seed(
            resourceRef: 'resource.a',
            displayName: 'a.txt',
            mediaType: 'text/plain',
            bytes: [1, 2, 3],
          );
      });

      test('lists, describes and materializes without exposing a path',
          () async {
        provider.seed(
          resourceRef: 'resource.b',
          displayName: 'b.txt',
          mediaType: 'text/plain',
          bytes: [4],
        );
        final first = await provider.list(limit: 1);
        expect(first.items.single.resourceRef, 'resource.a');
        expect(first.nextCursor, '1');
        expect((await provider.list(cursor: first.nextCursor)).items,
            hasLength(1));

        final descriptor = await provider.describe('resource.a');
        expect(provider.mountRef, 'mount.primary');
        expect(descriptor.toJson().containsKey('path'), isFalse);
        final materialized = await provider.materialize(
          'resource.a',
          audience: 'adapter.viewer',
          accessMode: MuseAccessMode.read,
          generation: 7,
          ttlMs: 100,
        );
        expect(materialized.value.toJson().containsKey('path'), isFalse);
        expect(
          provider.readMaterialization(
            materialized.value.handleRef,
            audience: 'adapter.viewer',
            generation: 7,
          ),
          [1, 2, 3],
        );
        await expectLater(
          provider.materialize(
            '/private/a.txt',
            audience: 'adapter.viewer',
            accessMode: MuseAccessMode.read,
            generation: 7,
            ttlMs: 100,
          ),
          throwsA(
            isA<MuseAuthorityException>().having(
              (error) => error.code,
              'code',
              'DENIED',
            ),
          ),
        );
      });

      test('materialization enforces audience, generation and expiry',
          () async {
        final value = await provider.materialize(
          'resource.a',
          audience: 'adapter.viewer',
          accessMode: MuseAccessMode.read,
          generation: 7,
          ttlMs: 10,
        );
        expect(
          () => provider.readMaterialization(
            value.value.handleRef,
            audience: 'adapter.other',
            generation: 7,
          ),
          throwsA(
            isA<MuseAuthorityException>().having(
              (error) => error.code,
              'code',
              'DENIED',
            ),
          ),
        );
        expect(
          () => provider.readMaterialization(
            value.value.handleRef,
            audience: 'adapter.viewer',
            generation: 6,
          ),
          throwsA(
            isA<MuseAuthorityException>().having(
              (error) => error.code,
              'code',
              'STALE_GENERATION',
            ),
          ),
        );
        now = 1010;
        expect(
          () => provider.readMaterialization(
            value.value.handleRef,
            audience: 'adapter.viewer',
            generation: 7,
          ),
          throwsA(
            isA<MuseAuthorityException>().having(
              (error) => error.code,
              'code',
              'EXPIRED',
            ),
          ),
        );
      });

      test('draft commit is CAS, idempotent and publishes an event', () async {
        final before = await provider.describe('resource.a');
        final snapshot = await provider.materialize(
          'resource.a',
          audience: 'adapter.snapshot',
          accessMode: MuseAccessMode.read,
          generation: 2,
          ttlMs: 100,
        );
        final draft = await provider.createDraft(
          'resource.a',
          audience: 'worker.office',
          generation: 2,
          ttlMs: 100,
        );
        await provider.writeDraft(
          draft.draftRef,
          [9, 8, 7],
          audience: 'worker.office',
          generation: 2,
        );
        final event = provider.subscribe().first;
        final receipt = await provider.commit(
          commitRef: 'commit.1',
          resourceRef: 'resource.a',
          expectedRevision: before.revision,
          draftRef: draft.draftRef,
          audience: 'worker.office',
          generation: 2,
          idempotencyKey: 'idem.1',
        );
        expect(receipt.result, MuseAuthorityCommitResult.committed);
        expect((await event).afterRevision, receipt.newRevision);
        expect(
          provider.readMaterialization(
            snapshot.value.handleRef,
            audience: 'adapter.snapshot',
            generation: 2,
          ),
          [1, 2, 3],
          reason: 'a materialization is pinned to its issued revision',
        );

        final repeated = await provider.commit(
          commitRef: 'commit.1',
          resourceRef: 'resource.a',
          expectedRevision: before.revision,
          draftRef: draft.draftRef,
          audience: 'worker.office',
          generation: 2,
          idempotencyKey: 'idem.1',
        );
        expect(repeated, same(receipt));

        final conflict = await provider.commit(
          commitRef: 'commit.2',
          resourceRef: 'resource.a',
          expectedRevision: before.revision,
          draftRef: draft.draftRef,
          audience: 'worker.office',
          generation: 2,
          idempotencyKey: 'idem.2',
        );
        expect(conflict.result, MuseAuthorityCommitResult.conflict);
        expect(conflict.currentRevision, receipt.newRevision);
        final repeatedConflict = await provider.commit(
          commitRef: 'commit.2',
          resourceRef: 'resource.a',
          expectedRevision: before.revision,
          draftRef: draft.draftRef,
          audience: 'worker.office',
          generation: 2,
          idempotencyKey: 'idem.2',
        );
        expect(repeatedConflict, same(conflict));
      });

      test('blob success plus metadata failure never returns commit success',
          () async {
        final before = await provider.describe('resource.a');
        final draft = await provider.createDraft(
          'resource.a',
          audience: 'worker.office',
          generation: 2,
          ttlMs: 100,
        );
        await provider.writeDraft(
          draft.draftRef,
          [6, 6, 6],
          audience: 'worker.office',
          generation: 2,
        );
        provider.failNextMetadataCommit = true;
        await expectLater(
          provider.commit(
            commitRef: 'commit.fail',
            resourceRef: 'resource.a',
            expectedRevision: before.revision,
            draftRef: draft.draftRef,
            audience: 'worker.office',
            generation: 2,
            idempotencyKey: 'idem.fail',
          ),
          throwsA(
            isA<MuseAuthorityException>().having(
              (error) => error.code,
              'code',
              'METADATA_COMMIT_FAILED',
            ),
          ),
        );
        expect(
            (await provider.describe('resource.a')).revision, before.revision);
        expect(provider.orphanBlobCount, 1);
      });
    });
  }

  test('resource descriptor parser rejects a physical path', () {
    expect(
      () => MuseResourceDescriptorV1.fromJson({
        'protocol': 'muse.resource/descriptor/v1',
        'resourceRef': 'resource.bad',
        'revision': 'revision.1',
        'displayName': 'bad.txt',
        'mediaType': 'text/plain',
        'path': '/private/bad.txt',
        'format': {'formatId': 'text.plain', 'confidence': 'claimed'},
        'capabilities': ['describe'],
        'security': {'classification': 'internal', 'activeContent': 'none'},
      }),
      throwsA(isA<MuseContractFormatException>()),
    );
  });
}
