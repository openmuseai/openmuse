import 'dart:typed_data';

import 'dsh_connector.dart';
import 'resource_client.dart';

enum PairedPermission { read, propose, apply }

enum DevicePresence { online, offline, sleeping, replaced }

final class DeviceKeyDescriptor {
  const DeviceKeyDescriptor({
    required this.keyRef,
    required this.storage,
    required this.hardwareBacked,
    required this.created,
  });
  final String keyRef;
  final String storage;
  final bool hardwareBacked;
  final bool created;
}

final class DevicePublicIdentity {
  DevicePublicIdentity({
    required Uint8List signingPublic,
    required Uint8List agreementPublic,
  }) : signingPublic = Uint8List.fromList(signingPublic),
       agreementPublic = Uint8List.fromList(agreementPublic) {
    if (this.signingPublic.length != 32 || this.agreementPublic.length != 32) {
      throw ArgumentError('device public keys must be 32 bytes');
    }
  }

  final Uint8List signingPublic;
  final Uint8List agreementPublic;
}

final class SignedDeviceOffer {
  SignedDeviceOffer({
    required this.accountRef,
    required this.deviceRef,
    required Uint8List signingPublic,
    required Uint8List agreementPublic,
    required Uint8List nonce,
    required this.registrationGeneration,
    required Uint8List signature,
  }) : signingPublic = Uint8List.fromList(signingPublic),
       agreementPublic = Uint8List.fromList(agreementPublic),
       nonce = Uint8List.fromList(nonce),
       signature = Uint8List.fromList(signature) {
    if (accountRef.isEmpty ||
        deviceRef.isEmpty ||
        this.signingPublic.length != 32 ||
        this.agreementPublic.length != 32 ||
        this.nonce.length != 32 ||
        this.signature.length != 64 ||
        registrationGeneration <= 0) {
      throw ArgumentError('invalid signed device offer');
    }
  }

  final String accountRef;
  final String deviceRef;
  final Uint8List signingPublic;
  final Uint8List agreementPublic;
  final Uint8List nonce;
  final int registrationGeneration;
  final Uint8List signature;

  Map<String, Object> toJson() => {
    'accountRef': accountRef,
    'deviceRef': deviceRef,
    'signingPublic': signingPublic.toList(growable: false),
    'agreementPublic': agreementPublic.toList(growable: false),
    'nonce': nonce.toList(growable: false),
    'registrationGeneration': registrationGeneration,
    'signature': signature.toList(growable: false),
  };
}

final class TrustedDeviceRegistration {
  TrustedDeviceRegistration({
    required this.accountRef,
    required this.deviceRef,
    required Uint8List signingPublic,
    required Uint8List agreementPublic,
    required this.generation,
    required this.revoked,
  }) : signingPublic = Uint8List.fromList(signingPublic),
       agreementPublic = Uint8List.fromList(agreementPublic) {
    if (accountRef.isEmpty ||
        deviceRef.isEmpty ||
        this.signingPublic.length != 32 ||
        this.agreementPublic.length != 32 ||
        generation <= 0) {
      throw ArgumentError('invalid trusted device registration');
    }
  }

  factory TrustedDeviceRegistration.fromOffer(SignedDeviceOffer offer) =>
      TrustedDeviceRegistration(
        accountRef: offer.accountRef,
        deviceRef: offer.deviceRef,
        signingPublic: offer.signingPublic,
        agreementPublic: offer.agreementPublic,
        generation: offer.registrationGeneration,
        revoked: false,
      );

  final String accountRef;
  final String deviceRef;
  final Uint8List signingPublic;
  final Uint8List agreementPublic;
  final int generation;
  final bool revoked;

  Map<String, Object> toJson() => {
    'accountRef': accountRef,
    'deviceRef': deviceRef,
    'signingPublic': signingPublic.toList(growable: false),
    'agreementPublic': agreementPublic.toList(growable: false),
    'generation': generation,
    'revoked': revoked,
  };
}

final class NativePairingHandshake {
  const NativePairingHandshake({
    required this.handle,
    required this.confirmationCode,
  });
  final int handle;
  final String confirmationCode;
}

final class NativePairedChannel {
  const NativePairedChannel({required this.handle, required this.channelRef});
  final int handle;
  final String channelRef;
}

/// Platform-owned device-key storage. No method returns seed/private material.
/// Cryptographic operations must be added behind the same native boundary.
abstract interface class DeviceKeyStorePort {
  Future<DeviceKeyDescriptor> ensure({
    required String accountRef,
    required String deviceRef,
  });
  Future<DevicePublicIdentity> publicIdentity(String keyRef);
  Future<SignedDeviceOffer> issueOffer({
    required String keyRef,
    required String accountRef,
    required String deviceRef,
    required int registrationGeneration,
  });
  Future<void> delete(String keyRef);
}

abstract interface class PairedCryptoPort {
  Future<NativePairingHandshake> beginHandshake({
    required String keyRef,
    required SignedDeviceOffer localOffer,
    required SignedDeviceOffer remoteOffer,
    required TrustedDeviceRegistration localRegistration,
    required TrustedDeviceRegistration remoteRegistration,
  });
  Future<NativePairedChannel> confirmHandshake({
    required int handshakeHandle,
    required String confirmationCode,
  });
  Future<RelayEnvelope> seal({
    required int channelHandle,
    required List<int> plaintext,
  });
  Future<Uint8List> open({
    required int channelHandle,
    required RelayEnvelope envelope,
  });
  Future<void> close(int handle);
}

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
