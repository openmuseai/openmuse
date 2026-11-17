import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:test/test.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

const workspace = '10000000-0000-0000-0000-000000000001';
const view = '20000000-0000-0000-0000-000000000001';
void main() {
  test(
    'original sharing source display name is optional for down-level servers',
    () {
      final person = {
        'view_id': view,
        'email': 'guest@example.test',
        'name': '',
        'role': 'Guest',
        'access_level': 10,
        'pending_invitation': false,
      };
      expect(AppFlowySharedPerson.fromJson(person).sourceViewName, isNull);
      expect(
        AppFlowySharedPerson.fromJson({
          ...person,
          'source_view_name': '父文档',
        }).sourceViewName,
        '父文档',
      );
    },
  );

  for (final mode in ['supported', 'unsupported', 'denied', 'late-account']) {
    test('original row creation capability and outcome: $mode', () async {
      const root = '30000000-0000-0000-0000-000000000001';
      const row = '40000000-0000-0000-0000-000000000001';
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var writes = 0;
      late AppFlowyCloudWorkspaceService service;
      service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'test-token',
        accountSubject: () => 'alice',
        deviceId: 'creation-api-test',
        allowHttpForTesting: true,
      );
      addTearDown(() async {
        service.closeClient();
        await server.close(force: true);
      });
      unawaited(
        server.forEach((request) async {
          var code = 0;
          Object? data;
          if (request.method == 'GET') {
            if (mode == 'unsupported') {
              request.response.statusCode = 404;
            } else {
              data = {
                'profile': 'appflowy.database-row-creation/1',
                'rootId': root,
                'maxStateBytes': 16 * 1024 * 1024,
              };
            }
          } else {
            writes++;
            expect(
              request.uri.queryParameters['include_creation_state'],
              'true',
            );
            expect(jsonDecode(await utf8.decoder.bind(request).join()), {
              'cells': {'title': 'Created'},
            });
            if (mode == 'late-account') service.invalidateAccountOperations();
            if (mode == 'denied') code = 1011;
            data = {
              'profile': 'appflowy.database-row-creation/1',
              'rootId': root,
              'rowId': row,
              'beforeRootState': 'AA==',
              'rootUpdate': 'AA==',
              'rowState': 'AA==',
            };
          }
          request.response.headers.contentType = ContentType.json;
          request.response.write(jsonEncode({'code': code, 'data': data}));
          await request.response.close();
        }),
      );
      final result = service.createDatabaseRow(workspace, root, {
        'title': 'Created',
      });
      if (mode == 'supported') {
        final created = await result;
        expect(created.rootId, root);
        expect(created.rowId, row);
        expect(writes, 1);
      } else if (mode == 'late-account') {
        await expectLater(result, throwsA(isA<CloudServiceException>()));
        expect(writes, 1);
      } else {
        await expectLater(
          result,
          throwsA(
            isA<AppFlowyRowCreationRejected>().having(
              (e) => e.code,
              'code',
              mode == 'denied' ? 'READ_ONLY' : 'ROW_CREATION_UNSUPPORTED',
            ),
          ),
        );
        expect(writes, mode == 'denied' ? 1 : 0);
      }
    });
  }
  test(
    'broken catalog GET retries once with fresh authorization; sharing writes never replay',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      var reads = 0, writes = 0, tokens = 0;
      final headers = <String?>[];
      final service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'test-${++tokens}',
        deviceId: 'retry-test',
        allowHttpForTesting: true,
      );
      addTearDown(() async {
        service.closeClient();
        await server.close(force: true);
      });
      unawaited(
        server.forEach((request) async {
          await request.drain<void>();
          headers.add(request.headers.value('authorization'));
          if (request.method == 'GET' && ++reads == 2) {
            request.response.headers.contentType = ContentType.json;
            request.response.write(
              jsonEncode({
                'code': 0,
                'data': [
                  {'workspace_id': workspace, 'role': 'Owner'},
                ],
              }),
            );
            await request.response.close();
          } else {
            if (request.method != 'GET') writes++;
            final socket = await request.response.detachSocket(
              writeHeaders: false,
            );
            socket.destroy();
          }
        }),
      );
      expect(
        await service.workspaceRole(workspace),
        AppFlowyWorkspaceRole.owner,
      );
      expect(reads, 2);
      expect(headers, ['Bearer test-1', 'Bearer test-2']);
      await expectLater(
        service.shareView(workspace, view, [
          'guest@example.com',
        ], AppFlowyAccessLevel.edit),
        throwsA(isA<HttpException>()),
      );
      expect(writes, 1);
      // A second broken read also stops at the same bound.
      await expectLater(
        service.workspaceRole(workspace),
        throwsA(isA<HttpException>()),
      );
      expect(reads, 4);
    },
  );
  test(
    'Folder views join canonical roots and reject ambiguous database ownership',
    () async {
      const root = '30000000-0000-0000-0000-000000000001';
      const otherRoot = '30000000-0000-0000-0000-000000000002';
      const dbView = '20000000-0000-0000-0000-000000000002';
      const missingView = '20000000-0000-0000-0000-000000000003';
      var ambiguous = false;
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'test-token',
        deviceId: 'catalog-test',
        allowHttpForTesting: true,
      );
      addTearDown(() async {
        service.closeClient();
        await server.close(force: true);
      });
      unawaited(
        server.forEach((request) async {
          Object data;
          if (request.uri.path == '/api/workspace') {
            data = [
              {'workspace_id': workspace, 'role': 'Owner'},
            ];
          } else if (request.uri.path.endsWith('/folder')) {
            data = {
              'view_id': workspace,
              'children': [
                {
                  'view_id': view,
                  'layout': 0,
                  'children': [
                    {'view_id': dbView, 'layout': 1, 'children': []},
                    {'view_id': missingView, 'layout': 2, 'children': []},
                  ],
                },
              ],
            };
          } else {
            data = [
              {
                'id': root,
                'views': [
                  {'view_id': dbView},
                ],
              },
              if (ambiguous)
                {
                  'id': otherRoot,
                  'views': [
                    {'view_id': dbView},
                  ],
                },
            ];
          }
          request.response.headers.contentType = ContentType.json;
          request.response.write(jsonEncode({'code': 0, 'data': data}));
          await request.response.close();
        }),
      );
      final pages = await service.workspacePages(workspace);
      expect(pages.length, 2);
      expect(pages.first, isA<AppFlowyDocumentPage>());
      final database = pages.last as AppFlowyDatabasePage;
      expect(database.databaseId, root);
      expect(database.viewId, dbView);
      expect(database.ancestors, [workspace, view]);
      ambiguous = true;
      await expectLater(
        service.workspacePages(workspace),
        throwsA(isA<CloudServiceException>()),
      );
    },
  );
  test('original membership and sharing routes, bodies and DTOs', () async {
    final seen = <(String, String, Object?)>[];
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
      accessToken: () async => 'secret-test',
      deviceId: 'access-test',
      allowHttpForTesting: true,
    );
    addTearDown(() async {
      service.closeClient();
      await server.close(force: true);
    });
    unawaited(
      server.forEach((request) async {
        final raw = await utf8.decoder.bind(request).join();
        seen.add((
          request.method,
          request.uri.toString(),
          raw.isEmpty ? null : jsonDecode(raw),
        ));
        expect(request.headers.value('authorization'), 'Bearer secret-test');
        expect(request.headers.value('device_id'), 'access-test');
        Object? data;
        if (request.uri.path == '/api/workspace') {
          data = [
            {'workspace_id': workspace, 'role': 'Owner'},
          ];
        } else if (request.method == 'GET') {
          data = [
            {'email': 'member@example.com', 'name': '', 'role': 'Member'},
          ];
        } else if (request.uri.path.endsWith('/access-details')) {
          data = {
            'view_id': view,
            'shared_with': [
              {
                'view_id': view,
                'email': 'guest@example.com',
                'name': '',
                'role': 'Guest',
                'access_level': 30,
                'pending_invitation': false,
              },
            ],
          };
        }
        request.response.headers.contentType = ContentType.json;
        request.response.write(jsonEncode({'code': 0, 'data': data}));
        await request.response.close();
      }),
    );
    expect(await service.workspaceRole(workspace), AppFlowyWorkspaceRole.owner);
    expect(
      (await service.workspaceMembers(workspace)).single.role,
      AppFlowyWorkspaceRole.member,
    );
    await service.inviteWorkspaceMembers(
      workspace,
      [' guest@example.com '],
      role: AppFlowyWorkspaceRole.guest,
      skipEmailSend: true,
    );
    await service.updateWorkspaceMember(
      workspace,
      'guest@example.com',
      AppFlowyWorkspaceRole.member,
    );
    await service.removeWorkspaceMembers(workspace, ['guest@example.com']);
    final access = await service.pageAccess(
      workspace,
      view,
      ancestors: [workspace],
    );
    expect(access.people.single.access, AppFlowyAccessLevel.edit);
    expect(access.people.single.viewId, view);
    await service.shareView(workspace, view, [
      'guest@example.com',
    ], AppFlowyAccessLevel.readOnly);
    await service.revokeViewAccess(workspace, view, ['guest@example.com']);
    final expected = [
      ('GET', '/api/workspace?include_role=true', null),
      ('GET', '/api/workspace/$workspace/member', null),
      (
        'POST',
        '/api/workspace/$workspace/invite',
        [
          {
            'email': 'guest@example.com',
            'role': 'Guest',
            'skip_email_send': true,
          },
        ],
      ),
      (
        'PUT',
        '/api/workspace/$workspace/member',
        {'email': 'guest@example.com', 'role': 'Member'},
      ),
      ('DELETE', '/api/workspace/$workspace/member', ['guest@example.com']),
      (
        'POST',
        '/api/sharing/workspace/$workspace/view/$view/access-details',
        {
          'ancestor_view_ids': [workspace],
        },
      ),
      (
        'PUT',
        '/api/sharing/workspace/$workspace/view',
        {
          'view_id': view,
          'emails': ['guest@example.com'],
          'access_level': 10,
        },
      ),
      (
        'POST',
        '/api/sharing/workspace/$workspace/view/$view/revoke-access',
        {
          'emails': ['guest@example.com'],
        },
      ),
    ];
    expect(seen.length, expected.length);
    for (var i = 0; i < seen.length; i++) {
      expect(seen[i].$1, expected[i].$1);
      expect(seen[i].$2, expected[i].$2);
      expect(seen[i].$3, expected[i].$3);
    }
  });

  test(
    'account switch during refresh never retries an old mutation with a new token',
    () async {
      var subject = 'alice', calls = 0;
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => subject,
        accountSubject: () => subject,
        refreshAccessToken: () async {
          subject = 'bob';
          return 'bob';
        },
        deviceId: 'access-test',
        allowHttpForTesting: true,
      );
      addTearDown(() async {
        service.closeClient();
        await server.close(force: true);
      });
      unawaited(
        server.forEach((request) async {
          calls++;
          request.response.statusCode = 401;
          await request.response.close();
        }),
      );
      await expectLater(
        service.shareView(workspace, view, [
          'guest@example.com',
        ], AppFlowyAccessLevel.edit),
        throwsA(isA<Exception>()),
      );
      expect(calls, 1);
    },
  );

  test('A-B-A account generation and late read responses are fenced', () async {
    final pending = Completer<HttpRequest>();
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
      accessToken: () async => 'alice',
      accountSubject: () => 'alice',
      deviceId: 'access-test',
      allowHttpForTesting: true,
    );
    addTearDown(() async {
      service.closeClient();
      await server.close(force: true);
    });
    server.listen(pending.complete);
    final result = expectLater(
      service.workspaceMembers(workspace),
      throwsA(isA<Exception>()),
    );
    final request = await pending.future;
    service.invalidateAccountOperations();
    request.response.headers.contentType = ContentType.json;
    request.response.write(jsonEncode({'code': 0, 'data': []}));
    await request.response.close();
    await result;
  });

  test(
    'invalid identifiers, roles and recipients fail before sending',
    () async {
      final service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('https://invalid.example'),
        accessToken: () async => throw StateError('must not request token'),
        deviceId: 'test',
      );
      addTearDown(service.closeClient);
      await expectLater(
        service.workspaceMembers('../wrong'),
        throwsArgumentError,
      );
      await expectLater(
        service.shareView(workspace, view, [], AppFlowyAccessLevel.edit),
        throwsArgumentError,
      );
      await expectLater(
        service.inviteWorkspaceMembers(workspace, [
          'owner@example.com',
        ], role: AppFlowyWorkspaceRole.owner),
        throwsArgumentError,
      );
      expect(() => AppFlowyAccessLevel.parse(99), throwsA(isA<Exception>()));
    },
  );
}
