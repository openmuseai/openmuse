import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';
import 'package:openmuse_web/auth/browser_auth.dart';
import 'package:openmuse_web/workbench/browser_paired_client.dart';

void main() {
  test(
    'authenticated device discovery creates a scoped Desktop grant',
    () async {
      final store = MemoryAuthSessionStore();
      await store.write(
        GoTrueSession(
          accessToken: 'access-fixture',
          refreshToken: 'refresh-fixture',
          expiresAt: DateTime.now().add(const Duration(hours: 1)),
          user: const GoTrueUser(
            id: 'account-fixture',
            email: 'person@example.test',
          ),
        ),
      );
      final auth = GoTrueAuthenticationController(
        provider: BrowserGoTrueProvider(
          origin: Uri.parse('https://auth.example.test/gotrue'),
          client: MockClient(
            (_) async => throw StateError('unexpected auth call'),
          ),
        ),
        store: store,
      );
      await auth.restore();
      final client = BrowserPairedClient(
        authentication: auth,
        origin: Uri.parse('https://web.example.test'),
        client: MockClient((request) async {
          expect(request.headers['authorization'], 'Bearer access-fixture');
          if (request.url.path == '/api/muse/devices') {
            return http.Response(
              jsonEncode({
                'code': 0,
                'data': [
                  {
                    'deviceId': 'desktop.fixture',
                    'displayName': 'Test Desktop',
                    'deviceKind': 'desktop',
                    'online': true,
                    'transportOrigin': 'https://link.example.test',
                    'capabilities': ['paired-desktop.transport'],
                  },
                  {
                    'deviceId': 'desktop.offline',
                    'displayName': 'Offline',
                    'deviceKind': 'desktop',
                    'online': false,
                    'transportOrigin': 'https://link.example.test',
                    'capabilities': ['paired-desktop.transport'],
                  },
                ],
              }),
              200,
            );
          }
          expect(request.url.path, '/v1/account/open');
          final body = jsonDecode(request.body) as Map<String, dynamic>;
          expect(body['targetDeviceRef'], 'desktop.fixture');
          expect(body['workspaceRef'], 'openmuse.local.default');
          expect(body['deviceRef'], startsWith('web.'));
          return http.Response(
            jsonEncode({
              'deviceRef': 'desktop.fixture',
              'workspaceRef': 'openmuse.local.default',
              'session': {'path': '/u/${'a' * 64}'},
            }),
            200,
          );
        }),
      );
      final desktops = await client.listDesktops();
      expect(desktops, hasLength(1));
      final connection = await client.connect(desktops.single);
      expect(connection.desktopRef, 'desktop.fixture');
      expect(connection.workspaceRef, 'openmuse.local.default');
      expect(connection.bootstrapPath, '/u/${'a' * 64}');
    },
  );
}
