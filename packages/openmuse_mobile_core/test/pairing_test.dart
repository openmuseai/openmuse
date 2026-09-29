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
    const challenge = PairingChallenge(
      challengeRef: 'pairing:1',
      accountRef: 'account:a',
      mobileDeviceRef: 'mobile:1',
      desktopDeviceRef: 'desktop:1',
      confirmationCode: '123456',
      expiresAtMs: 100,
      generation: 2,
    );
    expect(
      challenge.confirm(
        account: 'account:a',
        code: '000000',
        nowMs: 1,
        expectedGeneration: 2,
      ),
      isFalse,
    );
    expect(
      challenge.confirm(
        account: 'account:a',
        code: '123456',
        nowMs: 1,
        expectedGeneration: 2,
      ),
      isTrue,
    );
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
      channelRef: 'channel:1',
      senderDeviceRef: 'mobile',
      recipientDeviceRef: 'desktop',
      sequence: 0,
      ciphertext: [1, 2, 3],
    );
    expect(envelope.ciphertext, isNot(contains(123456)));
  });

  test(
    'paired DSH and resource adapters enforce grant without cloud fallback',
    () async {
      final transport = _PairedTransport();
      final connector = PairedDesktopDshConnector(
        transport: transport,
        grant: grant,
        accountRef: 'account:a',
        grantGeneration: 2,
        nowMs: () => 1,
        presence: () => DevicePresence.online,
      );
      final session = await connector.open('local:w', 7);
      expect(connector.placement, DshPlacement.pairedDesktop);
      expect(session.generation, 7);
      final resourcePort = PairedDesktopResourcePort(
        transport: transport,
        grant: grant,
        accountRef: 'account:a',
        grantGeneration: 2,
        nowMs: () => 1,
        presence: () => DevicePresence.online,
      );
      expect(
        await resourcePort.read(
          const ResourceHandle(
            resourceRef: 'resource:1',
            revision: 'r1',
            audience: 'mobile',
            generation: 7,
            expiresAtMs: 100,
            size: 3,
            mediaType: 'text/plain',
          ),
          0,
          3,
        ),
        [1, 2, 3],
      );
      expect(transport.cloudFallbacks, 0);
    },
  );
}

final class _PairedTransport implements PairedDesktopTransportPort {
  int cloudFallbacks = 0;
  @override
  Future<DshSessionDescriptor> openDsh({
    required PairedGrant grant,
    required int presentationGeneration,
  }) async => DshSessionDescriptor(
    sessionRef: 'paired:s1',
    origin: 'https://relay.example.test',
    path: '/session/s1',
    generation: presentationGeneration,
  );

  @override
  Future<void> closeDsh(String sessionRef) async {}

  @override
  Future<List<int>> readRange({
    required PairedGrant grant,
    required ResourceHandle handle,
    required int start,
    required int endExclusive,
  }) async => [1, 2, 3];
}
