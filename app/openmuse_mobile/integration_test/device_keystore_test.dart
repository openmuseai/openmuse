import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/platform_device_keystore.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('platform device key remains opaque and lifecycle is stable', (
    tester,
  ) async {
    const store = PlatformDeviceKeyStore();
    final first = await store.ensure(
      accountRef: 'account:android-device-keystore-integration',
      deviceRef: 'mobile:android-device-keystore-integration',
    );
    addTearDown(() => store.delete(first.keyRef));

    final second = await store.ensure(
      accountRef: 'account:android-device-keystore-integration',
      deviceRef: 'mobile:android-device-keystore-integration',
    );
    expect(second.keyRef, first.keyRef);
    expect(second.storage, 'android-keystore-wrapped');
    expect(second.created, isFalse);
    expect(second.keyRef, startsWith('device-key:'));
    expect(second.keyRef, isNot(contains('seed')));
    final firstIdentity = await store.publicIdentity(first.keyRef);
    final secondIdentity = await store.publicIdentity(second.keyRef);
    expect(secondIdentity.signingPublic, firstIdentity.signingPublic);
    expect(secondIdentity.agreementPublic, firstIdentity.agreementPublic);
    expect(firstIdentity.signingPublic, isNot(firstIdentity.agreementPublic));
    final firstOffer = await store.issueOffer(
      keyRef: first.keyRef,
      accountRef: 'account:android-device-keystore-integration',
      deviceRef: 'mobile:android-device-keystore-integration',
      registrationGeneration: 7,
    );
    final secondOffer = await store.issueOffer(
      keyRef: first.keyRef,
      accountRef: 'account:android-device-keystore-integration',
      deviceRef: 'mobile:android-device-keystore-integration',
      registrationGeneration: 7,
    );
    expect(firstOffer.signingPublic, firstIdentity.signingPublic);
    expect(firstOffer.agreementPublic, firstIdentity.agreementPublic);
    expect(firstOffer.signature, hasLength(64));
    expect(firstOffer.registrationGeneration, 7);
    expect(secondOffer.nonce, isNot(firstOffer.nonce));

    await store.delete(first.keyRef);
    final recreated = await store.ensure(
      accountRef: 'account:android-device-keystore-integration',
      deviceRef: 'mobile:android-device-keystore-integration',
    );
    expect(recreated.keyRef, first.keyRef);
    expect(recreated.created, isTrue);
    final recreatedIdentity = await store.publicIdentity(recreated.keyRef);
    expect(recreatedIdentity.signingPublic, isNot(firstIdentity.signingPublic));
  });
}
