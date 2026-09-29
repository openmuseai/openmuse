import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'auth_models.dart';
import 'auth_ports.dart';

final class GoTrueAuthenticationController extends ChangeNotifier
    implements
        OpenMuseAuthenticationController,
        OpenMuseEmailCodeAuthenticationController {
  GoTrueAuthenticationController({
    required GoTrueAuthProvider provider,
    required AuthSessionStore store,
    CloudAccountBootstrapper bootstrapper =
        const NoopCloudAccountBootstrapper(),
    DateTime Function()? clock,
    this.refreshSkew = const Duration(minutes: 2),
  }) : _provider = provider,
       _store = store,
       _bootstrapper = bootstrapper,
       _clock = clock ?? DateTime.now;

  final GoTrueAuthProvider _provider;
  final AuthSessionStore _store;
  final CloudAccountBootstrapper _bootstrapper;
  final DateTime Function() _clock;
  final Duration refreshSkew;

  GoTrueSession? _session;
  Future<GoTrueSession>? _refreshInFlight;
  OpenMuseAuthenticationSnapshot _snapshot =
      const OpenMuseAuthenticationSnapshot.restoring();

  @override
  OpenMuseAuthenticationSnapshot get snapshot => _snapshot;

  @override
  Future<void> restore() async {
    _setSnapshot(const OpenMuseAuthenticationSnapshot.restoring());
    try {
      final restored = await _store.read();
      if (restored == null) {
        _session = null;
        _setSnapshot(const OpenMuseAuthenticationSnapshot.signedOut());
        return;
      }
      _session = restored;
      final usable = await _usableSession();
      await _bootstrap(usable);
      _publishAuthenticated(usable);
    } on AuthFailure catch (error) {
      await _clearSession();
      _publishFailure(error);
    } catch (_) {
      await _clearSession();
      _publishFailure(
        const AuthFailure(
          AuthFailureKind.invalidResponse,
          'The saved session could not be restored.',
        ),
      );
    }
  }

  @override
  Future<void> signInWithPassword(String email, String password) async {
    if (_snapshot.phase == OpenMuseAuthenticationPhase.submitting ||
        _snapshot.phase == OpenMuseAuthenticationPhase.bootstrapping) {
      return;
    }
    _setSnapshot(
      const OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.submitting,
      ),
    );
    try {
      final session = await _provider.signInWithPassword(email, password);
      _setSnapshot(
        const OpenMuseAuthenticationSnapshot(
          phase: OpenMuseAuthenticationPhase.bootstrapping,
        ),
      );
      await _bootstrap(session);
      await _store.write(session);
      _session = session;
      _publishAuthenticated(session);
    } on AuthFailure catch (error) {
      _publishFailure(error);
    } catch (_) {
      _publishFailure(
        const AuthFailure(
          AuthFailureKind.bootstrap,
          'The account workspace could not be initialized.',
        ),
      );
    }
  }

  @override
  Future<void> requestSignInCode(String email) async {
    if (_snapshot.phase == OpenMuseAuthenticationPhase.submitting) return;
    _setSnapshot(
      const OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.submitting,
      ),
    );
    try {
      await _provider.requestSignInCode(email);
      _setSnapshot(
        const OpenMuseAuthenticationSnapshot(
          phase: OpenMuseAuthenticationPhase.awaitingPasscode,
        ),
      );
    } on AuthFailure catch (error) {
      _publishFailure(error);
    } catch (_) {
      _publishFailure(
        const AuthFailure(
          AuthFailureKind.server,
          'The sign-in email could not be sent.',
        ),
      );
    }
  }

  @override
  Future<void> signInWithCode(String email, String code) async {
    if (_snapshot.phase == OpenMuseAuthenticationPhase.submitting ||
        _snapshot.phase == OpenMuseAuthenticationPhase.bootstrapping) {
      return;
    }
    _setSnapshot(
      const OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.submitting,
      ),
    );
    try {
      final session = await _provider.signInWithCode(email, code);
      _setSnapshot(
        const OpenMuseAuthenticationSnapshot(
          phase: OpenMuseAuthenticationPhase.bootstrapping,
        ),
      );
      await _bootstrap(session);
      await _store.write(session);
      _session = session;
      _publishAuthenticated(session);
    } on AuthFailure catch (error) {
      _publishFailure(error);
    } catch (_) {
      _publishFailure(
        const AuthFailure(
          AuthFailureKind.bootstrap,
          'The account workspace could not be initialized.',
        ),
      );
    }
  }

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async {
    if (_session == null) return null;
    try {
      final session = forceRefresh
          ? await _refreshSession()
          : await _usableSession();
      return session.accessToken;
    } on AuthFailure catch (error) {
      if (error.kind == AuthFailureKind.sessionExpired) {
        await _clearSession();
        _publishFailure(error);
      }
      rethrow;
    }
  }

  @override
  Future<void> signOut() async {
    final session = _session;
    try {
      if (session != null) await _provider.logout(session.accessToken);
    } catch (_) {
      // Local sign-out is authoritative; remote revocation is best effort.
    } finally {
      await _clearSession();
      _setSnapshot(const OpenMuseAuthenticationSnapshot.signedOut());
    }
  }

  Future<GoTrueSession> _usableSession() async {
    final session = _session;
    if (session == null) throw const AuthFailure.sessionExpired();
    if (!session.expiresAt.isAfter(_clock().toUtc().add(refreshSkew))) {
      return _refreshSession();
    }
    return session;
  }

  Future<GoTrueSession> _refreshSession() {
    final running = _refreshInFlight;
    if (running != null) return running;
    final session = _session;
    if (session == null) {
      return Future<GoTrueSession>.error(const AuthFailure.sessionExpired());
    }
    _setSnapshot(
      OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.refreshing,
        identity: OpenMuseAuthenticatedIdentity(
          subject: session.user.id,
          email: session.user.email,
        ),
      ),
    );
    late final Future<GoTrueSession> tracked;
    tracked = _provider
        .refresh(session.refreshToken)
        .then((next) async {
          await _store.write(next);
          _session = next;
          _publishAuthenticated(next);
          return next;
        })
        .whenComplete(() {
          if (identical(_refreshInFlight, tracked)) _refreshInFlight = null;
        });
    _refreshInFlight = tracked;
    return tracked;
  }

  Future<void> _bootstrap(GoTrueSession session) async {
    try {
      await _bootstrapper.bootstrap(session);
    } on AuthFailure {
      rethrow;
    } catch (_) {
      throw const AuthFailure(
        AuthFailureKind.bootstrap,
        'The account workspace could not be initialized.',
      );
    }
  }

  Future<void> _clearSession() async {
    _session = null;
    _refreshInFlight = null;
    await _store.delete();
  }

  void _publishAuthenticated(GoTrueSession session) {
    _setSnapshot(
      OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.authenticated,
        identity: OpenMuseAuthenticatedIdentity(
          subject: session.user.id,
          email: session.user.email,
        ),
      ),
    );
  }

  void _publishFailure(AuthFailure failure) {
    _setSnapshot(
      OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.failure,
        failureCode: failure.code,
        failureMessage: failure.safeMessage,
      ),
    );
  }

  void _setSnapshot(OpenMuseAuthenticationSnapshot value) {
    _snapshot = value;
    notifyListeners();
  }
}
