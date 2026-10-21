import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mcp_dart/mcp_dart.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;

import 'mcp_market.dart';
import 'mcp_capability_gateway.dart';

/// Distribution services shared by ordinary MCP Apps, with no business IDs.
final class McpAppsEnvironment {
  McpAppsEnvironment({
    required this.market,
    required this.nodeExecutable,
    required this.scope,
  });
  final McpRegistryMarket market;
  final String nodeExecutable;
  final String Function() scope;
  final _configuration = <String, Map<String, Object?>>{};
  late final Future<Map<String, Object?>> identity = _identity();
  Future<Map<String, Object?>> _identity() async {
    final exe = File(Platform.resolvedExecutable);
    final appBinary = File(
      p.normalize(
        p.join(exe.parent.path, '../Frameworks/App.framework/Versions/A/App'),
      ),
    );
    final digest = sha256.convert([
      ...await exe.readAsBytes(),
      if (await appBinary.exists()) ...await appBinary.readAsBytes(),
    ]).toString();
    return {
      'implementation': {
        'name': 'OpenMuse',
        'version': '1.0.0',
        'buildSha256': digest,
      },
      'platform': Platform.operatingSystem,
      'profiles': [
        'openmuse-capabilities/1',
        'mcpb/0.3',
        'mcp-apps/2026-01-26',
      ],
      'development': {
        'runtime': 'node',
        'candidateIsolation': 'separate-view-and-process',
      },
    };
  }

  Future<List<CapabilityProvider>> providers() async {
    final result = <CapabilityProvider>[];
    for (final entry in await market.installedEntries()) {
      final runtime = McpBundleRuntime(
        bundleDirectory: market.directoryFor(entry),
        nodeExecutable: nodeExecutable,
      );
      McpBundleSession? session;
      try {
        session = await runtime.connect();
        final resources = await session.client.listResources();
        final catalog = resources.resources
            .where(
              (r) => r.mimeType == 'application/vnd.openmuse.capabilities+json',
            )
            .firstOrNull;
        if (catalog == null) {
          await session.close();
          continue;
        }
        final read = await session.client.readResource(
          ReadResourceRequest(uri: catalog.uri),
        );
        final data =
            jsonDecode(
                  read.contents.whereType<TextResourceContents>().single.text,
                )
                as Map;
        if (data['profile'] != 'openmuse-capabilities/1') {
          throw const FormatException('Unsupported capability profile');
        }
        final connected = session;
        result.add(
          CapabilityProvider(
            id: entry.name,
            version: entry.version,
            digest: entry.sha256,
            contracts: (data['contracts'] as List)
                .map((c) => Map<String, Object?>.from(c as Map))
                .toList(),
            tools: (await session.client.listTools()).tools
                .map((t) => t.toJson())
                .toList(),
            invoke: (name, args) async => (await connected.client.callTool(
              CallToolRequest(name: name, arguments: args),
            )).toJson(),
            isCurrent: () async => (await market.installedEntries()).any(
              (e) => e.name == entry.name && e.sha256 == entry.sha256,
            ),
            close: connected.close,
          ),
        );
      } catch (_) {
        await session?.close();
      }
    }
    return result;
  }
}

/// Standard MCP Apps View with one persistent originating MCP server.
/// Capability routing and development installation are format-independent.
final class McpInstalledApp extends StatefulWidget {
  const McpInstalledApp({
    super.key,
    required this.pluginId,
    required this.environment,
    this.developmentEntry,
    this.developmentDirectory,
  });
  final String pluginId;
  final McpAppsEnvironment environment;
  final McpMarketEntry? developmentEntry;
  final Directory? developmentDirectory;
  @override
  State<McpInstalledApp> createState() => _McpInstalledAppState();
}

final class _McpInstalledAppState extends State<McpInstalledApp> {
  McpBundleSession? _session;
  MethodChannel? _channel;
  McpCapabilityGateway? _gateway;
  List<Tool> _tools = [];
  late final String _scope = widget.environment.scope();
  bool get _valid => mounted && widget.environment.scope() == _scope;
  String? _html, _failure;
  final _temporary = <Directory>[];
  final _runs = <String, Directory>{};

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => unawaited(_load()));
  }

  Future<bool> _approve(String title, String details) async {
    if (!_valid) return false;
    return await showDialog<bool>(
              context: context,
              builder: (dialog) => AlertDialog(
                title: Text(title),
                content: SelectableText(details),
                actions: [
                  TextButton(
                    onPressed: () => Navigator.pop(dialog, false),
                    child: const Text('取消'),
                  ),
                  FilledButton(
                    onPressed: () => Navigator.pop(dialog, true),
                    child: const Text('允许'),
                  ),
                ],
              ),
            ) ==
            true &&
        _valid;
  }

  Future<GrantedResource?> _choose(String title, String mimeType) async {
    if (!mounted || !_valid) throw StateError('CALLER_SCOPE_REVOKED');
    final chosen = await FilePicker.platform.pickFiles(dialogTitle: title);
    if (chosen == null) return null;
    final file = File(chosen.files.single.path!);
    if (await file.length() > 64 * 1024 * 1024) {
      throw StateError('RESOURCE_TOO_LARGE');
    }
    return GrantedResource(
      title: p.basename(file.path),
      mediaType: mimeType,
      bytes: await file.readAsBytes(),
    );
  }

  Future<Map<String, Object?>> _configure(
    Directory directory,
    String identity,
  ) async {
    final manifest =
        jsonDecode(
              await File(
                p.join(directory.path, 'manifest.json'),
              ).readAsString(),
            )
            as Map;
    final schema = manifest['user_config'];
    if (schema is! Map || schema.isEmpty) return {};
    final key = '${_scope}_$identity';
    final existing = widget.environment._configuration[key];
    if (existing != null) return existing;
    final controllers = <String, TextEditingController>{};
    for (final item in schema.entries) {
      final value = item.value as Map;
      final fallback = '${value['default'] ?? ''}'.replaceAll(
        r'${HOME}',
        Platform.environment['HOME'] ?? '',
      );
      controllers[item.key as String] = TextEditingController(text: fallback);
    }
    try {
      if (!mounted || !_valid) throw StateError('CALLER_SCOPE_REVOKED');
      final accepted = await showDialog<bool>(
        context: context,
        barrierDismissible: false,
        builder: (dialog) => AlertDialog(
          title: const Text('配置已安装插件'),
          content: SizedBox(
            width: 580,
            child: SingleChildScrollView(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final item in schema.entries)
                    Padding(
                      padding: const EdgeInsets.only(bottom: 16),
                      child: TextField(
                        controller: controllers[item.key],
                        obscureText: (item.value as Map)['sensitive'] == true,
                        decoration: InputDecoration(
                          labelText:
                              '${(item.value as Map)['title'] ?? item.key}',
                          helperText:
                              '${(item.value as Map)['description'] ?? ''}',
                          helperMaxLines: 3,
                        ),
                      ),
                    ),
                  const Text('敏感配置仅在此应用进程内保存，关闭应用后需重新输入。'),
                ],
              ),
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(dialog, false),
              child: const Text('取消'),
            ),
            FilledButton(
              onPressed: () => Navigator.pop(dialog, true),
              child: const Text('启动插件'),
            ),
          ],
        ),
      );
      if (accepted != true || !_valid) throw StateError('USER_CANCELLED');
      final values = <String, Object?>{};
      for (final item in schema.entries) {
        final value = controllers[item.key]!.text;
        if ((item.value as Map)['required'] == true && value.trim().isEmpty) {
          throw StateError('Required configuration missing');
        }
        values[item.key as String] = value;
      }
      widget.environment._configuration[key] = values;
      return values;
    } finally {
      for (final c in controllers.values) {
        c.dispose();
      }
    }
  }

  Future<void> _load() async {
    try {
      final market = widget.environment.market;
      final entry =
          widget.developmentEntry ??
          (await market.installedEntries()).singleWhere(
            (e) => e.name == widget.pluginId,
          );
      final directory =
          widget.developmentDirectory ?? market.directoryFor(entry);
      final config = await _configure(
        directory,
        '${entry.name}@${entry.sha256}',
      );
      final session = await McpBundleRuntime(
        bundleDirectory: directory,
        nodeExecutable: widget.environment.nodeExecutable,
      ).connect(configuration: config);
      if (!_valid) {
        await session.close();
        return;
      }
      _session = session;
      _tools = (await session.client.listTools()).tools.where((t) {
        final visibility = (t.meta?['ui'] as Map?)?['visibility'];
        return visibility is! List || visibility.contains('app');
      }).toList();
      final launch = _tools.where((tool) {
        final ui = tool.meta?['ui'];
        return ui is Map &&
            ui['resourceUri'] is String &&
            ((tool.inputSchema.toJson()['required'] as List?) ?? []).isEmpty;
      }).firstOrNull;
      if (launch == null) throw StateError('插件未声明无参数 MCP Apps 入口，请从工作区选择资源打开');
      await session.client.callTool(
        CallToolRequest(name: launch.name, arguments: const {}),
      );
      final resource = await session.client.readResource(
        ReadResourceRequest(
          uri: (launch.meta!['ui'] as Map)['resourceUri'] as String,
        ),
      );
      final html = resource.contents
          .whereType<TextResourceContents>()
          .where((c) => c.mimeType == 'text/html;profile=mcp-app')
          .single
          .text;
      _gateway = McpCapabilityGateway(
        describeHost: () => widget.environment.identity,
        listProviders: widget.environment.providers,
        chooseResource: _choose,
        approve: _approve,
        scope: widget.environment.scope,
        openDevelopment: _openDevelopment,
        readDevelopment: _readDevelopment,
      );
      if (_valid) {
        setState(
          () => _html = buildMcpAppHostDocument(
            appHtml: html,
            arguments: const {},
            platform: 'desktop',
            serverAvailable: true,
          ),
        );
      }
    } catch (_) {
      await _session?.close();
      if (mounted) setState(() => _failure = '插件启动失败。请核对安装、必填配置与 MCP Apps 入口。');
    }
  }

  Future<Map<String, Object?>> _openDevelopment(
    Map<String, Object?> args,
  ) async {
    final bytes = base64Decode(args['content'] as String);
    if (bytes.length > 32 * 1024 * 1024 ||
        sha256.convert(bytes).toString() != args['sha256']) {
      throw StateError('CANDIDATE_DIGEST_MISMATCH');
    }
    final archive = ZipDecoder().decodeBytes(bytes);
    final manifest =
        jsonDecode(
              utf8.decode(
                archive.findFile('manifest.json')!.content as List<int>,
              ),
            )
            as Map;
    if (!await _approve(
      '运行开发候选包',
      '${manifest['display_name'] ?? manifest['name']} ${manifest['version']}\nSHA-256: ${args['sha256']}\n未审核的本地代码。新进程和独立资源授权；不会替换正式安装。',
    )) {
      throw StateError('PERMISSION_DENIED');
    }
    final root = await Directory.systemTemp.createTemp('openmuse-development-');
    _temporary.add(root);
    final market = McpRegistryMarket(
      origin: Uri.parse('http://127.0.0.1'),
      installRoot: root,
      nodeExecutable: widget.environment.nodeExecutable,
    );
    final entry = McpMarketEntry(
      name: 'development/${manifest['name']}',
      title: '${manifest['display_name'] ?? manifest['name']}',
      description: 'Development candidate',
      version: manifest['version'] as String,
      packageUri: Uri.parse('http://127.0.0.1/candidate.mcpb'),
      sha256: args['sha256'] as String,
    );
    await market.install(entry, bundleBytes: bytes);
    final runRef = sha256.convert(utf8.encode(root.path)).toString();
    _runs[runRef] = market.directoryFor(entry);
    if (!mounted || !_valid) throw StateError('CALLER_SCOPE_REVOKED');
    unawaited(
      Navigator.of(context).push<void>(
        MaterialPageRoute(
          builder: (_) => Scaffold(
            appBar: AppBar(title: Text('开发预览 · ${entry.title}')),
            body: McpInstalledApp(
              pluginId: entry.name,
              environment: widget.environment,
              developmentEntry: entry,
              developmentDirectory: market.directoryFor(entry),
            ),
          ),
        ),
      ),
    );
    return {
      'opened': true,
      'sha256': entry.sha256,
      'channel': 'development',
      'runRef': runRef,
    };
  }

  Future<Map<String, Object?>> _readDevelopment(
    Map<String, Object?> args,
  ) async {
    final directory = _runs[args['runRef']];
    if (directory == null || !_valid) {
      throw StateError('DEVELOPMENT_RUN_NOT_GRANTED');
    }
    final session = await McpBundleRuntime(
      bundleDirectory: directory,
      nodeExecutable: widget.environment.nodeExecutable,
    ).connect();
    try {
      final response = (await session.client.readResource(
        ReadResourceRequest(uri: args['uri'] as String),
      )).toJson();
      if (!_valid || jsonEncode(response).length > 1024 * 1024) {
        throw StateError('RUN_RESOURCE_REJECTED');
      }
      return response;
    } finally {
      await session.close();
    }
  }

  void _created(int id) {
    final channel = MethodChannel('com.openmuse.mcp/app-view/$id');
    _channel = channel;
    channel.setMethodCallHandler((call) async {
      if (call.method != 'message' || call.arguments is! Map) return;
      final request = Map<String, Object?>.from(call.arguments as Map);
      if (request['id'] == null) return;
      final response = <String, Object?>{'jsonrpc': '2.0', 'id': request['id']};
      try {
        if (!_valid || _session == null) {
          throw StateError('CALLER_SCOPE_REVOKED');
        }
        final params = Map<String, Object?>.from(
          request['params'] as Map? ?? {},
        );
        final client = _session!.client;
        switch (request['method']) {
          case 'tools/list':
            response['result'] = {
              'tools': [
                ..._tools.map((t) => t.toJson()),
                ...McpCapabilityGateway.tools,
              ],
            };
          case 'tools/call':
            final name = params['name'] as String;
            final args = Map<String, Object?>.from(
              params['arguments'] as Map? ?? {},
            );
            if (_gateway!.handles(name)) {
              final output = await _gateway!.call(name, args);
              response['result'] = {
                'content': [
                  {'type': 'text', 'text': jsonEncode(output)},
                ],
                'structuredContent': output,
              };
            } else {
              final definition = _tools.singleWhere((t) => t.name == name);
              _validator(definition.inputSchema)(args);
              response['result'] =
                  (await client
                          .callTool(
                            CallToolRequest(name: name, arguments: args),
                          )
                          .timeout(const Duration(seconds: 90)))
                      .toJson();
            }
          case 'resources/list':
            response['result'] = (await client.listResources()).toJson();
          case 'resources/read':
            response['result'] = (await client.readResource(
              ReadResourceRequest(uri: params['uri'] as String),
            )).toJson();
          default:
            throw StateError('METHOD_UNAVAILABLE');
        }
        if (!mounted || !_valid) throw StateError('CALLER_SCOPE_REVOKED');
      } catch (_) {
        response.remove('result');
        response['error'] = {
          'code': -32602,
          'message':
              'Request denied or failed; check permissions, inputs and provider availability.',
        };
      }
      if (mounted) await channel.invokeMethod<void>('message', response);
    });
  }

  @override
  void dispose() {
    _channel?.setMethodCallHandler(null);
    unawaited(_gateway?.close() ?? Future.value());
    unawaited(_session?.close() ?? Future.value());
    for (final root in _temporary) {
      unawaited(root.delete(recursive: true).catchError((_) => root));
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_failure != null) return Center(child: SelectableText(_failure!));
    if (_html == null) return const Center(child: CircularProgressIndicator());
    return AppKitView(
      viewType: 'com.openmuse.mcp/app-view',
      creationParams: {'html': _html},
      creationParamsCodec: const StandardMessageCodec(),
      onPlatformViewCreated: _created,
    );
  }
}

void Function(dynamic) _validator(JsonSchema schema) => schema.validate;
