import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'catalog_test_support.dart';
import 'document_creation_ui_test_support.dart';

void main() {
  test(
    'presentation delegates one original command and retains its original page',
    () async {
      final f = await DocumentUiFixture.create();
      addTearDown(f.close);
      final c = f.controller();
      addTearDown(c.dispose);
      await c.load();
      c.setName(' 新建文档😀 ');
      c.selectParent(catalogSibling);
      final page = await c.submit();
      expect(page, isA<AppFlowyDocumentPage>());
      expect(page!.ancestors.last, catalogSibling);
      expect(f.bodies.single['parent_view_id'], catalogSibling);
      expect(c.pending!.name, '新建文档😀');
      expect(c.readyPage(c.pending!), same(page));
      expect(await c.submit(), isNull);
      expect(f.bodies, hasLength(1));
    },
  );

  for (final sent in [false, true]) {
    test(
      'source retirement during ${sent ? 'POST' : 'preflight'} never opens or resends',
      () async {
        final f = await DocumentUiFixture.create();
        addTearDown(f.close);
        final c = f.controller();
        addTearDown(c.dispose);
        await c.load();
        c.setName('Retired name');
        final arrived = Completer<void>(), release = Completer<void>();
        f.intercept = (r) async {
          if (sent ? r.method != 'POST' : !r.uri.path.endsWith('/folder'))
            return false;
          arrived.complete();
          await release.future;
          if (!sent)
            await f.catalog.reply(r, f.catalog.folder(catalogWorkspace));
          return !sent;
        };
        final operation = c.submit();
        await arrived.future;
        c.retire();
        expect(c.name, isEmpty);
        expect(c.parents, isEmpty);
        release.complete();
        expect(await operation, isNull);
        expect(f.bodies.length, sent ? 1 : 0);
        expect(
          f.catalog.service.pendingDocumentCreation(catalogWorkspace) != null,
          sent,
        );
        expect(c.pending, isNull);
      },
    );
  }

  test(
    'unknown result survives controller close, GET review never submits again',
    () async {
      final f = await DocumentUiFixture.create();
      addTearDown(f.close);
      final first = f.controller();
      await first.load();
      first.setName('Retained draft');
      f.breakAck = true;
      expect(await first.submit(), isNull);
      final id = first.pending!.documentId;
      first.dispose();
      final review = f.controller();
      addTearDown(review.dispose);
      await review.load();
      expect(review.name, 'Retained draft');
      await review.check(review.pending!);
      expect(review.pending!.status, AppFlowyDocumentCreationStatus.ready);
      expect(review.readyPage(review.pending!)!.viewId, id);
      expect(f.bodies, hasLength(1));
    },
  );

  test('A-B-A account retirement hides creation inputs permanently', () async {
    final f = await DocumentUiFixture.create();
    addTearDown(f.close);
    final c = f.controller();
    addTearDown(c.dispose);
    await c.load();
    c.setName('Private input');
    f.catalog.auth.switchTo('bob');
    f.catalog.service.invalidateAccountOperations();
    f.catalog.auth.switchTo('alice');
    await c.load();
    expect(c.retired, isTrue);
    expect(c.name, isEmpty);
    expect(c.parents, isEmpty);
    expect(await c.submit(), isNull);
    expect(f.bodies, isEmpty);
  });

  test('original Guest role offers no parent or submit', () async {
    final f = await DocumentUiFixture.create();
    addTearDown(f.close);
    f.role = 'Guest';
    final c = f.controller();
    addTearDown(c.dispose);
    await c.load();
    expect(c.failure, 'readonly');
    expect(c.canSubmit, isFalse);
    expect(f.bodies, isEmpty);
  });
}
