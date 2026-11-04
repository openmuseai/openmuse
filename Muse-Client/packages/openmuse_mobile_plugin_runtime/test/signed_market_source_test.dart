import 'dart:convert';
import 'dart:io';

import 'package:cryptography/cryptography.dart';
import 'package:crypto/crypto.dart';
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

  test(
    'signed HTTP catalog installs a WebView/A2UI package end to end',
    () async {
      final algorithm = Ed25519();
      final pair = await algorithm.newKeyPair();
      final publicKey = await pair.extractPublicKey();
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final directory = await Directory.systemTemp.createTemp(
        'signed-web-market-',
      );
      addTearDown(() async {
        await server.close(force: true);
        await directory.delete(recursive: true);
      });
      final origin = 'http://127.0.0.1:${server.port}';
      final bytes = await File('assets/task-board.json').readAsBytes();
      final digest = sha256.convert(bytes).toString();
      final payload = utf8.encode(
        jsonEncode({
          'schema': 'openmuse-mobile-market/v1',
          'sequence': 1,
          'expiresAt': DateTime.now()
              .toUtc()
              .add(const Duration(days: 1))
              .toIso8601String(),
          'plugins': [
            {
              'id': 'com.openmuse.mobile.task-board',
              'name': '任务看板',
              'version': '0.2.0',
              'description': 'WebView/A2UI 演示',
              'author': 'OpenMuse',
              'platforms': ['android', 'ios'],
              'runtime': 'webview',
              'bundle': '$origin/task-board.json',
              'sha256': digest,
            },
          ],
        }),
      );
      final signature = await algorithm.sign(payload, keyPair: pair);
      final envelope = jsonEncode({
        'payload': base64Encode(payload),
        'signature': base64Encode(signature.bytes),
      });
      var tampered = false;
      server.listen((request) async {
        if (request.uri.path == '/index.json') {
          request.response.write(envelope);
        } else if (request.uri.path == '/task-board.json') {
          request.response.add(tampered ? [...bytes, 0] : bytes);
        } else {
          request.response.statusCode = HttpStatus.notFound;
        }
        await request.response.close();
      });
      final source = SignedHttpMobileMarketSource(
        indexUri: Uri.parse('$origin/index.json'),
        publicKey: publicKey.bytes,
        allowLoopbackHttpForTesting: true,
      );
      final entry = (await source.list()).single;
      final store = MobilePluginStore(directory);
      await store.install(entry, await source.artifact(entry));
      expect(store.installed(entry.id)!.isWebView, isTrue);
      tampered = true;
      await expectLater(
        store.install(entry, await source.artifact(entry)),
        throwsFormatException,
      );
      expect(store.installed(entry.id)!.isWebView, isTrue);
    },
  );
}
