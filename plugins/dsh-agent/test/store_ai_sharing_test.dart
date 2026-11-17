import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/src/dsh_sidecar.dart';

void main() {
  test(
    'Store does not launch DSH or permit generation before AI sharing consent',
    () async {
      final supervisor = DshSidecarSupervisor(environment: {});
      expect(DshSidecarSupervisor.requiresAiSharingConsent, isTrue);
      expect(supervisor.aiSharingAllowed, isFalse);
      expect(supervisor.requireAiSharing, throwsStateError);
      await supervisor.ensureStarted();
      expect(supervisor.launchCount, 0);
      expect(supervisor.state, DshSidecarState.stopped);
      supervisor.allowAiSharing();
      expect(supervisor.requireAiSharing, returnsNormally);
      await supervisor.revokeAiSharing();
      expect(supervisor.aiSharingAllowed, isFalse);
      expect(supervisor.requireAiSharing, throwsStateError);
      await supervisor.ensureStarted();
      expect(supervisor.launchCount, 0);
      supervisor.dispose();
    },
    skip: !const bool.fromEnvironment('OPENMUSE_MAC_APP_STORE'),
  );
}
