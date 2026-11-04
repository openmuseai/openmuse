import 'package:openmuse_i18n/openmuse_i18n.dart';
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
import 'mcp_workspace_tools.dart';

/// Distribution services shared by ordinary MCP Apps, with no business IDs.
final class McpAppsEnvironment {
  McpAppsEnvironment({
    required this.market,
    required this.nodeExecutable,
    required this.scope,
    this.navigatorKey,
    this.workspaceDirectory,
    this.selectedMarket,
  });
  final McpRegistryMarket market;
  final String nodeExecutable;
  final String Function() scope;
  final GlobalKey<NavigatorState>? navigatorKey;
  final String Function()? workspaceDirectory;
  final McpRegistryMarket? Function()? selectedMarket;
  Future<void> Function(McpMarketEntry)? onInstalled;
  Future<bool> Function(
    String caller,
    McpMarketEntry entry,
    Directory directory,
    Future<bool> Function() valid,
  )?
  onDevelopmentBundle;
  void Function(String caller)? onDevelopmentClosed;
  final _developmentRuns = <String, Map<String, Directory>>{};
  final _developmentReceipts = <String, Map<String, Map<String, Object?>>>{};
  final _developmentAuthorizations = <String, ValueNotifier<bool>>{};
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
        'openmuse-resource-extensions/1',
        'mcpb/0.3',
        'mcp-apps/2026-01-26',
      ],
      'development': {
        'runtime': 'node',
        'candidateIsolation': 'separate-view-and-process',
      },
    };
  }

  static final workspaceTools = mcpWorkspaceTools;
  Future<bool> approve(String title, String details) async {
    final context = navigatorKey?.currentContext;
    if (context == null || !context.mounted) return false;
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
        true;
  }

  Future<Map<String, Object?>> workspaceCall(
    String name,
    Map<String, Object?> args,
    String root,
  ) async {
    if (name == 'workspace_describe') {
      return {
        'roots': [
          {'uri': Directory(root).uri.toString(), 'name': p.basename(root)},
        ],
      };
    }
    final local = await market.installedEntries();
    final provider = selectedMarket?.call();
    var onlineAvailable = provider != null;
    List<McpMarketEntry> online = [];
    try {
      online = await provider?.list() ?? [];
    } on Object {
      if (name == 'registry_install') rethrow;
      onlineAvailable = false;
    }
    if (name == 'registry_install') {
      final entry = online.singleWhere((e) => e.name == args['name']);
      if (!await approve(
        '安装插件',
        '${entry.title} ${entry.version}\n${entry.packageUri.host}\nSHA-256: ${entry.sha256}',
      )) {
        throw StateError('PERMISSION_DENIED');
      }
      final result = await provider!.install(entry);
      await onInstalled?.call(entry);
      return result;
    }
    if (name != 'registry_search') throw StateError('METHOD_UNAVAILABLE');
    final query = (args['query'] as String).toLowerCase(),
        merged = {
          for (final e in local) e.name: e,
          for (final e in online) e.name: e,
        };
    return {
      'onlineAvailable': onlineAvailable,
      'plugins': [
        for (final entry in merged.values)
          if ('${entry.name} ${entry.title} ${entry.description}'
              .toLowerCase()
              .contains(query))
            {
              'name': entry.name,
              'title': entry.title,
              'description': entry.description,
              'version': entry.version,
              'sha256': entry.sha256,
              'installed': local.any(
                (e) => e.name == entry.name && e.sha256 == entry.sha256,
              ),
              'listed': onlineAvailable
                  ? online.any(
                      (e) => e.name == entry.name && e.sha256 == entry.sha256,
                    )
                  : null,
            },
      ],
    };
  }

  Future<Map<String, Object?>> openDevelopment(
    String caller,
    Map<String, Object?> args,
  ) async {
    if (const bool.fromEnvironment('OPENMUSE_MAC_APP_STORE')) {
      throw UnsupportedError('Development code execution is unavailable in the App Store distribution');
    }
    final ownerScope = scope(), bytes = base64Decode(args['content'] as String);
    if (bytes.length > 32 * 1024 * 1024 ||
        sha256.convert(bytes).toString() != args['sha256']) {
      throw StateError('CANDIDATE_DIGEST_MISMATCH');
    }
    final owned = _developmentRuns.putIfAbsent(caller, () => {});
    final authorization = _developmentAuthorizations.putIfAbsent(
      caller,
      () => ValueNotifier(true),
    );
    final receipts = _developmentReceipts.putIfAbsent(caller, () => {});
    final existing = receipts.values
        .where(
          (r) =>
              r['sha256'] == args['sha256'] && owned.containsKey(r['runRef']),
        )
        .firstOrNull;
    if (existing != null) return Map<String, Object?>.from(existing);
    if (owned.length >= 4 &&
        !receipts.values.any(
          (r) => r['channel'] == 'resource-extension-development',
        )) {
      throw StateError('DEVELOPMENT_RUN_BUDGET_EXCEEDED');
    }
    final archive = ZipDecoder().decodeBytes(bytes);
    final manifest =
        jsonDecode(
              utf8.decode(
                archive.findFile('manifest.json')!.content as List<int>,
              ),
            )
            as Map;
    if (!await approve(
      '运行开发候选包',
      '${manifest['display_name'] ?? manifest['name']} ${manifest['version']}\nSHA-256: ${args['sha256']}\n未审核的本地代码。独立 View 和进程，不替换正式安装。',
    )) {
      throw StateError('PERMISSION_DENIED');
    }
    if (scope() != ownerScope || !authorization.value) {
      throw StateError('CALLER_SCOPE_REVOKED');
    }
    final root = await Directory.systemTemp.createTemp('openmuse-development-');
    final staging = McpRegistryMarket(
      origin: Uri.parse('http://127.0.0.1'),
      installRoot: root,
      nodeExecutable: nodeExecutable,
    );
    final entry = McpMarketEntry(
      name: 'development/${manifest['name']}',
      title: '${manifest['display_name'] ?? manifest['name']}',
      description: 'Development candidate',
      version: manifest['version'] as String,
      packageUri: Uri.parse('http://127.0.0.1/candidate.mcpb'),
      sha256: args['sha256'] as String,
    );
    try {
      await staging.install(entry, bundleBytes: bytes);
    } catch (_) {
      await root.delete(recursive: true);
      rethrow;
    }
    final runRef = sha256.convert(utf8.encode(root.path)).toString();
    owned[runRef] = staging.directoryFor(entry);
    if (await onDevelopmentBundle?.call(
          caller,
          entry,
          staging.directoryFor(entry),
          () async =>
              scope() == ownerScope &&
              authorization.value &&
              owned[runRef] != null,
        ) ==
        true) {
      for (final old
          in receipts.entries
              .where(
                (e) => e.value['channel'] == 'resource-extension-development',
              )
              .toList()) {
        final directory = owned.remove(old.key);
        receipts.remove(old.key);
        if (directory != null) await directory.parent.delete(recursive: true);
      }
      final receipt = <String, Object?>{
        'opened': true,
        'runRef': runRef,
        'sha256': entry.sha256,
        'channel': 'resource-extension-development',
      };
      receipts[runRef] = receipt;
      return receipt;
    }
    if (owned.length > 4) {
      owned.remove(runRef);
      await root.delete(recursive: true);
      throw StateError('DEVELOPMENT_RUN_BUDGET_EXCEEDED');
    }
    final navigator = navigatorKey?.currentState;
    if (navigator == null || scope() != ownerScope || !authorization.value) {
      owned.remove(runRef);
      await root.delete(recursive: true);
      throw StateError('HOST_WINDOW_UNAVAILABLE');
    }
    unawaited(
      navigator.push<void>(
        MaterialPageRoute(
          builder: (_) => Scaffold(
            appBar: AppBar(title: Text('开发预览 · ${entry.title}')),
            body: McpInstalledApp(
              pluginId: entry.name,
              environment: this,
              developmentEntry: entry,
              developmentDirectory: staging.directoryFor(entry),
              authorization: authorization,
            ),
          ),
        ),
      ),
    );
    return {
      'opened': true,
      'runRef': runRef,
      'sha256': entry.sha256,
      'channel': 'development',
    };
  }

  Future<Map<String, Object?>> readDevelopment(
    String caller,
    Map<String, Object?> args,
  ) async {
    final directory = _developmentRuns[caller]?[args['runRef']];
    if (directory == null) throw StateError('DEVELOPMENT_RUN_NOT_GRANTED');
    final session = await McpBundleRuntime(
      bundleDirectory: directory,
      nodeExecutable: nodeExecutable,
    ).connect();
    try {
      final result = (await session.client.readResource(
        ReadResourceRequest(uri: args['uri'] as String),
      )).toJson();
      if (jsonEncode(result).length > 1024 * 1024) {
        throw StateError('OUTPUT_BUDGET_EXCEEDED');
      }
      return result;
    } finally {
      await session.close();
    }
  }

  Future<void> closeCaller(String caller) async {
    _developmentReceipts.remove(caller);
    onDevelopmentClosed?.call(caller);
    final authorization = _developmentAuthorizations.remove(caller);
    authorization?.value = false;
    authorization?.dispose();
    final runs = _developmentRuns.remove(caller);
    if (runs != null) {
      for (final dir in runs.values) {
        await dir.parent.delete(recursive: true).catchError((_) => dir.parent);
      }
    }
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
    this.authorization,
  });
  final String pluginId;
  final McpAppsEnvironment environment;
  final McpMarketEntry? developmentEntry;
  final Directory? developmentDirectory;
  final ValueNotifier<bool>? authorization;
  @override
  State<McpInstalledApp> createState() => _McpInstalledAppState();
}

final class _McpInstalledAppState extends State<McpInstalledApp> {
  McpBundleSession? _session;
  MethodChannel? _channel;
  McpCapabilityGateway? _gateway;
  List<Tool> _tools = [];
  late final String _scope;
  bool get _valid =>
      mounted &&
      widget.environment.scope() == _scope &&
      widget.authorization?.value != false;
  String? _html, _failure;
  Map<String, Object?> _presentation = const {};
  bool _viewReady = false;
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final next = mcpAppPresentation(context);
    if (jsonEncode(next) != jsonEncode(_presentation)) {
      _presentation = next;
      unawaited(_sendPresentation());
    }
  }

  Future<void> _sendPresentation() async {
    if (_viewReady && _channel != null) {
      try {
        await _channel!.invokeMethod<void>('message', {
          '_hostContext': _presentation,
        });
      } catch (_) {}
    }
  }

  late final String _caller =
      'view-${identityHashCode(this)}-${DateTime.now().microsecondsSinceEpoch}';

  @override
  void initState() {
    super.initState();
    _scope = widget.environment.scope();
    widget.authorization?.addListener(_authorizationChanged);
    WidgetsBinding.instance.addPostFrameCallback((_) => unawaited(_load()));
  }

  void _authorizationChanged() {
    if (widget.authorization?.value == false) {
      unawaited(_gateway?.close() ?? Future.value());
      unawaited(_session?.close() ?? Future.value());
      if (mounted) setState(() => _failure = '开发会话已结束。请从 Workspace 新终端重新运行。');
    }
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
      if (!_valid) throw StateError('CALLER_SCOPE_REVOKED');
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
        scope: () => _valid ? _scope : 'revoked-view',
        openDevelopment: _openDevelopment,
        readDevelopment: _readDevelopment,
      );
      if (_valid) {
        setState(
          () => _html = buildMcpAppHostDocument(
            appHtml: html,
            arguments: const {},
            platform: 'desktop',
            presentation: _presentation,
            serverAvailable: true,
          ),
        );
      }
    } catch (_) {
      await _session?.close();
      if (mounted) setState(() => _failure = '插件启动失败。请核对安装、必填配置与 MCP Apps 入口。');
    }
  }

  Future<Map<String, Object?>> _openDevelopment(Map<String, Object?> args) =>
      widget.environment.openDevelopment(_caller, args);
  Future<Map<String, Object?>> _readDevelopment(Map<String, Object?> args) =>
      widget.environment.readDevelopment(_caller, args);

  void _created(int id) {
    final channel = MethodChannel('com.openmuse.mcp/app-view/$id');
    _channel = channel;
    channel.setMethodCallHandler((call) async {
      if (call.method != 'message' || call.arguments is! Map) return;
      final request = Map<String, Object?>.from(call.arguments as Map);
      if (request['method'] == 'ui/notifications/initialized') {
        _viewReady = true;
        await _sendPresentation();
        return;
      }
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
                ...McpAppsEnvironment.workspaceTools,
              ],
            };
          case 'tools/call':
            final name = params['name'] as String;
            final args = Map<String, Object?>.from(
              params['arguments'] as Map? ?? {},
            );
            if (McpAppsEnvironment.workspaceTools.any(
              (t) => t['name'] == name,
            )) {
              final output = await widget.environment.workspaceCall(
                name,
                args,
                widget.environment.workspaceDirectory?.call() ??
                    Directory.current.path,
              );
              response['result'] = {
                'content': [
                  {'type': 'text', 'text': jsonEncode(output)},
                ],
                'structuredContent': output,
              };
            } else if (_gateway!.handles(name)) {
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
      } catch (error, stack) {
        // Do not log resource bytes, arguments, or third-party error messages.
        // Keep a call-site trace so native bridge failures are diagnosable.
        final method =
            const {
              'tools/list',
              'tools/call',
              'resources/list',
              'resources/read',
            }.contains(request['method'])
            ? request['method']
            : 'unsupported';
        final rawReason = error is PlatformException
            ? error.code
            : error is StateError
            ? error.message
            : '';
        final reason = RegExp(r'^[A-Z][A-Z0-9_]{1,63}$').hasMatch(rawReason)
            ? rawReason
            : 'REQUEST_FAILED';
        debugPrint(
          'MCP Apps $method failed: ${error.runtimeType} $reason\n$stack',
        );
        try {
          final log = File(
            p.join(
              widget.environment.market.installRoot.parent.path,
              'mcp-apps-diagnostic.log',
            ),
          );
          final append = await log.exists() && await log.length() < 128 * 1024;
          await log.writeAsString(
            '${DateTime.now().toUtc().toIso8601String()} '
            '$method ${error.runtimeType} $reason\n$stack\n',
            mode: append ? FileMode.append : FileMode.write,
          );
        } catch (_) {
          // Diagnostic storage must never alter the protocol response.
        }
        response.remove('result');
        response['error'] = {
          'code': -32602,
          'message':
              'Request denied or failed ($reason); check permissions, inputs and provider availability.',
        };
      }
      if (mounted) await channel.invokeMethod<void>('message', response);
    });
  }

  @override
  void dispose() {
    widget.authorization?.removeListener(_authorizationChanged);
    _channel?.setMethodCallHandler(null);
    unawaited(_gateway?.close() ?? Future.value());
    unawaited(_session?.close() ?? Future.value());
    unawaited(widget.environment.closeCaller(_caller));
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
