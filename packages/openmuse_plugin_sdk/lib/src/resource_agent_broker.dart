import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'package:flutter/foundation.dart';

/// Host-owned MCP router. Document tool schemas and results are opaque here.
/// Only mounted resource views can register; no agent-supplied path is opened.
final class ResourceAgentBroker extends ChangeNotifier {
  final Map<String, ResourceAppSession> _sessions = {};
  final String token = base64UrlEncode(
    List.generate(32, (_) => Random.secure().nextInt(256)),
  );
  HttpServer? _server;
  String? _activeUri;
  String? get activeUri => _activeUri;
  set activeUri(String? value) {
    if (value == _activeUri) return;
    _activeUri = value;
    notifyListeners();
  }

  Future<void> Function(List<Map<String, Object?>> content)? sendAgentMessage;

  Uri get endpoint =>
      Uri(scheme: 'http', host: '127.0.0.1', port: _server!.port, path: '/mcp');

  Future<void> start() async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    _server = server;
    server.listen((request) => unawaited(_handle(request)));
  }

  void register(ResourceAppSession session) => _sessions[session.uri] = session;
  void unregister(ResourceAppSession session) {
    if (identical(_sessions[session.uri], session)) {
      _sessions.remove(session.uri);
    }
  }

  void clear() {
    _sessions.clear();
    activeUri = null;
  }

  Future<void> close() async {
    clear();
    await _server?.close(force: true);
  }

  ResourceAppSession _authorized(Object? uri) {
    final session = _sessions[uri];
    if (session == null || uri != activeUri) {
      throw StateError('Resource is closed or is not the active document');
    }
    return session;
  }

  Future<Map<String, Object?>> dispatch(
    String method,
    Map<String, Object?> params,
  ) async {
    switch (method) {
      case 'initialize':
        return {
          'protocolVersion': '2025-11-25',
          'serverInfo': {
            'name': 'OpenMuse resource router',
            'version': '1.0.0',
          },
          'capabilities': {'tools': {}, 'resources': {}},
        };
      case 'ping':
        return {};
      case 'tools/list':
        return {'tools': resourceRouterTools};
      case 'resources/list':
        return {
          'resources': [
            for (final s in _sessions.values)
              if (s.uri == activeUri)
                {'uri': s.uri, 'name': s.title, 'mimeType': 'application/json'},
          ],
        };
      case 'resources/read':
        final s = _authorized(params['uri']);
        return {
          'contents': [
            {
              'uri': s.uri,
              'mimeType': 'application/json',
              'text': jsonEncode(s.context),
            },
          ],
        };
      case 'tools/call':
        final args = Map<String, Object?>.from(
          params['arguments'] as Map? ?? {},
        );
        Object? result;
        switch (params['name']) {
          case 'list_open_resources':
            result = [
              for (final s in _sessions.values)
                if (s.uri == activeUri)
                  {'uri': s.uri, 'name': s.title, 'context': s.context},
            ];
          case 'list_resource_tools':
            result = await _authorized(args['uri']).request('tools/list', {});
          case 'call_resource_tool':
            final s = _authorized(args['uri']);
            final listed = await s.request('tools/list', {});
            final tools = listed['tools'] as List? ?? [];
            if (!tools.any(
              (tool) => tool is Map && tool['name'] == args['name'],
            )) {
              throw StateError('Tool is not contributed by the resource');
            }
            if (!identical(_authorized(args['uri']), s)) {
              throw StateError('Resource session changed');
            }
            return s.request('tools/call', {
              'name': args['name'],
              'arguments': args['arguments'] ?? {},
            });
          default:
            throw StateError('Unknown resource router tool');
        }
        return {
          'content': [
            {'type': 'text', 'text': jsonEncode(result)},
          ],
        };
      default:
        throw UnsupportedError('Method unavailable');
    }
  }

  Future<void> _handle(HttpRequest request) async {
    final response = request.response;
    try {
      if (request.uri.path != '/mcp' ||
          request.headers.value('authorization') != 'Bearer $token' ||
          request.headers.value('origin') != null) {
        response.statusCode = HttpStatus.unauthorized;
      } else if (request.method != 'POST') {
        response.statusCode = HttpStatus.methodNotAllowed;
      } else {
        final bytes = await request
            .fold<List<int>>([], (a, b) {
              if (a.length + b.length > 1024 * 1024) {
                throw const FormatException('Request exceeds budget');
              }
              return a..addAll(b);
            })
            .timeout(const Duration(seconds: 10));
        final message = jsonDecode(utf8.decode(bytes)) as Map;
        if (message['jsonrpc'] != '2.0' || message['method'] is! String) {
          throw const FormatException('Invalid JSON-RPC');
        }
        if (!message.containsKey('id')) {
          response.statusCode = HttpStatus.accepted;
        } else {
          Map<String, Object?> reply;
          try {
            reply = {
              'jsonrpc': '2.0',
              'id': message['id'],
              'result': await dispatch(
                message['method'] as String,
                Map<String, Object?>.from(message['params'] as Map? ?? {}),
              ),
            };
          } catch (_) {
            reply = {
              'jsonrpc': '2.0',
              'id': message['id'],
              'error': {
                'code': -32602,
                'message':
                    'Resource request rejected; check active document and tool arguments',
              },
            };
          }
          response.headers.contentType = ContentType.json;
          response.headers.set('Cache-Control', 'no-store');
          response.write(jsonEncode(reply));
        }
      }
    } catch (_) {
      response.statusCode = HttpStatus.badRequest;
    }
    await response.close();
  }
}

final class ResourceAppSession {
  ResourceAppSession({
    required this.uri,
    required this.title,
    required this.request,
  });
  final String uri;
  final String title;
  final Future<Map<String, Object?>> Function(
    String method,
    Map<String, Object?> params,
  )
  request;
  Map<String, Object?> context = {};
}

const resourceRouterTools = [
  {
    'name': 'list_open_resources',
    'description':
        'List the active user-opened document. Use its resource tools to inspect and edit live unsaved content; do not edit its disk file behind the editor.',
    'inputSchema': {
      'type': 'object',
      'properties': {},
      'additionalProperties': false,
    },
    'annotations': {'readOnlyHint': true},
  },
  {
    'name': 'list_resource_tools',
    'description':
        'Discover tools and JSON Schemas contributed by an active resource editor. Business operations belong to that plugin.',
    'inputSchema': {
      'type': 'object',
      'properties': {
        'uri': {'type': 'string'},
      },
      'required': ['uri'],
      'additionalProperties': false,
    },
    'annotations': {'readOnlyHint': true},
  },
  {
    'name': 'call_resource_tool',
    'description':
        'Call a discovered live document tool. Follow its schema and revision checks. Save through the editor tool after a successful edit. Never guess plugin operations.',
    'inputSchema': {
      'type': 'object',
      'properties': {
        'uri': {'type': 'string'},
        'name': {'type': 'string'},
        'arguments': {'type': 'object'},
      },
      'required': ['uri', 'name', 'arguments'],
      'additionalProperties': false,
    },
  },
];
