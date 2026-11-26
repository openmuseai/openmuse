import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:openmuse_dsh_plugin/src/dsh_navigation.dart';

class _NativeHttp extends HttpOverrides {
  @override
  HttpClient createHttpClient(SecurityContext? context) =>
      super.createHttpClient(context);
}

void main() {
  testWidgets(
    'blank sessions belong to the authorized workspace registry; navigation sends no message',
    (tester) async {
      await HttpOverrides.runWithHttpOverrides(() async {
        final supervisor = DshSidecarSupervisor()
          ..state = DshSidecarState.ready;
        final mount = ValueNotifier<String?>('/current');
        final requests = <String>[];
        final server = await tester.runAsync(
          () => HttpServer.bind(InternetAddress.loopbackIPv4, 0),
        );
        supervisor.endpoint = Uri.parse('http://127.0.0.1:${server!.port}/');
        server.listen((req) async {
          expect(
            req.headers.value('x-openmuse-bridge-token'),
            supervisor.bridgeToken,
          );
          requests.add('${req.method} ${req.uri.path}');
          final result = switch (req.uri.path) {
            '/openmuse-native/v1/hello' => {
              'protocolVersion': 1,
              'dshVersion': '0.2.0-rc.2',
              'capabilities': ['sessions'],
              'compatibility': {
                'nativeConversationCompatible': true,
                'fallbackRequired': false,
              },
            },
            '/openmuse-native/v1/sessions' => {
              'items': [
                {
                  'sessionId': 'blank-current',
                  'blank': true,
                  'running': false,
                  'updatedAt': 2,
                  'projections': {},
                },
                {
                  'sessionId': 'other',
                  'cwd': '/other',
                  'blank': false,
                  'running': false,
                  'updatedAt': 1,
                  'projections': {
                    'values': {'title': 'Private other workspace'},
                  },
                },
              ],
            },
            '/openmuse-native/v1/workspaces' => {
              'items': [
                {
                  'workspaceId': 'current',
                  'title': 'Current',
                  'sessionIds': ['blank-current'],
                  'createdAt': '2026-11-26T00:00:00Z',
                  'updatedAt': '2026-11-26T00:00:00Z',
                },
              ],
            },
            _ => {'error': 'unexpected request'},
          };
          req.response.headers.contentType = ContentType.json;
          req.response.write(jsonEncode(result));
          await req.response.close();
        });
        await tester.pumpWidget(
          MaterialApp(
            home: SizedBox(
              width: 256,
              height: 800,
              child: Material(
                child: DshNavigation(
                  supervisor: supervisor,
                  activeMount: mount,
                  resolveWorkspace: (_) async => 'current',
                ),
              ),
            ),
          ),
        );
        for (
          var i = 0;
          i < 50 && find.text('New conversation').evaluate().isEmpty;
          i++
        ) {
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 20)),
          );
          await tester.pump();
        }
        expect(find.text('New conversation'), findsOneWidget);
        expect(find.text('Private other workspace'), findsNothing);
        await tester.tap(
          find.byKey(const ValueKey('agent-session.blank-current')),
        );
        expect(supervisor.requestedSession.value, 'blank-current');
        expect(requests.every((r) => r.startsWith('GET ')), isTrue);
        expect(requests.length, 3);
        await tester.pumpWidget(const SizedBox.shrink());
        await tester.runAsync(() => server.close(force: true));
        mount.dispose();
        supervisor.dispose();
      }, _NativeHttp());
    },
  );
  testWidgets(
    'denied AI sharing does not launch a sidecar or request a catalog',
    (tester) async {
      final supervisor = DshSidecarSupervisor()..aiSharingAllowed = false;
      final mount = ValueNotifier<String?>('/current');
      var resolved = 0;
      await tester.pumpWidget(
        MaterialApp(
          home: Material(
            child: DshNavigation(
              supervisor: supervisor,
              activeMount: mount,
              resolveWorkspace: (_) async {
                resolved++;
                return 'current';
              },
            ),
          ),
        ),
      );
      await tester.pump();
      expect(supervisor.launchCount, 0);
      expect(resolved, 0);
      expect(supervisor.endpoint, isNull);
      await tester.pumpWidget(const SizedBox.shrink());
      mount.dispose();
      supervisor.dispose();
    },
  );
}
