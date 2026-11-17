import 'dart:async';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:test/test.dart';
import 'appflowy_page_rename_test.dart' show RenameFixture;
import 'document_creation_test_support.dart';

void main() {
  test(
    'an earlier directory GET released last cannot regress the newer name observation',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      final arrived = Completer<void>(), release = Completer<void>();
      var held = false;
      final observed = <AppFlowyPageNameObservation>[];
      final subscription = f.service.pageNameChanges.listen(observed.add);
      addTearDown(subscription.cancel);
      f.guard = (r) async {
        if (!r.uri.path.endsWith('/folder') || held) return false;
        held = true;
        final old = f.base.folder();
        arrived.complete();
        await release.future;
        await f.base.reply(r, old);
        return true;
      };
      final earlier = f.service.workspacePages(documentWorkspace);
      await arrived.future;
      f.base.parentName = 'Newer verified name';
      await f.service.workspacePages(documentWorkspace);
      await Future<void>.delayed(Duration.zero);
      expect(observed.last.page.name, 'Newer verified name');
      release.complete();
      expect((await earlier).single.name, 'Newer verified name');
      await Future<void>.delayed(Duration.zero);
      expect(observed.last.page.name, 'Newer verified name');
    },
  );
  test(
    'newer failed read keeps last verified labels and never publishes an older delayed response',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      await f.service.workspacePages(documentWorkspace);
      final before = f.service.currentPageNameObservation(
        documentWorkspace,
        documentParent,
      )!;
      final observed = <AppFlowyPageNameObservation>[];
      final sub = f.service.pageNameChanges.listen(observed.add);
      addTearDown(sub.cancel);
      final arrived = Completer<void>(), release = Completer<void>();
      var folders = 0;
      f.guard = (r) async {
        if (!r.uri.path.endsWith('/folder')) return false;
        folders++;
        if (folders == 1) {
          final old = f.base.folder();
          arrived.complete();
          await release.future;
          await f.base.reply(r, old);
        } else {
          r.response.statusCode = 503;
          await r.response.close();
        }
        return true;
      };
      f.base.parentName = 'Older unobserved snapshot';
      final older = f.service.workspacePages(documentWorkspace);
      await arrived.future;
      await expectLater(
        f.service.workspacePages(documentWorkspace),
        throwsA(isA<Exception>()),
      );
      release.complete();
      expect((await older), isNotEmpty);
      await Future<void>.delayed(Duration.zero);
      expect(observed, isEmpty);
      expect(f.service.isCurrentPageNameObservation(before), isTrue);
      expect(
        f.service
            .currentPageNameObservation(documentWorkspace, documentParent)!
            .page
            .name,
        'Parent document',
      );
    },
  );

  test(
    'late rename confirmation observes a newer peer title and never reports the old desired name ready',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      final target = await f.target();
      final arrived = Completer<void>(), release = Completer<void>();
      var held = false;
      f.guard = (r) async {
        if (!r.uri.path.endsWith('/folder') || f.writes.isEmpty || held)
          return false;
        held = true;
        final old = f.base.folder();
        arrived.complete();
        await release.future;
        await f.base.reply(r, old);
        return true;
      };
      final rename = f.service.renamePage(target, 'Desired before peer');
      await arrived.future;
      f.base.parentName = 'Newer peer title';
      await f.service.workspacePages(documentWorkspace);
      release.complete();
      final result = await rename;
      expect(result.status, AppFlowyPageRenameStatus.unconfirmed);
      expect(result.observed!.name, 'Newer peer title');
      expect(result.name, 'Desired before peer');
      expect(f.writes, hasLength(1));
    },
  );

  test(
    'a superseded preflight uses newer original metadata and rejects before POST',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      final target = await f.target();
      final arrived = Completer<void>(), release = Completer<void>();
      var held = false;
      f.guard = (r) async {
        if (!r.uri.path.endsWith('/folder') || held) return false;
        held = true;
        final old = f.base.folder();
        arrived.complete();
        await release.future;
        await f.base.reply(r, old);
        return true;
      };
      final rename = f.service.renamePage(target, 'My draft');
      final checked = expectLater(
        rename,
        throwsA(
          isA<AppFlowyPageRenameBlocked>().having(
            (e) => e.code,
            'code',
            'TARGET_CHANGED',
          ),
        ),
      );
      await arrived.future;
      f.base.parentName = 'Peer changed before send';
      await f.service.workspacePages(documentWorkspace);
      release.complete();
      await checked;
      expect(f.writes, isEmpty);
      expect(f.pending, isNull);
    },
  );

  test(
    'complete newer empty catalog retires old observations without reviving removed pages',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      await f.service.workspacePages(documentWorkspace);
      final before = f.service.currentPageNameObservation(
        documentWorkspace,
        documentParent,
      )!;
      final arrived = Completer<void>(), release = Completer<void>();
      var folders = 0;
      f.guard = (r) async {
        if (!r.uri.path.endsWith('/folder')) return false;
        folders++;
        if (folders == 1) {
          final old = f.base.folder();
          arrived.complete();
          await release.future;
          await f.base.reply(r, old);
        } else {
          final empty = f.base.folder();
          empty['children'] = <Object?>[];
          await f.base.reply(r, empty);
        }
        return true;
      };
      final older = f.service.workspacePages(documentWorkspace);
      await arrived.future;
      expect(await f.service.workspacePages(documentWorkspace), isEmpty);
      release.complete();
      expect(await older, isEmpty);
      expect(
        f.service.currentPageNameObservation(documentWorkspace, documentParent),
        isNull,
      );
      expect(f.service.isCurrentPageNameObservation(before), isFalse);
    },
  );

  test(
    'A-B-A during an older read cannot repopulate the account label cache',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      await f.service.workspacePages(documentWorkspace);
      final before = f.service.currentPageNameObservation(
        documentWorkspace,
        documentParent,
      )!;
      final arrived = Completer<void>(), release = Completer<void>();
      f.guard = (r) async {
        if (!r.uri.path.endsWith('/folder')) return false;
        final old = f.base.folder();
        arrived.complete();
        await release.future;
        await f.base.reply(r, old);
        return true;
      };
      final older = f.service.workspacePages(documentWorkspace);
      final checked = expectLater(older, throwsA(isA<Exception>()));
      await arrived.future;
      f.base.subject = 'bob';
      f.service.invalidateAccountOperations();
      f.base.subject = 'alice';
      release.complete();
      await checked;
      expect(
        f.service.currentPageNameObservation(documentWorkspace, documentParent),
        isNull,
      );
      expect(f.service.isCurrentPageNameObservation(before), isFalse);
    },
  );

  test(
    'workspace read sequencing is independent while account observations stay bound to one service',
    () async {
      const otherWorkspace = '10000000-0000-0000-0000-000000000002';
      final f = await RenameFixture.create(),
          otherService = await RenameFixture.create();
      addTearDown(f.close);
      addTearDown(otherService.close);
      final arrived = Completer<void>(), release = Completer<void>();
      var held = false;
      f.guard = (r) async {
        if (r.uri.path == '/api/workspace') {
          await f.base.reply(r, [
            for (final wid in [documentWorkspace, otherWorkspace])
              {'workspace_id': wid, 'role': 'Owner'},
          ]);
          return true;
        }
        if (!r.uri.path.endsWith('/folder')) return false;
        if (r.uri.pathSegments[2] == otherWorkspace) {
          final folder = f.base.folder();
          folder['view_id'] = otherWorkspace;
          await f.base.reply(r, folder);
          return true;
        }
        if (held) return false;
        held = true;
        final folder = f.base.folder();
        arrived.complete();
        await release.future;
        await f.base.reply(r, folder);
        return true;
      };
      final first = f.service.workspacePages(documentWorkspace);
      await arrived.future;
      await f.service.workspacePages(otherWorkspace);
      release.complete();
      await first;
      final observation = f.service.currentPageNameObservation(
        documentWorkspace,
        documentParent,
      )!;
      expect(f.service.isCurrentPageNameObservation(observation), isTrue);
      expect(
        otherService.service.isCurrentPageNameObservation(observation),
        isFalse,
      );
      expect(
        f.service
            .currentPageNameObservation(otherWorkspace, documentParent)!
            .page
            .workspaceId,
        otherWorkspace,
      );
    },
  );

  for (final malformed in ['binding', 'duplicate']) {
    test(
      'malformed $malformed catalog cannot replace last verified names',
      () async {
        final f = await RenameFixture.create();
        addTearDown(f.close);
        await f.service.workspacePages(documentWorkspace);
        final before = f.service.currentPageNameObservation(
          documentWorkspace,
          documentParent,
        )!;
        f.guard = (r) async {
          if (!r.uri.path.endsWith('/folder')) return false;
          final folder = f.base.folder();
          if (malformed == 'binding') {
            folder['view_id'] = '10000000-0000-0000-0000-000000000002';
          } else {
            final space = (folder['children'] as List).first as Map;
            final parent = (space['children'] as List).first;
            space['children'] = [parent, parent];
          }
          await f.base.reply(r, folder);
          return true;
        };
        await expectLater(
          f.service.workspacePages(documentWorkspace),
          throwsA(isA<Exception>()),
        );
        expect(
          f.service
              .currentPageNameObservation(documentWorkspace, documentParent)!
              .page
              .name,
          before.page.name,
        );
        expect(f.service.isCurrentPageNameObservation(before), isTrue);
      },
    );
  }

  test(
    'derived label cache is bounded to32 workspaces and closes with the service',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      final workspaces = [
        for (var i = 1; i <= 33; i++)
          '10000000-0000-0000-0000-${i.toString().padLeft(12, '0')}',
      ];
      f.guard = (r) async {
        if (r.uri.path == '/api/workspace') {
          await f.base.reply(r, [
            for (final wid in workspaces)
              {'workspace_id': wid, 'role': 'Owner'},
          ]);
          return true;
        }
        if (!r.uri.path.endsWith('/folder')) return false;
        final folder = f.base.folder();
        folder['view_id'] = r.uri.pathSegments[2];
        await f.base.reply(r, folder);
        return true;
      };
      for (final wid in workspaces) {
        await f.service.workspacePages(wid);
      }
      expect(
        f.service.currentPageNameObservation(workspaces.first, documentParent),
        isNull,
      );
      expect(
        f.service.currentPageNameObservation(workspaces[1], documentParent),
        isNotNull,
      );
      final before = f.service.currentPageNameObservation(
        workspaces.last,
        documentParent,
      )!;
      f.service.closeClient();
      expect(
        f.service.currentPageNameObservation(workspaces.last, documentParent),
        isNull,
      );
      expect(f.service.isCurrentPageNameObservation(before), isFalse);
    },
  );
  test(
    'creation confirmation cannot accept a delayed name after a newer peer rename',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      final parent = await f.base.target();
      final arrived = Completer<void>(), release = Completer<void>();
      var held = false;
      f.guard = (r) async {
        if (!r.uri.path.endsWith('/folder') || f.base.bodies.isEmpty || held)
          return false;
        held = true;
        final old = f.base.folder();
        arrived.complete();
        await release.future;
        await f.base.reply(r, old);
        return true;
      };
      final creation = f.service.createDocument(
        parent,
        'Created original name',
      );
      await arrived.future;
      f.base.published.single['name'] = 'Peer renamed after creation';
      await f.service.workspacePages(documentWorkspace);
      release.complete();
      final result = await creation;
      expect(result.status, AppFlowyDocumentCreationStatus.unconfirmed);
      expect(result.page!.name, 'Peer renamed after creation');
      expect(result.name, 'Created original name');
      expect(f.base.bodies, hasLength(1));
      expect(
        f.service
            .currentPageNameObservation(documentWorkspace, result.documentId)!
            .page
            .name,
        'Peer renamed after creation',
      );
    },
  );
}
