import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

void main() {
  const grant = PairedGrant(
    deviceRef: 'desktop:1',
    workspaceRef: 'local:w',
    accountRef: 'account:a',
    permissions: {PairedPermission.read},
    expiresAtMs: 100,
    generation: 2,
  );
  test('same account still requires pairing and workspace grant', () {
    const challenge = PairingChallenge('mobile-key', 'desktop-key', '123456');
    expect(challenge.confirm('000000'), isFalse);
    expect(challenge.confirm('123456'), isTrue);
    expect(
      () => grant.authorize(
        account: 'account:a',
        workspace: 'local:w',
        permission: PairedPermission.apply,
        nowMs: 1,
        expectedGeneration: 2,
        presence: DevicePresence.online,
      ),
      throwsStateError,
    );
  });
  test(
    'expired offline replaced and wrong generation fail without cloud fallback',
    () {
      for (final presence in [
        DevicePresence.offline,
        DevicePresence.sleeping,
        DevicePresence.replaced,
      ]) {
        expect(
          () => grant.authorize(
            account: 'account:a',
            workspace: 'local:w',
            permission: PairedPermission.read,
            nowMs: 1,
            expectedGeneration: 2,
            presence: presence,
          ),
          throwsStateError,
        );
      }
      expect(
        () => grant.authorize(
          account: 'account:a',
          workspace: 'local:w',
          permission: PairedPermission.read,
          nowMs: 100,
          expectedGeneration: 2,
          presence: DevicePresence.online,
        ),
        throwsStateError,
      );
    },
  );
  test('relay envelope contains ciphertext and routing metadata only', () {
    const envelope = RelayEnvelope(
      senderDeviceRef: 'mobile',
      recipientDeviceRef: 'desktop',
      ciphertext: [1, 2, 3],
    );
    expect(envelope.ciphertext, isNot(contains(123456)));
  });
}
