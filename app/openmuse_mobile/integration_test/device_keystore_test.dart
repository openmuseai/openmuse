import 'dart:convert';

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/platform_device_keystore.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

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

  testWidgets('opaque native handles complete an encrypted round trip', (
    tester,
  ) async {
    const store = PlatformDeviceKeyStore();
    const crypto = PlatformPairedCrypto();
    final mobile = await store.ensure(
      accountRef: 'account:android-channel-integration',
      deviceRef: 'mobile:android-channel-integration',
    );
    final desktop = await store.ensure(
      accountRef: 'account:android-channel-integration',
      deviceRef: 'desktop:android-channel-integration',
    );
    NativePairedChannel? mobileChannel;
    NativePairedChannel? desktopChannel;
    try {
      final mobileOffer = await store.issueOffer(
        keyRef: mobile.keyRef,
        accountRef: 'account:android-channel-integration',
        deviceRef: 'mobile:android-channel-integration',
        registrationGeneration: 1,
      );
      final desktopOffer = await store.issueOffer(
        keyRef: desktop.keyRef,
        accountRef: 'account:android-channel-integration',
        deviceRef: 'desktop:android-channel-integration',
        registrationGeneration: 1,
      );
      final mobileRegistration = TrustedDeviceRegistration.fromOffer(
        mobileOffer,
      );
      final desktopRegistration = TrustedDeviceRegistration.fromOffer(
        desktopOffer,
      );
      final mobileHandshake = await crypto.beginHandshake(
        keyRef: mobile.keyRef,
        localOffer: mobileOffer,
        remoteOffer: desktopOffer,
        localRegistration: mobileRegistration,
        remoteRegistration: desktopRegistration,
      );
      final desktopHandshake = await crypto.beginHandshake(
        keyRef: desktop.keyRef,
        localOffer: desktopOffer,
        remoteOffer: mobileOffer,
        localRegistration: desktopRegistration,
        remoteRegistration: mobileRegistration,
      );
      expect(
        desktopHandshake.confirmationCode,
        mobileHandshake.confirmationCode,
      );
      mobileChannel = await crypto.confirmHandshake(
        handshakeHandle: mobileHandshake.handle,
        confirmationCode: mobileHandshake.confirmationCode,
      );
      desktopChannel = await crypto.confirmHandshake(
        handshakeHandle: desktopHandshake.handle,
        confirmationCode: desktopHandshake.confirmationCode,
      );
      expect(desktopChannel.channelRef, mobileChannel.channelRef);

      final plaintext = utf8.encode('bounded mobile DSH response');
      final envelope = await crypto.seal(
        channelHandle: mobileChannel.handle,
        plaintext: plaintext,
      );
      expect(envelope.ciphertext, isNot(contains(plaintext)));
      final opened = await crypto.open(
        channelHandle: desktopChannel.handle,
        envelope: envelope,
      );
      expect(opened, plaintext);
      await expectLater(
        crypto.open(channelHandle: desktopChannel.handle, envelope: envelope),
        throwsA(isA<PlatformException>()),
      );
    } finally {
      if (mobileChannel != null) await crypto.close(mobileChannel.handle);
      if (desktopChannel != null) await crypto.close(desktopChannel.handle);
      await store.delete(mobile.keyRef);
      await store.delete(desktop.keyRef);
    }
  });
}
