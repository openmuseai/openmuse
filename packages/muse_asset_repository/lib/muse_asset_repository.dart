import 'dart:async';
import 'package:muse_content_store/snapshot_store.dart';
import 'dart:convert';
import 'dart:typed_data';
import 'package:crypto/crypto.dart';
import 'package:http/http.dart' as http;
import 'package:muse_asset_contract/muse_asset_contract.dart';
export 'package:muse_asset_contract/muse_asset_contract.dart';

const _maxAssetBytes = 12 * 1024 * 1024;
final _uuid = RegExp(
  r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$',
);

/// Existing Cloud storage, accessed exclusively through object ACL routes.
/// Endpoint/token are injected capabilities and never persisted in references.
final class CloudAssetRepository implements AssetRepository {
  CloudAssetRepository({
    required this.origin,
    required this.workspaceId,
    required this.accessToken,
    http.Client? client,
  }) : _client = client ?? http.Client() {
    if (!_uuid.hasMatch(workspaceId) ||
        origin.userInfo.isNotEmpty ||
        origin.hasQuery ||
        origin.hasFragment ||
        (origin.path != '' && origin.path != '/') ||
        !(origin.scheme == 'https' ||
            (origin.scheme == 'http' &&
                {
                  'localhost',
                  '127.0.0.1',
                  '::1',
                  '10.0.2.2',
                }.contains(origin.host)))) {
      throw ArgumentError('INVALID_ASSET_ENDPOINT');
    }
  }
  final Uri origin;
  final String workspaceId;
  final Future<String> Function() accessToken;
  final http.Client _client;
  bool _closed = false;
  Uri _uri(String owner, String key) {
    if (!_uuid.hasMatch(owner) || !RegExp(r'^[a-f0-9]{64}$').hasMatch(key))
      throw const AssetFailure('INVALID_ASSET_ADDRESS');
    return origin.resolve(
      '/api/file_storage/$workspaceId/private/$owner/blob/$key',
    );
  }

  Future<http.Response> _request(
    String method,
    Uri target, {
    Uint8List? bytes,
    String? mediaType,
  }) async {
    if (_closed) throw const AssetFailure('SESSION_CLOSED');
    final request = http.Request(method, target)..followRedirects = false;
    request.headers['Authorization'] = 'Bearer ${await accessToken()}';
    if (mediaType != null) request.headers['Content-Type'] = mediaType;
    if (bytes != null) request.bodyBytes = bytes;
    try {
      final response = await _client
          .send(request)
          .timeout(const Duration(seconds: 30));
      final body = BytesBuilder();
      await for (final chunk in response.stream.timeout(
        const Duration(seconds: 30),
      )) {
        if (body.length + chunk.length > _maxAssetBytes)
          throw const AssetFailure('ASSET_TOO_LARGE');
        body.add(chunk);
      }
      final result = http.Response.bytes(
        body.takeBytes(),
        response.statusCode,
        headers: response.headers,
      );
      if ({401, 403}.contains(result.statusCode))
        throw const AssetFailure('FORBIDDEN');
      if (result.statusCode == 404) throw const AssetFailure('NOT_FOUND');
      if (result.statusCode != 200)
        throw AssetFailure('HTTP_${result.statusCode}');
      if ((result.headers['content-type'] ?? '').contains('application/json')) {
        final envelope = jsonDecode(result.body);
        if (envelope is Map &&
            envelope['code'] != null &&
            envelope['code'] != 0) {
          if ({1011, 1012, 1024}.contains(envelope['code']))
            throw const AssetFailure('FORBIDDEN');
          if (envelope['code'] == -2) throw const AssetFailure('NOT_FOUND');
          throw const AssetFailure('ASSET_REQUEST_REJECTED');
        }
      }
      return result;
    } on TimeoutException {
      throw const AssetFailure('UNAVAILABLE');
    } on http.ClientException {
      throw const AssetFailure('UNAVAILABLE');
    }
  }

  @override
  Future<AssetReference> put({
    required String owner,
    required Uint8List bytes,
    required String mediaType,
  }) async {
    if (bytes.isEmpty || bytes.length > _maxAssetBytes)
      throw const AssetFailure('ASSET_TOO_LARGE');
    final digest = sha256.convert(bytes).toString();
    final response = await _request(
      'PUT',
      _uri(owner, digest),
      bytes: bytes,
      mediaType: mediaType,
    );
    final raw = jsonDecode(response.body) as Map;
    final ref = AssetReference.fromJson({
      'version': 1,
      'provider': raw['provider'],
      'scope': raw['scope'],
      'owner': raw['owner'],
      'key': raw['key'],
      'mediaType': raw['media_type'],
      'byteLength': raw['byte_length'],
      'sha256': raw['sha256'],
    });
    if (ref.scope != workspaceId ||
        ref.owner != owner ||
        ref.key != digest ||
        ref.sha256 != digest ||
        ref.byteLength != bytes.length ||
        ref.mediaType != mediaType) {
      throw const AssetFailure('ASSET_RECEIPT_MISMATCH');
    }
    return ref;
  }

  @override
  Future<Uint8List> read(AssetReference reference) async {
    final ref = AssetReference.fromJson(reference.toJson());
    if (ref.provider != 'appflowy.private-assets/1' ||
        ref.scope != workspaceId ||
        ref.key != ref.sha256 ||
        ref.byteLength > _maxAssetBytes)
      throw const AssetFailure('ASSET_SCOPE_MISMATCH');
    final response = await _request('GET', _uri(ref.owner, ref.key));
    final bytes = response.bodyBytes;
    if (bytes.length != ref.byteLength ||
        sha256.convert(bytes).toString() != ref.sha256)
      throw const AssetFailure('ASSET_INTEGRITY_FAILED');
    return bytes;
  }

  @override
  void close() {
    _closed = true;
    _client.close();
  }
}

/// Remote permission is rechecked on each read. Only network unavailability may
/// use a cached, previously authorized copy; denial/not-found never falls back.
final class CachedAssetRepository implements AssetRepository {
  CachedAssetRepository({
    required this.source,
    required this.cache,
    required this.allowOffline,
  });
  final AssetRepository source;
  final AssetCache cache;
  final bool Function() allowOffline;
  @override
  Future<AssetReference> put({
    required String owner,
    required Uint8List bytes,
    required String mediaType,
  }) async {
    final ref = await source.put(
      owner: owner,
      bytes: bytes,
      mediaType: mediaType,
    );
    await cache.write(ref, bytes);
    return ref;
  }

  @override
  Future<Uint8List> read(AssetReference reference) async {
    try {
      final bytes = await source.read(reference);
      await cache.write(reference, bytes);
      return bytes;
    } on AssetFailure catch (error) {
      if (error.code == 'UNAVAILABLE' && allowOffline()) {
        final bytes = await cache.read(reference);
        if (bytes != null) return bytes;
      }
      rethrow;
    }
  }

  @override
  void close() => source.close();
}

/// Local resource capability, also used for credential-free reopening of copies
/// already cached under this private SDK resource/account scope. No remote ACK.
final class LocalAssetRepository implements AssetRepository {
  LocalAssetRepository({required this.scope, required this.cache});
  final String scope;
  final AssetCache cache;
  bool _closed = false;
  @override
  Future<AssetReference> put({
    required String owner,
    required Uint8List bytes,
    required String mediaType,
  }) async {
    if (_closed) throw const AssetFailure('SESSION_CLOSED');
    if (bytes.isEmpty || bytes.length > _maxAssetBytes)
      throw const AssetFailure('ASSET_TOO_LARGE');
    final digest = sha256.convert(bytes).toString();
    final ref = AssetReference(
      provider: 'muse.local-assets/1',
      scope: scope,
      owner: owner,
      key: digest,
      mediaType: mediaType,
      byteLength: bytes.length,
      sha256: digest,
    );
    await cache.write(ref, bytes);
    return ref;
  }

  @override
  Future<Uint8List> read(AssetReference reference) async {
    if (_closed) throw const AssetFailure('SESSION_CLOSED');
    final bytes = await cache.read(reference);
    if (bytes == null) throw const AssetFailure('NOT_CACHED');
    return bytes;
  }

  @override
  void close() {
    _closed = true;
  }
}

/// Uses the already verified atomic file / completed IndexedDB store adapters.
/// The factory must be scoped to the active account and workspace by the plugin.
final class SnapshotStoreAssetCache implements AssetCache {
  SnapshotStoreAssetCache(this.storeFor);
  final SnapshotStore Function(String cacheKey) storeFor;
  String _key(AssetReference ref) =>
      sha256.convert(utf8.encode(jsonEncode(ref.toJson()))).toString();
  @override
  Future<Uint8List?> read(AssetReference reference) async {
    final record = await storeFor(_key(reference)).read();
    if (record == null ||
        jsonEncode(record['reference']) != jsonEncode(reference.toJson()))
      return null;
    final value = record['bytes'];
    if (value is! String) return null;
    final bytes = base64Decode(value);
    if (bytes.length != reference.byteLength ||
        sha256.convert(bytes).toString() != reference.sha256)
      throw const AssetFailure('CACHE_INTEGRITY_FAILED');
    return bytes;
  }

  @override
  Future<void> write(AssetReference reference, Uint8List bytes) async {
    if (bytes.length != reference.byteLength ||
        bytes.length > _maxAssetBytes ||
        sha256.convert(bytes).toString() != reference.sha256)
      throw const AssetFailure('CACHE_INTEGRITY_FAILED');
    await storeFor(
      _key(reference),
    ).write({'reference': reference.toJson(), 'bytes': base64Encode(bytes)});
  }
}
