import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:test/test.dart';

const workspace = '10000000-0000-0000-0000-000000000001';
const view = '20000000-0000-0000-0000-000000000001';
const email = 'guest@example.test';

class Fixture {
  late HttpServer server;
  late AppFlowyCloudWorkspaceService service;
  String subject = 'alice';
  int writeStatus = 503;
  bool malformed = false;
  Completer<void>? release;
  Future<String?> Function()? token;
  final writes = <Map<String, Object?>>[];
  final members = <String, String>{'owner@example.test': 'Owner'};
  final grants = <String, int>{email: 10};
  static Future<Fixture> open({Future<String?> Function()? refresh}) async {
    final f = Fixture();
    f.server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    f.service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${f.server.port}'),
      accessToken: () => f.token?.call() ?? Future.value('synthetic-token'),
      accountSubject: () => f.subject,
      refreshAccessToken: refresh,
      deviceId: 'bounded-access-outcome',
      allowHttpForTesting: true,
    );
    unawaited(
      f.server.forEach((r) async {
        final mutation =
            r.method != 'GET' && !r.uri.path.endsWith('/access-details');
        Object? data;
        if (mutation) {
          final raw = await utf8.decoder.bind(r).join();
          f.writes.add({
            'method': r.method,
            'path': r.uri.path,
            'body': raw.isEmpty ? null : jsonDecode(raw),
          });
          if (f.release != null) await f.release!.future;
          r.response.statusCode = f.writeStatus;
        } else if (r.uri.path == '/api/workspace') {
          data = [
            {'workspace_id': workspace, 'role': 'Owner'},
          ];
        } else if (r.uri.path.endsWith('/member')) {
          data = [
            for (final m in f.members.entries)
              {'email': m.key, 'role': m.value},
          ];
        } else {
          final id = r.uri.pathSegments[r.uri.pathSegments.length - 2];
          data = {
            'view_id': id,
            'shared_with': [
              for (final g in f.grants.entries)
                {
                  'view_id': id,
                  'email': g.key,
                  'role': 'Guest',
                  'name': '',
                  'access_level': g.value,
                  'pending_invitation': false,
                },
            ],
          };
        }
        r.response.headers.contentType = ContentType.json;
        r.response.write(
          mutation && f.malformed ? '{' : jsonEncode({'code': 0, 'data': data}),
        );
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

  List<AppFlowyAccessMutation> get pending =>
      service.pendingAccessMutations(workspace, viewId: view);
  Future<void> share([
    String target = email,
    AppFlowyAccessLevel access = AppFlowyAccessLevel.edit,
  ]) => service.shareView(workspace, view, [target], access);
  Future<void> untilWrite() async {
    for (var i = 0; i < 100; i++) {
      if (writes.isNotEmpty) return;
      await Future<void>.delayed(const Duration(milliseconds: 2));
    }
    fail('bounded original access request did not dispatch');
  }
}

void main() {
  test(
    'unknown target remains fenced across reads, case changes, and action changes until current state is observed',
    () async {
      final f = await Fixture.open();
      await expectLater(f.share(), throwsA(anything));
      expect(f.pending.single.status, AppFlowyAccessMutationStatus.unconfirmed);
      await expectLater(
        f.share(' GUEST@example.test '),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      await expectLater(
        f.service.revokeViewAccess(workspace, view, [email]),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      await f.service.pageAccess(workspace, view);
      expect(f.pending, hasLength(1));
      expect(f.writes, hasLength(1));
      f.grants[email] = 30;
      await f.service.pageAccess(workspace, view);
      expect(f.pending, isEmpty);
      expect(f.writes, hasLength(1));
    },
  );
  test(
    'an unrelated target remains usable while one original request is unknown',
    () async {
      final f = await Fixture.open();
      await expectLater(f.share(), throwsA(anything));
      f.writeStatus = 200;
      await f.share('other@example.test');
      expect(f.writes, hasLength(2));
      expect(f.pending, hasLength(2));
      expect(f.pending.first.status, AppFlowyAccessMutationStatus.unconfirmed);
      expect(f.pending.last.status, AppFlowyAccessMutationStatus.accepted);
    },
  );
  test(
    'in-flight intent cannot be evicted by an overlapping request or an early read',
    () async {
      final f = await Fixture.open();
      f.release = Completer<void>();
      final task = f.share();
      final result = expectLater(task, throwsA(anything));
      await f.untilWrite();
      f.grants[email] = 30;
      await f.service.pageAccess(workspace, view);
      expect(f.pending.single.status, AppFlowyAccessMutationStatus.sending);
      await expectLater(
        f.share(),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      expect(f.writes, hasLength(1));
      f.release!.complete();
      await result;
    },
  );
  test(
    'an accepted pending invitation is not disproved by absence from the member list',
    () async {
      final f = await Fixture.open();
      f.writeStatus = 200;
      await f.service.inviteWorkspaceMembers(
        workspace,
        [email],
        role: AppFlowyWorkspaceRole.guest,
        skipEmailSend: true,
      );
      await f.service.workspaceMembers(workspace);
      expect(f.service.pendingAccessMutations(workspace), hasLength(1));
      await expectLater(
        f.service.inviteWorkspaceMembers(
          workspace,
          [email],
          role: AppFlowyWorkspaceRole.guest,
          skipEmailSend: true,
        ),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      expect(f.writes, hasLength(1));
      expect(
        ((f.writes.single['body'] as List).single as Map)['skip_email_send'],
        isTrue,
      );
      f.members[email] = 'Guest';
      await f.service.workspaceMembers(workspace);
      expect(f.service.pendingAccessMutations(workspace), isEmpty);
    },
  );
  test(
    'pre-dispatch token failure and explicit original authorization rejection do not become unknown writes',
    () async {
      final f = await Fixture.open();
      f.token = () async => null;
      await expectLater(f.share(), throwsA(anything));
      expect(f.writes, isEmpty);
      expect(f.pending, isEmpty);
      f.token = null;
      f.writeStatus = 403;
      await expectLater(f.share(), throwsA(anything));
      expect(f.writes, hasLength(1));
      expect(f.pending, isEmpty);
      f.writeStatus = 200;
      await f.share();
      expect(f.writes, hasLength(2));
    },
  );
  test(
    'A-B-A retirement discards a delayed accepted reply and never repopulates an old intent',
    () async {
      final f = await Fixture.open();
      f.writeStatus = 200;
      f.release = Completer<void>();
      final task = f.share();
      final result = expectLater(task, throwsA(anything));
      await f.untilWrite();
      f.subject = 'bob';
      f.service.invalidateAccountOperations();
      f.subject = 'alice';
      f.service.invalidateAccountOperations();
      f.release!.complete();
      await result;
      expect(f.pending, isEmpty);
      expect(f.writes, hasLength(1));
    },
  );
  test(
    'malformed response after dispatch is unknown and cannot be replayed',
    () async {
      final f = await Fixture.open();
      f.writeStatus = 200;
      f.malformed = true;
      await expectLater(f.share(), throwsA(anything));
      expect(f.pending.single.status, AppFlowyAccessMutationStatus.unconfirmed);
      await expectLater(
        f.share(),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      expect(f.writes, hasLength(1));
    },
  );
  test(
    'unknown intent budget is bounded without dropping older unresolved targets',
    () async {
      final f = await Fixture.open();
      for (var i = 0; i < 128; i++) {
        await expectLater(f.share('guest$i@example.test'), throwsA(anything));
      }
      expect(f.pending, hasLength(128));
      await expectLater(
        f.share('extra@example.test'),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      expect(f.writes, hasLength(128));
      for (var i = 0; i < 128; i++) {
        f.grants['guest$i@example.test'] = 30;
      }
      await f.service.pageAccess(workspace, view);
      expect(f.pending, isEmpty);
    },
  );
  test(
    'an original401 followed by refresh failure is a rejected delivery, not an unknown mutation',
    () async {
      final f = await Fixture.open(
        refresh: () async => throw StateError('synthetic refresh failure'),
      );
      f.writeStatus = 401;
      await expectLater(f.share(), throwsA(anything));
      expect(f.writes, hasLength(1));
      expect(f.pending, isEmpty);
    },
  );
  test(
    'accepted share is reconciled by a current read even when another change supersedes its value',
    () async {
      final f = await Fixture.open();
      f.writeStatus = 200;
      await f.share();
      expect(f.pending.single.status, AppFlowyAccessMutationStatus.accepted);
      f.grants[email] = 10;
      await f.service.pageAccess(workspace, view);
      expect(f.pending, isEmpty);
    },
  );
  test(
    'accepted role operation does not become stuck when the next member read shows a later removal',
    () async {
      final f = await Fixture.open();
      f.writeStatus = 200;
      await f.service.updateWorkspaceMember(
        workspace,
        email,
        AppFlowyWorkspaceRole.member,
      );
      expect(f.service.pendingAccessMutations(workspace), hasLength(1));
      await f.service.workspaceMembers(workspace);
      expect(f.service.pendingAccessMutations(workspace), isEmpty);
    },
  );
}
