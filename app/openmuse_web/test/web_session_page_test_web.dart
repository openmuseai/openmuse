import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_web/auth/browser_auth.dart';
import 'package:openmuse_web/workbench/browser_paired_client.dart';
import 'package:openmuse_web/workbench/web_session_page.dart';

void main() {
  testWidgets('device discovery failure stays inside the workbench', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(() {
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
    });
    final store = MemoryAuthSessionStore();
    await store.write(
      GoTrueSession(
        accessToken: 'fixture-access',
        refreshToken: 'fixture-refresh',
        expiresAt: DateTime.now().add(const Duration(hours: 1)),
        user: const GoTrueUser(
          id: 'fixture-user',
          email: 'person@example.test',
        ),
      ),
    );
    final auth = GoTrueAuthenticationController(
      provider: BrowserGoTrueProvider(
        origin: Uri.parse('https://auth.example.test/gotrue'),
        client: MockClient((_) async => throw StateError('unexpected')),
      ),
      store: store,
    );
    await auth.restore();
    final paired = BrowserPairedClient(
      authentication: auth,
      origin: Uri.parse('https://web.example.test'),
      client: MockClient((_) async => http.Response('unauthorized', 401)),
    );
    await tester.pumpWidget(
      MaterialApp(
        theme: buildOpenMuseTheme(),
        home: WebSessionPage(authentication: auth, pairedClient: paired),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('OpenMuse'), findsOneWidget);
    expect(find.text('Blank page'), findsOneWidget);
    expect(find.text('刷新并重连'), findsOneWidget);
    expect(find.text('连接 Desktop 后使用 DSH'), findsOneWidget);
    expect(find.textContaining('授权已失效'), findsOneWidget);
  });
}
