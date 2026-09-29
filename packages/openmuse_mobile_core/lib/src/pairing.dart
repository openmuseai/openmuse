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
  const PairingChallenge(
    this.mobileKey,
    this.desktopKey,
    this.confirmationCode,
  );
  final String mobileKey, desktopKey, confirmationCode;
  bool confirm(String code) =>
      code == confirmationCode && mobileKey.isNotEmpty && desktopKey.isNotEmpty;
}

final class RelayEnvelope {
  const RelayEnvelope({
    required this.senderDeviceRef,
    required this.recipientDeviceRef,
    required this.ciphertext,
  });
  final String senderDeviceRef, recipientDeviceRef;
  final List<int> ciphertext;
}
