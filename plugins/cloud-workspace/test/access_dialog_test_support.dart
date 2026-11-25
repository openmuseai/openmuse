import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'invitations_test_support.dart' show InvitationTestAuth;

const accessWorkspace = '10000000-0000-0000-0000-000000000001';
const accessView = '20000000-0000-0000-0000-000000000001';
const accessParent = '30000000-0000-0000-0000-000000000001';
const accessEmail = 'long.member.address.for.original.workspace@example.test';

class AccessDialogFixture {
  final auth = InvitationTestAuth();
  late final HttpServer server;
  late final AppFlowyCloudWorkspaceService service;
  late final WorkspaceAccessController c;
  final mutations = <({String method, String path, Object? body})>[];
  final reads = <String>[];
  String role = 'Owner', memberRole = 'Member';
  int level = 10;
  bool inherited = false, applyWrites = true;
  int mutationStatus = 200;
  bool invitationManagement = false;
  bool inviteCodes = false, codeJoined = false;
  String? inviteCode;
  int codeGenerations = 0;
  int invitationReadStatus = 200;
  final sentInvitations = <Map<String, Object?>>[];
  static Future<AccessDialogFixture> create({
    required bool page,
    String role = 'Owner',
    bool inherited = false,
  }) async {
    final f = AccessDialogFixture()
      ..role = role
      ..inherited = inherited;
    f.server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    f.service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${f.server.port}'),
      accessToken: () async => 'test',
      accountSubject: () => f.auth.snapshot.identity?.subject,
      deviceId: 'original-access-ui',
      allowHttpForTesting: true,
    );
    f.c = WorkspaceAccessController(
      authentication: f.auth,
      service: f.service,
      workspaceId: accessWorkspace,
      viewId: page ? accessView : null,
      ancestors: page ? [accessParent] : const [],
    );
    unawaited(
      f.server.forEach((r) async {
        Object? data;
        if (r.method != 'GET' && !r.uri.path.endsWith('/access-details')) {
          final text = await utf8.decoder.bind(r).join();
          final body = text.isEmpty ? null : jsonDecode(text);
          f.mutations.add((method: r.method, path: r.uri.path, body: body));
          r.response.statusCode = f.mutationStatus;
          if (f.inviteCodes && r.uri.path.endsWith('/invite-code')) {
            if (f.applyWrites && r.method == 'POST') {
              f.codeGenerations++;
              f.inviteCode = List.filled(
                16,
                String.fromCharCode(65 + f.codeGenerations),
              ).join();
            }
            if (f.applyWrites && r.method == 'DELETE') f.inviteCode = null;
            if (r.method == 'POST') data = {'code': f.inviteCode};
          }
          if (f.inviteCodes && r.uri.path.endsWith('/join-by-invite-code')) {
            if (f.applyWrites) f.codeJoined = true;
            data = {'workspace_id': accessWorkspace};
          }
          if (f.invitationManagement &&
              r.method == 'DELETE' &&
              r.uri.path.contains('/invite/')) {
            final index = f.sentInvitations.indexWhere(
              (row) => row['invite_id'] == r.uri.pathSegments.last,
            );
            if (index >= 0) {
              var row = f.sentInvitations[index];
              final outcome = row['status'] == 0
                  ? 'cancelled'
                  : row['status'] == 1
                  ? 'accepted'
                  : 'ended';
              if (f.applyWrites && row['status'] == 0) {
                row = {...row, 'status': 2};
                f.sentInvitations[index] = row;
              }
              data = {
                'profile': 'appflowy.workspace-invitation-management/1',
                'outcome': outcome,
                'invitation': row,
              };
            }
          }
          if (f.applyWrites &&
              r.method == 'DELETE' &&
              r.uri.path.endsWith('/member') &&
              body is List) {
            f.memberRole = 'Guest';
            for (var i = 0; i < f.sentInvitations.length; i++) {
              if (body.contains(f.sentInvitations[i]['invitee_email']))
                f.sentInvitations[i] = {
                  ...f.sentInvitations[i],
                  'member_role_id': null,
                };
            }
          }
          if (f.applyWrites && r.uri.path.endsWith('/member') && body is Map)
            f.memberRole = body['role'] as String;
          if (f.applyWrites && body is Map && body['access_level'] is int)
            f.level = body['access_level'] as int;
        } else {
          f.reads.add(r.uri.path);
          if (f.inviteCodes && r.uri.path.endsWith('/invite-code')) {
            data = {'code': f.inviteCode};
          } else if (f.inviteCodes && r.uri.path == '/api/invite-code-info') {
            data = {
              'workspace_id': accessWorkspace,
              'workspace_name': 'Original invited workspace',
              'owner_name': 'Original Owner',
              'member_count': 2,
              'is_member': f.codeJoined,
            };
          } else if (f.invitationManagement && r.uri.path.endsWith('/invite')) {
            r.response.statusCode = f.invitationReadStatus;
            final filter = r.uri.queryParameters['email'];
            data = {
              'profile': 'appflowy.workspace-invitation-management/1',
              'workspace_id': accessWorkspace,
              'items': f.sentInvitations
                  .where(
                    (row) => filter == null || row['invitee_email'] == filter,
                  )
                  .toList(),
              'has_more': false,
              'next_cursor': null,
            };
          } else if (r.uri.path == '/api/workspace')
            data = [
              {'workspace_id': accessWorkspace, 'role': f.role},
            ];
          else if (r.uri.path.endsWith('/member'))
            data = [
              {'email': 'owner@example.test', 'role': 'Owner'},
              if (f.memberRole != 'Guest')
                {'email': accessEmail, 'role': f.memberRole},
            ];
          else if (r.uri.path.endsWith('/access-details')) {
            final id = r.uri.pathSegments[r.uri.pathSegments.length - 2];
            data = {
              'view_id': id,
              'shared_with': [
                {
                  'view_id': f.inherited && id == accessView
                      ? accessParent
                      : id,
                  'email': accessEmail,
                  'name': 'Original guest',
                  'role': 'Guest',
                  'access_level': f.level,
                  'pending_invitation': false,
                  if (f.inherited) 'source_view_name': '原上级页面以及较长的权限继承来源名称',
                },
              ],
            };
          }
        }
        r.response.headers.contentType = ContentType.json;
        r.response.write(jsonEncode({'code': 0, 'data': data}));
        await r.response.close();
      }),
    );
    return f;
  }

  Future<void> close() async {
    c.dispose();
    service.closeClient();
    auth.dispose();
    await server.close(force: true);
  }
}
