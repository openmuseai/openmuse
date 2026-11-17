import 'dart:async';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'invitations_test_support.dart';

void main() {
  Future<InvitationFixture> fixture() async {
    final f = await InvitationFixture.create();
    addTearDown(f.close);
    return f;
  }

  test(
    'pending acceptance reads current original state, dispatches once, and refreshes the original catalog after confirmation',
    () async {
      final f = await fixture();
      await f.c.refresh();
      expect(f.c.canAccept(f.c.invitations.single), isTrue);
      await Future.wait([f.c.accept(invitationId), f.c.accept(invitationId)]);
      expect(
        f.c.invitations.single.status,
        AppFlowyWorkspaceInvitationStatus.accepted,
      );
      expect(f.posts, 1);
      expect(f.joined, 1);
      await f.c.accept(invitationId);
      expect(f.posts, 1);
      expect(f.seen, [
        'GET /api/workspace/invite',
        'GET /api/workspace/invite/$invitationId',
        'POST /api/workspace/accept-invite/$invitationId',
        'GET /api/workspace/invite',
      ]);
    },
  );
  test(
    'already accepted current server state refreshes the catalog without resubmitting',
    () async {
      final f = await fixture();
      await f.c.refresh();
      f.status = 'Accepted';
      await f.c.accept(invitationId);
      expect(f.posts, 0);
      expect(f.joined, 1);
      expect(
        f.c.invitations.single.status,
        AppFlowyWorkspaceInvitationStatus.accepted,
      );
    },
  );
  test(
    'unknown original status remains visible without an accept command',
    () async {
      final f = await fixture();
      f.status = 'FutureStatus';
      await f.c.refresh();
      expect(f.c.canAccept(f.c.invitations.single), isFalse);
      await f.c.accept(invitationId);
      expect(f.posts, 0);
      expect(f.seen, hasLength(1));
    },
  );
  for (final applied in [false, true]) {
    test(
      'lost acceptance response is reconciled by reads only, applied=$applied',
      () async {
        final f = await fixture();
        await f.c.refresh();
        f.intercept = (r) async {
          if (r.method != 'POST') return false;
          if (applied) f.status = 'Accepted';
          final socket = await r.response.detachSocket();
          socket.destroy();
          return true;
        };
        await f.c.accept(invitationId);
        expect(f.c.failure, 'unconfirmed');
        expect(f.c.unconfirmedIds, contains(invitationId));
        await f.c.refresh();
        await f.c.accept(invitationId);
        expect(f.posts, 1);
        expect(f.joined, applied ? 1 : 0);
        expect(f.c.failure, applied ? isNull : 'unconfirmed');
        expect(f.c.canAccept(f.c.invitations.single), isFalse);
      },
    );
  }
  test(
    'A-B-A retires the original controller before a delayed current-state read can dispatch acceptance',
    () async {
      final f = await fixture();
      await f.c.refresh();
      final pending = Completer<HttpRequest>();
      f.intercept = (r) async {
        pending.complete(r);
        return true;
      };
      final operation = f.c.accept(invitationId);
      final request = await pending.future;
      f.auth.switchTo('bob');
      f.auth.switchTo('alice');
      await f.reply(request, f.data);
      await operation;
      expect(f.c.retired, isTrue);
      expect(f.c.invitations, isEmpty);
      expect(f.posts, 0);
      expect(f.joined, 0);
      await f.c.refresh();
      expect(f.posts, 0);
    },
  );
  test(
    'closing during an acceptance response does not publish or refresh another presentation',
    () async {
      final f = await fixture();
      await f.c.refresh();
      final pending = Completer<HttpRequest>();
      f.intercept = (r) async {
        if (r.method != 'POST') return false;
        pending.complete(r);
        return true;
      };
      final operation = f.c.accept(invitationId);
      final request = await pending.future;
      f.c.dispose();
      f.status = 'Accepted';
      await f.reply(request, null);
      await operation;
      expect(f.posts, 1);
      expect(f.joined, 0);
      expect(
        f.seen.where((e) => e == 'GET /api/workspace/invite'),
        hasLength(1),
      );
    },
  );
  test(
    'a current invitation bound to a different workspace never dispatches',
    () async {
      final f = await fixture();
      await f.c.refresh();
      f.workspace = '30000000-0000-0000-0000-000000000001';
      await f.c.accept(invitationId);
      expect(f.posts, 0);
      expect(f.c.failure, 'invitation_changed');
    },
  );
}
