import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

typedef PairedTokenValidator = Future<String> Function(String accessToken);
typedef PairedDshEndpointProvider = Future<Uri> Function();
typedef RemoteSurfaceDispatcher =
    Future<Map<String, Object?>> Function(RemoteSurfaceDispatch request);
typedef RemoteMediaReader =
    Future<RemoteMediaSlice> Function(RemoteMediaQuery query);

/// A validated plugin interaction handed to the paired window. The gateway
/// only knows presentation data; plugin files and business logic stay outside.
final class PairedPluginInteraction {
  const PairedPluginInteraction({
    required this.id,
    required this.pluginId,
    required this.title,
    required this.imageBytes,
    required this.readStatus,
  });

  final String id;
  final String pluginId;
  final String title;
  final Uint8List imageBytes;
  final Future<Map<String, Object?>> Function() readStatus;
}

final class RemoteSurfaceDispatch {
  const RemoteSurfaceDispatch({
    required this.operation,
    required this.body,
    required this.accountRef,
    required this.deviceRef,
    required this.workspaceRef,
  });

  final String operation;
  final Map<String, Object?> body;
  final String accountRef;
  final String deviceRef;
  final String workspaceRef;
}

final class RemoteMediaQuery {
  const RemoteMediaQuery({
    required this.handle,
    required this.accountRef,
    required this.mobileDeviceRef,
    required this.desktopDeviceRef,
    required this.workspaceRef,
    required this.start,
    required this.endInclusive,
  });

  final String handle;
  final String accountRef;
  final String mobileDeviceRef;
  final String desktopDeviceRef;
  final String workspaceRef;
  final int start;
  final int? endInclusive;
}

sealed class RemoteMediaSlice {
  const RemoteMediaSlice();
}

final class RemoteMediaSliceDenied extends RemoteMediaSlice {
  const RemoteMediaSliceDenied();
}

final class RemoteMediaSliceUnsatisfiable extends RemoteMediaSlice {
  const RemoteMediaSliceUnsatisfiable();
}

final class RemoteMediaSliceBody extends RemoteMediaSlice {
  const RemoteMediaSliceBody({
    required this.bytes,
    required this.total,
    required this.start,
  });

  final Uint8List bytes;
  final int total;
  final int start;
}

final class PairedDesktopGateway {
  PairedDesktopGateway({
    required this.currentAccountRef,
    required this.validateToken,
    required this.dshEndpoint,
    required this.workspaceRef,
    required this.workspaceTitle,
    this.deviceRef = 'desktop.local',
    this.deviceName = 'OpenMuse Desktop',
    this.port = 13180,
    this.bindAddress,
    this.grantTtl = const Duration(minutes: 30),
    this.nativeApiToken,
    this.remoteSurface,
    this.remoteMedia,
    String? fixedPairingCode,
  }) : _fixedPairingCode = fixedPairingCode;

  final String? Function() currentAccountRef;
  final PairedTokenValidator validateToken;
  final PairedDshEndpointProvider dshEndpoint;
  final String workspaceRef;
  final String workspaceTitle;
  final String deviceRef;
  final String deviceName;
  final int port;
  final InternetAddress? bindAddress;
  final Duration grantTtl;

  /// Host-only token injected for the native conversation API. It is never
  /// returned to, or accepted from, the paired mobile client.
  final String? nativeApiToken;
  final RemoteSurfaceDispatcher? remoteSurface;
  final RemoteMediaReader? remoteMedia;
  final String? _fixedPairingCode;
  final Random _random = Random.secure();
  final Map<String, _DesktopGrant> _grants = {};
  final Map<String, PairedPluginInteraction> _interactions = {};
  String? _activeMobileGrantRef;
  DateTime? _activeMobilePromptAt;
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

  /// Route to the mobile window that most recently submitted a DSH prompt.
  /// A desktop-origin prompt has no grant, so its interaction stays local.
  bool offerPluginInteraction(PairedPluginInteraction interaction) {
    final grantRef = _activeMobileGrantRef;
    final promptAt = _activeMobilePromptAt;
    final grant = grantRef == null ? null : _grants[grantRef];
    if (grantRef == null ||
        grant == null ||
        promptAt == null ||
        DateTime.now().difference(promptAt) > const Duration(minutes: 10) ||
        grant.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      return false;
    }
    _interactions[grantRef] = interaction;
    _changed();
    return true;
  }

  Future<void> start() async {
    if (_server != null) return;
    try {
      final server = await HttpServer.bind(
        bindAddress ?? InternetAddress.loopbackIPv4,
        port,
      );
      _server = server;
      lastError = null;
      unawaited(_serve(server));
      _changed();
    } catch (error) {
      lastError = error;
      _changed();
      rethrow;
    }
  }

  /// In-process grant for the Desktop operator preview. This is not an HTTP
  /// route: a remote client still has to pair.
  String issueLoopbackOperatorGrant({
    required String accountRef,
    required String deviceRef,
  }) {
    final server = _server;
    if (server == null) {
      throw StateError('Paired Desktop gateway is not running.');
    }
    final grantRef = _randomHex(32);
    _grants[grantRef] = _DesktopGrant(
      grantRef: grantRef,
      accountRef: accountRef,
      deviceRef: deviceRef,
      workspaceRef: workspaceRef,
      expiresAtMs: DateTime.now().add(grantTtl).millisecondsSinceEpoch,
      upstream: Uri(
        scheme: 'http',
        host: server.address.host,
        port: server.port,
        path: '/',
      ),
    );
    _changed();
    return grantRef;
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
    _interactions.clear();
    _activeMobileGrantRef = null;
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
        await _open(request, requirePairingCode: true);
        return;
      }
      if (request.method == 'POST' && request.uri.path == '/v1/account/open') {
        await _open(request, requirePairingCode: false);
        return;
      }
      if (request.method == 'GET' && request.uri.path == '/v1/status') {
        _json(request.response, HttpStatus.ok, {
          'ready': running,
          'pairingArmed': _pairingCode != null,
        });
        return;
      }
      if (request.method == 'POST' &&
          request.uri.path == '/openmuse/remote-surface/v1') {
        await _remoteSurface(request);
        return;
      }
      if (request.method == 'GET' &&
          request.uri.path == '/openmuse/plugin-interaction/v1') {
        await _pluginInteraction(request);
        return;
      }
      if (request.method == 'GET' &&
          request.uri.path == '/openmuse/plugin-interaction/media/v1') {
        await _pluginInteractionMedia(request);
        return;
      }
      if (request.uri.path.startsWith('/openmuse/remote-surface/media/')) {
        await _remoteMedia(request);
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
      try {
        _json(request.response, HttpStatus.badGateway, {
          'code': 'PAIRED_DESKTOP_UNAVAILABLE',
          'message': 'Desktop transport 暂时不可用。',
        });
      } on StateError {
        // The DSH response was already streaming.
      }
    }
  }

  Future<void> _open(
    HttpRequest request, {
    required bool requirePairingCode,
  }) async {
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
    final requesterDeviceRef = body['deviceRef'];
    final targetDeviceRef = body['targetDeviceRef'];
    final requestedWorkspace = body['workspaceRef'];
    final now = DateTime.now().millisecondsSinceEpoch;
    if (requirePairingCode &&
        (code is! String ||
            code != _pairingCode ||
            _pairingExpiresAtMs == null ||
            now >= _pairingExpiresAtMs!)) {
      throw const _GatewayFailure(
        HttpStatus.forbidden,
        'PAIRING_CODE_DENIED',
        '配对码无效或已过期。',
      );
    }
    if (requesterDeviceRef is! String ||
        requesterDeviceRef.isEmpty ||
        requesterDeviceRef.length > 160 ||
        (targetDeviceRef != null && targetDeviceRef != deviceRef) ||
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
      deviceRef: requesterDeviceRef,
      workspaceRef: workspaceRef,
      expiresAtMs: expiresAtMs,
      upstream: upstream,
    );
    if (requirePairingCode) {
      _pairingCode = null;
      _pairingExpiresAtMs = null;
    }
    final advertised = _advertisedPublicOrigin(request);
    final publicOrigin =
        advertised ??
        Uri(
          scheme: 'http',
          host: _publicHost(request.headers.host),
          port: _server!.port,
        );
    _json(request.response, HttpStatus.ok, {
      'accountRef': desktopAccountRef,
      'deviceRef': deviceRef,
      'deviceName': deviceName,
      'workspaceRef': workspaceRef,
      'workspaceTitle': workspaceTitle,
      'grantRef': grantRef,
      'expiresAtMs': expiresAtMs,
      'session': {
        'sessionRef': 'paired-dsh:$grantRef',
        'origin': publicOrigin.toString(),
        'path': '/u/$grantRef',
        'generation': 1,
        'allowInsecureLoopback': advertised == null,
      },
    });
    _changed();
  }

  Future<void> _remoteSurface(HttpRequest request) async {
    final dispatcher = remoteSurface;
    if (dispatcher == null) {
      throw const _GatewayFailure(
        HttpStatus.serviceUnavailable,
        'SURFACE_UNAVAILABLE',
        '远程工作台未在这台 Desktop 上启用。',
      );
    }
    final grant = _grantFromCookie(request.cookies);
    if (grant == null ||
        grant.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw const _GatewayFailure(
        HttpStatus.unauthorized,
        'GRANT_REQUIRED',
        'Paired Desktop grant 无效或已过期。',
      );
    }
    final bytes = await request.fold<List<int>>(<int>[], (value, chunk) {
      if (value.length + chunk.length > 65536) {
        throw const _GatewayFailure(
          HttpStatus.requestEntityTooLarge,
          'REQUEST_TOO_LARGE',
          '远程工作台请求过大。',
        );
      }
      value.addAll(chunk);
      return value;
    });
    final decoded = jsonDecode(utf8.decode(bytes));
    const operations = {
      'discover',
      'open',
      'submit',
      'lookup',
      'snapshot',
      'events',
    };
    if (decoded is! Map ||
        decoded['operation'] is! String ||
        !operations.contains(decoded['operation']) ||
        decoded['body'] is! Map) {
      throw const _GatewayFailure(
        HttpStatus.badRequest,
        'SURFACE_REQUEST_INVALID',
        '远程工作台请求无效。',
      );
    }
    final result = await dispatcher(
      RemoteSurfaceDispatch(
        operation: decoded['operation'] as String,
        body: Map<String, Object?>.from(decoded['body'] as Map),
        accountRef: grant.accountRef,
        deviceRef: grant.deviceRef,
        workspaceRef: grant.workspaceRef,
      ),
    );
    _json(request.response, HttpStatus.ok, result);
  }

  _DesktopGrant _requireInteractionGrant(HttpRequest request) {
    final grant = _grantFromCookie(request.cookies);
    if (grant == null ||
        grant.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw const _GatewayFailure(
        HttpStatus.unauthorized,
        'GRANT_REQUIRED',
        'Paired Desktop grant 无效或已过期。',
      );
    }
    return grant;
  }

  Future<void> _pluginInteraction(HttpRequest request) async {
    final grant = _requireInteractionGrant(request);
    final interaction = _interactions[grant.grantRef];
    if (interaction == null) {
      _json(request.response, HttpStatus.ok, {'interaction': null});
      return;
    }
    final status = await interaction.readStatus();
    final state = status['state'];
    final safeState =
        state is String &&
            {
              'starting',
              'qr_ready',
              'success',
              'expired',
              'error',
            }.contains(state)
        ? state
        : 'error';
    final message = status['message'];
    _json(request.response, HttpStatus.ok, {
      'interaction': {
        'protocol': 'openmuse.plugin-interaction/v1',
        'type': 'image.challenge',
        'id': interaction.id,
        'pluginId': interaction.pluginId,
        'title': interaction.title,
        'state': safeState,
        'message': message is String
            ? message.substring(0, min(message.length, 200))
            : '',
        'mediaHandle': interaction.id,
      },
    });
  }

  Future<void> _pluginInteractionMedia(HttpRequest request) async {
    final grant = _requireInteractionGrant(request);
    final interaction = _interactions[grant.grantRef];
    if (interaction == null ||
        request.uri.queryParameters['handle'] != interaction.id) {
      throw const _GatewayFailure(
        HttpStatus.notFound,
        'MEDIA_NOT_FOUND',
        '交互图片不可用。',
      );
    }
    request.response.headers.contentType = ContentType('image', 'png');
    request.response.headers.set(HttpHeaders.cacheControlHeader, 'no-store');
    request.response.add(interaction.imageBytes);
    await request.response.close();
  }

  Future<void> _remoteMedia(HttpRequest request) async {
    final reader = remoteMedia;
    if (reader == null) {
      throw const _GatewayFailure(
        HttpStatus.serviceUnavailable,
        'MEDIA_UNAVAILABLE',
        '远程媒体未在这台 Desktop 上启用。',
      );
    }
    if (request.method != 'GET') {
      throw const _GatewayFailure(
        HttpStatus.methodNotAllowed,
        'MEDIA_REQUEST_INVALID',
        '远程媒体请求无效。',
      );
    }
    final segments = request.uri.pathSegments;
    final handle =
        segments.length == 5 &&
            segments[0] == 'openmuse' &&
            segments[1] == 'remote-surface' &&
            segments[2] == 'media' &&
            segments[3] == 'v1'
        ? segments[4]
        : null;
    if (handle == null ||
        !RegExp(r'^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$').hasMatch(handle)) {
      throw const _GatewayFailure(
        HttpStatus.badRequest,
        'MEDIA_REQUEST_INVALID',
        '远程媒体请求无效。',
      );
    }
    final grant = _grantFromCookie(request.cookies);
    if (grant == null ||
        grant.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw const _GatewayFailure(
        HttpStatus.unauthorized,
        'GRANT_REQUIRED',
        'Paired Desktop grant 无效或已过期。',
      );
    }
    final range = _byteRange(request.headers.value(HttpHeaders.rangeHeader));
    if (range == null) {
      throw const _GatewayFailure(
        HttpStatus.requestedRangeNotSatisfiable,
        'RANGE_NOT_SATISFIABLE',
        '远程媒体范围无效。',
      );
    }
    final slice = await reader(
      RemoteMediaQuery(
        handle: handle,
        accountRef: grant.accountRef,
        mobileDeviceRef: grant.deviceRef,
        desktopDeviceRef: deviceRef,
        workspaceRef: grant.workspaceRef,
        start: range.start,
        endInclusive: range.endInclusive,
      ),
    );
    if (slice is RemoteMediaSliceDenied) {
      throw const _GatewayFailure(
        HttpStatus.notFound,
        'MEDIA_UNAVAILABLE',
        '远程媒体不可用。',
      );
    }
    if (slice is! RemoteMediaSliceBody) {
      throw const _GatewayFailure(
        HttpStatus.requestedRangeNotSatisfiable,
        'RANGE_NOT_SATISFIABLE',
        '远程媒体范围无效。',
      );
    }
    final end = slice.start + slice.bytes.length - 1;
    final response = request.response
      ..statusCode = range.explicit ? HttpStatus.partialContent : HttpStatus.ok
      ..headers.set(HttpHeaders.acceptRangesHeader, 'bytes')
      ..headers.contentType = ContentType.binary
      ..contentLength = slice.bytes.length;
    if (range.explicit && slice.bytes.isNotEmpty) {
      response.headers.set(
        HttpHeaders.contentRangeHeader,
        'bytes ${slice.start}-$end/${slice.total}',
      );
    }
    response.add(slice.bytes);
    await response.close();
  }

  ({int start, int? endInclusive, bool explicit})? _byteRange(String? header) {
    if (header == null || header.isEmpty) {
      return (start: 0, endInclusive: null, explicit: false);
    }
    final match = RegExp(r'^bytes=(\d+)-(\d*)$').firstMatch(header.trim());
    if (match == null) return null;
    final start = int.parse(match.group(1)!);
    final endText = match.group(2)!;
    final end = endText.isEmpty ? null : int.parse(endText);
    if (end != null && end < start) return null;
    return (start: start, endInclusive: end, explicit: true);
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
      // Android WebView may keep an empty GET request stream open while the
      // response is pending. Waiting for that stream before close() deadlocks
      // the bootstrap navigation through adb reverse.
      if (request.method == 'POST' &&
          request.uri.path == '/openmuse-native/v1/session/prompt') {
        final body = await request.fold<List<int>>(<int>[], (bytes, chunk) {
          if (bytes.length + chunk.length > 65536) {
            throw const _GatewayFailure(
              HttpStatus.requestEntityTooLarge,
              'REQUEST_TOO_LARGE',
              '对话请求过大。',
            );
          }
          return bytes..addAll(chunk);
        });
        outbound.add(body);
        _activeMobileGrantRef = grant.grantRef;
        _activeMobilePromptAt = DateTime.now();
        _interactions.remove(grant.grantRef);
      } else if (request.method != 'GET' && request.method != 'HEAD') {
        await outbound.addStream(request);
      }
      final upstreamResponse = await outbound.close();
      request.response.statusCode = upstreamResponse.statusCode;
      // Dart otherwise holds the first 8KB until the upstream ends or the
      // buffer fills. A DSH follow snapshot is often smaller than that and
      // the stream never ends, so Mobile stays on "正在连接 DSH".
      request.response.bufferOutput = false;
      _copyResponseHeaders(
        upstreamResponse,
        request.response,
        upstreamUri: upstreamUri,
      );
      if (initial != null) {
        final advertised = _advertisedPublicOrigin(request);
        request.response.cookies.add(
          Cookie('OpenMuse-Paired', grant.grantRef)
            ..httpOnly = true
            ..secure = advertised != null
            ..sameSite = SameSite.strict
            ..path = '/'
            ..maxAge = grantTtl.inSeconds,
        );
      }
      final label = _proxyLabel(request);
      final eventStream =
          upstreamResponse.headers
              .value(HttpHeaders.contentTypeHeader)
              ?.contains('text/event-stream') ??
          false;
      stderr.writeln(
        'OpenMuse gateway: $label status=${upstreamResponse.statusCode} '
        'type=${upstreamResponse.headers.value(HttpHeaders.contentTypeHeader)} '
        'stream=$eventStream',
      );
      await request.response.addStream(
        _tapProxyBody(
          label: label,
          eventStream: eventStream,
          source: upstreamResponse,
        ),
      );
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
          lower == 'x-openmuse-bridge-token' ||
          lower == 'origin' ||
          lower == HttpHeaders.refererHeader ||
          lower == 'x-openmuse-paired-public-origin') {
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
    final nativeToken = nativeApiToken;
    if (source.uri.path.startsWith('/openmuse-native/') &&
        nativeToken != null &&
        nativeToken.length >= 32) {
      target.headers.set('x-openmuse-bridge-token', nativeToken);
    }
  }

  String _proxyLabel(HttpRequest request) {
    final sessionId = request.uri.queryParameters['sessionId'];
    final query = sessionId == null ? '' : '?sessionId=$sessionId';
    return '${request.method} ${request.uri.path}$query';
  }

  Stream<List<int>> _tapProxyBody({
    required String label,
    required bool eventStream,
    required Stream<List<int>> source,
  }) async* {
    var bytes = 0;
    var frames = 0;
    var carry = '';
    final jsonBody = StringBuffer();
    final captureJson =
        !eventStream &&
        (label.contains('/sessions') ||
            label.contains('/workspaces') ||
            label.contains('/prompt') ||
            label.contains('/session/create'));
    await for (final chunk in source) {
      bytes += chunk.length;
      if (eventStream) {
        carry += utf8.decode(chunk, allowMalformed: true);
        while (true) {
          final end = carry.indexOf('\n\n');
          if (end < 0) break;
          final block = carry.substring(0, end);
          carry = carry.substring(end + 2);
          frames += 1;
          stderr.writeln(
            'OpenMuse gateway: $label frame#$frames ${_frameSummary(block)}',
          );
        }
      } else if (captureJson && jsonBody.length < 65536) {
        jsonBody.write(utf8.decode(chunk, allowMalformed: true));
      }
      yield chunk;
    }
    if (eventStream) {
      stderr.writeln(
        'OpenMuse gateway: $label end bytes=$bytes frames=$frames tail=${carry.length}',
      );
    } else if (captureJson) {
      stderr.writeln(
        'OpenMuse gateway: $label end bytes=$bytes ${_jsonSummary(label, jsonBody.toString())}',
      );
    } else {
      stderr.writeln('OpenMuse gateway: $label end bytes=$bytes');
    }
  }

  String _frameSummary(String block) {
    final dataIndex = block.indexOf('data:');
    if (dataIndex < 0) return 'nodata bytes=${block.length}';
    final data = block.substring(dataIndex + 5).trim();
    try {
      final decoded = jsonDecode(data);
      if (decoded is! Map) return 'non-object';
      final type = decoded['type'];
      if (type == 'snapshot') {
        final records = decoded['records'];
        final count = records is List ? records.length : -1;
        var last = '-';
        if (records is List && records.isNotEmpty && records.last is Map) {
          final event = (records.last as Map)['event'];
          if (event is Map) last = '${event['type']}@${event['seq']}';
        }
        return 'snapshot cursor=${decoded['cursor']} records=$count last=$last';
      }
      if (type == 'event' && decoded['event'] is Map) {
        final event = decoded['event'] as Map;
        final source = event['source'];
        final rpc = source is Map && source['rpcId'] is String;
        return 'event ${event['type']}@${event['seq']} rpc=$rpc';
      }
      if (type == 'assistant-stream' && decoded['frame'] is Map) {
        return 'assistant-stream ${(decoded['frame'] as Map)['type']}';
      }
      return 'type=$type';
    } catch (error) {
      return 'parse-failed bytes=${data.length}';
    }
  }

  String _jsonSummary(String label, String body) {
    try {
      final decoded = jsonDecode(body);
      if (decoded is! Map) return 'non-object';
      if (label.contains('/prompt') || label.contains('/session/create')) {
        return 'accepted=${decoded['accepted']} session=${decoded['sessionId']}';
      }
      final items = decoded['items'];
      if (items is! List) return 'keys=${decoded.keys.join(",")}';
      if (label.contains('/sessions')) {
        final lines = <String>[];
        for (final item in items) {
          if (item is! Map) continue;
          final values = item['projections'] is Map
              ? (item['projections'] as Map)['values']
              : null;
          final title = values is Map ? values['title'] : null;
          lines.add(
            '${item['sessionId']} running=${item['running']} '
            'blank=${item['blank']} updated=${item['updatedAt']} '
            'title=${title is String ? title : ''}',
          );
        }
        return 'sessions=${items.length} ${lines.join(' | ')}';
      }
      return 'workspaces=${items.length}';
    } catch (error) {
      return 'parse-failed bytes=${body.length}';
    }
  }

  void _copyResponseHeaders(
    HttpClientResponse source,
    HttpResponse target, {
    required Uri upstreamUri,
  }) {
    source.headers.forEach((name, values) {
      final lower = name.toLowerCase();
      if (lower == HttpHeaders.contentLengthHeader ||
          // Dart HttpClient transparently decodes gzip by default. Forwarding
          // the upstream encoding after that would make WebView try to decode
          // the already-decoded bytes a second time and render a blank page.
          lower == HttpHeaders.contentEncodingHeader ||
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

  Uri? _advertisedPublicOrigin(HttpRequest request) {
    final remote = request.connectionInfo?.remoteAddress;
    if (remote == null || !remote.isLoopback) return null;
    final raw = request.headers.value('x-openmuse-paired-public-origin');
    if (raw == null || raw.isEmpty) return null;
    final uri = Uri.tryParse(raw);
    if (uri == null ||
        uri.scheme != 'https' ||
        uri.host.isEmpty ||
        uri.userInfo.isNotEmpty ||
        uri.hasQuery ||
        uri.hasFragment ||
        (uri.path.isNotEmpty && uri.path != '/')) {
      return null;
    }
    return uri.replace(path: '', query: null, fragment: null);
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
