import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/mobile_plugins/mobile_my_page.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  testWidgets('account deletion requires an explicit second confirmation', (
    tester,
  ) async {
    var requests = 0;
    await tester.pumpWidget(
      _localizedApp(
        home: MobileMyPage(
          themeMode: ThemeMode.system,
          onThemeMode: (_) {},
          workspaces: const [],
          deviceName: 'Test phone',
          authentication: _SignedInAuthentication(),
          onDeleteAccount: () async {
            requests++;
          },
        ),
      ),
    );

    await tester.tap(find.byKey(const ValueKey('my-account')));
    await tester.pumpAndSettle();
    await tester.scrollUntilVisible(
      find.byKey(const ValueKey('my-delete-account')),
      300,
      scrollable: find.byType(Scrollable).first,
    );
    await tester.tap(find.byKey(const ValueKey('my-delete-account')));
    await tester.pumpAndSettle();
    expect(requests, 0);
    expect(find.text('永久删除账户？'), findsOneWidget);

    await tester.tap(find.text('取消'));
    await tester.pumpAndSettle();
    expect(requests, 0);

    await tester.tap(find.byKey(const ValueKey('my-delete-account')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('confirm-delete-account')));
    await tester.pumpAndSettle();
    expect(requests, 1);
  });
}

final class _SignedInAuthentication extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot get snapshot =>
      const OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.authenticated,
        identity: OpenMuseAuthenticatedIdentity(
          subject: 'test-user',
          email: 'test@example.com',
        ),
      );

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => 'test';

  @override
  Future<void> restore() async {}

  @override
  Future<void> signInWithPassword(String email, String password) async {}

  @override
  Future<void> signOut() async {}
}

Widget _localizedApp({required Widget home}) => OpenMuseLocaleScope(
  controller: OpenMuseLocaleController(
    initialTag: OpenMuseCatalog.builtIn.fallbackLocale,
  ),
  child: MaterialApp(home: home),
);
