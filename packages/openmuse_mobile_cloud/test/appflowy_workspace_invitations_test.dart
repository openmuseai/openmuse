import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:test/test.dart';

const invitationId = '10000000-0000-0000-0000-000000000001';
const workspaceId = '20000000-0000-0000-0000-000000000001';
Map<String, Object?> invitation({String status = 'Pending'}) => {
  'invite_id': invitationId,
  'workspace_id': workspaceId,
  'workspace_name': '原工作区',
  'inviter_name': 'Owner',
  'inviter_email': null,
  'status': status,
  'updated_at': '2026-11-10T02:00:00Z',
};
void main() {
  test(
    'original wire status and nullable display metadata never imply authority',
    () {
      for (final status in [
        'Pending',
        'Accepted',
        'Rejected',
        'FutureStatus',
      ]) {
        final value = AppFlowyWorkspaceInvitation.fromJson(
          invitation(status: status),
        );
        expect(value.wireStatus, status);
        expect(value.inviterEmail, isNull);
        expect(value.workspaceId, workspaceId);
        expect(value.updatedAt.isUtc, isTrue);
        expect(
          value.status == AppFlowyWorkspaceInvitationStatus.pending,
          status == 'Pending',
        );
      }
      for (final invalid in [
        {...invitation(), 'invite_id': '../accept-invite'},
        {...invitation(), 'workspace_id': null},
        {...invitation(), 'status': 0},
        {...invitation(), 'updated_at': 'invalid'},
      ]) {
        expect(
          () => AppFlowyWorkspaceInvitation.fromJson(invalid),
          throwsA(anything),
        );
      }
    },
  );
  for (final mode in [
    'original',
    'duplicate',
    'wrong-id',
    'unknown',
    'late-account',
    'lost-post',
  ]) {
    test('original invitation HTTP contract: $mode', () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final seen = <String>[];
      var subject = 'alice';
      final service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'test',
        accountSubject: () => subject,
        deviceId: 'invitation-contract',
        allowHttpForTesting: true,
      );
      addTearDown(() async {
        service.closeClient();
        await server.close(force: true);
      });
      unawaited(
        server.forEach((r) async {
          seen.add('${r.method} ${r.uri.path}');
          expect(r.uri.hasQuery, isFalse);
          Object? data;
          if (r.method == 'POST') {
            if (mode == 'lost-post') {
              final socket = await r.response.detachSocket();
              socket.destroy();
              return;
            }
          } else if (r.uri.path.endsWith('/invite')) {
            data = [
              invitation(
                status: mode == 'unknown' ? 'FutureStatus' : 'Pending',
              ),
            ];
            if (mode == 'duplicate') (data as List).add(invitation());
          } else {
            data = {
              ...invitation(),
              if (mode == 'wrong-id') 'invite_id': workspaceId,
            };
          }
          if (mode == 'late-account') subject = 'bob';
          r.response.headers.contentType = ContentType.json;
          r.response.write(jsonEncode({'code': 0, 'data': data}));
          await r.response.close();
        }),
      );
      if (mode == 'duplicate' || mode == 'late-account') {
        await expectLater(service.workspaceInvitations(), throwsA(anything));
      } else if (mode == 'wrong-id') {
        await expectLater(
          service.workspaceInvitation(invitationId),
          throwsA(anything),
        );
      } else if (mode == 'lost-post') {
        await expectLater(
          service.acceptWorkspaceInvitation(invitationId),
          throwsA(anything),
        );
        expect(seen, ['POST /api/workspace/accept-invite/$invitationId']);
      } else {
        final values = await service.workspaceInvitations();
        if (mode == 'unknown') {
          expect(
            values.single.status,
            AppFlowyWorkspaceInvitationStatus.unsupported,
          );
        } else {
          expect(
            (await service.workspaceInvitation(invitationId)).id,
            invitationId,
          );
          await service.acceptWorkspaceInvitation(invitationId);
          expect(seen, [
            'GET /api/workspace/invite',
            'GET /api/workspace/invite/$invitationId',
            'POST /api/workspace/accept-invite/$invitationId',
          ]);
        }
      }
    });
  }
  test(
    'invitation deadline retires a delayed token before it can dispatch',
    () async {
      final token = Completer<String?>();
      var writes = 0;
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () => token.future,
        deviceId: 'deadline',
        allowHttpForTesting: true,
      );
      addTearDown(() async {
        service.closeClient();
        await server.close(force: true);
      });
      unawaited(
        server.forEach((r) async {
          writes++;
          await r.response.close();
        }),
      );
      await expectLater(
        service.acceptWorkspaceInvitation(
          invitationId,
          timeout: const Duration(milliseconds: 20),
        ),
        throwsA(isA<TimeoutException>()),
      );
      token.complete('test');
      await Future<void>.delayed(const Duration(milliseconds: 50));
      expect(writes, 0);
    },
  );
}
