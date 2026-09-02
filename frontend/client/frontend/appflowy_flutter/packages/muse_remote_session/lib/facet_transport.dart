import 'dart:async';
import 'dart:convert';
import 'dart:io';

/// Bytes/JSON only: no workspace, document, intent, or WebView dependencies.
abstract interface class MuseJsonTransport {
  Future<Map<String, Object?>> get(String suffix);
  Future<Map<String, Object?>> send(Map<String, Object?> message);
  Stream<Map<String, Object?>> events();
  void close();
}

final class MuseTransportException implements Exception {
  const MuseTransportException(this.code, [this.status]);
  final String code;
  final int? status;
  @override
  String toString() => 'MuseTransportException($code, $status)';
}

final class MuseHttpSseTransport implements MuseJsonTransport {
  MuseHttpSseTransport({
    required this.endpoint,
    Map<String, String> headers = const {},
    this.timeout = const Duration(seconds: 12),
    bool allowLoopbackForTests = false,
  }) : _headers = Map.unmodifiable(headers) {
    final testLoopback = allowLoopbackForTests &&
        endpoint.scheme == 'http' &&
        (endpoint.host == '127.0.0.1' || endpoint.host == '::1');
    if ((!testLoopback && endpoint.scheme != 'https') ||
        endpoint.host.isEmpty ||
        endpoint.userInfo.isNotEmpty ||
        endpoint.hasQuery ||
        endpoint.hasFragment) {
      throw const MuseTransportException('INVALID_ENDPOINT');
    }
    _client.connectionTimeout = timeout;
  }

  final Uri endpoint;
  final Duration timeout;
  final Map<String, String> _headers;
  final HttpClient _client = HttpClient();
  bool _closed = false;
  bool _streamOpened = false;

  Future<HttpClientResponse> _request(
    String method,
    String suffix, [
    Map<String, Object?>? message,
  ]) async {
    if (_closed) throw const MuseTransportException('TRANSPORT_CLOSED');
    if (!RegExp(r'^[a-z-]*$').hasMatch(suffix)) {
      throw const MuseTransportException('INVALID_PATH');
    }
    final uri = endpoint.replace(
      path: '${endpoint.path}${suffix.isEmpty ? '' : '/$suffix'}',
    );
    final request = await _client.openUrl(method, uri).timeout(timeout);
    request.followRedirects = false; // Never forward credentials to a redirect.
    _headers.forEach(request.headers.set);
    if (message != null) {
      final bytes = utf8.encode(jsonEncode(message));
      if (bytes.length > 32768) {
        request.abort();
        throw const MuseTransportException('PAYLOAD_TOO_LARGE');
      }
      request.headers.contentType = ContentType.json;
      request.add(bytes);
    }
    final response = await request.close().timeout(timeout);
    if (response.statusCode != 200) {
      // Do not expose server response text (which may contain sensitive data).
      await response.listen((_) {}).cancel();
      throw MuseTransportException('HTTP_ERROR', response.statusCode);
    }
    return response;
  }

  Future<Map<String, Object?>> _json(HttpClientResponse response) async {
    if (response.headers.contentType?.mimeType != 'application/json') {
      await response.listen((_) {}).cancel();
      throw const MuseTransportException('INVALID_RESPONSE');
    }
    final bytes = <int>[];
    await for (final chunk in response.timeout(timeout)) {
      if (bytes.length + chunk.length > 32768) {
        throw const MuseTransportException('PAYLOAD_TOO_LARGE');
      }
      bytes.addAll(chunk);
    }
    final Object? value;
    try {
      value = jsonDecode(utf8.decode(bytes));
    } on FormatException {
      throw const MuseTransportException('INVALID_RESPONSE');
    }
    if (value is! Map<String, dynamic>) {
      throw const MuseTransportException('INVALID_RESPONSE');
    }
    return value;
  }

  @override
  Future<Map<String, Object?>> get(String suffix) async =>
      _json(await _request('GET', suffix));
  @override
  Future<Map<String, Object?>> send(Map<String, Object?> message) async =>
      _json(await _request('POST', '', message));
  @override
  Stream<Map<String, Object?>> events() async* {
    if (_streamOpened) {
      throw const MuseTransportException('STREAM_ALREADY_OPEN');
    }
    _streamOpened = true;
    final response = await _request('GET', 'events');
    if (response.headers.contentType?.mimeType != 'text/event-stream') {
      await response.listen((_) {}).cancel();
      throw const MuseTransportException('INVALID_STREAM_TYPE');
    }
    yield* decodeMuseSse(response.timeout(const Duration(seconds: 45)));
  }

  @override
  void close() {
    _closed = true;
    _client.close(force: true);
  }
}

/// Bounded SSE parser: fragmented UTF-8, CRLF, comments, and multiline data.
/// Intentionally has no cursor/replay semantics.
Stream<Map<String, Object?>> decodeMuseSse(Stream<List<int>> bytes) async* {
  var pending = '';
  final data = <String>[];
  var eventBytes = 0;
  await for (final text in bytes.transform(utf8.decoder)) {
    for (final part in text.split('\n').asMap().entries) {
      if (part.key > 0) {
        final line = pending.endsWith('\r')
            ? pending.substring(0, pending.length - 1)
            : pending;
        pending = '';
        if (line.isEmpty) {
          if (data.isNotEmpty) {
            final decoded = jsonDecode(data.join('\n'));
            if (decoded is! Map<String, dynamic>) {
              throw const MuseTransportException('INVALID_EVENT');
            }
            yield decoded;
          }
          data.clear();
          eventBytes = 0;
        } else if (line.startsWith('data:')) {
          final value = line.substring(5);
          data.add(value.startsWith(' ') ? value.substring(1) : value);
          eventBytes += utf8.encode(line).length + 1;
          if (eventBytes > 32768) {
            throw const MuseTransportException('PAYLOAD_TOO_LARGE');
          }
        }
      }
      pending += part.value;
      if (utf8.encode(pending).length > 32768) {
        throw const MuseTransportException('PAYLOAD_TOO_LARGE');
      }
    }
  }
  // An incomplete event at EOF is not acknowledged or replayed.
}
