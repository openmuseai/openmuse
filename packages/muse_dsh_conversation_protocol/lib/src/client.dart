import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'models.dart';
import 'native_ui.dart';

final class DshNativeGatewayException implements Exception {
  const DshNativeGatewayException(this.message, {this.statusCode});
  final String message;
  final int? statusCode;

  @override
  String toString() => message;
}

final class DshNativeGatewayClient {
  DshNativeGatewayClient({
    required this.origin,
    required this.bootstrapPath,
    this.allowInsecureLoopback = false,
    this.allowInsecurePrivateNetworkForTesting = false,
    this.requestTimeout = const Duration(seconds: 20),
    Map<String, String> headers = const {},
    HttpClient? client,
  }) : headers = Map.unmodifiable(headers),
       _client = client ?? HttpClient() {
    _client.connectionTimeout = requestTimeout;
  }

  final Uri origin;
  final String bootstrapPath;
  final bool allowInsecureLoopback;
  final bool allowInsecurePrivateNetworkForTesting;
  final Duration requestTimeout;
  final Map<String, String> headers;
  final HttpClient _client;
  final Map<String, Cookie> _cookies = {};
  bool _bootstrapped = false;

  Future<DshGatewayHello> initialize() async {
    _validateOrigin();
    await _bootstrap();
    final value = await _jsonRequest('GET', '/openmuse-native/v1/hello');
    return DshGatewayHello.fromJson(value);
  }

  Future<List<DshNativeSessionSummary>> listSessions() async {
    final value = await _jsonRequest('GET', '/openmuse-native/v1/sessions');
    final items = value['items'];
    if (items is! List) throw const FormatException('sessions.items missing');
    return items
        .whereType<Map>()
        .map(
          (item) =>
              DshNativeSessionSummary.fromJson(item.cast<String, Object?>()),
        )
        .toList(growable: false);
  }

  Future<List<DshNativeWorkspaceSummary>> listWorkspaces() async {
    final value = await _jsonRequest('GET', '/openmuse-native/v1/workspaces');
    final items = value['items'];
    if (items is! List) throw const FormatException('workspaces.items missing');
    return items
        .whereType<Map>()
        .map(
          (item) =>
              DshNativeWorkspaceSummary.fromJson(item.cast<String, Object?>()),
        )
        .toList(growable: false);
  }

  Future<DshNativeCreatedSession> createSession({
    required String workspaceId,
    String agentPreset = 'standard',
  }) async => DshNativeCreatedSession.fromJson(
    await _jsonRequest(
      'POST',
      '/openmuse-native/v1/session/create',
      body: {'workspaceId': workspaceId, 'agentPreset': agentPreset},
    ),
  );

  Future<DshNativeNegotiation> negotiate(
    DshNativeCapabilities capabilities,
  ) async => DshNativeNegotiation.fromJson(
    await _jsonRequest(
      'POST',
      '/openmuse-native/v1/negotiate',
      body: capabilities.toJson(),
    ),
  );

  Stream<DshFollowFrame> follow(String sessionId) async* {
    final uri = origin
        .resolve('/openmuse-native/v1/session/follow')
        .replace(queryParameters: {'sessionId': sessionId});
    final request = await _client.getUrl(uri).timeout(requestTimeout);
    _applyHeaders(request);
    final response = await request.close().timeout(requestTimeout);
    _rememberCookies(response.cookies);
    if (response.statusCode != HttpStatus.ok) {
      throw DshNativeGatewayException(
        await _failureMessage(response),
        statusCode: response.statusCode,
      );
    }
    String? eventName;
    final data = StringBuffer();
    await for (final line
        in response.transform(utf8.decoder).transform(const LineSplitter())) {
      if (line.isEmpty) {
        if (data.isNotEmpty) {
          final decoded = jsonDecode(data.toString());
          if (eventName == 'error') {
            final value = jsonMap(decoded, 'SSE error');
            throw DshNativeGatewayException(
              value['detail'] as String? ?? 'DSH follow failed',
            );
          }
          yield DshFollowFrame.fromJson(jsonMap(decoded, 'SSE frame'));
        }
        eventName = null;
        data.clear();
        continue;
      }
      if (line.startsWith('event:')) {
        eventName = line.substring(6).trim();
      } else if (line.startsWith('data:')) {
        if (data.isNotEmpty) data.write('\n');
        data.write(line.substring(5).trimLeft());
      }
    }
  }

  /// Reads a bounded, durable journal page through the sequence advertised by
  /// [summary]. This is the recovery path for lost, buffered, or half-open SSE
  /// streams; callers can feed the returned snapshot into the same reducer as
  /// a follow snapshot.
  Future<DshSnapshotFrame> sessionPage(
    DshNativeSessionSummary summary, {
    int? beforeSeq,
  }) async {
    final throughSeq = summary.headSeq;
    final value = await _jsonRequest(
      'POST',
      '/openmuse-native/v1/session/page',
      body: {
        'sessionId': summary.sessionId,
        'throughSeq': throughSeq,
        if (beforeSeq != null) 'beforeSeq': beforeSeq,
      },
    );
    return DshFollowFrame.fromJson({
          'type': 'snapshot',
          'header': {'id': summary.sessionId},
          'cursor': throughSeq,
          'records': value['records'],
          'hasMore': value['hasMore'] == true,
          'projections': summary.projections,
        })
        as DshSnapshotFrame;
  }

  Future<void> prompt({
    required String sessionId,
    required String requestId,
    required String text,
    String mode = 'queue',
  }) async {
    final value = await _jsonRequest(
      'POST',
      '/openmuse-native/v1/session/prompt',
      body: {
        'sessionId': sessionId,
        'requestId': requestId,
        'mode': mode,
        'content': [
          {'type': 'text', 'text': text},
        ],
      },
    );
    if (value['accepted'] != true) {
      throw const DshNativeGatewayException('DSH rejected the prompt');
    }
  }

  Future<void> cancel(String sessionId) async {
    final value = await _jsonRequest(
      'POST',
      '/openmuse-native/v1/session/cancel',
      body: {'sessionId': sessionId},
    );
    if (value['accepted'] != true) {
      throw const DshNativeGatewayException('DSH rejected cancellation');
    }
  }

  Future<DshNativeSessionOptions> sessionOptions(
    String sessionId,
  ) async => DshNativeSessionOptions.fromJson(
    await _jsonRequest(
      'GET',
      '/openmuse-native/v1/session/options?sessionId=${Uri.encodeQueryComponent(sessionId)}',
    ),
  );

  Future<DshNativeModelSelection> selectModel({
    required String sessionId,
    required String provider,
    required String model,
    String? reasoningEffort,
  }) async {
    final value = await _jsonRequest(
      'POST',
      '/openmuse-native/v1/session/model',
      body: {
        'sessionId': sessionId,
        'provider': provider,
        'model': model,
        if (reasoningEffort != null) 'reasoningEffort': reasoningEffort,
      },
    );
    return DshNativeModelSelection.fromJson(jsonMap(value['selected']));
  }

  Future<String> selectPermission({
    required String sessionId,
    required String preset,
  }) async {
    final value = await _jsonRequest(
      'POST',
      '/openmuse-native/v1/session/permission',
      body: {'sessionId': sessionId, 'preset': preset},
    );
    final selected = value['selected'];
    if (selected is! String || selected.isEmpty) {
      throw const FormatException('selected permission missing');
    }
    return selected;
  }

  Future<void> answerQuestion({
    required String sessionId,
    required List<JsonMap> answers,
  }) async {
    final value = await _jsonRequest(
      'POST',
      '/openmuse-native/v1/session/question/answer',
      body: {'sessionId': sessionId, 'answers': answers},
    );
    if (value['accepted'] != true) {
      throw const DshNativeGatewayException('DSH rejected the question answer');
    }
  }

  Future<DshNativeWorkspaceChanges> workspaceChanges({
    required String sessionId,
    required int seq,
  }) async => DshNativeWorkspaceChanges.fromJson(
    await _jsonRequest(
      'GET',
      '/openmuse-native/v1/session/changes?sessionId=${Uri.encodeQueryComponent(sessionId)}&seq=$seq',
    ),
  );

  Future<DshNativeArtifactPreview> workspaceChangePreview({
    required String sessionId,
    required int seq,
    required int index,
  }) async => DshNativeArtifactPreview.fromJson(
    await _jsonRequest(
      'GET',
      '/openmuse-native/v1/session/changes/preview?sessionId=${Uri.encodeQueryComponent(sessionId)}&seq=$seq&index=$index',
    ),
  );

  Future<void> openWorkspaceChangeOnDesktop({
    required String sessionId,
    required int seq,
    required int index,
  }) async {
    final value = await _jsonRequest(
      'POST',
      '/openmuse-native/v1/session/changes/open',
      body: {'sessionId': sessionId, 'seq': seq, 'index': index},
    );
    if (value['opened'] != true) {
      throw const DshNativeGatewayException('Desktop did not open the file');
    }
  }

  void close() => _client.close(force: true);

  Future<void> _bootstrap() async {
    if (_bootstrapped || bootstrapPath.isEmpty) return;
    var target = origin.resolve(bootstrapPath);
    for (var redirect = 0; redirect < 4; redirect++) {
      final request = await _client.getUrl(target).timeout(requestTimeout);
      request.followRedirects = false;
      _applyHeaders(request);
      final response = await request.close().timeout(requestTimeout);
      _rememberCookies(response.cookies);
      await response.drain<void>();
      if (!response.isRedirect) {
        if (response.statusCode >= 400) {
          throw DshNativeGatewayException(
            'DSH bootstrap failed (${response.statusCode})',
            statusCode: response.statusCode,
          );
        }
        _bootstrapped = true;
        return;
      }
      final location = response.headers.value(HttpHeaders.locationHeader);
      if (location == null) {
        throw const DshNativeGatewayException(
          'DSH bootstrap redirect has no location',
        );
      }
      final next = target.resolve(location);
      if (!_sameOrigin(next, origin)) {
        throw const DshNativeGatewayException(
          'DSH bootstrap attempted a cross-origin redirect',
        );
      }
      target = next;
    }
    throw const DshNativeGatewayException('Too many DSH bootstrap redirects');
  }

  Future<JsonMap> _jsonRequest(
    String method,
    String path, {
    JsonMap? body,
  }) async {
    final request = await _client
        .openUrl(method, origin.resolve(path))
        .timeout(requestTimeout);
    _applyHeaders(request);
    if (body != null) {
      request.headers.contentType = ContentType.json;
      request.add(utf8.encode(jsonEncode(body)));
    }
    final response = await request.close().timeout(requestTimeout);
    _rememberCookies(response.cookies);
    final bytes = await response
        .fold<List<int>>(<int>[], (value, chunk) => value..addAll(chunk))
        .timeout(requestTimeout);
    final decoded = bytes.isEmpty
        ? <String, Object?>{}
        : jsonDecode(utf8.decode(bytes));
    if (response.statusCode < 200 || response.statusCode >= 300) {
      final value = decoded is Map ? decoded.cast<String, Object?>() : null;
      throw DshNativeGatewayException(
        value?['detail'] as String? ??
            value?['error'] as String? ??
            'DSH request failed (${response.statusCode})',
        statusCode: response.statusCode,
      );
    }
    return jsonMap(decoded, 'response');
  }

  void _applyHeaders(HttpClientRequest request) {
    for (final entry in headers.entries) {
      request.headers.set(entry.key, entry.value);
    }
    if (_cookies.isNotEmpty) {
      request.cookies.addAll(_cookies.values);
    }
  }

  void _rememberCookies(List<Cookie> cookies) {
    for (final cookie in cookies) {
      _cookies[cookie.name] = cookie;
    }
  }

  Future<String> _failureMessage(HttpClientResponse response) async {
    try {
      final decoded = jsonDecode(await utf8.decodeStream(response));
      final value = jsonMap(decoded);
      return value['detail'] as String? ??
          value['error'] as String? ??
          'DSH follow failed';
    } on Object {
      return 'DSH follow failed (${response.statusCode})';
    }
  }

  void _validateOrigin() {
    final loopback = const {
      'localhost',
      '127.0.0.1',
      '::1',
      '10.0.2.2',
    }.contains(origin.host);
    final safe =
        origin.scheme == 'https' ||
        (origin.scheme == 'http' && allowInsecureLoopback && loopback) ||
        (origin.scheme == 'http' &&
            allowInsecurePrivateNetworkForTesting &&
            _isPrivateIpv4(origin.host));
    if (!safe || origin.userInfo.isNotEmpty) {
      throw const DshNativeGatewayException('Unsafe DSH origin');
    }
  }

  bool _sameOrigin(Uri first, Uri second) =>
      first.scheme == second.scheme &&
      first.host == second.host &&
      first.port == second.port;

  bool _isPrivateIpv4(String host) {
    final parts = host.split('.').map(int.tryParse).toList(growable: false);
    if (parts.length != 4 || parts.any((part) => part == null)) return false;
    return parts[0] == 10 ||
        (parts[0] == 192 && parts[1] == 168) ||
        (parts[0] == 172 && parts[1]! >= 16 && parts[1]! <= 31);
  }
}
