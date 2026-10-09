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

  testWidgets('renders Simplified Chinese when the locale is zh', (
    tester,
  ) async {
    await tester.pumpWidget(
      _app(_FakeController(), locale: const Locale('zh')),
    );

    expect(find.text('欢迎使用 OpenMuse'), findsOneWidget);
    expect(find.text('使用密码继续'), findsOneWidget);
    expect(find.textContaining('服务条款'), findsOneWidget);
  });

  testWidgets('localizes a known failure code in Chinese', (tester) async {
    final controller = _FakeController();
    await tester.pumpWidget(_app(controller, locale: const Locale('zh')));
    await tester.enterText(
      find.byKey(const ValueKey('auth.email')),
      'muse@example.com',
    );
    await tester.tap(find.byKey(const ValueKey('auth.continue-password')));
    await tester.pumpAndSettle();
    controller.fail('The email or password is incorrect.');
    await tester.pump();

    expect(find.text('邮箱或密码不正确。'), findsOneWidget);
  });

  testWidgets('keeps the controller message for an unmapped failure code', (
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
    controller.fail('Something new happened.', code: 'unmapped_code');
    await tester.pump();

    expect(find.text('Something new happened.'), findsOneWidget);
  });

  testWidgets('signup validates password and opens confirmation step', (
    tester,
  ) async {
    final controller = _FakeController();
    await tester.pumpWidget(_app(controller));
    await tester.enterText(
      find.byKey(const ValueKey('auth.email')),
      'muse@example.com',
    );
    await tester.tap(find.byKey(const ValueKey('auth.sign-up')));
    await tester.pump();

    await tester.enterText(
      find.byKey(const ValueKey('auth.new-password')),
      'short',
    );
    await tester.enterText(
      find.byKey(const ValueKey('auth.confirm-password')),
      'short',
    );
    await tester.tap(find.byKey(const ValueKey('auth.submit-new-password')));
    await tester.pump();
    expect(find.text('Use at least 8 characters.'), findsOneWidget);
    expect(controller.signUps, 0);

    await tester.enterText(
      find.byKey(const ValueKey('auth.new-password')),
      'long-password',
    );
    await tester.enterText(
      find.byKey(const ValueKey('auth.confirm-password')),
      'long-password',
    );
    await tester.tap(find.byKey(const ValueKey('auth.submit-new-password')));
    await tester.pump();
    expect(controller.signUps, 1);
    expect(find.byKey(const ValueKey('auth.passcode')), findsOneWidget);
    await tester.enterText(
      find.byKey(const ValueKey('auth.passcode')),
      '123456',
    );
    await tester.tap(find.byKey(const ValueKey('auth.submit-passcode')));
    expect(controller.signupVerifications, 1);
  });

  testWidgets('create account opens a distinct form before entering email', (
    tester,
  ) async {
    final controller = _FakeController();
    await tester.pumpWidget(_app(controller));
    await tester.tap(find.byKey(const ValueKey('auth.sign-up')));
    await tester.pump();

    expect(find.text('Create your account'), findsOneWidget);
    expect(find.byKey(const ValueKey('auth.signup-email')), findsOneWidget);
    expect(find.byKey(const ValueKey('auth.continue-password')), findsNothing);
    await tester.tap(find.byKey(const ValueKey('auth.submit-new-password')));
    await tester.pump();
    expect(find.text('Please enter a valid email address.'), findsOneWidget);
    expect(controller.signUps, 0);
  });

  testWidgets('forgot password requests recovery code and new password', (
    tester,
  ) async {
    final controller = _FakeController();
    await tester.pumpWidget(_app(controller));
    await tester.enterText(
      find.byKey(const ValueKey('auth.email')),
      'muse@example.com',
    );
    await tester.tap(find.byKey(const ValueKey('auth.continue-password')));
    await tester.pump();
    await tester.tap(find.text('Forgot password?'));
    await tester.pump();
    expect(controller.recoveryRequests, 1);
    await tester.enterText(
      find.byKey(const ValueKey('auth.passcode')),
      '123456',
    );
    await tester.tap(find.byKey(const ValueKey('auth.submit-passcode')));
    await tester.pump();
    expect(find.byKey(const ValueKey('auth.new-password')), findsOneWidget);
  });
}

Widget _app(_FakeController controller, {Locale? locale}) => MaterialApp(
  locale: locale,
  localizationsDelegates: OpenMuseAuthLocalizations.localizationsDelegates,
  supportedLocales: OpenMuseAuthLocalizations.supportedLocales,
  home: OpenMuseLoginScreen(
    authentication: controller,
    cloudLabel: 'http://127.0.0.1:8000',
  ),
);

final class _FakeController extends ChangeNotifier
    implements
        OpenMuseAuthenticationController,
        OpenMuseEmailCodeAuthenticationController,
        OpenMuseAccountAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot snapshot =
      const OpenMuseAuthenticationSnapshot.signedOut();
  int signInCalls = 0;
  int codeRequests = 0;
  int codeSignIns = 0;
  int signUps = 0;
  int signupVerifications = 0;
  int recoveryRequests = 0;

  @override
  Future<void> signUp(String email, String password) async {
    signUps++;
    snapshot = const OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.awaitingPasscode,
    );
    notifyListeners();
  }

  @override
  Future<void> verifySignUpCode(String email, String code) async {
    signupVerifications++;
  }

  @override
  Future<void> resendSignUpCode(String email) async {}

  @override
  Future<void> requestPasswordRecovery(String email) async {
    recoveryRequests++;
    snapshot = const OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.awaitingPasscode,
    );
    notifyListeners();
  }

  @override
  Future<void> verifyRecoveryCode(String email, String code) async {
    snapshot = const OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.awaitingPasswordReset,
    );
    notifyListeners();
  }

  @override
  Future<void> resetPassword(String password) async {}

  @override
  void cancelPendingFlow() {
    snapshot = const OpenMuseAuthenticationSnapshot.signedOut();
    notifyListeners();
  }

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

  void fail(String message, {String code = 'invalid_credentials'}) {
    snapshot = OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.failure,
      failureCode: code,
      failureMessage: message,
    );
    notifyListeners();
  }
}
