import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:test/test.dart';

const wid = '10000000-0000-0000-0000-000000000001';
const other = '20000000-0000-0000-0000-000000000001';
const firstCode = 'AAAAAAAAAAAAAAAA';
const nextCode = 'BBBBBBBBBBBBBBBB';

class CodeFixture {
  late HttpServer server;
  late AppFlowyCloudWorkspaceService service;
  String subject = 'alice', role = 'Member';
  String? code;
  Object? readOverride, createOverride;
  bool overrideRead = false, dropCreate = false;
  String joinedWorkspace = wid;
  Completer<void>? releaseRead;
  final writes = <Map<String, Object?>>[];
  Map<String, Object?> preview = {
    'workspace_id': wid,
    'workspace_name': 'Original workspace',
    'owner_name': 'Owner',
    'member_count': 2,
    'is_member': false,
  };
  static Future<CodeFixture> open() async {
    final f = CodeFixture();
    f.server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    f.service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${f.server.port}'),
      accessToken: () async => 'synthetic-code-test',
      accountSubject: () => f.subject,
      deviceId: 'code-test',
      allowHttpForTesting: true,
    );
    unawaited(
      f.server.forEach((r) async {
        Object? data;
        if (r.method == 'GET') {
          if (r.uri.path.endsWith('/invite-code')) {
            final observed = f.overrideRead ? f.readOverride : {'code': f.code};
            if (f.releaseRead != null) await f.releaseRead!.future;
            data = observed;
          } else if (r.uri.path == '/api/invite-code-info') {
            data = f.preview;
          } else {
            data = [
              {'workspace_id': wid, 'role': f.role},
            ];
          }
        } else {
          final text = await utf8.decoder.bind(r).join();
          final body = text.isEmpty ? null : jsonDecode(text);
          f.writes.add({'method': r.method, 'path': r.uri.path, 'body': body});
          if (r.uri.path.endsWith('/invite-code')) {
            if (r.method == 'POST') {
              f.code = nextCode;
              data = {'code': f.createOverride ?? f.code};
              if (f.dropCreate) {
                final socket = await r.response.detachSocket(
                  writeHeaders: false,
                );
                socket.destroy();
                return;
              }
            } else {
              f.code = null;
            }
          } else if (r.uri.path.endsWith('/join-by-invite-code')) {
            data = {'workspace_id': f.joinedWorkspace};
          }
        }
        r.response.headers.contentType = ContentType.json;
        r.response.write(jsonEncode({'code': 0, 'data': data}));
        await r.response.close();
      }),
    );
    addTearDown(() async {
      if (f.releaseRead != null && !f.releaseRead!.isCompleted)
        f.releaseRead!.complete();
      f.service.closeClient();
      await f.server.close(force: true);
    });
    return f;
  }
}

void main() {
  test(
    'original code create/delete use exact contracts and old observation cannot repeat a write',
    () async {
      final f = await CodeFixture.open();
      final empty = await f.service.workspaceInviteCode(wid);
      expect(empty.code, isNull);
      final created = await f.service.createWorkspaceInviteCode(
        empty,
        validityPeriodHours: 24,
      );
      expect(created.code, nextCode);
      expect(f.writes.single['body'], {'validity_period_hours': 24});
      await expectLater(
        f.service.createWorkspaceInviteCode(empty),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      expect(f.writes.length, 1);
      await f.service.deleteWorkspaceInviteCode(created);
      expect(f.writes.last['method'], 'DELETE');
      expect(f.code, isNull);
      await expectLater(
        f.service.deleteWorkspaceInviteCode(created),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      expect(f.writes.length, 2);
    },
  );
  for (final raw in [
    {'wrong': null},
    {'code': ''},
    {'code': 'https://example.test'},
    {'code': ' AAAAAAAAAAAAAAAA'},
    {'code': 12},
  ]) {
    test(
      'malformed original code observation is rejected ${raw.keys.first} ${raw['code'].runtimeType}',
      () async {
        final f = await CodeFixture.open();
        f.overrideRead = true;
        f.readOverride = raw;
        await expectLater(
          f.service.workspaceInviteCode(wid),
          throwsA(anything),
        );
        expect(f.writes, isEmpty);
      },
    );
  }
  test(
    'explicit unlimited and bounded validity do not silently accept expired or excessive UI values',
    () async {
      final f = await CodeFixture.open();
      final empty = await f.service.workspaceInviteCode(wid);
      for (final hours in [-1, 0, 8761]) {
        await expectLater(
          f.service.createWorkspaceInviteCode(
            empty,
            validityPeriodHours: hours,
          ),
          throwsArgumentError,
        );
      }
      expect(f.writes, isEmpty);
      await f.service.createWorkspaceInviteCode(
        empty,
        validityPeriodHours: null,
      );
      expect(f.writes.single['body'], {'validity_period_hours': null});
    },
  );
  test(
    'fresh same-code read retires a captured earlier read before dispatch',
    () async {
      final f = await CodeFixture.open();
      f.code = firstCode;
      final before = await f.service.workspaceInviteCode(wid);
      await f.service.workspaceInviteCode(wid);
      await expectLater(
        f.service.deleteWorkspaceInviteCode(before),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      expect(f.writes, isEmpty);
    },
  );
  test(
    'lost create response remains fenced after current-state read and cannot affect email invitation intents',
    () async {
      final f = await CodeFixture.open();
      final before = await f.service.workspaceInviteCode(wid);
      f.dropCreate = true;
      await expectLater(
        f.service.createWorkspaceInviteCode(before),
        throwsA(anything),
      );
      expect(f.writes.length, 1);
      final read = await f.service.workspaceInviteCode(wid);
      expect(read.code, nextCode);
      expect(
        f.service.pendingInviteCodeMutations(wid).single.status,
        AppFlowyAccessMutationStatus.unconfirmed,
      );
      await expectLater(
        f.service.createWorkspaceInviteCode(read),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      await expectLater(
        f.service.deleteWorkspaceInviteCode(read),
        throwsA(isA<AppFlowyAccessMutationBlocked>()),
      );
      expect(f.writes.length, 1);
      expect(f.service.pendingAccessMutations(wid), isEmpty);
      await f.service.inviteWorkspaceMembers(wid, [
        'separate@example.test',
      ], skipEmailSend: true);
      expect(f.writes.length, 2);
    },
  );
  test(
    'known acceptance with malformed code can be reviewed without turning it into an unknown send',
    () async {
      final f = await CodeFixture.open();
      final before = await f.service.workspaceInviteCode(wid);
      f.createOverride = 'bad';
      await expectLater(
        f.service.createWorkspaceInviteCode(before),
        throwsA(anything),
      );
      expect(
        f.service.pendingInviteCodeMutations(wid).single.status,
        AppFlowyAccessMutationStatus.accepted,
      );
      final read = await f.service.workspaceInviteCode(wid);
      expect(read.code, nextCode);
      expect(f.service.pendingInviteCodeMutations(wid), isEmpty);
    },
  );
  test(
    'A-B-A account lifetime rejects delayed original code read and prior preview',
    () async {
      final f = await CodeFixture.open();
      final preview = await f.service.previewWorkspaceInviteCode(firstCode);
      f.releaseRead = Completer<void>();
      final pending = f.service.workspaceInviteCode(wid);
      final rejected = expectLater(pending, throwsA(anything));
      await Future<void>.delayed(const Duration(milliseconds: 20));
      f.subject = 'bob';
      f.service.invalidateAccountOperations();
      f.subject = 'alice';
      f.service.invalidateAccountOperations();
      f.releaseRead!.complete();
      await rejected;
      await expectLater(
        f.service.joinWorkspaceByInviteCode(preview),
        throwsA(anything),
      );
      expect(f.writes, isEmpty);
    },
  );
  test(
    'original preview does not join; joining reads actual Guest role instead of assuming Member',
    () async {
      final f = await CodeFixture.open();
      f.role = 'Guest';
      final preview = await f.service.previewWorkspaceInviteCode(firstCode);
      expect(preview.workspaceId, wid);
      expect(f.writes, isEmpty);
      expect(
        await f.service.joinWorkspaceByInviteCode(preview),
        AppFlowyWorkspaceRole.guest,
      );
      expect(f.writes.single['body'], {'code': firstCode});
      await expectLater(
        f.service.joinWorkspaceByInviteCode(preview),
        throwsA(anything),
      );
      expect(f.writes.length, 1);
    },
  );
  test(
    'join response cannot redirect the current preview into another workspace',
    () async {
      final f = await CodeFixture.open();
      f.joinedWorkspace = other;
      final preview = await f.service.previewWorkspaceInviteCode(firstCode);
      await expectLater(
        f.service.joinWorkspaceByInviteCode(preview),
        throwsA(anything),
      );
      expect(f.writes.length, 1);
    },
  );
  for (final patch in [
    {'is_member': null},
    {'member_count': -1},
    {'workspace_id': 'bad'},
  ]) {
    test(
      'unsupported original preview fields remain unavailable ${patch.keys.first}',
      () async {
        final f = await CodeFixture.open();
        f.preview.addAll(patch);
        await expectLater(
          f.service.previewWorkspaceInviteCode(firstCode),
          throwsA(anything),
        );
        expect(f.writes, isEmpty);
      },
    );
  }
}
