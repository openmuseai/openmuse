import 'dart:async';
import 'dart:convert';
import 'package:http/http.dart' as http;

/// Browser transport for the small streaming HTTP surface used by the shared
/// Cloud service. Account fencing, retries and domain envelopes stay there.
final class HttpClient {
  final http.Client _client = http.Client();
  Future<HttpClientRequest> openUrl(String method, Uri uri) async =>
      HttpClientRequest(_client, method, uri);
  void close({bool force = false}) => _client.close();
}

final class HttpClientRequest {
  HttpClientRequest(this._client, this._method, this._uri);
  final http.Client _client;
  final String _method;
  final Uri _uri;
  final headers = HttpHeaders();
  final _body = StringBuffer();
  final _aborted = Completer<void>();
  bool followRedirects = false;
  void write(Object value) => _body.write(value);
  void abort([Object? error, StackTrace? stack]) {
    if (!_aborted.isCompleted) _aborted.complete();
  }

  Future<HttpClientResponse> close() async {
    final request = http.AbortableRequest(
      _method,
      _uri,
      abortTrigger: _aborted.future,
    )..followRedirects = followRedirects;
    request.headers.addAll(headers.values);
    request.bodyBytes = utf8.encode(_body.toString());
    try {
      return HttpClientResponse(await _client.send(request));
    } on http.ClientException {
      throw const HttpException('Cloud browser request failed');
    }
  }
}

final class HttpClientResponse extends StreamView<List<int>> {
  HttpClientResponse(http.StreamedResponse response)
    : statusCode = response.statusCode,
      super(response.stream);
  final int statusCode;
}

final class HttpHeaders {
  static const authorizationHeader = 'authorization';
  static const acceptHeader = 'accept';
  final values = <String, String>{};
  void set(String name, Object value) => values[name.toLowerCase()] = '$value';
  set contentType(ContentType value) => set('content-type', value.value);
}

final class ContentType {
  const ContentType(this.value);
  static const json = ContentType('application/json; charset=utf-8');
  final String value;
}

class HttpException implements Exception {
  const HttpException(this.message);
  final String message;
  @override
  String toString() => message;
}

final class SocketException extends HttpException {
  const SocketException(super.message);
}
