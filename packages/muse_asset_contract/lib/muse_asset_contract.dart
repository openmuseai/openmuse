import 'dart:typed_data';
import 'dart:convert';

/// Identity/integrity metadata only. Does not grant access, contain credentials,
/// store a local absolute path, or prescribe an editor/document content model.
final class AssetReference {
  factory AssetReference({
    required String provider,
    required String scope,
    required String owner,
    required String key,
    required String mediaType,
    required int byteLength,
    required String sha256,
  }) => AssetReference.fromJson({
    'version': 1,
    'provider': provider,
    'scope': scope,
    'owner': owner,
    'key': key,
    'mediaType': mediaType,
    'byteLength': byteLength,
    'sha256': sha256,
  });
  const AssetReference._({
    required this.provider,
    required this.scope,
    required this.owner,
    required this.key,
    required this.mediaType,
    required this.byteLength,
    required this.sha256,
  });
  final String provider, scope, owner, key, mediaType, sha256;
  final int byteLength;
  Map<String, Object> toJson() => {
    'version': 1,
    'provider': provider,
    'scope': scope,
    'owner': owner,
    'key': key,
    'mediaType': mediaType,
    'byteLength': byteLength,
    'sha256': sha256,
  };

  /// Portable identity for original media fields whose URL is a string.
  /// This contains no endpoint or credential and never grants access.
  Uri toUri() => Uri(
    scheme: 'muse-asset',
    host: 'v1',
    pathSegments: [
      base64Url.encode(utf8.encode(jsonEncode(toJson()))).replaceAll('=', ''),
    ],
  );
  factory AssetReference.fromUri(Uri uri) {
    if (uri.toString().length > 8192 ||
        uri.scheme != 'muse-asset' ||
        uri.host != 'v1' ||
        uri.hasPort ||
        uri.userInfo.isNotEmpty ||
        uri.hasQuery ||
        uri.hasFragment ||
        uri.pathSegments.length != 1 ||
        !RegExp(r'^[A-Za-z0-9_-]+$').hasMatch(uri.pathSegments.single)) {
      throw const FormatException('INVALID_ASSET_URI');
    }
    try {
      final raw = jsonDecode(
        utf8.decode(
          base64Url.decode(base64Url.normalize(uri.pathSegments.single)),
        ),
      );
      if (raw is! Map<String, dynamic>) throw const FormatException();
      final ref = AssetReference.fromJson(raw);
      if (ref.toUri().toString() != uri.toString())
        throw const FormatException();
      return ref;
    } on Object {
      throw const FormatException('INVALID_ASSET_URI');
    }
  }
  factory AssetReference.fromJson(Map<String, dynamic> json) {
    const fields = {
      'version',
      'provider',
      'scope',
      'owner',
      'key',
      'mediaType',
      'byteLength',
      'sha256',
    };
    if (json.length != fields.length ||
        !json.keys.every(fields.contains) ||
        json['version'] != 1 ||
        !['provider', 'scope', 'owner', 'key', 'mediaType', 'sha256'].every(
          (k) =>
              json[k] is String &&
              (json[k] as String).isNotEmpty &&
              (json[k] as String).length <= 512 &&
              !(json[k] as String).contains(RegExp(r'[\x00-\x1f\x7f]')),
        ) ||
        json['byteLength'] is! int ||
        (json['byteLength'] as int) <= 0 ||
        !RegExp(r'^[a-f0-9]{64}$').hasMatch(json['sha256'] as String)) {
      throw const FormatException('INVALID_ASSET_REFERENCE');
    }
    return AssetReference._(
      provider: json['provider'],
      scope: json['scope'],
      owner: json['owner'],
      key: json['key'],
      mediaType: json['mediaType'],
      byteLength: json['byteLength'],
      sha256: json['sha256'],
    );
  }
}

final class AssetFailure implements Exception {
  const AssetFailure(this.code);
  final String code;
  @override
  String toString() => 'AssetFailure($code)';
}

/// Repository implementations own authorization, transport and account scope.
/// The editor owns insertion/deletion of references, not remote blob deletion.
abstract interface class AssetRepository {
  Future<AssetReference> put({
    required String owner,
    required Uint8List bytes,
    required String mediaType,
  });
  Future<Uint8List> read(AssetReference reference);
  void close();
}

/// Cache is account/workspace scoped by its owner. A cached copy is not evidence
/// of current remote permission; forbidden/not-found responses must not fall back.
abstract interface class AssetCache {
  Future<Uint8List?> read(AssetReference reference);
  Future<void> write(AssetReference reference, Uint8List bytes);
}
