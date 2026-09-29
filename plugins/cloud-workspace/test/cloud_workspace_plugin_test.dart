import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  test('plugin discovers a ready account-scoped DSH session', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    addTearDown(server.close);
    unawaited(
      server.forEach((request) async {
        request.response.headers.contentType = ContentType.json;
        if (request.uri.path == '/api/workspace') {
          request.response.write(
            jsonEncode({
              'code': 0,
              'data': [
                {
                  'workspace_id': 'workspace-1',
                  'workspace_name': 'Shared Office',
                  'role': 'Owner',
                },
              ],
            }),
          );
        } else if (request.uri.path == '/api/muse/dsh/sessions') {
          request.response.write(
            jsonEncode({
              'code': 0,
              'data': {
                'items': [
                  {
                    'sessionRef': 'session-1',
                    'workspaceRef': 'workspace-1',
                    'state': 'ready',
                    'instanceRef': 'instance-1',
                    'nodeId': 'node-1',
                    'createdAt': 1,
                    'lastActiveAt': 2,
                    'attachedDeviceCount': 1,
                  },
                ],
              },
            }),
          );
        } else {
          request.response.statusCode = HttpStatus.notFound;
        }
        await request.response.close();
      }),
    );
    final authentication = _Authentication();
    final plugin = OpenMuseCloudWorkspacePlugin(
      authentication: authentication,
      cloudOrigin: Uri.parse('http://127.0.0.1:${server.port}'),
      deviceId: 'desktop-test',
      allowInsecureLoopback: true,
    );
    addTearDown(plugin.deactivate);
    await plugin.activate(
      OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );

    expect(plugin.controller.snapshot.workspaces.single.title, 'Shared Office');
    expect(
      plugin.controller.snapshot.sessionFor('workspace-1')?.sessionRef,
      'session-1',
    );
  });
}

final class _Authentication extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot snapshot = OpenMuseAuthenticationSnapshot(
    phase: OpenMuseAuthenticationPhase.authenticated,
    identity: const OpenMuseAuthenticatedIdentity(
      subject: 'account-1',
      email: 'person@example.com',
    ),
  );

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => 'token';

  @override
  Future<void> restore() async {}

  @override
  Future<void> signInWithPassword(String email, String password) async {}

  @override
  Future<void> signOut() async {}
}
