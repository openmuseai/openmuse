import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'catalog_test_support.dart';

void main() {
  test(
    'original IDs/order, nearest visible ancestry and local collapse/search',
    () async {
      final f = await CatalogFixture.create();
      addTearDown(f.close);
      final c = f.controller();
      addTearDown(c.dispose);
      await c.refresh();
      expect(c.failure, isNull);
      expect(c.entries.map((e) => e.page.viewId), [
        catalogParent,
        catalogChild,
        catalogSibling,
      ]);
      final child = c.entries[1];
      expect((child.depth, child.breadcrumb), (1, 'Parent document'));
      expect((child.page as AppFlowyDatabasePage).databaseId, catalogRoot);
      expect(c.entries.first.hasChildren, isTrue);
      c.toggle(catalogParent);
      expect(c.entries.map((e) => e.page.viewId), [
        catalogParent,
        catalogSibling,
      ]);
      c.search('  ORIGINAL DATABASE  ');
      expect(c.entries.map((e) => e.page.viewId), [
        catalogParent,
        catalogChild,
      ]);
      expect(c.entries.first.expanded, isTrue);
      c.search('');
      expect(c.entries.map((e) => e.page.viewId), [
        catalogParent,
        catalogSibling,
      ]);
      expect(f.seen.every((s) => s.startsWith('GET ')), isTrue);
    },
  );

  test(
    'refresh drops stale entries on failure and supports explicit retry',
    () async {
      final f = await CatalogFixture.create();
      addTearDown(f.close);
      final c = f.controller();
      addTearDown(c.dispose);
      await c.refresh();
      f.intercept = (r) async {
        if (!r.uri.path.endsWith('/folder')) return false;
        await f.reply(r, {'view_id': catalogWorkspace});
        return true;
      };
      await c.refresh();
      expect(c.failure, 'load');
      expect(c.entries, isEmpty);
      f.intercept = null;
      await c.refresh();
      expect(c.failure, isNull);
      expect(c.pageCount, 3);
    },
  );

  test('late load cannot revive a retired A-B-A account catalog', () async {
    final f = await CatalogFixture.create();
    addTearDown(f.close);
    final c = f.controller();
    addTearDown(c.dispose);
    final arrived = Completer<void>(), release = Completer<void>();
    f.intercept = (r) async {
      if (!r.uri.path.endsWith('/folder')) return false;
      arrived.complete();
      await release.future;
      await f.reply(r, f.folder(catalogWorkspace));
      return true;
    };
    final reading = c.refresh();
    await arrived.future;
    f.auth.switchTo('bob');
    f.service.invalidateAccountOperations();
    f.auth.switchTo('alice');
    release.complete();
    await reading;
    expect(c.retired, isTrue);
    expect(c.entries, isEmpty);
    expect(c.busy, isFalse);
    final count = f.seen.length;
    await c.refresh();
    expect(f.seen.length, count);
  });

  test(
    'composite catalog rejects account changes instead of returning old document fallback',
    () async {
      final f = await CatalogFixture.create();
      addTearDown(f.close);
      final arrived = Completer<void>(), release = Completer<void>();
      f.intercept = (r) async {
        if (!r.uri.path.endsWith('/database')) return false;
        arrived.complete();
        await release.future;
        await f.reply(r, []);
        return true;
      };
      final result = f.service.workspacePages(catalogWorkspace);
      final checked = expectLater(
        result,
        throwsA(isA<CloudServiceException>()),
      );
      await arrived.future;
      f.auth.switchTo('bob');
      f.service.invalidateAccountOperations();
      f.auth.switchTo('alice');
      release.complete();
      await checked;
    },
  );

  for (final status in [404, 501]) {
    test(
      'same-account down-level database catalog retains original document fallback: $status',
      () async {
        final f = await CatalogFixture.create();
        addTearDown(f.close);
        f.intercept = (r) async {
          if (!r.uri.path.endsWith('/database')) return false;
          r.response.statusCode = status;
          await r.response.close();
          return true;
        };
        final pages = await f.service.workspacePages(catalogWorkspace);
        expect(pages.map((p) => p.viewId), [catalogParent, catalogSibling]);
      },
    );
  }

  for (final failure in [401, 403, 503, 1011, 1012, 1024]) {
    test(
      'catalog denial/failure $failure cannot return old document fallback',
      () async {
        final f = await CatalogFixture.create();
        addTearDown(f.close);
        f.intercept = (r) async {
          if (!r.uri.path.endsWith('/database')) return false;
          if (failure < 1000) {
            r.response.statusCode = failure;
            await r.response.close();
          } else {
            await f.reply(r, null, code: failure);
          }
          return true;
        };
        final c = f.controller();
        addTearDown(c.dispose);
        await c.refresh();
        expect(c.failure, 'load');
        expect(c.entries, isEmpty);
      },
    );
  }

  test(
    'malformed database response is an error rather than a document fallback',
    () async {
      final f = await CatalogFixture.create();
      addTearDown(f.close);
      f.intercept = (r) async {
        if (!r.uri.path.endsWith('/database')) return false;
        r.response.write('not-json');
        await r.response.close();
        return true;
      };
      final c = f.controller();
      addTearDown(c.dispose);
      await c.refresh();
      expect(c.failure, 'load');
      expect(c.entries, isEmpty);
    },
  );

  test(
    'one open at a time, stale page rejected and retired completion cannot navigate',
    () async {
      final f = await CatalogFixture.create();
      addTearDown(f.close);
      final c = f.controller();
      addTearDown(c.dispose);
      await c.refresh();
      final old = c.entries.first.page;
      await c.refresh();
      var calls = 0;
      expect(
        await c.open(old, (_) async {
          calls++;
        }),
        isFalse,
      );
      final release = Completer<void>();
      final opening = c.open(c.entries.first.page, (_) async {
        calls++;
        await release.future;
      });
      expect(
        await c.open(c.entries.last.page, (_) async {
          calls++;
        }),
        isFalse,
      );
      final count = f.seen.length;
      await c.refresh();
      expect(f.seen.length, count);
      f.auth.switchTo('bob');
      release.complete();
      expect(await opening, isFalse);
      expect(calls, 1);
      expect(c.openingId, isNull);
    },
  );

  test('disposed load never notifies after completion', () async {
    final f = await CatalogFixture.create();
    addTearDown(f.close);
    final c = f.controller();
    final arrived = Completer<void>(), release = Completer<void>();
    f.intercept = (r) async {
      if (!r.uri.path.endsWith('/folder')) return false;
      arrived.complete();
      await release.future;
      await f.reply(r, f.folder(catalogWorkspace));
      return true;
    };
    var notifications = 0;
    c.addListener(() => notifications++);
    final reading = c.refresh();
    await arrived.future;
    c.dispose();
    final count = notifications;
    release.complete();
    await reading;
    expect(notifications, count);
  });
}
