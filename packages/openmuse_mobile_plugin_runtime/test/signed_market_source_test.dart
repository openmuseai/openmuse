import 'dart:convert';
import 'dart:io';

import 'package:cryptography/cryptography.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
  test('signed catalog accepts only same-origin immutable artifacts', () async {
    final algorithm = Ed25519();
    final keyPair = await algorithm.newKeyPair();
    final publicKey = await keyPair.extractPublicKey();
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    addTearDown(server.close);
    final state = await Directory.systemTemp.createTemp(
      'openmuse-market-sequence-',
    );
    addTearDown(() => state.delete(recursive: true));
    final origin = 'http://127.0.0.1:${server.port}';
    List<int> payloadFor(int sequence) => utf8.encode(
      jsonEncode({
        'schema': 'openmuse-mobile-market/v1',
        'sequence': sequence,
        'expiresAt': DateTime.now()
            .toUtc()
            .add(const Duration(days: 1))
            .toIso8601String(),
        'plugins': [
          {
            'id': 'com.openmuse.example',
            'name': 'Example',
            'version': '0.1.0',
            'description': 'signed',
            'author': 'OpenMuse',
            'platforms': ['android'],
            'bundle': '$origin/example.json',
            'sha256': List.filled(64, 'a').join(),
          },
        ],
      }),
    );
    Future<String> sign(int sequence) async {
      final payload = payloadFor(sequence);
      final signature = await algorithm.sign(payload, keyPair: keyPair);
      return jsonEncode({
        'payload': base64Encode(payload),
        'signature': base64Encode(signature.bytes),
      });
    }

    var envelope = await sign(2);
    server.listen((request) async {
      request.response.write(envelope);
      await request.response.close();
    });
    final source = SignedHttpMobileMarketSource(
      indexUri: Uri.parse('$origin/index.json'),
      publicKey: publicKey.bytes,
      sequenceFile: File('${state.path}/sequence'),
      allowLoopbackHttpForTesting: true,
    );
    final entries = await source.list();
    expect(entries.single.id, 'com.openmuse.example');
    envelope = await sign(1);
    await expectLater(source.list(), throwsFormatException);
    envelope = await sign(2);
    final rejected = SignedHttpMobileMarketSource(
      indexUri: Uri.parse('$origin/index.json'),
      publicKey: List.filled(32, 0),
      allowLoopbackHttpForTesting: true,
    );
    await expectLater(rejected.list(), throwsFormatException);
  });
}
