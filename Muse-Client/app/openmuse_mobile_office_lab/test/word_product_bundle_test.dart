import 'dart:convert';

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_word_product/openmuse_word_product.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  test(
    'bundled mobile Word product is signed and has all ribbon tabs',
    () async {
      const prefix = '../openmuse_mobile/assets/word_product/';
      final identity =
          jsonDecode(await rootBundle.loadString('${prefix}identity.json'))
              as Map;
      final publicKey = await rootBundle.loadString('${prefix}public.pem');
      final data = await rootBundle.load('${prefix}word.mcpb');
      final bytes = data.buffer.asUint8List(
        data.offsetInBytes,
        data.lengthInBytes,
      );
      final product = await VerifiedWordProduct.loadBundle(
        bundle: bytes,
        trustedPublicKeyPem: publicKey,
        expectedKeyId: identity['keyId'] as String,
        expectedVersion: identity['version'] as String,
      );
      expect(
        product.definition.ribbon.tabs.map((tab) => tab.id),
        containsAll([
          'home',
          'insert',
          'draw',
          'design',
          'layout',
          'references',
          'review',
          'view',
        ]),
      );
      expect(product.definition.ribbon.menus.length, greaterThanOrEqualTo(62));

      final altered = Uint8List.fromList(bytes);
      altered[altered.length ~/ 2] ^= 1;
      await expectLater(
        VerifiedWordProduct.loadBundle(
          bundle: altered,
          trustedPublicKeyPem: publicKey,
          expectedKeyId: identity['keyId'] as String,
          expectedVersion: identity['version'] as String,
        ),
        throwsA(isA<Exception>()),
      );
    },
  );
}
