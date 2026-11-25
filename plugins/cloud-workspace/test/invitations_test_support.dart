import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';

const invitationId = '10000000-0000-0000-0000-000000000001';
const invitationWorkspace = '20000000-0000-0000-0000-000000000001';

class InvitationFixture {
  late final HttpServer server;
  final auth = InvitationTestAuth();
  late final AppFlowyCloudWorkspaceService service;
  late final WorkspaceInvitationsController c;
  final seen = <String>[];
  int posts = 0, joined = 0;
  String status = 'Pending', workspace = invitationWorkspace;
  Future<bool> Function(HttpRequest)? intercept;
  Map<String, Object?> get data => {
    'invite_id': invitationId,
    'workspace_id': workspace,
    'workspace_name': '原群组名称与较长文字不会扩大权限或溢出界面',
    'inviter_name': '原工作区所有者',
    'status': status,
    'updated_at': '2026-11-10T02:00:00Z',
  };
  static Future<InvitationFixture> create() async {
    final f = InvitationFixture();
    f.server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    f.service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${f.server.port}'),
      accessToken: () async => 'test',
      accountSubject: () => f.auth.snapshot.identity?.subject,
      deviceId: 'invitation-fixture',
      allowHttpForTesting: true,
    );
    f.c = WorkspaceInvitationsController(
      authentication: f.auth,
      service: f.service,
      onJoined: () async {
        f.joined++;
      },
    );
    unawaited(
      f.server.forEach((r) async {
        f.seen.add('${r.method} ${r.uri.path}');
        if (r.method == 'POST') f.posts++;
        if (await f.intercept?.call(r) ?? false) return;
        if (r.method == 'POST') f.status = 'Accepted';
        await f.reply(
          r,
          r.method == 'POST'
              ? null
              : r.uri.path.endsWith('/invite')
              ? [f.data]
              : f.data,
        );
      }),
    );
    return f;
  }

  Future<void> reply(HttpRequest r, Object? data) async {
    r.response.headers.contentType = ContentType.json;
    r.response.write(jsonEncode({'code': 0, 'data': data}));
    await r.response.close();
  }

  Future<void> close() async {
    c.dispose();
    service.closeClient();
    auth.dispose();
    await server.close(force: true);
  }
}

class InvitationTestAuth extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot snapshot =
      const OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.authenticated,
        identity: OpenMuseAuthenticatedIdentity(
          subject: 'alice',
          email: 'alice@example.com',
        ),
      );
  void switchTo(String subject) {
    snapshot = OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.authenticated,
      identity: OpenMuseAuthenticatedIdentity(
        subject: subject,
        email: '$subject@example.com',
      ),
    );
    notifyListeners();
  }

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => 'test';
  @override
  Future<void> restore() async {}
  @override
  Future<void> signInWithPassword(String email, String password) async {}
  @override
  Future<void> signOut() async {}
}
