import 'dart:async';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';
import 'document_creation_test_support.dart';

Matcher blocked(String code) =>
    isA<AppFlowyDocumentCreationBlocked>().having((e) => e.code, 'code', code);

void main() {
  test(
    'original creation uses shared Unicode title rules without treating slash as a path',
    () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      final title = '${'😀' * 511}/';
      final result = await f.service.createDocument(await f.target(), title);
      expect(result.status, AppFlowyDocumentCreationStatus.ready);
      expect(result.page!.name, title);
      expect(f.bodies.single['name'], title);
      f.service.finishDocumentCreation(result);
      final target = await f.target();
      for (final invalid in [
        '😀' * 513,
        'line\u2028break',
        'line\u2029break',
      ]) {
        await expectLater(
          f.service.createDocument(target, invalid),
          throwsArgumentError,
        );
      }
      expect(f.bodies, hasLength(1));
      expect(f.service.pendingDocumentCreation(documentWorkspace), isNull);
    },
  );

  for (final sent in [false, true]) {
    test('retirement cancels only unsent preflight: sent=$sent', () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      final target = await f.target();
      final arrived = Completer<void>(), release = Completer<void>();
      if (sent) {
        f.post = (r, value) async {
          arrived.complete();
          await release.future;
          await f.reply(r, {'view_id': value['view_id']});
        };
      } else {
        f.intercept = (r) async {
          if (!r.uri.path.endsWith('/folder')) return false;
          arrived.complete();
          await release.future;
          await f.reply(r, f.folder());
          return true;
        };
      }
      final operation = f.service.createDocument(target, 'Retired draft');
      final checked = sent
          ? null
          : expectLater(operation, throwsA(blocked('SOURCE_ENDED')));
      await arrived.future;
      final pending = f.service.pendingDocumentCreation(documentWorkspace)!;
      expect(f.service.cancelDocumentCreationPreflight(pending), !sent);
      release.complete();
      if (sent) {
        expect((await operation).status, AppFlowyDocumentCreationStatus.ready);
        expect(f.service.pendingDocumentCreation(documentWorkspace), isNotNull);
      } else {
        await checked;
        expect(f.service.pendingDocumentCreation(documentWorkspace), isNull);
      }
      expect(f.bodies.length, sent ? 1 : 0);
    });
  }
  test(
    'original targets retain source identity and canonical one-POST creation',
    () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      final targets = await f.service.documentCreationParents(
        documentWorkspace,
      );
      expect(targets.map((p) => p.viewId), [documentSpace, documentParent]);
      expect(targets.last.breadcrumb, 'General');
      expect(() => targets.last.ancestors.add('x'), throwsUnsupportedError);
      final result = await f.service.createDocument(
        targets.last,
        '  Original document😀  ',
      );
      expect(result.status, AppFlowyDocumentCreationStatus.ready);
      expect(result.serverReturnedId, isTrue);
      expect(result.name, 'Original document😀');
      expect(result.page!.viewId, result.documentId);
      expect(f.bodies, hasLength(1));
      expect(f.bodies.single, {
        'parent_view_id': documentParent,
        'layout': 0,
        'name': result.name,
        'view_id': result.documentId,
        'collab_id': result.documentId,
      });
      f.service.finishDocumentCreation(result);
      expect(f.service.pendingDocumentCreation(documentWorkspace), isNull);
    },
  );

  for (final kind in ['renamed', 'moved', 'Guest']) {
    test(
      'preflight $kind rejects before POST and retains no ambiguous intent',
      () async {
        final f = await DocumentCreationFixture.create();
        addTearDown(f.close);
        final target = await f.target();
        if (kind == 'renamed') f.parentName = 'New parent name';
        if (kind == 'moved') f.movedParent = true;
        if (kind == 'Guest') f.role = 'Guest';
        await expectLater(
          f.service.createDocument(target, 'Draft'),
          throwsA(blocked(kind == 'Guest' ? 'READ_ONLY' : 'TARGET_CHANGED')),
        );
        expect(f.bodies, isEmpty);
        expect(f.service.pendingDocumentCreation(documentWorkspace), isNull);
      },
    );
  }

  test(
    'Guest receives no creation parent choices from a readable workspace',
    () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      f.role = 'Guest';
      await expectLater(f.target(), throwsA(blocked('READ_ONLY')));
      expect(f.folderReads, 0);
      expect(f.bodies, isEmpty);
    },
  );

  test(
    'parent from another service cannot create in a same-ID workspace',
    () async {
      final a = await DocumentCreationFixture.create(),
          b = await DocumentCreationFixture.create();
      addTearDown(a.close);
      addTearDown(b.close);
      await expectLater(
        b.service.createDocument(await a.target(), 'Draft'),
        throwsA(blocked('SOURCE_ENDED')),
      );
      expect(b.bodies, isEmpty);
    },
  );

  test(
    'invalid names fail before reserving or delivering an operation',
    () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      final target = await f.target();
      for (final name in [
        '',
        '  ',
        'x' * 513,
        'line\nbreak',
        'bad\u0000title',
      ]) {
        await expectLater(
          f.service.createDocument(target, name),
          throwsArgumentError,
        );
      }
      expect(f.bodies, isEmpty);
      expect(f.service.pendingDocumentCreation(documentWorkspace), isNull);
    },
  );

  test(
    'broken ACK retains input and a second submit never replays POST',
    () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      f.post = (r, _) async {
        final socket = await r.response.detachSocket(writeHeaders: false);
        socket.destroy();
      };
      final target = await f.target();
      final pending = await f.service.createDocument(
        target,
        'Retained original draft',
      );
      expect(pending.status, AppFlowyDocumentCreationStatus.unconfirmed);
      expect(pending.serverReturnedId, isFalse);
      expect(pending.name, 'Retained original draft');
      await expectLater(
        f.service.createDocument(target, 'Other input'),
        throwsA(blocked('REVIEW_REQUIRED')),
      );
      expect(f.bodies, hasLength(1));
      final observed = await f.service.reconcileDocumentCreation(pending);
      expect(observed.status, AppFlowyDocumentCreationStatus.ready);
      expect(observed.documentId, pending.documentId);
      expect(f.bodies, hasLength(1));
    },
  );

  test(
    'accepted ID with delayed Folder publication is checked with GET only',
    () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      f.publish = false;
      final pending = await f.service.createDocument(
        await f.target(),
        'Publishing document',
      );
      expect(pending.status, AppFlowyDocumentCreationStatus.unconfirmed);
      expect(pending.serverReturnedId, isTrue);
      expect(pending.page, isNull);
      expect(
        () => f.service.finishDocumentCreation(pending),
        throwsA(blocked('REVIEW_REQUIRED')),
      );
      f.published.add(f.bodies.single);
      final result = await f.service.reconcileDocumentCreation(pending);
      expect(result.status, AppFlowyDocumentCreationStatus.ready);
      expect(f.bodies, hasLength(1));
    },
  );

  test(
    'same-ID resource with changed name stays reviewable and is never overwritten',
    () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      f.publish = false;
      final pending = await f.service.createDocument(
        await f.target(),
        'Original input',
      );
      f.published.add({...f.bodies.single, 'name': 'Peer renamed'});
      final result = await f.service.reconcileDocumentCreation(pending);
      expect(result.status, AppFlowyDocumentCreationStatus.unconfirmed);
      expect(result.page!.name, 'Peer renamed');
      expect(result.name, 'Original input');
      expect(f.bodies, hasLength(1));
    },
  );

  test(
    'explicitly ending review does not retry/delete, old callback cannot finish a new intent',
    () async {
      final f = await DocumentCreationFixture.create();
      addTearDown(f.close);
      f.publish = false;
      final target = await f.target(),
          first = await f.service.createDocument(await f.target(), 'First');
      f.service.finishDocumentCreation(first, reviewed: true);
      final second = await f.service.createDocument(target, 'Second');
      expect(
        () => f.service.finishDocumentCreation(first, reviewed: true),
        throwsA(blocked('SOURCE_ENDED')),
      );
      expect(
        f.service.pendingDocumentCreation(documentWorkspace)!.documentId,
        second.documentId,
      );
      expect(f.bodies, hasLength(2));
    },
  );

  for (final stage in ['preflight', 'POST']) {
    test(
      'account A-B-A during $stage cannot populate the new source',
      () async {
        final f = await DocumentCreationFixture.create();
        addTearDown(f.close);
        final target = await f.target(),
            arrived = Completer<void>(),
            release = Completer<void>();
        if (stage == 'preflight') {
          f.intercept = (r) async {
            if (!r.uri.path.endsWith('/folder')) return false;
            arrived.complete();
            await release.future;
            await f.reply(r, f.folder());
            return true;
          };
        } else {
          f.post = (r, value) async {
            arrived.complete();
            await release.future;
            await f.reply(r, {'view_id': value['view_id']});
          };
        }
        final future = f.service.createDocument(target, 'Old account draft');
        final checked = expectLater(
          future,
          throwsA(isA<CloudServiceException>()),
        );
        await arrived.future;
        f.subject = 'bob';
        f.service.invalidateAccountOperations();
        f.subject = 'alice';
        release.complete();
        await checked;
        expect(f.service.pendingDocumentCreation(documentWorkspace), isNull);
        expect(f.bodies.length, stage == 'POST' ? 1 : 0);
      },
    );
  }

  test(
    'delivery timeout remains unknown and late ACK never reinstates finished input',
    () async {
      final f = await DocumentCreationFixture.create(
        timeout: const Duration(milliseconds: 100),
      );
      addTearDown(f.close);
      final release = Completer<void>();
      f.post = (r, value) async {
        await release.future;
        await f.reply(r, {'view_id': value['view_id']});
      };
      final pending = await f.service.createDocument(
        await f.target(),
        'Timed out draft',
      );
      expect(pending.status, AppFlowyDocumentCreationStatus.unconfirmed);
      expect(f.bodies, hasLength(1));
      final ready = await f.service.reconcileDocumentCreation(pending);
      expect(ready.status, AppFlowyDocumentCreationStatus.ready);
      f.service.finishDocumentCreation(ready);
      release.complete();
      await Future<void>.delayed(const Duration(milliseconds: 30));
      expect(f.service.pendingDocumentCreation(documentWorkspace), isNull);
      expect(f.bodies, hasLength(1));
    },
  );
}
