import 'dart:convert';
import 'dart:typed_data';
import 'package:http/http.dart' as http;
import 'package:muse_collab_contract/muse_collab_contract.dart';
import 'src/write_capabilities.dart';
import 'src/model_contract.dart';
export 'package:muse_collab_contract/muse_collab_contract.dart';
export 'src/write_capabilities.dart';
export 'src/model_contract.dart';

final class AppFlowyObjectTransport
    implements CollaborationProvider, CollaborationObjectScope {
  AppFlowyObjectTransport({
    required this.origin,
    required this.binding,
    required this.collabType,
    required this.workspaceId,
    required this.accessToken,
    required this.deviceId,
    http.Client? client,
    this.boundObjectId,
    this.checkAuthority,
  }) : _client = client ?? http.Client() {
    final loopback = {
      'localhost',
      '127.0.0.1',
      '::1',
      '10.0.2.2',
    }.contains(origin.host);
    if ((origin.scheme != 'https' && !(origin.scheme == 'http' && loopback)) ||
        origin.userInfo.isNotEmpty ||
        origin.hasQuery ||
        origin.hasFragment ||
        (origin.path != '' && origin.path != '/') ||
        deviceId.isEmpty ||
        deviceId.length > 128 ||
        binding.protocol != 'appflowy.full-sync/1' ||
        binding.encoding != 'yrs.update-v1' ||
        collabType < 0 ||
        collabType > 16 ||
        !RegExp(
          r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$',
        ).hasMatch(workspaceId)) {
      throw ArgumentError('Invalid collaboration endpoint or workspace');
    }
  }
  @override
  final CollaborationBinding binding;

  /// Original AppFlowy wire discriminator; the domain binding chooses it.
  final int collabType;
  final Uri origin;
  final String workspaceId, deviceId;
  final Future<String> Function() accessToken;
  @override
  final String? boundObjectId;
  final void Function()? checkAuthority;
  final http.Client _client;
  bool _closed = false;
  static const _limit = 8 * 1024 * 1024;
  Future<http.Response> _request(
    String method,
    String route, {
    Object? body,
    bool binary = false,
  }) async {
    try {
      return await _requestOnce(method, route, body: body, binary: binary);
    } on http.ClientException {
      // A suspended/native-busy client may miss the server keep-alive FIN.
      // Only an idempotent GET can be retried, once, with fresh authorization.
      // Never replay writes whose application/durability may be ambiguous.
      if (method != 'GET' || _closed) rethrow;
      return _requestOnce(method, route, body: body, binary: binary);
    }
  }

  Future<http.Response> _requestOnce(
    String method,
    String route, {
    Object? body,
    bool binary = false,
  }) async {
    if (_closed) throw const AppFlowyTransportFailure('SESSION_CLOSED');
    checkAuthority?.call();
    final request = http.Request(method, origin.resolve(route))
      ..followRedirects = false;
    request.headers.addAll({
      'Authorization': 'Bearer ${await accessToken()}',
      'Device-Id': deviceId,
      'client-version': '0.16.5',
      'Content-Type': binary ? 'application/octet-stream' : 'application/json',
    });
    checkAuthority?.call();
    if (body is List<int>) {
      request.bodyBytes = body;
    } else if (body != null) {
      request.body = jsonEncode(body);
    }
    final response = await _client
        .send(request)
        .timeout(const Duration(seconds: 20));
    final bytes = BytesBuilder();
    await for (final chunk in response.stream.timeout(
      const Duration(seconds: 20),
    )) {
      if (bytes.length + chunk.length > _limit)
        throw const AppFlowyTransportFailure('QUOTA_EXCEEDED');
      bytes.add(chunk);
    }
    checkAuthority?.call();
    return http.Response.bytes(
      bytes.takeBytes(),
      response.statusCode,
      headers: response.headers,
    );
  }

  Map<String, dynamic> _data(http.Response response) {
    if (response.statusCode == 413) {
      throw const AppFlowyTransportFailure('QUOTA_EXCEEDED');
    }
    if (response.statusCode == 401 || response.statusCode == 403)
      throw const AppFlowyTransportFailure('FORBIDDEN');
    Map<String, dynamic> result;
    try {
      result = (jsonDecode(response.body) as Map).cast<String, dynamic>();
    } on Object {
      throw AppFlowyTransportFailure('CLOUD_HTTP_${response.statusCode}');
    }
    if (result['code'] == 1016) {
      throw const AppFlowyTransportFailure('QUOTA_EXCEEDED');
    }
    if (result['code'] == -4)
      throw const AppFlowyTransportFailure('OBJECT_DELETED');
    if ({1011, 1012, 1024}.contains(result['code']))
      throw const AppFlowyTransportFailure('FORBIDDEN');
    if (response.statusCode != 200 || result['code'] != 0) {
      throw AppFlowyTransportFailure(
        'CLOUD_${result['code'] ?? response.statusCode}',
      );
    }
    return result;
  }

  void _object(String objectId) {
    if (boundObjectId != null && objectId != boundObjectId) {
      throw const AppFlowyTransportFailure('COLLABORATION_OBJECT_MISMATCH');
    }
    if (!RegExp(
      r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$',
    ).hasMatch(objectId))
      throw const AppFlowyTransportFailure('INVALID_OBJECT_ID');
  }

  /// Missing discovery on an older Cloud does not disable original editing.
  /// Unknown/malformed profiles are never accepted as support for new semantics.
  Future<AppFlowyWriteCapabilities?> readWriteCapabilities(
    String objectId,
  ) async {
    _object(objectId);
    final response = await _request(
      'GET',
      '/api/workspace/v1/$workspaceId/collab/$objectId/write-capabilities?collab_type=$collabType',
    );
    if (_closed) throw const AppFlowyTransportFailure('SESSION_CLOSED');
    if (response.statusCode == 404) {
      Object? envelope;
      try {
        envelope = jsonDecode(response.body);
      } on FormatException {
        // Original Cloud without this route returns an unstructured 404.
      }
      if (envelope is! Map || !envelope.containsKey('code')) return null;
    }
    final data = _data(response)['data'];
    if (data is! Map || data['profile'] != AppFlowyWriteCapabilities.profile) {
      throw const AppFlowyTransportFailure('CAPABILITIES_UNSUPPORTED');
    }
    if (data['workspaceId'] is! String ||
        (data['workspaceId'] as String).toLowerCase() !=
            workspaceId.toLowerCase() ||
        data['objectId'] is! String ||
        (data['objectId'] as String).toLowerCase() != objectId.toLowerCase() ||
        data['collabType'] is! int ||
        data['collabType'] != collabType) {
      throw const AppFlowyTransportFailure('COLLABORATION_OBJECT_MISMATCH');
    }
    final encodings = data['updateEncodings'];
    final extensions = data['semanticExtensions'];
    final snapshotProfiles = data.containsKey('snapshotValidationProfiles')
        ? data['snapshotValidationProfiles']
        : const <String>[];
    if (snapshotProfiles is! List ||
        snapshotProfiles.length > 32 ||
        snapshotProfiles.any(
          (v) =>
              v is! String ||
              v.isEmpty ||
              v.length > 128 ||
              !RegExp(r'^[a-z0-9][a-z0-9._/-]*$').hasMatch(v),
        ) ||
        snapshotProfiles.toSet().length != snapshotProfiles.length) {
      throw const AppFlowyTransportFailure('INVALID_CAPABILITIES');
    }
    bool positiveLimit(String key) =>
        data[key] is int && data[key] > 0 && data[key] <= 0x7fffffff;
    if (data['canWrite'] is! bool ||
        data['qualifiedEditing'] is! bool ||
        data['documentActorBits'] is! int ||
        data['documentActorBits'] !=
            AppFlowyWriteCapabilities.documentActorBits ||
        !data.containsKey('epoch') ||
        data['epoch'] != null ||
        data['writerFencing'] != false ||
        extensions is! List ||
        extensions.isNotEmpty ||
        encodings is! List ||
        encodings.length != 2 ||
        !encodings.contains('yrs.update-v1') ||
        !encodings.contains('yrs.update-v2') ||
        !positiveLimit('maxFullSyncBytes') ||
        !positiveLimit('maxLegacyRealtimeMessageBytes') ||
        !positiveLimit('maxBatchBytes') ||
        !positiveLimit('maxBatchObjects')) {
      throw const AppFlowyTransportFailure('INVALID_CAPABILITIES');
    }
    return AppFlowyWriteCapabilities(
      workspaceId: workspaceId,
      objectId: objectId,
      collabType: collabType,
      canWrite: data['canWrite'] as bool,
      snapshotValidationProfiles: snapshotProfiles.cast<String>(),
      qualifiedEditing: data['qualifiedEditing'] as bool,
      maxFullSyncBytes: data['maxFullSyncBytes'] as int,
      maxLegacyRealtimeMessageBytes:
          data['maxLegacyRealtimeMessageBytes'] as int,
      maxBatchBytes: data['maxBatchBytes'] as int,
      maxBatchObjects: data['maxBatchObjects'] as int,
    );
  }

  /// Model identity is separate from the original capability descriptor. A
  /// persisted epoch does not assert that fencing or advanced editing exists.
  Future<AppFlowyModelContract?> readModelContract(String objectId) async {
    _object(objectId);
    final response = await _request(
      'GET',
      '/api/workspace/v1/$workspaceId/collab/$objectId/model-contract?collab_type=$collabType',
    );
    if (_closed) throw const AppFlowyTransportFailure('SESSION_CLOSED');
    if (response.statusCode == 404) {
      Object? envelope;
      try {
        envelope = jsonDecode(response.body);
      } on FormatException {
        /* Older server route. */
      }
      if (envelope is! Map || !envelope.containsKey('code')) return null;
    }
    final data = _data(response)['data'];
    if (data is! Map || data['protocol'] != AppFlowyModelContract.protocol) {
      throw const AppFlowyTransportFailure('MODEL_CONTRACT_UNSUPPORTED');
    }
    if (data['workspaceId'] is! String ||
        (data['workspaceId'] as String).toLowerCase() !=
            workspaceId.toLowerCase() ||
        data['objectId'] is! String ||
        (data['objectId'] as String).toLowerCase() != objectId.toLowerCase() ||
        data['collabType'] is! int ||
        data['collabType'] != collabType) {
      throw const AppFlowyTransportFailure('COLLABORATION_OBJECT_MISMATCH');
    }
    final profile = data['modelProfile'],
        epoch = data['epoch'],
        binding = data['binding'];
    final extensions = data['semanticExtensions'];
    final validEpoch =
        epoch is String &&
        RegExp(r'^[1-9][0-9]{0,18}$').hasMatch(epoch) &&
        BigInt.parse(epoch) <= BigInt.parse('9223372036854775807');
    final implicit = binding == 'implicit-original';
    if (profile is! String ||
        profile.isEmpty ||
        profile.length > 128 ||
        !RegExp(r'^[a-z0-9][a-z0-9._/-]*$').hasMatch(profile) ||
        !data.containsKey('epoch') ||
        (binding != 'persisted' && !implicit) ||
        (implicit
            ? epoch != null || profile != AppFlowyWriteCapabilities.profile
            : !validEpoch) ||
        data['originalPipelineAvailable'] != implicit ||
        data['writerFencing'] != false ||
        extensions is! List ||
        extensions.isNotEmpty) {
      throw const AppFlowyTransportFailure('INVALID_MODEL_CONTRACT');
    }
    return AppFlowyModelContract(
      workspaceId: workspaceId,
      objectId: objectId,
      collabType: collabType,
      modelProfile: profile,
      epoch: epoch as String?,
      binding: binding as String,
      originalPipelineAvailable: implicit,
    );
  }

  @override
  Future<Uint8List?> readState(String objectId) async {
    _object(objectId);
    final response = await _request(
      'GET',
      '/api/workspace/v1/$workspaceId/collab/$objectId?collab_type=$collabType',
    );
    if (response.statusCode == 401 || response.statusCode == 403)
      throw const AppFlowyTransportFailure('FORBIDDEN');
    Object? envelope;
    try {
      envelope = jsonDecode(response.body);
    } on FormatException {
      /* Validate HTTP status below. */
    }
    if (envelope is Map && envelope['code'] == -4)
      throw const AppFlowyTransportFailure('OBJECT_DELETED');
    if (response.statusCode == 404) return null;
    // Only an authorized successful RecordNotFound response permits creation.
    if (response.statusCode == 200 && envelope is Map && envelope['code'] == -2)
      return null;
    final data = _data(response)['data'] as Map;
    return Uint8List.fromList((data['doc_state'] as List).cast<int>());
  }

  @override
  Future<void> initializeState(String objectId, Uint8List encodedCollab) async {
    _object(objectId);
    _data(
      await _request(
        'POST',
        '/api/workspace/$workspaceId/collab/$objectId',
        body: {
          'workspace_id': workspaceId,
          'object_id': objectId,
          'encoded_collab_v1': encodedCollab.toList(),
          'collab_type': collabType,
        },
      ),
    );
  }

  @override
  Future<Uint8List> exchange(String objectId, Uint8List request) async {
    _object(objectId);
    final response = await _request(
      'POST',
      '/api/workspace/v1/$workspaceId/collab/$objectId/full-sync',
      body: request,
      binary: true,
    );
    if (response.statusCode != 200 ||
        (response.headers['content-type'] ?? '').contains('application/json')) {
      _data(response);
      throw const AppFlowyTransportFailure('INVALID_SYNC_RESPONSE');
    }
    // Full-sync is an accepted exchange, not a durable-write acknowledgement.
    return response.bodyBytes;
  }

  @override
  void close() {
    _closed = true;
    _client.close();
  }
}

final class AppFlowyTransportFailure implements Exception {
  const AppFlowyTransportFailure(this.code);
  final String code;
  @override
  String toString() => 'AppFlowyTransportFailure($code)';
}
