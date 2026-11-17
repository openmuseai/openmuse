import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:test/test.dart';
import 'document_creation_test_support.dart';

Matcher blocked(String code) =>
    isA<AppFlowyPageRenameBlocked>().having((e) => e.code, 'code', code);

class RenameFixture {
  RenameFixture(this.base);
  final DocumentCreationFixture base;
  final writes = <Map<String, dynamic>>[];
  bool breakAck = false, publish = true;
  Future<bool> Function(HttpRequest)? guard;
  Future<void> Function()? beforeAck;
  static Future<RenameFixture> create({
    Duration timeout = const Duration(seconds: 2),
  }) async {
    final f = RenameFixture(
      await DocumentCreationFixture.create(renameTimeout: timeout),
    );
    f.base.intercept = (r) async {
      if (await f.guard?.call(r) ?? false) return true;
      if (!r.uri.path.endsWith('/update-name')) return false;
      final body = (jsonDecode(await utf8.decoder.bind(r).join()) as Map)
          .cast<String, dynamic>();
      f.writes.add(body);
      if (f.publish) f.base.parentName = body['name'] as String;
      await f.beforeAck?.call();
      if (f.breakAck) {
        final socket = await r.response.detachSocket(writeHeaders: false);
        socket.destroy();
      } else {
        await f.base.reply(r, null);
      }
      return true;
    };
    return f;
  }

  AppFlowyCloudWorkspaceService get service => base.service;
  Future<AppFlowyPageRenameTarget> target() =>
      service.preparePageRename(documentWorkspace, documentParent);
  AppFlowyPageRename? get pending =>
      service.pendingPageRename(documentWorkspace, documentParent);
  Future<void> close() => base.close();
}

void main() {
  test(
    'SiYuan Unicode title limit preserves 512 code points and literal slash',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      final title = '${'😀' * 511}/';
      final result = await f.service.renamePage(await f.target(), title);
      expect(result.status, AppFlowyPageRenameStatus.ready);
      expect(result.observed!.name, title);
      expect(f.writes.single['name'], title);
      f.service.finishPageRename(result);
      final target = await f.target();
      for (final invalid in [
        '😀' * 513,
        'line\u2028break',
        'line\u2029break',
      ]) {
        await expectLater(
          f.service.renamePage(target, invalid),
          throwsArgumentError,
        );
      }
      expect(f.writes, hasLength(1));
      expect(f.pending, isNull);
    },
  );

  test(
    'original rename route keeps view identity and emits only GET-observed name',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      final events = <AppFlowyPageNameObservation>[];
      final sub = f.service.pageNameChanges.listen(events.add);
      addTearDown(sub.cancel);
      final target = await f.target(),
          result = await f.service.renamePage(await f.target(), ' 原标题😀 ');
      expect(result.status, AppFlowyPageRenameStatus.ready);
      expect(result.target.page.viewId, target.page.viewId);
      expect(result.target.page.name, 'Parent document');
      expect(result.observed!.name, '原标题😀');
      expect(f.writes, [
        {'name': '原标题😀'},
      ]);
      await Future<void>.delayed(Duration.zero);
      expect(events, isNotEmpty);
      expect(events.every((e) => e.page.viewId == documentParent), isTrue);
      expect(events.last.page.name, '原标题😀');
      expect(f.service.isCurrentPageNameObservation(events.last), isTrue);
      f.service.invalidateAccountOperations();
      expect(f.service.isCurrentPageNameObservation(events.last), isFalse);
    },
  );
  for (final kind in ['renamed', 'moved', 'Guest']) {
    test('preflight $kind never sends original rename POST', () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      final target = await f.target();
      if (kind == 'renamed') f.base.parentName = 'Peer title';
      if (kind == 'moved') f.base.movedParent = true;
      if (kind == 'Guest') f.base.role = 'Guest';
      await expectLater(
        f.service.renamePage(target, 'Desired'),
        throwsA(blocked(kind == 'Guest' ? 'READ_ONLY' : 'TARGET_CHANGED')),
      );
      expect(f.writes, isEmpty);
      expect(f.pending, isNull);
    });
  }
  test(
    'foreign service target and invalid names cannot reserve an operation',
    () async {
      final a = await RenameFixture.create(), b = await RenameFixture.create();
      addTearDown(a.close);
      addTearDown(b.close);
      final target = await a.target();
      await expectLater(
        b.service.renamePage(target, 'Wrong source'),
        throwsA(blocked('SOURCE_ENDED')),
      );
      for (final name in ['', 'x' * 513, 'bad\nname']) {
        await expectLater(
          a.service.renamePage(target, name),
          throwsArgumentError,
        );
      }
      expect(a.pending, isNull);
      expect(a.writes, isEmpty);
      expect(b.writes, isEmpty);
    },
  );
  test(
    'lost ACK retains desired/original names, GET reconciles without retry',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      f.breakAck = true;
      final target = await f.target(),
          result = await f.service.renamePage(await f.target(), 'Desired');
      expect(result.status, AppFlowyPageRenameStatus.unconfirmed);
      expect(result.target.page.name, 'Parent document');
      expect(result.name, 'Desired');
      await expectLater(
        f.service.renamePage(target, 'Other'),
        throwsA(blocked('REVIEW_REQUIRED')),
      );
      final checked = await f.service.reconcilePageRename(result);
      expect(checked.status, AppFlowyPageRenameStatus.ready);
      expect(f.writes, hasLength(1));
      f.service.finishPageRename(checked);
      expect(f.pending, isNull);
    },
  );
  test(
    'peer rename after accepted command stays visible and unconfirmed, never overwritten',
    () async {
      final f = await RenameFixture.create();
      addTearDown(f.close);
      f.beforeAck = () async => f.base.parentName = 'New peer title';
      final result = await f.service.renamePage(await f.target(), 'Desired');
      expect(result.serverReturnedSuccess, isTrue);
      expect(result.status, AppFlowyPageRenameStatus.unconfirmed);
      expect(result.observed!.name, 'New peer title');
      expect(result.name, 'Desired');
      expect(
        () => f.service.finishPageRename(result),
        throwsA(blocked('REVIEW_REQUIRED')),
      );
      f.service.finishPageRename(result, reviewed: true);
      expect(f.writes, hasLength(1));
      f.beforeAck = null;
      final next = await f.service.renamePage(await f.target(), 'Next');
      expect(
        () => f.service.finishPageRename(result, reviewed: true),
        throwsA(blocked('SOURCE_ENDED')),
      );
      expect(f.pending!.operationId, next.operationId);
    },
  );
  for (final sent in [false, true]) {
    test(
      'retiring ${sent ? 'sent request' : 'preflight'} preserves delivery boundary',
      () async {
        final f = await RenameFixture.create();
        addTearDown(f.close);
        final target = await f.target();
        final arrived = Completer<void>(), release = Completer<void>();
        if (sent) {
          f.beforeAck = () async {
            arrived.complete();
            await release.future;
          };
        } else {
          f.guard = (r) async {
            if (!r.uri.path.endsWith('/folder')) return false;
            arrived.complete();
            await release.future;
            await f.base.reply(r, f.base.folder());
            return true;
          };
        }
        final operation = f.service.renamePage(target, 'Draft');
        final checked = sent
            ? null
            : expectLater(operation, throwsA(blocked('SOURCE_ENDED')));
        await arrived.future;
        expect(f.service.cancelPageRenamePreflight(f.pending!), !sent);
        release.complete();
        if (sent) {
          expect((await operation).status, AppFlowyPageRenameStatus.ready);
        } else {
          await checked;
        }
        expect(f.writes.length, sent ? 1 : 0);
        expect(f.pending != null, sent);
      },
    );
  }
  for (final sent in [false, true]) {
    test(
      'account A-B-A during ${sent ? 'POST' : 'GET'} cannot publish old metadata',
      () async {
        final f = await RenameFixture.create();
        addTearDown(f.close);
        final target = await f.target();
        final arrived = Completer<void>(), release = Completer<void>();
        if (sent) {
          f.beforeAck = () async {
            arrived.complete();
            await release.future;
          };
        } else {
          f.guard = (r) async {
            if (!r.uri.path.endsWith('/folder')) return false;
            arrived.complete();
            await release.future;
            await f.base.reply(r, f.base.folder());
            return true;
          };
        }
        final operation = f.service.renamePage(target, 'Old account');
        final checked = expectLater(operation, throwsA(isA<Exception>()));
        final events = <AppFlowyPageNameObservation>[];
        final sub = f.service.pageNameChanges.listen(events.add);
        addTearDown(sub.cancel);
        await arrived.future;
        f.base.subject = 'bob';
        f.service.invalidateAccountOperations();
        f.base.subject = 'alice';
        release.complete();
        await checked;
        expect(f.pending, isNull);
        expect(events.where(f.service.isCurrentPageNameObservation), isEmpty);
        expect(events.any((e) => e.page.name == 'Old account'), isFalse);
        expect(f.writes.length, sent ? 1 : 0);
      },
    );
  }
  test('timeout/late ACK never resend or revive a finished rename', () async {
    final f = await RenameFixture.create(
      timeout: const Duration(milliseconds: 100),
    );
    addTearDown(f.close);
    final release = Completer<void>();
    f.beforeAck = () => release.future;
    final result = await f.service.renamePage(
      await f.target(),
      'Timeout draft',
    );
    expect(result.status, AppFlowyPageRenameStatus.unconfirmed);
    final checked = await f.service.reconcilePageRename(result);
    expect(checked.status, AppFlowyPageRenameStatus.ready);
    f.service.finishPageRename(checked);
    release.complete();
    await Future<void>.delayed(const Duration(milliseconds: 30));
    expect(f.pending, isNull);
    expect(f.writes, hasLength(1));
  });
}
