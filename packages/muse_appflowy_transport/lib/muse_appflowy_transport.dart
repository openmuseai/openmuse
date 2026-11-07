import 'dart:convert';
import 'dart:typed_data';
import 'package:http/http.dart' as http;
import 'package:muse_collab_contract/muse_collab_contract.dart';
export 'package:muse_collab_contract/muse_collab_contract.dart';

final class AppFlowyObjectTransport implements CollaborationProvider {
  AppFlowyObjectTransport({
    required this.origin,
    required this.binding,
    required this.collabType,
    required this.workspaceId,
    required this.accessToken,
    required this.deviceId,
    http.Client? client,
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
    final request = http.Request(method, origin.resolve(route))
      ..followRedirects = false;
    request.headers.addAll({
      'Authorization': 'Bearer ${await accessToken()}',
      'Device-Id': deviceId,
      'client-version': '0.16.5',
      'Content-Type': binary ? 'application/octet-stream' : 'application/json',
    });
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
    return http.Response.bytes(
      bytes.takeBytes(),
      response.statusCode,
      headers: response.headers,
    );
  }

  Map<String, dynamic> _data(http.Response response) {
    if (response.statusCode == 401 || response.statusCode == 403)
      throw const AppFlowyTransportFailure('FORBIDDEN');
    Map<String, dynamic> result;
    try {
      result = (jsonDecode(response.body) as Map).cast<String, dynamic>();
    } on Object {
      throw AppFlowyTransportFailure('CLOUD_HTTP_${response.statusCode}');
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
    if (!RegExp(
      r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$',
    ).hasMatch(objectId))
      throw const AppFlowyTransportFailure('INVALID_OBJECT_ID');
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
