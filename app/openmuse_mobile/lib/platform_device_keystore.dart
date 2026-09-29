import 'dart:convert';

import 'package:flutter/services.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

final class PlatformDeviceKeyStore implements DeviceKeyStorePort {
  const PlatformDeviceKeyStore({
    this.channel = const MethodChannel('io.openmuse/device_keystore'),
  });

  final MethodChannel channel;

  @override
  Future<DeviceKeyDescriptor> ensure({
    required String accountRef,
    required String deviceRef,
  }) async {
    _validateRef(accountRef);
    _validateRef(deviceRef);
    final value = await channel.invokeMapMethod<String, Object?>('ensure', {
      'accountRef': accountRef,
      'deviceRef': deviceRef,
    });
    if (value == null ||
        value.keys.any(
          (key) => const {'seed', 'privateKey', 'secret'}.contains(key),
        )) {
      throw StateError('invalid device keystore response');
    }
    final keyRef = value['keyRef'];
    final storage = value['storage'];
    final hardwareBacked = value['hardwareBacked'];
    final created = value['created'];
    if (keyRef is! String ||
        keyRef.isEmpty ||
        storage is! String ||
        storage.isEmpty ||
        hardwareBacked is! bool ||
        created is! bool) {
      throw StateError('invalid device keystore descriptor');
    }
    return DeviceKeyDescriptor(
      keyRef: keyRef,
      storage: storage,
      hardwareBacked: hardwareBacked,
      created: created,
    );
  }

  @override
  Future<DevicePublicIdentity> publicIdentity(String keyRef) async {
    _validateRef(keyRef);
    final value = await channel.invokeMapMethod<String, Object?>(
      'publicIdentity',
      {'keyRef': keyRef},
    );
    if (value == null ||
        value.keys.any(
          (key) => const {'seed', 'privateKey', 'secret'}.contains(key),
        )) {
      throw StateError('invalid device public identity response');
    }
    final signingPublic = value['signingPublic'];
    final agreementPublic = value['agreementPublic'];
    if (signingPublic is! Uint8List || agreementPublic is! Uint8List) {
      throw StateError('invalid device public identity');
    }
    return DevicePublicIdentity(
      signingPublic: signingPublic,
      agreementPublic: agreementPublic,
    );
  }

  @override
  Future<SignedDeviceOffer> issueOffer({
    required String keyRef,
    required String accountRef,
    required String deviceRef,
    required int registrationGeneration,
  }) async {
    _validateRef(keyRef);
    _validateRef(accountRef);
    _validateRef(deviceRef);
    if (registrationGeneration <= 0) {
      throw ArgumentError('registration generation must be positive');
    }
    final encoded = await channel.invokeMethod<String>('issueOffer', {
      'keyRef': keyRef,
      'accountRef': accountRef,
      'deviceRef': deviceRef,
      'registrationGeneration': registrationGeneration,
    });
    if (encoded == null) {
      throw StateError('invalid signed device offer response');
    }
    final value = jsonDecode(encoded);
    if (value is! Map<String, Object?> ||
        value.keys.toSet().difference(const {
          'accountRef',
          'deviceRef',
          'signingPublic',
          'agreementPublic',
          'nonce',
          'registrationGeneration',
          'signature',
        }).isNotEmpty) {
      throw StateError('invalid signed device offer');
    }
    try {
      return SignedDeviceOffer(
        accountRef: value['accountRef']! as String,
        deviceRef: value['deviceRef']! as String,
        signingPublic: _jsonBytes(value['signingPublic']),
        agreementPublic: _jsonBytes(value['agreementPublic']),
        nonce: _jsonBytes(value['nonce']),
        registrationGeneration: value['registrationGeneration']! as int,
        signature: _jsonBytes(value['signature']),
      );
    } on Object {
      throw StateError('invalid signed device offer');
    }
  }

  @override
  Future<void> delete(String keyRef) async {
    _validateRef(keyRef);
    await channel.invokeMethod<void>('delete', {'keyRef': keyRef});
  }

  static void _validateRef(String value) {
    if (value.isEmpty || value.length > 256 || value.contains('\u0000')) {
      throw ArgumentError('invalid device keystore ref');
    }
  }

  static Uint8List _jsonBytes(Object? value) {
    if (value is! List<Object?> ||
        value.any((byte) => byte is! int || byte < 0 || byte > 255)) {
      throw const FormatException('invalid byte array');
    }
    return Uint8List.fromList(value.cast<int>());
  }
}

final class PlatformPairedCrypto implements PairedCryptoPort {
  const PlatformPairedCrypto({
    this.channel = const MethodChannel('io.openmuse/device_keystore'),
  });

  final MethodChannel channel;

  @override
  Future<NativePairingHandshake> beginHandshake({
    required String keyRef,
    required SignedDeviceOffer localOffer,
    required SignedDeviceOffer remoteOffer,
    required TrustedDeviceRegistration localRegistration,
    required TrustedDeviceRegistration remoteRegistration,
  }) async {
    final encoded = await channel.invokeMethod<String>('beginHandshake', {
      'keyRef': keyRef,
      'localOfferJson': jsonEncode(localOffer.toJson()),
      'remoteOfferJson': jsonEncode(remoteOffer.toJson()),
      'localRegistrationJson': jsonEncode(localRegistration.toJson()),
      'remoteRegistrationJson': jsonEncode(remoteRegistration.toJson()),
    });
    final value = _jsonObject(encoded, const {
      'handshakeHandle',
      'confirmationCode',
    });
    final handle = value['handshakeHandle'];
    final code = value['confirmationCode'];
    if (handle is! int ||
        handle <= 0 ||
        code is! String ||
        !RegExp(r'^[0-9]{6}$').hasMatch(code)) {
      throw StateError('invalid native handshake descriptor');
    }
    return NativePairingHandshake(handle: handle, confirmationCode: code);
  }

  @override
  Future<NativePairedChannel> confirmHandshake({
    required int handshakeHandle,
    required String confirmationCode,
  }) async {
    _validateHandle(handshakeHandle);
    if (!RegExp(r'^[0-9]{6}$').hasMatch(confirmationCode)) {
      throw ArgumentError('invalid confirmation code');
    }
    final encoded = await channel.invokeMethod<String>('confirmHandshake', {
      'handshakeHandle': handshakeHandle,
      'confirmationCode': confirmationCode,
    });
    final value = _jsonObject(encoded, const {'channelHandle', 'channelRef'});
    final handle = value['channelHandle'];
    final channelRef = value['channelRef'];
    if (handle is! int ||
        handle <= 0 ||
        channelRef is! String ||
        !channelRef.startsWith('paired:')) {
      throw StateError('invalid native channel descriptor');
    }
    return NativePairedChannel(handle: handle, channelRef: channelRef);
  }

  @override
  Future<RelayEnvelope> seal({
    required int channelHandle,
    required List<int> plaintext,
  }) async {
    _validateHandle(channelHandle);
    if (plaintext.isEmpty ||
        plaintext.length > 64 * 1024 ||
        plaintext.any((byte) => byte < 0 || byte > 255)) {
      throw ArgumentError('invalid paired plaintext frame');
    }
    final encoded = await channel.invokeMethod<String>('channelSeal', {
      'channelHandle': channelHandle,
      'plaintext': Uint8List.fromList(plaintext),
    });
    final value = _jsonObject(encoded, const {
      'channelRef',
      'senderDeviceRef',
      'recipientDeviceRef',
      'sequence',
      'ciphertext',
    });
    final channelRef = value['channelRef'];
    final sender = value['senderDeviceRef'];
    final recipient = value['recipientDeviceRef'];
    final sequence = value['sequence'];
    if (channelRef is! String ||
        sender is! String ||
        recipient is! String ||
        sequence is! int ||
        sequence < 0) {
      throw StateError('invalid relay envelope');
    }
    return RelayEnvelope(
      channelRef: channelRef,
      senderDeviceRef: sender,
      recipientDeviceRef: recipient,
      sequence: sequence,
      ciphertext: _jsonBytes(value['ciphertext']),
    );
  }

  @override
  Future<Uint8List> open({
    required int channelHandle,
    required RelayEnvelope envelope,
  }) async {
    _validateHandle(channelHandle);
    final value = await channel.invokeMethod<Uint8List>('channelOpen', {
      'channelHandle': channelHandle,
      'envelopeJson': jsonEncode({
        'channelRef': envelope.channelRef,
        'senderDeviceRef': envelope.senderDeviceRef,
        'recipientDeviceRef': envelope.recipientDeviceRef,
        'sequence': envelope.sequence,
        'ciphertext': envelope.ciphertext,
      }),
    });
    if (value == null || value.length > 64 * 1024) {
      throw StateError('invalid paired plaintext response');
    }
    return Uint8List.fromList(value);
  }

  @override
  Future<void> close(int handle) async {
    _validateHandle(handle);
    await channel.invokeMethod<void>('closeNativeHandle', {'handle': handle});
  }

  static Map<String, Object?> _jsonObject(
    String? encoded,
    Set<String> expectedKeys,
  ) {
    if (encoded == null) throw StateError('missing native crypto response');
    final value = jsonDecode(encoded);
    if (value is! Map<String, Object?> ||
        value.keys.toSet().difference(expectedKeys).isNotEmpty ||
        expectedKeys.difference(value.keys.toSet()).isNotEmpty) {
      throw StateError('invalid native crypto response');
    }
    return value;
  }

  static Uint8List _jsonBytes(Object? value) {
    if (value is! List<Object?> ||
        value.any((byte) => byte is! int || byte < 0 || byte > 255)) {
      throw StateError('invalid native byte array');
    }
    return Uint8List.fromList(value.cast<int>());
  }

  static void _validateHandle(int handle) {
    if (handle <= 0) throw ArgumentError('invalid native handle');
  }
}
