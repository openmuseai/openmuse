import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'package:mcp_dart/mcp_dart.dart';

/// Stateless standard MCP Streamable HTTP. Tokens are issued per terminal,
/// scoped to its account and explicit working directory, and never shared
/// with the Agent's resource router.
final class McpTerminalService {
  McpTerminalService({
    required this.scope,
    required this.tools,
    required this.call,
    this.onRevoke,
  });
  final String Function() scope;
  final Future<void> Function(String caller)? onRevoke;
  final List<Map<String, Object?>> tools;
  final Future<Map<String, Object?>> Function(
    String caller,
    String root,
    String name,
    Map<String, Object?> arguments,
  )
  call;
  final _sessions =
      <String, ({String account, String root, DateTime expires})>{};
  final _expiryTimers = <String, Timer>{};
  HttpServer? _server;
  Uri get endpoint =>
      Uri(scheme: 'http', host: '127.0.0.1', port: _server!.port, path: '/mcp');
  Future<void> start() async {
    _server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    _server!.listen((r) => unawaited(_handle(r)));
  }

  Map<String, String> issue(String root) {
    for (final token in _sessions.keys.toList()) {
      final session = _sessions[token]!;
      if (session.expires.isBefore(DateTime.now()) ||
          session.account != scope()) {
        unawaited(revoke(token));
      }
    }
    if (_sessions.length >= 32) {
      throw StateError('Terminal session budget exceeded');
    }
    final token = base64UrlEncode(
      List.generate(32, (_) => Random.secure().nextInt(256)),
    );
    _sessions[token] = (
      account: scope(),
      root: Directory(root).resolveSymbolicLinksSync(),
      expires: DateTime.now().add(const Duration(hours: 4)),
    );
    _expiryTimers[token] = Timer(
      const Duration(hours: 4),
      () => unawaited(revoke(token)),
    );
    return {'MCP_HOST_URL': endpoint.toString(), 'MCP_HOST_TOKEN': token};
  }

  Future<void> revoke(String token) async {
    _expiryTimers.remove(token)?.cancel();
    _sessions.remove(token);
    await onRevoke?.call(token);
  }

  Future<void> close() async {
    for (final token in _sessions.keys.toList()) {
      await revoke(token);
    }
    await _server?.close(force: true);
  }

  Future<void> _handle(HttpRequest request) async {
    final response = request.response;
    try {
      final token = request.headers
          .value(HttpHeaders.authorizationHeader)
          ?.replaceFirst('Bearer ', '');
      final session = _sessions[token];
      if (session == null ||
          session.account != scope() ||
          session.expires.isBefore(DateTime.now()) ||
          request.headers.value('origin') != null) {
        response.statusCode = 401;
        await response.close();
        return;
      }
      if (request.uri.path != '/mcp') {
        response.statusCode = 404;
        await response.close();
        return;
      }
      if (request.method == 'DELETE') {
        await revoke(token!);
        response.statusCode = 204;
        await response.close();
        return;
      }
      if (request.method != 'POST' || request.uri.path != '/mcp') {
        response.statusCode = 405;
        await response.close();
        return;
      }
      final bytes = <int>[];
      await for (final chunk in request.timeout(const Duration(seconds: 15))) {
        bytes.addAll(chunk);
        if (bytes.length > 48 * 1024 * 1024) {
          throw const FormatException('Request budget exceeded');
        }
      }
      final message = jsonDecode(utf8.decode(bytes)) as Map;
      if (message['jsonrpc'] != '2.0') {
        throw const FormatException('Invalid JSON-RPC');
      }
      if (message['id'] == null) {
        response.statusCode = 202;
        await response.close();
        return;
      }
      final reply = <String, Object?>{'jsonrpc': '2.0', 'id': message['id']};
      try {
        final params = Map<String, Object?>.from(
          message['params'] as Map? ?? {},
        );
        switch (message['method']) {
          case 'initialize':
            reply['result'] = {
              'protocolVersion': '2025-11-25',
              'serverInfo': {
                'name': 'OpenMuse public workspace capabilities',
                'version': '1.0.0',
              },
              'capabilities': {'tools': {}},
            };
          case 'ping':
            reply['result'] = {};
          case 'tools/list':
            reply['result'] = {'tools': tools};
          case 'tools/call':
            final name = params['name'] as String;
            if (!tools.any((t) => t['name'] == name)) {
              throw StateError('Tool not available');
            }
            if (session.account != scope() || !_sessions.containsKey(token)) {
              throw StateError('Scope revoked');
            }
            JsonSchema.fromJson(
              Map<String, dynamic>.from(
                tools.singleWhere((t) => t['name'] == name)['inputSchema']
                    as Map,
              ),
            ).validate(params['arguments'] ?? {});
            final output = await call(
              token!,
              session.root,
              name,
              Map<String, Object?>.from(params['arguments'] as Map? ?? {}),
            );
            if (session.account != scope() || !_sessions.containsKey(token)) {
              throw StateError('Scope revoked');
            }
            reply['result'] = {
              'content': [
                {'type': 'text', 'text': jsonEncode(output)},
              ],
              'structuredContent': output,
            };
          default:
            throw StateError('Method unavailable');
        }
      } catch (_) {
        reply.remove('result');
        reply['error'] = {
          'code': -32602,
          'message':
              'Request rejected; check tool arguments and workspace authorization.',
        };
      }
      response.headers.contentType = ContentType.json;
      response.write(jsonEncode(reply));
      await response.close();
    } catch (_) {
      try {
        response.statusCode = 400;
        await response.close();
      } catch (_) {}
    }
  }
}
