import 'package:flutter/widgets.dart';

enum OpenMuseAuthenticationPhase {
  restoring,
  signedOut,
  submitting,
  bootstrapping,
  awaitingPasscode,
  authenticated,
  refreshing,
  failure,
}

@immutable
final class OpenMuseAuthenticatedIdentity {
  const OpenMuseAuthenticatedIdentity({
    required this.subject,
    required this.email,
  });

  final String subject;
  final String email;
}

@immutable
final class OpenMuseAuthenticationSnapshot {
  const OpenMuseAuthenticationSnapshot({
    required this.phase,
    this.identity,
    this.failureCode,
    this.failureMessage,
  });

  const OpenMuseAuthenticationSnapshot.restoring()
    : this(phase: OpenMuseAuthenticationPhase.restoring);

  const OpenMuseAuthenticationSnapshot.signedOut()
    : this(phase: OpenMuseAuthenticationPhase.signedOut);

  final OpenMuseAuthenticationPhase phase;
  final OpenMuseAuthenticatedIdentity? identity;
  final String? failureCode;
  final String? failureMessage;

  bool get isAuthenticated =>
      phase == OpenMuseAuthenticationPhase.authenticated ||
      phase == OpenMuseAuthenticationPhase.refreshing;
}

abstract interface class OpenMuseAuthenticationController
    implements Listenable {
  OpenMuseAuthenticationSnapshot get snapshot;

  Future<void> restore();

  Future<void> signInWithPassword(String email, String password);

  Future<String?> accessToken({bool forceRefresh = false});

  Future<void> signOut();
}

/// Optional email magic-link/passcode capability. Hosts and login surfaces
/// feature-detect it instead of forcing every authentication provider to
/// implement GoTrue-specific flows.
abstract interface class OpenMuseEmailCodeAuthenticationController {
  Future<void> requestSignInCode(String email);

  Future<void> signInWithCode(String email, String code);
}

/// Optional, privileged contribution supplied by a built-in authentication
/// plugin. Hosts select it explicitly when more than one provider is active.
abstract interface class OpenMuseAuthenticationContributor {
  OpenMuseAuthenticationController get authentication;

  Widget buildAuthenticationGate(
    BuildContext context, {
    required Widget authenticatedChild,
  });
}
