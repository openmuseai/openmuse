import 'dart:convert';
import '../openmuse_plugin_sdk.dart';

/// One mounted extension, bound to exactly one active resource and account.
final class ResourceExtensionSession {
  ResourceExtensionSession({
    required this.caller,
    required this.resourceUri,
    required this.broker,
    required this.contributors,
    required this.approve,
    required this.isCurrent,
    this.resourceTools = const {},
    this.serverTools = const [],
    this.serverCall,
  });
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
