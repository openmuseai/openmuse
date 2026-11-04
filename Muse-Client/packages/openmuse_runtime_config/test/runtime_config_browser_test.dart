@TestOn('browser')
library;

import 'dart:convert';

import 'package:cryptography/cryptography.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

String _b64(List<int> bytes) => base64UrlEncode(bytes).replaceAll('=', '');

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  const installed = String.fromEnvironment('OPENMUSE_DEPLOYMENT_JSON');
  if (installed.isEmpty) {
    test('browser configuration fixture must be compiled explicitly', () {
      fail(
        'Run compile_deployment and pass --dart-define-from-file to the browser test.',
      );
    });
    return;
  }

  test(
    'installed web mapping preserves the current edge origin and mount independence',
    () async {
      final config = (await loadDeploymentBundle()).snapshot;
      final page = Uri.parse('https://app.example/zh/workspaces/example');
      final auth = config.webServiceUri(ServiceId.auth, page);
      expect(auth.origin, page.origin);
      expect(auth.path, '/gotrue');
      expect(config.webServiceUri(ServiceId.cloud, page).path, '/');
      expect(
        () => config.webServiceUri(
          ServiceId.auth,
          Uri.parse('http://untrusted.example'),
        ),
        throwsFormatException,
      );
    },
  );

  test('Ed25519 compact JWS verifies in the browser runtime', () async {
    final snapshot = (await loadDeploymentBundle()).snapshot;
    final algorithm = Ed25519();
    final key = await algorithm.newKeyPair();
    final public = await key.extractPublicKey();
    final header = _b64(
      utf8.encode(
        jsonEncode({
          'alg': 'EdDSA',
          'typ': 'openmuse.runtime-config+json',
          'kid': 'browser-test',
        }),
      ),
    );
    final payload = _b64(utf8.encode(jsonEncode(snapshot.json)));
    final input = '$header.$payload';
    final signature = await algorithm.sign(utf8.encode(input), keyPair: key);
    final verifier = SignedConfigVerifier(
      publicKeys: {'browser-test': public.bytes},
    );
    final verified = await verifier.verify('$input.${_b64(signature.bytes)}');
    expect(verified.identity, snapshot.identity);
  });
}
