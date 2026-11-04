import 'dart:convert';

import 'package:cryptography/cryptography.dart';

import 'model.dart';

List<int> decodeConfigBase64(String value) {
  if (!RegExp(r'^[A-Za-z0-9_-]+$').hasMatch(value)) {
    throw const FormatException('Invalid unpadded base64url');
  }
  return base64Url.decode(base64Url.normalize(value));
}

final class SignedConfigVerifier {
  SignedConfigVerifier({required Map<String, List<int>> publicKeys})
    : _keys = Map.unmodifiable(
        publicKeys.map((k, v) => MapEntry(k, List<int>.unmodifiable(v))),
      );

  final Map<String, List<int>> _keys;

  Future<RuntimeConfig> verify(String compactJws) async {
    if (compactJws.length > 1024 * 1024) {
      throw const FormatException('Config is too large');
    }
    final parts = compactJws.split('.');
    if (parts.length != 3) throw const FormatException('Expected compact JWS');
    final header = configObject(
      jsonDecode(utf8.decode(decodeConfigBase64(parts[0]))),
      'JWS header',
    );
    if (header['alg'] != 'EdDSA' ||
        header['typ'] != 'openmuse.runtime-config+json' ||
        header.keys.any((k) => !{'alg', 'typ', 'kid'}.contains(k))) {
      throw const FormatException('Unsupported config signature header');
    }
    final key = _keys[header['kid']];
    if (key == null || key.length != 32) {
      throw const FormatException('Unknown config signing key');
    }
    final signature = decodeConfigBase64(parts[2]);
    if (signature.length != 64 ||
        !await Ed25519().verify(
          utf8.encode('${parts[0]}.${parts[1]}'),
          signature: Signature(
            signature,
            publicKey: SimplePublicKey(key, type: KeyPairType.ed25519),
          ),
        )) {
      throw const FormatException('Invalid config signature');
    }
    return RuntimeConfig.fromJson(
      jsonDecode(utf8.decode(decodeConfigBase64(parts[1]))),
    );
  }
}
