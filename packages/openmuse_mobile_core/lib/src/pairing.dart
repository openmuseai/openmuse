import 'dsh_connector.dart';
import 'resource_client.dart';

enum PairedPermission { read, propose, apply }

enum DevicePresence { online, offline, sleeping, replaced }

final class PairedGrant {
  const PairedGrant({
    required this.deviceRef,
    required this.workspaceRef,
    required this.accountRef,
    required this.permissions,
    required this.expiresAtMs,
    required this.generation,
  });
  final String deviceRef, workspaceRef, accountRef;
  final Set<PairedPermission> permissions;
  final int expiresAtMs, generation;
  void authorize({
    required String account,
    required String workspace,
    required PairedPermission permission,
    required int nowMs,
    required int expectedGeneration,
    required DevicePresence presence,
  }) {
    if (account != accountRef ||
        workspace != workspaceRef ||
        !permissions.contains(permission) ||
        nowMs >= expiresAtMs ||
        expectedGeneration != generation ||
        presence != DevicePresence.online)
      throw StateError('paired grant denied');
  }
}

final class PairingChallenge {
  const PairingChallenge({
    required this.challengeRef,
    required this.accountRef,
    required this.mobileDeviceRef,
    required this.desktopDeviceRef,
    required this.confirmationCode,
    required this.expiresAtMs,
    required this.generation,
  });
  final String challengeRef;
  final String accountRef;
  final String mobileDeviceRef;
  final String desktopDeviceRef;
  final String confirmationCode;
  final int expiresAtMs;
  final int generation;

  bool confirm({
    required String account,
    required String code,
    required int nowMs,
    required int expectedGeneration,
  }) =>
      account == accountRef &&
      code == confirmationCode &&
      nowMs < expiresAtMs &&
      expectedGeneration == generation &&
      challengeRef.isNotEmpty &&
      mobileDeviceRef.isNotEmpty &&
      desktopDeviceRef.isNotEmpty;
}

final class RelayEnvelope {
  const RelayEnvelope({
    required this.channelRef,
    required this.senderDeviceRef,
    required this.recipientDeviceRef,
    required this.sequence,
    required this.ciphertext,
  });
  final String channelRef, senderDeviceRef, recipientDeviceRef;
  final int sequence;
  final List<int> ciphertext;
}

/// Encrypted paired transport implemented by the Rust paired-relay core and a
/// platform bridge. Flutter never receives device private keys or plaintext
/// relay frames outside the requested capability response.
abstract interface class PairedDesktopTransportPort {
  Future<DshSessionDescriptor> openDsh({
    required PairedGrant grant,
    required int presentationGeneration,
  });
  Future<void> closeDsh(String sessionRef);
  Future<List<int>> readRange({
    required PairedGrant grant,
    required ResourceHandle handle,
    required int start,
    required int endExclusive,
  });
}

final class PairedDesktopDshConnector implements DshRuntimeConnector {
  const PairedDesktopDshConnector({
    required this.transport,
    required this.grant,
    required this.accountRef,
    required this.grantGeneration,
    required this.nowMs,
    required this.presence,
  });
  final PairedDesktopTransportPort transport;
  final PairedGrant grant;
  final String accountRef;
  final int grantGeneration;
  final int Function() nowMs;
  final DevicePresence Function() presence;

  @override
  DshPlacement get placement => DshPlacement.pairedDesktop;

  @override
  Future<DshSessionDescriptor> open(String workspaceRef, int generation) async {
    grant.authorize(
      account: accountRef,
      workspace: workspaceRef,
      permission: PairedPermission.read,
      nowMs: nowMs(),
      expectedGeneration: grantGeneration,
      presence: presence(),
    );
    final value = await transport.openDsh(
      grant: grant,
      presentationGeneration: generation,
    );
    if (value.generation != generation) {
      throw StateError('paired DSH generation mismatch');
    }
    return value;
  }

  @override
  Future<void> close(String sessionRef) => transport.closeDsh(sessionRef);
}

final class PairedDesktopResourcePort implements ResourceRangePort {
  const PairedDesktopResourcePort({
    required this.transport,
    required this.grant,
    required this.accountRef,
    required this.grantGeneration,
    required this.nowMs,
    required this.presence,
  });
  final PairedDesktopTransportPort transport;
  final PairedGrant grant;
  final String accountRef;
  final int grantGeneration;
  final int Function() nowMs;
  final DevicePresence Function() presence;

  @override
  Future<List<int>> read(ResourceHandle handle, int start, int endExclusive) {
    grant.authorize(
      account: accountRef,
      workspace: grant.workspaceRef,
      permission: PairedPermission.read,
      nowMs: nowMs(),
      expectedGeneration: grantGeneration,
      presence: presence(),
    );
    return transport.readRange(
      grant: grant,
      handle: handle,
      start: start,
      endExclusive: endExclusive,
    );
  }
}
