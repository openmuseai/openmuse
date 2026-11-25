import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'access_dialog_test_support.dart';
import 'workspace_access_dialog_test.dart' show mountAccess, finishAccess;

const invitationId = '40000000-0000-0000-0000-000000000001';
Map<String, Object?> invitation() => {
  'invite_id': invitationId,
  'workspace_id': accessWorkspace,
  'invitee_email': accessEmail,
  'role_id': 2,
  'status': 0,
  'created_at': '2026-11-10T02:00:00Z',
  'updated_at': '2026-11-10T02:00:00.123456Z',
  'member_role_id': null,
};
Future<void> settled(WidgetTester t, AccessDialogFixture f) async {
  for (var i = 0; i < 180; i++) {
    await t.pump(const Duration(milliseconds: 20));
    await t.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 3)),
    );
    if (i > 8 &&
        !f.c.busy &&
        find.byKey(const ValueKey('cloud-sent.busy')).evaluate().isEmpty)
      return;
  }
  fail('Original Owner invitation UI did not settle');
}

Future<AccessDialogFixture> fixture(WidgetTester t) async {
  final f = (await t.runAsync(() => AccessDialogFixture.create(page: false)))!;
  f.invitationManagement = true;
  f.memberRole = 'Guest';
  f.sentInvitations.add(invitation());
  return f;
}

void main() {
  for (final width in [320.0, 430.0, 1100.0]) {
    testWidgets(
      'original Owner Pending withdrawal and stale callback at compact width$width',
      (t) async {
        t.view.physicalSize = Size(width, 900);
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final oldHttp = HttpOverrides.current;
        HttpOverrides.global = null;
        final f = await fixture(t);
        addTearDown(() async {
          await t.runAsync(f.close);
          HttpOverrides.global = oldHttp;
        });
        try {
          await mountAccess(t, f, compact: true);
          await settled(t, f);
          final action = find.byKey(
            const ValueKey('cloud-sent.cancel.$invitationId'),
          );
          await t.ensureVisible(action);
          await settled(t, f);
          expect(t.getSize(action).height, greaterThanOrEqualTo(44));
          final callback = t.widget<TextButton>(action).onPressed!;
          await t.tap(action);
          await settled(t, f);
          expect(f.mutations, hasLength(1));
          expect(f.mutations.single.method, 'DELETE');
          expect(f.sentInvitations.single['status'], 2);
          expect(action, findsNothing);
          expect(
            find.byKey(const ValueKey('cloud-sent.notice')),
            findsOneWidget,
          );
          callback();
          await settled(t, f);
          expect(f.mutations, hasLength(1));
          expect(f.c.canManageMembers, isTrue);
          expect(t.takeException(), isNull);
        } finally {
          await finishAccess(t, f);
        }
      },
    );
  }
  testWidgets(
    'accepted original invitation is shown as accepted without a false membership withdrawal',
    (t) async {
      final oldHttp = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = await fixture(t);
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = oldHttp;
      });
      try {
        await mountAccess(t, f);
        await settled(t, f);
        f.sentInvitations[0] = {
          ...f.sentInvitations[0],
          'status': 1,
          'member_role_id': 3,
        };
        final action = find.byKey(
          const ValueKey('cloud-sent.cancel.$invitationId'),
        );
        await t.ensureVisible(action);
        await settled(t, f);
        await t.tap(action);
        await settled(t, f);
        expect(f.sentInvitations.single['member_role_id'], 3);
        expect(action, findsNothing);
        expect(
          find.textContaining('Membership was not removed'),
          findsOneWidget,
        );
        expect(f.mutations, hasLength(1));
        expect(t.takeException(), isNull);
      } finally {
        await finishAccess(t, f);
      }
    },
  );
  testWidgets(
    'old server unsupported owner list preserves the original member form',
    (t) async {
      final oldHttp = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = await fixture(t);
      f.invitationReadStatus = 404;
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = oldHttp;
      });
      try {
        await mountAccess(t, f);
        await settled(t, f);
        expect(find.byKey(const ValueKey('cloud-sent.panel')), findsNothing);
        expect(
          find.byKey(const ValueKey('cloud-access.email')),
          findsOneWidget,
        );
        expect(f.c.canManageMembers, isTrue);
        expect(f.mutations, isEmpty);
        expect(t.takeException(), isNull);
      } finally {
        await finishAccess(t, f);
      }
    },
  );
  testWidgets(
    'losing Owner clears invitation rows and fences an earlier withdrawal callback',
    (t) async {
      final oldHttp = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = await fixture(t);
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = oldHttp;
      });
      try {
        await mountAccess(t, f);
        await settled(t, f);
        final action = find.byKey(
          const ValueKey('cloud-sent.cancel.$invitationId'),
        );
        await t.ensureVisible(action);
        await settled(t, f);
        final earlier = t.widget<TextButton>(action).onPressed!;
        f.role = 'Guest';
        final task = f.c.refresh();
        await settled(t, f);
        await task;
        expect(
          find.byKey(const ValueKey('cloud-sent.row.$invitationId')),
          findsNothing,
        );
        earlier();
        await settled(t, f);
        expect(f.mutations, isEmpty);
        expect(f.c.canManageMembers, isFalse);
        expect(t.takeException(), isNull);
      } finally {
        await finishAccess(t, f);
      }
    },
  );
  testWidgets(
    'original member removal refreshes the existing invitation membership fact without a read loop',
    (t) async {
      final oldHttp = HttpOverrides.current;
      HttpOverrides.global = null;
      final f = await fixture(t);
      f.memberRole = 'Member';
      f.sentInvitations[0] = {
        ...f.sentInvitations[0],
        'status': 1,
        'member_role_id': 2,
      };
      addTearDown(() async {
        await t.runAsync(f.close);
        HttpOverrides.global = oldHttp;
      });
      try {
        await mountAccess(t, f);
        await settled(t, f);
        final reads = f.reads.where((p) => p.endsWith('/invite')).length;
        final operation = f.c.removeMember(accessEmail);
        await settled(t, f);
        await operation;
        await settled(t, f);
        expect(f.reads.where((p) => p.endsWith('/invite')).length, reads + 1);
        final fact = find.byKey(
          const ValueKey('cloud-sent.member.$invitationId'),
        );
        expect(t.widget<Text>(fact).data, 'Not currently a member');
        expect(t.takeException(), isNull);
      } finally {
        await finishAccess(t, f);
      }
    },
  );
}
