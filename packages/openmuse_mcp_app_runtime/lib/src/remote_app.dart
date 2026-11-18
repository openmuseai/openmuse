import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:mcp_dart/mcp_dart.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'remote_app_view.dart';

/// Explicit developer installation, separate from the production marketplace.
/// Only verified, self-contained UI bytes enter the sandbox. Credentials stay
/// in the Host secure store; the backend owns project authorization.
final class RemoteMcpApp extends ChangeNotifier
    implements
        OpenMusePlugin,
        OpenMuseWorkspaceActionsContributor,
        OpenMuseMobileWorkbenchContributor,
        OpenMuseMobileNativeDetailContributor {
  RemoteMcpApp._(
    this.definition,
    this.directory,
    this.html,
    this.scope,
    this.readSecret,
    this.writeSecret,
  );
  final Map<String, dynamic> definition;
  final Directory directory;
  final String html;
  final String Function() scope;
  final Future<String?> Function(String) readSecret;
  final Future<void> Function(String, String) writeSecret;
  McpClient? _client;
  int _epoch = 0;
  Future<McpClient>? _connecting;
  String? _connectedScope;
  List<Tool> _tools = [];
  VoidCallback? onOpen;
  /// Host auditing hook. No arguments, results or credentials are included.
  void Function(String method, String? tool, bool agent)? onRequestCompleted;
  String get id => definition['id'] as String;
  String get title => definition['title'] as String;
  String get panelId => '$id.main';
  String get uiUri => definition['uiResourceUri'] as String;
  String get _owner => sha256.convert(utf8.encode(scope())).toString();

  static Future<List<RemoteMcpApp>> load({
    required Directory directory,
    required String Function() scope,
    required Future<String?> Function(String) readSecret,
    required Future<void> Function(String, String) writeSecret,
  }) async {
    final manifest = File('${directory.path}/apps.json');
    if (!await manifest.exists()) return [];
    final entries = jsonDecode(await manifest.readAsString()) as List;
    final result = <RemoteMcpApp>[];
    for (final raw in entries) {
      final d = Map<String, dynamic>.from(raw as Map);
      if (!RegExp(r'^[a-zA-Z0-9._-]+$').hasMatch(d['id'] as String) ||
          d['uiResourceUri'] is! String ||
          !('${d['uiResourceUri']}').startsWith('ui://')) {
        throw const FormatException('Invalid remote App identity');
      }
      final endpoint = Uri.parse(d['endpoint'] as String);
      if (endpoint.userInfo.isNotEmpty ||
          endpoint.hasQuery ||
          endpoint.hasFragment ||
          !(endpoint.scheme == 'https' ||
              (endpoint.scheme == 'http' &&
                  const [
                    'localhost',
                    '127.0.0.1',
                    '::1',
                  ].contains(endpoint.host)))) {
        throw const FormatException(
          'Remote App requires HTTPS or development loopback',
        );
      }
      final file = d['uiFile'] as String;
      if (!RegExp(r'^[a-zA-Z0-9._-]+\.html$').hasMatch(file)) {
        throw const FormatException('Invalid reviewed UI file');
      }
      final bytes = await File('${directory.path}/$file').readAsBytes();
      if (bytes.length > 8 * 1024 * 1024 ||
          sha256.convert(bytes).toString() != d['uiSha256']) {
        throw StateError('APP_UI_DIGEST_MISMATCH');
      }
      result.add(
        RemoteMcpApp._(
          d,
          directory,
          utf8.decode(bytes),
          scope,
          readSecret,
          writeSecret,
        ),
      );
    }
    if (result.map((a) => a.id).toSet().length != result.length) {
      throw const FormatException('Duplicate remote App identity');
    }
    return result;
  }

  void _check(String owner) {
    if (_owner != owner ||
        (definition['owner'] != null && definition['owner'] != owner)) {
      throw StateError('APP_ACCOUNT_SCOPE_REVOKED');
    }
  }

  Future<McpClient> _connect() async {
    final owner = _owner;
    _check(owner);
    if (_client != null && _connectedScope == owner) return _client!;
    if (_connecting != null) return _connecting!;
    final connecting = _open(owner, _epoch);
    _connecting = connecting;
    try {
      return await connecting;
    } finally {
      _connecting = null;
    }
  }

  Future<McpClient> _open(String owner, int epoch) async {
    _check(owner);
    final key = 'openmuse.remote-app.$id.$owner';
    var token = await readSecret(key);
    final bootstrap = File('${directory.path}/$id.token');
    if (token == null && await bootstrap.exists()) {
      final candidate = (await bootstrap.readAsString()).trim();
      _check(owner);
      if (candidate.isEmpty || candidate.contains('\n'))
        throw StateError('APP_CREDENTIAL_INVALID');
      await writeSecret(key, candidate);
      _check(owner);
      // Verify durable secure storage before consuming the one-time import.
      token = await readSecret(key);
      if (token != candidate) throw StateError('APP_CREDENTIAL_STORAGE_FAILED');
      definition['owner'] = owner;
      final profiles = File('${directory.path}/apps.json');
      final list = jsonDecode(await profiles.readAsString()) as List;
      for (final d in list) {
        if (d['id'] == id) d['owner'] = owner;
      }
      await profiles.writeAsString(jsonEncode(list), flush: true);
      await bootstrap.delete();
    }
    if (token == null) throw StateError('APP_CREDENTIAL_REQUIRED');
    _check(owner);
    final client = McpClient(
      const Implementation(name: 'OpenMuse remote App Host', version: '0.1.0'),
      options: const McpClientOptions(protocol: McpProtocol.require2026),
    );
    try {
      await client
          .connect(
            StreamableHttpClientTransport(
              Uri.parse(definition['endpoint'] as String),
              opts: StreamableHttpClientTransportOptions(
                requestInit: {
                  'headers': {'Authorization': 'Bearer $token'},
                },
              ),
            ),
          )
          .timeout(const Duration(seconds: 20));
      _check(owner);
      final tools = (await client.listTools()).tools;
      if (!tools.any(
        (t) =>
            (t.toJson()['_meta'] as Map?)?['ui'] is Map &&
            ((t.toJson()['_meta'] as Map)['ui'] as Map)['resourceUri'] == uiUri,
      )) {
        throw StateError('APP_ENTRY_NOT_DISCOVERED');
      }
      final resource = (await client.readResource(
        ReadResourceRequest(uri: uiUri),
      )).toJson();
      final contents = resource['contents'] as List;
      if (contents.length != 1 ||
          contents.single['uri'] != uiUri ||
          contents.single['mimeType'] != 'text/html;profile=mcp-app' ||
          contents.single['text'] is! String ||
          sha256
                  .convert(utf8.encode(contents.single['text'] as String))
                  .toString() !=
              definition['uiSha256']) {
        throw StateError('APP_REMOTE_UI_DIGEST_MISMATCH');
      }
      _check(owner);
      if (epoch != _epoch) throw StateError('APP_SESSION_REVOKED');
      _tools = tools;
      _connectedScope = owner;
      _client = client;
      return client;
    } catch (_) {
      await client.close();
      rethrow;
    }
  }

  bool _visible(Tool tool, bool agent) {
    final ui = (tool.toJson()['_meta'] as Map?)?['ui'];
    final visibility = ui is Map ? ui['visibility'] : null;
    return visibility == null ||
        (visibility is List && visibility.contains(agent ? 'model' : 'app'));
  }

  Future<Map<String, Object?>> request(
    String method,
    Map<String, Object?> params, {
    bool agent = false,
  }) async {
    final owner = _owner, epoch = _epoch;
    _check(owner);
    final client = await _connect();
    _check(owner);
    Object? result;
    switch (method) {
      case 'tools/list':
        result = {
          'tools': _tools
              .where((t) => _visible(t, agent))
              .map((t) => t.toJson())
              .toList(),
        };
      case 'tools/call':
        final name = params['name'];
        final tool = _tools.singleWhere(
          (t) => t.name == name,
          orElse: () => throw StateError('APP_TOOL_UNDECLARED'),
        );
        if (!_visible(tool, agent)) throw StateError('APP_TOOL_NOT_VISIBLE');
        final arguments = Map<String, dynamic>.from(
          params['arguments'] as Map? ?? {},
        );
        tool.inputSchema.validate(arguments);
        result =
            (await client
                    .callTool(
                      CallToolRequest(
                        name: name as String,
                        arguments: arguments,
                      ),
                    )
                    .timeout(const Duration(seconds: 90)))
                .toJson();
      case 'resources/list':
        result = (await client.listResources()).toJson();
      case 'resources/read':
        if (params['uri'] != uiUri) throw StateError('APP_RESOURCE_UNDECLARED');
        // Use the reviewed local UI, never replace it with remote executable bytes.
        result = {
          'contents': [
            {
              'uri': uiUri,
              'mimeType': 'text/html;profile=mcp-app',
              'text': html,
            },
          ],
        };
      default:
        throw UnsupportedError('APP_METHOD_UNAVAILABLE');
    }
    _check(owner);
    if (epoch != _epoch) throw StateError('APP_SESSION_REVOKED');
    if (utf8.encode(jsonEncode(result)).length > 8 * 1024 * 1024)
      throw StateError('APP_RESULT_TOO_LARGE');
    onRequestCompleted?.call(method, params['name'] as String?, agent);
    return Map<String, Object?>.from(result as Map);
  }

  Future<void> closeSession() async {
    _epoch++;
    notifyListeners();
    final client = _client;
    _client = null;
    _tools = [];
    _connectedScope = null;
    await client?.close();
  }

  @override
  OpenMusePluginDescriptor get descriptor => OpenMusePluginDescriptor(
    id: id,
    name: title,
    version: definition['version'] as String? ?? '0.1.0',
    runtime: OpenMusePluginRuntime.webView,
    activationEvents: const ['onStartup'],
    panels: [
      OpenMusePanelContribution(
        id: panelId,
        region: OpenMuseSurfaceRegion.rightSidebar,
      ),
    ],
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() => closeSession();
  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      RemoteMcpAppView(app: this);
  @override
  Widget buildPanel(BuildContext context, String panelId) =>
      RemoteMcpAppView(app: this);
  @override
  Widget buildMobileDetail(BuildContext context) => RemoteMcpAppView(app: this);
  @override
  List<Widget> buildWorkspaceActions(BuildContext context) => [
    TextButton.icon(
      icon: const Icon(Icons.account_tree_outlined, size: 18),
      label: Text(title),
      onPressed: () {
        if (onOpen != null) {
          onOpen!();
          return;
        }
        Navigator.of(context).push(
          MaterialPageRoute<void>(
            builder: (_) => Scaffold(
              appBar: AppBar(title: Text(title)),
              body: RemoteMcpAppView(app: this),
            ),
          ),
        );
      },
    ),
  ];
  @override
  OpenMuseWorkbenchCard get workbenchCard =>
      OpenMuseWorkbenchCard(title: title, summary: '已安装 · 点击打开');
  @override
  Listenable get workbenchChanges => this;
  @override
  List<Map<String, Object?>> buildWorkbenchPreview() => [
    {
      'version': 'v0.9',
      'createSurface': {
        'surfaceId': 'main',
        'catalogId':
            'https://a2ui.org/specification/v0_9/catalogs/basic/catalog.json',
      },
    },
    {
      'version': 'v0.9',
      'updateComponents': {
        'surfaceId': 'main',
        'components': [
          {'id': 'root', 'component': 'Text', 'text': '$title · 点击打开插件工作台'},
        ],
      },
    },
  ];
  @override
  String buildMobileDetailDocument(Brightness brightness) => html;
  @override
  Future<Object?> handleMobileDetailRequest(String method, Object? arguments) =>
      request(method, Map<String, Object?>.from(arguments as Map? ?? {}));
}

/// The Agent sees only installed Apps, their discovered schemas and results.
/// Business tools, projects and release state remain owned by each plugin.
final class RemoteMcpAppsRouter {
  RemoteMcpAppsRouter(this.apps);
  final List<RemoteMcpApp> apps;
  Future<Map<String, Object?>> call(
    String name,
    Map<String, Object?> args,
  ) async {
    if (name == 'apps_list')
      return _result({
        'apps': [
          for (final a in apps)
            {'id': a.id, 'title': a.title, 'uiResourceUri': a.uiUri},
        ],
      });
    final app = apps.singleWhere((a) => a.id == args['appId']);
    if (name == 'apps_open') {
      app.onOpen?.call();
      return _result({'opened': app.id});
    }
    if (name == 'apps_tools_list')
      return _result(await app.request('tools/list', {}, agent: true));
    if (name == 'apps_call_tool')
      return app.request('tools/call', {
        'name': args['name'],
        'arguments': args['arguments'] ?? {},
      }, agent: true);
    throw UnsupportedError('APP_ROUTER_METHOD_UNAVAILABLE');
  }

  Map<String, Object?> _result(Map<String, Object?> data) => {
    'content': [
      {'type': 'text', 'text': jsonEncode(data)},
    ],
    'structuredContent': data,
  };
}
