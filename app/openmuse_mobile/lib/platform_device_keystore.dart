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
  Future<void> delete(String keyRef) async {
    _validateRef(keyRef);
    await channel.invokeMethod<void>('delete', {'keyRef': keyRef});
  }

  static void _validateRef(String value) {
    if (value.isEmpty || value.length > 256 || value.contains('\u0000')) {
      throw ArgumentError('invalid device keystore ref');
    }
  }
}
