import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  for (final size in const [
    Size(320, 568),
    Size(390, 844),
    Size(720, 900),
    Size(1440, 900),
  ]) {
    testWidgets('email page fits ${size.width}x${size.height}', (tester) async {
      tester.view.physicalSize = size;
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);

      await tester.pumpWidget(_app(_FakeController()));

      expect(find.text('Welcome to OpenMuse'), findsOneWidget);
      expect(find.byKey(const ValueKey('auth.email')), findsOneWidget);
      expect(find.text('Continue with password'), findsOneWidget);
      expect(tester.takeException(), isNull);
    });
  }

  testWidgets('small screen remains usable with keyboard inset', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(320, 568);
    tester.view.devicePixelRatio = 1;
    tester.view.viewInsets = const FakeViewPadding(bottom: 280);
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    addTearDown(tester.view.resetViewInsets);

    await tester.pumpWidget(_app(_FakeController()));
    await tester.enterText(
      find.byKey(const ValueKey('auth.email')),
      'muse@example.com',
    );
    final continueButton = find.byKey(const ValueKey('auth.continue-password'));
    await tester.ensureVisible(continueButton);
    await tester.tap(continueButton);
    await tester.pumpAndSettle();

    expect(find.byKey(const ValueKey('auth.password')), findsOneWidget);
    expect(find.byType(SingleChildScrollView), findsOneWidget);
    expect(tester.takeException(), isNull);
  });

  testWidgets('anonymous mode skips the login form', (tester) async {
    var anonymous = 0;
    await tester.pumpWidget(
      MaterialApp(
        home: OpenMuseLoginScreen(
          authentication: _FakeController(),
          onAnonymous: () => anonymous++,
        ),
      ),
    );

    await tester.tap(find.byKey(const ValueKey('auth.anonymous')));
    await tester.pump();

    expect(find.text('Anonymous mode'), findsOneWidget);
    expect(anonymous, 1);
    expect(find.byKey(const ValueKey('auth.password')), findsNothing);
  });

  testWidgets('invalid email stays on the email page', (tester) async {
    await tester.pumpWidget(_app(_FakeController()));
    await tester.enterText(find.byKey(const ValueKey('auth.email')), 'invalid');
    await tester.tap(find.byKey(const ValueKey('auth.continue-password')));
    await tester.pump();

    expect(find.text('Please enter a valid email address.'), findsOneWidget);
    expect(find.byKey(const ValueKey('auth.password')), findsNothing);
  });

  testWidgets('email flow requests a code and renders passcode page', (
    tester,
  ) async {
    final controller = _FakeController();
    await tester.pumpWidget(_app(controller));
    await tester.enterText(
      find.byKey(const ValueKey('auth.email')),
      'muse@example.com',
    );
    await tester.tap(find.byKey(const ValueKey('auth.continue-email')));
    await tester.pump();

    expect(controller.codeRequests, 1);
    expect(find.byKey(const ValueKey('auth.passcode')), findsOneWidget);
    await tester.enterText(
      find.byKey(const ValueKey('auth.passcode')),
      '123456',
    );
    await tester.tap(find.byKey(const ValueKey('auth.submit-passcode')));
    expect(controller.codeSignIns, 1);
  });

  testWidgets('password submission is disabled while submitting', (
    tester,
  ) async {
    final controller = _FakeController();
    await tester.pumpWidget(_app(controller));
    await tester.enterText(
      find.byKey(const ValueKey('auth.email')),
      'muse@example.com',
    );
    await tester.tap(find.byKey(const ValueKey('auth.continue-password')));
    await tester.pumpAndSettle();
    await tester.enterText(
      find.byKey(const ValueKey('auth.password')),
      'secret',
    );
    await tester.tap(find.byKey(const ValueKey('auth.submit-password')));
    await tester.pump();

    expect(controller.signInCalls, 1);
    expect(find.byType(CircularProgressIndicator), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('auth.submit-password')));
    expect(controller.signInCalls, 1);
  });

  testWidgets('authentication error is rendered without password exposure', (
    tester,
  ) async {
    final controller = _FakeController();
    await tester.pumpWidget(_app(controller));
    await tester.enterText(
      find.byKey(const ValueKey('auth.email')),
      'muse@example.com',
    );
    await tester.tap(find.byKey(const ValueKey('auth.continue-password')));
    await tester.pumpAndSettle();
    controller.fail('The email or password is incorrect.');
    await tester.pump();

    expect(find.text('The email or password is incorrect.'), findsOneWidget);
  });
}

Widget _app(_FakeController controller) => MaterialApp(
  home: OpenMuseLoginScreen(
    authentication: controller,
    cloudLabel: 'http://127.0.0.1:8000',
  ),
);

final class _FakeController extends ChangeNotifier
    implements
        OpenMuseAuthenticationController,
        OpenMuseEmailCodeAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot snapshot =
      const OpenMuseAuthenticationSnapshot.signedOut();
  int signInCalls = 0;
  int codeRequests = 0;
  int codeSignIns = 0;

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => null;
  @override
  Future<void> restore() async {}
  @override
  Future<void> signInWithPassword(String email, String password) async {
    signInCalls++;
    snapshot = const OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.submitting,
    );
    notifyListeners();
  }

  @override
  Future<void> signOut() async {}

  @override
  Future<void> requestSignInCode(String email) async {
    codeRequests++;
    snapshot = const OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.awaitingPasscode,
    );
    notifyListeners();
  }

  @override
  Future<void> signInWithCode(String email, String code) async {
    codeSignIns++;
  }

  void fail(String message) {
    snapshot = OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.failure,
      failureCode: 'invalid_credentials',
      failureMessage: message,
    );
    notifyListeners();
  }
}
