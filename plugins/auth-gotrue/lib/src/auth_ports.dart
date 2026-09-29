import 'auth_models.dart';

abstract interface class GoTrueAuthProvider {
  Future<GoTrueSession> signInWithPassword(String email, String password);
  Future<GoTrueSession> refresh(String refreshToken);
  Future<GoTrueUser> currentUser(String accessToken);
  Future<void> logout(String accessToken);
  Future<GoTrueSettings> settings();
}

abstract interface class AuthSessionStore {
  Future<GoTrueSession?> read();
  Future<void> write(GoTrueSession session);
  Future<void> delete();
}

abstract interface class CloudAccountBootstrapper {
  Future<void> bootstrap(GoTrueSession session);
}

final class NoopCloudAccountBootstrapper implements CloudAccountBootstrapper {
  const NoopCloudAccountBootstrapper();

  @override
  Future<void> bootstrap(GoTrueSession session) async {}
}

final class MemoryAuthSessionStore implements AuthSessionStore {
  GoTrueSession? _session;

  @override
  Future<void> delete() async => _session = null;

  @override
  Future<GoTrueSession?> read() async => _session;

  @override
  Future<void> write(GoTrueSession session) async => _session = session;
}
