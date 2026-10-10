import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  final now = DateTime.utc(2026, 9, 29, 12);

  GoTrueSession session({
    String access = 'access-one',
    String refresh = 'refresh-one',
    Duration lifetime = const Duration(hours: 1),
  }) => GoTrueSession(
    accessToken: access,
    refreshToken: refresh,
    expiresAt: now.add(lifetime),
    user: const GoTrueUser(id: 'user-1', email: 'muse@example.com'),
  );

  test('anonymous mode opens a local session without a cloud token', () async {
    final provider = _FakeProvider();
    final store = _RecordingStore();
    final bootstrap = _RecordingBootstrapper();
    final controller = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      bootstrapper: bootstrap,
      clock: () => now,
    );

    await controller.signInAnonymously();

    expect(bootstrap.calls, 0);
    expect(provider.refreshCalls, 0);
    expect(store.value?.user.id, localAnonymousUserId);
    expect(controller.snapshot.isAuthenticated, isTrue);
    expect(await controller.accessToken(), isNull);

    final restored = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      bootstrapper: bootstrap,
      clock: () => now,
    );
    await restored.restore();
    expect(restored.snapshot.identity?.subject, localAnonymousUserId);
    expect(provider.refreshCalls, 0);
    expect(await restored.accessToken(forceRefresh: true), isNull);

    await restored.signOut();
    expect(store.value, isNull);
    expect(restored.snapshot.phase, OpenMuseAuthenticationPhase.signedOut);
  });

  test('password sign-in bootstraps before persisting', () async {
    final provider = _FakeProvider(signInResult: session());
    final store = _RecordingStore();
    final bootstrap = _RecordingBootstrapper();
    final controller = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      bootstrapper: bootstrap,
      clock: () => now,
    );

    await controller.signInWithPassword(' muse@example.com ', 'secret');

    expect(provider.signInEmail, ' muse@example.com ');
    expect(bootstrap.calls, 1);
    expect(store.writes, 1);
    expect(
      controller.snapshot.phase,
      OpenMuseAuthenticationPhase.authenticated,
    );
    expect(controller.snapshot.identity?.email, 'muse@example.com');
  });

  test('invalid credentials are typed and never persisted', () async {
    final provider = _FakeProvider(
      signInError: const AuthFailure.invalidCredentials(),
    );
    final store = _RecordingStore();
    final controller = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      clock: () => now,
    );

    await controller.signInWithPassword('muse@example.com', 'wrong');

    expect(controller.snapshot.failureCode, 'invalid_credentials');
    expect(store.writes, 0);
  });

  test(
    'email passcode flow requests code then persists verified session',
    () async {
      final provider = _FakeProvider(signInResult: session());
      final store = _RecordingStore();
      final controller = GoTrueAuthenticationController(
        provider: provider,
        store: store,
        clock: () => now,
      );

      await controller.requestSignInCode(' muse@example.com ');
      expect(provider.requestedCodeEmail, ' muse@example.com ');
      expect(
        controller.snapshot.phase,
        OpenMuseAuthenticationPhase.awaitingPasscode,
      );

      await controller.signInWithCode('muse@example.com', '123456');
      expect(provider.signInCode, '123456');
      expect(store.writes, 1);
      expect(controller.snapshot.isAuthenticated, isTrue);
    },
  );

  test('restore refreshes a near-expiry session and rotates store', () async {
    final old = session(lifetime: const Duration(seconds: 30));
    final fresh = session(access: 'access-two', refresh: 'refresh-two');
    final provider = _FakeProvider(refreshResult: fresh);
    final store = _RecordingStore(initial: old);
    final controller = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      clock: () => now,
    );

    await controller.restore();

    expect(provider.refreshCalls, 1);
    expect(store.value?.refreshToken, 'refresh-two');
    expect(await controller.accessToken(), 'access-two');
  });

  test('concurrent token callers share one refresh request', () async {
    final old = session(lifetime: const Duration(seconds: 30));
    final completer = Completer<GoTrueSession>();
    final provider = _FakeProvider(refreshFuture: completer.future);
    final store = _RecordingStore(initial: old);
    final controller = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      clock: () => now,
    );
    final restoring = controller.restore();
    await Future<void>.delayed(Duration.zero);

    final callers = List.generate(10, (_) => controller.accessToken());
    completer.complete(session(access: 'shared-access'));

    await restoring;
    expect(await Future.wait(callers), everyElement('shared-access'));
    expect(provider.refreshCalls, 1);
  });

  test(
    'refresh keeps the rotated token when secure storage rejects a write',
    () async {
      final provider = _FakeProvider(
        signInResult: session(),
        refreshResult: session(access: 'access-two', refresh: 'refresh-two'),
      );
      final controller = GoTrueAuthenticationController(
        provider: provider,
        store: _ThrowingWriteStore(),
        clock: () => now,
      );

      await controller.signInWithPassword('muse@example.com', 'secret');
      expect(await controller.accessToken(forceRefresh: true), 'access-two');
      expect(await controller.accessToken(), 'access-two');
      expect(
        controller.snapshot.phase,
        OpenMuseAuthenticationPhase.authenticated,
      );
    },
  );

  test(
    'temporary refresh failure restores authenticated state for retry',
    () async {
      final controller = GoTrueAuthenticationController(
        provider: _FakeProvider(
          signInResult: session(),
          refreshError: const AuthFailure(
            AuthFailureKind.network,
            'Cannot reach the authentication service.',
          ),
        ),
        store: _RecordingStore(),
        clock: () => now,
      );

      await controller.signInWithPassword('muse@example.com', 'secret');
      await expectLater(
        controller.accessToken(forceRefresh: true),
        throwsA(isA<AuthFailure>()),
      );
      expect(
        controller.snapshot.phase,
        OpenMuseAuthenticationPhase.authenticated,
      );
      expect(await controller.accessToken(), 'access-one');
    },
  );

  test('expired refresh clears local state', () async {
    final provider = _FakeProvider(
      refreshError: const AuthFailure.sessionExpired(),
    );
    final store = _RecordingStore(
      initial: session(lifetime: const Duration(seconds: 30)),
    );
    final controller = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      clock: () => now,
    );

    await controller.restore();

    expect(store.value, isNull);
    expect(controller.snapshot.failureCode, 'session_expired');
  });

  test(
    'temporary refresh outage during restore preserves the saved session',
    () async {
      final store = _RecordingStore(
        initial: session(lifetime: const Duration(seconds: 30)),
      );
      final controller = GoTrueAuthenticationController(
        provider: _FakeProvider(
          refreshError: const AuthFailure(
            AuthFailureKind.network,
            'Cannot reach the authentication service.',
          ),
        ),
        store: store,
        clock: () => now,
      );

      await controller.restore();

      expect(store.value?.refreshToken, 'refresh-one');
      expect(
        controller.snapshot.phase,
        OpenMuseAuthenticationPhase.authenticated,
      );
    },
  );

  test('restore exits restoring state when secure deletion fails', () async {
    final controller = GoTrueAuthenticationController(
      provider: _FakeProvider(),
      store: _FailingStore(),
      clock: () => now,
    );

    await controller.restore();

    expect(controller.snapshot.phase, OpenMuseAuthenticationPhase.failure);
    expect(controller.snapshot.failureCode, 'invalid_response');
  });

  test('local sign-out succeeds even if remote logout fails', () async {
    final provider = _FakeProvider(
      signInResult: session(),
      logoutError: const AuthFailure(
        AuthFailureKind.network,
        'Cannot reach authentication service.',
      ),
    );
    final store = _RecordingStore();
    final controller = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      clock: () => now,
    );
    await controller.signInWithPassword('muse@example.com', 'secret');

    await controller.signOut();

    expect(store.value, isNull);
    expect(controller.snapshot.phase, OpenMuseAuthenticationPhase.signedOut);
  });

  test('bootstrap failure does not persist the GoTrue session', () async {
    final store = _RecordingStore();
    final controller = GoTrueAuthenticationController(
      provider: _FakeProvider(signInResult: session()),
      store: store,
      bootstrapper: _RecordingBootstrapper(shouldFail: true),
      clock: () => now,
    );

    await controller.signInWithPassword('muse@example.com', 'secret');

    expect(store.writes, 0);
    expect(controller.snapshot.failureCode, 'bootstrap');
  });

  test('keychain write failure still completes sign-in', () async {
    final controller = GoTrueAuthenticationController(
      provider: _FakeProvider(signInResult: session()),
      store: _ThrowingWriteStore(),
      clock: () => now,
    );

    await controller.signInWithPassword('muse@example.com', 'secret');

    expect(
      controller.snapshot.phase,
      OpenMuseAuthenticationPhase.authenticated,
    );
    expect(await controller.accessToken(), 'access-one');
  });

  test(
    'signup waits for email verification before creating a session',
    () async {
      final provider = _FakeProvider(signInResult: session());
      final store = _RecordingStore();
      final bootstrap = _RecordingBootstrapper();
      final controller = GoTrueAuthenticationController(
        provider: provider,
        store: store,
        bootstrapper: bootstrap,
        clock: () => now,
      );

      await controller.signUp('muse@example.com', 'long-password');
      expect(
        controller.snapshot.phase,
        OpenMuseAuthenticationPhase.awaitingPasscode,
      );
      expect(store.writes, 0);
      expect(bootstrap.calls, 0);
      await controller.resendSignUpCode('muse@example.com');
      expect(provider.signupResends, 1);

      await controller.verifySignUpCode('muse@example.com', '123456');
      expect(
        controller.snapshot.phase,
        OpenMuseAuthenticationPhase.authenticated,
      );
      expect(store.writes, 1);
      expect(bootstrap.calls, 1);
    },
  );

  test(
    'auto-confirming GoTrue still requires a delivered email code',
    () async {
      final issued = session();
      final provider = _FakeProvider(signInResult: issued)
        ..signUpResult = GoTrueSignUpResult(user: issued.user, session: issued);
      final store = _RecordingStore();
      final controller = GoTrueAuthenticationController(
        provider: provider,
        store: store,
        clock: () => now,
      );

      await controller.signUp('muse@example.com', 'long-password');
      expect(provider.requestedCodeEmail, 'muse@example.com');
      expect(
        controller.snapshot.phase,
        OpenMuseAuthenticationPhase.awaitingPasscode,
      );
      expect(store.writes, 0);
      await controller.verifySignUpCode('muse@example.com', '123456');
      expect(provider.signInCode, '123456');
      expect(store.writes, 1);
    },
  );

  test('recovery credential stays transient until password changes', () async {
    final provider = _FakeProvider(signInResult: session());
    final store = _RecordingStore();
    final controller = GoTrueAuthenticationController(
      provider: provider,
      store: store,
      clock: () => now,
    );

    await controller.requestPasswordRecovery('muse@example.com');
    await controller.verifyRecoveryCode('muse@example.com', '123456');
    expect(
      controller.snapshot.phase,
      OpenMuseAuthenticationPhase.awaitingPasswordReset,
    );
    expect(store.writes, 0);
    expect(await controller.accessToken(), isNull);

    await controller.resetPassword('new-long-password');
    expect(provider.updatedPassword, 'new-long-password');
    expect(provider.signInEmail, 'muse@example.com');
    expect(store.writes, 1);
    expect(
      controller.snapshot.phase,
      OpenMuseAuthenticationPhase.authenticated,
    );
  });
}

final class _ThrowingWriteStore implements AuthSessionStore {
  @override
  Future<void> delete() async {}

  @override
  Future<GoTrueSession?> read() async => null;

  @override
  Future<void> write(GoTrueSession session) =>
      Future<void>.error(StateError('keychain denied'));
}

final class _FailingStore implements AuthSessionStore {
  @override
  Future<void> delete() => Future<void>.error(StateError('keychain denied'));
  @override
  Future<GoTrueSession?> read() =>
      Future<GoTrueSession?>.error(StateError('keychain denied'));
  @override
  Future<void> write(GoTrueSession session) async {}
}

final class _RecordingStore implements AuthSessionStore {
  _RecordingStore({GoTrueSession? initial}) : value = initial;

  GoTrueSession? value;
  int writes = 0;

  @override
  Future<void> delete() async => value = null;
  @override
  Future<GoTrueSession?> read() async => value;
  @override
  Future<void> write(GoTrueSession session) async {
    writes++;
    value = session;
  }
}

final class _RecordingBootstrapper implements CloudAccountBootstrapper {
  _RecordingBootstrapper({this.shouldFail = false});
  final bool shouldFail;
  int calls = 0;

  @override
  Future<void> bootstrap(GoTrueSession session) async {
    calls++;
    if (shouldFail) throw StateError('not ready');
  }
}

final class _FakeProvider implements GoTrueAuthProvider {
  _FakeProvider({
    this.signInResult,
    this.signInError,
    this.refreshResult,
    this.refreshFuture,
    this.refreshError,
    this.logoutError,
  });

  final GoTrueSession? signInResult;
  final Object? signInError;
  final GoTrueSession? refreshResult;
  final Future<GoTrueSession>? refreshFuture;
  final Object? refreshError;
  final Object? logoutError;
  int refreshCalls = 0;
  String? signInEmail;
  String? requestedCodeEmail;
  String? signInCode;
  GoTrueSignUpResult? signUpResult;
  GoTrueSession? recoveryResult;
  String? updatedPassword;
  int signupResends = 0;

  @override
  Future<GoTrueSignUpResult> signUp(String email, String password) async =>
      signUpResult ??
      GoTrueSignUpResult(
        user: const GoTrueUser(id: 'user-1', email: 'muse@example.com'),
      );

  @override
  Future<GoTrueSession> verifySignUpCode(String email, String code) async =>
      signInResult!;

  @override
  Future<void> resendSignUpCode(String email) async {
    signupResends++;
  }

  @override
  Future<void> requestPasswordRecovery(String email) async {}

  @override
  Future<GoTrueSession> verifyRecoveryCode(String email, String code) async =>
      recoveryResult ?? signInResult!;

  @override
  Future<void> updatePassword(String accessToken, String password) async {
    updatedPassword = password;
  }

  @override
  Future<GoTrueUser> currentUser(String accessToken) async =>
      const GoTrueUser(id: 'user-1', email: 'muse@example.com');
  @override
  Future<void> logout(String accessToken) async {
    if (logoutError != null) throw logoutError!;
  }

  @override
  Future<GoTrueSession> refresh(String refreshToken) async {
    refreshCalls++;
    if (refreshError != null) throw refreshError!;
    if (refreshFuture != null) return refreshFuture!;
    return refreshResult!;
  }

  @override
  Future<GoTrueSession> signInWithPassword(
    String email,
    String password,
  ) async {
    signInEmail = email;
    if (signInError != null) throw signInError!;
    return signInResult!;
  }

  @override
  Future<void> requestSignInCode(String email) async {
    requestedCodeEmail = email;
  }

  @override
  Future<GoTrueSession> signInWithCode(String email, String code) async {
    signInCode = code;
    if (signInError != null) throw signInError!;
    return signInResult!;
  }

  @override
  Future<GoTrueSettings> settings() async =>
      const GoTrueSettings(emailEnabled: true, providers: {});
}
