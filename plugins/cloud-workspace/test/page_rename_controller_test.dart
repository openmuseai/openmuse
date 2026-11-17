import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'catalog_test_support.dart';
import 'page_rename_test_support.dart';

void main() {
  test(
    'current original title loads, one rename confirms without changing page identity',
    () async {
      final f = await RenameUiFixture.create();
      addTearDown(f.close);
      final c = f.controller();
      addTearDown(c.dispose);
      f.names[catalogParent] = 'Current original title';
      await c.load();
      expect(c.name, 'Current original title');
      expect(c.canSubmit, isFalse);
      c.setName('原目录改名😀');
      final result = await c.submit();
      expect(result!.viewId, catalogParent);
      expect(result.name, '原目录改名😀');
      expect(f.writes, [
        {'name': '原目录改名😀'},
      ]);
      c.finishConfirmed(c.pending!);
      expect(c.pending, isNull);
    },
  );
  test(
    'closed controller retains unknown intent and next presentation performs GET-only review',
    () async {
      final f = await RenameUiFixture.create();
      addTearDown(f.close);
      f.breakAck = true;
      final c = f.controller();
      await c.load();
      c.setName('Retained rename');
      expect(await c.submit(), isNull);
      final id = c.pending!.operationId;
      c.dispose();
      final review = f.controller();
      addTearDown(review.dispose);
      await review.load();
      expect(review.name, 'Retained rename');
      await review.check(review.pending!);
      expect(review.pending!.status, AppFlowyPageRenameStatus.ready);
      expect(review.confirmed(review.pending!)!.name, 'Retained rename');
      expect(review.pending!.operationId, id);
      expect(f.writes, hasLength(1));
    },
  );
  for (final sent in [false, true]) {
    test(
      'source retirement fences rename presentation after ${sent ? 'POST' : 'preflight'}',
      () async {
        final f = await RenameUiFixture.create();
        addTearDown(f.close);
        final c = f.controller();
        addTearDown(c.dispose);
        await c.load();
        c.setName('Old source input');
        final arrived = Completer<void>(), release = Completer<void>();
        f.guard = (r) async {
          if (sent
              ? !r.uri.path.endsWith('/update-name')
              : !r.uri.path.endsWith('/folder'))
            return false;
          arrived.complete();
          await release.future;
          return false;
        };
        final operation = c.submit();
        await arrived.future;
        c.retire();
        release.complete();
        expect(await operation, isNull);
        expect(c.retired, isTrue);
        expect(c.name, isEmpty);
        expect(c.target, isNull);
        expect(f.writes.length, sent ? 1 : 0);
        expect(
          f.catalog.service.pendingPageRename(
                catalogWorkspace,
                catalogParent,
              ) !=
              null,
          sent,
        );
      },
    );
  }
  test(
    'Guest has no rename target and A-B-A cannot revive a retired draft',
    () async {
      final f = await RenameUiFixture.create();
      addTearDown(f.close);
      f.role = 'Guest';
      final c = f.controller();
      addTearDown(c.dispose);
      await c.load();
      expect(c.failure, 'readonly');
      expect(c.target, isNull);
      f.role = 'Owner';
      await c.load();
      c.setName('Private draft');
      f.catalog.auth.switchTo('bob');
      f.catalog.service.invalidateAccountOperations();
      f.catalog.auth.switchTo('alice');
      await c.load();
      expect(c.name, isEmpty);
      expect(c.retired, isTrue);
      expect(await c.submit(), isNull);
      expect(f.writes, isEmpty);
    },
  );
}
