import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

typedef PairedTokenValidator = Future<String> Function(String accessToken);
typedef PairedDshEndpointProvider = Future<Uri> Function();

final class PairedDesktopGateway {
  PairedDesktopGateway({
    required this.currentAccountRef,
    required this.validateToken,
    required this.dshEndpoint,
    required this.workspaceRef,
    required this.workspaceTitle,
    this.port = 13180,
    this.bindAddress,
    this.grantTtl = const Duration(minutes: 30),
    String? fixedPairingCode,
  }) : _fixedPairingCode = fixedPairingCode;

  final String? Function() currentAccountRef;
  final PairedTokenValidator validateToken;
  final PairedDshEndpointProvider dshEndpoint;
  final String workspaceRef;
  final String workspaceTitle;
  final int port;
  final InternetAddress? bindAddress;
  final Duration grantTtl;
  final String? _fixedPairingCode;
  final Random _random = Random.secure();
  final Map<String, _DesktopGrant> _grants = {};
  HttpServer? _server;
  String? _pairingCode;
  int? _pairingExpiresAtMs;
  Object? lastError;
  void Function()? onChanged;

  bool get running => _server != null;
  Uri? get origin => _server == null
      ? null
      : Uri(scheme: 'http', host: '127.0.0.1', port: _server!.port);
  String? get pairingCode => _pairingCode;
  int? get pairingExpiresAtMs => _pairingExpiresAtMs;

  Future<void> start() async {
    if (_server != null) return;
    try {
      final server = await HttpServer.bind(
        bindAddress ?? InternetAddress.loopbackIPv4,
        port,
      );
      _server = server;
      armPairing();
      unawaited(_serve(server));
      _changed();
    } catch (error) {
      lastError = error;
      _changed();
      rethrow;
    }
  }

  void armPairing() {
    _pairingCode =
        _fixedPairingCode ??
        List<int>.generate(6, (_) => _random.nextInt(10)).join();
    _pairingExpiresAtMs = DateTime.now()
        .add(const Duration(minutes: 15))
        .millisecondsSinceEpoch;
    _changed();
  }

  Future<void> stop() async {
    final server = _server;
    _server = null;
    _grants.clear();
    _pairingCode = null;
    _pairingExpiresAtMs = null;
    await server?.close(force: true);
    _changed();
  }

  Future<void> _serve(HttpServer server) async {
    await for (final request in server) {
      unawaited(_handle(request));
    }
  }

  Future<void> _handle(HttpRequest request) async {
    try {
      if (request.method == 'POST' && request.uri.path == '/v1/pair/open') {
        await _pair(request);
        return;
      }
      if (request.method == 'GET' && request.uri.path == '/v1/status') {
        _json(request.response, HttpStatus.ok, {
          'ready': running,
          'pairingArmed': _pairingCode != null,
        });
        return;
      }
      await _proxy(request);
    } on _GatewayFailure catch (error) {
      _json(request.response, error.statusCode, {
        'code': error.code,
        'message': error.safeMessage,
      });
    } catch (error) {
      lastError = error;
      _changed();
      _json(request.response, HttpStatus.badGateway, {
        'code': 'PAIRED_DESKTOP_UNAVAILABLE',
        'message': 'Desktop transport 暂时不可用。',
      });
    }
  }

  Future<void> _pair(HttpRequest request) async {
    final authorization = request.headers.value(
      HttpHeaders.authorizationHeader,
    );
    if (authorization == null || !authorization.startsWith('Bearer ')) {
      throw const _GatewayFailure(
        HttpStatus.unauthorized,
        'UNAUTHENTICATED',
        '请先登录。',
      );
    }
    final bytes = await request.fold<List<int>>(<int>[], (value, chunk) {
      if (value.length + chunk.length > 16 * 1024) {
        throw const _GatewayFailure(
          HttpStatus.requestEntityTooLarge,
          'REQUEST_TOO_LARGE',
          '配对请求过大。',
        );
      }
      value.addAll(chunk);
      return value;
    });
    final decoded = jsonDecode(utf8.decode(bytes));
    if (decoded is! Map) {
      throw const _GatewayFailure(
        HttpStatus.badRequest,
        'INVALID_REQUEST',
        '配对请求无效。',
      );
    }
    final body = decoded.cast<String, Object?>();
    final code = body['pairingCode'];
    final deviceRef = body['deviceRef'];
    final requestedWorkspace = body['workspaceRef'];
    final now = DateTime.now().millisecondsSinceEpoch;
    if (code is! String ||
        code != _pairingCode ||
        _pairingExpiresAtMs == null ||
        now >= _pairingExpiresAtMs!) {
      throw const _GatewayFailure(
        HttpStatus.forbidden,
        'PAIRING_CODE_DENIED',
        '配对码无效或已过期。',
      );
    }
    if (deviceRef is! String ||
        deviceRef.isEmpty ||
        deviceRef.length > 160 ||
        requestedWorkspace != workspaceRef) {
      throw const _GatewayFailure(
        HttpStatus.forbidden,
        'WORKSPACE_GRANT_DENIED',
        'Desktop Workspace 未授权。',
      );
    }
    final desktopAccountRef = currentAccountRef();
    if (desktopAccountRef == null || desktopAccountRef.isEmpty) {
      throw const _GatewayFailure(
        HttpStatus.locked,
        'DESKTOP_SIGNED_OUT',
        'Desktop 尚未登录。',
      );
    }
    final mobileAccountRef = await validateToken(authorization.substring(7));
    if (mobileAccountRef != desktopAccountRef) {
      throw const _GatewayFailure(
        HttpStatus.forbidden,
        'ACCOUNT_MISMATCH',
        'Mobile 与 Desktop 必须登录同一个账号。',
      );
    }
    final upstream = await dshEndpoint();
    if (!_isLoopbackHttp(upstream)) {
      throw const _GatewayFailure(
        HttpStatus.badGateway,
        'INVALID_DSH_ENDPOINT',
        'Desktop DSH endpoint 无效。',
      );
    }
    final grantRef = _randomHex(32);
    final expiresAtMs = now + grantTtl.inMilliseconds;
    _grants[grantRef] = _DesktopGrant(
      grantRef: grantRef,
      accountRef: desktopAccountRef,
      deviceRef: deviceRef,
      workspaceRef: workspaceRef,
      expiresAtMs: expiresAtMs,
      upstream: upstream,
    );
    _pairingCode = null;
    _pairingExpiresAtMs = null;
    final publicOrigin = Uri(
      scheme: 'http',
      host: _publicHost(request.headers.host),
      port: _server!.port,
    );
    _json(request.response, HttpStatus.ok, {
      'accountRef': desktopAccountRef,
      'deviceRef': deviceRef,
      'workspaceRef': workspaceRef,
      'workspaceTitle': workspaceTitle,
      'grantRef': grantRef,
      'expiresAtMs': expiresAtMs,
      'session': {
        'sessionRef': 'paired-dsh:$grantRef',
        'origin': publicOrigin.toString(),
        'path': '/u/$grantRef',
        'generation': 1,
        'allowInsecureLoopback': true,
      },
    });
    _changed();
  }

  Future<void> _proxy(HttpRequest request) async {
    final initial = _grantFromInitialPath(request.uri.path);
    final grant = initial ?? _grantFromCookie(request.cookies);
    if (grant == null ||
        grant.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw const _GatewayFailure(
        HttpStatus.unauthorized,
        'GRANT_REQUIRED',
        'Paired Desktop grant 无效或已过期。',
      );
    }
    final upstreamUri = initial == null
        ? grant.upstream.replace(
            path: request.uri.path,
            query: request.uri.query,
          )
        : grant.upstream;
    if (WebSocketTransformer.isUpgradeRequest(request)) {
      await _proxyWebSocket(request, grant, upstreamUri);
      return;
    }
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 10);
    try {
      final outbound = await client.openUrl(request.method, upstreamUri);
      // DSH's bootstrap URL answers with a one-time 303 + auth cookie. The
      // browser, not this stateless proxy client, must follow that redirect so
      // the cookie is installed on the paired origin.
      outbound.followRedirects = false;
      _copyRequestHeaders(request, outbound, upstreamUri: upstreamUri);
      await outbound.addStream(request);
      final upstreamResponse = await outbound.close();
      request.response.statusCode = upstreamResponse.statusCode;
      _copyResponseHeaders(
        upstreamResponse,
        request.response,
        upstreamUri: upstreamUri,
      );
      if (initial != null) {
        request.response.cookies.add(
          Cookie('OpenMuse-Paired', grant.grantRef)
            ..httpOnly = true
            ..sameSite = SameSite.strict
            ..path = '/'
            ..maxAge = grantTtl.inSeconds,
        );
      }
      await request.response.addStream(upstreamResponse);
      await request.response.close();
    } finally {
      client.close(force: true);
    }
  }

  Future<void> _proxyWebSocket(
    HttpRequest request,
    _DesktopGrant grant,
    Uri upstreamUri,
  ) async {
    final incoming = await WebSocketTransformer.upgrade(request);
    final outgoing = await WebSocket.connect(
      upstreamUri.replace(scheme: 'ws').toString(),
      headers: {
        'origin': grant.upstream.origin,
        if (_upstreamCookie(request.cookies).isNotEmpty)
          HttpHeaders.cookieHeader: _upstreamCookie(request.cookies),
      },
    );
    var closed = false;
    Future<void> closeBoth() async {
      if (closed) return;
      closed = true;
      await Future.wait([incoming.close(), outgoing.close()]);
    }

    incoming.listen(
      outgoing.add,
      onDone: closeBoth,
      onError: (_) => closeBoth(),
    );
    outgoing.listen(
      incoming.add,
      onDone: closeBoth,
      onError: (_) => closeBoth(),
    );
  }

  _DesktopGrant? _grantFromInitialPath(String path) {
    final segments = Uri(path: path).pathSegments;
    if (segments.length != 2 || segments.first != 'u') return null;
    return _grants[segments.last];
  }

  _DesktopGrant? _grantFromCookie(List<Cookie> cookies) {
    for (final cookie in cookies) {
      if (cookie.name == 'OpenMuse-Paired') return _grants[cookie.value];
    }
    return null;
  }

  String _upstreamCookie(List<Cookie> cookies) => cookies
      .where((cookie) => cookie.name != 'OpenMuse-Paired')
      .map((cookie) => '${cookie.name}=${cookie.value}')
      .join('; ');

  void _copyRequestHeaders(
    HttpRequest source,
    HttpClientRequest target, {
    required Uri upstreamUri,
  }) {
    source.headers.forEach((name, values) {
      final lower = name.toLowerCase();
      if (lower == HttpHeaders.hostHeader ||
          lower == HttpHeaders.contentLengthHeader ||
          lower == HttpHeaders.connectionHeader ||
          lower == HttpHeaders.authorizationHeader ||
          lower == HttpHeaders.cookieHeader ||
          lower == 'origin' ||
          lower == HttpHeaders.refererHeader) {
        return;
      }
      target.headers.set(name, values);
    });
    if (source.headers.value('origin') != null) {
      target.headers.set('origin', upstreamUri.origin);
    }
    if (source.headers.value(HttpHeaders.refererHeader) != null) {
      target.headers.set(
        HttpHeaders.refererHeader,
        Uri(
          scheme: upstreamUri.scheme,
          host: upstreamUri.host,
          port: upstreamUri.port,
          path: '/',
        ).toString(),
      );
    }
    final cookie = _upstreamCookie(source.cookies);
    if (cookie.isNotEmpty) target.headers.set(HttpHeaders.cookieHeader, cookie);
  }

  void _copyResponseHeaders(
    HttpClientResponse source,
    HttpResponse target, {
    required Uri upstreamUri,
  }) {
    source.headers.forEach((name, values) {
      final lower = name.toLowerCase();
      if (lower == HttpHeaders.contentLengthHeader ||
          lower == HttpHeaders.transferEncodingHeader ||
          lower == HttpHeaders.connectionHeader ||
          lower == HttpHeaders.locationHeader) {
        return;
      }
      target.headers.set(name, values);
    });
    final location = source.headers.value(HttpHeaders.locationHeader);
    if (location != null) {
      final resolved = upstreamUri.resolve(location);
      final sameUpstream =
          resolved.scheme == upstreamUri.scheme &&
          resolved.host == upstreamUri.host &&
          resolved.port == upstreamUri.port;
      if (!sameUpstream) {
        throw const _GatewayFailure(
          HttpStatus.badGateway,
          'CROSS_ORIGIN_REDIRECT',
          'Desktop DSH 返回了不安全的跳转。',
        );
      }
      target.headers.set(
        HttpHeaders.locationHeader,
        resolved.hasQuery
            ? '${resolved.path}?${resolved.query}'
            : resolved.path,
      );
    }
  }

  String _randomHex(int bytes) => List<int>.generate(
    bytes,
    (_) => _random.nextInt(256),
  ).map((value) => value.toRadixString(16).padLeft(2, '0')).join();

  void _json(HttpResponse response, int status, Map<String, Object?> value) {
    response
      ..statusCode = status
      ..headers.contentType = ContentType.json
      ..write(jsonEncode(value))
      ..close();
  }

  bool _isLoopbackHttp(Uri uri) =>
      uri.scheme == 'http' &&
      (uri.host == '127.0.0.1' || uri.host == 'localhost' || uri.host == '::1');

  String _publicHost(String? hostHeader) {
    final bound = _server?.address;
    if (bound == null || bound.isLoopback) return '127.0.0.1';
    try {
      final candidate = Uri.parse('http://$hostHeader').host;
      if (_isPrivateIpv4(candidate)) return candidate;
    } on FormatException {
      // An untrusted Host header never widens the paired transport origin.
    }
    return '127.0.0.1';
  }

  bool _isPrivateIpv4(String host) {
    final parts = host.split('.').map(int.tryParse).toList(growable: false);
    if (parts.length != 4 || parts.any((value) => value == null)) return false;
    final first = parts[0]!;
    final second = parts[1]!;
    return first == 10 ||
        (first == 192 && second == 168) ||
        (first == 172 && second >= 16 && second <= 31);
  }

  void _changed() => onChanged?.call();

  void dispose() {
    unawaited(stop());
  }
}

final class _DesktopGrant {
  const _DesktopGrant({
    required this.grantRef,
    required this.accountRef,
    required this.deviceRef,
    required this.workspaceRef,
    required this.expiresAtMs,
    required this.upstream,
  });
  final String grantRef;
  final String accountRef;
  final String deviceRef;
  final String workspaceRef;
  final int expiresAtMs;
  final Uri upstream;
}

final class _GatewayFailure implements Exception {
  const _GatewayFailure(this.statusCode, this.code, this.safeMessage);
  final int statusCode;
  final String code;
  final String safeMessage;
}
