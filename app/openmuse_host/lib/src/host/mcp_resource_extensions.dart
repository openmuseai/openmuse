import 'dart:convert';

import 'package:mcp_dart/mcp_dart.dart';
import 'package:flutter/foundation.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'mcp_market.dart';
import 'resource_agent_broker.dart';

/// Public Resource Extensions profile; no document or Agent business schema.
final class McpResourceExtension {
  const McpResourceExtension(
    this.id,
    this.html,
    this.width,
    this.height,
    this.isCurrent,
    this.tools,
    this.invoke,
    this.resourceTools,
  );
  final Set<String> resourceTools;
  final List<Map<String, Object?>> tools;
  final Future<Map<String, Object?>> Function(String, Map<String, Object?>)
  invoke;
  final String id, html;
  final int width, height;
  final Future<bool> Function() isCurrent;
}

final class McpResourceExtensions extends ChangeNotifier {
  McpResourceExtensions({required this.market, required this.nodeExecutable});
  final McpRegistryMarket market;
  final String nodeExecutable;
  Future<List<({McpResourceExtension extension, Set<String> types})>>? _cache;
  final _development =
      <
        String,
        ({
          McpMarketEntry entry,
          McpBundleRuntime runtime,
          Future<bool> Function() valid,
        })
      >{};
  void invalidate() {
    _cache = null;
    notifyListeners();
  }

  Future<bool> addDevelopment(
    String caller,
    McpMarketEntry entry,
    McpBundleRuntime runtime,
    Future<bool> Function() valid,
  ) async {
    final session = await runtime.connect();
    try {
      if (!(await session.client.listResources()).resources.any(
        (r) =>
            r.mimeType == 'application/vnd.openmuse.resource-extensions+json',
      )) {
        return false;
      }
    } finally {
      await session.close();
    }
    _development[caller] = (entry: entry, runtime: runtime, valid: valid);
    invalidate();
    return true;
  }

  void removeDevelopment(String caller) {
    if (_development.remove(caller) != null) invalidate();
  }

  Future<List<McpResourceExtension>> forType(String type) async => [
    for (final entry in await (_cache ??= _discover()))
      if (entry.types.contains(type)) entry.extension,
  ];
  Future<List<({McpResourceExtension extension, Set<String> types})>>
  _discover() async {
    final found = <({McpResourceExtension extension, Set<String> types})>[];
    final sources = [
      for (final entry in await market.installedEntries())
        (
          entry: entry,
          runtime: McpBundleRuntime(
            bundleDirectory: market.directoryFor(entry),
            nodeExecutable: nodeExecutable,
          ),
          valid: () async => (await market.installedEntries()).any(
            (e) => e.name == entry.name && e.sha256 == entry.sha256,
          ),
        ),
      ..._development.values,
    ];
    for (final source in sources) {
      final entry = source.entry, runtime = source.runtime;
      McpBundleSession? session;
      try {
        session = await runtime.connect();
        final resources = await session.client.listResources();
        for (final resource in resources.resources.where(
          (r) =>
              r.mimeType == 'application/vnd.openmuse.resource-extensions+json',
        )) {
          final read = await session.client.readResource(
            ReadResourceRequest(uri: resource.uri),
          );
          final catalog =
              jsonDecode(
                    read.contents.whereType<TextResourceContents>().single.text,
                  )
                  as Map;
          if (catalog['profile'] != 'openmuse-resource-extensions/1') continue;
          final items = catalog['extensions'] as List;
          if (items.length > 4) throw const FormatException('Extension budget');
          for (final item in items.whereType<Map>()) {
            final id = item['id'], tool = item['tool'];
            if (id is! String ||
                !RegExp(r'^[a-zA-Z0-9._-]{1,80}$').hasMatch(id) ||
                tool is! String) {
              throw const FormatException('Invalid extension');
            }
            final width = item['width'], height = item['height'];
            if (width is! int ||
                width < 200 ||
                width > 480 ||
                height is! int ||
                height < 48 ||
                height > 180) {
              throw const FormatException('Invalid extension bounds');
            }
            final types = (item['mediaTypes'] as List).cast<String>().toSet();
            final html = await runtime.uiDocument(tool);
            if (html == null || utf8.encode(html).length > 4 * 1024 * 1024) {
              continue;
            }
            found.add((
              extension: McpResourceExtension(
                '${entry.name}:$id',
                html,
                width,
                height,
                source.valid,
                (await session.client.listTools()).tools
                    .map((t) => t.toJson())
                    .toList(),
                runtime.callTool,
                (item['resourceTools'] as List? ?? []).cast<String>().toSet(),
              ),
              types: types,
            ));
          }
        }
      } catch (_) {
        // An incompatible optional contribution must not prevent file opening.
      } finally {
        await session?.close();
      }
    }
    return found.take(4).toList();
  }
}

/// One mounted extension, bound to exactly one active resource and account.
final class ResourceExtensionSession {
  ResourceExtensionSession({
    required this.caller,
    required this.resourceUri,
    required this.broker,
    required this.contributors,
    required this.approve,
    required this.isCurrent,
    this.reveal,
    this.resourceTools = const {},
    this.serverTools = const [],
    this.serverCall,
  });
  final Future<void> Function(String)? reveal;
  final Set<String> resourceTools;
  final List<Map<String, Object?>> serverTools;
  final Future<Map<String, Object?>> Function(String, Map<String, Object?>)?
  serverCall;
  final String caller, resourceUri;
  final ResourceAgentBroker broker;
  final List<({String id, OpenMuseToolContributor plugin})> Function()
  contributors;
  final Future<bool> Function(String title, String details) approve;
  final Future<bool> Function() isCurrent;
  final _grants = <String>{};
  bool _closed = false;
  Future<void> close() async {
    _closed = true;
    _grants.clear();
    for (final p in contributors()) {
      await p.plugin.disposeToolCaller(caller);
    }
  }

  Future<void> _check() async {
    if (_closed || broker.activeUri != resourceUri || !await isCurrent()) {
      throw StateError('RESOURCE_SESSION_CLOSED');
    }
  }

  static Map<String, Object?> _result(Object? value) => {
    'content': [
      {'type': 'text', 'text': jsonEncode(value)},
    ],
    'structuredContent': value,
  };
  static const tools = [
    ...resourceRouterTools,
    {
      'name': 'list_plugin_tools',
      'description':
          'Discover installed plugin tools; business schemas belong to the contributing plugin.',
      'inputSchema': {
        'type': 'object',
        'properties': {},
        'additionalProperties': false,
      },
      'annotations': {'readOnlyHint': true},
    },
    {
      'name': 'call_plugin_tool',
      'description':
          'Call an installed plugin tool after user authorization. Caller lifetime and resource are scoped by the Host.',
      'inputSchema': {
        'type': 'object',
        'properties': {
          'plugin': {'type': 'string'},
          'name': {'type': 'string'},
          'arguments': {'type': 'object'},
        },
        'required': ['plugin', 'name', 'arguments'],
        'additionalProperties': false,
      },
    },
  ];
  Future<Map<String, Object?>> dispatch(
    String method,
    Map<String, Object?> params,
  ) async {
    await _check();
    if (method == 'tools/list') {
      return {
        'tools': [
          ...tools,
          ...serverTools.where(
            (t) => !tools.any((host) => host['name'] == t['name']),
          ),
        ],
      };
    }
    if (method != 'tools/call') throw StateError('METHOD_UNAVAILABLE');
    if (jsonEncode(params).length > 128 * 1024) {
      throw StateError('REQUEST_BUDGET_EXCEEDED');
    }
    final name = params['name'];
    final args = Map<String, Object?>.from(params['arguments'] as Map? ?? {});
    if (name == 'list_plugin_tools') {
      return _result({
        'plugins': [
          for (final p in contributors())
            {'id': p.id, 'tools': p.plugin.contributedTools},
        ],
      });
    }
    if (name == 'call_plugin_tool') {
      final target = contributors().where((p) => p.id == args['plugin']).single;
      if (!target.plugin.contributedTools.any(
        (t) => t['name'] == args['name'],
      )) {
        throw StateError('TOOL_UNAVAILABLE');
      }
      final grant = target.id;
      if (!_grants.contains(grant)) {
        if (!await approve(
          '允许插件协作',
          '当前文档 → ${target.plugin is OpenMusePlugin ? (target.plugin as OpenMusePlugin).descriptor.name : target.id}\n按该插件原有的权限设置处理。关闭或切换当前文档后，停止本次调用。',
        )) {
          throw StateError('PERMISSION_DENIED');
        }
        await _check();
        _grants.add(grant);
      }
      await _check();
      await reveal?.call(target.id);
      await _check();
      return target.plugin.callContributedTool(
        caller,
        args['name'] as String,
        Map<String, Object?>.from(args['arguments'] as Map? ?? {}),
      );
    }
    if (args.containsKey('uri') && args['uri'] != resourceUri) {
      throw StateError('RESOURCE_SCOPE_MISMATCH');
    }
    if (!resourceRouterTools.any((t) => t['name'] == name)) {
      if (serverCall != null && serverTools.any((t) => t['name'] == name)) {
        return serverCall!(name as String, args);
      }
      throw StateError('TOOL_UNAVAILABLE');
    }
    if (name == 'call_resource_tool' && !resourceTools.contains(args['name'])) {
      throw StateError('RESOURCE_TOOL_NOT_GRANTED');
    }
    return broker.dispatch(method, params);
  }
}
