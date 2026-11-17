library;

import 'dart:convert';
import 'dart:typed_data';
import 'package:crypto/crypto.dart';
import 'package:http/http.dart' as http;
import 'package:openmuse_smart_document/openmuse_smart_document.dart';

final class DocumentBrowserTarget {
  const DocumentBrowserTarget(this.workspaceId, this.viewId);
  final String workspaceId, viewId;
  String get route => '/documents/$workspaceId/$viewId';
  static DocumentBrowserTarget? parseEditing(String route) =>
      route.endsWith('/edit')
      ? parse(route.substring(0, route.length - 5))
      : null;
  static DocumentBrowserTarget? parse(String route) {
    if (route.length > 256) return null;
    final uuid =
        r'[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}';
    final match = RegExp('^/documents/($uuid)/($uuid)\$').firstMatch(route);
    return match == null
        ? null
        : DocumentBrowserTarget(
            match[1]!.toLowerCase(),
            match[2]!.toLowerCase(),
          );
  }
}

/// An anonymous capability only for original publication reads. It has no
/// access-token provider and cannot initialize or exchange collaboration data.
final class PublicDocumentReader
    implements CollaborationProvider, CollaborationObjectScope {
  PublicDocumentReader({
    required this.origin,
    required this.target,
    http.Client? client,
    this.onUnavailable,
    this.onNameChanged,
  }) : _client = client ?? http.Client() {
    if (origin.userInfo.isNotEmpty ||
        origin.hasQuery ||
        origin.hasFragment ||
        (origin.path.isNotEmpty && origin.path != '/') ||
        (origin.scheme != 'https' &&
            !(origin.scheme == 'http' &&
                {'localhost', '127.0.0.1', '::1'}.contains(origin.host)))) {
      throw ArgumentError('Invalid published document origin');
    }
    if (DocumentBrowserTarget.parse(
          '/documents/${target.workspaceId}/${target.viewId}',
        ) ==
        null) {
      throw ArgumentError('Invalid published document identity');
    }
  }
  final Uri origin;
  final DocumentBrowserTarget target;
  final http.Client _client;
  final void Function()? onUnavailable;
  final void Function()? onNameChanged;
  bool _closed = false;
  String name = 'SmartDocument';
  @override
  CollaborationBinding get binding => smartDocumentCollaborationBinding;
  @override
  String get boundObjectId => target.viewId;
  Uri get _base => origin.resolve(
    '/api/workspace/${target.workspaceId}/published-document/${target.viewId}/',
  );

  Future<Uint8List> _get(Uri uri, int budget) async {
    if (_closed) throw const SmartDocumentFailure('SESSION_CLOSED');
    final request = http.Request('GET', uri)..followRedirects = false;
    final response = await _client
        .send(request)
        .timeout(const Duration(seconds: 20));
    if (response.statusCode != 200) {
      await response.stream.drain<void>();
      if ({401, 403, 404}.contains(response.statusCode)) onUnavailable?.call();
      throw const SmartDocumentFailure('PUBLIC_DOCUMENT_UNAVAILABLE');
    }
    final bytes = BytesBuilder(copy: false);
    await for (final chunk in response.stream.timeout(
      const Duration(seconds: 20),
    )) {
      bytes.add(chunk);
      if (bytes.length > budget)
        throw const SmartDocumentFailure('PUBLIC_DOCUMENT_TOO_LARGE');
    }
    if (_closed) throw const SmartDocumentFailure('SESSION_CLOSED');
    return bytes.takeBytes();
  }

  @override
  Future<Uint8List?> readState(String objectId) async {
    if (objectId != target.viewId)
      throw const SmartDocumentFailure('COLLABORATION_OBJECT_MISMATCH');
    final raw = jsonDecode(
      utf8.decode(await _get(_base.resolve('live'), 24 * 1024 * 1024)),
    );
    if (raw is! Map || raw['code'] != 0) {
      onUnavailable?.call();
      throw const SmartDocumentFailure('PUBLIC_DOCUMENT_UNAVAILABLE');
    }
    final data = raw['data'];
    if (data is! Map ||
        data['profile'] != 'openmuse.public-document/1' ||
        data['workspaceId'] != target.workspaceId ||
        data['objectId'] != target.viewId ||
        data['name'] is! String ||
        (data['name'] as String).length > 4096 ||
        data['docState'] is! String) {
      throw const SmartDocumentFailure('INVALID_PUBLIC_DOCUMENT');
    }
    final updatedName = data['name'] as String;
    final state = base64Decode(data['docState'] as String);
    if (state.isEmpty || state.length > 16 * 1024 * 1024)
      throw const SmartDocumentFailure('INVALID_PUBLIC_DOCUMENT');
    if (name != updatedName) {
      name = updatedName;
      onNameChanged?.call();
    }
    return state;
  }

  Future<Uint8List> readAsset(AssetReference ref) async {
    if (ref.provider != 'appflowy.private-assets/1' ||
        ref.scope != target.workspaceId ||
        ref.owner != target.viewId ||
        ref.key != ref.sha256 ||
        !RegExp(r'^[a-f0-9]{64}$').hasMatch(ref.key) ||
        ref.byteLength > 12 * 1024 * 1024) {
      throw const AssetFailure('ASSET_SCOPE_MISMATCH');
    }
    final bytes = await _get(
      _base.resolve('asset/${ref.key}'),
      12 * 1024 * 1024,
    );
    if (bytes.length != ref.byteLength ||
        sha256.convert(bytes).toString() != ref.sha256) {
      throw const AssetFailure('ASSET_INTEGRITY_FAILED');
    }
    return bytes;
  }

  @override
  Future<void> initializeState(String objectId, Uint8List encodedState) async =>
      throw const SmartDocumentFailure('READ_ONLY');
  @override
  Future<Uint8List> exchange(String objectId, Uint8List request) async =>
      throw const SmartDocumentFailure('READ_ONLY');
  @override
  void close() {
    _closed = true;
    _client.close();
  }
}

final class PublicDocumentAssets implements AssetRepository {
  PublicDocumentAssets(this.reader);
  final PublicDocumentReader reader;
  @override
  Future<Uint8List> read(AssetReference reference) =>
      reader.readAsset(reference);
  @override
  Future<AssetReference> put({
    required String owner,
    required Uint8List bytes,
    required String mediaType,
  }) async => throw const AssetFailure('READ_ONLY');
  @override
  void close() {}
}

/// Public browsing never falls back to an offline copy after revocation.
final class TransientDocumentStore implements DocumentStore {
  Map<String, dynamic>? _value;
  @override
  Future<Map<String, dynamic>?> read() async => _value;
  @override
  Future<void> write(Map<String, dynamic> value) async {
    _value = value;
  }
}
