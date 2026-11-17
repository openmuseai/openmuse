import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

const wid = '10000000-0000-0000-0000-000000000001';
const vid = '20000000-0000-0000-0000-000000000001';
void main() {
  Future<({WorkspaceAccessController c, _Auth auth, List<String> seen})>
  fixture({
    String role = 'Owner',
    bool inherited = false,
    Future<void> Function(HttpRequest)? onRequest,
  }) async {
    final auth = _Auth();
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final seen = <String>[];
    final service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
      accessToken: () async => 'test',
      accountSubject: () => auth.snapshot.identity?.subject,
      deviceId: 'test',
      allowHttpForTesting: true,
    );
    final c = WorkspaceAccessController(
      authentication: auth,
      service: service,
      workspaceId: wid,
      viewId: vid,
      ancestors: [wid],
    );
    addTearDown(() async {
      c.dispose();
      service.closeClient();
      auth.dispose();
      await server.close(force: true);
    });
    unawaited(
      server.forEach((request) async {
        seen.add('${request.method} ${request.uri.path}');
        if (onRequest != null) {
          await onRequest(request);
          return;
        }
        Object? data;
        if (request.uri.path == '/api/workspace') {
          data = [
            {'workspace_id': wid, 'role': role},
          ];
        } else if (request.uri.path.endsWith('/member')) {
          data = [
            {'email': 'owner@example.com', 'role': 'Owner'},
          ];
        } else if (request.uri.path.endsWith('/access-details')) {
          data = {
            'view_id': vid,
            'shared_with': [
              {
                'view_id': inherited ? wid : vid,
                if (inherited) 'source_view_name': '原上级页面',
                'email': 'guest@example.com',
                'role': 'Guest',
                'access_level': 10,
                'pending_invitation': false,
              },
            ],
          };
        } else {
          request.response.statusCode = 503;
        }
        request.response.headers.contentType = ContentType.json;
        request.response.write(jsonEncode({'code': 0, 'data': data}));
        await request.response.close();
      }),
    );
    return (c: c, auth: auth, seen: seen);
  }

  test(
    'original Guest role cannot manage workspace members or shares',
    () async {
      final f = await fixture(role: 'Guest');
      await f.c.refresh();
      expect(f.c.canManageMembers, isFalse);
      expect(f.c.canShare, isFalse);
      await f.c.share('guest@example.com', AppFlowyAccessLevel.full);
      await f.c.changeRole('guest@example.com', AppFlowyWorkspaceRole.member);
      expect(f.seen, ['GET /api/workspace']);
    },
  );
  test(
    'mutation failure retains only confirmed grants and exposes unconfirmed result',
    () async {
      final f = await fixture();
      await f.c.refresh();
      final confirmed = f.c.people;
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(f.c.failure, 'unconfirmed');
      expect(f.c.people, same(confirmed));
      expect(f.c.people.single.access, AppFlowyAccessLevel.readOnly);
      await f.c.refresh();
      expect(f.c.failure, 'unconfirmed');
    },
  );
  test('inherited grant cannot be revoked through a child page', () async {
    final f = await fixture(inherited: true);
    await f.c.refresh();
    final requests = f.seen.length;
    await f.c.revoke(f.c.people.single);
    expect(f.seen.length, requests);
    expect(f.c.people.single.viewId, wid);
    expect(f.c.people.single.sourceViewName, '原上级页面');
  });
  test(
    'A-B-A retires an access dialog and discards a pending response',
    () async {
      final pending = Completer<HttpRequest>();
      final f = await fixture(
        onRequest: (r) async {
          pending.complete(r);
        },
      );
      final refresh = f.c.refresh();
      final request = await pending.future;
      f.auth.switchTo('bob');
      f.auth.switchTo('alice');
      request.response.headers.contentType = ContentType.json;
      request.response.write(
        jsonEncode({
          'code': 0,
          'data': [
            {'workspace_id': wid, 'role': 'Owner'},
          ],
        }),
      );
      await request.response.close();
      await refresh;
      expect(f.c.retired, isTrue);
      expect(f.c.role, isNull);
      expect(f.c.people, isEmpty);
      expect(f.c.members, isEmpty);
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(f.seen.length, 1);
    },
  );
  test(
    'unconfirmed original share cannot be resent through the same UI intent or a refresh',
    () async {
      final f = await fixture();
      await f.c.refresh();
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(f.c.failure, 'unconfirmed');
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(f.seen.where((v) => v.startsWith('PUT ')), hasLength(1));
      await f.c.refresh();
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(f.seen.where((v) => v.startsWith('PUT ')), hasLength(1));
    },
  );
  test(
    'reopening a sharing controller cannot replay an unconfirmed account service intent',
    () async {
      final f = await fixture();
      await f.c.refresh();
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      f.c.dispose();
      final reopened = WorkspaceAccessController(
        authentication: f.auth,
        service: f.c.service,
        workspaceId: wid,
        viewId: vid,
        ancestors: [wid],
      );
      addTearDown(reopened.dispose);
      await reopened.refresh();
      await reopened.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(f.seen.where((v) => v.startsWith('PUT ')), hasLength(1));
    },
  );
  test(
    'accepted original share followed by a read failure is not shown as an unknown write',
    () async {
      var accepted = false;
      final f = await fixture(
        onRequest: (r) async {
          Object? data;
          if (r.method == 'PUT') {
            await r.drain<void>();
            accepted = true;
          } else if (accepted) {
            r.response.statusCode = 503;
          } else if (r.uri.path == '/api/workspace') {
            data = [
              {'workspace_id': wid, 'role': 'Owner'},
            ];
          } else if (r.uri.path.endsWith('/member')) {
            data = [
              {'email': 'owner@example.com', 'role': 'Owner'},
            ];
          } else {
            data = {'view_id': vid, 'shared_with': []};
          }
          r.response.headers.contentType = ContentType.json;
          r.response.write(jsonEncode({'code': 0, 'data': data}));
          await r.response.close();
        },
      );
      await f.c.refresh();
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(accepted, isTrue);
      expect(f.c.failure, 'refresh_unavailable');
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(f.seen.where((v) => v.startsWith('PUT ')), hasLength(1));
    },
  );
  test(
    'a definitive loss of original workspace read access clears cached members and grants before another write',
    () async {
      var denied = false;
      final f = await fixture(
        onRequest: (r) async {
          Object? data;
          if (denied) {
            r.response.statusCode = 403;
          } else if (r.uri.path == '/api/workspace') {
            data = [
              {'workspace_id': wid, 'role': 'Owner'},
            ];
          } else if (r.uri.path.endsWith('/member')) {
            data = [
              {'email': 'owner@example.com', 'role': 'Owner'},
            ];
          } else {
            data = {
              'view_id': vid,
              'shared_with': [
                {
                  'view_id': vid,
                  'email': 'guest@example.com',
                  'name': '',
                  'role': 'Guest',
                  'access_level': 10,
                  'pending_invitation': false,
                },
              ],
            };
          }
          r.response.headers.contentType = ContentType.json;
          r.response.write(jsonEncode({'code': 0, 'data': data}));
          await r.response.close();
        },
      );
      await f.c.refresh();
      expect(f.c.canShare, isTrue);
      expect(f.c.people, isNotEmpty);
      denied = true;
      await f.c.refresh();
      expect(f.c.canShare, isFalse);
      expect(f.c.canManageMembers, isFalse);
      expect(f.c.role, isNull);
      expect(f.c.people, isEmpty);
      expect(f.c.members, isEmpty);
      final before = f.seen.length;
      await f.c.share('guest@example.com', AppFlowyAccessLevel.edit);
      expect(f.seen.length, before);
    },
  );
}

class _Auth extends ChangeNotifier implements OpenMuseAuthenticationController {
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
