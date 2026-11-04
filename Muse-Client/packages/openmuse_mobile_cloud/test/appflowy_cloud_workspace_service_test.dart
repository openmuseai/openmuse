import 'dart:convert';
import 'dart:io';

import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

void main() {
  test(
    'maps existing workspace, session list, and session open contracts',
    () async {
      final seen = <String>[];
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final serving = server.forEach((request) async {
        seen.add('${request.method} ${request.uri.path}');
        expect(
          request.headers.value(HttpHeaders.authorizationHeader),
          'Bearer access',
        );
        Object? data;
        switch (request.uri.path) {
          case '/api/workspace':
            data = [
              {
                'workspace_id': 'workspace-1',
                'database_storage_id': 'storage-1',
                'workspace_name': 'Desktop Workspace',
                'role': 'Owner',
              },
            ];
          case '/api/muse/dsh/sessions':
            data = {
              'items': [
                {
                  'sessionRef': 'session-1',
                  'workspaceRef': 'workspace-1',
                  'state': 'ready',
                  'instanceRef': 'instance-1',
                  'nodeId': 'local',
                  'createdAt': 1000,
                  'lastActiveAt': 2000,
                  'attachedDeviceCount': 1,
                },
              ],
            };
          case '/api/muse/dsh/session/open':
            final body =
                jsonDecode(await utf8.decoder.bind(request).join()) as Map;
            expect(body['workspaceId'], 'workspace-1');
            expect(body['deviceId'], 'mobile-test');
            data = {
              'sessionRef': 'session-1',
              'instanceRef': 'instance-1',
              'webUrl': 'https://dsh.openmuse.test/u/tenant/?token=launch',
              'nodeId': 'local',
            };
          default:
            request.response.statusCode = 404;
            await request.response.close();
            return;
        }
        request.response.headers.contentType = ContentType.json;
        request.response.write(
          jsonEncode({'code': 0, 'message': '', 'data': data}),
        );
        await request.response.close();
      });
      final service = AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
        accessToken: () async => 'access',
        deviceId: 'mobile-test',
        allowHttpForTesting: true,
      );

      final workspaces = await service.listWorkspaces();
      final sessions = await service.listSessions();
      final opened = await service.open('workspace-1', 7);
      service.closeClient();
      await server.close(force: true);
      await serving;

      expect(workspaces.single.workspaceRef, 'workspace-1');
      expect(workspaces.single.title, 'Desktop Workspace');
      expect(sessions.single.sessionRef, 'session-1');
      expect(sessions.single.isRunning, isTrue);
      expect(opened.sessionRef, sessions.single.sessionRef);
      expect(opened.origin, 'https://dsh.openmuse.test');
      expect(opened.path, '/u/tenant/?token=launch');
      expect(opened.generation, 7);
      expect(seen, [
        'GET /api/workspace',
        'GET /api/muse/dsh/sessions',
        'POST /api/muse/dsh/session/open',
      ]);
    },
  );

  test('business errors fail even when HTTP status is successful', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final serving = server.forEach((request) async {
      request.response.headers.contentType = ContentType.json;
      request.response.write(
        jsonEncode({'code': 1001, 'message': 'denied', 'data': null}),
      );
      await request.response.close();
    });
    final service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
      accessToken: () async => 'access',
      deviceId: 'mobile-test',
      allowHttpForTesting: true,
    );

    await expectLater(
      service.listWorkspaces(),
      throwsA(
        isA<CloudServiceException>().having(
          (error) => error.code,
          'code',
          CloudServiceErrorCode.unavailable,
        ),
      ),
    );
    service.closeClient();
    await server.close(force: true);
    await serving;
  });

  test('HTTP is limited to explicit loopback development', () {
    expect(
      () => AppFlowyCloudWorkspaceService(
        baseUri: Uri.parse('http://cloud.example.com'),
        accessToken: () async => 'access',
        deviceId: 'mobile-test',
        allowHttpForTesting: true,
      ),
      throwsArgumentError,
    );
  });
}
