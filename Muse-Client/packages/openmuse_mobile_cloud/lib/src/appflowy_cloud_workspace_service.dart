import 'dart:convert';
import 'dart:io';

import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

typedef RefreshAccessTokenProvider = Future<String?> Function();

final class AppFlowyCloudWorkspaceService
    implements
        CloudWorkspaceService,
        CloudResourceCatalogPort,
        DshRuntimeConnector,
        DshSessionCatalogPort,
        ResourceRangePort,
        OfficeResourceCommitPort {
  AppFlowyCloudWorkspaceService({
    required this.baseUri,
    required this.accessToken,
    required this.deviceId,
    this.refreshAccessToken,
    HttpClient? client,
    this.allowHttpForTesting = false,
    this.maxJsonBytes = 1024 * 1024,
  }) : _client = client ?? HttpClient() {
    final loopback =
        baseUri.host == 'localhost' ||
        baseUri.host == '127.0.0.1' ||
        baseUri.host == '::1' ||
        baseUri.host == '10.0.2.2';
    if (baseUri.host.isEmpty ||
        baseUri.hasQuery ||
        baseUri.hasFragment ||
        (baseUri.path.isNotEmpty && baseUri.path != '/') ||
        (baseUri.scheme != 'https' &&
            !(allowHttpForTesting && baseUri.scheme == 'http' && loopback)) ||
        deviceId.isEmpty ||
        deviceId.length > 256) {
      throw ArgumentError('invalid AppFlowy Cloud endpoint or device id');
    }
  }

  final Uri baseUri;
  final Future<String?> Function() accessToken;
  final RefreshAccessTokenProvider? refreshAccessToken;
  final String deviceId;
  final bool allowHttpForTesting;
  final int maxJsonBytes;
  final HttpClient _client;

  @override
  DshPlacement get placement => DshPlacement.cloudRemote;

  void closeClient() => _client.close(force: true);

  @override
  Future<List<CloudWorkspaceRecord>> listWorkspaces() async {
    final data = await _envelope('GET', '/api/workspace');
    if (data is! List) throw _invalid('workspace data must be a list');
    return data
        .map((item) {
          final value = _object(item, 'workspace');
          final role = value['role']?.toString().toLowerCase();
          return CloudWorkspaceRecord(
            workspaceRef: _requiredString(value, 'workspace_id'),
            title: _requiredString(value, 'workspace_name'),
            revision:
                _optionalString(value, 'database_storage_id') ??
                _optionalString(value, 'created_at') ??
                'cloud',
            writable: role == null || !role.contains('guest'),
            storageState: CloudStorageState.available,
          );
        })
        .toList(growable: false);
  }

  @override
  Future<List<DshSessionSummary>> listSessions() async {
    final data = await _envelope('GET', '/api/muse/dsh/sessions');
    final value = _object(data, 'session list');
    final items = value['items'];
    if (items is! List) throw _invalid('session items must be a list');
    return items
        .map((item) {
          final session = _object(item, 'session');
          return DshSessionSummary(
            sessionRef: _requiredString(session, 'sessionRef'),
            workspaceRef: _requiredString(session, 'workspaceRef'),
            state: _requiredString(session, 'state'),
            instanceRef: _optionalString(session, 'instanceRef'),
            nodeId: _requiredString(session, 'nodeId'),
            createdAtMs: _requiredInt(session, 'createdAt'),
            lastActiveAtMs: _requiredInt(session, 'lastActiveAt'),
            attachedDeviceCount: _requiredInt(session, 'attachedDeviceCount'),
            queuePosition: _optionalInt(session, 'queuePosition'),
          );
        })
        .toList(growable: false);
  }

  @override
  Future<DshSessionDescriptor> open(String workspaceRef, int generation) async {
    final data = await _envelope(
      'POST',
      '/api/muse/dsh/session/open',
      body: {'workspaceId': workspaceRef, 'deviceId': deviceId},
    );
    final value = _object(data, 'session');
    final webUrl = Uri.tryParse(_requiredString(value, 'webUrl'));
    if (webUrl == null || webUrl.host.isEmpty || webUrl.path.isEmpty) {
      throw _invalid('webUrl must be an absolute URL');
    }
    final origin = Uri(
      scheme: webUrl.scheme,
      host: webUrl.host,
      port: webUrl.hasPort ? webUrl.port : null,
    ).toString();
    if (webUrl.hasFragment)
      throw _invalid('webUrl must not contain a fragment');
    final path = webUrl.hasQuery
        ? '${webUrl.path}?${webUrl.query}'
        : webUrl.path;
    return DshSessionDescriptor(
      sessionRef: _requiredString(value, 'sessionRef'),
      origin: origin,
      path: path,
      generation: generation,
      allowInsecureLoopback: allowHttpForTesting,
    );
  }

  @override
  Future<void> close(String sessionRef) async {
    await _envelope(
      'POST',
      '/api/muse/dsh/session/close',
      body: {'sessionRef': sessionRef, 'deviceId': deviceId},
    );
  }

  @override
  Future<List<CloudResourceRecord>> listResources({
    required String workspaceRef,
    required String revision,
    required int generation,
  }) async {
    final data = await _envelope(
      'POST',
      '/api/muse/workspace/tree',
      body: {'workspaceId': workspaceRef, 'depth': 4, 'limit': 64},
    );
    final value = _object(data, 'workspace tree');
    final items = value['items'];
    if (items is! List) throw _invalid('workspace tree items must be a list');
    return items
        .map(_resourceFromTreeItem)
        .whereType<CloudResourceRecord>()
        .toList(growable: false);
  }

  CloudResourceRecord? _resourceFromTreeItem(Object? item) {
    final value = _object(item, 'workspace tree item');
    if (value['isSpace'] == true) return null;
    final layout = _requiredString(value, 'layout');
    return CloudResourceRecord(
      resourceRef: _requiredString(value, 'viewId'),
      title: _requiredString(value, 'title'),
      revision: 'view',
      size: 0,
      mediaType: 'application/x-openmuse-$layout',
      writable: false,
    );
  }

  @override
  Future<ResourceHandle> issueResourceHandle({
    required String workspaceRef,
    required String resourceRef,
    required String revision,
    required String audience,
    required int generation,
  }) => throw _unavailable('resource handles are not available on this server');

  @override
  Future<CloudChangeProposal> propose({
    required String workspaceRef,
    required String expectedRevision,
    required String instruction,
    required int generation,
  }) => throw _unavailable(
    'workspace proposals are not available on this server',
  );

  @override
  Future<CloudApplyReceipt> approve({
    required CloudChangeProposal proposal,
    required int generation,
  }) => throw _unavailable('workspace apply is not available on this server');

  @override
  Future<List<int>> read(ResourceHandle handle, int start, int endExclusive) =>
      throw _unavailable('resource ranges are not available on this server');

  @override
  Future<OfficeResourceCommitReceipt> commit({
    required String resourceRef,
    required String expectedRevision,
    required List<int> bytes,
    required String idempotencyKey,
    required int generation,
  }) => throw _unavailable('Office commits are not available on this server');

  Future<Object?> _envelope(
    String method,
    String path, {
    Map<String, Object?>? body,
  }) async {
    var response = await _request(method, path, body: body);
    if (response.statusCode == 401 && refreshAccessToken != null) {
      await response.drain<void>();
      final refreshed = await refreshAccessToken!();
      if (refreshed != null && refreshed.isNotEmpty) {
        response = await _request(method, path, body: body);
      }
    }
    if (response.statusCode < 200 || response.statusCode >= 300) {
      final status = response.statusCode;
      await response.drain<void>();
      throw CloudServiceException(
        status == 401 || status == 403
            ? CloudServiceErrorCode.unauthorized
            : CloudServiceErrorCode.unavailable,
        'Cloud request failed with HTTP $status',
      );
    }
    final root = _object(await _readJson(response), 'Cloud envelope');
    final code = root['code'];
    if (code is! num || code.toInt() != 0) {
      throw const CloudServiceException(
        CloudServiceErrorCode.unavailable,
        'Cloud returned a business error',
      );
    }
    return root['data'];
  }

  Future<HttpClientResponse> _request(
    String method,
    String path, {
    Map<String, Object?>? body,
  }) async {
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
    if (body != null) {
      request.headers.contentType = ContentType.json;
      request.write(jsonEncode(body));
    }
    return request.close();
  }

  Future<Object?> _readJson(HttpClientResponse response) async {
    final bytes = <int>[];
    await for (final chunk in response) {
      bytes.addAll(chunk);
      if (bytes.length > maxJsonBytes)
        throw _invalid('JSON response too large');
    }
    try {
      return jsonDecode(utf8.decode(bytes));
    } on FormatException {
      throw _invalid('invalid JSON response');
    }
  }
}

Map<String, Object?> _object(Object? value, String name) {
  if (value is! Map) throw _invalid('$name must be an object');
  return value.cast<String, Object?>();
}

String _requiredString(Map<String, Object?> value, String key) {
  final result = value[key];
  if (result is! String || result.isEmpty)
    throw _invalid('$key must be a string');
  return result;
}

String? _optionalString(Map<String, Object?> value, String key) {
  final result = value[key];
  return result is String && result.isNotEmpty ? result : null;
}

int _requiredInt(Map<String, Object?> value, String key) {
  final result = value[key];
  if (result is! num || result.toInt() < 0)
    throw _invalid('$key must be an integer');
  return result.toInt();
}

int? _optionalInt(Map<String, Object?> value, String key) {
  final result = value[key];
  return result is num && result.toInt() >= 0 ? result.toInt() : null;
}

CloudServiceException _invalid(String message) =>
    CloudServiceException(CloudServiceErrorCode.invalidResponse, message);

CloudServiceException _unavailable(String message) =>
    CloudServiceException(CloudServiceErrorCode.unavailable, message);
