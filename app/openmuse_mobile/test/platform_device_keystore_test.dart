import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/platform_device_keystore.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  const channel = MethodChannel('test/device_keystore');
  const store = PlatformDeviceKeyStore(channel: channel);

  tearDown(() async {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, null);
  });

  test('returns only an opaque platform descriptor', () async {
    final calls = <MethodCall>[];
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, (call) async {
          calls.add(call);
          return {
            'keyRef': 'device-key:abc',
            'storage': 'android-keystore-wrapped',
            'hardwareBacked': true,
            'created': true,
          };
        });
    final value = await store.ensure(
      accountRef: 'account:a',
      deviceRef: 'mobile:1',
    );
    expect(value.keyRef, 'device-key:abc');
    expect(value.hardwareBacked, isTrue);
    expect(calls.single.method, 'ensure');
    expect(calls.single.arguments.toString(), isNot(contains('seed')));
  });

  test('secret-bearing and malformed responses fail closed', () async {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(
          channel,
          (call) async => {
            'keyRef': 'device-key:abc',
            'storage': 'bad',
            'hardwareBacked': false,
            'created': true,
            'seed': [1, 2, 3],
          },
        );
    await expectLater(
      store.ensure(accountRef: 'account:a', deviceRef: 'mobile:1'),
      throwsStateError,
    );
    await expectLater(
      store.ensure(accountRef: '', deviceRef: 'mobile:1'),
      throwsArgumentError,
    );
  });
}
