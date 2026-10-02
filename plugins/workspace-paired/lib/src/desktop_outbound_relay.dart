import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

enum DesktopRelayPhase { idle, connecting, attached, reconnecting }

/// Keeps one outbound WebSocket to the public relay and forwards each
/// stream to the loopback Desktop gateway. Presence stays on the device
/// directory; this loop only owns the data plane.
final class DesktopOutboundRelay {
  DesktopOutboundRelay({
    required this.attachUri,
    required this.accessToken,
    required this.deviceId,
    required this.localGateway,
    required this.publicOrigin,
    Random? random,
  }) : _random = random ?? Random();

  final Uri attachUri;
  final Future<String?> Function() accessToken;
  final String deviceId;
  final Uri? Function() localGateway;
  final Uri publicOrigin;
  final Random _random;
  DesktopRelayPhase _phase = DesktopRelayPhase.idle;
  void Function()? onChanged;
  bool _started = false;
  bool _stopped = false;
  WebSocket? _socket;
  final Map<int, HttpClient> _httpClients = {};

  DesktopRelayPhase get phase => _phase;

  Future<void> start() async {
    if (_started) return;
    _started = true;
    _stopped = false;
    unawaited(_loop());
  }

  Future<void> stop() async {
    _stopped = true;
    _started = false;
    final socket = _socket;
    _socket = null;
    await socket?.close();
    _setPhase(DesktopRelayPhase.idle);
  }

  Future<void> _loop() async {
    var failures = 0;
    while (!_stopped) {
      final token = await accessToken();
      if (_stopped) return;
      if (token == null || token.isEmpty) {
        _setPhase(DesktopRelayPhase.idle);
        await Future<void>.delayed(const Duration(seconds: 2));
        continue;
      }
      _setPhase(
        failures == 0
            ? DesktopRelayPhase.connecting
            : DesktopRelayPhase.reconnecting,
      );
      final startedAt = DateTime.now();
      var attached = false;
      try {
        await _session(token, () {
          attached = true;
          _setPhase(DesktopRelayPhase.attached);
        });
      } catch (error, stack) {
        stderr.writeln('OpenMuse desktop relay: $error');
        stderr.writeln(stack);
      }
      if (_stopped) return;
      final stable =
          attached &&
          DateTime.now().difference(startedAt) > const Duration(seconds: 30);
      failures = stable ? 0 : min(failures + 1, 6);
      _setPhase(DesktopRelayPhase.reconnecting);
      await Future<void>.delayed(_delay(failures == 0 ? 0 : failures - 1));
    }
  }

  Duration _delay(int step) {
    final baseMs = min(30000, 1000 * (1 << step.clamp(0, 5)));
    final jitter = baseMs <= 4 ? 0 : _random.nextInt(baseMs ~/ 4);
    return Duration(milliseconds: baseMs + jitter);
  }

  Future<void> _session(String token, void Function() onReady) async {
    final socket = await WebSocket.connect(
      attachUri.toString(),
      headers: {HttpHeaders.authorizationHeader: 'Bearer $token'},
    );
    _socket = socket;
    final upstreams = <int, WebSocket>{};
    try {
      socket.add(
        jsonEncode({
          'type': 'hello',
          'protocol': 1,
          'deviceId': deviceId,
        }),
      );
      await for (final message in socket) {
        if (_stopped) return;
        if (message is! String) continue;
        final decoded = jsonDecode(message);
        if (decoded is! Map) continue;
        final frame = decoded.cast<String, Object?>();
        switch (frame['type']) {
          case 'hello-ok':
            onReady();
          case 'ping':
            socket.add(jsonEncode({'type': 'pong', 'id': frame['id']}));
          case 'http.request':
            unawaited(_http(socket, frame));
          case 'http.abort':
            final id = frame['id'];
            if (id is int) {
              _httpClients.remove(id)?.close(force: true);
            }
          case 'ws.open':
            unawaited(_openWebSocket(socket, frame, upstreams));
          case 'ws.data':
            _forwardWebSocketData(upstreams, frame);
          case 'ws.close':
            final id = frame['id'];
            if (id is int) {
              unawaited(upstreams.remove(id)?.close());
            }
        }
      }
    } finally {
      for (final upstream in upstreams.values) {
        unawaited(upstream.close());
      }
      if (identical(_socket, socket)) _socket = null;
      await socket.close();
    }
  }

  Future<void> _http(WebSocket socket, Map<String, Object?> frame) async {
    final id = frame['id'];
    final gateway = localGateway();
    final rawPath = frame['path'];
    if (id is! int ||
        gateway == null ||
        rawPath is! String ||
        !rawPath.startsWith('/') ||
        rawPath.contains('..')) {
      _send(socket, {'type': 'error', 'id': id, 'code': 'UPSTREAM_UNAVAILABLE'});
      return;
    }
    final parsed = Uri.parse('http://127.0.0.1$rawPath');
    final target = gateway.replace(
      path: parsed.path,
      query: parsed.hasQuery ? parsed.query : null,
    );
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 20);
    _httpClients[id] = client;
    try {
      final method = frame['method'] is String
          ? frame['method']! as String
          : 'GET';
      final request = await client.openUrl(method, target);
      request.followRedirects = false;
      request.headers.set(
        'x-openmuse-paired-public-origin',
        publicOrigin.replace(path: '', query: null, fragment: null).toString(),
      );
      for (final header in _headerPairs(frame['headers'])) {
        final name = header.$1.toLowerCase();
        if (name == 'host' ||
            name == 'content-length' ||
            name == 'transfer-encoding' ||
            name == 'x-openmuse-paired-public-origin') {
          continue;
        }
        request.headers.add(header.$1, header.$2);
      }
      final body = frame['body'];
      if (body is String && body.isNotEmpty) {
        request.add(base64Decode(body));
      }
      final response = await request.close();
      final headers = <List<String>>[];
      response.headers.forEach((name, values) {
        final lower = name.toLowerCase();
        if (lower == 'content-length' ||
            lower == 'transfer-encoding' ||
            lower == 'content-encoding') {
          return;
        }
        for (final value in values) {
          headers.add([name, value]);
        }
      });
      final eventStream = headers.any(
        (header) =>
            header[0].toLowerCase() == 'content-type' &&
            header[1].toLowerCase().contains('text/event-stream'),
      );
      stderr.writeln(
        'OpenMuse relay: ${frame['method'] ?? 'GET'} $rawPath '
        'status=${response.statusCode} stream=$eventStream',
      );
      _send(socket, {
        'type': 'http.response.start',
        'id': id,
        'status': response.statusCode,
        'headers': headers,
      });
      var bytes = 0;
      var chunks = 0;
      await for (final chunk in response) {
        bytes += chunk.length;
        chunks += 1;
        if (eventStream || chunks == 1 || chunks % 20 == 0) {
          stderr.writeln(
            'OpenMuse relay: $rawPath chunk#$chunks bytes=${chunk.length} total=$bytes',
          );
        }
        var offset = 0;
        while (offset < chunk.length) {
          final end = min(offset + 32768, chunk.length);
          _send(socket, {
            'type': 'http.response.chunk',
            'id': id,
            'data': base64Encode(chunk.sublist(offset, end)),
          });
          offset = end;
        }
      }
      stderr.writeln('OpenMuse relay: $rawPath end chunks=$chunks bytes=$bytes');
      _send(socket, {'type': 'http.response.end', 'id': id});
    } catch (error) {
      stderr.writeln('OpenMuse desktop relay http: $error');
      _send(socket, {'type': 'error', 'id': id, 'code': 'UPSTREAM_FAILED'});
    } finally {
      _httpClients.remove(id);
      client.close(force: true);
    }
  }

  Future<void> _openWebSocket(
    WebSocket socket,
    Map<String, Object?> frame,
    Map<int, WebSocket> upstreams,
  ) async {
    final id = frame['id'];
    final gateway = localGateway();
    final rawPath = frame['path'];
    if (id is! int ||
        gateway == null ||
        rawPath is! String ||
        !rawPath.startsWith('/')) {
      _send(socket, {'type': 'error', 'id': id, 'code': 'UPSTREAM_UNAVAILABLE'});
      return;
    }
    final parsed = Uri.parse('http://127.0.0.1$rawPath');
    final target = gateway.replace(
      scheme: 'ws',
      path: parsed.path,
      query: parsed.hasQuery ? parsed.query : null,
    );
    try {
      final headers = <String, Object>{
        'x-openmuse-paired-public-origin': publicOrigin
            .replace(path: '', query: null, fragment: null)
            .toString(),
      };
      for (final header in _headerPairs(frame['headers'])) {
        headers[header.$1] = header.$2;
      }
      final upstream = await WebSocket.connect(
        target.toString(),
        headers: headers,
      );
      upstreams[id] = upstream;
      _send(socket, {'type': 'ws.ready', 'id': id});
      upstream.listen(
        (event) {
          if (event is String) {
            _send(socket, {
              'type': 'ws.data',
              'id': id,
              'encoding': 'text',
              'data': event,
            });
          } else if (event is List<int>) {
            _send(socket, {
              'type': 'ws.data',
              'id': id,
              'encoding': 'binary',
              'data': base64Encode(event),
            });
          }
        },
        onDone: () {
          upstreams.remove(id);
          _send(socket, {'type': 'ws.close', 'id': id});
        },
        onError: (_) {
          upstreams.remove(id);
          _send(socket, {'type': 'ws.close', 'id': id});
        },
      );
    } catch (error) {
      stderr.writeln('OpenMuse desktop relay websocket: $error');
      _send(socket, {'type': 'error', 'id': id, 'code': 'UPSTREAM_FAILED'});
    }
  }

  void _forwardWebSocketData(
    Map<int, WebSocket> upstreams,
    Map<String, Object?> frame,
  ) {
    final id = frame['id'];
    final upstream = id is int ? upstreams[id] : null;
    if (upstream == null) return;
    if (frame['encoding'] == 'binary' && frame['data'] is String) {
      upstream.add(base64Decode(frame['data']! as String));
      return;
    }
    if (frame['data'] is String) upstream.add(frame['data']! as String);
  }

  void _send(WebSocket socket, Map<String, Object?> frame) {
    if (socket.readyState != WebSocket.open) return;
    socket.add(jsonEncode(frame));
  }

  void _setPhase(DesktopRelayPhase value) {
    if (_phase == value) return;
    _phase = value;
    onChanged?.call();
  }

  Iterable<(String, String)> _headerPairs(Object? value) sync* {
    if (value is! List) return;
    for (final item in value) {
      if (item is List &&
          item.length == 2 &&
          item[0] is String &&
          item[1] is String) {
        yield (item[0] as String, item[1] as String);
      }
    }
  }
}

Uri? desktopRelayPublicOrigin({
  required Uri cloudOrigin,
  String configured = '',
}) {
  final raw = configured.trim();
  if (raw.isEmpty || cloudOrigin.host.isEmpty) return null;
  final uri = Uri.tryParse(raw);
  if (uri == null ||
      uri.scheme != 'https' ||
      uri.host.isEmpty ||
      uri.userInfo.isNotEmpty ||
      uri.hasQuery ||
      uri.hasFragment ||
      (uri.path.isNotEmpty && uri.path != '/')) {
    throw ArgumentError('relay public origin must be https');
  }
  return uri.replace(path: '', query: null, fragment: null);
}
