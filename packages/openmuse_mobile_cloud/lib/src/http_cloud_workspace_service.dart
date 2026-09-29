import 'dart:convert';
import 'dart:io';

import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

typedef AccessTokenProvider = Future<String?> Function();

final class HttpCloudWorkspaceService
    implements
        CloudWorkspaceService,
        DshRuntimeConnector,
        ResourceRangePort,
        OfficeResourceCommitPort {
  HttpCloudWorkspaceService({
    required this.baseUri,
    required this.accessToken,
    HttpClient? client,
    this.allowHttpForTesting = false,
    this.maxJsonBytes = 1024 * 1024,
  }) : _client = client ?? HttpClient() {
    if ((!allowHttpForTesting && baseUri.scheme != 'https') ||
        (allowHttpForTesting && !{'http', 'https'}.contains(baseUri.scheme)) ||
        baseUri.host.isEmpty ||
        baseUri.hasQuery ||
        baseUri.hasFragment ||
        (baseUri.path.isNotEmpty && baseUri.path != '/')) {
      throw ArgumentError.value(
        baseUri,
        'baseUri',
        'must be an origin-only HTTPS URI',
      );
    }
  }

  final Uri baseUri;
  final AccessTokenProvider accessToken;
  final bool allowHttpForTesting;
  final int maxJsonBytes;
  final HttpClient _client;

  @override
  DshPlacement get placement => DshPlacement.cloudRemote;

  void closeClient() => _client.close(force: true);

  @override
  Future<List<CloudWorkspaceRecord>> listWorkspaces() async {
    final value = await _json('GET', '/v1/workspaces');
    final items = _list(value, 'items');
    return items
        .map((item) {
          final map = _map(item, 'workspace');
          return CloudWorkspaceRecord(
            workspaceRef: _string(map, 'workspaceRef'),
            title: _string(map, 'title'),
            revision: _string(map, 'revision'),
            writable: _bool(map, 'writable'),
            storageState: CloudStorageState.values.byName(
              _string(map, 'storageState'),
            ),
          );
        })
        .toList(growable: false);
  }

  @override
  Future<DshSessionDescriptor> open(String workspaceRef, int generation) async {
    final value = await _json(
      'POST',
      '/v1/dsh/sessions',
      body: {'workspaceRef': workspaceRef, 'generation': generation},
    );
    return DshSessionDescriptor(
      sessionRef: _string(value, 'sessionRef'),
      origin: _string(value, 'origin'),
      path: _string(value, 'path'),
      generation: _integer(value, 'generation'),
    );
  }

  @override
  Future<void> close(String sessionRef) async {
    await _json(
      'POST',
      '/v1/dsh/sessions/close',
      body: {'sessionRef': sessionRef},
    );
  }

  @override
  Future<ResourceHandle> issueResourceHandle({
    required String workspaceRef,
    required String resourceRef,
    required String revision,
    required String audience,
    required int generation,
  }) async {
    final value = await _json(
      'POST',
      '/v1/resources/handles',
      body: {
        'workspaceRef': workspaceRef,
        'resourceRef': resourceRef,
        'revision': revision,
        'audience': audience,
        'generation': generation,
      },
    );
    return ResourceHandle(
      resourceRef: _string(value, 'resourceRef'),
      revision: _string(value, 'revision'),
      audience: _string(value, 'audience'),
      generation: _integer(value, 'generation'),
      expiresAtMs: _integer(value, 'expiresAtMs'),
      size: _integer(value, 'size'),
      mediaType: _string(value, 'mediaType'),
    );
  }

  @override
  Future<List<int>> read(
    ResourceHandle handle,
    int start,
    int endExclusive,
  ) async {
    final response = await _request(
      'POST',
      '/v1/resources/range',
      body: {
        'resourceRef': handle.resourceRef,
        'revision': handle.revision,
        'audience': handle.audience,
        'generation': handle.generation,
        'expiresAtMs': handle.expiresAtMs,
        'start': start,
        'endExclusive': endExclusive,
      },
    );
    final bytes = <int>[];
    await for (final chunk in response) {
      bytes.addAll(chunk);
      if (bytes.length > endExclusive - start) {
        throw const CloudServiceException(
          CloudServiceErrorCode.invalidResponse,
          'range response exceeded request',
        );
      }
    }
    if (bytes.length != endExclusive - start) {
      throw const CloudServiceException(
        CloudServiceErrorCode.invalidResponse,
        'range response length mismatch',
      );
    }
    return bytes;
  }

  @override
  Future<OfficeResourceCommitReceipt> commit({
    required String resourceRef,
    required String expectedRevision,
    required List<int> bytes,
    required String idempotencyKey,
    required int generation,
  }) async {
    if (resourceRef.isEmpty ||
        expectedRevision.isEmpty ||
        idempotencyKey.isEmpty ||
        bytes.isEmpty ||
        bytes.length > 64 * 1024 * 1024 ||
        bytes.any((value) => value < 0 || value > 255)) {
      throw const CloudServiceException(
        CloudServiceErrorCode.invalidResponse,
        'invalid resource commit request',
      );
    }
    final request = await _openRequest('POST', '/v1/resources/commit');
    request.headers
      ..contentType = ContentType.binary
      ..set('OpenMuse-Resource-Ref', resourceRef)
      ..set('OpenMuse-Expected-Revision', expectedRevision)
      ..set('Idempotency-Key', idempotencyKey)
      ..set('OpenMuse-Generation', generation.toString());
    request.contentLength = bytes.length;
    request.add(bytes);
    final value = await _readJson(await _checkedResponse(request));
    return OfficeResourceCommitReceipt(
      commitRef: _string(value, 'commitRef'),
      resourceRef: _string(value, 'resourceRef'),
      previousRevision: _string(value, 'previousRevision'),
      newRevision: _string(value, 'newRevision'),
      generation: _integer(value, 'generation'),
    );
  }

  @override
  Future<CloudChangeProposal> propose({
    required String workspaceRef,
    required String expectedRevision,
    required String instruction,
    required int generation,
  }) async {
    final value = await _json(
      'POST',
      '/v1/proposals',
      body: {
        'workspaceRef': workspaceRef,
        'expectedRevision': expectedRevision,
        'instruction': instruction,
        'generation': generation,
      },
    );
    return CloudChangeProposal(
      proposalRef: _string(value, 'proposalRef'),
      workspaceRef: _string(value, 'workspaceRef'),
      expectedRevision: _string(value, 'expectedRevision'),
      summary: _string(value, 'summary'),
      generation: _integer(value, 'generation'),
    );
  }

  @override
  Future<CloudApplyReceipt> approve({
    required CloudChangeProposal proposal,
    required int generation,
  }) async {
    final value = await _json(
      'POST',
      '/v1/proposals/apply',
      body: {
        'proposalRef': proposal.proposalRef,
        'workspaceRef': proposal.workspaceRef,
        'expectedRevision': proposal.expectedRevision,
        'generation': generation,
      },
    );
    return CloudApplyReceipt(
      receiptRef: _string(value, 'receiptRef'),
      workspaceRef: _string(value, 'workspaceRef'),
      previousRevision: _string(value, 'previousRevision'),
      newRevision: _string(value, 'newRevision'),
      generation: _integer(value, 'generation'),
    );
  }

  Future<Map<String, Object?>> _json(
    String method,
    String path, {
    Map<String, Object?>? body,
  }) async {
    final response = await _request(method, path, body: body);
    return _readJson(response);
  }

  Future<Map<String, Object?>> _readJson(HttpClientResponse response) async {
    final bytes = <int>[];
    await for (final chunk in response) {
      bytes.addAll(chunk);
      if (bytes.length > maxJsonBytes) {
        throw const CloudServiceException(
          CloudServiceErrorCode.invalidResponse,
          'JSON response too large',
        );
      }
    }
    try {
      return _map(jsonDecode(utf8.decode(bytes)), 'response');
    } catch (error) {
      if (error is CloudServiceException) rethrow;
      throw const CloudServiceException(
        CloudServiceErrorCode.invalidResponse,
        'invalid JSON response',
      );
    }
  }

  Future<HttpClientResponse> _request(
    String method,
    String path, {
    Map<String, Object?>? body,
  }) async {
    final request = await _openRequest(method, path);
    if (body != null) {
      request.headers.contentType = ContentType.json;
      request.write(jsonEncode(body));
    }
    return _checkedResponse(request);
  }

  Future<HttpClientRequest> _openRequest(String method, String path) async {
    final token = await accessToken();
    if (token == null || token.isEmpty || token.length > 8192) {
      throw const CloudServiceException(
        CloudServiceErrorCode.unauthorized,
        'login required',
      );
    }
    final request = await _client.openUrl(method, baseUri.resolve(path));
    request.followRedirects = false;
    request.headers
      ..set(HttpHeaders.authorizationHeader, 'Bearer $token')
      ..set(HttpHeaders.acceptHeader, 'application/json');
    return request;
  }

  Future<HttpClientResponse> _checkedResponse(HttpClientRequest request) async {
    final response = await request.close();
    if (response.statusCode >= 200 && response.statusCode < 300)
      return response;
    await response.drain<void>();
    throw CloudServiceException(switch (response.statusCode) {
      401 || 403 => CloudServiceErrorCode.unauthorized,
      409 => CloudServiceErrorCode.staleRevision,
      424 || 503 => CloudServiceErrorCode.storageUnavailable,
      _ => CloudServiceErrorCode.unavailable,
    }, 'Cloud request failed with HTTP ${response.statusCode}');
  }
}

Map<String, Object?> _map(Object? value, String name) {
  if (value is! Map<String, Object?>) {
    throw CloudServiceException(
      CloudServiceErrorCode.invalidResponse,
      '$name must be an object',
    );
  }
  return value;
}

List<Object?> _list(Map<String, Object?> value, String key) {
  final result = value[key];
  if (result is! List<Object?>)
    throw CloudServiceException(
      CloudServiceErrorCode.invalidResponse,
      '$key must be a list',
    );
  return result;
}

String _string(Map<String, Object?> value, String key) {
  final result = value[key];
  if (result is! String || result.isEmpty)
    throw CloudServiceException(
      CloudServiceErrorCode.invalidResponse,
      '$key must be a string',
    );
  return result;
}

int _integer(Map<String, Object?> value, String key) {
  final result = value[key];
  if (result is! int || result < 0)
    throw CloudServiceException(
      CloudServiceErrorCode.invalidResponse,
      '$key must be an integer',
    );
  return result;
}

bool _bool(Map<String, Object?> value, String key) {
  final result = value[key];
  if (result is! bool)
    throw CloudServiceException(
      CloudServiceErrorCode.invalidResponse,
      '$key must be a boolean',
    );
  return result;
}
