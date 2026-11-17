import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:test/test.dart';

const workspace = '10000000-0000-0000-0000-000000000001';
const invite = '20000000-0000-0000-0000-000000000001';
const email = 'guest@example.test';
Map<String, Object?> row({int status = 0, int role = 2, int? member}) => {
  'invite_id': invite,
  'workspace_id': workspace,
  'invitee_email': email,
  'role_id': role,
  'status': status,
  'created_at': '2026-11-10T01:00:00Z',
  'updated_at': '2026-11-10T01:00:00.123456Z',
  'member_role_id': member,
};

class OwnerFixture {
  late HttpServer server;
  late AppFlowyCloudWorkspaceService service;
  String subject = 'alice';
  int readStatus = 200, writeStatus = 200;
  bool wrongProfile = false, more = false, badCursor = false;
  Completer<void>? release;
  List<Map<String, Object?>> rows = [row()];
  final writes = <Map<String, Object?>>[];
  static Future<OwnerFixture> open() async {
    final f = OwnerFixture();
    f.server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    f.service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${f.server.port}'),
      accessToken: () async => 'synthetic-owner-token',
      accountSubject: () => f.subject,
      deviceId: 'owner-invitation-contract',
      allowHttpForTesting: true,
    );
    unawaited(
      f.server.forEach((r) async {
        Object? data;
        if (r.method == 'GET') {
          if (f.release != null) await f.release!.future;
          r.response.statusCode = f.readStatus;
          data = {
            'profile': f.wrongProfile
                ? 'wrong'
                : 'appflowy.workspace-invitation-management/1',
            'workspace_id': workspace,
            'items': f.rows,
            'has_more': f.more,
            'next_cursor': f.more
                ? {
                    'created_at': f.rows.last['created_at'],
                    'invite_id': f.badCursor
                        ? '30000000-0000-0000-0000-000000000001'
                        : f.rows.last['invite_id'],
                  }
                : null,
          };
        } else {
          final raw = await utf8.decoder.bind(r).join();
          f.writes.add({
            'method': r.method,
            'path': r.uri.path,
            'body': raw.isEmpty ? null : jsonDecode(raw),
          });
          r.response.statusCode = f.writeStatus;
          if (r.method == 'DELETE' && f.writeStatus == 200) {
            final current = {...f.rows.first, 'status': 2};
            f.rows = [current];
            data = {
              'profile': 'appflowy.workspace-invitation-management/1',
              'outcome': 'cancelled',
              'invitation': current,
            };
          }
        }
        r.response.headers.contentType = ContentType.json;
        r.response.write(jsonEncode({'code': 0, 'data': data}));
        await r.response.close();
      }),
    );
    addTearDown(() async {
      if (f.release != null && !f.release!.isCompleted) f.release!.complete();
      f.service.closeClient();
      await f.server.close(force: true);
    });
    return f;
  }
}

void main() {
  test(
    'original Owner page preserves raw unsupported role and state without Pending fallback',
    () async {
      final f = await OwnerFixture.open();
      f.rows = [row(status: 99, role: 99, member: 98)];
      final page = await f.service.ownerWorkspaceInvitations(workspace);
      expect(
        page.items.single.status,
        AppFlowyWorkspaceInvitationStatus.unsupported,
      );
      expect(page.items.single.role, isNull);
      expect(page.items.single.memberRole, isNull);
      expect(page.items.single.roleId, 99);
      expect(page.items.single.memberRoleId, 98);
      expect(page.items.single.canCancel, isFalse);
      await expectLater(
        f.service.cancelOwnerWorkspaceInvitation(page.items.single),
        throwsArgumentError,
      );
      expect(f.writes, isEmpty);
    },
  );
  for (final mode in ['profile', 'workspace', 'duplicate', 'cursor']) {
    test('Owner page fails closed on $mode mismatch', () async {
      final f = await OwnerFixture.open();
      if (mode == 'profile') f.wrongProfile = true;
      if (mode == 'workspace')
        f.rows = [
          {...row(), 'workspace_id': '40000000-0000-0000-0000-000000000001'},
        ];
      if (mode == 'duplicate') f.rows = [row(), row()];
      if (mode == 'cursor') {
        f.more = true;
        f.badCursor = true;
      }
      await expectLater(
        f.service.ownerWorkspaceInvitations(workspace),
        throwsA(anything),
      );
      expect(f.writes, isEmpty);
    });
  }
  for (final code in [404, 405]) {
    test(
      'old server$code reports unsupported without a replacement invitation domain',
      () async {
        final f = await OwnerFixture.open();
        f.readStatus = code;
        await expectLater(
          f.service.ownerWorkspaceInvitations(workspace),
          throwsA(isA<AppFlowyInvitationManagementUnsupported>()),
        );
        expect(f.writes, isEmpty);
      },
    );
  }
  test(
    'query bounds fail before HTTP and original observation fields are sent for cancellation',
    () async {
      final f = await OwnerFixture.open();
      for (final limit in [0, 101]) {
        await expectLater(
          f.service.ownerWorkspaceInvitations(workspace, limit: limit),
          throwsArgumentError,
        );
      }
      final page = await f.service.ownerWorkspaceInvitations(
        workspace,
        email: email,
      );
      final outcome = await f.service.cancelOwnerWorkspaceInvitation(
        page.items.single,
      );
      expect(outcome.outcome, 'cancelled');
      expect(outcome.invitation.wireStatus, 2);
      expect(f.writes, hasLength(1));
      expect(f.writes.single['method'], 'DELETE');
      expect(f.writes.single['body'], {
        'expected_updated_at': '2026-11-10T01:00:00.123456Z',
        'expected_role_id': 2,
        'expected_invitee_email': email,
      });
    },
  );
  test(
    'unknown send becomes a current observed Pending fact and known withdrawal releases only that bound invitation',
    () async {
      final f = await OwnerFixture.open();
      f.writeStatus = 503;
      await expectLater(
        f.service.inviteWorkspaceMembers(workspace, [
          email,
        ], skipEmailSend: true),
        throwsA(anything),
      );
      expect(
        f.service.pendingAccessMutations(workspace).single.status,
        AppFlowyAccessMutationStatus.unconfirmed,
      );
      final page = await f.service.ownerWorkspaceInvitations(workspace);
      expect(
        f.service.pendingAccessMutations(workspace).single.status,
        AppFlowyAccessMutationStatus.observed,
      );
      await expectLater(
        f.service.inviteWorkspaceMembers(workspace, [
          email,
        ], skipEmailSend: true),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      f.writeStatus = 200;
      await f.service.cancelOwnerWorkspaceInvitation(page.items.single);
      expect(f.service.pendingAccessMutations(workspace), isEmpty);
      expect(f.writes, hasLength(2));
    },
  );
  test(
    'Owner read confirms accepted Guest without relying on the guest-excluding member list',
    () async {
      final f = await OwnerFixture.open();
      await f.service.inviteWorkspaceMembers(
        workspace,
        [email],
        role: AppFlowyWorkspaceRole.guest,
        skipEmailSend: true,
      );
      f.rows = [row(status: 1, role: 3, member: 3)];
      await f.service.ownerWorkspaceInvitations(workspace);
      expect(f.service.pendingAccessMutations(workspace), isEmpty);
      expect(f.writes, hasLength(1));
    },
  );
  test(
    'account A-B-A discards a delayed Owner read before it can observe any pending target',
    () async {
      final f = await OwnerFixture.open();
      f.release = Completer<void>();
      final read = f.service.ownerWorkspaceInvitations(workspace);
      final expected = expectLater(read, throwsA(anything));
      await Future<void>.delayed(const Duration(milliseconds: 10));
      f.subject = 'bob';
      f.service.invalidateAccountOperations();
      f.subject = 'alice';
      f.service.invalidateAccountOperations();
      f.release!.complete();
      await expected;
      expect(f.service.pendingAccessMutations(workspace), isEmpty);
    },
  );
  test(
    'acknowledged history cannot exhaust the delivery budget for fresh targets',
    () async {
      final f = await OwnerFixture.open();
      for (var i = 0; i < 140; i++) {
        await f.service.inviteWorkspaceMembers(workspace, [
          'recipient$i@example.test',
        ], skipEmailSend: true);
      }
      expect(f.writes, hasLength(140));
      expect(
        f.service.pendingAccessMutations(workspace).length,
        lessThanOrEqualTo(128),
      );
    },
  );
}
