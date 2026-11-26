import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mcp_app_runtime/openmuse_mcp_app_runtime.dart';

void main() {
  test(
    'navigation is a bounded UI-only projection and is revoked on account change',
    () async {
      final dir = await Directory.systemTemp.createTemp('navigation-contract-');
      var scope = 'authorized-account';
      const html =
          '<!doctype html><html><body>Plugin-owned navigation</body></html>';
      await File('${dir.path}/ui.html').writeAsString(html);
      await File('${dir.path}/apps.json').writeAsString(
        jsonEncode([
          {
            'id': 'test.navigation',
            'title': 'Test',
            'version': '1',
            'endpoint': 'http://127.0.0.1:1/mcp',
            'uiResourceUri': 'ui://test/main',
            'uiFile': 'ui.html',
            'uiSha256': sha256.convert(utf8.encode(html)).toString(),
            'owner': sha256.convert(utf8.encode(scope)).toString(),
            'desktopNavigation': 1,
          },
        ]),
      );
      final app = (await RemoteMcpApp.load(
        directory: dir,
        scope: () => scope,
        readSecret: (_) async => null,
        writeSecret: (_, __) async {},
      )).single;
      var opens = 0;
      app.onOpen = () => opens++;
      await app.request('tools/call', {
        'name': 'openmuse_navigation_select',
        'arguments': {'projectId': 'one'},
      });
      expect(app.navigationState, {'projectId': 'one'});
      expect(opens, 1);
      await app.request('tools/call', {
        'name': 'openmuse_navigation_select',
        'arguments': {'projectId': 'one'},
      });
      expect(opens, 1);
      await expectLater(
        app.request('tools/call', {
          'name': 'openmuse_navigation_select',
          'arguments': {'projectId': 'two'},
        }, agent: true),
        throwsStateError,
      );
      await expectLater(
        app.request('tools/call', {
          'name': 'openmuse_navigation_select',
          'arguments': {
            'nested': {'script': 'invalid'},
          },
        }),
        throwsStateError,
      );
      scope = 'different-account';
      await expectLater(
        app.request('tools/call', {
          'name': 'openmuse_navigation_select',
          'arguments': {'projectId': 'two'},
        }),
        throwsStateError,
      );
      expect(app.navigationState, isEmpty);
      await app.closeSession();
      app.dispose();
      await dir.delete(recursive: true);
    },
  );
}
