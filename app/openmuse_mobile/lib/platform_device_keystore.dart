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
